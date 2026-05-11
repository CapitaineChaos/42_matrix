use std::ops::{Add, Mul, Sub};

use num_traits::MulAdd;

/// Contrainte minimale pour un élément d'un espace vectoriel ou matriciel.
///
/// Regroupe les opérations nécessaires à la combinaison linéaire :
/// copie, valeur nulle par défaut, addition, soustraction, multiplication,
/// et fused multiply-add.
pub trait LinearElement:
    Copy
    + Default
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + MulAdd<Output = Self>
{
}

impl<T> LinearElement for T
where
    T: Copy
        + Default
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + MulAdd<Output = T>,
{
}
