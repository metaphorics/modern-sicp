(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.29 *)

(** Exercise 3.29: an or-gate composed of and-gates and inverters. *)

(** [or_gate sim a b out] is de Morgan's or: two inverters feed an
    and-gate whose output is inverted again. Its delay is twice the
    inverter delay plus one and-gate delay. *)

(** [ex_3_29 ()] is [(the four output signals, the measured delay for
    inputs 1,0, the measured delay for 0,1, the expected delay 2 * 2 +
    3)]. *)
let or_gate = raise Sicp_common.Pending.Pending_solution

let ex_3_29 = raise Sicp_common.Pending.Pending_solution
