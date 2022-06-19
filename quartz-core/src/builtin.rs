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
            "_mult",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_div",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_mod",
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
        (
            "_gt",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Bool)),
        ),
        ("_free", Type::Fn(vec![Type::Any], Box::new(Type::Unit))),
        (
            "_new",
            Type::Fn(vec![Type::Int], Box::new(Type::Array(Box::new(Type::Byte)))),
        ),
        (
            "_len",
            Type::Fn(vec![Type::Array(Box::new(Type::Byte))], Box::new(Type::Int)),
        ),
        ("_gc", Type::Unit),
        (
            "_padd",
            Type::Fn(vec![Type::Any, Type::Int], Box::new(Type::Byte)),
        ),
        (
            "_println",
            Type::Fn(
                vec![Type::Struct("string".to_string())],
                Box::new(Type::Unit),
            ),
        ),
        (
            "_or",
            Type::Fn(vec![Type::Bool, Type::Bool], Box::new(Type::Bool)),
        ),
        (
            "_copy",
            Type::Fn(
                vec![
                    Type::Array(Box::new(Type::Byte)),
                    Type::Array(Box::new(Type::Byte)),
                    Type::Int,
                ],
                Box::new(Type::Unit),
            ),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}
