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
