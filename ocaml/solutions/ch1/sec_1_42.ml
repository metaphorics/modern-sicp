(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program compose in SICP section 1.3
   exercise 1.42 *)

(** Exercise 1.42: [compose f g x = f (g x)], the book's [f after g]. *)

let compose f g x = f (g x)
let square x = x * x
let inc x = x + 1
let ex_1_42 () = compose square inc 6
