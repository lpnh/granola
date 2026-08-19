#![allow(unused_qualifications)]

use gungraun::prelude::*;
use std::hint::black_box;

use granola::prelude::HtmlDocument;

main!(library_benchmark_groups = [html]);

library_benchmark_group!(
    name = html,
    benchmarks = [html_standard, html_macros, html_recipes]
);

#[library_benchmark(setup = html_fixture_standard)]
fn html_standard(snippet: HtmlDocument) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = html_fixture_macros)]
fn html_macros(snippet: HtmlDocument) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = html_fixture_recipes)]
fn html_recipes(snippet: HtmlDocument) -> String {
    black_box(snippet.bake())
}

pub fn html_fixture_standard() -> HtmlDocument {
    fixtures::standard::page()
}

pub fn html_fixture_macros() -> HtmlDocument {
    fixtures::macros::page()
}

pub fn html_fixture_recipes() -> HtmlDocument {
    fixtures::recipes::page()
}
