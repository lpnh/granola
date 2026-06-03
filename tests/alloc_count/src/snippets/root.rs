use granola::prelude::*;

use crate::{Stats, bake_block, bake_block_naive, bake_naive, measure};

pub fn root_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let charset = HtmlMeta::new();
        let viewport = HtmlMeta::new();
        let title = HtmlTitle::new();

        let head = HtmlHead::new().content(bake_block![charset, viewport, title]);

        let span = HtmlSpan::new();
        let p = HtmlP::new().content(span);
        let div = HtmlDiv::new().content(p);

        let body = HtmlBody::new().content(div);

        let html = HtmlRoot::new().content((head, body));
        html.bake()
    });

    let (naive_out, naive) = measure(|| {
        let charset = HtmlMeta::new();
        let viewport = HtmlMeta::new();
        let title = HtmlTitle::new();

        let head = HtmlHead::new().content(bake_block_naive![charset, viewport, title]);

        let span = HtmlSpan::new();
        let p = HtmlP::new().content(span);
        let div = HtmlDiv::new().content(p);

        let body = HtmlBody::new().content(div);

        let html = HtmlRoot::new().content((head, body));
        bake_naive(&html)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn root_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let charset: HtmlMeta = HtmlMeta::new().charset();
        let viewport: HtmlMeta = HtmlMeta::new()
            .name("viewport")
            .content("width=device-width");
        let title = HtmlTitle::new().content("Document title");

        let head = HtmlHead::new().content(bake_block![charset, viewport, title]);

        let span = HtmlSpan::new().content("hello, world!");
        let p = HtmlP::new().content(span).id("paragraph");
        let div = HtmlDiv::new().content(p).class("container");

        let body = HtmlBody::new().content(div);

        let html = HtmlRoot::new().content((head, body)).lang("en");
        html.bake()
    });

    let (naive_out, naive) = measure(|| {
        let charset: HtmlMeta = HtmlMeta::new().charset();
        let viewport: HtmlMeta = HtmlMeta::new()
            .name("viewport")
            .content("width=device-width");
        let title = HtmlTitle::new().content("Document title");

        let head = HtmlHead::new().content(bake_block_naive![charset, viewport, title]);

        let span = HtmlSpan::new().content("hello, world!");
        let p = HtmlP::new().content(span).id("paragraph");
        let div = HtmlDiv::new().content(p).class("container");

        let body = HtmlBody::new().content(div);

        let html = HtmlRoot::new().content((head, body)).lang("en");
        bake_naive(&html)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
