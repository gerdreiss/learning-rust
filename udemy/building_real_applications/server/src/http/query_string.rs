use crate::parse::error::ParseError;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> TryFrom<&'buf str> for QueryString<'buf> {
    type Error = ParseError;

    fn try_from(value: &'buf str) -> Result<QueryString<'buf>, Self::Error> {
        let data: HashMap<&'buf str, Value<'buf>> = value
            .split('&')
            .filter(|v| !v.is_empty())
            .map(|s| s.splitn(2, '='))
            .map(|mut s| (s.next(), s.next()))
            .filter(|(maybe_name, _)| maybe_name.is_some())
            .map(|(maybe_name, maybe_value)| {
                (
                    maybe_name.unwrap(),             // here we know that maybe_name is not None
                    maybe_value.unwrap_or_default(), // this could an empty value, therefore a None
                )
            })
            .fold(HashMap::new(), |mut map, (name, value)| {
                map.entry(name)
                    .and_modify(|existing| match existing {
                        Value::Single(prev) => *existing = Value::Multiple(vec![prev, value]),
                        Value::Multiple(prev) => prev.push(value),
                    })
                    .or_insert(Value::Single(value));
                map
            });

        Ok(QueryString { data })
    }
}
