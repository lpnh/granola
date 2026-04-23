use granola::prelude::*;

use crate::{Stats, bake_block, bake_block_naive, bake_newline, bake_newline_naive, measure};

pub fn pre_empty() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let ferris_ascii = bake_newline!("");
        let pre: HtmlPre = HtmlPre::new(ferris_ascii);
        let url: HtmlA = HtmlA::empty();
        let cite: HtmlCite = HtmlCite::new(url);
        bake_block![pre, cite]
    });

    let (naive_out, naive) = measure(|| {
        let ferris_ascii = bake_newline_naive!("");
        let pre: HtmlPre = HtmlPre::new(ferris_ascii);
        let url: HtmlA = HtmlA::empty();
        let cite: HtmlCite = HtmlCite::new(url);
        bake_block_naive![pre, cite]
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

pub fn pre_example() -> (Stats, Stats) {
    let (opt_out, opt) = measure(|| {
        let ferris_ascii = bake_newline!(FERRIS);
        let pre: HtmlPre = HtmlPre::new(ferris_ascii)
            .role("img")
            .aria_label("ASCII ferris");
        let url: HtmlA = HtmlA::new("ferris-says").href("https://crates.io/crates/ferris-says");
        let cite: HtmlCite = HtmlCite::new(url);
        bake_block![pre, cite]
    });

    let (naive_out, naive) = measure(|| {
        let ferris_ascii = bake_newline_naive!(FERRIS);
        let pre: HtmlPre = HtmlPre::new(ferris_ascii)
            .role("img")
            .aria_label("ASCII ferris");
        let url: HtmlA = HtmlA::new("ferris-says").href("https://crates.io/crates/ferris-says");
        let cite: HtmlCite = HtmlCite::new(url);
        bake_block_naive![pre, cite]
    });

    assert_eq!(opt_out, naive_out);

    (opt, naive)
}

const FERRIS: &str = r#" __________________________
&lt; Hello fellow Rustaceans! &gt;
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \"#;
