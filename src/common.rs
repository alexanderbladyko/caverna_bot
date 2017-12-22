macro_rules! hash_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
    ($hm:ident, { $($key:expr => $value:expr),+ } ) => (
        {
            $(
                $hm.insert($key, $value);
            )+
        }
    );
);
