use granola::prelude::*;

use crate::{Stats, bake_block_naive, bake_naive, measure};

pub fn form_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let input = HtmlInput::new();
        let label = HtmlLabel::new().content(input);
        let button = HtmlButton::new();
        let form = HtmlForm::new()
            .content(bake_block![label, button])
            .method(FormMethod::Get);
        form.bake()
    });

    let (naive_out, naive) = measure(|| {
        let input = HtmlInput::new();
        let label = HtmlLabel::new().content(input);
        let button = HtmlButton::new();
        let form = HtmlForm::new()
            .content(bake_block_naive![label, button])
            .method(FormMethod::Get);
        bake_naive(&form)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn form_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let input: HtmlInput = HtmlInput::from_name("cast-wish");
        let label = HtmlLabel::new().content(bake_block!["Wish:", input]);
        let button = HtmlButton::new().content("Cast");
        let form = HtmlForm::new()
            .content(bake_block![label, button])
            .method(FormMethod::Get);
        form.bake()
    });

    let (naive_out, naive) = measure(|| {
        let input: HtmlInput = HtmlInput::from_name("cast-wish");
        let label = HtmlLabel::new().content(bake_block_naive!["Wish:", input]);
        let button = HtmlButton::new().content("Cast");
        let form = HtmlForm::new()
            .content(bake_block_naive![label, button])
            .method(FormMethod::Get);
        bake_naive(&form)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
