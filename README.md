a simple library for operator


# Example - Ternary

 ```rust
 use op::ternary;
 let n = ternary!(true => 0; 1);
 assert_eq!(n, 0);
 let n = ternary!(false => 0; 1);
 assert_eq!(n, 1);
 ````



# Example - catch

 ```rust
use op::catch;

let result1: Result<i32, i32> = Ok(1);
let result2: Result<i32, i32> = Ok(2);
assert_eq!(Ok(3), catch!(i32 -> Ok(result1? + result2?)));

let option1: Option<i32> = Some(1);
let option2: Option<i32> = Some(2);
assert_eq!(Some(3), catch!(Some(option1? + option2?)));
 ````