type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default, Clone)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

enum Action<T, U> {
    Call(T),
    Handle(U),
}

fn generate_tree(level: usize, counter: &mut i32) -> NodeRef<i32> {
    if level == 0 {
        None
    } else {
        let mut node = Node {
            value: *counter,
            left: None,
            right: None,
        };
        *counter += 1;
        node.left = generate_tree(level - 1, counter);
        node.right = generate_tree(level - 1, counter);
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

fn print_tree<T: std::fmt::Display>(root: &NodeRef<T>, level: usize) {
    if let Some(node) = root {
        print_tree(&node.left, level + 1);
        for _ in 0..level {
            print!("    ")
        }
        println!("{}", node.value);
        print_tree(&node.right, level + 1);
    }
}

fn invert_tree<T: Clone>(root: NodeRef<T>) -> NodeRef<T> {
    root.map(|node| {
        Box::new(Node {
            value: node.value.clone(),
            left: invert_tree(node.right),
            right: invert_tree(node.left),
        })
    })
}

fn main() {
    let mut counter = 1;
    let tree = generate_tree(4, &mut counter);
    print_tree_nonrec(&tree);
    println!("--------------------------");
    print_tree_nonrec(&invert_tree(tree));
}
