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
        a, abbr, area, audio, b, base, bdi, bdo, blockquote, body, br, button, canvas, cite, code,
        datalist, dd, del, details, dialog, div, dl, doctype, dt, embed, fieldset, figcaption,
        figure, form, head, hr, iframe, img, input, ins, label, legend, li, link, map, menu, meta,
        meter, noscript, object, ol, optgroup, option, output, p, picture, pre, progress, root,
        script, select, source, style, summary, textarea, title, track, ul, video, wbr,
    };
}
