#![no_std]
#[macro_export]
/// ## Ternary Operator
///
/// condition ? true : false
///
/// ## Examples
/// ```rust
/// use op::ternary;
///
/// let n = ternary!(true => 0, 1);
/// assert_eq!(n, 0);
/// let n = ternary!(false => 0, 1);
/// assert_eq!(n, 1);
/// ````
macro_rules! ternary {
    ($condition:expr => $true:expr; $false:expr) => {
        if $condition {
            $true
        } else {
            $false
        }
    };
    ($condition:expr => $true:expr, $false:expr) => {
        if $condition {
            $true
        } else {
            $false
        }
    };
}
#[macro_export]
/// A simple macro that handle `Option<T>`.
macro_rules! some {
    ($arg:expr; $err:expr) => {
        if let Some(v) = $arg {
            v
        } else {
            $err
        }
    };
    ($arg:expr; ret) => {
        if let Some(v) = $arg {
            v
        } else {
            return;
        }
    };
    ($arg:expr; ret $result:expr) => {
        if let Some(v) = $arg {
            v
        } else {
            return $result;
        }
    };

    ($arg:expr; con) => {
        if let Some(v) = $arg {
            v
        } else {
            continue;
        }
    };
}
#[macro_export]
/// A simple macro that handle `Result<T, E>`.
macro_rules! result {
    ($arg:expr; $err:expr) => {
        if let Ok(v) = $arg {
            v
        } else {
            $err
        }
    };
    ($arg:expr; ret) => {
        if let Ok(v) = $arg {
            v
        } else {
            return;
        }
    };
    ($arg:expr; ret $result:expr) => {
        if let Ok(v) = $arg {
            v
        } else {
            return $result;
        }
    };

    ($arg:expr; con) => {
        if let Ok(v) = $arg {
            v
        } else {
            continue;
        }
    };
}

/// A simple macro that catch `Result<T, E>` and `Option<T>`
///
/// # Examples
///  ```
/// use op::catch;
/// type Error = &'static str;
/// let result1: Result<i32, &'static str> = Ok(1);
/// let result2: Result<i32, &'static str> = Ok(2);
/// let result3: Result<i32, &'static str> = Err("Error");
/// assert_eq!(Ok(3), catch!(Error -> Ok(result1? + result2?)));
/// assert_eq!(Err("Error"), catch!(Error -> Ok(result2? + result3?)));
///
/// let option1: Option<i32> = Some(1);
/// let option2: Option<i32> = Some(2);
/// let option3: Option<i32> = None;
/// assert_eq!(Some(3), catch!(Some(option1? + option2?)));
/// assert_eq!(None, catch!(Some(option2? + option3?)));
/// ```
#[macro_export]
macro_rules! catch {
    ($err:ident -> $stmt:stmt) => {
        (|| -> ::core::result::Result<_, $err> { $stmt })()
    };
    ($stmt:stmt) => {
        (|| -> ::core::option::Option<_> { $stmt })()
    };
}



pub trait Operator: Sized {
    /// A method that behaves similarly to a ternary operator
    /// 
    /// Equivalent to `if $flag { $true } else { $false }`
    /// # Usage
    /// 
    /// ```
    /// use op::Operator;
    /// 
    /// assert_eq!(2, 2.if_else(true, 5));
    /// assert_eq!(5, 2.if_else(false, 5));
    /// 
    /// ```
    fn if_else(self, flag: bool, r#false: Self) -> Self;
}

impl<T> Operator for T {
    fn if_else(self, flag: bool, r#false: Self) -> Self {
        if flag {
            self
        } else {
            r#false
        }
    }
}