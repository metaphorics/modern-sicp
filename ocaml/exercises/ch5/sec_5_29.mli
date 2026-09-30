(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 5.29: the stack of the tree-recursive Fibonacci, with the depth formula and the pushes formula S(n) = a*Fib(n+1) + b. *)

(** Exercise 5.29: fib measured for n = 2 to 9, with the three formula checks. *)
val ex_5_29 : unit -> (string list, Sicp_common.Eval_error.t) result
