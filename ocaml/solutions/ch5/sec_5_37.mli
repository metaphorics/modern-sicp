(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.37: the compiler with [preserving] disabled, and the
    stack operations it wastes. *)

val factorial_source : string
val no_preserving : Sicp_ch5.Sec_5_5.config

val compile_count
  :  Sicp_ch5.Sec_5_5.config
  -> string
  -> (int * int, Sicp_ch5.Sec_5_5.error) result

val run_monitored
  :  Sicp_ch5.Sec_5_5.config
  -> int
  -> (string list, Sicp_ch5.Sec_5_5.error) result

val ex_5_37 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
