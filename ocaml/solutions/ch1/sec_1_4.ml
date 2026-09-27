(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_04 in SICP section 1.1 *)

(** Exercise 1.4: the conditional yields the operator itself, applied
    to [a] and [b]. In OCaml the operator names [( + )] and [( - )]
    are ordinary function values, so the translation is direct. *)

let ex_1_04 a b = (if b > 0 then ( + ) else ( - )) a b
