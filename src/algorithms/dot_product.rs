use crate::traits::LinearElement;
use crate::types::Vector;

pub fn dot_product<K: LinearElement>(u: &Vector<K>, v: &Vector<K>) -> K {
    assert_eq!(u.size(), v.size());
    u.data.iter().zip(v.data.iter())
        .fold(K::default(), |acc, (&a, &b)| acc + a * b)
}

impl<K: LinearElement> Vector<K> {
    pub fn dot(&self, rhs: &Vector<K>) -> K {
        dot_product(self, rhs)
    }
}

