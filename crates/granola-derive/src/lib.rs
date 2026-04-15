use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Derive macro for templates.
///
/// Implements:
/// - `bake()` via `askama::Template::render_into`.
/// - `From<T> for Cow<'static, str>` via `bake()`.
#[proc_macro_derive(Granola)]
pub fn granola_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn bake(&self) -> ::std::string::String {
                let mut buf = ::std::string::String::with_capacity(
                    <Self as ::askama::Template>::SIZE_HINT,
                );
                let _ = ::askama::Template::render_into(self, &mut buf);
                buf
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
