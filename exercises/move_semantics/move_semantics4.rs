// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we instead create it within `fn build_vec` and transfer the
// freshly created vector from build_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!

fn main() {
    let mut vec1 = build_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `build_vec()` no longer take `vec: Vec<i32>` as argument
fn build_vec() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
