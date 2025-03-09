# a simple library for operator

## Example - Ternary

```rust
 use op::if_else;
 let n = if_else!(true => 0; 1);
 assert_eq!(n, 0);
 let n = if_else!(false => 0; 1);
 assert_eq!(n, 1);

 ```

## Example - catch

```rust
use op::catch;
type Error = &'static str;
let result1: Result<i32, &'static str> = Ok(1);
let result2: Result<i32, &'static str> = Ok(2);
let result3: Result<i32, &'static str> = Err("Error");
assert_eq!(Ok(3), catch!(Error -> Ok(result1? + result2?)));
assert_eq!(Err("Error"), catch!(Error -> Ok(result2? + result3?)));

let option1: Option<i32> = Some(1);
let option2: Option<i32> = Some(2);
let option3: Option<i32> = None;
assert_eq!(Some(3), catch!(Some(option1? + option2?)));
assert_eq!(None, catch!(Some(option2? + option3?)));
```
