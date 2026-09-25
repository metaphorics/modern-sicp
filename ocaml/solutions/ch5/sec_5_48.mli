(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.48: the compile-and-run primitive. *)

val datum_of_value : Sicp_common.Value.t -> Sicp_common.Ast.datum

val compile_and_run_op
  :  Sicp_ch5.Sec_5_5.state
  -> string list ref
  -> string * Sicp_ch5.Sec_5_4.op

val ex_5_48 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
