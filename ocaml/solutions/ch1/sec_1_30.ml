(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sum in SICP section 1.3
   exercise 1.30 *)

(** Exercise 1.30: filling the book's blanks in OCaml, [iter]'s base
    case is [a > b], and its step folds [term a] into [result] before
    moving [a] to [next a]; every call to [iter] is in tail position,
    so this runs in constant stack space. *)

let sum_iterative term a next b =
  let rec iter a result = if a > b then result else iter (next a) (result +. term a) in
  iter a 0.0
;;

let identity x = x
let inc x = x +. 1.0
let ex_1_30 () = sum_iterative identity 1.0 inc 10.0
