#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum Value {
    Bool(bool),
    I32(i32),
    Pointer(#[cfg_attr(test, proptest(filter = "|p| p % 4 == 0"))] i32),
}

impl Value {
    // Value represented as 64bit data
    //
    // pointer: [32bit address][0 * 31]1
    // i32:     [32bit value  ][0 * 31]0
    // bool:    00...*10
    pub fn as_i64(&self) -> i64 {
        match self {
            Value::Bool(b) => {
                if *b {
                    0b110
                } else {
                    0b010
                }
            }
            Value::I32(i) => (*i as i64) << 32,
            Value::Pointer(p) => (*p as i64) | 0b1_i64,
        }
    }

    pub fn from_i64(i: i64) -> Value {
        if i & 0b1 == 0b1 {
            Value::Pointer((i & !0b1_i64) as i32)
        } else if i & 0b10 == 0b10 {
            Value::Bool(i == 0b110)
        } else {
            Value::I32((i >> 32) as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn value_as_i64_from_i64(v: Value) {
            assert_eq!(v, Value::from_i64(v.as_i64()));
        }
    }
}
