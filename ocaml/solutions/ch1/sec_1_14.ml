(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program count-change in SICP section 1.2
   exercise 1.14 *)

(** Exercise 1.14: the tree is drawn in [ex_1_14.md]; this file counts
    its nodes instead of drawing it, one call to [cc] per node
    (including the leaves that return 1 or 0). [ex_1_14a] measures the
    count at four doubling amounts to check the accepted
    @math{{\Theta(a^5)}} steps / @math{{\Theta(a)}} space answer against
    real numbers instead of the argument alone. *)

let first_denomination kinds_of_coins =
  match kinds_of_coins with
  | 1 -> 1
  | 2 -> 5
  | 3 -> 10
  | 4 -> 25
  | _ -> 50
;;

let cc_with_count amount =
  let calls = ref 0 in
  let rec cc amount kinds_of_coins =
    incr calls;
    if amount = 0
    then 1
    else if amount < 0 || kinds_of_coins = 0
    then 0
    else
      cc amount (kinds_of_coins - 1)
      + cc (amount - first_denomination kinds_of_coins) kinds_of_coins
  in
  let ways = cc amount 5 in
  ways, !calls
;;

let ex_1_14 () =
  let _, calls = cc_with_count 11 in
  calls
;;

let ex_1_14a () =
  List.map (fun amount -> amount, snd (cc_with_count amount)) [ 50; 100; 200; 400 ]
;;
