(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.29 *)

(** Exercise 3.29: an or-gate composed of and-gates and inverters. *)

open Sicp_ch3.Sec_3_3

(** [or_gate sim a b out] is de Morgan's or: two inverters feed an
    and-gate whose output is inverted again. Its delay is twice the
    inverter delay plus one and-gate delay. *)
val or_gate : Circuit.sim -> Circuit.wire -> Circuit.wire -> Circuit.wire -> unit

(** [ex_3_29 ()] is [(the four output signals, the measured delay for
    inputs 1,0, the measured delay for 0,1, the expected delay 2 * 2 +
    3)]. *)
val ex_3_29 : unit -> int list * int * int * int
