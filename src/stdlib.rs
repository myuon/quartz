use std::collections::HashMap;

use crate::ast::Type;

pub fn stdlib() -> HashMap<String, Type> {
    vec![("_object", Type::Any), ("_vec", Type::Any)]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}
