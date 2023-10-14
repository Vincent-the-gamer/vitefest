#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $value: expr ),*) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}