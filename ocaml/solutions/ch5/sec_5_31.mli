(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.31: which of the evaluator's saves survive the compiler's
    [preserving] for four combinations. *)

(** The four combinations, in the exercise's order. *)
val combinations : string list

val ex_5_31 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
