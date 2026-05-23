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
            /// Panics if [`askama::Template::render`] returns an error.
            /// Writing into a [`String`] via [`core::fmt::Write`] is infallible,
            /// so the only way this fails is if the template itself errors.
            /// See [`askama::Error`].
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

/// Derive macro for element recipe.
///
/// Field-driven: generates one recipe hook per struct field (excluding `_recipe`).
/// Special fields:
/// - `content`: when `#[recipe(content = DefaultType)]` is set, uses an associated type
///   with `Into<DefaultType>` bound; also generates `new()`.
/// - `global_attrs`, `global_aria_attrs`, `custom_data_attrs`, `event_handlers`:
///   generate `Has*` impls and `empty()`.
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

    // All fields except _recipe, and except `content` when it uses an associated type
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

    let has_global_attrs = field_idents.iter().any(|i| i == "global_attrs");

    // Trait: optional Content associated type + content_recipe
    let trait_content = if let Some(ref content_type) = default_content_type {
        quote! {
            type Content:
                ::askama::FastWritable
                + ::std::default::Default
                + ::std::clone::Clone
                + ::std::fmt::Debug
                + crate::oven::BakeInto<#content_type>
                = #content_type;
            fn content_recipe(_content: &mut Self::Content) {}
        }
    } else {
        quote! {}
    };

    // (A, B) impl: where clause and optional Content type + content_recipe
    let tuple_where = if has_content {
        quote! { where A: #trait_name, B: #trait_name<Content = A::Content>, }
    } else {
        quote! { where A: #trait_name, B: #trait_name, }
    };

    let tuple_content = if has_content {
        quote! {
            type Content = A::Content;
            fn content_recipe(content: &mut Self::Content) {
                A::content_recipe(content);
                B::content_recipe(content);
            }
        }
    } else {
        quote! {}
    };

    // Has* impls for HTML-specific fields
    let global_attrs_impl = if has_global_attrs {
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
    let global_aria_attrs_impl = if field_idents.iter().any(|i| i == "global_aria_attrs") {
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
    let custom_data_attrs_impl = if field_idents.iter().any(|i| i == "custom_data_attrs") {
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
    let event_handlers_impl = if field_idents.iter().any(|i| i == "event_handlers") {
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

    // from_recipe(): content initialization
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

    // empty() only for HTML-like structs (those with global_attrs)
    let empty_method = if has_global_attrs {
        quote! {
            pub fn empty() -> Self {
                Self {
                    ..::std::default::Default::default()
                }
            }
        }
    } else {
        quote! {}
    };

    // new() only when content is present
    let new_method = if has_content {
        quote! {
            pub fn new(content: impl ::std::convert::Into<#type_param::Content>) -> Self {
                let mut content = content.into();
                #type_param::content_recipe(&mut content);
                #(
                    let mut #field_idents =
                        <#field_types as ::std::default::Default>::default();
                    #type_param::#method_names(&mut #field_idents);
                )*
                Self {
                    content,
                    #(#field_idents,)*
                    ..::std::default::Default::default()
                }
            }
        }
    } else {
        quote! {}
    };

    // bake_recipe(): content field
    let bake_content_field = if has_content {
        quote! { content: crate::oven::BakeInto::bake_into(self.content), }
    } else {
        quote! {}
    };

    // on_unimplemented messages for the recipe trait (the `{Self}` braces are doubled
    // so they pass through `format!` and reach the diagnostic as literal placeholders).
    let trait_str = trait_name.to_string();
    let chain_msg = format!("`{{Self}}` is not a valid step in a `{trait_str}` recipe chain");
    let chain_label = format!("`{{Self}}` must implement `{trait_str}`");

    // Doc contract for the generated trait.
    let trait_docs = if has_content {
        quote! {
            #[doc = "Recipe trait generated by `#[derive(Recipe)]`. Implement it on a unit struct to define a recipe; compose recipes with `rec![A, B, ...]`."]
            #[doc = ""]
            #[doc = "Overriding `type Content` requires `impl From<YourContent> for DefaultContent`, so the recipe can be baked back into `Foo<()>`."]
        }
    } else {
        quote! {
            #[doc = "Recipe trait generated by `#[derive(Recipe)]`. Implement it on a unit struct to define a recipe; compose recipes with `rec![A, B, ...]`."]
        }
    };

    quote! {
        #trait_docs
        #[diagnostic::on_unimplemented(
            message = #chain_msg,
            label = #chain_label,
            note = "recipe steps compose via `rec![A, B, ...]` (nested tuples); each step and the `()` base must implement this trait"
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

        impl #trait_name for () {}

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

        impl<#type_param: #trait_name> #struct_name #ty_generics #where_clause {
            #empty_method

            pub fn from_recipe() -> Self {
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

            #new_method
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
