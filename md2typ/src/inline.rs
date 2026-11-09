//! Inline Markdown -> Typst markup.

/// Characters that start markup in Typst and must be escaped in literal text.
const ESCAPE: &[char] = &[
    '\\', '#', '$', '*', '_', '`', '<', '>', '@', '[', ']', '~', '=',
];

fn push_escaped(out: &mut String, c: char) {
    if ESCAPE.contains(&c) {
        out.push('\\');
    }
    out.push(c);
}

/// Escape a string for use inside a Typst string literal.
pub fn strlit(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Convert one run of inline Markdown into Typst markup.
///
/// Supported: `**strong**`, `*emph*`, `_emph_`, `` `raw` ``, `[text](url)`,
/// and `\x` backslash escapes. Everything else is literal.
pub fn inline(src: &str) -> String {
    let b: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(src.len() + 16);
    let mut i = 0;

    while i < b.len() {
        let c = b[i];

        // backslash escape
        if c == '\\' && i + 1 < b.len() {
            push_escaped(&mut out, b[i + 1]);
            i += 2;
            continue;
        }

        // **strong**
        if c == '*' && i + 1 < b.len() && b[i + 1] == '*' {
            if let Some(end) = find_delim(&b, i + 2, "**") {
                let inner: String = b[i + 2..end].iter().collect();
                out.push_str("#strong[");
                out.push_str(&inline(&inner));
                out.push(']');
                i = end + 2;
                continue;
            }
        }

        // *emph* / _emph_
        if (c == '*' || c == '_') && i + 1 < b.len() {
            let d = c.to_string();
            if let Some(end) = find_delim(&b, i + 1, &d) {
                let inner: String = b[i + 1..end].iter().collect();
                if !inner.trim().is_empty() {
                    out.push_str("#emph[");
                    out.push_str(&inline(&inner));
                    out.push(']');
                    i = end + 1;
                    continue;
                }
            }
        }

        // `raw`
        if c == '`' {
            if let Some(end) = find_delim(&b, i + 1, "`") {
                let inner: String = b[i + 1..end].iter().collect();
                out.push_str("#raw(");
                out.push_str(&strlit(&inner));
                out.push(')');
                i = end + 1;
                continue;
            }
        }

        // [text](url)
        if c == '[' {
            if let Some(close) = find_delim(&b, i + 1, "]") {
                if b.get(close + 1) == Some(&'(') {
                    if let Some(paren) = find_delim(&b, close + 2, ")") {
                        let text: String = b[i + 1..close].iter().collect();
                        let url: String = b[close + 2..paren].iter().collect();
                        out.push_str("#link(");
                        out.push_str(&strlit(&url));
                        out.push_str(")[");
                        out.push_str(&inline(&text));
                        out.push(']');
                        i = paren + 1;
                        continue;
                    }
                }
            }
        }

        // `//` would start a Typst comment
        if c == '/' && b.get(i + 1) == Some(&'/') {
            out.push_str("\\/");
            i += 1;
            continue;
        }

        push_escaped(&mut out, c);
        i += 1;
    }

    // `-` and `+` only mean something to Typst at the start of a line, but
    // that is exactly where a converted run can land.
    if out.starts_with('-') || out.starts_with('+') {
        out.insert(0, '\\');
    }
    out
}

/// Find the next unescaped occurrence of `delim` at or after `from`.
fn find_delim(b: &[char], from: usize, delim: &str) -> Option<usize> {
    let d: Vec<char> = delim.chars().collect();
    let mut i = from;
    while i + d.len() <= b.len() {
        if b[i] == '\\' {
            i += 2;
            continue;
        }
        if b[i..i + d.len()] == d[..] {
            return Some(i);
        }
        i += 1;
    }
    None
}
