(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program integral in SICP section 1.3
   exercise 1.29 *)

(** Exercise 1.29: Simpson's Rule sums [f] at [n + 1] equally spaced
    points with alternating weights 4, 2, 4, 2, ..., 4, capped by 1 at
    both ends, then scales by [h /. 3.]. *)

let simpson f a b n =
  let h = (b -. a) /. float_of_int n in
  let y k = f (a +. (float_of_int k *. h)) in
  let rec weighted_sum k =
    if k >= n
    then y n
    else (
      let coefficient = if k mod 2 = 1 then 4.0 else 2.0 in
      (coefficient *. y k) +. weighted_sum (k + 1))
  in
  (y 0 +. weighted_sum 1) *. h /. 3.0
;;

let cube x = x *. x *. x
let ex_1_29 () = simpson cube 0.0 1.0 100, simpson cube 0.0 1.0 1000
