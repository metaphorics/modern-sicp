(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.42: lexical addressing in the code generators. *)

val nested_example : string
val compiled_statements : string -> (string list, Sicp_ch5.Sec_5_5.error) result
val lexical_accesses : string list -> string list
val ex_5_42 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
