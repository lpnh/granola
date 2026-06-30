#![feature(associated_type_defaults)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate self as granola;

pub use granola_derive::{Granola, Recipe};

pub mod css;
pub mod filters;
pub mod homemade;
pub mod html;
pub mod oven;
pub mod recipes;
pub mod svg;

#[cfg(feature = "pretty")]
#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
pub mod pretty;

pub mod prelude {
    pub use super::{Granola, Recipe, css::*, html::*, oven::BakeRecipe, recipes, svg::*};
    pub use crate::{bake, bake_block, recipe_boilerplate};
}

pub mod macros {
    // css
    pub use crate::{
        at_rule, complex_selector, compound_selector, declaration, declarations_block, rule,
        selectors_list, simple_selector, stylesheet, type_selector,
    };
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
    // svg
    pub use crate::{path, rect, svg, text};
}
