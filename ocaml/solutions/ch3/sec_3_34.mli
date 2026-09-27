(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.34 *)

(** Exercise 3.34: the flawed one-constraint squarer. *)

open Sicp_ch3.Sec_3_3

(** [squarer a b] is Louis's device: a bare multiplier whose two input
    terminals are the same connector. *)
val squarer : Constraints.connector -> Constraints.connector -> unit

(** [ex_3_34 ()] is [(whether a has a value after b is set to 25, the
    value of a after a is named, the value of b then)], demonstrating
    that the device computes squares but never square roots. *)
val ex_3_34 : unit -> bool * int * int
