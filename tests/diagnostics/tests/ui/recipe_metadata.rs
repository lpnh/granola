use std::marker::PhantomData;

use granola::Recipe;

#[derive(Recipe)]
struct MissingAttribute<R> {
    _recipe: PhantomData<R>,
}

#[derive(Recipe)]
#[recipe(name)]
struct MalformedAttribute<R> {
    _recipe: PhantomData<R>,
}

#[derive(Recipe)]
#[recipe(name = FirstRecipe, name = SecondRecipe)]
struct DuplicateName<R> {
    _recipe: PhantomData<R>,
}

#[derive(Recipe)]
#[recipe(name = ContentRecipe, content = String, content = Vec<u8>)]
struct DuplicateContent<R> {
    _recipe: PhantomData<R>,
    content: String,
}

#[derive(Recipe)]
#[recipe(name = FirstAttributeRecipe)]
#[recipe(name = SecondAttributeRecipe)]
struct DuplicateAttribute<R> {
    _recipe: PhantomData<R>,
}

#[derive(Recipe)]
#[recipe(name = UnknownKeyRecipe, unknown = ())]
struct UnknownKey<R> {
    _recipe: PhantomData<R>,
}

fn main() {}
