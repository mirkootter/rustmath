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

pub fn parse_metadata<'a>(s: &'a str) -> Option<Vec<(&'a str, String)>> {
    let (_, result) = parse_metadata_imp(s).ok()?;
    Some(result)
}

type ParseResult<'a, T> = nom::IResult<&'a str, T>;

fn whitespace(src: &str) -> ParseResult<()> {
    use nom::InputTakeAtPosition;
    let (src, _) = src.split_at_position_complete(|ch| !ch.is_whitespace())?;
    Ok((src, ()))
}

fn parse_attr<'a>(src: &'a str) -> ParseResult<(&'a str, &'a str)> {
    let (src, ()) = whitespace(src)?;
    let (src, id) = nom::character::complete::alpha1(src)?;
    let (src, _) =
        nom::sequence::delimited(whitespace, nom::character::complete::char('='), whitespace)(src)?;

    let (src, value) = nom::sequence::delimited(
        nom::character::complete::char('"'),
        nom::bytes::complete::take_until("\""),
        nom::character::complete::char('"'),
    )(src)?;

    let (src, _) = whitespace(src)?;

    Ok((src, (id, value)))
}

fn skip_attrs(src: &str) -> ParseResult<()> {
    nom::multi::fold_many0(parse_attr, || (), |_, _| ())(src)
}

fn parse_escaped_string(src: &str) -> ParseResult<String> {
    let parse_number = nom::combinator::map_opt(nom::character::complete::digit1, |s: &str| {
        s.parse::<u8>().ok()
    });
    let parse_number_as_char = nom::combinator::map_res(parse_number, char::try_from);

    let parse_escape_sequence = nom::sequence::delimited(
        nom::bytes::complete::tag("&#"),
        parse_number_as_char,
        nom::character::complete::char(';'),
    );

    let parse_escaped_char = nom::branch::alt((
        parse_escape_sequence,
        nom::character::complete::none_of("<"),
    ));

    nom::multi::fold_many0(parse_escaped_char, String::new, |mut result, ch| {
        result.push(ch);
        result
    })(src)
}

fn metadata_tag<'a>(src: &'a str) -> ParseResult<'a, (&'a str, String)> {
    let (src, _) =
        nom::sequence::preceded(whitespace, nom::bytes::complete::tag("<metadata "))(src)?;
    let (src2, (key, value)) = parse_attr(src)?;

    if key != "id" {
        let err = nom::error::make_error(src2, nom::error::ErrorKind::Tag);
        return Err(nom::Err::Error(err));
    }

    let (src, _) = nom::sequence::preceded(whitespace, nom::bytes::complete::tag(">"))(src2)?;
    let (src, content) = parse_escaped_string(src)?;
    let (src, _) = nom::bytes::complete::tag("</metadata>")(src)?;

    Ok((src, (value, content)))
}

fn parse_metadata_imp<'a>(src: &'a str) -> ParseResult<'a, Vec<(&'a str, String)>> {
    let (src, _) = nom::sequence::preceded(whitespace, nom::bytes::complete::tag("<svg "))(src)?;
    let (src, ()) = skip_attrs(src)?;
    let (src, _) = nom::sequence::preceded(whitespace, nom::bytes::complete::tag(">"))(src)?;

    nom::multi::many0(metadata_tag)(src)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_skip_attrs() {
        let src = "  version=\"1.1\" xy=\"hello world\"  > 12";
        let (src, ()) = super::skip_attrs(src).unwrap();
        assert_eq!(src, "> 12");
    }

    #[test]
    fn test_parse_metadata() {
        let source = concat!(
            "<svg version=\"1.1\" width=\"554.2304pt\" height=\"97.192pt\" viewBox=\"0 0 554.2304 97.192\" xmlns=\"...\">\n",
            "  <metadata id=\"source\">rustmath</metadata>\n",
            "  <metadata id=\"rustmath_src\">...</metadata>\n",
        );

        let metadata = super::parse_metadata(source).unwrap();
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0], ("source", "rustmath".to_owned()));
        assert_eq!(metadata[1], ("rustmath_src", "...".to_owned()));
    }
}
