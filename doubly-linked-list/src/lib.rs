use std::ptr::NonNull;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

// Node structure for the doubly linked list
struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element,
            next: None,
            prev: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: Option<NonNull<Node<T>>>,
}

pub struct Iter<'a, T> {
    current: Option<NonNull<Node<T>>>,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            _phantom: std::marker::PhantomData,
        }
    }

    // Helper methods for push/pop operations used by pre_implemented module
    fn push_front_node(&mut self, mut node: NonNull<Node<T>>) {
        // SAFETY: We just created this node, so it's valid
        unsafe {
            node.as_mut().next = self.head;
            node.as_mut().prev = None;
        }

        match self.head {
            Some(mut head) => {
                // SAFETY: head is valid as it's in our list
                unsafe {
                    head.as_mut().prev = Some(node);
                }
            }
            None => {
                self.tail = Some(node);
            }
        }

        self.head = Some(node);
        self.len += 1;
    }

    fn push_back_node(&mut self, mut node: NonNull<Node<T>>) {
        // SAFETY: We just created this node, so it's valid
        unsafe {
            node.as_mut().next = None;
            node.as_mut().prev = self.tail;
        }

        match self.tail {
            Some(mut tail) => {
                // SAFETY: tail is valid as it's in our list
                unsafe {
                    tail.as_mut().next = Some(node);
                }
            }
            None => {
                self.head = Some(node);
            }
        }

        self.tail = Some(node);
        self.len += 1;
    }

    fn pop_front_node(&mut self) -> Option<NonNull<Node<T>>> {
        self.head.map(|head| {
            // SAFETY: head is valid as it's in our list
            let next = unsafe { head.as_ref().next };

            match next {
                Some(mut next) => {
                    // SAFETY: next is valid as it's in our list
                    unsafe {
                        next.as_mut().prev = None;
                    }
                    self.head = Some(next);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }

            self.len -= 1;
            head
        })
    }

    fn pop_back_node(&mut self) -> Option<NonNull<Node<T>>> {
        self.tail.map(|tail| {
            // SAFETY: tail is valid as it's in our list
            let prev = unsafe { tail.as_ref().prev };

            match prev {
                Some(mut prev) => {
                    // SAFETY: prev is valid as it's in our list
                    unsafe {
                        prev.as_mut().next = None;
                    }
                    self.tail = Some(prev);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }

            self.len -= 1;
            tail
        })
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.current.map(|mut node| {
            // SAFETY: node is valid as it's in our list
            unsafe { &mut node.as_mut().element }
        })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if let Some(current) = self.current {
            // SAFETY: current is valid as it's in our list
            self.current = unsafe { current.as_ref().next };
        }
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if let Some(current) = self.current {
            // SAFETY: current is valid as it's in our list
            self.current = unsafe { current.as_ref().prev };
        }
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let current = self.current?;

        // SAFETY: current is valid as it's in our list
        let (next, prev) = unsafe {
            let node = current.as_ref();
            (node.next, node.prev)
        };

        // Update adjacent nodes
        match prev {
            Some(mut prev_node) => {
                // SAFETY: prev_node is valid as it's in our list
                unsafe {
                    prev_node.as_mut().next = next;
                }
            }
            None => {
                self.list.head = next;
            }
        }

        match next {
            Some(mut next_node) => {
                // SAFETY: next_node is valid as it's in our list
                unsafe {
                    next_node.as_mut().prev = prev;
                }
            }
            None => {
                self.list.tail = prev;
            }
        }

        self.list.len -= 1;

        // Move cursor to next element, or prev if next doesn't exist
        self.current = next.or(prev);

        // SAFETY: current is valid as we just removed it from our list
        let node = unsafe { Box::from_raw(current.as_ptr()) };
        Some(node.element)
    }

    pub fn insert_after(&mut self, element: T) {
        let mut new_node = NonNull::new(Box::into_raw(Box::new(Node::new(element))))
            .expect("Box::into_raw should never return null");

        match self.current {
            Some(mut current) => {
                // SAFETY: current is valid as it's in our list
                let next = unsafe { current.as_ref().next };

                // SAFETY: new_node is valid as we just created it
                unsafe {
                    new_node.as_mut().next = next;
                    new_node.as_mut().prev = Some(current);
                }

                // SAFETY: current is valid as it's in our list
                unsafe {
                    current.as_mut().next = Some(new_node);
                }

                match next {
                    Some(mut next_node) => {
                        // SAFETY: next_node is valid as it's in our list
                        unsafe {
                            next_node.as_mut().prev = Some(new_node);
                        }
                    }
                    None => {
                        self.list.tail = Some(new_node);
                    }
                }

                self.list.len += 1;
            }
            None => {
                // Empty list or cursor at ghost position
                self.list.push_back_node(new_node);
                self.current = Some(new_node);
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        let mut new_node = NonNull::new(Box::into_raw(Box::new(Node::new(element))))
            .expect("Box::into_raw should never return null");

        match self.current {
            Some(mut current) => {
                // SAFETY: current is valid as it's in our list
                let prev = unsafe { current.as_ref().prev };

                // SAFETY: new_node is valid as we just created it
                unsafe {
                    new_node.as_mut().next = Some(current);
                    new_node.as_mut().prev = prev;
                }

                // SAFETY: current is valid as it's in our list
                unsafe {
                    current.as_mut().prev = Some(new_node);
                }

                match prev {
                    Some(mut prev_node) => {
                        // SAFETY: prev_node is valid as it's in our list
                        unsafe {
                            prev_node.as_mut().next = Some(new_node);
                        }
                    }
                    None => {
                        self.list.head = Some(new_node);
                    }
                }

                self.list.len += 1;
            }
            None => {
                // Empty list or cursor at ghost position
                self.list.push_front_node(new_node);
                self.current = Some(new_node);
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.current.map(|node| {
            // SAFETY: node is valid as it's in our list and we hold a reference
            let node_ref = unsafe { node.as_ref() };
            self.current = node_ref.next;
            &node_ref.element
        })
    }
}

// Drop implementation to prevent memory leaks
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.pop_front_node() {
            // SAFETY: We just removed this node from our list
            let _node = unsafe { Box::from_raw(node.as_ptr()) };
            // The Box will be dropped here, properly dropping the contained element
        }
    }
}

// Default implementation
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
