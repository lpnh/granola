#![allow(unused_qualifications)]

use gungraun::prelude::*;
use std::hint::black_box;

use granola::{
    homemade::Homemade,
    prelude::{CssStylesheet, HtmlDocument},
};

main!(library_benchmark_groups = [html, css]);

library_benchmark_group!(
    name = html,
    benchmarks = [html_standard, html_macros, html_recipes]
);

#[library_benchmark(setup = html_fixture_standard)]
fn html_standard(snippet: HtmlDocument<Homemade>) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = html_fixture_macros)]
fn html_macros(snippet: HtmlDocument<Homemade>) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = html_fixture_recipes)]
fn html_recipes(snippet: HtmlDocument<Homemade>) -> String {
    black_box(snippet.bake())
}

library_benchmark_group!(
    name = css,
    benchmarks = [css_standard, css_macros, css_recipes]
);

#[library_benchmark(setup = css_fixture_standard)]
fn css_standard(snippet: CssStylesheet) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = css_fixture_macros)]
fn css_macros(snippet: CssStylesheet) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = css_fixture_recipes)]
fn css_recipes(snippet: CssStylesheet) -> String {
    black_box(snippet.bake())
}

pub fn html_fixture_standard() -> HtmlDocument<Homemade> {
    fixtures::standard::page()
}

pub fn html_fixture_macros() -> HtmlDocument<Homemade> {
    fixtures::macros::page()
}

pub fn html_fixture_recipes() -> HtmlDocument<Homemade> {
    fixtures::recipes::page()
}

pub fn css_fixture_standard() -> CssStylesheet {
    fixtures::standard::style()
}

pub fn css_fixture_macros() -> CssStylesheet {
    fixtures::macros::style()
}

pub fn css_fixture_recipes() -> CssStylesheet {
    fixtures::recipes::style()
}
