use wrapped_vec::WrappedVec;

use std::iter::FromIterator;

#[test]
fn test_from_iter() {
    #[derive(WrappedVec)]
    #[CollectionName = "Fruits"]
    pub struct Fruit {};

    let fruits = Fruits::from_iter(vec![Fruit {}, Fruit {}]);
    assert_eq!(fruits.len(), 2);
}

#[test]
fn test_collection_derives() {
    #[derive(Clone, Debug, WrappedVec)]
    #[CollectionName = "Fruits"]
    #[CollectionDerives = "Clone, Debug"]
    pub struct Fruit {};

    let _debug = format!("{:?}", Fruit {});
    let _clone = (Fruit {}).clone();
}
