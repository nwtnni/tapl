use crate::term;

use typed_arena::Arena;

impl<'a> term::T<'a> {
    /// The one-step evaluation relation.
    pub fn step(&self, arena: &'a Arena<term::T<'a>>) -> Option<term::T<'a>> {
        use term::T::*;
        let next = match self {
        //
        // ----------------------------- (E-IfTrue)
        // if true then t₂ else t₃ -> t₂
        | IfElse(True, &t_2, _) => t_2,

        //
        // ------------------------------ (E-IfFalse)
        // if false then t₂ else t₃ -> t₂
        | IfElse(False, _, &t_3) => t_3,

        //                    t₁ -> t₁'
        // ----------------------------------------------- (E-If)
        // if t₁ then t₂ else t₃ -> if t₁' then t₂ else t₃
        | IfElse(t_1, t_2, t_3) => IfElse(arena.alloc(t_1.step(arena)?), t_2, t_3),

        //      t₁ -> t₁'
        // ------------------- (E-Succ)
        // succ t₁ -> succ t₁'
        | Succ(t_1) => Succ(arena.alloc(t_1.step(arena)?)),

        //
        // ----------- (E-PredZero)
        // pred 0 -> 0
        | Pred(Zero) => Zero,

        //
        // ---------------------- (E-PredSucc)
        // pred (succ nv₁) -> nv₁
        | Pred(&Succ(&nv_1)) if nv_1.is_numeric() => nv_1,

        //      t₁ -> t₁'
        // ------------------- (E-Pred)
        // pred t₁ -> pred t₁'
        | Pred(t_1) => Pred(arena.alloc(t_1.step(arena)?)),

        //
        // ---------------- (E-IsZeroZero)
        // iszero 0 -> true
        | IsZero(Zero) => True,

        //
        // -------------------------- (E-IsZeroSucc)
        // iszero (succ nv₁) -> false
        | IsZero(Succ(nv_1)) if nv_1.is_numeric() => False,

        //        t₁ -> t₁'
        // ----------------------- (E-IsZero)
        // iszero t₁ -> iszero t₁'
        | IsZero(t_1) => IsZero(arena.alloc(t_1.step(arena)?)),

        // Stuck.
        | _ => return None,
        };

        Some(next)
    }

    pub fn is_value(&self) -> bool {
        use term::T::*;
        match self {
        | True => true,
        | False => true,
        | t => t.is_numeric(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        use term::T::*;
        match self {
        | Zero => true,
        | Succ(t_1) => t_1.is_numeric(),
        | _ => false,
        }
    }
}
