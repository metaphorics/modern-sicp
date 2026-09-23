(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs new-if and sqrt-iter in SICP
   section 1.1 exercise 1.6 *)

(** Exercise 1.6: Alyssa's [new_if] as an ordinary function, and her
    square-root driver rewritten with it. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [ex_1_06_new_if predicate consequent alternative] is [consequent]
    when [predicate] holds and [alternative] otherwise, but as an
    ordinary function it evaluates all three arguments first. *)
val ex_1_06_new_if : bool -> 'a -> 'a -> 'a

(** [ex_1_06_sqrt_iter guess x] recurses through [ex_1_06_new_if] and
    therefore never terminates; it exists to be reasoned about, not
    run. *)
val ex_1_06_sqrt_iter : float -> float -> float
