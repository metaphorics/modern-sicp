(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.37 *)

(** Exercise 3.37: the expression-style constraint combinators. Each
    combinator makes a fresh result connector, wires the primitive
    constraint into its arguments, and returns the connector -- so a
    converter is one expression. The integer connectors of this edition
    reassociate the book's ((9/5) * C) + 32 into the equivalent
    ((9 * C) / 5) + 32, because the factor 9/5 has no integer value;
    the network still computes in both directions. *)

open Sicp_ch3.Sec_3_3
module K = Constraints

let c_add x y =
  let z = K.make_connector () in
  ignore (K.adder x y z);
  z
;;

let c_sub x y =
  (* x - y = z, that is, z + y = x *)
  let z = K.make_connector () in
  ignore (K.adder z y x);
  z
;;

let c_mul x y =
  let z = K.make_connector () in
  ignore (K.multiplier x y z);
  z
;;

let c_div x y =
  (* x / y = z, that is, y * z = x *)
  let z = K.make_connector () in
  ignore (K.multiplier y z x);
  z
;;

let cv value =
  let z = K.make_connector () in
  ignore (K.constant value z);
  z
;;

let celsius_fahrenheit_converter x = c_add (c_div (c_mul (cv 9) x) (cv 5)) (cv 32)

let ex_3_37 () =
  let c = K.make_connector () in
  let f = celsius_fahrenheit_converter c in
  c.set_value 25 K.User;
  let f_at_25 = f.get_value () in
  c.forget_value K.User;
  f.set_value 212 K.User;
  let c_at_212 = c.get_value () in
  (* a second network, built by c- alone: x - y = z both ways *)
  let x = K.make_connector () in
  let y = K.make_connector () in
  let z = c_sub x y in
  x.set_value 10 K.User;
  y.set_value 4 K.User;
  let sub = z.get_value () in
  x.forget_value K.User;
  z.set_value 3 K.User;
  let solved_x = x.get_value () in
  f_at_25, c_at_212, sub, solved_x
;;
