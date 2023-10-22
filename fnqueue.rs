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
        FnQueue {
            front: Vec::new(),
            back: Vec::new(),
        }
    }
}

enum Posn {
    InFront(usize),
    InBack(usize),
    Empty,
}

pub struct FnQueueIter<'a, T> {
    q: &'a FnQueue<T>,
    posn: Posn,
}

impl<'a, T> Iterator for FnQueueIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.posn {
            Posn::InFront(i) => Some(&self.q.front[i]),
            Posn::InBack(i) => Some(&self.q.back[i]),
            Posn::Empty => None,
        };
        self.posn = match self.posn {
            Posn::InFront(i) => {
                if i > 0 {
                    Posn::InFront(i - 1)
                } else {
                    Posn::InBack(0)
                }
            }
            Posn::InBack(i) => {
                if i < self.q.back.len() - 1 {
                    Posn::InBack(i + 1)
                } else {
                    Posn::Empty
                }
            }
            Posn::Empty => Posn::Empty,
        };
        result
    }
}

impl<'a, T> IntoIterator for &'a FnQueue<T> {
    type Item = &'a T;
    type IntoIter = FnQueueIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> FnQueue<T> {
    fn unstack(&mut self) {
        assert!(self.front.is_empty());
        std::mem::swap(&mut self.front, &mut self.back);
        self.front.reverse();
    }

    pub fn iter(&self) -> FnQueueIter<T> {
        let posn = if !self.front.is_empty() {
            Posn::InFront(self.front.len() - 1)
        } else if !self.back.is_empty() {
            Posn::InBack(0)
        } else {
            Posn::Empty
        };
        FnQueueIter { q: self, posn }
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

impl<T> Iterator for FnQueue<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop_front()
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
