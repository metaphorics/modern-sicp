(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.31 *)

(** Exercise 3.31: the initial run of a newly attached action. *)

open Sicp_ch3.Sec_3_3

(** [make_wire_without_initial_run ()] is the wire of the exercise's
    variant, whose [add_action] appends without running the procedure. *)
val make_wire_without_initial_run : unit -> Circuit.wire

(** [ex_3_31 ()] is [(the inverter's output when actions run at attach,
    the output when they do not)], one propagation each from a wire
    settled at 0. *)
val ex_3_31 : unit -> int * int
