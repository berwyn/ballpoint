use std::str;

use nom::{
    branch::alt,
    bytes::complete::{tag as tagged, take_until, take_while},
    character::{
        complete::{alpha1, alphanumeric1, line_ending, multispace0, not_line_ending},
        is_space,
    },
    combinator::{eof, map_res, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, terminated},
};

use token::Token;

mod token;

fn whitespace<I, F, O, E: nom::error::ParseError<I>>(
    inner: F,
) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    F: nom::Parser<I, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn identifier(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    recognize(pair(
        alt((alpha1, tagged("_"))),
        many0(alt((alphanumeric1, tagged("_")))),
    ))(input)
}

fn glue(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, _) = tagged("<>")(input)?;
    let (input, _) = take_while(is_space)(input)?;
    let (input, text) = map_res(not_line_ending, str::from_utf8)(input)?;

    Ok((input, Token::Glue(text.into())))
}

fn tag(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, _) = tagged("#")(input)?;
    let (input, _) = take_while(is_space)(input)?;
    let (input, text) = map_res(not_line_ending, str::from_utf8)(input)?;

    Ok((input, Token::Tag(text.into())))
}

fn choice(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, _) = tagged("*")(input)?;
    let (input, _) = take_while(is_space)(input)?;
    let (input, text) = map_res(not_line_ending, str::from_utf8)(input)?;

    Ok((input, Token::Choice(text.into())))
}

fn line_comment(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, _) = tagged("//")(input)?;
    let (input, _) = take_while(is_space)(input)?;
    let (input, text) = map_res(not_line_ending, str::from_utf8)(input)?;

    Ok((input, Token::Comment(text.into())))
}

fn block_comment(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, text) = map_res(
        delimited(tagged("/*"), take_until("*/"), tagged("*/")),
        str::from_utf8,
    )(input)?;

    Ok((input, Token::Comment(text.into())))
}

fn comment(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, token) = alt((line_comment, block_comment))(input)?;

    Ok((input, token))
}

fn suppression(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, text) = map_res(
        delimited(tagged("["), take_until("]"), tagged("]")),
        str::from_utf8,
    )(input)?;

    Ok((input, Token::Suppression(text.into())))
}

fn divert(input: &[u8]) -> nom::IResult<&[u8], Token> {
    let (input, _) = tagged("->")(input)?;
    let (input, ident) = map_res(whitespace(identifier), str::from_utf8)(input)?;

    Ok((input, Token::Divert(ident.into())))
}

fn line(input: &[u8]) -> nom::IResult<&[u8], Token> {
    alt((comment, choice))(input)
}

fn document(input: &[u8]) -> nom::IResult<&[u8], Vec<Token>> {
    many1(terminated(line, alt((line_ending, eof))))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_glue() {
        let text = b"<> test";
        let parsed = glue(text).unwrap();

        assert_eq!((&[] as &[u8], Token::Glue("test".into())), parsed);
    }

    #[test]
    fn it_parses_tags() {
        let text = b"# this is a tag\n";
        let parsed = tag(text).unwrap();

        assert_eq!((b"\n" as &[u8], Token::Tag("this is a tag".into())), parsed);
    }

    #[test]
    fn it_parses_choices() {
        let text = b"* this is a choice\r\n";
        let parsed = choice(text).unwrap();

        assert_eq!(
            (b"\r\n" as &[u8], Token::Choice("this is a choice".into())),
            parsed
        );
    }

    #[test]
    fn it_parses_comments() {
        let text = b"// test";
        let parsed = comment(text).unwrap();

        assert_eq!((&[] as &[u8], Token::Comment("test".into())), parsed);

        let text = b"/*
            test
        */";
        let parsed = block_comment(text).unwrap();

        assert_eq!(
            (
                &[] as &[u8],
                Token::Comment(
                    r#"
            test
        "#
                    .into()
                )
            ),
            parsed
        );
    }

    #[test]
    fn it_parses_suppressions() {
        let text = b"[test]";
        let parsed = suppression(text).unwrap();

        assert_eq!(((&[] as &[u8], Token::Suppression("test".into()))), parsed);
    }

    #[test]
    fn it_parses_diverts() {
        let text = b"-> to_the_place";
        let parsed = divert(text).unwrap();

        assert_eq!((&[] as &[u8], Token::Divert("to_the_place".into())), parsed);
    }

    #[test]
    fn it_parses_a_line() {
        let text = b"// test";
        let parsed = line(text).unwrap();

        assert_eq!((&[] as &[u8], Token::Comment("test".into())), parsed);

        let text = b"* test";
        let parsed = line(text).unwrap();

        assert_eq!((&[] as &[u8], Token::Choice("test".into())), parsed);
    }

    #[test]
    fn it_parses_a_document() {
        let text = r#"
// This is a sample document
* First choice
* Second choice
"#
        .trim();

        let expected = vec![
            Token::Comment("This is a sample document".into()),
            Token::Choice("First choice".into()),
            Token::Choice("Second choice".into()),
        ];

        assert_eq!((&[] as &[u8], expected), document(text.as_bytes()).unwrap());
    }
}
