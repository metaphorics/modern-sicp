(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sqrt in SICP section 1.1 exercise
   1.7 *)

(** Exercise 1.7: the section's program, then the improved end test
    that watches the change from the previous guess. The absolute
    tolerance of the original fails for very small radicands and
    cannot be met at all for very large ones; the relative change
    test works for both. *)

let square x = x *. x
let average x y = (x +. y) /. 2.0
let improve guess x = average guess (x /. guess)
let good_enough guess x = Float.abs (square guess -. x) < 0.001

let rec sqrt_iter guess x =
  if good_enough guess x then guess else sqrt_iter (improve guess x) x
;;

let ex_1_07_sqrt x = sqrt_iter 1.0 x
let good_enough_change previous guess = Float.abs (guess -. previous) < 1e-3 *. guess

let rec sqrt_iter_change previous guess x =
  if good_enough_change previous guess
  then guess
  else sqrt_iter_change guess (improve guess x) x
;;

let ex_1_07_sqrt_improved x = if x = 0.0 then 0.0 else sqrt_iter_change x 1.0 x
