#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]

use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    fmt::Write,
};

#[derive(Clone, Copy)]
struct Meter {
    enabled: bool,
    allocs: usize,
    churn: usize,
    live: usize,
    peak: usize,
}

impl Meter {
    const ZERO: Self = Self {
        enabled: false,
        allocs: 0,
        churn: 0,
        live: 0,
        peak: 0,
    };
}

fn main() {
    println!("{}", screenshot(html_snippets()));
    println!("{}", screenshot(css_snippets()));
    println!("{}", screenshot(recipe_snippets()));
}

pub fn screenshot(snippets: &[Snippet]) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{:>20}  {:>6}  {:>8}  {:>8}",
        "snippet", "allocs", "churn", "peak",
    );
    let _ = writeln!(
        out,
        "{:>20}  {:>6}  {:>8}  {:>8}",
        "-------", "------", "-----", "----",
    );
    for (label, run) in snippets {
        let s = run();
        let _ = writeln!(
            out,
            "{label:>20}  {:>6}  {:>8}  {:>8}",
            s.allocs, s.churn, s.peak
        );
    }
    out
}

thread_local! {
    static METER: Cell<Meter> = const { Cell::new(Meter::ZERO) };
}

#[global_allocator]
static COUNTING: CountingAllocator = CountingAllocator;

struct CountingAllocator;

// don't override `realloc`: the default decomposes into alloc/dealloc, so
// growth stays metered. delegating to `System.realloc` would erase reallocation
// churn.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = METER.try_with(|m| {
            let mut s = m.get();
            if s.enabled {
                let size = layout.size();
                s.allocs += 1;
                s.churn += size;
                s.live += size;
                s.peak = s.peak.max(s.live);
                m.set(s);
            }
        });
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _ = METER.try_with(|m| {
            let mut s = m.get();
            if s.enabled {
                s.live = s.live.saturating_sub(layout.size());
                m.set(s);
            }
        });
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub allocs: usize,
    pub churn: usize,
    pub peak: usize,
}

pub fn measure<R>(f: impl FnOnce() -> R) -> (R, Stats) {
    METER.set(Meter {
        enabled: true,
        ..Meter::ZERO
    });
    let r = f();
    let s = METER.get();
    METER.set(Meter::ZERO);
    (
        r,
        Stats {
            allocs: s.allocs,
            churn: s.churn,
            peak: s.peak,
        },
    )
}

