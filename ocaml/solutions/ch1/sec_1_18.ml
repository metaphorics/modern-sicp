(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program * in SICP section 1.2 exercise 1.18 *)

(** Exercise 1.18: [mult_iter] combines exercise 1.16's invariant
    technique with exercise 1.17's doubling: [total + a * b] never
    changes. On an even [b], doubling [a] and halving [b] preserves it;
    on an odd [b], folding one [a] into [total] and decrementing [b]
    preserves it. When [b] reaches 0 the invariant reads [total], the
    answer, and the self-call is the whole result throughout, so this
    is an iterative process. *)

let is_even n = n mod 2 = 0
let double x = x + x
let halve x = x / 2

let rec mult_iter total a b =
  if b = 0
  then total
  else if is_even b
  then mult_iter total (double a) (halve b)
  else mult_iter (total + a) a (b - 1)
;;

let ex_1_18 a b = mult_iter 0 a b
