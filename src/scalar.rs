use std::fmt;

use crate::complexe::Complexe;

pub trait ScalarFormat {
    fn fmt_scalar(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl ScalarFormat for f32 {
    fn fmt_scalar(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>9.3}", self)
    }
}

impl ScalarFormat for Complexe {
    fn fmt_scalar(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}