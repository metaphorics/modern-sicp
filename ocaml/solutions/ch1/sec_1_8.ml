(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme cube-root program of SICP section 1.1
   exercise 1.8 *)

(** Exercise 1.8: the Newton iteration for cube roots. The zero guard
    keeps the relative end test from iterating forever on
    [cube_root 0.0]. *)

let improve_cube guess x = ((x /. (guess *. guess)) +. (2.0 *. guess)) /. 3.0
let good_enough_cube previous guess = Float.abs (guess -. previous) < 1e-6 *. guess

let rec cube_iter previous guess x =
  if good_enough_cube previous guess
  then guess
  else cube_iter guess (improve_cube guess x) x
;;

let ex_1_08_cube_root x = if x = 0.0 then 0.0 else cube_iter x 1.0 x
