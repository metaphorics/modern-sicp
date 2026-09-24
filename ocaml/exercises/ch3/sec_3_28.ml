(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.28 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.28: the or-gate as a primitive function box. *)

(** [logical_or] is the logical or of two signals. *)

(** [or_gate sim a1 a2 output] attaches an or-gate to the wires, with
    the section's delay contract: either input changing to 1 forces the
    output to 1 one or-gate-delay later. *)

(** [ex_3_28 ()] is [(the outputs for the four input combinations, the
    four answers of logical_or)], the first four taken one full
    propagation after the inputs settle. *)
let logical_or _a _b = raise Sicp_common.Pending.Pending_solution

let or_gate _sim _a _b _out = raise Sicp_common.Pending.Pending_solution
let ex_3_28 () = raise Sicp_common.Pending.Pending_solution
