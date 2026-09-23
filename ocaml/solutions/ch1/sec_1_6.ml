(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs new-if and sqrt-iter in SICP
   section 1.1 exercise 1.6 *)

(** Exercise 1.6: [new_if] is an ordinary function, so both branches
    are evaluated before the call. Alyssa's [sqrt_iter] therefore
    evaluates its own recursive call on every iteration, including
    the last, and never returns. *)

let ex_1_06_new_if predicate consequent alternative =
  match predicate with
  | true -> consequent
  | false -> alternative
;;

let square x = x *. x
let average x y = (x +. y) /. 2.0
let improve guess x = average guess (x /. guess)
let good_enough guess x = Float.abs (square guess -. x) < 0.001

let rec ex_1_06_sqrt_iter guess x =
  ex_1_06_new_if (good_enough guess x) guess (ex_1_06_sqrt_iter (improve guess x) x)
;;
