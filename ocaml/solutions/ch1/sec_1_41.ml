(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program double in SICP section 1.3
   exercise 1.41 *)

(** Exercise 1.41: [double f x = f (f x)] applies [f] to its own
    result once. [double double] therefore applies its argument 4
    times; [double (double double)] applies its argument 16 times, so
    [((double (double double)) inc) 5 = 5 + 16 = 21]. *)

let double f x = f (f x)
let inc x = x + 1
let ex_1_41 () = double (double double) inc 5
