(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.15 *)

(** Exercise 3.15: in [z1] the same pair [x] stands in both the car and
    the cdr, so [set_to_wow] changes what both slots show; in [z2] the
    two (a b) lists are distinct pairs and only the car's copy changes.
    [ex_3_15] records the printed structures and the physical-equality
    facts behind them. *)

open Sicp_ch3.Sec_3_3.Mpairs

(* The book's z1 = (cons x x): one pair, x, in both slots. The wrapped
   value is built once and reused, not rebuilt twice, so the two slots
   hold the very same [mobj] and [==] can see it. *)
let z1_of x =
  let shared = Pair (pair_of x) in
  { car = shared; cdr = shared }
;;

let ex_3_15 () =
  let x = from_symbols [ "a"; "b" ] in
  let z1 = Pair (z1_of x) in
  let z2 = mcons (from_symbols [ "a"; "b" ]) (from_symbols [ "a"; "b" ]) in
  let z1_slots_shared = car z1 == cdr z1 in
  let z2_slots_shared = car z2 == cdr z2 in
  ignore (set_to_wow z1);
  ignore (set_to_wow z2);
  let z1_shown = show z1 in
  let z2_shown = show z2 in
  z1_slots_shared, z2_slots_shared, z1_shown, z2_shown
;;
