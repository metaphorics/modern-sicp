(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.56 *)

(** Exercise 2.56: extend the differentiator with the exponentiation
    variant [Power (base, exponent)] and the rule
    [d(u^n)/dx = n * u^(n - 1) * du/dx]. Anything raised to the power 0
    is 1 and anything raised to the power 1 is the base itself; both
    rules live in [make_exponentiation], the way [Deriv]'s
    [make_sum]/[make_product] build in their own rules. *)

type expr =
  | Const of int
  | Var of string
  | Sum of expr * expr
  | Prod of expr * expr
  | Power of expr * expr

let is_number exp n =
  match exp with
  | Const m -> m = n
  | _ -> false
;;

let same_variable v1 v2 =
  match v1, v2 with
  | Var a, Var b -> a = b
  | _ -> false
;;

let make_sum a1 a2 =
  if is_number a1 0
  then a2
  else if is_number a2 0
  then a1
  else (
    match a1, a2 with
    | Const m, Const n -> Const (m + n)
    | _ -> Sum (a1, a2))
;;

let make_product m1 m2 =
  if is_number m1 0 || is_number m2 0
  then Const 0
  else if is_number m1 1
  then m2
  else if is_number m2 1
  then m1
  else (
    match m1, m2 with
    | Const m, Const n -> Const (m * n)
    | _ -> Prod (m1, m2))
;;

let base = function
  | Power (b, _) -> b
  | _ -> invalid_arg "base: not a power"
;;

let exponent = function
  | Power (_, e) -> e
  | _ -> invalid_arg "exponent: not a power"
;;

let make_exponentiation base_exp exponent_exp =
  if is_number exponent_exp 0
  then Const 1
  else if is_number exponent_exp 1
  then base_exp
  else (
    match base_exp, exponent_exp with
    | Const b, Const e when e >= 0 ->
      Const (int_of_float (float_of_int b ** float_of_int e))
    | _ -> Power (base_exp, exponent_exp))
;;

let rec deriv exp var =
  match exp with
  | Const _ -> Const 0
  | Var _ -> if same_variable exp (Var var) then Const 1 else Const 0
  | Sum (a1, a2) -> make_sum (deriv a1 var) (deriv a2 var)
  | Prod (m1, m2) ->
    make_sum (make_product m1 (deriv m2 var)) (make_product (deriv m1 var) m2)
  | Power (u, n) ->
    make_product
      (make_product n (make_exponentiation u (make_sum n (Const (-1)))))
      (deriv u var)
;;

(** [ex_2_56 ()] is the derivative of [x^3] with respect to [x]. *)
let ex_2_56 () = deriv (Power (Var "x", Const 3)) "x"
