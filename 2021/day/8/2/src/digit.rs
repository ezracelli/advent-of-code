use std::borrow::Cow;
use std::ops::Deref;

use crate::mapping::Mapping;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digit(Cow<'static, str>);

impl Digit {
    pub const ZERO: Self = Digit(Cow::Borrowed("abcefg"));
    pub const ONE: Self = Digit(Cow::Borrowed("cf"));
    pub const TWO: Self = Digit(Cow::Borrowed("acdeg"));
    pub const THREE: Self = Digit(Cow::Borrowed("acdfg"));
    pub const FOUR: Self = Digit(Cow::Borrowed("bcdf"));
    pub const FIVE: Self = Digit(Cow::Borrowed("abdfg"));
    pub const SIX: Self = Digit(Cow::Borrowed("abdefg"));
    pub const SEVEN: Self = Digit(Cow::Borrowed("acf"));
    pub const EIGHT: Self = Digit(Cow::Borrowed("abcdefg"));
    pub const NINE: Self = Digit(Cow::Borrowed("abcdfg"));

    pub fn new<S: AsRef<str>>(digit: S) -> Self {
        Self(Cow::Owned(digit.as_ref().to_string()))
    }

    pub fn as_usize(&self) -> usize {
        match self {
            d if d == &Self::ZERO => 0,
            d if d == &Self::ONE => 1,
            d if d == &Self::TWO => 2,
            d if d == &Self::THREE => 3,
            d if d == &Self::FOUR => 4,
            d if d == &Self::FIVE => 5,
            d if d == &Self::SIX => 6,
            d if d == &Self::SEVEN => 7,
            d if d == &Self::EIGHT => 8,
            d if d == &Self::NINE => 9,
            _ => unreachable!("unknown digit"),
        }
    }

    pub fn is_valid(&self) -> bool {
        self == &Self::ZERO
            || self == &Self::ONE
            || self == &Self::TWO
            || self == &Self::THREE
            || self == &Self::FOUR
            || self == &Self::FIVE
            || self == &Self::SIX
            || self == &Self::SEVEN
            || self == &Self::EIGHT
            || self == &Self::NINE
    }

    pub fn map(&mut self, mapping: &Mapping) {
        self.0 = Cow::Owned(
            self.chars()
                .map(|char| match char {
                    c if c == mapping[&'a'] => 'a',
                    c if c == mapping[&'b'] => 'b',
                    c if c == mapping[&'c'] => 'c',
                    c if c == mapping[&'d'] => 'd',
                    c if c == mapping[&'e'] => 'e',
                    c if c == mapping[&'f'] => 'f',
                    c if c == mapping[&'g'] => 'g',
                    _ => unreachable!("unexpected char"),
                })
                .collect::<String>(),
        );
    }

    pub fn sort(&mut self) {
        let mut chars = self.chars().collect::<Vec<_>>();
        chars.sort();

        self.0 = Cow::Owned(chars.into_iter().collect());
    }
}

impl Deref for Digit {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
