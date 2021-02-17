macro_rules! yaml_str {
    ($d:expr, $y:expr, $i:ident) => {
        if let Some(v) = $y[stringify!($i)].as_str() {
            $d.$i = Some(v.into());
        }
    };
}

macro_rules! cmp_ignore_none {
    ($_self:ident, $other:ident, $field:ident) => {
        if let Some(ref self_field) = $_self.$field {
            if let Some(ref other_field) = $other.$field {
                if self_field != other_field {
                    return false;
                }
            }
        }
    }
}
