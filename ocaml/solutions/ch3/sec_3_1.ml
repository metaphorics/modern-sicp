(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.1 exercise 3.1 *)

(** Exercise 3.1: the running sum is a captured [ref], the same
    local-state idiom @ref{3.1.1} uses for [make_withdraw]. *)

let make_accumulator initial =
  let sum = ref initial in
  fun amount ->
    sum := !sum + amount;
    !sum
;;

let ex_3_01 () =
  let a = make_accumulator 5 in
  let first = a 10 in
  let second = a 10 in
  first, second
;;
