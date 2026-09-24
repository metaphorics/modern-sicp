(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.53 *)

type symbol = Sym of string

val memq : 'a -> 'a list -> 'a list option

(** [ex_2_53 ()] is the seven printed values the statement's
    expressions produce, in order. *)
val ex_2_53 : unit -> string list
