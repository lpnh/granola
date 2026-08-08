use granola::Granola;

#[derive(Granola)]
#[granola(format = xml)]
struct UnknownFormatter;

#[derive(Granola)]
#[granola(format = html, format = css)]
struct DuplicateFormatter;

fn main() {}
