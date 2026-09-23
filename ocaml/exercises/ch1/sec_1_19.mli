(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fib-iter in SICP section 1.2
   exercise 1.19 *)

(** Exercise 1.19: the Fibonacci numbers in a logarithmic number of
    steps, via successive squaring of the state transformation. Carried
    in this edition's 63-bit [int], correct only through @code{fib 90}.
    The stub raises [Sicp_common.Pending.Pending_solution] until it is
    solved. *)

(** [fib_iter_log a b p q count] applies [T_pq] to [(a, b)] [count]
    times by successive squaring. *)
val fib_iter_log : int -> int -> int -> int -> int -> int

(** [ex_1_19 n] is [Fib(n)] via [fib_iter_log], starting from
    [(1, 0, 0, 1, n)]. *)
val ex_1_19 : int -> int
