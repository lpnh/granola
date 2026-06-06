use granola::prelude::*;

use crate::{Stats, bake_block_naive, bake_naive, measure};

pub fn fieldset_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let legend = HtmlLegend::new();
        let input = HtmlInput::new();
        let label = HtmlLabel::new();
        let fieldset = HtmlFieldset::new().content(bake_block![legend, input, label]);
        fieldset.bake()
    });

    let (naive_out, naive) = measure(|| {
        let legend = HtmlLegend::new();
        let input = HtmlInput::new();
        let label = HtmlLabel::new();
        let fieldset = HtmlFieldset::new().content(bake_block_naive![legend, input, label]);
        bake_naive(&fieldset)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn fieldset_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let legend = HtmlLegend::new().content("To be, or not to be?");
        let input: HtmlInput = HtmlInput::from_type(InputType::Checkbox)
            .id("chbx")
            .name("to-be")
            .value("dunno");
        let label = HtmlLabel::new()
            .content("That is the question")
            .for_id("chbx");
        let fieldset = HtmlFieldset::new().content(bake_block![legend, input, label]);
        fieldset.bake()
    });

    let (naive_out, naive) = measure(|| {
        let legend = HtmlLegend::new().content("To be, or not to be?");
        let input: HtmlInput = HtmlInput::from_type(InputType::Checkbox)
            .id("chbx")
            .name("to-be")
            .value("dunno");
        let label = HtmlLabel::new()
            .content("That is the question")
            .for_id("chbx");
        let fieldset = HtmlFieldset::new().content(bake_block_naive![legend, input, label]);
        bake_naive(&fieldset)
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
