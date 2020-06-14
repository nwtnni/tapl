use std::collections::HashSet;

use maplit::hashset;
use typed_arena::Arena;

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
    /// Definition 3.2.3 - The set `S_i` such that:
    ///
    /// ```text
    /// S_{0}     = ∅
    /// S_{i + 1} = { true, false, 0 }
    ///           ∪ { succ t₁, pred t₁, iszero t₁ | t₁ ∈ S_i }
    ///           ∪ { if t₁ then t₂ else t₃ | t₁, t₂, t₃ ∈ S_i }
    /// ```
    pub fn generate(arena: &'a Arena<T<'a>>, index: usize) -> HashSet<T<'a>> {
        use T::*;
        match index {
        | 0 => hashset!(),
        | j => {
            let i = j - 1;
            let s_i = Self::generate(arena, i);

            // { true, false, 0 }
            let mut s_j = hashset!(True, False, Zero);

            // ∪ { succ t₁, pred t₁, iszero t₁ | t₁ ∈ S_i }
            for &t_1 in &s_i {
                s_j.insert(Succ(arena.alloc(t_1)));
                s_j.insert(Pred(arena.alloc(t_1)));
                s_j.insert(IsZero(arena.alloc(t_1)));
            }

            // ∪ { if t₁ then t₂ else t₃ | t₁, t₂, t₃ ∈ S_i }
            for &t_1 in &s_i {
                for &t_2 in &s_i {
                    for &t_3 in &s_i {
                        s_j.insert(IfElse(arena.alloc(t_1), arena.alloc(t_2), arena.alloc(t_3)));
                    }
                }
            }

            s_j
        }
        }
    }

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

    /// Definition 3.3.2 - The depth `depth(t)` of a term `t`, i.e. the smallest
    /// `i` such that `t ∈ S_i` according to Definition 3.2.3.
    pub fn depth(&self) -> usize {
        use T::*;
        match self {
        // depth(true) = 1
        | True => 1,

        // depth(false) = 1
        | False => 1,

        // depth(0) = 1
        | Zero => 1,

        // depth(succ t₁) = depth(t₁) + 1
        | Succ(t_1) => t_1.depth() + 1,

        // depth(pred t₁) = depth(t₁) + 1
        | Pred(t_1) => t_1.depth() + 1,

        // depth(iszero t₁) = depth(t₁) + 1
        | IsZero(t_1) => t_1.depth() + 1,

        // depth(if t₁ then t₂ else t₃) = max(depth(t₁), depth(t₂), depth(t₃)) + 1
        | IfElse(t_1, t_2, t_3) => t_1.depth()
            .max(t_2.depth())
            .max(t_3.depth())
            + 1,
        }
    }
}
