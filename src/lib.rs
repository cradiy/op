#[cfg(feature = "full")]
pub use op_proc::*;
mod catch;
mod input;
mod some;

#[macro_export]
/// ## Ternary Operator
///
/// condition ? true : false
///
/// ## Examples
/// ```rust
/// use op::if_else;
///
/// let n = if_else!(true => 0, 1);
/// assert_eq!(n, 0);
/// let n = if_else!(false => 0, 1);
/// assert_eq!(n, 1);
/// ````
macro_rules! if_else {
    ($condition:expr => $true:expr; $false:expr) => {
        if $condition { $true } else { $false }
    };
    ($condition:expr => $true:expr, $false:expr) => {
        if $condition { $true } else { $false }
    };
}
