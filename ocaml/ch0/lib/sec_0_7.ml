(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Closures and lexical scope: the named definitions behind the listings
    of section 0.7. *)

let make_adder n = fun x -> x + n
let add_five = make_adder 5
let compose f g = fun x -> f (g x)

let make_withdraw initial =
  let balance = ref initial in
  fun amount ->
    if !balance < amount
    then None
    else (
      balance := !balance - amount;
      Some !balance)
;;

let withdrawal_sequence () =
  let w = make_withdraw 100 in
  let first = w 60 in
  let second = w 60 in
  first, second
;;

let independent_withdrawals () =
  let w1 = make_withdraw 100 in
  let w2 = make_withdraw 100 in
  let a = w1 20 in
  let b = w2 30 in
  let c = w1 80 in
  a, b, c
;;
