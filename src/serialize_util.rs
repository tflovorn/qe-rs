pub fn push_bool_field(lines: &mut Vec<String>, name: &str, b: Option<bool>) {
    if let Some(b) = b {
        let val = if b {
            String::from(".true.")
        } else {
            String::from(".false.")
        };

        lines.push(format!("    {}={},", name, val));
    };
}
