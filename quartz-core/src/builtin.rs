use std::collections::HashMap;

use crate::ast::Type;

pub fn builtin() -> HashMap<String, Type> {
    vec![
        (
            "_add",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_sub",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_mult",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_eq",
            Type::Fn(vec![Type::Any, Type::Any], Box::new(Type::Bool)),
        ),
        (
            "_lt",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Bool)),
        ),
        ("_free", Type::Fn(vec![Type::Any], Box::new(Type::Unit))),
        (
            "_new",
            Type::Fn(vec![Type::Int], Box::new(Type::Array(Box::new(Type::Byte)))),
        ),
        ("_gc", Type::Unit),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}
