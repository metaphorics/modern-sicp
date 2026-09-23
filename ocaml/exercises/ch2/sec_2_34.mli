(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.34:
   Horner's rule *)

(** [ex_2_34 x coefficient_sequence] evaluates the polynomial with the
    given coefficients, [a_0] first, at [x]. *)
val ex_2_34 : int -> int list -> int
