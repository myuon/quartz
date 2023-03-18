pub struct SExprWriter {
    pub buffer: String,
    depth: usize,
    index: usize,
}

impl SExprWriter {
    pub fn new() -> SExprWriter {
        SExprWriter {
            buffer: String::new(),
            depth: 0,
            index: 0,
        }
    }

    pub fn write(&mut self, text: impl std::fmt::Display) {
        self.buffer.push_str(&format!(
            "{}{}",
            if self.index == 0 { "" } else { " " },
            text
        ));
        self.index += 1;
    }

    pub fn start(&mut self) {
        self.new_statement();
        self.write("(");
        self.depth += 1;
        self.index = 0;
    }

    pub fn end(&mut self) {
        self.depth -= 1;
        self.index = 0;
        self.write(")");
    }

    pub fn new_statement(&mut self) {
        if self.index != 0 {
            self.write(&format!("\n{}", " ".repeat(self.depth * 4)));
        }
        self.index = 0;
    }

    pub fn finalize(&mut self) {
        for _ in 0..self.depth {
            self.end();
        }
    }
}
