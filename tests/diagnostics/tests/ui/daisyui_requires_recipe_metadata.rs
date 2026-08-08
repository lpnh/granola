use granola::DaisyUI;

#[derive(DaisyUI)]
struct MissingRecipe<R>(R);

#[derive(DaisyUI)]
#[recipe(content = ())]
struct MalformedRecipe<R>(R);

fn main() {}
