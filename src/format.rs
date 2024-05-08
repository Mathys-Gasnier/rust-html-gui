use once_cell::sync::Lazy;
use regex::{Match, Regex};
use serde_json::Value;

pub fn fmt(context: &Value, str: String) -> String {

    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)([^\{]|^)(\{[a-zA-Z_]+?\})([^\}]|$)").unwrap());

    let matches: Vec<Match> = REGEX
        .captures_iter(str.as_str())
        .filter_map(|capture| capture.get(2))
        .collect();

    let mut output = str.clone();
    for mat in matches.iter().rev() {
        let key = mat.as_str().trim_matches([ '{', '}' ]);

        let replace_str = if let Some(value) = find_key(context, key) {
            as_str(value)
        } else {
            format!("NOT FOUND {{{}}}", key)
        };

        output.replace_range(mat.range(), &replace_str);
    }

    output
}

pub fn find_key<'a>(context: &'a Value, key: &'a str) -> Option<&'a Value> {
    match context {
        Value::Object(map) => map.get(key),
        Value::Array(arr) => {
            let n: usize = key.parse().unwrap();
            arr.get(n)
        },
        _ => None,
    }
}

fn as_str(value: &Value) -> String {
    match value {
        Value::Null => String::from("null"),
        Value::Bool(bool) => if *bool { String::from("true") } else { String::from("false") },
        Value::Number(number) => format!("{}", number),
        Value::String(string) => string.to_owned(),
        Value::Array(arr) => format!("Array[{}]", arr.len()),
        Value::Object(_) => String::from("Object{ .. }"),
    }
}