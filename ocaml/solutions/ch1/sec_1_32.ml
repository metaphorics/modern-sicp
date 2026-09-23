(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program accumulate in SICP section 1.3
   exercise 1.32 *)

(** Exercise 1.32: [accumulate]'s [iter] folds [term a] into [result]
    with [combiner] and steps to [next a], the same shape [sum] and
    [product] already used; part b's iterative process falls out of
    writing it this way. [sum] is [accumulate (+.) 0.]; [product] is
    [accumulate ( *. ) 1.]. *)

let accumulate combiner null_value term a next b =
  let rec iter a result =
    if a > b then result else iter (next a) (combiner result (term a))
  in
  iter a null_value
;;

let sum_via_accumulate term a next b = accumulate ( +. ) 0.0 term a next b
let product_via_accumulate term a next b = accumulate ( *. ) 1.0 term a next b
let identity x = x
let inc x = x +. 1.0

let ex_1_32 () =
  sum_via_accumulate identity 1.0 inc 10.0, product_via_accumulate identity 1.0 inc 6.0
;;
