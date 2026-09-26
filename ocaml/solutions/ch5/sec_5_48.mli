(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.48: the compile-and-run primitive. *)

(** [compile_and_run_primitive state blocks] is the [compile-and-run]
    primitive bound in the machine's global environment: it compiles
    its quoted argument with [state] and records the controller block
    in [blocks] for the next assembly. *)
val compile_and_run_primitive
  :  Sicp_ch5.Sec_5_5.state
  -> string list ref
  -> string * Sicp_common.Value.primitive

val ex_5_48 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
