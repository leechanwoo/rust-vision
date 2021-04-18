
use frunk::prelude::*;
use frunk::{ 
    self
    , monoid
    , Semigroup
    , Generic
    , hlist
    , hlist_pat
    , LabelledGeneric
};

#[derive(Generic, LabelledGeneric)]
struct ApiUser<'a> {
    FirstName: &'a str,
    LastName: &'a str,
    Age: usize,
}

#[derive(Generic, LabelledGeneric)]
struct NewUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}

#[derive(LabelledGeneric)]
struct SavedUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: usize,
}



fn main() {
    let v = vec![Some(1), Some(3)];
    assert_eq!(monoid::combine_all(&v), Some(4));

    let h = hlist![1, "hi"];
    assert_eq!(h.len(), 2);

    let hlist_pat!(a, b) = h;
    assert_eq!(a, 1);
    assert_eq!(b, "hi");

    let h1 = hlist![Some(1), 3.3, 53i64, "hello".to_owned()];
    let h2 = hlist![Some(2), 1.2, 1i64, " world".to_owned()];
    let h3 = hlist![Some(3), 4.5, 54, "hello world".to_owned()];

    assert_eq!(h1.combine(&h2), h3);


    let a_user: ApiUser = frunk::from_generic(hlist!["Joe", "Blow", 30]);
    let n_user: NewUser = Generic::convert_from(a_user);

    let s_user: SavedUser = frunk::labelled_convert_from(n_user);

    assert_eq!(s_user.first_name, "Joe");
    assert_eq!(s_user.last_name, "Blow");
    assert_eq!(s_user.age, 30);

}
