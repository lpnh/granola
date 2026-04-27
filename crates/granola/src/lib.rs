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
        area, audio, base, body, button, datalist, del, details, dialog, doctype, embed, fieldset,
        form, head, iframe, img, input, ins, label, legend, link, map, meta, meter, object,
        optgroup, option, output, picture, progress, root, select, source, style, summary,
        textarea, title, track, video,
    };
}
