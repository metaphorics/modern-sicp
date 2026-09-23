(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program f in SICP section 1.2 exercise 1.11 *)

(** Exercise 1.11: [f_recursive] translates the rule directly into a
    tree-recursive process, one call per term. [f_iterative] slides a
    window of the three preceding values forward, state variables [a],
    [b], [c] holding [f(k+2)], [f(k+1)], [f(k)]; each step computes the
    next value and shifts the window, so the self-call is the whole
    result. *)

let rec f_recursive n =
  if n < 3
  then n
  else f_recursive (n - 1) + (2 * f_recursive (n - 2)) + (3 * f_recursive (n - 3))
;;

let f_iterative n =
  let rec iter a b c count =
    if count = 0 then a else iter (a + (2 * b) + (3 * c)) a b (count - 1)
  in
  if n < 3 then n else iter 2 1 0 (n - 2)
;;

let ex_1_11 n = f_recursive n, f_iterative n
