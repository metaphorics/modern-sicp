(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.29: the stack of the tree-recursive Fibonacci on the
    monitored evaluator, and the two formulas: the maximum depth in
    terms of n, and the total pushes as [a*Fib(n+1) + b], grown by the
    recurrence [S(n) = S(n-1) + S(n-2) + k]. *)

(** [fib_source] is the tree-recursive Fibonacci the exercise defines. *)
val fib_source : string

(** [measure_fib ns] runs [(fib n)] for each [n] on a fresh monitored
    machine and answers the counters. *)
val measure_fib : int list -> (Sec_5_26.stats list, Sicp_ch5.Sec_5_4.error) result

(** [fib n] is the n-th Fibonacci number, the oracle for the formula
    check. *)
val fib : int -> int

(** [ex_5_29 ()] measures fib for n = 2 to 9 and answers the table
    plus the three formula checks the rationale describes. *)
val ex_5_29 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
