use granola::prelude::*;

use crate::{Stats, bake_block_naive, measure};

pub fn dialog_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let open_button = HtmlButton::new();
        let close_button = HtmlButton::new();
        let dialog = HtmlDialog::new().content(bake_block!["Hello, there!", close_button]);
        bake_block![open_button, dialog]
    });

    let (naive_out, naive) = measure(|| {
        let open_button = HtmlButton::new();
        let close_button = HtmlButton::new();
        let dialog = HtmlDialog::new().content(bake_block_naive!["Hello, there!", close_button]);
        bake_block_naive![open_button, dialog]
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn dialog_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let open_button = HtmlButton::new()
            .content("open dialog")
            .popovertarget("modal_popover");
        let close_button = HtmlButton::new()
            .content("Close")
            .popovertarget("modal_popover")
            .popovertargetaction("hide");
        let dialog = HtmlDialog::new()
            .content(bake_block!["Hello, there!", close_button])
            .id("modal_popover")
            .popover("auto");
        bake_block![open_button, dialog]
    });

    let (naive_out, naive) = measure(|| {
        let open_button = HtmlButton::new()
            .content("open dialog")
            .popovertarget("modal_popover");
        let close_button = HtmlButton::new()
            .content("Close")
            .popovertarget("modal_popover")
            .popovertargetaction("hide");
        let dialog = HtmlDialog::new()
            .content(bake_block_naive!["Hello, there!", close_button])
            .id("modal_popover")
            .popover("auto");
        bake_block_naive![open_button, dialog]
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
