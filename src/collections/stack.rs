/// Stack data structure
#[derive(Debug, Clone)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Returns whether the stack is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the stack of all items.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Creates a new stack
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    /// Returns the length of the stack
    pub fn len(&self) -> usize {
        self.items.len()
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
    pub fn peek(&self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        } else {
            Some(&self.items[self.items.len() - 1])
        }
    }
}
