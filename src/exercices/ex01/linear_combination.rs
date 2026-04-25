
// pub trait LinearCombination<K>: Sized {
//     fn lc(u: &[Self], coeffs: &[K]) -> Self;
// }

// pub fn linear_combination<K, T>(u: &[T], coeffs: &[K]) -> T
// where T: LinearCombination<K>,
// {
//     T::lc(u, coeffs)
// }



pub trait LinearCombination<K>: Sized {
    fn lc(items: &[&Self], coeffs: &[K]) -> Self;
}

pub fn linear_combination<K, T>(items: &[&T], coeffs: &[K]) -> T
where T: LinearCombination<K>,
{
    T::lc(items, coeffs)
}
