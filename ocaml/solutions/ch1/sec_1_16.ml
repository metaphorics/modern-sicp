(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fast-expt in SICP section 1.2
   exercise 1.16 *)

(** Exercise 1.16: the invariant quantity [a * b^n] never changes from
    state to state. On an even [n], halving [n] and squaring [b] keeps
    it: [a * (b^2)^(n/2) = a * b^n]. On an odd [n], folding one factor
    of [b] into [a] keeps it: [(a * b) * b^(n-1) = a * b^n]. When [n]
    reaches 0 the invariant reads [a * b^0 = a], the answer. *)

let is_even n = n mod 2 = 0

let rec fast_expt_iter a b n =
  if n = 0
  then a
  else if is_even n
  then fast_expt_iter a (b * b) (n / 2)
  else fast_expt_iter (a * b) b (n - 1)
;;

let ex_1_16 b n = fast_expt_iter 1 b n
