use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ast::{Module, Type},
    typechecker::typecheck_with,
};

pub fn stdlib() -> HashMap<String, Type> {
    vec![
        ("_object", Type::Any),
        ("_vec", Type::Any),
        ("_passign", Type::Any),
        ("_tuple", Type::Any),
        ("_get", Type::Any),
        ("_print", Type::Any),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

pub fn typecheck_with_stdlib(m: &mut Module) -> Result<()> {
    return typecheck_with(m, stdlib());
}
