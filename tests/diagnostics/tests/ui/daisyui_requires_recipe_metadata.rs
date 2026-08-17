use granola::DaisyUI;

#[derive(DaisyUI)]
struct MissingRecipe<R>(R);

#[derive(DaisyUI)]
struct MalformedRecipe<R>(R);

fn main() {}
