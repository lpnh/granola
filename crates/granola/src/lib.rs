#![feature(associated_type_defaults)]

pub use granola_derive::{Granola, Recipe};

pub mod cookbook;
pub mod css;
pub mod filters;
pub mod homemade;
pub mod html;
pub mod oven;
pub mod template;

pub mod prelude {
    pub use super::{
        Granola, Recipe,
        css::*,
        html::*,
        oven::{BakeFrom, BakeRecipe},
    };
    pub use crate::{bake_block, bake_inline, bake_newline, cookbook};
}

pub mod macros {
    // css
    pub use crate::{at_rule, rule, stylesheet};
    // html
    pub use crate::{
        a, abbr, address, area, article, aside, audio, b, base, bdi, bdo, blockquote, body, br,
        button, canvas, caption, cite, code, col, colgroup, data, datalist, dd, del, details, dfn,
        dialog, div, dl, doctype, dt, em, embed, fieldset, figcaption, figure, footer, form, h1,
        h2, h3, h4, head, header, hgroup, hr, i, iframe, img, input, ins, kbd, label, legend, li,
        link, main, map, mark, menu, meta, meter, nav, noscript, object, ol, optgroup, option,
        output, p, picture, pre, progress, q, root, rp, rt, ruby, s, samp, script, search, section,
        select, slot, small, source, span, strong, style, sub, summary, sup, table, tbody, td,
        template, textarea, tfoot, th, thead, time, title, tr, track, u, ul, var, video, wbr,
    };
}
