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

    /// Definition 3.3.2 - The size `size(t)` of a term `t`, i.e. the number of
    /// nodes in its abstract syntax tree.
    pub fn size(&self) -> usize {
        use T::*;
        match self {
        // size(true) = 1
        | True => 1,

        // size(false) = 1
        | False => 1,

        // size(0) = 1
        | Zero => 1,

        // size(succ t₁) = size(t₁) + 1
        | Succ(t_1) => t_1.size() + 1,

        // size(pred t₁) = size(t₁) + 1
        | Pred(t_1) => t_1.size() + 1,

        // size(iszero t₁) = size(t₁) + 1
        | IsZero(t_1) => t_1.size() + 1,

        // size(if t₁ then t₂ else t₃) = size(t₁) + size(t₂) + size(t₃) + 1
        | IfElse(t_1, t_2, t_3) => t_1.size()
            + t_2.size()
            + t_3.size()
            + 1,
        }
    }
}
