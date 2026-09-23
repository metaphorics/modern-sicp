(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.26:
   predicting the three list combinations *)

(** The results of [x @ y] and of [[x; y]] for the statement's [x] and
    [y]; the third expression, [x :: y], does not type-check, which the
    rationale explains. *)
val ex_2_26 : unit -> int list * int list list
