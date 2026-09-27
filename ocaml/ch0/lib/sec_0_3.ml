(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Bindings, functions, and modules: the named definitions behind the
    listings of section 0.3. *)

let square x = x * x
let sum_of_squares x y = square x + square y

let f x =
  let a = x + 1 in
  a + square a
;;

let rec factorial n = if n = 0 then 1 else n * factorial (n - 1)
let inc = ( + ) 1
let double = ( * ) 2
let pad_to ~width s = String.make (max 0 (width - String.length s)) '.' ^ s
