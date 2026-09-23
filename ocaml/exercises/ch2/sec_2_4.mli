(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cons in SICP section 2.1
   exercise 2.4 *)

(** Exercise 2.4: pairs as a function that applies its selector to its
    two elements, an alternative to @ref{2.1.3}'s numeric-dispatch
    [cons]. The stubs raise [Sicp_common.Pending.Pending_solution]
    until they are solved. *)

val cons : 'a -> 'a -> ('a -> 'a -> 'a) -> 'a
val car : (('a -> 'a -> 'a) -> 'a) -> 'a

(** [cdr z] is [z]'s second element; the exercise's own ask. *)
val cdr : (('a -> 'a -> 'a) -> 'a) -> 'a

(** [ex_2_04 x y] is [true] exactly when [car (cons x y) = x] and
    [cdr (cons x y) = y]. *)
val ex_2_04 : int -> int -> bool
