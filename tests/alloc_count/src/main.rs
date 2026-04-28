#![allow(unsafe_code)]
#![allow(clippy::undocumented_unsafe_blocks)]

mod snippets;
use snippets::*;

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use askama::Template;

use granola::prelude::*;

fn main() {
    println!("regular:\n");
    snippets();
    println!("\n-/-");
    println!("\nmostly empty:\n");
    snippets_empty();
}

fn snippets() {
    let snippets: &[(&str, fn() -> (Stats, Stats))] = &[
        ("dialog", dialog_example),
        ("div", div_example),
        ("fieldset", fieldset_example),
        ("form", form_example),
        ("p", p_example),
        ("picture", picture_example),
        ("pre", pre_example),
        ("root", root_example),
        ("ruby", ruby_example),
    ];

    for (label, res) in snippets {
        let (opt, naive) = res();
        report(label, opt, naive);
    }
}

fn snippets_empty() {
    let snippets: &[(&str, fn() -> (Stats, Stats))] = &[
        ("dialog", dialog_empty),
        ("div", div_empty),
        ("fieldset", fieldset_empty),
        ("form", form_empty),
        ("p", p_empty),
        ("picture", picture_empty),
        ("pre", pre_empty),
        ("root", root_empty),
        ("ruby", ruby_empty),
    ];

    for (label, res) in snippets {
        let (opt, naive) = res();
        report(label, opt, naive);
    }
}

fn report(label: &str, opt: Stats, naive: Stats) {
    let allocs_diff = opt.allocs as isize - naive.allocs as isize;
    let bytes_diff = opt.bytes as isize - naive.bytes as isize;
    println!(
        "{label:>10}: opt allocs:{:>3}, bytes:{:>5} | naive allocs:{:>3}, bytes:{:>5} | diff allocs:{:+4}, bytes:{:+6}",
        opt.allocs, opt.bytes, naive.allocs, naive.bytes, allocs_diff, bytes_diff,
    );
}

thread_local! {
    static ALLOCS: Cell<usize> = const { Cell::new(0) };
    static BYTES: Cell<usize> = const { Cell::new(0) };
    static ENABLED: Cell<bool> = const { Cell::new(false) };
}

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = ENABLED.try_with(|e| {
            if e.get() {
                let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
                let _ = BYTES.try_with(|c| c.set(c.get() + layout.size()));
            }
        });
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTING: CountingAllocator = CountingAllocator;

#[derive(Debug, Clone, Copy)]
struct Stats {
    allocs: usize,
    bytes: usize,
}

fn measure<R>(f: impl FnOnce() -> R) -> (R, Stats) {
    ALLOCS.with(|c| c.set(0));
    BYTES.with(|c| c.set(0));
    ENABLED.with(|c| c.set(true));
    let r = f();
    ENABLED.with(|c| c.set(false));
    (
        r,
        Stats {
            allocs: ALLOCS.with(Cell::get),
            bytes: BYTES.with(Cell::get),
        },
    )
}

fn bake_naive<T: Template>(t: &T) -> String {
    let mut buf = String::new();
    let _ = t.render_into(&mut buf);
    buf
}

#[macro_export]
macro_rules! bake_block_naive {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        #[allow(unused_imports)]
        use granola::oven::Roast as _;
        let mut buf = String::new();
        granola::oven::Bake(&$first).bake_content(&mut buf);
        $({
            buf.push('\n');
            granola::oven::Bake(&$rest).bake_content(&mut buf);
        })*
        buf
    }};
}

#[macro_export]
macro_rules! bake_inline_naive {
    ($($item:expr),+ $(,)?) => {{
        #[allow(unused_imports)]
        use granola::oven::Roast as _;
        let mut buf = String::new();
        $({
            granola::oven::Bake(&$item).bake_content(&mut buf);
        })*
        buf
    }};
}

#[macro_export]
macro_rules! bake_newline_naive {
    ($item:expr $(,)?) => {{
        #[allow(unused_imports)]
        use granola::oven::Roast as _;
        let mut buf = String::new();
        buf.push('\n');
        granola::oven::Bake(&$item).bake_content(&mut buf);
        buf
    }};
}

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";

#[cfg(test)]
mod alloc_tests {
    #[test]
    fn dialog_example() {
        let (opt, naive) = super::dialog_example();
        assert_eq!((opt.allocs, opt.bytes), (14, 1237));
        assert_eq!((naive.allocs, naive.bytes), (17, 1406));
    }

