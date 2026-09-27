(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.51: the explicit-control evaluator translated to C. *)

val c_translator : string
val runtime_c : string
val compile_and_run : string -> (string, Sicp_ch5.Sec_5_5.error) result
val ex_5_51 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
