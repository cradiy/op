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
///
/// let result1: Result<i32, i32> = Ok(1);
/// let result2: Result<i32, i32> = Ok(2);
/// assert_eq!(Ok(3), catch!(i32 -> Ok(result1? + result2?)));
///
/// let option1: Option<i32> = Some(1);
/// let option2: Option<i32> = Some(2);
/// assert_eq!(Some(3), catch!(Some(option1? + option2?)));
/// ```
#[macro_export]
macro_rules! catch {
    ($e:ident -> $stmt:stmt) => {{
        let catch = || -> std::result::Result<_, $e> { $stmt };
        catch()
    }};
    ($stmt:stmt) => {{
        let catch = || -> std::option::Option<_> { $stmt };
        catch()
    }};
}
