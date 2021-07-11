use crate::ast::Module;

pub struct TypeChecker {
    module: Module,
}

impl TypeChecker {
    fn new(module: Module) -> TypeChecker {
        TypeChecker { module }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
