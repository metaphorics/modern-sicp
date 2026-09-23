(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-rat in SICP section 2.1
   exercise 2.1 *)

(** Exercise 2.1: a sign-normalizing [make_rat] that also refuses a
    zero denominator, extending @ref{2.1.2}'s [Rational.make]. The
    stub raises [Sicp_common.Pending.Pending_solution] until it is
    solved. *)

type rational_error = Zero_denominator of int (** the rejected numerator *)

(** [ex_2_01 n d] is a rational number with numerator and denominator
    reduced to lowest terms: positive denominator if the value is
    positive, negative numerator only if the value is negative, or
    [Error (Zero_denominator n)] when [d = 0]. *)
val ex_2_01 : int -> int -> (int * int, rational_error) result
