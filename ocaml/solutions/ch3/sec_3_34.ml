(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.34 *)

(** Exercise 3.34: Louis Reasoner's squarer. The multiplier cannot run
    backwards here: with [b] set and [a] unknown, no branch of
    [process_new_value] has two known quantities, because both of the
    multiplier's inputs are the one unknown connector. [ex_3_34]
    demonstrates it: setting b alone computes nothing, and the device
    works only in the forward direction. *)

open Sicp_ch3.Sec_3_3
open Constraints
module K = Constraints

let squarer a b = ignore (K.multiplier a a b)

let ex_3_34 () =
  let a = K.make_connector () in
  let b = K.make_connector () in
  squarer a b;
  b.set_value 25 K.User;
  let a_after_set_b = a.has_value () in
  (* the forward direction still works: naming a fills in b *)
  a.set_value 5 K.User;
  let a_now = a.get_value () in
  let b_now = b.get_value () in
  a_after_set_b, a_now, b_now
;;
