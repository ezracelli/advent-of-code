use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Floor(Vec<Vec<usize>>);

impl Default for Floor {
    fn default() -> Self {
        if cfg!(test) {
            Self(vec![vec![0; 10]; 10])
        } else {
            Self(vec![vec![0; 1024]; 1024])
        }
    }
}

impl Deref for Floor {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Floor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
