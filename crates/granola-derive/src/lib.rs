use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Field, Fields, GenericParam, Ident, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

/// Derive macro for templates.
///
/// Implements:
/// - `bake` via `askama::Template::render`.
/// - `bake_pretty` via `bake` + `markup_fmt` and `malva`.
/// - `From<T> for Bake` and `From<&T> for Bake`.
#[proc_macro_derive(Granola, attributes(granola))]
pub fn granola_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    granola_derive_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn granola_derive_impl(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut pretty_fn = quote! { ::granola::pretty::pretty };
    let mut has_format = false;
    for attr in &input.attrs {
        if !attr.path().is_ident("granola") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("format") {
                if has_format {
                    return Err(meta.error("duplicate `format` option in #[granola(...)]"));
                }
                has_format = true;

                let format: Ident = meta.value()?.parse()?;
                pretty_fn = match format.to_string().as_str() {
                    "html" => quote! { ::granola::pretty::pretty },
                    "css" => quote! { ::granola::pretty::pretty_css },
                    other => {
                        return Err(meta.error(format!(
                            "unknown formatter `{other}`, expected `html` or `css`"
                        )));
                    }
                };
                Ok(())
            } else {
                Err(meta.error("unknown `granola` option, expected `format`"))
            }
        })?;
    }

    let bake_pretty = if cfg!(feature = "pretty") {
        quote! {
            /// Renders the template and formats the result for readable
            /// output (e.g. snapshots, debugging).
            ///
            /// # Panics
            ///
            /// Panics if the formatter returns an error.
            pub fn bake_pretty(&self) -> ::std::string::String {
                #pretty_fn(&self.bake())
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Renders the template into a new [`String`].
            ///
            /// # Panics
            ///
            /// Panics if [`askama::Template::render`] returns an error. See
            /// [`askama::Error`].
            pub fn bake(&self) -> ::std::string::String {
                ::askama::Template::render(self).unwrap()
            }

            #bake_pretty
        }

        impl #impl_generics ::std::convert::From<#name #ty_generics> for ::granola::oven::Bake
        #where_clause
        {
            fn from(c: #name #ty_generics) -> Self {
                Self::from(&c)
            }
        }

        #[diagnostic::do_not_recommend]
        impl #impl_generics ::std::convert::From<&#name #ty_generics> for ::granola::oven::Bake
        #where_clause
        {
            fn from(c: &#name #ty_generics) -> Self {
                ::granola::oven::Bake::new(c)
            }
        }
    })
}

struct RecipeArgs {
    recipe_name: Ident,
    content_type: Option<Type>,
    attr_span: Span,
}

struct ParsedRecipeArgs {
    recipe_name: Option<Ident>,
    content_type: Option<Type>,
}

impl Parse for ParsedRecipeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut recipe_name: Option<Ident> = None;
        let mut content_type: Option<Type> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_span = key.span();
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "name" => {
                    if recipe_name.is_some() {
                        return Err(syn::Error::new(key_span, "duplicate `name` key"));
                    }
                    recipe_name = Some(input.parse()?);
                }
                "content" => {
                    if content_type.is_some() {
                        return Err(syn::Error::new(key_span, "duplicate `content` key"));
                    }
                    content_type = Some(input.parse()?);
                }
                _ => return Err(syn::Error::new(key_span, format!("unknown key `{key}`"))),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            recipe_name,
            content_type,
        })
    }
}

fn parse_recipe_args(input: &DeriveInput) -> syn::Result<RecipeArgs> {
    let mut recipe_attrs = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("recipe"));
    let recipe_attr = recipe_attrs
        .next()
        .ok_or_else(|| syn::Error::new_spanned(&input.ident, "missing #[recipe(...)] attribute"))?;
    if let Some(duplicate) = recipe_attrs.next() {
        return Err(syn::Error::new_spanned(
            duplicate,
            "duplicate #[recipe(...)] attribute",
        ));
    }

    let args: ParsedRecipeArgs = recipe_attr.parse_args()?;
    let recipe_name = args.recipe_name.ok_or_else(|| {
        syn::Error::new_spanned(recipe_attr, "`name` is required in #[recipe(name = ...)]")
    })?;

    Ok(RecipeArgs {
        recipe_name,
        content_type: args.content_type,
        attr_span: recipe_attr.span(),
    })
}

