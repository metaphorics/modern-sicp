(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program * in SICP section 1.2 exercise 1.17 *)

(** Exercise 1.17: [fast_mult] mirrors [fast_expt] of @ref{1.2.4} with
    doubling standing in for squaring: on an even [b], halve [b] and
    double [a * halve(b)]'s partner instead of adding [a] to itself [b]
    times; on an odd [b], fold in one addition and recurse on [b - 1].
    Each halving roughly halves the work remaining, so the process is
    @math{{\Theta(\log b)}}. *)

let is_even n = n mod 2 = 0
let double x = x + x
let halve x = x / 2
let rec times a b = if b = 0 then 0 else a + times a (b - 1)

let rec fast_mult a b =
  if b = 0
  then 0
  else if is_even b
  then double (fast_mult a (halve b))
  else a + fast_mult a (b - 1)
;;

let ex_1_17 a b = fast_mult a b
