use granola::prelude::*;

use crate::{Stats, bake_inline, bake_inline_naive, bake_naive, measure};

pub fn ruby_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let opening_rp = HtmlRp::new();
        let rt = HtmlRt::new();
        let closing_rp = HtmlRp::new();
        let tori = bake_inline!["鳥", opening_rp, rt, closing_rp];
        let ruby = HtmlRuby::new().content(tori);
        ruby.bake()
    });

    let (naive_out, naive) = measure(|| {
        let opening_rp = HtmlRp::new();
        let rt = HtmlRt::new();
        let closing_rp = HtmlRp::new();
        let tori = bake_inline_naive!["鳥", opening_rp, rt, closing_rp];
        let ruby = HtmlRuby::new().content(tori);
        bake_naive(&ruby)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn ruby_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let opening_rp = HtmlRp::new().content("(");
        let rt = HtmlRt::new().content("とり");
        let closing_rp = HtmlRp::new().content(")");
        let tori = bake_inline!["鳥", opening_rp, rt, closing_rp];
        let ruby = HtmlRuby::new().content(tori);
        ruby.bake()
    });

    let (naive_out, naive) = measure(|| {
        let opening_rp = HtmlRp::new().content("(");
        let rt = HtmlRt::new().content("とり");
        let closing_rp = HtmlRp::new().content(")");
        let tori = bake_inline_naive!["鳥", opening_rp, rt, closing_rp];
        let ruby = HtmlRuby::new().content(tori);
        bake_naive(&ruby)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
