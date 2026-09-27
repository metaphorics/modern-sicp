(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fib in SICP section 1.2 exercise 1.13 *)

(** Exercise 1.13: the closed form of the Fibonacci numbers. The stubs
    raise [Sicp_common.Pending.Pending_solution] until they are
    solved. *)

(** [closed_form_fib n] rounds @math{\varphi^n / \sqrt5} to the nearest
    integer. *)
val closed_form_fib : int -> int

(** [fib_direct n] is the tree-recursive [fib] of @ref{1.2.2}, used only
    to check [closed_form_fib] against, never for large [n]. *)
val fib_direct : int -> int

(** [ex_1_13 n] is [closed_form_fib n]. *)
val ex_1_13 : int -> int
