use op::input;

fn main() {
    let any = input!().expect("");
    let print_ = input!("Input your name: ").unwrap();
    println!("{},  {}", any, print_)
}
