use op::Operator;

fn main() {
    assert_eq!(2, 2.if_else(true, 5));
    assert_eq!(5, 2.if_else(false, 5));
}
