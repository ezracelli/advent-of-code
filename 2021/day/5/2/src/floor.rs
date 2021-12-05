use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

pub struct Floor(Vec<Vec<usize>>);

impl Debug for Floor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                match self.0[i][j] {
                    0 => write!(f, ".")?,
                    n => write!(f, "{}", n)?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

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
