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

macro_rules! hash_set(
    { $($x:expr),+ } => {
        {
            let mut m = HashSet::new();
            $(
                m.insert($x);
            )+
            m
        }
    };
);