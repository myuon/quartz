use std::io::Write;

use anyhow::{anyhow, Result};
use wasmer::{imports, Instance, Module, Store};
use wasmer::{Function, Value};

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {}
    }

    pub fn _run(&mut self, wat: &str) -> Result<Box<[Value]>> {
        let mut store = Store::default();
        let module = Module::new(&store, &wat)?;
        let import_object = imports! {
            "env" => {
                "write_stdout" => Function::new_typed(&mut store, |ch: u32| {
                        std::io::stdout().lock().write(&[ch as u8]).unwrap();
                }),
                "debug_i32" => Function::new_typed(&mut store, |i: i32| {
                    println!("[DEBUG_I32] {}", i);
                }),
                "abort" => Function::new_typed(&mut store, || {
                    panic!("[ABORT]");
                }),
            }
        };
        let instance = Instance::new(&mut store, &module, &import_object)?;

        let main = instance.exports.get_function("main")?;
        let result = main.call(&mut store, &[])?;

        Ok(result)
    }

    pub fn run(&mut self, input: &str) -> Result<Box<[Value]>> {
        self._run(input).map_err(|err| {
            let message = err.to_string();
            // regexp test (at offset %d) against message
            let Ok(re) = regex::Regex::new(r"\(at offset (\d+)\)") else {
                return anyhow!("Original Error: {}", err);
            };
            let Some(cap) = re.captures(&message) else {
                return anyhow!("Original Error: {}", err);
            };
            let Ok(offset) = cap[1].parse::<usize>() else {
                return anyhow!("Original Error: {}", err);
            };

            let wasm = wat::parse_str(input).unwrap();

            let Ok(mut file) = std::fs::File::create("build/build.wat") else {
                return anyhow!("Original Error: {}", err);
            };
            file.write_all(input.as_bytes()).unwrap();

            let Ok(mut file) = std::fs::File::create("build/error.wasm") else {
                return anyhow!("Original Error: {}", err);
            };
            file.write_all(&wasm).unwrap();

            // offset in base-16
            let offset_hex = format!("{:x}", offset);

            anyhow!("Error at: {}\n\nOriginal Error: {}", offset_hex, err)
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use pretty_assertions::assert_eq;

    use crate::compiler::Compiler;

    use super::*;

    #[test]
    fn test_compile_and_run() {
        let mut runtime = Runtime::new();
        let cases = vec![
            (
                r#"
fun main(): i32 {
    let x: i32 = 10;
    return x + 1;
}
"#,
                vec![Value::I32(11)],
            ),
            (
                r#"
fun main() {
    let x = 10;
    return x + 1;
}
"#,
                vec![Value::I32(11)],
            ),
            (
                r#"
fun calc(b: i32): i32 {
    let a = 1;
    let z = 10;
    return z + a + b;
}

fun main() {
    return calc(2);
}
"#,
                vec![Value::I32(13)],
            ),
            (
                r#"
let a = 5;

fun f() {
    a = a + 10;
}

fun main() {
    f();
    return a;
}
"#,
                vec![Value::I32(15)],
            ),
            (
                r#"
fun factorial(n: i32): i32 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }

    return 0;
}

fun main() {
    return factorial(5);
}
"#,
                vec![Value::I32(120)],
            ),
            (
                r#"
fun main() {
    let x = 10;
    let n = 0;
    while n < 10 {
        x = x + n;
        n = n + 1;
    }

    return x;
}
"#,
                vec![Value::I32(55)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
};

fun main() {
    let p = Point { x: 10, y: 20 };

    return p.y;
}
"#,
                vec![Value::I32(20)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
    z: i32,
};

fun main() {
    let p = Point { x: 10, y: 20, z: 0 };
    p.z = p.x + p.y;

    return p.z;
}
"#,
                vec![Value::I32(30)],
            ),
            (
                r#"
fun main() {
    let p = make[array[i32,20]]();
    p.at(0) = 10;
    p.at(1) = 20;
    p.at(2) = p.at(0) + p.at(1);

    return p.at(2);
}
"#,
                vec![Value::I32(30)],
            ),
            (
                r#"
fun main() {
    let a = alloc(10);
    let b = alloc(5);
    let c = alloc(5);

    return 0;
}
"#,
                vec![Value::I32(0)],
            ),
            (
                r#"
fun main() {
    let str = "Hello, World!";
    println(str);

    return str.length;
}
"#,
                vec![Value::I32(13)],
            ),
            (
                r#"
fun h(b: i32): i32 {
    return b;
}

fun g(b: i32): i32 {
    let a = 20;

    return h(b) + a;
}

fun f(b: i32): i32 {
    let a = 10;

    return g(b) + a;
}

fun main() {
    let a = 5;
    return f(a);
}
"#,
                vec![Value::I32(35)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
};

fun get_x(p: Point): i32 {
    return p.x;
}

fun main() {
    return get_x(Point {
        x: 10,
        y: 20,
    });
}
"#,
                vec![Value::I32(10)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
};

fun point(x: i32, y: i32): Point {
    return Point {
        x: x,
        y: y,
    };
}

fun main() {
    return point(10, 20).y;
}
"#,
                vec![Value::I32(20)],
            ),
            (
                r#"
fun main() {
    let p = make[array[i32,20]]();
    for i in 0..10 {
        p.at(i) = i;
    }
    
    let sum = 0;
    for i in 0..10 {
        sum = sum + p.at(i);
    }

    return sum;
}
"#,
                vec![Value::I32(45)],
            ),
            (
                r#"
fun main() {
    let p = make[vec[i32]]();
    for i in 0..100 {
        p.push(i);
    }

    return p.at(40) + p.at(60);
}
"#,
                vec![Value::I32(100)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
};

module Point {
    fun get_x(self): i32 {
        return self.x;
    }

    fun sum(self): i32 {
        return self.get_x() + self.y;
    }

    fun new(x: i32, y: i32): Point {
        return Point {
            x: x,
            y: y,
        };
    }
}

fun main() {
    let p = Point::new(10, 20);

    return p.sum();
}
"#,
                vec![Value::I32(30)],
            ),
            (
                r#"
type Container = {
    x: i32?,
};

fun main() {
    let c = Container {
        x: 0?,
    };

    return c.x == nil;
}
"#,
                vec![Value::I32(0)],
            ),
            (
                r#"
type Point = {
    x: i32?,
    y: i32?,
};

module Point {
    fun swap(self) {
        let tmp = self.x;
        self.x = self.y;
        self.y = tmp;
    }
}

fun main() {
    let p = Point {
        x: nil,
        y: 10?,
    };
    p.swap();

    return p.x == nil;
}
"#,
                vec![Value::I32(0)],
            ),
            (
                r#"
fun int_to_string(n: i32): string {
    let digit = 0;
    let tmp = n;
    while tmp > 0 {
        tmp = tmp / 10;
        digit = digit + 1;
    }

    let str = alloc(digit) as ptr[byte];
    tmp = n;
    for i in 0..digit {
        let d = tmp % 10;
        str.at(digit - i - 1) = ((d + 48) as byte);
        tmp = tmp / 10;
    }

    return string {
        data: str,
        length: digit,
    };
}

fun main() {
    let str = int_to_string(123456);

    return str.data.at(5) as i32 - 48;
}
"#,
                vec![Value::I32(6)],
            ),
            (
                r#"
fun f(): string {
    let s = "foo" as string?;

    if true {
        return s!;
    } else {
        return "";
    }
}

fun main(): i32 {
    return 0;
}
"#,
                vec![Value::I32(0)],
            ),
            (
                r#"
fun main() {
    if "hello".equal("hello") {
        return 0;
    } else {
        return 1;
    }
}
"#,
                vec![Value::I32(0)],
            ),
            (
                r#"
fun main() {
    let t = "hello".concat("world");
    println(t);

    return t.equal("helloworld");
}
"#,
                vec![Value::I32(1)],
            ),
            (
                r#"
fun hoge(): string? {
    return "hoge"?;
}

fun main() {
    let t = hoge();

    return t != nil;
}
"#,
                vec![Value::I32(1)],
            ),
            (
                r#"
type P = {
    x: i32,
};

fun modify_p(p: P) {
    p.x = 10;
}

fun main() {
    let p = P {
        x: 0,
    };
    modify_p(p);

    return p.x;
}
"#,
                vec![Value::I32(10)],
            ),
            (
                r#"
fun main() {
    let paragraph = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let count = make[map[string, i32]]();

    for i in 0..paragraph.length {
        let ch = paragraph.at(i);
        count.insert(ch, count.at(ch) + 1);
    }

    return count.at("a");
}
"#,
                vec![Value::I32(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                        .chars()
                        .filter(|c| *c == 'a')
                        .count() as i32,
                )],
            ),
            (
                r#"
type Nested = {
    child: struct {
        x: i32,
        y: string,
    }
};

fun main() {
    let n = Nested {
        child: struct {
            x: 10,
            y: "hello",
        },
    };

    return n.child.y.length;
}
"#,
                vec![Value::I32(5)],
            ),
        ];

        for (input, expected) in cases {
            let mut compiler = Compiler::new();
            let wat = compiler
                .compile("", input)
                .context(format!("\n== SOURCE\n{}", input))
                .unwrap();
            let result = runtime
                .run(&wat)
                .context(format!("\n== SOURCE\n{}", input))
                .unwrap();
            assert_eq!(expected.as_slice(), result.as_ref(), "case: {}", input);
        }
    }
}
