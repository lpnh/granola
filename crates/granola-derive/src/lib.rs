use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
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
    let input: DeriveInput = syn::parse_macro_input!(input);
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

/// Derive macro for HTML elements.
///
/// Implements the mutable accessors for:
/// - `GlobalAttrs`
/// - `DataAttrs`
/// - `EventHandlers`
/// - `GlobalAriaAttrs`
#[proc_macro_derive(MutAttrs)]
pub fn granola_attrs_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics crate::html::HasGlobalAttrs for #name #ty_generics #where_clause {
            fn global_attrs_mut(&mut self) -> &mut crate::html::GlobalAttrs {
                &mut self.global_attrs
            }
        }

        impl #impl_generics crate::html::HasDataAttrs for #name #ty_generics #where_clause {
            fn data_attrs_mut(&mut self) -> &mut crate::html::DataAttrs {
                &mut self.data_attrs
            }
        }

        impl #impl_generics crate::html::HasEventHandlers for #name #ty_generics #where_clause {
            fn event_handlers_mut(&mut self) -> &mut crate::html::EventHandlers {
                &mut self.event_handlers
            }
        }

        impl #impl_generics crate::html::HasGlobalAriaAttrs for #name #ty_generics #where_clause {
            fn global_aria_attrs_mut(&mut self) -> &mut crate::html::GlobalAriaAttrs {
                &mut self.global_aria_attrs
            }
        }
    }
    .into()
}

struct RecipeArgs {
    name: Ident,
    content: Option<Type>,
}

impl Parse for RecipeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name: Option<Ident> = None;
        let mut content: Option<Type> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_span = key.span();
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "name" => name = Some(input.parse()?),
                "content" => content = Some(input.parse()?),
                _ => return Err(syn::Error::new(key_span, format!("unknown key `{key}`"))),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(RecipeArgs {
            name: name.ok_or_else(|| {
                syn::Error::new(
                    Span::call_site(),
                    "`name` is required in #[recipe(name = ...)]",
                )
            })?,
            content,
        })
    }
}

/// Derive macro for HTML element recipe.
#[proc_macro_derive(Recipe, attributes(recipe))]
pub fn recipe_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let recipe_attr = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("recipe"))
        .expect("missing #[recipe(...)] attribute");

    let args: RecipeArgs = recipe_attr
        .parse_args()
        .expect("failed to parse #[recipe(...)]");

    let trait_name = &args.name;

    if let Some(default_content) = args.content {
        quote! {
            pub trait #trait_name:
                ::std::default::Default
                + ::std::clone::Clone
                + ::std::fmt::Debug
                + 'static
            {
                type Content:
                    ::askama::FastWritable
                    + ::std::default::Default
                    + ::std::clone::Clone
                    + ::std::fmt::Debug = #default_content;

                fn content_recipe(_content: &mut Self::Content) {}

                fn decoration_recipe<R: #trait_name>(element: #struct_name<R>) -> #struct_name<R> {
                    element
                }
            }

            impl #trait_name for () {}

            impl<A, B> #trait_name for (A, B)
            where
                A: #trait_name,
                B: #trait_name<Content = A::Content>,
            {
                type Content = A::Content;

                fn content_recipe(content: &mut Self::Content) {
                    A::content_recipe(content);
                    B::content_recipe(content);
                }

                fn decoration_recipe<R: #trait_name>(element: #struct_name<R>) -> #struct_name<R> {
                    B::decoration_recipe(A::decoration_recipe(element))
                }
            }

            impl #impl_generics #struct_name #ty_generics #where_clause {
                pub fn empty() -> Self {
                    Self {
                        ..::std::default::Default::default()
                    }
                }

                pub fn from_recipe() -> Self {
                    let mut content = <M::Content as ::std::default::Default>::default();
                    M::content_recipe(&mut content);
                    let element = Self {
                        content,
                        ..::std::default::Default::default()
                    };
                    M::decoration_recipe(element)
                }

                pub fn new(content: impl ::std::convert::Into<M::Content>) -> Self {
                    let mut content = content.into();
                    M::content_recipe(&mut content);
                    let element = Self {
                        content,
                        ..::std::default::Default::default()
                    };
                    M::decoration_recipe(element)
                }
            }
        }
        .into()
    } else {
        quote! {
            pub trait #trait_name:
                ::std::default::Default
                + ::std::clone::Clone
                + ::std::fmt::Debug
                + 'static
            {
                fn decoration_recipe<R: #trait_name>(element: #struct_name<R>) -> #struct_name<R> {
                    element
                }
            }

            impl #trait_name for () {}

            impl<A: #trait_name, B: #trait_name> #trait_name for (A, B) {
                fn decoration_recipe<R: #trait_name>(element: #struct_name<R>) -> #struct_name<R> {
                    B::decoration_recipe(A::decoration_recipe(element))
                }
            }

            impl #impl_generics #struct_name #ty_generics #where_clause {
                pub fn empty() -> Self {
                    Self {
                        ..::std::default::Default::default()
                    }
                }

                pub fn from_recipe() -> Self {
                    let element = Self {
                        ..::std::default::Default::default()
                    };
                    M::decoration_recipe(element)
                }
            }
        }
        .into()
    }
}
