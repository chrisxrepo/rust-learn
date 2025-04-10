#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.items.iter_mut()
    }

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.items.into_iter()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    for item in stack.iter() {
        println!("{}", item);
    }
    println!("{:?}", stack);

    for item in stack.iter_mut() {
        *item *= 2;
    }
    println!("{:?}", stack);

    for item in stack.into_iter() {
        println!("{}", item);
    }
    // println!("{:?}", stack);
}
