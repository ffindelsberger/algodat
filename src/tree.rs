mod bst {
    //Implementation of a Binary Search Tree
    // - a BST does not allow duplicate values
    use std::{
        borrow::BorrowMut,
        cell::{Cell, RefCell},
        collections::VecDeque,
        rc::{Rc, Weak},
        thread::current,
    };

    type Link = Option<Rc<RefCell<Node>>>;
    type WeakLink = Option<Weak<RefCell<Node>>>;

    struct Node {
        parent: WeakLink,
        left_child: Link,
        right_child: Link,
        data: usize,
    }

    impl Node {
        fn new(value: usize) -> Self {
            Node {
                parent: Option::default(),
                left_child: Option::default(),
                right_child: Option::default(),
                data: value,
            }
        }
    }

    struct Tree {
        root: Link,
    }

    trait InorderIter {
        fn inorder_iter(&self) -> InorderIterator {
            todo!();
        }
    }

    impl Tree {
        fn new() -> Self {
            Tree {
                root: Option::default(),
            }
        }

        fn insert(&mut self, value: usize) {
            let mut new_node = Node::new(value);

            let Some(mut current) = self.root.clone() else {
                self.root = Some(Rc::new(RefCell::new(new_node)));
                return;
            };

            loop {
                let mut next_node = {
                    let current_ref = current.borrow();
                    match current_ref.data.cmp(&value) {
                        std::cmp::Ordering::Less => current_ref.left_child.clone(),
                        std::cmp::Ordering::Greater => current_ref.right_child.clone(),
                        std::cmp::Ordering::Equal => todo!(),
                    }
                };

                match next_node {
                    Some(node) => current = node,
                    None => {
                        next_node = Some(Rc::new(RefCell::new(new_node)));
                        return;
                    }
                }
            }
            todo!()
        }

        // This Method moves to the First Element for the Inorder Traversal
        // This means the resulting Iterator can already start at the correct element
        fn inorder(&self) -> InorderIterator {
            InorderIterator {
                stack: vec![&self.root],
            }
        }
    }

    struct InorderIterator<'a> {
        stack: Vec<&'a Link>,
    }

    impl<'a> Iterator for InorderIterator<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            todo!();
            // match self.stack {
            //     Some(node) => {
            //         let node_ref = node.borrow();
            //         let next = node_ref.data;

            //         return Some(next);
            //     }
            //     None => return None,
            // };
        }
    }

    mod test {
        use super::{InorderIter, InorderIterator, Tree};

        #[test]
        fn test_tree() {
            let mut tree = Tree::new();
            tree.insert(5);
            tree.insert(8);
            for value in tree.inorder() {
                println!("{}", value);
            }
        }
    }
}
