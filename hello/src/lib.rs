pub fn world () -> String {
    String::from("Hello, world")
}

#[test]
fn world_returns_hello_world(){
    let result = world();
    let want = String::from("Hello, world");
    assert_eq!(result, want, "want {want}, got {result}");
}