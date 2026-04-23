use granola::prelude::*;

use crate::{Stats, bake_inline, bake_inline_naive, bake_naive, measure};

pub fn ruby_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let opening_rp: HtmlRp = HtmlRp::empty();
        let rt: HtmlRt = HtmlRt::empty();
        let closing_rp: HtmlRp = HtmlRp::empty();
        let tori = bake_inline!["鳥", opening_rp, rt, closing_rp];
        let ruby: HtmlRuby = HtmlRuby::new(tori);
        ruby.bake()
    });

    let (naive_out, naive) = measure(|| {
        let opening_rp: HtmlRp = HtmlRp::empty();
        let rt: HtmlRt = HtmlRt::empty();
        let closing_rp: HtmlRp = HtmlRp::empty();
        let tori = bake_inline_naive!["鳥", opening_rp, rt, closing_rp];
        let ruby: HtmlRuby = HtmlRuby::new(tori);
        bake_naive(&ruby)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn ruby_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let opening_rp: HtmlRp = HtmlRp::new("(");
        let rt: HtmlRt = HtmlRt::new("とり");
        let closing_rp: HtmlRp = HtmlRp::new(")");
        let tori = bake_inline!["鳥", opening_rp, rt, closing_rp];
        let ruby: HtmlRuby = HtmlRuby::new(tori);
        ruby.bake()
    });

    let (naive_out, naive) = measure(|| {
        let opening_rp: HtmlRp = HtmlRp::new("(");
        let rt: HtmlRt = HtmlRt::new("とり");
        let closing_rp: HtmlRp = HtmlRp::new(")");
        let tori = bake_inline_naive!["鳥", opening_rp, rt, closing_rp];
        let ruby: HtmlRuby = HtmlRuby::new(tori);
        bake_naive(&ruby)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
