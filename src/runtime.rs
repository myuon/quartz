use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use wasmer::{imports, Instance, Module, Store};
use wasmer::{Function, Value as WasmValue};

use crate::value::Value;

#[derive(Clone)]
struct Handler {
    path: Arc<Mutex<Option<Vec<u8>>>>,
    file: Arc<Mutex<Option<File>>>,
}

pub struct Runtime {
    handler: Handler,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            handler: Handler {
                path: Arc::new(Mutex::new(None)),
                file: Arc::new(Mutex::new(None)),
            },
        }
    }

    pub fn _run(&mut self, wat: &str) -> Result<Box<[WasmValue]>> {
        let mut store = Store::default();
        let module = Module::new(&store, &wat)?;

        let handler_create = self.handler.clone();
        let handler_open = self.handler.clone();
        let handler_initialize = self.handler.clone();
        let handler_read = self.handler.clone();

        let import_object = imports! {
            "env" => {
                "write_stdout" => Function::new_typed(&mut store, |ch: i64| {
                    let w = Value::from_i64(ch);
                    match w {
                        Value::Byte(b) => {
                            std::io::stdout().lock().write(&[b]).unwrap();
                        }
                        _ => println!("write_stdout: invalid value, {:?}", w),
                    }

                    Value::i32(0).as_i64()
                }),
                "debug_i32" => Function::new_typed(&mut store, |i: i64| {
                    let w = Value::from_i64(i);

                    println!("[DEBUG] {:?}", w);
                    Value::i32(0).as_i64()
                }),
                "debug" => Function::new_typed(&mut store, |i: i64| {
                    let w = Value::from_i64(i);

                    println!("[DEBUG] {:?} ({:#032b})", w, i >> 32);
                    Value::i32(0).as_i64()
                }),
                "abort" => Function::new_typed(&mut store, || -> i64 {
                    panic!("[ABORT]");
                }),
                "read_stdin" => Function::new_typed(&mut store, || {
                    let mut buffer = [0u8; 1];
                    std::io::stdin().lock().read(&mut buffer).unwrap();
                    Value::Byte(buffer[0]).as_i64()
                }),
                "create_handler" => Function::new_typed(&mut store, move || {
                    handler_create.path.lock().unwrap().replace(Vec::new());

                    Value::i32(0).as_i64()
                }),
                "open_handler_stream" => Function::new_typed(&mut store, move |_handler_code: i64, i: i64| {
                    let v = Value::from_i64(i);
                    match v {
                        Value::Byte(b) => {
                            let mut handler = handler_open.path.lock().unwrap();
                            if let Some(ref mut path) = *handler {
                                path.push(b);
                            }
                        }
                        _ => unreachable!()
                    }

                    Value::nil().as_i64()
                }),
                "open_handler_initialize" => Function::new_typed(&mut store, move |_handler_code: i64| {
                    let mut handler = handler_initialize.path.lock().unwrap();
                    let path = handler.take().unwrap();
                    let path = String::from_utf8(path).unwrap();

                    let file = File::open(path).unwrap();
                    *handler_initialize.file.lock().unwrap() = Some(file);

                    Value::nil().as_i64()
                }),
                "read_handler" => Function::new_typed(&mut store, move |_handler_code: i64| {
                    let mut handler = handler_read.file.lock().unwrap();
                    let file = handler.as_mut().unwrap();
                    let mut buf = [0u8; 1];
                    let bs = file.read(&mut buf).unwrap();
                    if bs == 0 {
                        Value::Byte(0).as_i64()
                    } else {
                        Value::Byte(buf[0]).as_i64()
                    }
                }),
                "i64_to_string_at" => Function::new_typed(&mut store, |a_value: i64, b_value: i64, at_value: i64| {
                    let a = Value::from_i64(a_value).as_i32().unwrap();
                    let b = Value::from_i64(b_value).as_i32().unwrap();
                    let at = Value::from_i64(at_value).as_i32().unwrap() as usize;

                    let bs = format!("{}", ((a as i64) << 32_i64) | (b as i64)).chars().collect::<Vec<_>>();

                    if at >= bs.len() {
                        Value::I32(-1)
                    } else {
                        Value::I32(bs[at as usize].to_digit(10).unwrap() as i32)
                    }.as_i64()
                }),
            }
        };
        let instance = Instance::new(&mut store, &module, &import_object)?;

        let main = instance.exports.get_function("main")?;
        let result = main.call(&mut store, &[])?;

        Ok(result)
    }

    pub fn run(&mut self, input: &str) -> Result<Box<[WasmValue]>> {
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

    use crate::{compiler::Compiler, ir::IrTerm};

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
fun calc(b: i32): i32 {
    let a = 1;
    let z = 10;
    return z + a + b;
}

fun main(): i32 {
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

fun main(): i32 {
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

fun main(): i32 {
    return factorial(5);
}
"#,
                vec![Value::I32(120)],
            ),
            (
                r#"
fun main(): i32 {
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

fun main(): i32 {
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
};

fun main(): i32 {
    let p = Point { x: 10, y: 20 };

    return p.x;
}
"#,
                vec![Value::I32(10)],
            ),
            (
                r#"
type Point = {
    x: i32,
    y: i32,
    z: i32,
};

fun main(): i32 {
    let p = Point { x: 10, y: 20, z: 0 };
    p.z = p.x + p.y;

    return p.z;
}
"#,
                vec![Value::I32(30)],
            ),
            (
                r#"
fun main(): i32 {
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
fun main(): i32 {
    let a = alloc(10 * sizeof[i32]());
    let b = alloc(5 * sizeof[i32]());
    let c = alloc(5 * sizeof[i32]());

    return 0;
}
"#,
                vec![Value::I32(0)],
            ),
            (
                r#"
fun main(): i32 {
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

fun main(): i32 {
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

fun main(): i32 {
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

fun main(): i32 {
    return point(10, 20).y;
}
"#,
                vec![Value::I32(20)],
            ),
            (
                r#"
fun main(): i32 {
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
fun main(): i32 {
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

fun main(): i32 {
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

fun main(): bool {
    let c = Container {
        x: 0?,
    };

    return c.x == nil;
}
"#,
                vec![Value::Bool(false)],
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

fun main(): bool {
    let p = Point {
        x: nil,
        y: 10?,
    };
    p.swap();

    return p.x == nil;
}
"#,
                vec![Value::Bool(false)],
            ),
            (
                r#"
fun main(): byte {
    let p = make[ptr[byte]](3);
    p.at(0) = 48 as byte;
    p.at(1) = 56 as byte;
    p.at(2) = 72 as byte;

    let s = new_string(p, 3);
    println(s);

    return s.at(0);
}
"#,
                vec![Value::Byte(48)],
            ),
            (
                r#"
fun main(): byte {
    let a = "hello";

    return a.at(3);
}
"#,
                vec![Value::Byte(b'l')],
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

    let str = make[ptr[byte]](digit);
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

fun main(): i32 {
    let str = int_to_string(123456);
    println(str);

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
fun main(): i32 {
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
fun main(): bool {
    let t = "hello".concat("world");
    println(t);

    return t.equal("helloworld");
}
"#,
                vec![Value::Bool(true)],
            ),
            (
                r#"
fun hoge(): string? {
    return "hoge"?;
}

fun main(): bool {
    let t = hoge();

    return t != nil;
}
"#,
                vec![Value::Bool(true)],
            ),
            (
                r#"
type P = {
    x: i32,
};

fun modify_p(p: P) {
    p.x = 10;
}

fun main(): i32 {
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
fun main(): i32 {
    let paragraph = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let count = make[map[string, i32]]();

    for i in 0..paragraph.length {
        let ch_byte = paragraph.at(i);
        let ch_ptr = make[ptr[byte]](1);
        ch_ptr.at(0) = ch_byte;
        let ch = new_string(ch_ptr, 1);

        if !count.has(ch) {
            count.insert(ch, 0);
        }

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

fun main(): i32 {
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
            (
                r#"
fun f(): bool {
    panic("foo");

    return false;
}

fun main(): bool {
    return false && f();
}
"#,
                vec![Value::Bool(false)],
            ),
            (
                r#"
fun main() {
    for i in 0..5 {
        continue;
    }
}
"#,
                vec![Value::nil()],
            ),
            (
                r#"
type T = {
};

module T {
    fun method(self) {
    }

    fun f(self, method: i32): i32 {
        return method;
    }
}

fun main() {
}
"#,
                vec![Value::nil()],
            ),
            (
                r#"
fun vec(..t: vec[i32]): i32 {
    return t.length;
}

fun main(): i32 {
    let t1 = vec(1,2,3,4);
    let t2 = vec(1,2,3,4,5);

    return t1 + t2;
}
"#,
                vec![
                    Value::I32(9),
                ],
            ),
            (
                r#"
fun main(): bool {
    let r1 = "helloll".replace_first("ll", "x");
    let r2 = "ab".replace_first("a", "eee").replace_first("b", "fff");

    return r1.equal("hexoll")
        && r2.equal("eeefff");
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
fun main(): bool {
    return "abc".concat("def").equal("abcdef");
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
fun main(): bool {
    let f = format("hello, {}, {}, {}", "world", 1.to_string(), true.to_string());
    println(f);

    return f.equal("hello, world, 1, true");
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
fun f(v: i32, ..t: vec[i32]): i32 {
    return v + t.length;
}

fun main(): i32 {
    let t = make[vec[i32]]();
    t.push(1);
    t.push(2);
    t.push(3);

    return f(10, ..t);
}
"#,
                vec![
                    Value::I32(13),
                ],
            ),
            (
                r#"
fun f(a: i32): i32 or string {
    if a == 0 {
        return _ or "zero";
    } else {
        return a;
    }
}

fun main(): bool {
    let a or b = f(0);
    return a == nil && b!.equal("zero");
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
fun div(n: i32, m: i32): i32 or error {
    if m == 0 {
        return _ or error::new("zero division exception");
    }

    return n / m;
}

fun calc(): i32 or error {
    let n = div(10, 0).try;

    return n + 1;
}

fun main(): bool {
    let result or err = calc();
    if err != nil {
        println(err!.message);
        return true;
    } else {
        return false;
    }
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
fun f(n: i32): i32 or error {
    if n == 0 {
        return _ or error::new("zero");
    } else {
        return n + 1;
    }
}

fun g(n0: i32): i32 or error {
    let n = n0;
    for i in 0..4 {
        n = f(n).try;
    }

    return n;
}

fun main(): bool {
    let r or err = g(0);
    if err != nil {
        println(err!.message);
    }
    if r != nil {
        println(r!.to_string());
    }

    return err != nil && r == nil;
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
struct Position {
    x: i32,
}

fun main(): i32 {
    let p = Position {
        x: 10,
    };

    return p.x;
}
"#,
                vec![
                    Value::I32(10),
                ],
            ),
            (
                r#"
fun sum(..t: vec[i32]): i32 {
    let r = 0;
    for i in 0..t.length {
        r = r + t.at(i);
    }

    return r;
}

fun main(): i32 {
    return sum(10, 4, 2, 30, 100);
}
"#,
                vec![
                    Value::I32(146),
                ],
            ),
            (
                r#"
fun main(): bool {
    reflection::print_type(10);
    reflection::print_type("hello");
    reflection::print_type(true);

    return reflection::is_pointer("hello");
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
struct P {
    x: i32,
    y: string,
}

fun main(): bool {
    let p = P {
        x: 10,
        y: "hello",
    };
    println(derive::to_string(p));

    return derive::to_string(p).equal(`P { x: 10, y: "hello" }`);
}
"#,
                vec![
                    Value::Bool(true),
                ],
            ),
            (
                r#"
fun main() {
    let v = make[vec[i32]]();
    for i in 0..5 {
        v.push(i);
    }

    for n in v {
        println(n.to_string());
    }
}
"#,
                vec![
                    Value::nil(),
                ],
            ),
            (
                r#"
fun main(): i32 {
    let t = 3;
    if t == 0 {
        return t;
    } else if t == 1 {
        return t;
    } else if t == 2 {
        return t;
    } else if t == 3 {
        return t;
    } else if t == 4 {
        return t;
    } else {
        return t;
    }
}
"#,
                vec![
                    Value::i32(3),
                ],
            ),
            (
                r#"
fun sum(v: vec[i32]): i32 {
    let r = 0;
    for n in v {
        r = r + n;
    }

    return r;
}

fun main(): i32 {
    return sum(make[vec[i32]](1, 2, 3, 4, 5));
}
"#,
                vec![
                    Value::i32(15),
                ],
            ),
            (
                r#"
fun main(): i32 {
    return make[vec[string]]("hi", "how", "are", "you", "doing").length;
}
"#,
                vec![
                    Value::i32(5),
                ],
            ),
        ];

        for (input, expected) in cases {
            let mut compiler = Compiler::new();
            let wat = compiler
                .compile("", input, None)
                .context(format!("\n== SOURCE\n{}", input))
                .unwrap();
            let result = runtime
                .run(&wat)
                .context(format!("\n== SOURCE\n{}\n", input,))
                .unwrap();

            let ir = compiler.ir.unwrap();
            let ir_module = match ir {
                IrTerm::Module { elements: m } => m[0].clone(),
                _ => todo!(),
            };

            assert_eq!(
                expected,
                result
                    .into_iter()
                    .map(|v| match v {
                        WasmValue::I64(v) => Value::from_i64(*v),
                        _ => todo!(),
                    })
                    .collect::<Vec<Value>>(),
                "case: {}\n== IR\n{}\n",
                input,
                ir_module.to_string()
            );
        }
    }
}