    #[test]
    fn div_example() {
        let (opt, naive) = super::div_example();
        assert_eq!((opt.allocs, opt.bytes), (54, 56593));
        assert_eq!((naive.allocs, naive.bytes), (57, 56613));
    }

    #[test]
    fn fieldset_example() {
        let (opt, naive) = super::fieldset_example();
        assert_eq!((opt.allocs, opt.bytes), (16, 1700));
        assert_eq!((naive.allocs, naive.bytes), (18, 1447));
    }

    #[test]
    fn form_example() {
        let (opt, naive) = super::form_example();
        assert_eq!((opt.allocs, opt.bytes), (17, 1104));
        assert_eq!((naive.allocs, naive.bytes), (21, 1110));
    }

    #[test]
    fn p_example() {
        let (opt, naive) = super::p_example();
        assert_eq!((opt.allocs, opt.bytes), (48, 38757));
        assert_eq!((naive.allocs, naive.bytes), (49, 38747));
    }

    #[test]
    fn picture_example() {
        let (opt, naive) = super::picture_example();
        assert_eq!((opt.allocs, opt.bytes), (11, 961));
        assert_eq!((naive.allocs, naive.bytes), (14, 1024));
    }

    #[test]
    fn pre_example() {
        let (opt, naive) = super::pre_example();
        assert_eq!((opt.allocs, opt.bytes), (14, 1674));
        assert_eq!((naive.allocs, naive.bytes), (17, 1892));
    }

    #[test]
    fn root_example() {
        let (opt, naive) = super::root_example();
        assert_eq!((opt.allocs, opt.bytes), (27, 2368));
        assert_eq!((naive.allocs, naive.bytes), (30, 2419));
    }

    #[test]
    fn ruby_example() {
        let (opt, naive) = super::ruby_example();
        assert_eq!((opt.allocs, opt.bytes), (9, 235));
        assert_eq!((naive.allocs, naive.bytes), (11, 322));
    }
}

#[cfg(test)]
mod alloc_tests_empty {
    #[test]
    fn dialog_empty() {
        let (opt, naive) = super::dialog_empty();
        assert_eq!((opt.allocs, opt.bytes), (6, 338));
        assert_eq!((naive.allocs, naive.bytes), (9, 385));
    }

    #[test]
    fn div_empty() {
        let (opt, naive) = super::div_empty();
        assert_eq!((opt.allocs, opt.bytes), (23, 1153));
        assert_eq!((naive.allocs, naive.bytes), (26, 1380));
    }

    #[test]
    fn fieldset_empty() {
        let (opt, naive) = super::fieldset_empty();
        assert_eq!((opt.allocs, opt.bytes), (5, 256));
        assert_eq!((naive.allocs, naive.bytes), (9, 305));
    }

    #[test]
    fn form_empty() {
        let (opt, naive) = super::form_empty();
        assert_eq!((opt.allocs, opt.bytes), (11, 614));
        assert_eq!((naive.allocs, naive.bytes), (15, 666));
    }

    #[test]
    fn p_empty() {
        let (opt, naive) = super::p_empty();
        assert_eq!((opt.allocs, opt.bytes), (18, 617));
        assert_eq!((naive.allocs, naive.bytes), (20, 791));
    }

    #[test]
    fn picture_empty() {
        let (opt, naive) = super::picture_empty();
        assert_eq!((opt.allocs, opt.bytes), (5, 195));
        assert_eq!((naive.allocs, naive.bytes), (8, 194));
    }

    #[test]
    fn pre_empty() {
        let (opt, naive) = super::pre_empty();
        assert_eq!((opt.allocs, opt.bytes), (6, 120));
        assert_eq!((naive.allocs, naive.bytes), (8, 169));
    }

    #[test]
    fn root_empty() {
        let (opt, naive) = super::root_empty();
        assert_eq!((opt.allocs, opt.bytes), (16, 814));
        assert_eq!((naive.allocs, naive.bytes), (18, 601));
    }

    #[test]
    fn ruby_empty() {
        let (opt, naive) = super::ruby_empty();
        assert_eq!((opt.allocs, opt.bytes), (6, 203));
        assert_eq!((naive.allocs, naive.bytes), (7, 202));
    }
}
