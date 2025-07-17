pub mod income;
pub mod voting;

use unicode_width::UnicodeWidthChar;

fn format_node_name(name: &str, width: usize) -> String {
    let cleaned: String = if name.trim().is_empty() {
        "-".to_string()
    } else {
        name.chars()
            .filter(|c| c.width().unwrap_or(0) <= 1)
            .collect()
    };

    let visible_width = cleaned
        .chars()
        .map(|c| c.width().unwrap_or(0))
        .sum::<usize>();
    if visible_width >= width {
        let mut result = String::new();
        let mut current = 0;
        for c in cleaned.chars() {
            let w = c.width().unwrap_or(0);
            if current + w >= width - 1 {
                result.push('…');
                break;
            }
            result.push(c);
            current += w;
        }
        result
    } else {
        format!("{:<width$}", cleaned, width = width)
    }
}
