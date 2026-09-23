(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cons in SICP section 2.1
   exercise 2.4 *)

(** Exercise 2.4: pairs as a function applying its selector to its two
    elements. *)

val cons : 'a -> 'a -> ('a -> 'a -> 'a) -> 'a
val car : (('a -> 'a -> 'a) -> 'a) -> 'a
val cdr : (('a -> 'a -> 'a) -> 'a) -> 'a
val ex_2_04 : int -> int -> bool
