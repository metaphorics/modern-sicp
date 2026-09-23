(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smallest-divisor in SICP section 1.2
   exercise 1.23 *)

(** Exercise 1.23: [next_test_divisor] steps 2, 3, 5, 7, 9, @dots{}
    instead of 2, 3, 4, 5, 6, @dots{}, halving the number of test
    steps. [smallest_divisor_naive] is kept here, private, exactly as
    @ref{1.2.6} defined it, so this file can compare the two searches
    without depending on another exercise's file; the timing comparison
    is [ex_1_23.md]'s job. *)

let square x = x * x
let next_test_divisor d = if d = 2 then 3 else d + 2

let rec find_divisor_fast n test_divisor =
  if square test_divisor > n
  then n
  else if n mod test_divisor = 0
  then test_divisor
  else find_divisor_fast n (next_test_divisor test_divisor)
;;

let smallest_divisor_fast n = find_divisor_fast n 2

let rec find_divisor_naive n test_divisor =
  if square test_divisor > n
  then n
  else if n mod test_divisor = 0
  then test_divisor
  else find_divisor_naive n (test_divisor + 1)
;;

let smallest_divisor_naive n = find_divisor_naive n 2
let ex_1_23 n = smallest_divisor_fast n
