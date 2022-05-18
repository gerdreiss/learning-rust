struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.document.find(self.delimiter) {
            let (first, second) = self.document.split_at(index);
            self.document = &second[self.delimiter.len()..];
            Some(first)
        } else {
            None
        }
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}

fn main() {
    println!("{:?}", str_before("hello world", ' '));
}
