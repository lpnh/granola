use granola::prelude::*;

use crate::{Stats, bake_block, bake_block_naive, bake_naive, measure};

pub fn picture_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let source: HtmlSource = HtmlSource::empty();
        let img: HtmlImg = HtmlImg::empty();
        let picture: HtmlPicture = HtmlPicture::new(bake_block![source, img]);
        picture.bake()
    });

    let (naive_out, naive) = measure(|| {
        let source: HtmlSource = HtmlSource::empty();
        let img: HtmlImg = HtmlImg::empty();
        let picture: HtmlPicture = HtmlPicture::new(bake_block_naive![source, img]);
        bake_naive(&picture)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn picture_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let source: HtmlSource = HtmlSource::empty()
            .srcset("logo-wide.png")
            .media("(width >= 600px)");
        let img: HtmlImg = HtmlImg::new("logo-narrow.png", "logo");
        let picture: HtmlPicture = HtmlPicture::new(bake_block![source, img]);
        picture.bake()
    });

    let (naive_out, naive) = measure(|| {
        let source: HtmlSource = HtmlSource::empty()
            .srcset("logo-wide.png")
            .media("(width >= 600px)");
        let img: HtmlImg = HtmlImg::new("logo-narrow.png", "logo");
        let picture: HtmlPicture = HtmlPicture::new(bake_block_naive![source, img]);
        bake_naive(&picture)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
