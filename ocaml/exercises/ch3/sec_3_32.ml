(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.32 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.32: segment order, FIFO against LIFO, on the and-gate
    whose inputs change together. *)

(** A minimal agenda whose segments are stacks. *)
type lifo_sim =
  { mutable now : int
  ; mutable segs : (int * (unit -> unit) list ref) list
  }

(** [ex_3_32 ()] is [(the and-gate output under the section's FIFO
    agenda, the output under the LIFO agenda)] for inputs changing from
    (0, 1) to (1, 0) in one segment: [(0, 1)], the LIFO answer being
    the stale one. *)
let make_lifo () = raise Sicp_common.Pending.Pending_solution

let lifo_add _s _time _action = raise Sicp_common.Pending.Pending_solution
let lifo_propagate _s = raise Sicp_common.Pending.Pending_solution
let lifo_and_gate _s _a _b _out = raise Sicp_common.Pending.Pending_solution
let ex_3_32 () = raise Sicp_common.Pending.Pending_solution
