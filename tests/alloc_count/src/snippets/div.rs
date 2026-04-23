use granola::prelude::*;

use crate::{LOREM_IPSUM, Stats, bake_block_naive, bake_naive, measure};

pub fn div_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let span1: HtmlSpan = HtmlSpan::empty();
        let span2: HtmlSpan = HtmlSpan::empty();
        let span3: HtmlSpan = HtmlSpan::empty();
        let span4: HtmlSpan = HtmlSpan::empty();
        let span5: HtmlSpan = HtmlSpan::empty();
        let span6: HtmlSpan = HtmlSpan::empty();
        let span7: HtmlSpan = HtmlSpan::empty();

        let p1: HtmlP = HtmlP::new(span1);
        let p2: HtmlP = HtmlP::new(span2);
        let p3: HtmlP = HtmlP::new(span3);
        let p4: HtmlP = HtmlP::new(span4);
        let p5: HtmlP = HtmlP::new(span5);
        let p6: HtmlP = HtmlP::new(span6);
        let p7: HtmlP = HtmlP::new(span7);

        let content = bake_block![p1, p2, p3, p4, p5, p6, p7];
        let div: HtmlDiv = HtmlDiv::new(content);

        div.bake()
    });

    let (naive_out, naive) = measure(|| {
        let span1: HtmlSpan = HtmlSpan::empty();
        let span2: HtmlSpan = HtmlSpan::empty();
        let span3: HtmlSpan = HtmlSpan::empty();
        let span4: HtmlSpan = HtmlSpan::empty();
        let span5: HtmlSpan = HtmlSpan::empty();
        let span6: HtmlSpan = HtmlSpan::empty();
        let span7: HtmlSpan = HtmlSpan::empty();

        let p1: HtmlP = HtmlP::new(span1);
        let p2: HtmlP = HtmlP::new(span2);
        let p3: HtmlP = HtmlP::new(span3);
        let p4: HtmlP = HtmlP::new(span4);
        let p5: HtmlP = HtmlP::new(span5);
        let p6: HtmlP = HtmlP::new(span6);
        let p7: HtmlP = HtmlP::new(span7);

        let content = bake_block_naive![p1, p2, p3, p4, p5, p6, p7];
        let div: HtmlDiv = HtmlDiv::new(content);

        bake_naive(&div)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn div_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let span1: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span2: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span3: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span4: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span5: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span6: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span7: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");

        let p1: HtmlP = HtmlP::new(span1).class("paragraph text-lg");
        let p2: HtmlP = HtmlP::new(span2).class("paragraph text-lg");
        let p3: HtmlP = HtmlP::new(span3).class("paragraph text-lg");
        let p4: HtmlP = HtmlP::new(span4).class("paragraph text-lg");
        let p5: HtmlP = HtmlP::new(span5).class("paragraph text-lg");
        let p6: HtmlP = HtmlP::new(span6).class("paragraph text-lg");
        let p7: HtmlP = HtmlP::new(span7).class("paragraph text-lg");

        let content = bake_block![p1, p2, p3, p4, p5, p6, p7];
        let div: HtmlDiv = HtmlDiv::new(content).id("div");

        div.bake()
    });

    let (naive_out, naive) = measure(|| {
        let span1: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span2: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span3: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span4: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span5: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span6: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");
        let span7: HtmlSpan = HtmlSpan::new(LOREM_IPSUM)
            .id("lorem-ipsum")
            .aria_label("Lorem ipsum");

        let p1: HtmlP = HtmlP::new(span1).class("paragraph text-lg");
        let p2: HtmlP = HtmlP::new(span2).class("paragraph text-lg");
        let p3: HtmlP = HtmlP::new(span3).class("paragraph text-lg");
        let p4: HtmlP = HtmlP::new(span4).class("paragraph text-lg");
        let p5: HtmlP = HtmlP::new(span5).class("paragraph text-lg");
        let p6: HtmlP = HtmlP::new(span6).class("paragraph text-lg");
        let p7: HtmlP = HtmlP::new(span7).class("paragraph text-lg");

        let content = bake_block_naive![p1, p2, p3, p4, p5, p6, p7];
        let div: HtmlDiv = HtmlDiv::new(content).id("div");

        bake_naive(&div)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
