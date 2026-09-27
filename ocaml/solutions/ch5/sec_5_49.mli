(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.49: the read-compile-execute-print loop. *)

val loop : string list -> (string list list, Sicp_ch5.Sec_5_5.error) result
val ex_5_49 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
