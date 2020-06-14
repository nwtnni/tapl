use std::collections::HashSet;

use maplit::hashset;

/// The set of terms `T`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum T<'a> {
    /// Constant true.
    ///
    /// ```text
    ///
    /// --------
    /// true ∈ T
    /// ```
    True,

    /// Constant false.
    ///
    /// ```text
    ///
    /// ---------
    /// false ∈ T
    /// ```
    False,

    /// Constant zero.
    ///
    /// ```text
    ///
    /// -----
    /// 0 ∈ T
    /// ```
    Zero,

    /// Successor.
    ///
    /// ```text
    ///   t₁ ∈ T
    /// -----------
    /// succ t₁ ∈ T
    /// ```
    Succ(&'a T<'a>),

    /// Predecessor.
    ///
    /// ```text
    ///   t₁ ∈ T
    /// -----------
    /// pred t₁ ∈ T
    /// ```
    Pred(&'a T<'a>),

    /// Zero test.
    ///
    /// ```text
    ///    t₁ ∈ T
    /// -------------
    /// iszero t₁ ∈ T
    /// ```
    IsZero(&'a T<'a>),

    /// Conditional.
    ///
    /// ```text
    /// t₁ ∈ T    t₂ ∈ T    t₃ ∈ T
    /// --------------------------
    /// if t₁ then t₂ else t₃ ∈ T
    /// ```
    IfElse(&'a T<'a>, &'a T<'a>, &'a T<'a>),
}

impl<'a> T<'a> {
    /// Definition 3.3.1 - The set of constants `Consts(t)` appearing in a term `t`.
    pub fn consts(&self) -> HashSet<T<'a>> {
        use T::*;
        match self {
        // Consts(true) = { true }
        | True => hashset!(True),

        // Consts(false) = { false }
        | False => hashset!(False),

        // Consts(0) = { 0 }
        | Zero => hashset!(Zero),

        // Consts(succ t₁) = Consts(t₁)
        | Succ(t_1) => t_1.consts(),

        // Consts(pred t₁) = Consts(t₁)
        | Pred(t_1) => t_1.consts(),

        // Consts(iszero t₁) = Consts(t₁)
        | IsZero(t_1) => t_1.consts(),

        // Consts(if t₁ then t₂ else t₃) = Consts(t₁) ∪ Consts(t₂) ∪ Consts(t₃)
        | IfElse(t_1, t_2, t_3) => t_1.consts()
            .into_iter()
            .chain(t_2.consts())
            .chain(t_3.consts())
            .collect(),
        }
    }
}
