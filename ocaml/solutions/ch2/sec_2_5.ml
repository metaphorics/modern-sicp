(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cons in SICP section 2.1
   exercise 2.5 *)

(** [car]/[cdr] recover [a]/[b] by dividing out the other's prime
    repeatedly and counting the divisions: [2] and [3] are coprime, so
    dividing [2^a * 3^b] by [2] while it divides evenly strips exactly
    the [2^a] factor, leaving [3^b] untouched, and symmetrically for
    [3]. *)

let rec pow base exponent = if exponent = 0 then 1 else base * pow base (exponent - 1)
let cons a b = pow 2 a * pow 3 b

let rec count_factor n factor =
  if n mod factor <> 0 then 0 else 1 + count_factor (n / factor) factor
;;

let car z = count_factor z 2
let cdr z = count_factor z 3
let ex_2_05 a b = car (cons a b) = a && cdr (cons a b) = b
