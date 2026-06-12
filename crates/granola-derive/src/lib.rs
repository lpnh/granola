use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    DeriveInput, Ident, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// Derive macro for templates.
///
/// Implements:
/// - `bake()` via `askama::Template::render`.
/// - `From<T> for Cow<'static, str>` via `bake()`.
#[proc_macro_derive(Granola)]
pub fn granola_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Renders the template into a new [`String`].
            ///
            /// # Panics
            ///
            /// Panics if [`askama::Template::render`] returns an error. See [`askama::Error`].
            pub fn bake(&self) -> ::std::string::String {
                ::askama::Template::render(self).unwrap()
            }
        }

        impl #impl_generics From<#name #ty_generics> for ::std::borrow::Cow<'static, str>
        #where_clause
        {
            fn from(c: #name #ty_generics) -> Self {
                ::std::borrow::Cow::Owned(c.bake())
            }
        }
    }
    .into()
}

struct RecipeArgs {
    recipe_name: Ident,
    content_type: Option<Type>,
}

impl Parse for RecipeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut recipe_name: Option<Ident> = None;
        let mut content_type: Option<Type> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_span = key.span();
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "name" => recipe_name = Some(input.parse()?),
                "content" => content_type = Some(input.parse()?),
                _ => return Err(syn::Error::new(key_span, format!("unknown key `{key}`"))),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(RecipeArgs {
            recipe_name: recipe_name.ok_or_else(|| {
                syn::Error::new(
                    Span::call_site(),
                    "`name` is required in #[recipe(name = ...)]",
                )
            })?,
            content_type,
        })
    }
}

