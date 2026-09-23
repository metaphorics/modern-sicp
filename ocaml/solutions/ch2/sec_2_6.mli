(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program zero in SICP section 2.1
   exercise 2.6 *)

(** Exercise 2.6: Church numerals. *)

val zero : ('a -> 'a) -> 'a -> 'a
val add_1 : (('a -> 'a) -> 'a -> 'a) -> ('a -> 'a) -> 'a -> 'a
val one : ('a -> 'a) -> 'a -> 'a
val two : ('a -> 'a) -> 'a -> 'a

val church_add
  :  (('a -> 'a) -> 'a -> 'a)
  -> (('a -> 'a) -> 'a -> 'a)
  -> ('a -> 'a)
  -> 'a
  -> 'a

val church_to_int : ((int -> int) -> int -> int) -> int
val ex_2_06 : unit -> int
