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
