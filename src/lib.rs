#[macro_export]
/// ## Ternary Operator
/// 
/// condition ? true : false
/// 
/// ## Examples 
/// ```rust
/// use op::ternary;
/// 
/// let n = ternary!(true => 0; 1);
/// assert_eq!(n, 0);
/// let n = ternary!(false => 0; 1);
/// assert_eq!(n, 1);
/// ````
macro_rules! ternary {
    ($condition:expr => $true:expr; $false:expr) => {
        if $condition { $true } else { $false }
    };
}
#[macro_export]
/// A simple macro that handle `Option<T>`.
macro_rules! some {
    ($arg:expr; ret) => {
        if let Some(v) = $arg { v } else { return; }
    };
    ($arg:expr; ret $result:expr) => {
        if let Some(v) = $arg { v } else { return $result; }
    };

    ($arg:expr; con) => {
        if let Some(v) = $arg { v } else { continue; }
    };
    ($arg:expr; break) => {
        if let Some(v) = $arg { v } else { break; }
    };
    ($arg:expr; break $end:expr) => {
        if let Some(v) = $arg { v } else { break $end; }
    };
}
#[macro_export]
/// A simple macro that handle `Result<T, E>`.
macro_rules! result {
    ($arg:expr; ret) => {
        if let Ok(v) = $arg { v } else { return; }
    };
    ($arg:expr; ret $result:expr) => {
        if let Ok(v) = $arg { v } else { return $result; }
    };

    ($arg:expr; con) => {
        if let Ok(v) = $arg { v } else { continue; }
    };
    ($arg:expr; break) => {
        if let Ok(v) = $arg { v } else { break; }
    };
    ($arg:expr; break $end:expr) => {
        if let Ok(v) = $arg { v } else { break $end; }
    };
}
