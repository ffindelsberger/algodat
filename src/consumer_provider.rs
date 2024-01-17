use std::sync::{Condvar, Mutex, MutexGuard};

/// A classic implementation of a Consumer Provider Queue/Stack using a single Mutex and
/// Conditional Variables for Signaling
///
//https://www.reddit.com/r/rust/comments/2581s5/informal_survey_which_is_clearer_mutability_or/
//https://www.appsloveworld.com/rust/55/rust-cannot-borrow-data-in-arc-as-mutable-but-inner-data-is-protected-via-mut?expand_article=1
struct WorkStack<T> {
    inner: Mutex<Vec<T>>,
    reader: Condvar,
    writer: Condvar,
}

impl<T> WorkStack<T> {
    fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(Vec::with_capacity(capacity)),
            reader: Condvar::new(),
            writer: Condvar::new(),
        }
    }

    /// self does not need to be mut here. We do not need a unique Reference as we can aquire a
    /// referene through the underlying Mutex in inner.
    /// If self would be mut here we also would encounter problems later when trying to call "read"
    /// through an Arc as it is not possible to aquire a mut reference through an Arc
    fn read(&self) -> T {
        let mut inner = self
            .inner
            .lock()
            .expect("Could not aquire Lock for read operation");

        while inner.is_empty() {
            inner = self
                .reader
                .wait(inner)
                .expect("Could not wait on Conditional while in Read operation")
        }

        //https://stackoverflow.com/questions/17101922/do-i-have-to-acquire-lock-before-calling-condition-variable-notify-one
        let res = inner.pop().expect("Read operation, list was empty.");
        core::mem::drop(inner);
        self.writer.notify_one();
        return res;
    }

    fn add(&self, task: T) {
        let mut inner_guard = self
            .inner
            .lock()
            .expect("Could not aquire the Lock in write operation");

        loop {
            match inner_guard.len() >= inner_guard.capacity() {
                true => inner_guard = self.writer.wait(inner_guard).expect("error"),
                false => {
                    inner_guard.push(task);
                    core::mem::drop(inner_guard);
                    self.reader.notify_one();
                    return;
                }
            }
        }
    }
}

impl<T> Default for WorkStack<T> {
    fn default() -> Self {
        Self {
            inner: Mutex::new(Vec::with_capacity(10)),
            reader: Default::default(),
            writer: Default::default(),
        }
    }
}

#[cfg(test)]
mod concurrency_tests {
    use std::{sync::Arc, thread, time::Duration};

    use super::WorkStack;

    #[test]
    fn test() {
        let stack = Arc::new(WorkStack::new(10));

        let stack_1 = stack.clone();
        let _ = thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            stack_1.add(5);
            println!("Added 5 to the stack")
        });

        let stack_2 = stack.clone();
        let _ = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            stack_2.add(6);
            println!("Added 6 to the stack")
        });

        let stack_3 = stack.clone();
        let _ = thread::spawn(move || {
            let val = stack_3.read();
            println!("read {} to the stack", val);
        });

        thread::sleep(Duration::from_secs(10))
    }
}
