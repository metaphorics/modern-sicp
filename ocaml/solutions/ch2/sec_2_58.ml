(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.58 *)

(** Exercise 2.58: differentiate ordinary infix notation instead of
    prefix lists. Part (a) represents a fully parenthesized infix
    expression as its own variant, [infix], with [Plus]/[Times] taking
    the place of [Sum]/[Prod]; [deriv] keeps the book's two rules,
    written over the new constructors. Part (b) drops the requirement
    that every expression be fully parenthesized: [parse_standard]
    reads a token list under the usual precedence, where [*] binds
    tighter than [+], into the very same [infix] tree, so [deriv]
    needs no second version. *)

type infix =
  | Num of int
  | Var of string
  | Plus of infix * infix
  | Times of infix * infix

let is_number exp n =
  match exp with
  | Num m -> m = n
  | _ -> false
;;

let make_sum a1 a2 =
  if is_number a1 0
  then a2
  else if is_number a2 0
  then a1
  else (
    match a1, a2 with
    | Num m, Num n -> Num (m + n)
    | _ -> Plus (a1, a2))
;;

let make_product m1 m2 =
  if is_number m1 0 || is_number m2 0
  then Num 0
  else if is_number m1 1
  then m2
  else if is_number m2 1
  then m1
  else (
    match m1, m2 with
    | Num m, Num n -> Num (m * n)
    | _ -> Times (m1, m2))
;;

let rec deriv exp var =
  match exp with
  | Num _ -> Num 0
  | Var v -> if v = var then Num 1 else Num 0
  | Plus (a1, a2) -> make_sum (deriv a1 var) (deriv a2 var)
  | Times (m1, m2) ->
    make_sum (make_product m1 (deriv m2 var)) (make_product (deriv m1 var) m2)
;;

(** Part (a): [x + (3 * (x + (y + 2)))], fully parenthesized. *)
let ex_2_58_parenthesized_expr =
  Plus (Var "x", Times (Num 3, Plus (Var "x", Plus (Var "y", Num 2))))
;;

let ex_2_58_parenthesized () = deriv ex_2_58_parenthesized_expr "x"

(* Part (b): a token reader that respects precedence, so parentheses
   are needed only to override it. *)
type tok =
  | TNum of int
  | TVar of string
  | TPlus
  | TTimes
  | TOpen
  | TClose

let rec parse_sum toks =
  let first, rest = parse_term toks in
  match rest with
  | TPlus :: rest' ->
    let next, rest'' = parse_sum rest' in
    Plus (first, next), rest''
  | _ -> first, rest

and parse_term toks =
  let first, rest = parse_factor toks in
  match rest with
  | TTimes :: rest' ->
    let next, rest'' = parse_term rest' in
    Times (first, next), rest''
  | _ -> first, rest

and parse_factor toks =
  match toks with
  | TNum n :: rest -> Num n, rest
  | TVar v :: rest -> Var v, rest
  | TOpen :: rest ->
    let inner, rest' = parse_sum rest in
    (match rest' with
     | TClose :: rest'' -> inner, rest''
     | _ -> invalid_arg "parse_standard: expected a closing parenthesis")
  | _ -> invalid_arg "parse_standard: expected a number, a variable, or (\")\""
;;

(** [parse_standard toks] is the [infix] tree [toks] names under the
    usual precedence, where [*] binds tighter than [+].
    [invalid_arg] on a malformed token list. *)
let parse_standard toks =
  match parse_sum toks with
  | tree, [] -> tree
  | _, _ :: _ -> invalid_arg "parse_standard: trailing tokens"
;;

(** [x + 3 * (x + y + 2)], written without the parentheses the fully
    parenthesized form of part (a) requires around every sum. *)
let ex_2_58_standard_tokens =
  [ TVar "x"
  ; TPlus
  ; TNum 3
  ; TTimes
  ; TOpen
  ; TVar "x"
  ; TPlus
  ; TVar "y"
  ; TPlus
  ; TNum 2
  ; TClose
  ]
;;

let ex_2_58_standard () = deriv (parse_standard ex_2_58_standard_tokens) "x"
