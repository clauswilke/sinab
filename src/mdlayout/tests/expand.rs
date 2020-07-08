// A mock integration test to make it easy to look at expanded macro code for specific macros.
// Run with: cargo expand --test expand --theme GitHub

#[derive(Debug)]
struct A {
    x: i32,
    y: i32,
    z: i64,
}

#[test]
fn expand_test() {
    let a = A{ x: 5, y: 2, z: 54 };

    println!("Hello world! {:?}", a);
}