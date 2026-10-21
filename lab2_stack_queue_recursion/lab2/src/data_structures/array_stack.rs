// Note:
// I deliberately implement manual stack indexing here
// just for the pedagogical purpose. It's some kind of middle ground It's some kind of middle
// ground.
// If I really go further, I will touch upon unsafe Rust.
// If I use Vec<T> directly, there's really nothing much to practise.

pub struct Stack<T> {
    // Invariants:
    // 1. 0 <= top <= capacity
    // 2. capacity = data.len()
    data: Vec<Option<T>>,
    capacity: usize,
    // Some traditional data structure textbooks
    // often use `top` to refer to the index of the actual
    // top element
    //
    // But in this lab, `top` denotes the index one past the
    // top element.
    top: usize,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Stack<T> {
    const INITIAL_CAPACITY: usize = 5;
    pub fn len(&self) -> usize {
        self.top
    }
    pub fn new() -> Self {
        // Create a new stack instance
        // which starts with some reasonable small capacity
        // and it's empty at the moment.
        Self {
            data: std::iter::repeat_with(|| None)
                .take(Self::INITIAL_CAPACITY)
                .collect(),
            capacity: Self::INITIAL_CAPACITY,
            top: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        // First test if we've hit the capacity now
        // If so, we first expand the capacity and then insert
        // Or otherwise, we insert the new element directly.
        if self.top == self.capacity {
            self.capacity *= 2;
            self.data.resize_with(self.capacity, || None);
        }
        self.data[self.top] = Some(value);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // We pop the top element off the stack
        // If there's such an element, we return Some(value)
        // Otherwise, we return None
        if self.is_empty() {
            return None;
        }
        self.top -= 1;
        self.data[self.top].take()
    }
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
}
