(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.33 *)

(** Exercise 3.33: the averager. The sum [s] of [a] and [b] feeds a
    multiplier whose other factor is the constant 2, and [c] is the
    product: c * 2 = a + b. Every direction works: (a, b) set the
    average, and (c, a) set the missing addend. *)

open Sicp_ch3.Sec_3_3
open Constraints
module K = Constraints

let averager a b c =
  let s = K.make_connector () in
  let half = K.make_connector () in
  ignore (K.adder a b s);
  ignore (K.multiplier c half s);
  ignore (K.constant 2 half)
;;

let value_of c = c.get_value ()

let ex_3_33 () =
  let a = K.make_connector () in
  let b = K.make_connector () in
  let c = K.make_connector () in
  averager a b c;
  a.set_value 6 K.User;
  b.set_value 14 K.User;
  let average = value_of c in
  (* forget, then drive the network from c and a *)
  a.forget_value K.User;
  b.forget_value K.User;
  c.set_value 12 K.User;
  a.set_value 4 K.User;
  let b_now = value_of b in
  let a_now = value_of a in
  average, b_now, a_now, c.get_value ()
;;