/// Derive macro for recipes.
///
/// For a struct `Foo<R>`, it generates:
///
/// - the recipe trait named by `#[recipe(name = ...)]`, with one hook per field
///   and impls for `()` and `(A, B)` so recipes compose as tuples;
/// - the `new()` and `from_cookbook()` constructors, plus a `From<R>` impl
///   (`Foo::from(recipe)`);
/// - a `BakeRecipe` impl lowering `Foo<R>` to `Foo<()>`.
///
/// Some field names add more:
/// - `content` (with `#[recipe(content = T)]`): a `Content` associated type, a
///   `content(content)` constructor, and a required `bake_content` method
///   mapping `Content` back into the default content type `T`.
/// - `global_attrs`, `global_aria_attrs`, `custom_data_attrs`,
///   `event_handlers`, `global_svg_attrs`, `paint_attrs`, `shape_attrs`,
///   `text_content_attrs`: the matching `Has*` impl.
#[proc_macro_derive(Recipe, attributes(recipe))]
pub fn recipe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let recipe_attr = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("recipe"))
        .expect("missing #[recipe(...)] attribute");
    let args: RecipeArgs = recipe_attr
        .parse_args()
        .expect("failed to parse #[recipe(...)]");

    let trait_name = &args.recipe_name;
    let default_content_type = args.content_type;
    let has_content = default_content_type.is_some();

    let type_param = input
        .generics
        .type_params()
        .next()
        .expect("Recipe requires a type parameter")
        .ident
        .clone();
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();

    let named_fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(f) => &f.named,
            _ => panic!("Recipe only supports named fields"),
        },
        _ => panic!("Recipe only supports structs"),
    };

    let first = named_fields.first().expect("struct has no fields");
    if first.ident.as_ref().map(|i| i != "_recipe").unwrap_or(true) {
        panic!("first field must be `_recipe: PhantomData<...>`");
    }

    // Every field except the leading `_recipe` marker and `content` (threaded
    // separately below). These drive the per-field recipe hooks.
    let other_fields: Vec<_> = named_fields
        .iter()
        .skip(1)
        .filter(|f| !(has_content && f.ident.as_ref().map(|i| i == "content").unwrap_or(false)))
        .collect();

    let field_idents: Vec<Ident> = other_fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();
    let field_types: Vec<&Type> = other_fields.iter().map(|f| &f.ty).collect();
    let method_names: Vec<Ident> = field_idents
        .iter()
        .map(|i| format_ident!("{i}_recipe"))
        .collect();
    let param_names: Vec<Ident> = field_idents.iter().map(|i| format_ident!("_{i}")).collect();
    let has_field = |name: &str| field_idents.iter().any(|i| i == name);

    // When the recipe carries a `content` field (`#[recipe(content = T)]`), the
    // trait gains a `Content` associated type plus `bake_content` /
    // `content_recipe`, and the constructors thread content through. All such
    // content-gated fragments are grouped here.
    //
    // `bake_content` is emitted required (no default body) so a recipe that
    // overrides `type Content` must supply the map-back itself, surfacing the
    // gap on the author's own impl rather than downstream at `bake_recipe`.
    let trait_content = if let Some(ref content_type) = default_content_type {
        quote! {
            type Content:
                ::askama::FastWritable
                + ::std::default::Default
                + ::std::clone::Clone
                + ::std::fmt::Debug
                = #content_type;

            /// Bakes this recipe's content back into the element's default
            /// content type, called when the recipe is lowered via
            /// [`BakeRecipe`](crate::oven::BakeRecipe).
            ///
            /// See [`recipe_boilerplate!`](crate::recipe_boilerplate).
            fn bake_content(content: Self::Content) -> #content_type;

            fn content_recipe(_content: &mut Self::Content) {}
        }
    } else {
        quote! {}
    };

    // `(A, B)` impl: where clause plus the composed Content type + hooks.
    let tuple_where = if has_content {
        quote! { where A: #trait_name, B: #trait_name<Content = A::Content>, }
    } else {
        quote! { where A: #trait_name, B: #trait_name, }
    };
    let tuple_content = if let Some(ref content_type) = default_content_type {
        quote! {
            type Content = A::Content;
            fn bake_content(content: Self::Content) -> #content_type {
                A::bake_content(content)
            }
            fn content_recipe(content: &mut Self::Content) {
                A::content_recipe(content);
                B::content_recipe(content);
            }
        }
    } else {
        quote! {}
    };

    // `()` impl: identity bake-back for the default content type.
    let unit_content = if let Some(ref content_type) = default_content_type {
        quote! {
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
            let mut content = <#type_param::Content as ::std::default::Default>::default();
            #type_param::content_recipe(&mut content);
        }
    } else {
        quote! {}
    };
    let content_struct_field = if has_content {
        quote! { content, }
    } else {
        quote! {}
    };
    // `content(content)`: sets the content on `#struct_name<R>`, keeping the
    // recipe `R`. Returns `Self`, so the recipe is fixed at construction and
    // flows through unchanged.
    let content_method = if has_content {
        quote! {
            pub fn content(
                mut self,
                content: impl ::std::convert::Into<#type_param::Content>,
            ) -> Self {
                let mut content = content.into();
                #type_param::content_recipe(&mut content);
                self.content = content;
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
            impl<#type_param: #trait_name> crate::html::HasGlobalAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn global_attrs_mut(&mut self) -> &mut crate::html::GlobalAttrs {
                    &mut self.global_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let global_aria_attrs_impl = if has_field("global_aria_attrs") {
        quote! {
            impl<#type_param: #trait_name> crate::html::HasGlobalAriaAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn global_aria_attrs_mut(&mut self) -> &mut crate::html::GlobalAriaAttrs {
                    &mut self.global_aria_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let custom_data_attrs_impl = if has_field("custom_data_attrs") {
        quote! {
            impl<#type_param: #trait_name> crate::html::HasCustomDataAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn custom_data_attrs_mut(&mut self) -> &mut crate::html::CustomDataAttrs {
                    &mut self.custom_data_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let event_handlers_impl = if has_field("event_handlers") {
        quote! {
            impl<#type_param: #trait_name> crate::html::HasEventHandlers
                for #struct_name #ty_generics #where_clause
            {
                fn event_handlers_mut(&mut self) -> &mut crate::html::EventHandlers {
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
            impl<#type_param: #trait_name> crate::svg::HasGlobalSvgAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn global_svg_attrs_mut(&mut self) -> &mut crate::svg::GlobalSvgAttrs {
                    &mut self.global_svg_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let paint_attrs_impl = if has_field("paint_attrs") {
        quote! {
            impl<#type_param: #trait_name> crate::svg::HasPaintAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn paint_attrs_mut(&mut self) -> &mut crate::svg::PaintAttrs {
                    &mut self.paint_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let shape_attrs_impl = if has_field("shape_attrs") {
        quote! {
            impl<#type_param: #trait_name> crate::svg::HasShapeAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn shape_attrs_mut(&mut self) -> &mut crate::svg::ShapeAttrs {
                    &mut self.shape_attrs
                }
            }
        }
    } else {
        quote! {}
    };
    let text_content_attrs_impl = if has_field("text_content_attrs") {
        quote! {
            impl<#type_param: #trait_name> crate::svg::HasTextContentAttrs
                for #struct_name #ty_generics #where_clause
            {
                fn text_content_attrs_mut(&mut self) -> &mut crate::svg::TextContentAttrs {
                    &mut self.text_content_attrs
                }
            }
        }
    } else {
        quote! {}
    };

    // `new()`: empty constructor, only on `#struct_name<()>`.
    let new_method = quote! {
        pub fn new() -> Self {
            Self {
                ..::std::default::Default::default()
            }
        }
    };

    // Trait docs + `on_unimplemented` diagnostics.
    let trait_doc = format!("Recipe trait for [`{struct_name}`].");
    let trait_str = trait_name.to_string();
    let msg = format!("`{{Self}}` is not a recipe of `{trait_str}`");
    let label = format!("all recipes must implement `{trait_str}`");

    quote! {
        #[doc = #trait_doc]
        #[diagnostic::on_unimplemented(
            message = #msg,
            label = #label,
        )]
        pub trait #trait_name:
            ::std::default::Default
            + ::std::clone::Clone
            + ::std::fmt::Debug
            + 'static
        {
            #trait_content
            #(fn #method_names(#param_names: &mut #field_types) {})*
        }

        impl #trait_name for () {
            #unit_content
        }

        #[doc(hidden)]
        impl<A, B> #trait_name for (A, B)
        #tuple_where
        {
            #tuple_content
            #(
                fn #method_names(#field_idents: &mut #field_types) {
                    A::#method_names(#field_idents);
                    B::#method_names(#field_idents);
                }
            )*
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

        impl<#type_param: #trait_name> #struct_name #ty_generics #where_clause {
            #content_method

            pub fn from_cookbook() -> Self {
                #content_init
                #(
                    let mut #field_idents =
                        <#field_types as ::std::default::Default>::default();
                    #type_param::#method_names(&mut #field_idents);
                )*
                Self {
                    #content_struct_field
                    #(#field_idents,)*
                    ..::std::default::Default::default()
                }
            }
        }

        impl<#type_param: #trait_name> ::std::convert::From<#type_param>
            for #struct_name #ty_generics #where_clause
        {
            fn from(_recipe: #type_param) -> Self {
                Self::from_cookbook()
            }
        }

        impl<#type_param: #trait_name> crate::oven::BakeRecipe
            for #struct_name #ty_generics #where_clause
        {
            type Baked = #struct_name<()>;

            fn bake_recipe(self) -> Self::Baked {
                #struct_name {
                    _recipe: ::std::marker::PhantomData,
                    #bake_content_field
                    #(#field_idents: self.#field_idents,)*
                }
            }
        }
    }
    .into()
}
