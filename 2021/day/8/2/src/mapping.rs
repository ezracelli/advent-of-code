use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mapping(BTreeMap<char, char>);

impl Mapping {
    pub fn builder() -> MappingBuilder {
        MappingBuilder::default()
    }
}

impl Deref for Mapping {
    type Target = BTreeMap<char, char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
pub struct MappingBuilder(BTreeMap<char, BTreeSet<char>>);

impl MappingBuilder {
    pub fn contains(&self, char: &char) -> bool {
        self.0
            .values()
            .flatten()
            .collect::<BTreeSet<_>>()
            .contains(char)
    }

    pub fn permutations(&self) -> BTreeSet<Mapping> {
        let mut set = BTreeSet::new();

        for a in self[&'a'].iter() {
            for b in self[&'b'].iter() {
                if b == a {
                    continue;
                }

                for c in self[&'c'].iter() {
                    if c == a || c == b {
                        continue;
                    }

                    for d in self[&'d'].iter() {
                        if d == c || d == b || d == a {
                            continue;
                        }

                        for e in self[&'e'].iter() {
                            if e == d || e == c || e == b || e == a {
                                continue;
                            }

                            for f in self[&'f'].iter() {
                                if f == e || f == d || f == c || f == b || f == a {
                                    continue;
                                }

                                for g in self[&'g'].iter() {
                                    if g == f || g == e || g == d || g == c || g == b || g == a {
                                        continue;
                                    }

                                    set.insert(Mapping(
                                        [
                                            ('a', *a),
                                            ('b', *b),
                                            ('c', *c),
                                            ('d', *d),
                                            ('e', *e),
                                            ('f', *f),
                                            ('g', *g),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        set
    }
}

impl Deref for MappingBuilder {
    type Target = BTreeMap<char, BTreeSet<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MappingBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