fn recipe_type_parameter(input: &DeriveInput, derive_name: &str) -> syn::Result<Ident> {
    input
        .generics
        .type_params()
        .next()
        .map(|parameter| parameter.ident.clone())
        .ok_or_else(|| {
            syn::Error::new_spanned(
                &input.ident,
                format!("`{derive_name}` requires a recipe type parameter"),
            )
        })
}

fn recipe_type_parameter_for_recipe(input: &DeriveInput) -> syn::Result<Ident> {
    let mut parameters = input.generics.params.iter();
    let first = parameters.next();

    match (first, parameters.next()) {
        (Some(GenericParam::Type(parameter)), None) => Ok(parameter.ident.clone()),
        (None, _) => Err(syn::Error::new_spanned(
            &input.ident,
            "`Recipe` requires one recipe type parameter",
        )),
        _ => Err(syn::Error::new_spanned(
            &input.generics,
            "`Recipe` supports exactly one type parameter, the recipe parameter",
        )),
    }
}

fn recipe_impl_generics(
    input: &DeriveInput,
    recipe_type_param: &Ident,
    recipe_trait: &Ident,
) -> syn::Generics {
    let mut generics = input.generics.clone();

    for parameter in &mut generics.params {
        if let GenericParam::Type(parameter) = parameter
            && parameter.ident == *recipe_type_param
        {
            parameter.bounds.push(syn::parse_quote!(#recipe_trait));
            break;
        }
    }

    generics
}

fn recipe_named_fields(input: &DeriveInput) -> syn::Result<&Punctuated<Field, Token![,]>> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Ok(&fields.named),
            Fields::Unnamed(fields) => Err(syn::Error::new_spanned(
                fields,
                "`Recipe` only supports structs with named fields",
            )),
            Fields::Unit => Err(syn::Error::new_spanned(
                &input.ident,
                "`Recipe` only supports structs with named fields",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            &input.ident,
            "`Recipe` only supports structs",
        )),
    }
}

fn is_phantom_data_of(ty: &Type, type_param: &Ident) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };
    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };
    if segment.ident != "PhantomData" {
        return false;
    }
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };
    let Some(syn::GenericArgument::Type(Type::Path(argument))) = arguments.args.first() else {
        return false;
    };

    arguments.args.len() == 1 && argument.qself.is_none() && argument.path.is_ident(type_param)
}

fn validate_recipe_layout(
    named_fields: &Punctuated<Field, Token![,]>,
    recipe_type_param: &Ident,
    args: &RecipeArgs,
) -> syn::Result<()> {
    let first = named_fields.first().ok_or_else(|| {
        syn::Error::new(
            args.attr_span,
            "`Recipe` requires an `_recipe: PhantomData<R>` field",
        )
    })?;
    if first.ident.as_ref().is_none_or(|ident| ident != "_recipe") {
        return Err(syn::Error::new_spanned(
            first,
            "the first field must be `_recipe: PhantomData<R>`",
        ));
    }
    if !is_phantom_data_of(&first.ty, recipe_type_param) {
        return Err(syn::Error::new_spanned(
            &first.ty,
            "the `_recipe` field must have type `PhantomData<R>`",
        ));
    }
    if args.content_type.is_some()
        && !named_fields
            .iter()
            .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "content"))
    {
        return Err(syn::Error::new(
            args.attr_span,
            "#[recipe(content = ...)] requires a `content` field",
        ));
    }

    Ok(())
}

