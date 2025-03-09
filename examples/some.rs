use op::some;

fn main() {
    let s1 = Some(1);
    let s2 = Some("Hello");
    assert_eq!(Some((1, "Hello")), some!(s1, s2))
}
