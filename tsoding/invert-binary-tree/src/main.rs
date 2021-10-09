type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, Clone)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

fn generate_tree_nonrec(level: usize) -> NodeRef<i32> {
    let mut counter = 1;
    let mut arg_stack = Vec::<Action<usize, i32>>::new();
    let mut ret_stack = Vec::<NodeRef<i32>>::new();

    arg_stack.push(Action::Call(level));
    while let Some(action) = arg_stack.pop() {
        match action {
            Action::Call(level) => {
                if level > 0 {
                    arg_stack.push(Action::Handle(counter));
                    counter += 1;
                    arg_stack.push(Action::Call(level - 1));
                    arg_stack.push(Action::Call(level - 1));
                } else {
                    ret_stack.push(None);
                }
            }
            Action::Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node { value, left, right })));
            }
        }
    }

    ret_stack.pop().unwrap()
}

#[allow(dead_code)]
fn generate_tree_rec(level: usize, counter: &mut i32) -> NodeRef<i32> {
    if level == 0 {
        None
    } else {
        let mut node = Node {
            value: *counter,
            left: None,
            right: None,
        };
        *counter += 1;
        node.left = generate_tree_rec(level - 1, counter);
        node.right = generate_tree_rec(level - 1, counter);
        Some(Box::new(node))
    }
}

fn print_tree_nonrec<T: std::fmt::Display>(root: &NodeRef<T>) {
    let mut stack = Vec::<Action<(&NodeRef<T>, usize), (&T, usize)>>::new();
    stack.push(Action::Call((root, 0)));
    while let Some(action) = stack.pop() {
        match action {
            Action::Call((root, level)) => {
                if let Some(node) = root {
                    stack.push(Action::Call((&node.left, level + 1)));
                    stack.push(Action::Handle((&node.value, level)));
                    stack.push(Action::Call((&node.right, level + 1)));
                }
            }
            Action::Handle((value, level)) => {
                for _ in 0..level {
                    print!("    ")
                }
                println!("{}", value);
            }
        }
    }
}

#[allow(dead_code)]
fn print_tree_rec<T: std::fmt::Display>(root: &NodeRef<T>, level: usize) {
    if let Some(node) = root {
        print_tree_rec(&node.left, level + 1);
        for _ in 0..level {
            print!("    ")
        }
        println!("{}", node.value);
        print_tree_rec(&node.right, level + 1);
    }
}

fn invert_tree_nonrec<T: Clone>(root: &NodeRef<T>) -> NodeRef<T> {
    let mut arg_stack = Vec::<Action<&NodeRef<T>, &T>>::new();
    let mut ret_stack = Vec::<NodeRef<T>>::new();

    arg_stack.push(Action::Call(root));
    while let Some(action) = arg_stack.pop() {
        match action {
            Action::Call(root) => {
                if let Some(node) = root {
                    arg_stack.push(Action::Handle(&node.value));
                    arg_stack.push(Action::Call(&node.right));
                    arg_stack.push(Action::Call(&node.left));
                } else {
                    ret_stack.push(None);
                }
            }
            Action::Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node {
                    value: value.clone(),
                    left,
                    right,
                })));
            }
        }
    }

    ret_stack.pop().unwrap()
}

#[allow(dead_code)]
fn invert_tree_rec<T: Clone>(root: &NodeRef<T>) -> NodeRef<T> {
    if let Some(node) = root {
        Some(Box::new(Node {
            value: node.value.clone(),
            left: invert_tree_rec(&node.right),
            right: invert_tree_rec(&node.left),
        }))
    } else {
        None
    }
}

fn main() {
    let tree = generate_tree_nonrec(4);
    print_tree_nonrec(&tree);
    println!("--------------------------");
    print_tree_nonrec(&invert_tree_nonrec(&tree));
}
