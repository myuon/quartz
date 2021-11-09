use std::collections::HashMap;

use crate::{
    ast::{Expr, Statement, Type},
    compiler::Compiler,
};

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
        ("_print", Type::Fn(vec![Type::Any], Box::new(Type::Unit))),
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
        (
            "_lt",
            Type::Fn(vec![Type::Int, Type::Int], Box::new(Type::Bool)),
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
    let r = Compiler::new()
        .parse(
            r#"
                fn (x: int) add(y: int): int {
                    return _add(x,y);
                }

                fn (x: int) sub(y: int): int {
                    return _sub(x,y);
                }

                fn (x: int) eq(y: int): bool {
                    return _eq(x,y);
                }

                fn (x: int) lt(y: int): bool {
                    return _lt(x,y);
                }

                fn (s: string) len(): int {
                    return _len_string(s);
                }

                fn (x: string) concat(y: string): string {
                    return _concat_string(x,y);
                }

                fn (x: string) slice(i: int, j: int): string {
                    return _slice_string(x,i,j);
                }

                // FIXME: support vec type
                fn (a: any) push(b: any) {
                    a = _vpush(a,b);
                }
                "#,
        )
        .unwrap();

    let mut result = HashMap::new();
    for decl in r.0 {
        let func = decl.into_function().unwrap();
        let method_of = func.method_of.unwrap();
        result.insert(
            (method_of.1, func.name.to_string()),
            (
                method_of.0,
                func.args
                    .into_iter()
                    .map(|(n, t)| (n.to_string(), t))
                    .collect(),
                Box::new(func.return_type),
                func.body,
            ),
        );
    }

    result.into_iter().map(|(k, v)| (k, v)).collect()
}
