(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.27 *)

(** Exercise 3.27: memoized Fibonacci over a hash table, with the
    environment diagram traded for a counted trace. *)

(** [memoize f] is [(the memoized f, the hit counter, the compute
    counter)]; the table lives in the closure, one table per call. *)

(** [fib] is the exponential procedure of @ref{1.2.2}; [fib_calls]
    accumulates the number of calls since the last reset. *)

(** [memo_fib_steps n] is [(the nth Fibonacci number, the table hits,
    the table computes)] for the self-referential memo-fib of the
    statement. *)

(** [ex_3_27 ()] is [(memo-fib 25, plain fib 25, the computes and hits
    of that memo run, the plain call count for fib 25, whether the
    memoized step count is the smaller one)]. *)
let memoize = raise Sicp_common.Pending.Pending_solution

let fib = raise Sicp_common.Pending.Pending_solution
let fib_calls = raise Sicp_common.Pending.Pending_solution
let memo_fib_steps = raise Sicp_common.Pending.Pending_solution
let ex_3_27 = raise Sicp_common.Pending.Pending_solution
