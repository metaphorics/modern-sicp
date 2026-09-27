(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.19: counting change over a coin list. *)

let us_coins = [ 50.0; 25.0; 10.0; 5.0; 1.0 ]
let uk_coins = [ 100.0; 50.0; 20.0; 10.0; 5.0; 2.0; 1.0; 0.5 ]

let first_denomination = function
  | coin :: _ -> coin
  | [] -> invalid_arg "first_denomination: empty coin list"
;;

let except_first_denomination = function
  | _ :: rest -> rest
  | [] -> []
;;

let no_more = function
  | [] -> true
  | _ :: _ -> false
;;

let rec cc amount coin_values =
  if amount = 0.0
  then 1
  else if amount < 0.0 || no_more coin_values
  then 0
  else
    cc amount (except_first_denomination coin_values)
    + cc (amount -. first_denomination coin_values) coin_values
;;

let ex_2_19 () = cc 100.0 us_coins
