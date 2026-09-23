(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fib in SICP section 1.2 exercise 1.13 *)

(** Exercise 1.13: the proof is by induction on [n] and belongs in
    [ex_1_13.md]; this file carries only the empirical check the proof
    predicts. [closed_form_fib] evaluates the closed form with [float]
    and rounds; [fib_direct] is the book's tree-recursive [fib], kept
    for comparison and never called above [n] a tree recursion can
    still afford. *)

let phi = (1.0 +. sqrt 5.0) /. 2.0
let psi = (1.0 -. sqrt 5.0) /. 2.0

let closed_form_fib n =
  Float.to_int
    (Float.round (((phi ** float_of_int n) -. (psi ** float_of_int n)) /. sqrt 5.0))
;;

let rec fib_direct n =
  if n = 0 then 0 else if n = 1 then 1 else fib_direct (n - 1) + fib_direct (n - 2)
;;

let ex_1_13 n = closed_form_fib n
