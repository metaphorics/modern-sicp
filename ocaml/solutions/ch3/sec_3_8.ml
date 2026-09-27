(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.8 *)

(** Exercise 3.8: a procedure [f] such that [f 0 + f 1] exposes
    OCaml's operand evaluation order, the same way the book's
    [(+ (f 0) (f 1))] exposes Scheme's. OCaml's evaluation order for
    function arguments is unspecified by the language, the same
    premise the SICP exercise trades on for Scheme, so this exercise
    keeps its number and its statement unchanged (map class [T]). *)

let make_order_probe () =
  let first_call = ref true in
  fun x ->
    if !first_call
    then (
      first_call := false;
      x)
    else 0
;;

let ex_3_08 () =
  let f = make_order_probe () in
  f 0 + f 1
;;
