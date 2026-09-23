(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program A in SICP section 1.2 exercise 1.10 *)

(** Reference solution of exercise 1.10. *)

(** [ackermann x y] is the book's Ackermann function. *)
val ackermann : int -> int -> int

(** [f n] is [ackermann 0 n], equal to [2 * n]. *)
val f : int -> int

(** [g n] is [ackermann 1 n], equal to [2 ** n] for [n >= 1]. *)
val g : int -> int

(** [h n] is [ackermann 2 n], a tower of [n] twos for [n >= 1]. *)
val h : int -> int

(** [k n] is [5 * n * n], the book's worked example. *)
val k : int -> int

(** [ex_1_10 ()] is [(ackermann 1 10, ackermann 2 4, ackermann 3 3)],
    equal to [(1024, 65536, 65536)]. *)
val ex_1_10 : unit -> int * int * int
