(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.25: [unless] breaks under applicative order.  Under the
    lazy experiment the [unless] factorial computes 120 for [n >= 1]
    because the recursive operand stays a thunk until the [if] inside
    [unless] chooses, so the descent stops at the base case.  Under the
    applicative-order evaluator the operand is evaluated before
    [unless] is called, so the descent never reaches the base case.
    The strict side runs under a fuel-bounded variant of the direct
    evaluator, so the divergence is observed as a typed error naming
    the budget instead of a hung run. *)

(** [ex_4_25 ()] answers, in order, the lazy factorial of 5, the lazy
    armed call [unless (1 = 1) (1 / 0) 42], and the same two programs
    under the fuel-bounded applicative-order evaluator.  Lazy
    transcripts end with the experiment's thunk counts. *)
val ex_4_25 : unit -> string list
