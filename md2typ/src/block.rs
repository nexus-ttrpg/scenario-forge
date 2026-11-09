//! Block-level Markdown -> Typst.

use crate::inline::{inline, strlit};
use crate::statblock;
use std::collections::BTreeMap;

/// `name key=value key="value with spaces" free text title`
pub struct Info {
    pub name: String,
    pub title: String,
    pub attrs: BTreeMap<String, String>,
}

pub fn parse_info(s: &str) -> Info {
    let s = s.trim();
    let (name, rest) = match s.split_once(char::is_whitespace) {
        Some((n, r)) => (n.to_string(), r.trim()),
        None => (s.to_string(), ""),
    };

    let mut attrs = BTreeMap::new();
    let mut title_parts: Vec<String> = Vec::new();
    let chars: Vec<char> = rest.chars().collect();
    let mut i = 0;
    let mut token = String::new();

    let flush = |tok: &mut String, attrs: &mut BTreeMap<String, String>, title: &mut Vec<String>| {
        if tok.is_empty() {
            return;
        }
        match tok.split_once('=') {
            Some((k, v)) if !k.is_empty() && k.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') => {
                attrs.insert(k.to_string(), v.trim_matches('"').to_string());
            }
            _ => title.push(tok.clone()),
        }
        tok.clear();
    };

    while i < chars.len() {
        let c = chars[i];
        if c == '"' {
            token.push('"');
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                token.push(chars[i]);
                i += 1;
            }
            token.push('"');
            i += 1;
            continue;
        }
        if c.is_whitespace() {
            flush(&mut token, &mut attrs, &mut title_parts);
            i += 1;
            continue;
        }
        token.push(c);
        i += 1;
    }
    flush(&mut token, &mut attrs, &mut title_parts);

    let title = attrs
        .remove("title")
        .unwrap_or_else(|| title_parts.join(" ").trim_matches('"').to_string());

    Info { name, title, attrs }
}

fn fence_at(line: &str) -> Option<(char, usize, String)> {
    let t = line.trim_start();
    for ch in ['`', '~'] {
        let n = t.chars().take_while(|&c| c == ch).count();
        if n >= 3 {
            return Some((ch, n, t[n..].trim().to_string()));
        }
    }
    None
}

fn is_table_delim(line: &str) -> bool {
    let t = line.trim();
    t.contains('|')
        && t.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
        && t.contains('-')
}

fn split_row(line: &str) -> Vec<String> {
    let t = line.trim().trim_start_matches('|').trim_end_matches('|');
    t.split('|').map(|c| c.trim().to_string()).collect()
}

fn bool_attr(info: &Info, key: &str, default: bool) -> bool {
    match info.attrs.get(key).map(|s| s.as_str()) {
        Some("true") | Some("yes") | Some("on") => true,
        Some("false") | Some("no") | Some("off") => false,
        _ => default,
    }
}

