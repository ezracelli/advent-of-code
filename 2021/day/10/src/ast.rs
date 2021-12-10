use std::ops::{Deref, DerefMut};

pub mod parsers {
    use nom::branch::alt;
    use nom::character::complete::char;
    use nom::combinator::{cut, map};
    use nom::error::{context, ContextError, ParseError};
    use nom::multi::many0;
    use nom::sequence::{preceded, terminated};
    use nom::IResult;

    pub mod contexts {
        pub const ANGLE_BRACKET: &str = "ANGLE_BRACKET";
        pub const CURLY_BRACKET: &str = "CURLY_BRACKET";
        pub const PAREN: &str = "PAREN";
        pub const SQUARE_BRACKET: &str = "SQUARE_BRACKET";
    }

    fn angle_bracket<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Vec<super::Chunk>, E> {
        context(
            self::contexts::ANGLE_BRACKET,
            preceded(char('<'), cut(terminated(chunks, char('>')))),
        )(input)
    }

    fn curly_bracket<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Vec<super::Chunk>, E> {
        context(
            self::contexts::CURLY_BRACKET,
            preceded(char('{'), cut(terminated(chunks, char('}')))),
        )(input)
    }

    fn paren<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Vec<super::Chunk>, E> {
        context(
            self::contexts::PAREN,
            preceded(char('('), cut(terminated(chunks, char(')')))),
        )(input)
    }

    fn square_bracket<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Vec<super::Chunk>, E> {
        context(
            self::contexts::SQUARE_BRACKET,
            preceded(char('['), cut(terminated(chunks, char(']')))),
        )(input)
    }

    fn chunk<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, super::Chunk, E> {
        alt((
            map(angle_bracket, super::Chunk::AngleBracket),
            map(curly_bracket, super::Chunk::CurlyBracket),
            map(paren, super::Chunk::Paren),
            map(square_bracket, super::Chunk::SquareBracket),
        ))(input)
    }

    pub fn chunks<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, Vec<super::Chunk>, E> {
        many0(chunk)(input)
    }
}

#[derive(Debug)]
pub struct Line(Vec<Chunk>);

impl Deref for Line {
    type Target = Vec<Chunk>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub enum Chunk {
    AngleBracket(Vec<Chunk>),
    CurlyBracket(Vec<Chunk>),
    Paren(Vec<Chunk>),
    SquareBracket(Vec<Chunk>),
}

impl Deref for Chunk {
    type Target = Vec<Chunk>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::AngleBracket(chunks) => chunks,
            Self::Paren(chunks) => chunks,
            Self::SquareBracket(chunks) => chunks,
            Self::CurlyBracket(chunks) => chunks,
        }
    }
}

impl DerefMut for Chunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::AngleBracket(chunks) => chunks,
            Self::Paren(chunks) => chunks,
            Self::SquareBracket(chunks) => chunks,
            Self::CurlyBracket(chunks) => chunks,
        }
    }
}
