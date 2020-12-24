extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[derive(Debug)]
struct Row {
    name: String,
    ttype: String,
}

fn parse_row<I: Iterator<Item = TokenTree>>(i: &mut I) -> Option<Row> {
    let name = i.next()?.to_string();
    if &name == "pub" {
        name = i.next()?.to_string();
    }
    i.next()?;
    let mut ttype = String::new();
    let mut d = 0;
    loop {
        match i.next() {
            None => return Some(Row { name, ttype }),
            Some(TokenTree::Punct(p)) => match p.as_char() {
                ',' => {
                    if d == 0 {
                        return Some(Row { name, ttype });
                    } else {
                        ttype.push(',');
                    }
                }
                '<' | '(' => {
                    d += 1;
                    ttype.push(p.as_char());
                }
                '>' | ')' => {
                    d -= 1;
                    ttype.push(p.as_char());
                }
                c => ttype.push(c),
            },
            Some(v) => ttype.push_str(&v.to_string()),
        }
    }
}

#[proc_macro_derive(Setter)]
pub fn setter_derive(input: TokenStream) -> TokenStream {
    let mut top = input.into_iter();
    let mut ttype: TokenTree = top.next().unwrap();
    if ttype.to_string() == "pub".to_string() {
        ttype = top.next().unwrap();
    }
    let name = top.next().unwrap();

    let mut row_iter = if let Some(TokenTree::Group(g)) = top.next() {
        g.stream().into_iter()
    } else {
        panic!();
    };

    let mut rows: Vec<Row> = Vec::new();
    while let Some(r) = parse_row(&mut row_iter) {
        eprintln!("Row = {:?}", r);
        rows.push(r);
    }

    let lines: String = rows
        .iter()
        .map(|r| {
            format!(
                "fn set_{0}(&mut self, v:{1}) {{
                    self.{0} = v;
                }}\n",
                r.name, r.ttype
            )
        })
        .collect();

    let res = format!(
        "impl {} {{
            {}
        }}",
        name, lines
    );

    eprintln!("Res = {}", res);

    res.parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
