macro_rules! assert_between {
    ($val:expr, $low:expr, $high:expr) => {
        if $val < $low || $val > $high {
            panic!(
                "assertion failed: {} should be between {} and {} (inclusive), but was {}",
                stringify!($val),
                $low,
                $high,
                $val,
            );
        }
    };
}

macro_rules! hashmap {
    {} => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $val:expr),+ $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $val);
        )+
        map
    }};
}
