(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.57 *)

(** Exercise 2.57: extend the differentiator to sums and products of
    two or more terms. [Sum] and [Prod] now hold a term list instead
    of a pair, so [addend]/[augend] and [multiplier]/[multiplicand]
    split the list into its first term and the rest; [deriv] keeps the
    book's sum and product rules, generalized from two terms to a
    list of them. *)

type expr =
  | Const of int
  | Var of string
  | Sum of expr list
  | Prod of expr list

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

(** [numeric_fold combine unit_value terms] folds every [Const] term
    of [terms] into one number with [combine], starting from
    [unit_value], and returns that number alongside the non-numeric
    terms in their original order. *)
let numeric_fold combine unit_value terms =
  List.fold_left
    (fun (acc, rest) term ->
       match term with
       | Const n -> combine acc n, rest
       | _ -> acc, rest @ [ term ])
    (unit_value, [])
    terms
;;

(** [make_sum terms] drops the constant [0]s, folds the remaining
    constants into one, and collapses to a single term or [Const 0]
    when nothing else is left. *)
let make_sum terms =
  let total, others = numeric_fold ( + ) 0 terms in
  let others = if total = 0 then others else others @ [ Const total ] in
  match others with
  | [] -> Const 0
  | [ single ] -> single
  | _ -> Sum others
;;

let addend = function
  | Sum (a :: _) -> a
  | _ -> invalid_arg "addend: not a sum"
;;

let augend = function
  | Sum (_ :: rest) -> make_sum rest
  | _ -> invalid_arg "augend: not a sum"
;;

(** [make_product factors] is [0] when any factor is [0]; otherwise it
    drops the constant [1]s, folds the remaining constants into one,
    and collapses to a single factor or [Const 1] when nothing else is
    left. *)
let make_product factors =
  if List.exists (fun f -> is_number f 0) factors
  then Const 0
  else (
    let total, others = numeric_fold ( * ) 1 factors in
    let others = if total = 1 then others else others @ [ Const total ] in
    match others with
    | [] -> Const 1
    | [ single ] -> single
    | _ -> Prod others)
;;

let multiplier = function
  | Prod (m :: _) -> m
  | _ -> invalid_arg "multiplier: not a product"
;;

let multiplicand = function
  | Prod (_ :: rest) -> make_product rest
  | _ -> invalid_arg "multiplicand: not a product"
;;

(** [drop_nth n terms] is [terms] without its [n]-th element. *)
let drop_nth n terms = List.filteri (fun i _ -> i <> n) terms

let rec deriv exp var =
  match exp with
  | Const _ -> Const 0
  | Var _ -> if same_variable exp (Var var) then Const 1 else Const 0
  | Sum terms -> make_sum (List.map (fun t -> deriv t var) terms)
  | Prod terms ->
    (* Each term contributes its own derivative times every other
       term unchanged, and the contributions add. *)
    make_sum
      (List.mapi (fun i term -> make_product (deriv term var :: drop_nth i terms)) terms)
;;

(** [ex_2_57 ()] is the derivative of [x * y * (x + 3)] with respect
    to [x]. *)
let ex_2_57 () = deriv (Prod [ Var "x"; Var "y"; Sum [ Var "x"; Const 3 ] ]) "x"
