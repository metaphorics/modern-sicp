(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.31: [lazy] and [lazy-memo] parameter declarations.  The
    subset's concrete syntax has no parameter annotations, so the
    declaration rides on the parameter's name, which the pinned checker
    admits unchanged: [b_lazy] is delayed without memoization,
    [d_lazy_memo] is delayed through a memoized thunk, and every other
    name is strict.  The evaluator is the applicative-order direct
    evaluator with its application clause binding each closure
    parameter per its declaration and its variable clause forcing a
    delayed parameter where it is read.  Ordinary definitions keep
    their strictness, so the extension is upward-compatible. *)

(** [ex_4_31 ()] answers, in order: a [_lazy] parameter never read
    saves an armed operand; the same procedure with a strict parameter
    dies on it; the four-way declaration evaluates the strict operands
    at the call, forces the [_lazy] operand once per read, and the
    [_lazy_memo] operand once in total, so [count] reads 5; and
    [twice (id 10)] under [_lazy] runs [id] twice where [_lazy_memo]
    runs it once, so [count] reads 2 and then 3. *)
val ex_4_31 : unit -> string list
