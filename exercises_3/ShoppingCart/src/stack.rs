pub struct Stack {
    items: Vec<String>
}

impl Stack {
    pub fn new(items: Vec<String>) -> Self {
        Stack { items }
    }
    
    pub fn get(&mut self) -> Option<String> {
        self.items.pop()
    }
    
    pub fn put(&mut self, item: String) {
        self.items.push(item);
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}