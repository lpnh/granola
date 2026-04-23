use granola::prelude::*;

use crate::{Stats, bake_block, bake_block_naive, bake_naive, measure};

pub fn root_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let charset: HtmlMeta = HtmlMeta::empty();
        let viewport: HtmlMeta = HtmlMeta::empty();
        let title: HtmlTitle = HtmlTitle::empty();

        let head: HtmlHead = HtmlHead::new(bake_block![charset, viewport, title]);

        let span: HtmlSpan = HtmlSpan::empty();
        let p: HtmlP = HtmlP::new(span);
        let div: HtmlDiv = HtmlDiv::new(p);

        let body: HtmlBody = HtmlBody::new(div);

        let html: HtmlRoot = HtmlRoot::new((head, body));
        html.bake()
    });

    let (naive_out, naive) = measure(|| {
        let charset: HtmlMeta = HtmlMeta::empty();
        let viewport: HtmlMeta = HtmlMeta::empty();
        let title: HtmlTitle = HtmlTitle::empty();

        let head: HtmlHead = HtmlHead::new(bake_block_naive![charset, viewport, title]);

        let span: HtmlSpan = HtmlSpan::empty();
        let p: HtmlP = HtmlP::new(span);
        let div: HtmlDiv = HtmlDiv::new(p);

        let body: HtmlBody = HtmlBody::new(div);

        let html: HtmlRoot = HtmlRoot::new((head, body));
        bake_naive(&html)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn root_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let charset: HtmlMeta = HtmlMeta::empty().charset();
        let viewport: HtmlMeta = HtmlMeta::empty()
            .name("viewport")
            .content("width=device-width");
        let title: HtmlTitle = HtmlTitle::new("Document title");

        let head: HtmlHead = HtmlHead::new(bake_block![charset, viewport, title]);

        let span: HtmlSpan = HtmlSpan::new("hello, world!");
        let p: HtmlP = HtmlP::new(span).id("paragraph");
        let div: HtmlDiv = HtmlDiv::new(p).class("container");

        let body: HtmlBody = HtmlBody::new(div);

        let html: HtmlRoot = HtmlRoot::new((head, body)).lang("en");
        html.bake()
    });

    let (naive_out, naive) = measure(|| {
        let charset: HtmlMeta = HtmlMeta::empty().charset();
        let viewport: HtmlMeta = HtmlMeta::empty()
            .name("viewport")
            .content("width=device-width");
        let title: HtmlTitle = HtmlTitle::new("Document title");

        let head: HtmlHead = HtmlHead::new(bake_block_naive![charset, viewport, title]);

        let span: HtmlSpan = HtmlSpan::new("hello, world!");
        let p: HtmlP = HtmlP::new(span).id("paragraph");
        let div: HtmlDiv = HtmlDiv::new(p).class("container");

        let body: HtmlBody = HtmlBody::new(div);

        let html: HtmlRoot = HtmlRoot::new((head, body)).lang("en");
        bake_naive(&html)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
