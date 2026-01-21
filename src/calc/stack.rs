pub struct Stack {
    buffer: Vec<i64>,
}

impl Stack {
    pub fn default() -> Stack {
        Stack { buffer: Vec::new() }
    }
    pub fn push(&mut self, e: i64) {
        self.buffer.push(e);
    }
    pub fn bipop(&mut self) -> Option<(i64,i64)>{
        if self.buffer.len()<2 {
            return None;
        }
        Some((self.buffer.pop()?,self.buffer.pop()?))
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.buffer.pop()
    }
}
