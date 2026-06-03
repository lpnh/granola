use granola::prelude::*;

use crate::{LOREM_IPSUM, Stats, bake_block_naive, bake_naive, measure};

pub fn div_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let span1 = HtmlSpan::new();
        let span2 = HtmlSpan::new();
        let span3 = HtmlSpan::new();
        let span4 = HtmlSpan::new();
        let span5 = HtmlSpan::new();
        let span6 = HtmlSpan::new();
        let span7 = HtmlSpan::new();

        let p1 = HtmlP::new().content(span1);
        let p2 = HtmlP::new().content(span2);
        let p3 = HtmlP::new().content(span3);
        let p4 = HtmlP::new().content(span4);
        let p5 = HtmlP::new().content(span5);
        let p6 = HtmlP::new().content(span6);
        let p7 = HtmlP::new().content(span7);

        let content = bake_block![p1, p2, p3, p4, p5, p6, p7];
        let div = HtmlDiv::new().content(content);

        div.bake()
    });

    let (naive_out, naive) = measure(|| {
        let span1 = HtmlSpan::new();
        let span2 = HtmlSpan::new();
        let span3 = HtmlSpan::new();
        let span4 = HtmlSpan::new();
        let span5 = HtmlSpan::new();
        let span6 = HtmlSpan::new();
        let span7 = HtmlSpan::new();

        let p1 = HtmlP::new().content(span1);
        let p2 = HtmlP::new().content(span2);
        let p3 = HtmlP::new().content(span3);
        let p4 = HtmlP::new().content(span4);
        let p5 = HtmlP::new().content(span5);
        let p6 = HtmlP::new().content(span6);
        let p7 = HtmlP::new().content(span7);

        let content = bake_block_naive![p1, p2, p3, p4, p5, p6, p7];
        let div = HtmlDiv::new().content(content);

        bake_naive(&div)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn div_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let span1 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span2 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span3 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span4 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span5 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span6 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span7 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");

        let p1 = HtmlP::new().content(span1).class("paragraph text-lg");
        let p2 = HtmlP::new().content(span2).class("paragraph text-lg");
        let p3 = HtmlP::new().content(span3).class("paragraph text-lg");
        let p4 = HtmlP::new().content(span4).class("paragraph text-lg");
        let p5 = HtmlP::new().content(span5).class("paragraph text-lg");
        let p6 = HtmlP::new().content(span6).class("paragraph text-lg");
        let p7 = HtmlP::new().content(span7).class("paragraph text-lg");

        let content = bake_block![p1, p2, p3, p4, p5, p6, p7];
        let div = HtmlDiv::new().content(content).id("div");

        div.bake()
    });

    let (naive_out, naive) = measure(|| {
        let span1 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span2 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span3 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span4 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span5 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span6 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span7 = HtmlSpan::new()
            .content(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");

        let p1 = HtmlP::new().content(span1).class("paragraph text-lg");
        let p2 = HtmlP::new().content(span2).class("paragraph text-lg");
        let p3 = HtmlP::new().content(span3).class("paragraph text-lg");
        let p4 = HtmlP::new().content(span4).class("paragraph text-lg");
        let p5 = HtmlP::new().content(span5).class("paragraph text-lg");
        let p6 = HtmlP::new().content(span6).class("paragraph text-lg");
        let p7 = HtmlP::new().content(span7).class("paragraph text-lg");

        let content = bake_block_naive![p1, p2, p3, p4, p5, p6, p7];
        let div = HtmlDiv::new().content(content).id("div");

        bake_naive(&div)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
