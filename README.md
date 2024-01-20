a simple library for operator


# Example

 ```rust
 use op::ternary;
 let n = ternary!(true => 0; 1);
 assert_eq!(n, 0);
 let n = ternary!(false => 0; 1);
 assert_eq!(n, 1);
 ````