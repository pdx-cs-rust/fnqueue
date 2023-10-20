use fnqueue::*;

fn main() {
    let mut q = FnQueue::default();
    for i in 1..=3 {
        q.push_back(i);
    }
    for i in q.iter() {
        println!("{}", i);
    }
}
