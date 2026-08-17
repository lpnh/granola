// Content type is not `FastWritable`.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct Foo;

fn main() {
    let _ = HtmlSpan::new().content(Foo);
}
