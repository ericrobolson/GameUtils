/// Stack data structure
#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new stack
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    /// Pushes an item onto the stack
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Pops an item off the stack
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Peeks at the top item of the stack
    pub fn peek(&self) -> Option<&T>{
        if self.items.is_empty(){
            return None
        } else{
            Some(&self.items[self.items.len() - 1])
        }
    }
}
