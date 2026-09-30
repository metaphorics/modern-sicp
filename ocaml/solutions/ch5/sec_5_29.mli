(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.29: the stack of the tree-recursive Fibonacci, with the
    depth formula and the pushes formula [S(n) = a * Fib(n+1) + b]. *)

(** [fib_source n] is the 1.2.2 tree-recursive Fibonacci, ending with
    the call at [n]. *)
val fib_source : int -> string

(** [fib n] is the [n]th Fibonacci number, [fib 0 = 0]. *)
val fib : int -> int

(** [recurrence_constants pushes] is [S(n) - S(n-1) - S(n-2)] for every
    [n] of the [(n, S(n))] samples whose two predecessors were measured. *)
val recurrence_constants : (int * int) list -> int list

(** [ex_5_29 ()] measures fib for n = 2 to 9 and answers the eight
    table lines, then three checks computed from the data: the maximum
    depth grows by one constant step per n, the pushes obey
    [S(n) = S(n-1) + S(n-2) + k] with one [k], and the closed form
    [S(n) = a * Fib(n+1) + b] fitted from two points holds on all. *)
val ex_5_29 : unit -> (string list, Sicp_common.Eval_error.t) result
