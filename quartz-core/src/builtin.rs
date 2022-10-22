use std::collections::HashMap;

use crate::ast::Type;

pub fn builtin() -> HashMap<String, Type> {
    vec![
        (
            "_add",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_sub",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_mult",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_div",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_mod",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Int)),
        ),
        (
            "_eq",
            Type::Fn(vec![], vec![Type::Any, Type::Any], Box::new(Type::Bool)),
        ),
        (
            "_neq",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Bool)),
        ),
        (
            "_lt",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Bool)),
        ),
        (
            "_gt",
            Type::Fn(vec![], vec![Type::Int, Type::Int], Box::new(Type::Bool)),
        ),
        (
            "_free",
            Type::Fn(vec![], vec![Type::Any], Box::new(Type::Nil)),
        ),
        (
            "_len",
            Type::Fn(
                vec![],
                vec![Type::Array(Box::new(Type::Any))],
                Box::new(Type::Int),
            ),
        ),
        ("_gc", Type::Nil),
        ("_panic", Type::Nil),
        ("_debug", Type::Nil),
        ("_start_debugger", Type::Nil),
        (
            "_padd",
            Type::Fn(vec![], vec![Type::Any, Type::Int], Box::new(Type::Byte)),
        ),
        (
            "_println",
            Type::Fn(
                vec![],
                vec![Type::Struct("string".to_string())],
                Box::new(Type::Nil),
            ),
        ),
        (
            "_and",
            Type::Fn(vec![], vec![Type::Bool, Type::Bool], Box::new(Type::Bool)),
        ),
        (
            "_or",
            Type::Fn(vec![], vec![Type::Bool, Type::Bool], Box::new(Type::Bool)),
        ),
        (
            "_not",
            Type::Fn(vec![], vec![Type::Bool], Box::new(Type::Bool)),
        ),
        (
            "_byte_to_int",
            Type::Fn(vec![], vec![Type::Byte], Box::new(Type::Int)),
        ),
        (
            "_copy",
            Type::Fn(
                vec![],
                vec![
                    Type::Int,                        // size
                    Type::Array(Box::new(Type::Any)), // source
                    Type::Int,                        // source offset
                    Type::Array(Box::new(Type::Any)), // target,
                    Type::Int,                        // target offset
                ],
                Box::new(Type::Nil),
            ),
        ),
        (
            "_println_any",
            Type::Fn(vec![], vec![Type::Any], Box::new(Type::Nil)),
        ),
        (
            "_sizeof",
            Type::Fn(vec!["T".to_string()], vec![], Box::new(Type::Int)),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}
