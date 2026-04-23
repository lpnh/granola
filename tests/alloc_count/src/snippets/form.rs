use granola::prelude::*;

use crate::{Stats, bake_block_naive, bake_naive, measure};

pub fn form_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let input: HtmlInput = HtmlInput::empty();
        let label: HtmlLabel = HtmlLabel::new(input);
        let button: HtmlButton = HtmlButton::empty();
        let form: HtmlForm = HtmlForm::new(bake_block![label, button]).method("get");
        form.bake()
    });

    let (naive_out, naive) = measure(|| {
        let input: HtmlInput = HtmlInput::empty();
        let label: HtmlLabel = HtmlLabel::new(input);
        let button: HtmlButton = HtmlButton::empty();
        let form: HtmlForm = HtmlForm::new(bake_block_naive![label, button]).method("get");
        bake_naive(&form)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn form_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let input: HtmlInput = HtmlInput::new("cast-wish");
        let label: HtmlLabel = HtmlLabel::new(bake_block!["Wish:", input]);
        let button: HtmlButton = HtmlButton::new("Cast");
        let form: HtmlForm = HtmlForm::new(bake_block![label, button]).method("get");
        form.bake()
    });

    let (naive_out, naive) = measure(|| {
        let input: HtmlInput = HtmlInput::new("cast-wish");
        let label: HtmlLabel = HtmlLabel::new(bake_block_naive!["Wish:", input]);
        let button: HtmlButton = HtmlButton::new("Cast");
        let form: HtmlForm = HtmlForm::new(bake_block_naive![label, button]).method("get");
        bake_naive(&form)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
