(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program repeated in SICP section 1.3
   exercise 1.43 *)

(** Exercise 1.43: [repeated f n] folds [n] copies of [f] together with
    exercise 1.42's [compose], the hint the book gives. *)

let compose f g x = f (g x)
let identity x = x

let repeated f n =
  let rec go n acc = if n = 0 then acc else go (n - 1) (compose f acc) in
  go n identity
;;

let square x = x * x
let ex_1_43 () = repeated square 2 5
