use std::{marker::PhantomData, ptr::NonNull};

// SAFETY INVARIANTS:
//
// 1. Every node pointer refers to a live allocation originally
//    created by Box::into_raw.
//
// 2. Each node is uniquely owned by this list and reclaimed
//    exactly once.
//
// 3. Starting from head, following next visits every node
//    exactly once and terminates at None.
//
// 4. tail points to the last node, whose next is None.
//
// 5. head and tail are both None exactly when len == 0.
//
// 6. len equals the number of nodes reachable from head.

pub struct LinkedList<T> {
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>,
    len: usize,
    // use marker to document the intended structure
    marker: PhantomData<Box<LinkedListNode<T>>>,
}

struct LinkedListNode<T> {
    elem: T,
    next: Option<NonNull<LinkedListNode<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // while let Some(_) = self.pop() {}
        // It's suggested by Clippy to change from the commented
        // version to the following version:
        //
        //  In the while let version:
        //    the result of `pop()` is held through the loop body.
        //  In the `.is_some()` version:
        //    the result is dropped before entering the loop body.
        while self.pop().is_some() {}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0usize,
            marker: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            self.len += 1;
            let new_tail = Box::into_raw(Box::new(LinkedListNode { elem, next: None }));

            // Some actions that dependent on whether the list already contains some elements.
            if self.tail.is_some() {
                (*self.tail.take().unwrap().as_ptr()).next = Some(NonNull::new_unchecked(new_tail));
            } else {
                self.head = Some(NonNull::new_unchecked(new_tail));
            }

            // Note:
            // It's important to understand that there are multiple ways
            // to construct the `NonNull<T>` here.
            //
            // You can start from a `Box`, and do it safely.
            //     ps: You can even retain ownership in the `Box`.
            // You can start form a raw pointer safely (handle the result!)
            // You can start from a raw pointer and leave it unchecked.
            // You can just start from a reference(but no ownership transfer!!)
            //
            // summary:
            // from -- no ownership transfer! (References!!)
            // new  -- handles the ownership issue
            self.tail = Some(NonNull::new_unchecked(new_tail));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            self.head.take().map(|ptr_to_old_head| {
                self.len -= 1;
                let head = Box::from_raw(ptr_to_old_head.as_ptr());
                self.head = head.next;

                if self.head.is_none() {
                    self.tail = None;
                }

                head.elem
            })
        }
    }
}

// =============== All kinds of iterators ===============
// ==== Data Structures Needed
// 1. IntoIter
pub struct LinkedListIntoIter<T>(LinkedList<T>);

// 2. Iter
pub struct LinkedListIter<'a, T> {
    current: Option<NonNull<LinkedListNode<T>>>,
    marker: PhantomData<&'a T>,
}

// 3. IterMut
pub struct LinkedListIterMut<'a, T> {
    current: Option<NonNull<LinkedListNode<T>>>,
    marker: PhantomData<&'a mut T>,
}

// === Implementations for IntoIter ===
impl<T> Iterator for LinkedListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = LinkedListIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIntoIter(self)
    }
}

impl<T> LinkedList<T> {
    fn into_iter(self) -> LinkedListIntoIter<T> {
        <Self as IntoIterator>::into_iter(self)
    }
}

// === Implementation for Iter ===
impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current_ptr| unsafe {
            self.current = (*current_ptr.as_ptr()).next;
            &(*current_ptr.as_ptr()).elem
        })
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = LinkedListIter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current: self.head.as_ref().map(|head| *head),
            marker: PhantomData,
        }
    }
}

impl<'a, T> LinkedList<T> {
    pub fn iter(&'a self) -> LinkedListIter<'a, T> {
        self.into_iter()
    }
}

// === Implementation for IterMut ===
impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current_ptr| unsafe {
            self.current = (*current_ptr.as_ptr()).next;
            &mut (*current_ptr.as_ptr()).elem
        })
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = LinkedListIterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current: self.head.as_ref().map(|head| *head),
            marker: PhantomData,
        }
    }
}

impl<'a, T> LinkedList<T> {
    fn iter_mut(&'a mut self) -> LinkedListIterMut<'a, T> {
        self.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mutable_iteration() {
        let mut list = LinkedList::new();

        list.push(10);
        list.push(20);

        let mut iter = list.iter_mut();

        let first = iter.next().unwrap();
        let second = iter.next().unwrap();

        *first += 1;
        *second += 2;

        assert_eq!((*first, *second), (11, 22));
    }
}
