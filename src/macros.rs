macro_rules! yaml_str {
    ($d:expr, $y:expr, $i:ident) => {
        if let Some(v) = $y[stringify!($i)].as_str() {
            $d.$i = Some(v.into());
        }
    };
}
