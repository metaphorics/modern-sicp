(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.15 *)

(** Exercise 3.15: what [set_to_wow] does to the shared structure [z1]
    and the unshared [z2]. *)

open Sicp_ch3.Sec_3_3.Mpairs

(** [z1_of x] is the pair the book forms by [(cons x x)]: [x]'s own pair
    stands in both slots. *)
val z1_of : mobj -> mpair

(** [ex_3_15 ()] is [(whether z1's car and cdr slots hold the same pair,
    whether z2's do, the printed z1 after set_to_wow, the printed z2
    after set_to_wow)]. *)
val ex_3_15 : unit -> bool * bool * string * string
