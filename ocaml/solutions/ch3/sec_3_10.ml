(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.2 exercise 3.10 *)

(** Exercise 3.10: the lifetime of the ref cell that [make_withdraw]
    allocates at its [let] binding and its returned closure captures.
    The traced variant hands back the cell as a witness, so the exercise
    can show by physical equality that one account's every call reaches
    the same cell while two accounts hold different cells. *)

type withdraw_result =
  | Balance of int
  | Insufficient_funds

let make_withdraw balance =
  let balance = ref balance in
  fun amount ->
    if !balance >= amount
    then (
      balance := !balance - amount;
      Balance !balance)
    else Insufficient_funds
;;

type withdraw_with_cell =
  { withdraw : int -> withdraw_result
  ; balance_cell : int ref
  }

let make_withdraw_traced initial =
  let balance = ref initial in
  let withdraw amount =
    if !balance >= amount
    then (
      balance := !balance - amount;
      Balance !balance)
    else Insufficient_funds
  in
  { withdraw; balance_cell = balance }
;;

let ex_3_10 () =
  let w1 = make_withdraw_traced 100 in
  let w2 = make_withdraw_traced 100 in
  ignore (w1.withdraw 50);
  let contents_after_first_call = !(w1.balance_cell) in
  let cells_distinct = not (w1.balance_cell == w2.balance_cell) in
  ignore (w2.withdraw 30);
  let w1_sees = !(w1.balance_cell) in
  let w2_sees = !(w2.balance_cell) in
  (* A direct write to the returned cell reaches the very cell the
     closure reads: the next withdrawal starts from 1000. *)
  w1.balance_cell := 1_000;
  let sees_direct_write = w1.withdraw 300 in
  contents_after_first_call, cells_distinct, (w1_sees, w2_sees), sees_direct_write
;;
