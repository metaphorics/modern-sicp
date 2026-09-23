(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.25 *)

(** Exercise 1.25: Alyssa is right about @math{{\Theta(\log n)}}
    multiplications in a language with unbounded integers, where
    [fast_expt] never wraps. This edition's [int] is 63 bits, so
    [expmod_naive] silently corrupts once [base ** exp] passes
    @math{2^{62} - 1}: [7 ** 200] does at [7 ** 23], long before the
    exponent 200 the test tries. [expmod_correct] never builds a value
    near [base ** exp]; every intermediate is already a remainder below
    [m]. [ex_1_25.md] works the concrete case. *)

let is_even n = n mod 2 = 0
let square x = x * x

let rec expmod_correct base exp m =
  if exp = 0
  then 1
  else if is_even exp
  then square (expmod_correct base (exp / 2) m) mod m
  else base * expmod_correct base (exp - 1) m mod m
;;

let rec fast_expt b n =
  if n = 0
  then 1
  else if is_even n
  then square (fast_expt b (n / 2))
  else b * fast_expt b (n - 1)
;;

let expmod_naive base exp m = fast_expt base exp mod m
let ex_1_25 () = expmod_correct 7 200 13, expmod_naive 7 200 13
