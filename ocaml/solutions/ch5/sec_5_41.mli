(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.41: [find-variable] over the fragment's compile-time
    environment. *)

val render : (int * int) option -> string
val cenv_of_example : string list list
val ex_5_41 : unit -> (string list, 'a) result
