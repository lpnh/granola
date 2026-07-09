#[cfg(test)]
mod tests {
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

    pub fn print_result(snippets: &[Snippet]) -> String {
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
    #[allow(unsafe_code)]
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

    macro_rules! fixture {
        ($label:ident, $build:expr) => {
            (
                stringify!($label),
                (|| measure(|| ($build).bake()).1) as fn() -> Stats,
            )
        };
    }

    fn html_fixtures() -> &'static [Snippet] {
        &[
            fixture!(html_standard, fixtures::standard::page()),
            fixture!(html_macros, fixtures::macros::page()),
            fixture!(html_recipes, fixtures::recipes::page()),
        ]
    }

    fn css_fixtures() -> &'static [Snippet] {
        &[
            fixture!(css_standard, fixtures::standard::style()),
            fixture!(css_macros, fixtures::macros::style()),
            fixture!(css_recipes, fixtures::recipes::style()),
        ]
    }

    #[test]
    fn alloc_html() {
        insta::with_settings!({
            omit_expression => true,
            prepend_module_to_snapshot => false,
        },
            {insta::assert_snapshot!(print_result(html_fixtures()))
        })
    }

    #[test]
    fn alloc_css() {
        insta::with_settings!({
            omit_expression => true,
            prepend_module_to_snapshot => false,
        },
            {insta::assert_snapshot!(print_result(css_fixtures()))
        })
    }
}
