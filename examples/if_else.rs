use op::if_else;

fn main() {
    let flag = false;
    let res = if_else!(flag => 2, 3);
    assert_eq!(res, 3)
}
