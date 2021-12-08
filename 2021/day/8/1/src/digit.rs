use std::borrow::Cow;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
pub struct Digit(Cow<'static, str>);

impl Digit {
    #[allow(unused)]
    pub const ZERO: Self = Digit(Cow::Borrowed("abcefg"));
    pub const ONE: Self = Digit(Cow::Borrowed("cf"));
    #[allow(unused)]
    pub const TWO: Self = Digit(Cow::Borrowed("acdeg"));
    #[allow(unused)]
    pub const THREE: Self = Digit(Cow::Borrowed("acdfg"));
    pub const FOUR: Self = Digit(Cow::Borrowed("bcdf"));
    #[allow(unused)]
    pub const FIVE: Self = Digit(Cow::Borrowed("abdfg"));
    #[allow(unused)]
    pub const SIX: Self = Digit(Cow::Borrowed("abdefg"));
    pub const SEVEN: Self = Digit(Cow::Borrowed("acf"));
    pub const EIGHT: Self = Digit(Cow::Borrowed("abcdefg"));
    #[allow(unused)]
    pub const NINE: Self = Digit(Cow::Borrowed("abcdfg"));

    pub fn new<S: AsRef<str>>(digit: S) -> Self {
        Self(Cow::Owned(digit.as_ref().to_string()))
    }
}

impl Deref for Digit {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
