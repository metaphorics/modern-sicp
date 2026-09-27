(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.37 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.37: expression-style constraint combinators. *)

(** [c_add], [c_sub], [c_mul], and [c_div] each return a fresh
    connector constrained to the sum, difference, product, or quotient
    of the arguments; [cv value] is a constant-valued connector. *)

(** [celsius_fahrenheit_converter x] is the one-expression network:
    F = ((9 * C) / 5) + 32 with integer connectors. *)

(** [ex_3_37 ()] is [(the Fahrenheit when C = 25, the Celsius when F =
    212, the difference 10 - 4 through c-sub, the x solved when the
    difference is set to 3 with y = 4)]. *)
let c_add _a _b = raise Sicp_common.Pending.Pending_solution

let c_sub _a _b = raise Sicp_common.Pending.Pending_solution
let c_mul _a _b = raise Sicp_common.Pending.Pending_solution
let c_div _a _b = raise Sicp_common.Pending.Pending_solution
let cv _n = raise Sicp_common.Pending.Pending_solution
let celsius_fahrenheit_converter _x = raise Sicp_common.Pending.Pending_solution
let ex_3_37 () = raise Sicp_common.Pending.Pending_solution
