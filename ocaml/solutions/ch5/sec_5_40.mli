(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.40: the compile-time environment threading, dumped. *)

val nested_example : string
val render_cenv : string list list -> string
val dump : string -> (string list, Sicp_ch5.Sec_5_5.error) result
val ex_5_40 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
