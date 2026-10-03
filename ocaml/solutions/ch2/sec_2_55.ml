(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 2.3 exercise 2.55 *)

(** Exercise 2.55 (this edition's replacement): quotation as data.  A
    quoted expression is a symbolic datum; writing the quotation of a
    quotation builds a two-element list whose first element is the
    symbol [quote] itself, which is what Eva Lu Ator saw printed. *)

type datum =
  | Symbol of string
  | List of datum list

let quote d = List [ Symbol "quote"; d ]

let car = function
  | List (first :: _) -> first
  | _ -> invalid_arg "car: not a nonempty list"
;;

let name_of = function
  | Symbol s -> s
  | List _ -> invalid_arg "name_of: not a symbol"
;;

(** The outer quotation evaluates to the datum it wraps: the quotation
    of [abracadabra], whose [car] is the symbol [quote]. *)
let ex_2_55 () =
  match quote (quote (Symbol "abracadabra")) with
  | List [ Symbol "quote"; quoted ] -> name_of (car quoted)
  | _ -> invalid_arg "ex_2_55: a quotation is a two-element list"
;;
