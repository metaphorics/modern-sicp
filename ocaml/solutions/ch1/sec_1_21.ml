(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smallest-divisor in SICP section 1.2
   exercise 1.21 *)

(** Exercise 1.21: the section's [smallest_divisor], applied directly.
    199 and 1999 are their own smallest divisor, so they are prime;
    19999 is [7 * 2857]. *)

let square x = x * x

let rec find_divisor n test_divisor =
  if square test_divisor > n
  then n
  else if n mod test_divisor = 0
  then test_divisor
  else find_divisor n (test_divisor + 1)
;;

let smallest_divisor n = find_divisor n 2
let ex_1_21 () = smallest_divisor 199, smallest_divisor 1999, smallest_divisor 19999
