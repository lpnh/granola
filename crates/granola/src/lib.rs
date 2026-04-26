#![feature(associated_type_defaults)]

pub use granola_derive::{Granola, MutAttrs};

pub mod filters;
pub mod html;
pub mod oven;

pub mod prelude {
    pub use super::{Granola, MutAttrs, html::*};
    pub use crate::{bake_block, bake_inline, bake_newline};
}

pub mod macros {
    pub use crate::{
        button, datalist, del, embed, fieldset, form, iframe, input, ins, label, legend, meter,
        object, optgroup, option, output, picture, progress, select, source, textarea,
    };
}
