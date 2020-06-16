use std::collections::HashSet;

use maplit::hashset;

/// The untyped lambda calculus.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum T<'a> {
    /// Variable.
    ///
    /// ```text
    /// x ∈ V
    /// -----
    /// x ∈ T
    /// ```
    Var(String),

    /// Abstraction.
    ///
    /// ```text
    /// t₁ ∈ T    x ∈ V
    /// ---------------
    ///   λx. t₁ ∈ T
    /// ```
    Abs(String, &'a T<'a>),

    /// Application.
    ///
    /// ```text
    /// t₁ ∈ T    t₂ ∈ T
    /// ----------------
    ///    t₁ t₂ ∈ T
    /// ```
    App(&'a T<'a>, &'a T<'a>),
}

impl<'a> T<'a> {
    /// Definition 5.3.2 - The set of free variables of a term `t`, written FV(t).
    pub fn free(&self) -> HashSet<String> {
        use T::*;
        match self {
        // FV(x) = { x }
        | Var(x) => hashset!(x.clone()),

        // FV(λx. t₁) = FV(t₁) \ { x }
        | Abs(x, t_1) => {
            let mut fvs = t_1.free();
            fvs.remove(x);
            fvs
        }

        // FV(t₁, t₂) = FV(t₁) ∪ FV(t₂)
        | App(t_1, t_2) => t_1.free()
            .into_iter()
            .chain(t_2.free())
            .collect(),
        }
    }
}
