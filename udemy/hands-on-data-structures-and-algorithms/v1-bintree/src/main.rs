use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    h: u8, // Could be boolean for Red and Black, but maths is easier with small int.
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinData<T> {
    pub fn rot_left(mut self) -> Box<Self> {
        // result is the right node
        let mut res = match self.right.0.take() {
            Some(res) => res,
            None => return Box::new(self), // No right node - how can we rotate?
        };
        // move left of right node to right of start node
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        // set the results left node to the start node
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
    pub fn rot_right(mut self) -> Box<Self> {
        // result is the left node
        let mut res = match self.left.0.take() {
            Some(res) => res,
            None => return Box::new(self), // No left node - how can we rotate?
        };
        // move left of right node to right of start node
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        // set the results right node to the start node
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();
        res.h = 1 + std::cmp::max(res.left.height(), res.right.height());
        res
    }
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }
    pub fn height(&self) -> u8 {
        match self.0 {
            Some(ref t) => t.h,
            None => 0,
        }
    }
    pub fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.h = 1 + std::cmp::max(t.left.height(), t.right.height());
        }
    }
    pub fn rot_left(&mut self) {
        self.0 = self.0.take().map(|x| x.rot_left());
    }
    pub fn rot_right(&mut self) {
        self.0 = self.0.take().map(|x| x.rot_right());
    }
}

enum Direction {
    LEFT,
    RIGHT,
    NONE,
}

impl<T: PartialOrd> BinTree<T> {
    pub fn add_sorted(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.add_sorted(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        Direction::RIGHT
                    } else {
                        Direction::NONE
                    }
                } else {
                    bd.right.add_sorted(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        Direction::LEFT
                    } else {
                        Direction::NONE
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData {
                    data,
                    h: 0,
                    left: BinTree::new(),
                    right: BinTree::new(),
                }));
                Direction::NONE
            }
        };
        match rot_dir {
            Direction::LEFT => self.rot_left(),
            Direction::RIGHT => self.rot_right(),
            Direction::NONE => self.set_height(),
        }
    }
}

impl<T: Debug> BinTree<T> {
    pub fn print_lfirst(&self, dp: i32) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(dp + 1);
            let mut spc = String::new();
            for _ in 0..dp {
                spc.push('.');
            }
            println!("{}:{}{:?}", bd.h, spc, bd.data);
            bd.right.print_lfirst(dp + 1);
        }
    }
}

fn main() {
    let mut t = BinTree::new();
    for i in 0..100000 {
        t.add_sorted(i);
    }

    t.print_lfirst(0);
}
