(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.10: a new surface syntax for the machine language,
    isolated in its own reader. *)

(** [read_syntax text] parses a controller written in the new syntax --
    bare register names, operations in head position, bare branch and
    goto targets -- into the same typed program the old syntax
    produces. *)
val read_syntax : string -> (Sicp_ch5.Sec_5_2.program, Sicp_ch5.Sec_5_2.error) result

(** [ex_5_10 ()] runs the GCD machine in both syntaxes and shows one
    compiled instruction of each: the same typed instruction. *)
val ex_5_10 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