/// Derive macro for daisyUI capabilities on recipe-backed elements.
#[proc_macro_derive(DaisyUI, attributes(recipe))]
pub fn daisyui_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    daisyui_derive_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn daisyui_derive_impl(input: &DeriveInput) -> syn::Result<TokenStream2> {
    parse_recipe_args(input)?;
    let recipe_type_param = recipe_type_parameter(input, "DaisyUI")?;

    const CAPABILITIES: [(&str, &str, &str, &str); 7] = [
        ("HasStyle", "DaisyUIStyle", "ComponentStyle", "Style"),
        (
            "HasBehavior",
            "DaisyUIBehavior",
            "ComponentBehavior",
            "Behavior",
        ),
        ("HasColor", "DaisyUIColor", "ComponentColor", "Color"),
        ("HasSize", "DaisyUISize", "ComponentSize", "Size"),
        (
            "HasPlacement",
            "DaisyUIPlacement",
            "ComponentPlacement",
            "Placement",
        ),
        (
            "HasDirection",
            "DaisyUIDirection",
            "ComponentDirection",
            "Direction",
        ),
        (
            "HasModifier",
            "DaisyUIModifier",
            "ComponentModifier",
            "Modifier",
        ),
    ];

    let capability_impls = CAPABILITIES.map(
        |(marker_trait, capability_trait, capability_type, marker_type)| {
            let marker_trait = format_ident!("{marker_trait}");
            let capability_trait = format_ident!("{capability_trait}");
            daisyui_capability_impl(
                input,
                &recipe_type_param,
                quote!(::granola::daisyui::#marker_trait),
                quote!(::granola::daisyui::#capability_trait),
                format_ident!("{capability_type}"),
                format_ident!("{marker_type}"),
            )
        },
    );

    Ok(quote! {
        #(#capability_impls)*
    })
}

fn daisyui_capability_impl(
    input: &DeriveInput,
    recipe_type_param: &Ident,
    marker_trait: TokenStream2,
    capability_trait: TokenStream2,
    capability_type: Ident,
    marker_type: Ident,
) -> TokenStream2 {
    let struct_name = &input.ident;
    let mut generics = input.generics.clone();

    for parameter in &mut generics.params {
        if let GenericParam::Type(parameter) = parameter
            && parameter.ident == *recipe_type_param
        {
            parameter.bounds.push(syn::parse_quote!(#marker_trait));
            break;
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #capability_trait for #struct_name #ty_generics #where_clause {
            type #capability_type = <#recipe_type_param as #marker_trait>::#marker_type;
        }
    }
}

/// Derive macro for recipes.
///
/// Requires a named-field struct with exactly one type parameter, the recipe
/// parameter, and a leading `_recipe: PhantomData<R>` field.
///
/// For a struct `Foo<R>`, it generates:
/// - the recipe trait named by `#[recipe(name = ...)]`, with one hook per field
///   and an impl for `()` (the baked, no-op recipe)
/// - the `new` and `from_cookbook` constructors, plus a `From<R>` impl
///   (`Foo::from(recipe)`)
/// - a `bake_recipe` method lowering `Foo<R>` to `Foo<()>`
///
/// With `content`:
/// - a `Content` associated type, a `content(content)` builder method
/// - a required `bake_content` method mapping `Content` back into the default
///   content type `T`
/// - a `From<(R, impl Into<R::Content>)>` impl (`Foo::from((recipe, content))`)
///
/// And also the matching `Has*` impl for:
/// - `global_attrs`, `global_aria_attrs`, `custom_data_attrs`,
///   `event_handlers`, `global_svg_attrs`, `paint_attrs`, `shape_attrs`,
///   `text_content_attrs`
#[proc_macro_derive(Recipe, attributes(recipe))]
pub fn recipe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    recipe_derive_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn recipe_derive_impl(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let struct_name = &input.ident;

    let args = parse_recipe_args(input)?;

    let trait_name = &args.recipe_name;
    let default_content_type = args.content_type.as_ref();
    let has_content = default_content_type.is_some();

    let type_param = recipe_type_parameter_for_recipe(input)?;
    let named_fields = recipe_named_fields(input)?;
    validate_recipe_layout(named_fields, &type_param, &args)?;

    let recipe_generics = recipe_impl_generics(input, &type_param, trait_name);
    let (impl_generics, ty_generics, where_clause) = recipe_generics.split_for_impl();
    let mut content_generics = recipe_generics.clone();
    content_generics.params.push(syn::parse_quote!(
        __IntoContent: ::std::convert::Into<#type_param::Content>
    ));
    let (content_impl_generics, _, content_where_clause) = content_generics.split_for_impl();

    // Every field except the leading `_recipe` marker and `content` (threaded
    // separately below). These drive the per-field recipe hooks.
    let other_fields: Vec<_> = named_fields
        .iter()
        .skip(1)
        .filter(|f| !(has_content && f.ident.as_ref().map(|i| i == "content").unwrap_or(false)))
        .collect();

    let field_idents: Vec<Ident> = other_fields
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect();
    let field_types: Vec<&Type> = other_fields.iter().map(|f| &f.ty).collect();
    let method_names: Vec<Ident> = field_idents
        .iter()
        .map(|i| format_ident!("{i}_recipe"))
        .collect();
    let has_field = |name: &str| field_idents.iter().any(|i| i == name);

    // When the recipe carries a `content` field (`#[recipe(content = T)]`), the
    // trait gains a `Content` associated type plus `bake_content` /
    // `content_recipe`, and the constructors thread content through. All such
    // content-gated fragments are grouped here.
    //
    // `bake_content` is emitted required (no default body) so a recipe that
    // overrides `type Content` must supply the map-back itself, surfacing the
    // gap on the author's own impl rather than downstream at `bake_recipe`.
    let trait_content = if let Some(content_type) = default_content_type {
        quote! {
            type Content:
                ::askama::FastWritable
                + ::std::default::Default
                + ::std::clone::Clone
                + ::std::fmt::Debug;

            /// Bakes this recipe's content back into the element's default
            /// content type, called when the recipe is lowered via
            /// `bake_recipe`.
            ///
            /// See [`recipe_boilerplate!`](::granola::recipe_boilerplate).
            fn bake_content(content: Self::Content) -> #content_type;

            fn content_recipe() -> Self::Content {
                ::std::default::Default::default()
            }
        }
    } else {
        quote! {}
    };

    // `()` impl: default content type, identity bake-back.
    let unit_content = if let Some(content_type) = default_content_type {
        quote! {
            type Content = #content_type;

            fn bake_content(content: #content_type) -> #content_type {
                content
            }
        }
    } else {
        quote! {}
    };

    // Constructor pieces that thread content through `from_cookbook`,
    // `content(...)`, and the `bake_recipe` lowering.
    let content_init = if has_content {
        quote! {
            let content = #type_param::content_recipe();
        }
    } else {
        quote! {}
    };
    let content_struct_field = if has_content {
        quote! { content, }
    } else {
        quote! {}
    };
    // `content`: sets the content on `#struct_name<R>`, keeping the
    // recipe `R`. Returns `Self`, so the recipe is fixed at construction and
    // flows through unchanged.
    let content_method = if has_content {
        quote! {
            pub fn content(
                mut self,
                content: impl ::std::convert::Into<#type_param::Content>,
            ) -> Self {
                self.content = content.into();
                self
            }
        }
    } else {
        quote! {}
    };
    let bake_content_field = if has_content {
        quote! { content: #type_param::bake_content(self.content), }
    } else {
        quote! {}
    };

    // HTML
    let global_attrs_impl = if has_field("global_attrs") {
        quote! {
            impl #impl_generics ::granola::html::HasGlobalAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn global_attrs_mut(&mut self) -> &mut ::granola::html::GlobalAttrs {
                    &mut self.global_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let global_aria_attrs_impl = if has_field("global_aria_attrs") {
        quote! {
            impl #impl_generics ::granola::html::HasGlobalAriaAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn global_aria_attrs_mut(&mut self) -> &mut ::granola::html::GlobalAriaAttrs {
                    &mut self.global_aria_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let custom_data_attrs_impl = if has_field("custom_data_attrs") {
        quote! {
            impl #impl_generics ::granola::html::HasCustomDataAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn custom_data_attrs_mut(&mut self) -> &mut ::granola::html::CustomDataAttrs {
                    &mut self.custom_data_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let event_handlers_impl = if has_field("event_handlers") {
        quote! {
            impl #impl_generics ::granola::html::HasEventHandlers
                for #struct_name #ty_generics #where_clause
            {
                fn event_handlers_mut(&mut self) -> &mut ::granola::html::EventHandlers {
                    &mut self.event_handlers
                }
            }
        }
    } else {
        quote! {}
    };

    // SVG
    let global_svg_attrs_impl = if has_field("global_svg_attrs") {
        quote! {
            impl #impl_generics ::granola::svg::HasGlobalSvgAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn global_svg_attrs_mut(&mut self) -> &mut ::granola::svg::GlobalSvgAttrs {
                    &mut self.global_svg_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let paint_attrs_impl = if has_field("paint_attrs") {
        quote! {
            impl #impl_generics ::granola::svg::HasPaintAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn paint_attrs_mut(&mut self) -> &mut ::granola::svg::PaintAttrs {
                    &mut self.paint_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let shape_attrs_impl = if has_field("shape_attrs") {
        quote! {
            impl #impl_generics ::granola::svg::HasShapeAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn shape_attrs_mut(&mut self) -> &mut ::granola::svg::ShapeAttrs {
                    &mut self.shape_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let text_content_attrs_impl = if has_field("text_content_attrs") {
        quote! {
            impl #impl_generics ::granola::svg::HasTextContentAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn text_content_attrs_mut(&mut self) -> &mut ::granola::svg::TextContentAttrs {
                    &mut self.text_content_attrs
                }
            }
        }
    } else {
        quote! {}
    };

    let from_recipe_and_content = if has_content {
        quote! {
            impl #content_impl_generics
                ::std::convert::From<(#type_param, __IntoContent)>
                for #struct_name #ty_generics #content_where_clause
            {
                fn from((_recipe, content): (#type_param, __IntoContent)) -> Self {
                    Self::from_cookbook().content(content)
                }
            }
        }
    } else {
        quote! {}
    };

    // `new`: empty constructor, only on `#struct_name<()>`.
    let new_method = quote! {
        pub fn new() -> Self {
            Self {
                ..::std::default::Default::default()
            }
        }
    };

    Ok(quote! {
        pub trait #trait_name:
            ::std::default::Default
            + ::std::clone::Clone
            + ::std::fmt::Debug
            + 'static
        {
            #trait_content
            #(fn #method_names() -> #field_types {
                ::std::default::Default::default()
            })*
        }

        impl #trait_name for () {
            #unit_content
        }

        #global_attrs_impl
        #global_aria_attrs_impl
        #custom_data_attrs_impl
        #event_handlers_impl

        #global_svg_attrs_impl
        #paint_attrs_impl
        #shape_attrs_impl
        #text_content_attrs_impl

        impl #struct_name<()> {
            #new_method
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #content_method

            pub fn from_cookbook() -> Self {
                #content_init
                Self {
                    #content_struct_field
                    #(#field_idents: #type_param::#method_names(),)*
                    ..::std::default::Default::default()
                }
            }
        }

        impl #impl_generics ::std::convert::From<#type_param>
            for #struct_name #ty_generics #where_clause
        {
            fn from(_recipe: #type_param) -> Self {
                Self::from_cookbook()
            }
        }

        #from_recipe_and_content

        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Converts this element into its recipe-free form, replacing
            /// the recipe type parameter with its default.
            pub fn bake_recipe(self) -> #struct_name<()> {
                #struct_name {
                    _recipe: ::std::marker::PhantomData,
                    #bake_content_field
                    #(#field_idents: self.#field_idents,)*
                }
            }
        }
    })
}
