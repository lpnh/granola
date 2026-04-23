use granola::prelude::*;

use crate::{Stats, bake_block_naive, measure};

pub fn dialog_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let open_button: HtmlButton = HtmlButton::empty();
        let close_button: HtmlButton = HtmlButton::empty();
        let dialog: HtmlDialog = HtmlDialog::new(bake_block!["Hello, there!", close_button]);
        bake_block![open_button, dialog]
    });

    let (naive_out, naive) = measure(|| {
        let open_button: HtmlButton = HtmlButton::empty();
        let close_button: HtmlButton = HtmlButton::empty();
        let dialog: HtmlDialog = HtmlDialog::new(bake_block_naive!["Hello, there!", close_button]);
        bake_block_naive![open_button, dialog]
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn dialog_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let open_button: HtmlButton = HtmlButton::new("open dialog").popovertarget("modal_popover");
        let close_button: HtmlButton = HtmlButton::new("Close")
            .popovertarget("modal_popover")
            .popovertargetaction("hide");
        let dialog: HtmlDialog = HtmlDialog::new(bake_block!["Hello, there!", close_button])
            .id("modal_popover")
            .popover("auto");
        bake_block![open_button, dialog]
    });

    let (naive_out, naive) = measure(|| {
        let open_button: HtmlButton = HtmlButton::new("open dialog").popovertarget("modal_popover");
        let close_button: HtmlButton = HtmlButton::new("Close")
            .popovertarget("modal_popover")
            .popovertargetaction("hide");
        let dialog: HtmlDialog = HtmlDialog::new(bake_block_naive!["Hello, there!", close_button])
            .id("modal_popover")
            .popover("auto");
        bake_block_naive![open_button, dialog]
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}
