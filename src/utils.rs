use colored::*;

pub fn colorize_substring(haystack: &str, needle: &str, color: Color) -> String{
    let start_index = haystack.find(needle).unwrap_or(0);

    let before_substring = &haystack[..start_index];
    let after_substring = &haystack[start_index + needle.len()..];

    let colored_needle = needle.color(color);

    format!(
        "{}{}{}",
        before_substring,
        colored_needle,
        after_substring
    )
}
