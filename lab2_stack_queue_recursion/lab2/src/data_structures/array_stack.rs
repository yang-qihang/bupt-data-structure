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

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn new_stack_is_empty() {
        let stack = Stack::<i32>::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn push_and_pop_follow_lifo_order() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.len(), 3);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn pop_on_empty_stack_returns_none() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn grows_beyond_initial_capacity() {
        // The initial capacity is 5; pushing more must trigger a reallocation
        // while preserving every element.
        let mut stack = Stack::new();
        for value in 0..20 {
            stack.push(value);
        }
        assert_eq!(stack.len(), 20);
        for expected in (0..20).rev() {
            assert_eq!(stack.pop(), Some(expected));
        }
        assert!(stack.is_empty());
    }

    #[test]
    fn remains_usable_after_draining() {
        let mut stack = Stack::new();
        for value in 0..8 {
            stack.push(value);
        }
        for _ in 0..8 {
            assert!(stack.pop().is_some());
        }
        assert!(stack.is_empty());

        // The stack must still accept new elements after being drained.
        stack.push(42);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop(), Some(42));
    }
}
