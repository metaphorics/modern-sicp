(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

let sum_with_product = 2 + (4 * 6)
let square x = x * x
let seven_squared = square 7
let larger a b = if a > b then a else b
let larger_of_4_and_11 = larger 4 11
let anonymous_square = (fun x -> x * x) 5
