(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.47: compiled calls to interpreted procedures. *)

val compound_calls : Sicp_ch5.Sec_5_5.config
val f_source : string
val driver_source : string
val ex_5_47 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