pub fn convert(src: &str) -> String {
    let lines: Vec<&str> = src.lines().collect();
    let mut out = String::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // ---- fenced block -------------------------------------------------
        if let Some((ch, n, info_str)) = fence_at(line) {
            let info = parse_info(&info_str);
            let mut body = Vec::new();
            i += 1;
            while i < lines.len() {
                if let Some((c2, n2, _)) = fence_at(lines[i]) {
                    if c2 == ch && n2 >= n && lines[i].trim().chars().all(|c| c == ch) {
                        i += 1;
                        break;
                    }
                }
                body.push(lines[i]);
                i += 1;
            }
            out.push_str(&fenced(&info, &body.join("\n")));
            continue;
        }

        // ---- heading ------------------------------------------------------
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            let text = trimmed[level..].trim();
            let f = match level {
                1 => "section",
                2 => "subsection",
                3 => "subtier",
                _ => "minorhead",
            };
            out.push_str(&format!("#{}[{}]\n\n", f, inline(text)));
            i += 1;
            continue;
        }

        // ---- blockquote (read-aloud) ---------------------------------------
        if trimmed.starts_with('>') {
            let mut buf = Vec::new();
            while i < lines.len() {
                let t = lines[i].trim();
                if t.starts_with('>') {
                    buf.push(t[1..].trim_start().to_string());
                    i += 1;
                } else if t.is_empty() && buf.last().map_or(false, |l: &String| !l.is_empty()) {
                    // a blank line ends the quote unless the next line resumes it
                    if lines.get(i + 1).map_or(false, |n| n.trim().starts_with('>')) {
                        buf.push(String::new());
                        i += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            out.push_str("#readaloud[\n");
            out.push_str(&convert(&buf.join("\n")));
            out.push_str("]\n\n");
            continue;
        }

        // ---- table ----------------------------------------------------------
        if trimmed.contains('|')
            && lines.get(i + 1).map_or(false, |l| is_table_delim(l))
        {
            let header = split_row(lines[i]);
            i += 2;
            let mut rows: Vec<Vec<String>> = Vec::new();
            while i < lines.len() && lines[i].trim().contains('|') && !lines[i].trim().is_empty() {
                rows.push(split_row(lines[i]));
                i += 1;
            }
            let ncols = header.len();
            let cols: Vec<&str> = (0..ncols)
                .map(|c| if c + 1 == ncols { "1fr" } else { "auto" })
                .collect();
            out.push_str(&format!("#datatable(\n  columns: ({},),\n", cols.join(", ")));
            out.push_str(&format!(
                "  header: ({},),\n",
                header.iter().map(|h| format!("[{}]", inline(h))).collect::<Vec<_>>().join(", ")
            ));
            out.push_str("  rows: (\n");
            for r in rows {
                let cells: Vec<String> = (0..ncols)
                    .map(|c| format!("[{}]", inline(r.get(c).map(|s| s.as_str()).unwrap_or(""))))
                    .collect();
                out.push_str(&format!("    ({},),\n", cells.join(", ")));
            }
            out.push_str("  ),\n)\n\n");
            continue;
        }

        // ---- list --------------------------------------------------------
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
            while i < lines.len() {
                let t = lines[i].trim();
                if t.starts_with("- ") || t.starts_with("* ") || t.starts_with("+ ") {
                    let indent = lines[i].len() - lines[i].trim_start().len();
                    out.push_str(&format!(
                        "{}- {}\n",
                        " ".repeat(indent),
                        inline(&t[2..])
                    ));
                    i += 1;
                } else if !t.is_empty() && lines[i].starts_with("  ") {
                    out.push_str(&format!("  {}\n", inline(t))); // continuation
                    i += 1;
                } else {
                    break;
                }
            }
            out.push('\n');
            continue;
        }

        // ---- thematic break -------------------------------------------------
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            out.push_str("#line(length: 100%, stroke: 0.5pt + palette.hairline)\n\n");
            i += 1;
            continue;
        }

        // ---- paragraph ------------------------------------------------------
        // A line ending in `\` or two spaces is a hard break, as in Markdown.
        let mut para: Vec<(String, bool)> = Vec::new();
        while i < lines.len() {
            let raw = lines[i];
            let t = raw.trim();
            if t.is_empty()
                || t.starts_with('#')
                || t.starts_with('>')
                || t.starts_with("- ")
                || t.starts_with("* ")
                || fence_at(raw).is_some()
            {
                break;
            }
            let (text, brk) = if let Some(stripped) = t.strip_suffix('\\') {
                (stripped.trim_end().to_string(), true)
            } else if raw.ends_with("  ") {
                (t.to_string(), true)
            } else {
                (t.to_string(), false)
            };
            para.push((text, brk));
            i += 1;
        }
        // Inline markup may span source lines, so convert the joined
        // paragraph and only then restore the hard breaks.
        const BRK: char = '\u{0}';
        let mut joined = String::new();
        for (n, (text, brk)) in para.iter().enumerate() {
            joined.push_str(text);
            if *brk {
                joined.push(BRK);
            } else if n + 1 < para.len() {
                joined.push(' ');
            }
        }
        out.push_str(&inline(&joined).replace(BRK, " #linebreak()\n"));
        out.push_str("\n\n");
    }
    out
}

fn fenced(info: &Info, body: &str) -> String {
    let title = if info.title.trim().is_empty() {
        None
    } else {
        Some(info.title.trim().to_string())
    };

    match info.name.as_str() {
        "typst" => format!("{}\n\n", body),

        "statblock" => {
            let autobold = bool_attr(info, "autobold", true);
            format!("{}\n", statblock::convert(body, autobold))
        }

        "sidebar" | "scaling" | "planet" => {
            let f = match info.name.as_str() {
                "sidebar" => "sidebar",
                "scaling" => "scalingbox",
                _ => "databar",
            };
            let mut args: Vec<String> = Vec::new();
            if let Some(t) = &title {
                if f == "databar" {
                    args.push(format!("title: {}", strlit(t)));
                } else {
                    args.push(format!("title: [{}]", inline(t)));
                }
            }
            if f != "databar" {
                let default_float = f == "sidebar";
                args.push(format!("float: {}", bool_attr(info, "float", default_float)));
            }
            format!("#{}({})[\n{}]\n\n", f, args.join(", "), convert(body))
        }

        "map" => {
            let mut args: Vec<String> = Vec::new();
            if let Some(t) = &title {
                args.push(format!("title: {}", strlit(t)));
            }
            if let Some(c) = info.attrs.get("credit") {
                args.push(format!("credit: {}", strlit(c)));
            }
            if let Some(s) = info.attrs.get("scale") {
                args.push(format!("scale: {}", strlit(s)));
            }
            if let Some(src) = info.attrs.get("src") {
                args.push(format!("src: {}", strlit(src)));
            }
            args.push(format!(
                "height: {}",
                info.attrs.get("height").map(|s| s.as_str()).unwrap_or("300pt")
            ));
            args.push(format!("span: {}", bool_attr(info, "span", true)));
            format!("#maparea({})\n\n", args.join(", "))
        }

        other => format!(
            "// md2typ: unknown block `{}` passed through as plain text\n{}\n\n",
            other,
            convert(body)
        ),
    }
}
