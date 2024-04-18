use negative_impl::negative_impl;

struct ABC;

#[negative_impl]
impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}

// this doesn't compile
// #[negative_impl]
// impl !Sized for ABC {}

// optional sized type
// this would not compile without referencing the parameter
fn func<T: ?Sized>(t: &T) {}

struct IncorrectlyDefinedUnsizedStruct {
    sized_field: i32,
    unsized_field: [i32], // <-- this is an unsized type
                          // sized_2: i32, <-- this would be illegal
                          // since only the last field can be unsized
                          // sized_2: &i32, <-- this would be illegal, too
                          // only one unsized field is allowed
}

struct CorrectlyDefinedUnsizedStruct<T: ?Sized> {
    sized_field: i32,
    unsized_field: T,
}

fn sized_traits() {
    let x: i32 = Default::default();
}
