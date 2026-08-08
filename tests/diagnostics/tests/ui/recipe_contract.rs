use std::marker::PhantomData;

use granola::Recipe;

#[derive(Recipe)]
#[recipe(name = TupleRecipe)]
struct Tuple<R>(PhantomData<R>);

#[derive(Recipe)]
#[recipe(name = EnumRecipe)]
enum NotAStruct<R> {
    Recipe(PhantomData<R>),
}

#[derive(Recipe)]
#[recipe(name = MarkerRecipe)]
struct WrongMarker<R> {
    marker: PhantomData<R>,
}

#[derive(Recipe)]
#[recipe(name = MarkerTypeRecipe)]
struct WrongMarkerType<R> {
    _recipe: (),
    _type: PhantomData<R>,
}

#[derive(Recipe)]
#[recipe(name = ContentRecipe, content = String)]
struct MissingContent<R> {
    _recipe: PhantomData<R>,
}

#[derive(Recipe)]
#[recipe(name = GenericRecipe)]
struct ExtraGeneric<'a, R> {
    _recipe: PhantomData<R>,
    _borrowed: PhantomData<&'a ()>,
}

fn main() {}
