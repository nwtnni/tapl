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
