pub struct Stack<T> {
    items: Vec<T>
}

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    } 

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.items.last()
    }

    pub fn is_empty(&mut self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&mut self) -> usize {
        self.items.len()
    }
}
