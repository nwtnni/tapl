use crate::term;

use typed_arena::Arena;

impl<'a> term::T<'a> {
    /// The multi-step evaluation relation.
    pub fn eval(&self, arena: &'a Arena<term::T<'a>>) -> term::T<'a> {
        use term::T::*;
        match self {
        //
        // ----- (B-Value)
        // v ↓ v
        | &v if v.is_value() => v,

        //    t₁ ↓ true    t₂ ↓ v₂
        // -------------------------- (B-IfTrue)
        // if t₁ then t₂ else t₃ ↓ v₂
        | IfElse(t_1, t_2, _) if t_1.eval(arena) == True => t_2.eval(arena),

        //   t₁ ↓ false    t₃ ↓ v₃
        // -------------------------- (B-IfFalse)
        // if t₁ then t₂ else t₃ ↓ v₃
        | IfElse(t_1, _, t_3) if t_1.eval(arena) == False => t_3.eval(arena),

        //      t₁ ↓ nv₁
        // ------------------ (B-Succ)
        // succ t₁ ↓ succ nv₁
        | Succ(t_1) if t_1.eval(arena).is_numeric() => Succ(arena.alloc(t_1.eval(arena))),

        //   t₁ ↓ 0
        // ----------- (B-PredZero)
        // pred t₁ ↓ 0
        | Pred(t_1) if t_1.eval(arena) == Zero => Zero,

        // t₁ ↓ succ nv₁
        // ------------- (B-PredSucc)
        // pred t₁ ↓ nv₁
        | Pred(t_1) => {
            match t_1.eval(arena) {
            | Succ(&nv_1) if nv_1.is_numeric() => nv_1,
            | _ => Pred(t_1),
            }
        }

        //      t₁ ↓ 0
        // ---------------- (B-IsZeroZero)
        // iszero t₁ ↓ true
        | IsZero(t_1) if t_1.eval(arena) == Zero => True,

        //   t₁ ↓ succ nv₁
        // ----------------- (B-IsZeroSucc)
        // iszero t₁ ↓ false
        | IsZero(t_1) => {
            match t_1.eval(arena) {
            | Succ(&nv_1) if nv_1.is_numeric() => False,
            | _ => IsZero(t_1),
            }
        }

        // Stuck.
        | &t => t,
        }
    }

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
