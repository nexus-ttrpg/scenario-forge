//! A deliberately small YAML subset for scenario front matter.
//!
//! Supported shapes:
//!
//! ```text
//! key: scalar value
//! key: "quoted value"
//! key:
//!   - list item
//!   - another
//! key: >
//!   folded block: blank lines separate paragraphs,
//!   single newlines are joined.
//! key: |
//!   literal block: newlines are kept.
//! ```
//!
//! Anything else is reported as an error rather than silently ignored.

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    List(Vec<String>),
    Text(String),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) | Value::Text(s) => Some(s),
            Value::List(_) => None,
        }
    }
    pub fn as_list(&self) -> Vec<String> {
        match self {
            Value::List(v) => v.clone(),
            Value::Str(s) | Value::Text(s) => vec![s.clone()],
        }
    }
}

pub type Front = BTreeMap<String, Value>;

fn unquote(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2
        && ((t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')))
    {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

/// Split a document into (front matter, body). Front matter is the block
/// between a leading `---` line and the next `---` line.
pub fn split(src: &str) -> (Option<String>, String) {
    let mut lines = src.lines();
    match lines.next() {
        Some(l) if l.trim() == "---" => {}
        _ => return (None, src.to_string()),
    }
    let rest: Vec<&str> = src.lines().skip(1).collect();
    if let Some(end) = rest.iter().position(|l| l.trim() == "---") {
        let front = rest[..end].join("\n");
        let body = rest[end + 1..].join("\n");
        (Some(front), body)
    } else {
        (None, src.to_string())
    }
}

pub fn parse(src: &str) -> Result<Front, String> {
    let mut map = Front::new();
    let lines: Vec<&str> = src.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let raw = lines[i];
        if raw.trim().is_empty() || raw.trim_start().starts_with('#') {
            i += 1;
            continue;
        }
        if raw.starts_with(char::is_whitespace) {
            return Err(format!("front matter line {} is indented but follows no key", i + 1));
        }
        let (key, rest) = match raw.split_once(':') {
            Some(p) => p,
            None => return Err(format!("front matter line {}: expected `key: value`", i + 1)),
        };
        let key = key.trim().to_string();
        let rest = rest.trim();
        i += 1;

        // block scalar
        if rest == ">" || rest == "|" {
            let folded = rest == ">";
            let mut buf: Vec<String> = Vec::new();
            while i < lines.len()
                && (lines[i].trim().is_empty() || lines[i].starts_with(char::is_whitespace))
            {
                buf.push(lines[i].trim().to_string());
                i += 1;
            }
            while buf.last().map_or(false, |l| l.is_empty()) {
                buf.pop();
            }
            let text = if folded {
                buf.split(|l| l.is_empty())
                    .map(|para| para.join(" "))
                    .filter(|p| !p.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join("\n\n")
            } else {
                buf.join("\n")
            };
            map.insert(key, Value::Text(text));
            continue;
        }

        // list
        if rest.is_empty() {
            let mut items = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() {
                    i += 1;
                    continue;
                }
                let t = l.trim_start();
                if !l.starts_with(char::is_whitespace) {
                    break;
                }
                if let Some(item) = t.strip_prefix("- ") {
                    items.push(unquote(item));
                } else if let Some(last) = items.last_mut() {
                    last.push(' ');
                    last.push_str(t);
                } else {
                    return Err(format!("front matter line {}: expected `- item`", i + 1));
                }
                i += 1;
            }
            map.insert(key, Value::List(items));
            continue;
        }

        map.insert(key, Value::Str(unquote(rest)));
    }
    Ok(map)
}
