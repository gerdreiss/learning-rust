pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut st = Stepper {
        curr: 0,
        step: 2,
        max: 20,
    };

    // for i in st {
    //     println!("{}", i);
    // }

    while let Some(i) = st.next() {
        println!("{}", i);
    }
}
