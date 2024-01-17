pub mod bad_stack {
    use std::usize;

    #[derive(Debug)]
    enum BadList {
        Empty,
        Element(i32, Box<BadList>),
    }

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Link,
    }

    #[derive(Debug)]
    pub struct List {
        head: Link,
    }

    #[derive(Debug)]
    enum Link {
        Empty,
        More(Box<Node>),
    }

    impl List {
        fn new() -> Self {
            List { head: Link::Empty }
        }

        fn push(&mut self, value: i32) {
            let new_node = Node {
                value,
                //Because we can`t move out of self we have to put a value back to not leave it
                //partially initialized
                next: std::mem::replace(&mut self.head, Link::Empty),
            };

            // Having the extra step of replacing is not cool but for now thats the best we can do, we
            // can solve this problem using other tools later.
            self.head = Link::More(Box::new(new_node));
        }

        fn pop(&mut self) -> Option<i32> {
            match std::mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(node) => {
                    self.head = node.next;
                    Some(node.value)
                }
            }
        }
    }

    #[cfg(test)]
    mod bad_stack_tests {

        use super::{BadList, List};

        #[test]
        fn test_bad_list() {
            let list = BadList::Element(4, Box::new(BadList::Element(5, Box::new(BadList::Empty))));

            println!("{:?}", list);
        }

        #[test]
        fn test_push() {
            let mut list = List {
                head: super::Link::Empty,
            };

            println!("{:?}", list);

            list.push(10);

            println!("{:?}", list);
        }

        #[test]
        fn test_pop() {
            let mut list = List {
                head: super::Link::Empty,
            };

            println!("{:?}", list);

            list.push(10);

            println!("{:?}", list.pop());
        }
    }
}

pub mod ok_stack {

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    struct List<T> {
        head: Link<T>,
    }

    #[derive(Debug)]
    struct Node<T> {
        element: T,
        next: Link<T>,
    }

    impl<T> List<T> {
        fn new() -> Self {
            List { head: None }
        }

        fn push(&mut self, element: T) {
            let new_node = Some(Box::new(Node {
                element,
                next: self.head.take(),
            }));

            self.head = new_node;
        }

        fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.element
            })
        }

        ///instead of explicitly converting &Option<T> to a Option<&T> using match.
        ///we utilizy as_ref() and the dot operator before map, map takes care of the dereference
        fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.element)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| &mut node.element)
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();
            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }

    struct ListIterator<T>(List<T>);

    impl<T> Iterator for ListIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    impl<T> IntoIterator for List<T> {
        type Item = T;

        type IntoIter = ListIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            ListIterator(self)
        }
    }

    #[cfg(test)]
    mod ok_stack_tests {
        use crate::lists::ok_stack::List;

        #[test]
        fn test_ok_list() {}

        #[test]
        fn test_push() {}

        #[test]
        fn test_pop() {}

        #[test]
        fn peek() {
            let mut list = List::new();
            assert_eq!(list.peek(), None);
            assert_eq!(list.peek_mut(), None);
            list.push(1);
            list.push(2);
            list.push(3);

            assert_eq!(list.peek(), Some(&3));
            assert_eq!(list.peek_mut(), Some(&mut 3));

            list.peek_mut().map(|value| *value = 12);
        }
    }
}
