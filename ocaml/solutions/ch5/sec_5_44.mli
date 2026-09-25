(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.44: open coding respects shadowing. *)

val shadowed_source : string
val open_code : Sicp_ch5.Sec_5_5.config
val open_code_ops : string -> (int, Sicp_ch5.Sec_5_5.error) result
val ex_5_44 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
