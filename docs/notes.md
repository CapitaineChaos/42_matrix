impl<'a, K: LinearElement>   Lerp<&'a Matrix<K>, K>   for &'a Matrix<K>
│                            │                         │
│                            │                         └─ "pour le type &Matrix<K>"
│                            │                            (self = une référence vers une matrice)
│                            │
│                            └─ "on implémente le trait Lerp"
│                               - 1er param: Rhs = &Matrix<K>  (le type de rhs)
│                               - 2e param:  K             (le type du scalaire t)
│
└─ déclaration des paramètres génériques utilisés dans le reste :
   - 'a  : une durée de vie (les deux références doivent vivre aussi longtemps)
   - K   : un type quelconque qui respecte LinearElement



Pour tout type K qui est un LinearElement, j'implémente lerp(self, rhs: &Matrix<K>, t: K) sur &Matrix<K>.

dans Lerp<&'a Matrix<K>, K> :
Le trait est défini Lerp<Rhs, K> — le premier K c'est le type des éléments de la matrice, le second K c'est le type du scalaire t. Ici ils sont identiques (on lerp une Matrix<f32> avec un t: f32)



---




zip

[1, 2, 3].iter().zip([10, 20, 30].iter())
// → (1,10), (2,20), (3,30)




---

```rust
impl<K: Copy + Default + Add<Output = K> + Mul<Output = K>> Vector<K> {
    pub fn dot<V: AsRef<Vector<K>>>(&self, rhs: V) -> K {
        self.data.iter().zip(rhs.as_ref().data.iter())
            .fold(K::default(), |acc, (&a, &b)| acc + a * b)
    }
}
```

`impl<K: Copy + Default + Add<Output = K> + Mul<Output = K>> Vector<K>`
Pour tout type K qui sait être copié, avoir une valeur zéro (Default), s'additionner et se multiplier.

`self.data.iter().zip(rhs.as_ref().data.iter())`
Assemble les deux vecteurs en paires : (a₀,b₀), (a₁,b₁), ...

`.fold(K::default(), |acc, (&a, &b)| acc + a * b)`
fold parcourt les paires en maintenant un accumulateur :
K::default() → valeur de départ (le zéro du type)|acc, (&a, &b)| → closure : acc = valeur courante, (&a, &b) = la paire (le & devant désassemble les références)

Les |...| syntaxe de closure (fonction anonyme) équivalent des lambdas :
|acc, (&a, &b)| acc + a * b
// ^ paramètres ^  ^ corps ^
Pareil qu'en Python : lambda acc, a, b: acc + a * b
---