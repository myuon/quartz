use std::collections::HashMap;

use crate::ast::{Expr, Statement, Type};

pub fn stdlib() -> HashMap<String, Type> {
    vec![
        ("_object", Type::Any),
        ("_vec", Type::Any),
        (
            "_vpush",
            Type::Fn(vec![Type::Any, Type::Any], Box::new(Type::Unit)),
        ),
        ("_passign", Type::Any),
        ("_tuple", Type::Fn(vec![Type::Any], Box::new(Type::Any))),
        ("_get", Type::Any),
        ("_set", Type::Any),
        (
            "_regex",
            Type::Fn(vec![Type::String, Type::String], Box::new(Type::Bool)),
        ),
        ("_print", Type::Any),
        ("_panic", Type::Any),
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
        ("_free", Type::Fn(vec![Type::Any], Box::new(Type::Unit))),
        (
            "_slice",
            Type::Fn(vec![Type::Any, Type::Int, Type::Int], Box::new(Type::Any)),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

pub fn stdlib_methods() -> HashMap<
    (String, String), // receiver type, method name
    (
        String,              // receiver name
        Vec<(String, Type)>, // argument types
        Box<Type>,           // return type
        Vec<Statement>,      // body
    ),
> {
    vec![
        (
            ("int".to_string(), "add".to_string()),
            (
                "x".to_string(),
                vec![("y".to_string(), Type::Int)],
                Box::new(Type::Int),
                vec![Statement::Return(Expr::Call(
                    Box::new(Expr::Var("_add".to_string())),
                    vec![Expr::Var("x".to_string()), Expr::Var("y".to_string())],
                ))],
            ),
        ),
        (
            ("int".to_string(), "sub".to_string()),
            (
                "x".to_string(),
                vec![("y".to_string(), Type::Int)],
                Box::new(Type::Int),
                vec![Statement::Return(Expr::Call(
                    Box::new(Expr::Var("_sub".to_string())),
                    vec![Expr::Var("x".to_string()), Expr::Var("y".to_string())],
                ))],
            ),
        ),
        (
            ("int".to_string(), "eq".to_string()),
            (
                "x".to_string(),
                vec![("y".to_string(), Type::Int)],
                Box::new(Type::Bool),
                vec![Statement::Return(Expr::Call(
                    Box::new(Expr::Var("_eq".to_string())),
                    vec![Expr::Var("x".to_string()), Expr::Var("y".to_string())],
                ))],
            ),
        ),
        (
            ("string".to_string(), "len".to_string()),
            (
                "x".to_string(),
                vec![],
                Box::new(Type::Int),
                vec![Statement::Return(Expr::Call(
                    Box::new(Expr::Var("_len_string".to_string())),
                    vec![Expr::Var("x".to_string())],
                ))],
            ),
        ),
        (
            ("string".to_string(), "concat".to_string()),
            (
                "x".to_string(),
                vec![("y".to_string(), Type::String)],
                Box::new(Type::String),
                vec![Statement::Return(Expr::Call(
                    Box::new(Expr::Var("_concat_string".to_string())),
                    vec![Expr::Var("x".to_string()), Expr::Var("y".to_string())],
                ))],
            ),
        ),
        (
            ("string".to_string(), "slice".to_string()),
            (
                "x".to_string(),
                vec![("i".to_string(), Type::Int), ("j".to_string(), Type::Int)],
                Box::new(Type::String),
                vec![Statement::Return(Expr::Call(
                    Box::new(Expr::Var("_slice_string".to_string())),
                    vec![
                        Expr::Var("x".to_string()),
                        Expr::Var("i".to_string()),
                        Expr::Var("j".to_string()),
                    ],
                ))],
            ),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k, v))
    .collect()
}
