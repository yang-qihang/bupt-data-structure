// used for problem 1 & problem 3
// a sequential stack
pub mod array_stack;

// used for problem 2
// a stack based on a singly linked list
pub mod linked_stack;

// used for problem 3
// a queue based on a singly linked list
// But the implementation is quite different from the
// stack used in problem 2.
pub mod linked_queue;

// Note:
// The implementation for the linked stack and queue
// are separated since to make the Enqueue operation O(1)
// we need to maintain a non-owning pointer to the head
// Since we want to provide good API, it's clumsy if we
// use Rc<RefCell<T>>, thus unsafe Rust is necessary for a good implementation.
// But a stack which is implemented using a signly linked list
// wouldn't require this, so Option<Box<T>> is enough.
