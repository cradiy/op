use op::ternary;
fn main() {
    let flag = false;
    let res = ternary!(flag => 2, 3);
    assert_eq!(res, 3)
}
