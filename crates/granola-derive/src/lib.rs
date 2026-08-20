use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Field, Fields, GenericParam, Ident, Token, Type, parse_macro_input,
    punctuated::Punctuated,
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

fn parse_recipe_name(input: &DeriveInput) -> syn::Result<Ident> {
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

    recipe_attr.parse_args::<Ident>()
}

fn recipe_type_parameter(input: &DeriveInput, derive_name: &str) -> syn::Result<Ident> {
    let mut parameters = input.generics.params.iter();
    let first = parameters.next();

    match (first, parameters.next()) {
        (Some(GenericParam::Type(parameter)), None) => Ok(parameter.ident.clone()),
        (None, _) => Err(syn::Error::new_spanned(
            &input.ident,
            format!("`{derive_name}` requires a recipe type parameter"),
        )),
        _ => Err(syn::Error::new_spanned(
            &input.generics,
            format!("`{derive_name}` supports exactly one type parameter"),
        )),
    }
}

fn with_type_param_bound(
    generics: &syn::Generics,
    type_param: &Ident,
    bound: syn::TypeParamBound,
) -> syn::Generics {
    let mut generics = generics.clone();

    generics
        .type_params_mut()
        .find(|parameter| parameter.ident == *type_param)
        .expect("validated recipe type parameter")
        .bounds
        .push(bound);

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
    input: &DeriveInput,
    named_fields: &Punctuated<Field, Token![,]>,
    recipe_type_param: &Ident,
) -> syn::Result<()> {
    let first = named_fields.first().ok_or_else(|| {
        syn::Error::new_spanned(
            &input.ident,
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
    parse_recipe_name(input)?;
    let recipe_type_param = recipe_type_parameter(input, "DaisyUI")?;

    const CAPABILITIES: [&str; 7] = [
        "Style",
        "Behavior",
        "Color",
        "Size",
        "Placement",
        "Direction",
        "Modifier",
    ];

    let struct_name = &input.ident;
    let capability_impls = CAPABILITIES.map(|capability| {
        let capability_trait = format_ident!("DaisyUI{capability}");
        let capability_type = format_ident!("Component{capability}");
        let recipe_trait = format_ident!("Has{capability}");
        let recipe_associated_type = format_ident!("{capability}");

        let generics = with_type_param_bound(
            &input.generics,
            &recipe_type_param,
            syn::parse_quote!(::granola::daisyui::#recipe_trait),
        );
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            impl #impl_generics ::granola::daisyui::#capability_trait
                for #struct_name #ty_generics #where_clause
            {
                type #capability_type = <#recipe_type_param as ::granola::daisyui::#recipe_trait>::#recipe_associated_type;
            }
        }
    });

    Ok(quote! {
        #(#capability_impls)*
    })
}

/// Derive macro for recipes.
///
/// Requires a named-field struct with exactly one type parameter, the recipe
/// parameter, and a leading `_recipe: PhantomData<R>` field.
///
/// For a struct `Foo<R>`, it generates:
/// - the recipe trait named by `#[recipe(RecipeName)]`, with one hook per field
///   and an impl for `()` (the baked, no-op recipe)
/// - the `new` and `from_recipe` constructors, plus a `From<R>` impl
///   (`Foo::from(recipe)`)
/// - a `bake_recipe` method lowering `Foo<R>` to `Foo<()>`
///
/// For structs with a `content` field:
/// - a `content(content)` builder method
/// - an `escape(content)` builder method
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

    let trait_name = parse_recipe_name(input)?;
    let type_param = recipe_type_parameter(input, "Recipe")?;
    let named_fields = recipe_named_fields(input)?;
    validate_recipe_layout(input, named_fields, &type_param)?;

    let recipe_generics =
        with_type_param_bound(&input.generics, &type_param, syn::parse_quote!(#trait_name));
    let (impl_generics, ty_generics, where_clause) = recipe_generics.split_for_impl();

    let other_fields: Vec<_> = named_fields.iter().skip(1).collect();

    let field_idents: Vec<&Ident> = other_fields
        .iter()
        .map(|field| field.ident.as_ref().expect("named fields have identifiers"))
        .collect();
    let field_types: Vec<&Type> = other_fields.iter().map(|f| &f.ty).collect();
    let method_names: Vec<Ident> = field_idents
        .iter()
        .map(|i| format_ident!("{i}_recipe"))
        .collect();
    let has_field = |name: &str| field_idents.iter().any(|i| *i == name);
    let has_content = has_field("content");

    let content_method = if has_content {
        quote! {
            pub fn content(
                mut self,
                content: impl ::std::convert::Into<::granola::oven::Bake>,
            ) -> Self {
                self.content = content.into();
                self
            }

            pub fn escape(
                mut self,
                content: impl ::askama::FastWritable,
            ) -> Self {
                self.content = ::granola::oven::escape_content(content);
                self
            }
        }
    } else {
        quote! {}
    };

    let attr_impls = other_fields.iter().filter_map(|field| {
        let field_ident = field.ident.as_ref().expect("named fields have identifiers");
        let trait_path = match field_ident.to_string().as_str() {
            "global_attrs" => quote!(::granola::html::HasGlobalAttrs),
            "global_aria_attrs" => quote!(::granola::html::HasGlobalAriaAttrs),
            "custom_data_attrs" => quote!(::granola::html::HasCustomDataAttrs),
            "event_handlers" => quote!(::granola::html::HasEventHandlers),
            "global_svg_attrs" => quote!(::granola::svg::HasGlobalSvgAttrs),
            "paint_attrs" => quote!(::granola::svg::HasPaintAttrs),
            "shape_attrs" => quote!(::granola::svg::HasShapeAttrs),
            "text_content_attrs" => quote!(::granola::svg::HasTextContentAttrs),
            _ => return None,
        };
        let method_name = format_ident!("{field_ident}_mut");
        let field_type = &field.ty;

        Some(quote! {
            impl #impl_generics #trait_path
                for #struct_name #ty_generics #where_clause
            {
                fn #method_name(&mut self) -> &mut #field_type {
                    &mut self.#field_ident
                }
            }
        })
    });

    Ok(quote! {
        pub trait #trait_name:
            ::std::default::Default
            + ::std::clone::Clone
            + ::std::fmt::Debug
            + 'static
        {
            #(fn #method_names() -> #field_types {
                ::std::default::Default::default()
            })*
        }

        impl #trait_name for () {}

        #(#attr_impls)*

        impl #struct_name<()> {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #content_method

            pub fn from_recipe() -> Self {
                Self {
                    _recipe: ::std::marker::PhantomData,
                    #(#field_idents: #type_param::#method_names(),)*
                }
            }

            /// Converts this element into its recipe-free form by replacing
            /// the recipe type parameter with `()`.
            pub fn bake_recipe(self) -> #struct_name<()> {
                #struct_name {
                    _recipe: ::std::marker::PhantomData,
                    #(#field_idents: self.#field_idents,)*
                }
            }
        }

        impl #impl_generics ::std::convert::From<#type_param>
            for #struct_name #ty_generics #where_clause
        {
            fn from(_recipe: #type_param) -> Self {
                Self::from_recipe()
            }
        }
    })
}
