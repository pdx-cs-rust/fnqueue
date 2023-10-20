pub trait Queue<T> {
    fn push_back(&mut self, v: T);
    fn pop_front(&mut self) -> Option<T>;
    fn peek_front(&self) -> Option<&T>;
}

/// Idea: Push onto the back vector. Pop from the front vector.
/// If the front vector is empty on pop, reverse the back vector
/// and make it the front vector.
pub struct FnQueue<T> {
    front: Vec<T>,
    back: Vec<T>,
}

impl<T> Default for FnQueue<T> {
    fn default() -> Self {
        FnQueue { front: Vec::new(), back: Vec::new() }
    }
}

impl<T> FnQueue<T> {
    fn unstack(&mut self) {
        while let Some(v) = self.back.pop() {
            self.front.push(v);
        }
    }
}

impl<T> Queue<T> for FnQueue<T> {
    fn push_back(&mut self, v: T) {
        self.back.push(v);
    }

    fn pop_front(&mut self) -> Option<T> {
        match self.front.pop() {
            Some(v) => Some(v),
            None => {
                self.unstack();
                self.front.pop()
            }
        }
    }

    fn peek_front(&self) -> Option<&T> {
        match self.front.last() {
            Some(v) => Some(v),
            None => self.front.first(),
        }
    }
}

#[test]
fn test_queue() {
    let mut q = FnQueue::default();
    for i in 1..=3 {
        q.push_back(i);
    }
    assert_eq!(Some(1), q.pop_front());
    q.push_back(4);
    assert_eq!(Some(&2), q.peek_front());
    for i in 2..=4 {
        assert_eq!(Some(i), q.pop_front());
    }
    assert!(q.peek_front().is_none());
    assert!(q.pop_front().is_none());
}
