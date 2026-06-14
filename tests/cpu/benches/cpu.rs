#![allow(unused_qualifications)]

use gungraun::{Cachegrind, CachegrindMetric, prelude::*};
use std::hint::black_box;

use granola::{homemade::Homemade, prelude::*, recipes::Preflight};

// soft = 3% band on `Ir` vs the saved baseline
// hard = absolute ceiling, measured ×1.05 (Ir) / ×1.25 (I1mr)
fn limits(ir: u64, i1mr: u64) -> LibraryBenchmarkConfig {
    let mut config = LibraryBenchmarkConfig::default();
    config.tool(
        Cachegrind::default()
            .soft_limits([(CachegrindMetric::Ir, 3.0)])
            .hard_limits([(CachegrindMetric::Ir, ir), (CachegrindMetric::I1mr, i1mr)]),
    );
    config
}

main!(library_benchmark_groups = [empty_elements, html_snippets, css_snippets, recipe_snippets]);

library_benchmark_group!(
    name = empty_elements,
    benchmarks = [br, canvas, figcaption, hr, map, pre]
);

#[library_benchmark(config = limits(2750, 230))]
fn br() -> String {
    black_box(HtmlBr::new().bake())
}

#[library_benchmark(config = limits(3000, 260))]
fn canvas() -> String {
    black_box(HtmlCanvas::new().bake())
}

#[library_benchmark(config = limits(2950, 250))]
fn figcaption() -> String {
    black_box(HtmlFigcaption::new().bake())
}

#[library_benchmark(config = limits(2750, 230))]
fn hr() -> String {
    black_box(HtmlHr::new().bake())
}

#[library_benchmark(config = limits(3050, 260))]
fn map() -> String {
    black_box(HtmlMap::new().bake())
}

#[library_benchmark(config = limits(2900, 240))]
fn pre() -> String {
    black_box(HtmlPre::new().bake())
}

library_benchmark_group!(
    name = html_snippets,
    benchmarks = [a, article, meter, optgroup, root, table, ul]
);

#[library_benchmark(setup = corpus::a, config = limits(3600, 250))]
fn a(snippet: HtmlA) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::article, config = limits(6300, 270))]
fn article(snippet: HtmlArticle) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::meter, config = limits(5900, 290))]
fn meter(snippet: HtmlMeter) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::optgroup, config = limits(13900, 290))]
fn optgroup(snippet: HtmlOptgroup) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::root, config = limits(8000, 290))]
fn root(snippet: HtmlRoot) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::table, config = limits(14000, 300))]
fn table(snippet: HtmlTable) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::ul, config = limits(13450, 280))]
fn ul(snippet: HtmlUl) -> String {
    black_box(snippet.bake())
}

library_benchmark_group!(
    name = css_snippets,
    benchmarks = [rule, at_rule, stylesheet]
);

#[library_benchmark(setup = corpus::rule, config = limits(2200, 140))]
fn rule(snippet: CssRule) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::at_rule, config = limits(2500, 140))]
fn at_rule(snippet: CssAtRule) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::stylesheet, config = limits(3000, 160))]
fn stylesheet(snippet: CssStylesheet) -> String {
    black_box(snippet.bake())
}

library_benchmark_group!(
    name = recipe_snippets,
    benchmarks = [document_homemade, stylesheet_preflight]
);

#[library_benchmark(setup = corpus::document_homemade, config = limits(56100,
440))]
fn document_homemade(snippet: HtmlDocument<Homemade>) -> String {
    black_box(snippet.bake())
}

#[library_benchmark(setup = corpus::stylesheet_preflight, config =
limits(109700, 200))]
fn stylesheet_preflight(snippet: CssStylesheet<Preflight>) -> String {
    black_box(snippet.bake())
}
