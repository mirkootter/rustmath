pub fn write_escaped_string(s: &str, out: &mut impl core::fmt::Write) -> core::fmt::Result {
    for ch in s.chars() {
        let needs_escape = match ch {
            '\t' | '\n' | '\r' => false,
            '\0'..='\x1A' => true,
            '<' | '>' | '&' => true,
            '\u{0085}' => false,
            '\x7F'..='\u{009F}' => true,
            '\u{FDD0}'..='\u{FDEF}' => true,
            _ => {
                let ch = (ch as u32) & 0xFFFF;
                ch == 0xFFFE || ch == 0xFFFF
            }
        };

        if needs_escape {
            write!(out, "&#{};", ch as u32)?;
        } else {
            out.write_char(ch)?;
        }
    }

    Ok(())
}
