use op::some;


fn main() {
    println!("{}", test(Some(45)));
    println!("{}", test(None));

}

fn test(i: Option<i32>) -> i32 {
    let i = some!(i; return 34);
    i + 1
}