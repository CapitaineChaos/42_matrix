use std::fmt;

use crate::types::Matrix;

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for row in 0..self.rows {
            write!(f, "  [")?;
            for col in 0..self.cols {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self[(row, col)])?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    }
}