(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.38: open-coded primitives and the measured comparison. *)

val factorial_source : string
val open_code : Sicp_ch5.Sec_5_5.config

val compile_count
  :  Sicp_ch5.Sec_5_5.config
  -> string
  -> (int, Sicp_ch5.Sec_5_5.error) result

val run
  :  Sicp_ch5.Sec_5_5.config
  -> string
  -> string
  -> (string list, Sicp_ch5.Sec_5_5.error) result

val ex_5_38 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
