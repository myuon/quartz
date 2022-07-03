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
            "_neq",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Bool)),
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
        ("_panic", Type::Unit),
        ("_debug", Type::Unit),
        ("_start_debugger", Type::Unit),
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
            "_and",
            Type::Fn(vec![Type::Bool, Type::Bool], Box::new(Type::Bool)),
        ),
        (
            "_or",
            Type::Fn(vec![Type::Bool, Type::Bool], Box::new(Type::Bool)),
        ),
        ("_not", Type::Fn(vec![Type::Bool], Box::new(Type::Bool))),
        (
            "_int_to_byte",
            Type::Fn(vec![Type::Int], Box::new(Type::Byte)),
        ),
        (
            "_byte_to_int",
            Type::Fn(vec![Type::Byte], Box::new(Type::Int)),
        ),
        (
            "_copy",
            Type::Fn(
                vec![
                    Type::Int,
                    Type::Int,
                    Type::Any, // bytes or pointer (_padd(bytes, int) can be applied here)
                    Type::Array(Box::new(Type::Byte)),
                ],
                Box::new(Type::Unit),
            ),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}
