(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.32 *)

(** Exercise 3.32: segment order, FIFO against LIFO, on the and-gate
    whose inputs change together. *)

open Sicp_ch3.Sec_3_3

(** A minimal agenda whose segments are stacks. *)
type lifo_sim =
  { mutable now : int
  ; mutable segs : (int * (unit -> unit) list ref) list
  }

val make_lifo : unit -> lifo_sim
val lifo_add : lifo_sim -> int -> (unit -> unit) -> unit
val lifo_propagate : lifo_sim -> unit
val lifo_and_gate : lifo_sim -> Circuit.wire -> Circuit.wire -> Circuit.wire -> unit

(** [ex_3_32 ()] is [(the and-gate output under the section's FIFO
    agenda, the output under the LIFO agenda)] for inputs changing from
    (0, 1) to (1, 0) in one segment: [(0, 1)], the LIFO answer being
    the stale one. *)
val ex_3_32 : unit -> int * int
