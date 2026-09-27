(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fast-expt in SICP section 1.2
   exercise 1.16 *)

(** Exercise 1.16: an iterative, logarithmic-step exponentiation
    process built on the invariant [a * b^n]. The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

(** [fast_expt_iter a b n] keeps [a * b^n] invariant across every
    state; when [n] reaches 0 the answer is [a]. *)
val fast_expt_iter : int -> int -> int -> int

(** [ex_1_16 b n] is [b ** n] by [fast_expt_iter], starting the
    invariant from [a] = 1. *)
val ex_1_16 : int -> int -> int
