(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.50: the metacircular evaluator compiled, and the three
    interpretation levels timed. *)

val metacircular_session : unit -> (string list * int, Sicp_ch5.Sec_5_5.error) result
val level_0_steps : int -> (int, Sicp_ch5.Sec_5_5.error) result
val level_1_steps : int -> (int, Sicp_ch5.Sec_5_4.error) result
val level_2_steps : int -> int
val ex_5_50 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
