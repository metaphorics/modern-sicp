(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.36: the compiler's right-to-left operand order, and its
    measured cost. *)

val compiled_statements
  :  Sicp_ch5.Sec_5_5.config
  -> string
  -> (string list, Sicp_ch5.Sec_5_5.error) result

val run
  :  Sicp_ch5.Sec_5_5.config
  -> string
  -> (string list * int, Sicp_ch5.Sec_5_5.error) result

val ex_5_36 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
