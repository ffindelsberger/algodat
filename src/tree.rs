mod bst {
    //Implementation of a Binary Search Tree
    // - a BST does not allow duplicate values
    use std::{
        borrow::BorrowMut,
        cell::{Cell, RefCell},
        rc::{Rc, Weak},
        thread::current,
    };

    struct Node {
        parent: Option<Weak<Cell<Node>>>,
        left_child: Option<Rc<Cell<Node>>>,
        right_child: Option<Rc<Node>>,
        value: usize,
    }

    impl Node {
        fn new(value: usize) -> Self {
            Node {
                parent: Option::default(),
                left_child: Option::default(),
                right_child: Option::default(),
                value,
            }
        }
    }

    struct Tree {
        root: Option<Rc<Cell<Node>>>,
    }

    impl Tree {
        fn new() -> Self {
            Tree {
                root: Option::default(),
            }
        }

        fn insert(&mut self, value: usize) {
            let mut new_node = Node::new(value);

            let Some(current) = self.root else {
                self.root = Some(Rc::new(Cell::new(new_node)));
                return;
            };

            loop {
                match current.borrow_mut().value.cmp(value) {
                    std::cmp::Ordering::Less => {
                        //Add inorder Sucessor
                        let Some(current) = &current.left_child else {
                            new_node.parent = Some(Rc::downgrade(current));
                            current.left_child = Some(Rc::new(Cell::new(new_node)));
                            break;
                        };
                    }
                    std::cmp::Ordering::Equal => {
                        //add inorder predecessor
                        todo!();
                    }
                    std::cmp::Ordering::Greater => break,
                }
            }
            todo!()
        }
    }
}
