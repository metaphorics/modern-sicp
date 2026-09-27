(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fib-iter in SICP section 1.2
   exercise 1.19 *)

(** Reference solution of exercise 1.19; the derivation of [p'] and
    [q'] is in [ex_1_19.md]. *)

(** [fib_iter_log a b p q count] applies [T_pq] to [(a, b)] [count]
    times, by successive squaring of the transform. *)
val fib_iter_log : int -> int -> int -> int -> int -> int

(** [ex_1_19 n] is [Fib(n)]; exact through [n = 90], silently wrapped
    from [n = 91] on, since this edition carries the state in [int]. *)
val ex_1_19 : int -> int
