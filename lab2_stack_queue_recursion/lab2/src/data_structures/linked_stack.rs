// This is a singly linked list that can be used as a stack.
// So it supports operations such as:
// pop(), push(), new(), is_empty(), from(), iter().
//
struct LinkedListNode<T> {
    elem: T,
    next: Option<Box<LinkedListNode<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    // To express the semantics of "inserting something",
    // here the `to_insert` parameter is passed by value
    // So we move it rather than borrow it.
    pub fn push(&mut self, to_insert: T) {
        let tmp_new_node = Some(Box::new(LinkedListNode {
            elem: to_insert,
            next: self.head.take(),
        }));
        self.head = tmp_new_node;
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn pop(&mut self) -> Option<T> {
        // We need to consider two cases that cannot be handled together.
        // case 1: if the list is none -> return none
        // case 2: if the list has at least one element:
        //         pop this element
        //         mutate the head of the list
        self.head.take().map(|mut some_head_node| {
            self.head = some_head_node.next.take();
            some_head_node.elem
        })
    }
    pub fn from(mut source: Vec<T>) -> Self {
        let mut tmp_res = Self::new();
        while let Some(cur_elem) = source.pop() {
            tmp_res.push(cur_elem);
        }
        tmp_res
    }
    pub fn iter(&self) -> LinkedListIter<'_, T> {
        self.into_iter()
    }
}

// ===== Iterator =====
pub struct LinkedListIter<'a, T> {
    current: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            self.current = current.next.as_deref();
            &current.elem
        })
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = LinkedListIter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}
