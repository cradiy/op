#[macro_export]
/// A simple macro that handle `Option<T>`.
macro_rules! some {
    ($($option:expr), +) => {
        || -> ::core::option::Option<_> {
            ::core::option::Option::Some(($($option?), +))
        }()
    }
}
