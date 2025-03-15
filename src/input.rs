#[macro_export]
macro_rules! input {
    () => {{
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).map(|_| buf.trim().to_owned())
    }};
    ($($arg:tt)*) => {{
        let mut buf = String::new();
        print!($($arg)*);
        std::io::Write::flush(&mut std::io::stdout()).and_then(|_| {
            std::io::stdin().read_line(&mut buf).map(|_| buf.trim().to_owned())
        })
    }};
}
