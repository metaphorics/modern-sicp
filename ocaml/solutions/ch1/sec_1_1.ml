(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_01 in SICP section 1.1 *)

(** Exercise 1.1: evaluate the section's expression sequence in order.
    The sequence evaluates to ten integers and one boolean, in
    source order. The definitions of [a] and [b] contribute no
    results; [a] is 3 and [b] is 4 for the interactions that follow
    them. *)

let ex_1_01 () =
  ( [ 10
    ; 5 + 3 + 4
    ; 9 - 1
    ; 6 / 2
    ; (2 * 4) + (4 - 6)
    ; 3 + 4 + (3 * 4)
    ; (if 4 > 3 && 4 < 3 * 4 then 4 else 3)
    ; (if 3 = 4 then 6 else if 4 = 4 then 6 + 7 + 3 else 25)
    ; (2 + if 4 > 3 then 4 else 3)
    ; (if 3 > 4 then 3 else if 3 < 4 then 4 else -1) * (3 + 1)
    ]
  , 3 = 4 )
;;
