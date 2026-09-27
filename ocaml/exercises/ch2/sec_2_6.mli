(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program zero in SICP section 2.1
   exercise 2.6 *)

(** Exercise 2.6: Church numerals, natural numbers represented as
    "apply [f], [n] times." The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [zero] applies its function argument zero times: the identity on
    [x], given by the book. *)
val zero : ('a -> 'a) -> 'a -> 'a

(** [add_1 n] applies [f] one more time than [n] does, given by the
    book. *)
val add_1 : (('a -> 'a) -> 'a -> 'a) -> ('a -> 'a) -> 'a -> 'a

(** [one] and [two], defined directly rather than through [add_1]. *)
val one : ('a -> 'a) -> 'a -> 'a

val two : ('a -> 'a) -> 'a -> 'a

(** [church_add a b] applies [f] as many times as [a] and [b]
    together, defined directly rather than through repeated [add_1]. *)
val church_add
  :  (('a -> 'a) -> 'a -> 'a)
  -> (('a -> 'a) -> 'a -> 'a)
  -> ('a -> 'a)
  -> 'a
  -> 'a

(** [church_to_int n] is the ordinary [int] a Church numeral [n]
    counts, this edition's way to observe a numeral's value. *)
val church_to_int : ((int -> int) -> int -> int) -> int

(** [ex_2_06 ()] is [church_to_int (church_add one two)]. *)
val ex_2_06 : unit -> int
