(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.35 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.35: the squarer as a primitive constraint. *)

(** [isqrt v] is the integer square root of [v]. *)

(** [squarer a b] is a primitive constraint with a.in * a.in = b:
    setting b computes a as the integer square root, setting a computes
    b, and a negative b is a programming error. *)

(** [ex_3_35 ()] is [(the a computed when b is set to 49, the b
    computed when, after a forget, a is set to 6)]. *)

(** [negative_raises ()] is whether setting b to a negative value
    raises [Invalid_argument]. *)
let isqrt _v = raise Sicp_common.Pending.Pending_solution

let squarer _a _b = raise Sicp_common.Pending.Pending_solution
let ex_3_35 () = raise Sicp_common.Pending.Pending_solution
let negative_raises () = raise Sicp_common.Pending.Pending_solution
