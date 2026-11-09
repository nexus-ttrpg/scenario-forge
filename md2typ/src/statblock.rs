//! Stat block mini-syntax -> Typst.
//!
//! ```text
//! SCRAP HOUND (2) | CR 1/2
//! N Small animal
//! Init +3; Senses low-light vision; Perception +4
//!
//! DEFENSE | HP 13 each
//! EAC 10; KAC 12
//! OFFENSE
//! Speed 40 ft.
//! SPECIAL ABILITIES
//! Scent Salvage (Ex) A scrap hound can smell worked metal through a
//!   bulkhead, and always knows where the nearest powered device is.
//! ```
//!
//! * The first non-blank line is the black name bar; text after `|` is the CR.
//! * An ALL-CAPS line is a ruled section bar; text after `|` sits at its right
//!   (that is where `HP` goes).
//! * Everything else is a stat line. Known keywords at the start of the line,
//!   and after each `; `, are bolded automatically.
//! * A line indented by two or more spaces continues the line above it.

use crate::inline::{inline, strlit};

/// Longest first, so `Init` wins over `Int`.
const KEYWORDS: &[&str] = &[
    "Spell-Like Abilities", "Defensive Abilities", "Offensive Abilities",
    "Other Abilities", "Before Combat", "During Combat", "Spells Known",
    "Augmentations", "Organization", "Perception DC", "Resistances",
    "Multiattack", "Environment", "Immunities", "Weaknesses", "Languages",
    "Perception", "Treasure", "Trigger", "Disable", "Effect", "Morale",
    "Senses", "Skills", "Speed", "Spells", "Space", "Reach", "Melee",
    "Ranged", "Reset", "Feats", "Aura", "Init", "Gear", "Type", "Fort",
    "Will", "EAC", "KAC", "Ref", "Str", "Dex", "Con", "Int", "Wis", "Cha",
    "DR", "SR", "HP", "RP", "XP", "CR",
];

const ABILITY_TAGS: &[&str] = &["(Ex)", "(Su)", "(Sp)", "(Ex/Su)", "(Su/Sp)"];

fn is_section_head(s: &str) -> bool {
    let t = s.trim();
    if t.len() < 3 {
        return false;
    }
    let mut has_alpha = false;
    for c in t.chars() {
        if c.is_alphabetic() {
            has_alpha = true;
            if !c.is_uppercase() {
                return false;
            }
        } else if !(c.is_ascii_digit() || c == ' ' || c == '-' || c == '&' || c == '/') {
            return false;
        }
    }
    has_alpha
}

/// Bold a leading keyword in one `; `-separated segment.
fn bold_keyword(seg: &str) -> String {
    let trimmed = seg.trim_start();
    let lead = &seg[..seg.len() - trimmed.len()];
    if trimmed.starts_with("**") {
        return format!("{}{}", lead, inline(trimmed)); // author already marked it up
    }
    for kw in KEYWORDS {
        if let Some(rest) = trimmed.strip_prefix(kw) {
            let next_ok = rest.is_empty() || rest.starts_with(|c: char| !c.is_alphanumeric());
            if next_ok {
                return format!("{}#strong[{}]{}", lead, inline(kw), inline(rest));
            }
        }
    }
    format!("{}{}", lead, inline(trimmed))
}

/// In SPECIAL ABILITIES, bold the name up to and including the `(Ex)` tag.
fn bold_ability(line: &str) -> Option<String> {
    for tag in ABILITY_TAGS {
        if let Some(pos) = line.find(tag) {
            if pos <= 64 {
                let head = &line[..pos + tag.len()];
                let rest = &line[pos + tag.len()..];
                return Some(format!("#strong[{}]{}", inline(head), inline(rest)));
            }
        }
    }
    None
}

pub fn convert(body: &str, autobold: bool) -> String {
    // Fold continuation lines (indented by two or more spaces) into their parent.
    let mut lines: Vec<String> = Vec::new();
    for raw in body.lines() {
        let is_cont = raw.starts_with("  ") && !raw.trim().is_empty();
        if is_cont {
            if let Some(last) = lines.last_mut() {
                if !last.trim().is_empty() {
                    last.push(' ');
                    last.push_str(raw.trim());
                    continue;
                }
            }
        }
        lines.push(raw.trim_end().to_string());
    }

    let mut out = String::from("#statblock[\n");
    let mut titled = false;
    let mut in_abilities = false;

    for line in lines {
        let t = line.trim();
        if t.is_empty() {
            if titled {
                out.push_str("  #sb-gap()\n");
            }
            continue;
        }

        let (left, right) = match t.split_once('|') {
            Some((l, r)) => (l.trim(), Some(r.trim())),
            None => (t, None),
        };

        if !titled {
            out.push_str(&format!("  #sb-title({}", strlit(left)));
            if let Some(cr) = right {
                if !cr.is_empty() {
                    out.push_str(&format!(", cr: {}", strlit(cr)));
                }
            }
            out.push_str(")\n");
            titled = true;
            continue;
        }

        if is_section_head(left) {
            in_abilities = left.contains("ABILITIES");
            out.push_str(&format!("  #sb-head({}", strlit(left)));
            if let Some(tr) = right {
                if !tr.is_empty() {
                    out.push_str(&format!(", trailing: [{}]", render_line(tr, autobold, false)));
                }
            }
            out.push_str(")\n");
            continue;
        }

        out.push_str(&format!("  #sb-line[{}]\n", render_line(t, autobold, in_abilities)));
    }

    out.push_str("]\n");
    out
}

fn render_line(text: &str, autobold: bool, abilities: bool) -> String {
    if !autobold {
        return inline(text);
    }
    if abilities {
        if let Some(s) = bold_ability(text) {
            return s;
        }
    }
    text.split("; ")
        .map(bold_keyword)
        .collect::<Vec<_>>()
        .join("; ")
}
