(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.28 *)

(** Exercise 3.28: the or-gate as a primitive function box. *)

open Sicp_ch3.Sec_3_3

(** [logical_or] is the logical or of two signals. *)
val logical_or : int -> int -> int

(** [or_gate sim a1 a2 output] attaches an or-gate to the wires, with
    the section's delay contract: either input changing to 1 forces the
    output to 1 one or-gate-delay later. *)
val or_gate : Circuit.sim -> Circuit.wire -> Circuit.wire -> Circuit.wire -> unit

(** [ex_3_28 ()] is [(the outputs for the four input combinations, the
    four answers of logical_or)], the first four taken one full
    propagation after the inputs settle. *)
val ex_3_28 : unit -> (int * int * int * int) * int * int * int * int
