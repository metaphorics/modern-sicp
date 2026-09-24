(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.27 *)

(** Exercise 3.27: memoized Fibonacci over a hash table, with the
    environment diagram traded for a counted trace. *)

(** [memoize f] is [(the memoized f, the hit counter, the compute
    counter)]; the table lives in the closure, one table per call. *)
val memoize : (int -> int) -> (int -> int) * int ref * int ref

(** [fib] is the exponential procedure of @ref{1.2.2}; [fib_calls]
    accumulates the number of calls since the last reset. *)
val fib : int -> int

val fib_calls : int ref

(** [memo_fib_steps n] is [(the nth Fibonacci number, the table hits,
    the table computes)] for the self-referential memo-fib of the
    statement. *)
val memo_fib_steps : int -> int * int * int

(** [ex_3_27 ()] is [(memo-fib 25, plain fib 25, the computes and hits
    of that memo run, the plain call count for fib 25, whether the
    memoized step count is the smaller one)]. *)
val ex_3_27 : unit -> int * int * int * int * int * bool
