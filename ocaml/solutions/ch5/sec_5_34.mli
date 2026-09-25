(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.34: the iterative factorial's compilation, and its
    constant stack depth. *)

val factorial_iter_source : string
val tail_call_statements : string list
val depth_at : int -> (int, Sicp_ch5.Sec_5_5.error) result
val ex_5_34 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
