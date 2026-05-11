use std::fmt;

use crate::types::Vector;

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self[i])?;
        }
        write!(f, "]")
    }
}