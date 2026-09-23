(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_02 in SICP section 1.1 *)

(** Exercise 1.2, replaced for this edition: the fraction expressed as
    pure nested calls. Only the four named combinators appear; no
    infix operator is used in the expression itself. *)

let add x y = x +. y
let sub x y = x -. y
let mul x y = x *. y
let div x y = x /. y

let ex_1_02 () =
  div
    (add (add 5.0 4.0) (sub 2.0 (sub 3.0 (add 6.0 (div 4.0 5.0)))))
    (mul (mul 3.0 (sub 6.0 2.0)) (sub 2.0 7.0))
;;