type Snippet = (&'static str, fn() -> Stats);

macro_rules! snippet {
    ($name:ident => bake) => {
        (
            stringify!($name),
            (|| measure(|| corpus::$name().bake()).1) as fn() -> Stats,
        )
    };
    ($name:ident => macro) => {
        (
            stringify!($name),
            (|| measure(|| corpus::$name()).1) as fn() -> Stats,
        )
    };
}

#[rustfmt::skip]
fn html_snippets() -> &'static [Snippet] {
    &[
        snippet!(a => bake),
        snippet!(abbr => bake),
        snippet!(address => bake),
        snippet!(area => bake),
        snippet!(article => bake),
        snippet!(aside => bake),
        snippet!(audio => bake),
        snippet!(b => macro),
        snippet!(base => bake),
        snippet!(bdi => macro),
        snippet!(bdo => bake),
        snippet!(blockquote => bake),
        snippet!(body => bake),
        snippet!(br => bake),
        snippet!(button => bake),
        snippet!(canvas => macro),
        snippet!(caption => bake),
        snippet!(cite => bake),
        snippet!(code => bake),
        snippet!(col => macro),
        snippet!(colgroup => bake),
        snippet!(data => bake),
        snippet!(datalist => bake),
        snippet!(dd => macro),
        snippet!(del => bake),
        snippet!(details => bake),
        snippet!(dfn => bake),
        snippet!(dialog => macro),
        snippet!(div => bake),
        snippet!(dl => bake),
        snippet!(doctype => bake),
        snippet!(dt => macro),
        snippet!(em => macro),
        snippet!(embed => bake),
        snippet!(fieldset => bake),
        snippet!(figcaption => macro),
        snippet!(figure => bake),
        snippet!(footer => bake),
        snippet!(form => bake),
        snippet!(h1 => bake),
        snippet!(h2 => bake),
        snippet!(h3 => bake),
        snippet!(h4 => bake),
        snippet!(head => bake),
        snippet!(header => bake),
        snippet!(hgroup => bake),
        snippet!(hr => macro),
        snippet!(i => macro),
        snippet!(iframe => bake),
        snippet!(img => bake),
        snippet!(input => bake),
        snippet!(ins => bake),
        snippet!(kbd => bake),
        snippet!(label => bake),
        snippet!(legend => bake),
        snippet!(li => macro),
        snippet!(link => bake),
        snippet!(main => bake),
        snippet!(map => macro),
        snippet!(mark => macro),
        snippet!(menu => bake),
        snippet!(meta => bake),
        snippet!(meter => bake),
        snippet!(nav => bake),
        snippet!(noscript => bake),
        snippet!(object => bake),
        snippet!(ol => bake),
        snippet!(optgroup => bake),
        snippet!(option => bake),
        snippet!(output => bake),
        snippet!(p => bake),
        snippet!(picture => bake),
        snippet!(pre => macro),
        snippet!(progress => bake),
        snippet!(q => bake),
        snippet!(root => bake),
        snippet!(rp => macro),
        snippet!(rt => bake),
        snippet!(ruby => bake),
        snippet!(s => macro),
        snippet!(samp => bake),
        snippet!(script => bake),
        snippet!(search => bake),
        snippet!(section => bake),
        snippet!(select => bake),
        snippet!(slot => bake),
        snippet!(small => bake),
        snippet!(source => bake),
        snippet!(span => bake),
        snippet!(strong => bake),
        snippet!(style => bake),
        snippet!(sub => macro),
        snippet!(summary => bake),
        snippet!(sup => macro),
        snippet!(table => bake),
        snippet!(tbody => bake),
        snippet!(td => bake),
        snippet!(template => bake),
        snippet!(textarea => bake),
        snippet!(tfoot => bake),
        snippet!(th => bake),
        snippet!(thead => bake),
        snippet!(time => bake),
        snippet!(title => bake),
        snippet!(tr => bake),
        snippet!(track => bake),
        snippet!(u => macro),
        snippet!(ul => bake),
        snippet!(var => macro),
        snippet!(video => bake),
        snippet!(wbr => bake),
    ]
}

#[rustfmt::skip]
fn css_snippets() -> &'static [Snippet] {
    &[
        snippet!(stylesheet => bake),
        snippet!(rule => bake),
        snippet!(at_rule => bake),
        snippet!(declarations_block => bake),
        snippet!(selectors_list => bake),
        snippet!(declaration => bake),
        snippet!(type_selector => bake),
        snippet!(simple_selector => bake),
        snippet!(compound_selector => bake),
        snippet!(complex_selector => bake),
    ]
}

#[rustfmt::skip]
fn recipe_snippets() -> &'static [Snippet] {
    &[
        snippet!(document_homemade => bake),
        snippet!(stylesheet_preflight => bake),
    ]
}

#[cfg(test)]
mod tests {
    use super::{css_snippets, html_snippets, recipe_snippets, screenshot};

    #[test]
    fn alloc_html() {
        insta::with_settings!({
            omit_expression => true,
            prepend_module_to_snapshot => false,
        },
            {insta::assert_snapshot!(screenshot(html_snippets()))
        })
    }

    #[test]
    fn alloc_css() {
        insta::with_settings!({
            omit_expression => true,
            prepend_module_to_snapshot => false,
        },
            {insta::assert_snapshot!(screenshot(css_snippets()))
        })
    }

    #[test]
    fn alloc_recipe() {
        insta::with_settings!({
            omit_expression => true,
            prepend_module_to_snapshot => false,
        },
            {insta::assert_snapshot!(screenshot(recipe_snippets()))
        })
    }
}
