(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.4 exercise 2.73 *)

(** Exercise 2.73: rewrite [deriv] to dispatch data-directedly on the
    operator of a compound expression, instead of testing [is_sum]
    and [is_product] with [cond]. Part (a): [number?]/[variable?]
    (here, [Const]/[Var]) cannot be assimilated into the table
    because they carry no operator string to key a lookup on -- they
    are the base cases the table dispatch exists to fall through to,
    not further cases of it. Part (d): keying the table the other
    way, on the operator first and the operation second, is exactly
    what [Data_directed]'s [install_rectangular_package] does in the
    main text: each operator would own a small sub-table of its own
    rules, and adding an operator would mean adding one sub-table
    rather than one row spread across every operation's table. *)

type expr =
  | Const of int
  | Var of string
  | Compound of string * expr list

let operator = function
  | Compound (op, _) -> op
  | Const _ | Var _ -> invalid_arg "operator: not a compound expression"
;;

let operands = function
  | Compound (_, args) -> args
  | Const _ | Var _ -> invalid_arg "operands: not a compound expression"
;;

type deriv_rule = (expr -> string -> expr) -> expr list -> string -> expr
type table = (string, deriv_rule) Hashtbl.t

let make_table () : table = Hashtbl.create 8

let is_number exp n =
  match exp with
  | Const m -> m = n
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
    | _ -> Compound ("+", [ a1; a2 ]))
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
    | _ -> Compound ("*", [ m1; m2 ]))
;;

let make_expt base n =
  if n = 0 then Const 1 else if n = 1 then base else Compound ("**", [ base; Const n ])
;;

let rec deriv table exp var =
  match exp with
  | Const _ -> Const 0
  | Var v -> if String.equal v var then Const 1 else Const 0
  | Compound (op, args) ->
    (match Hashtbl.find_opt table op with
     | Some rule -> rule (fun e v -> deriv table e v) args var
     | None -> invalid_arg (Printf.sprintf "deriv: unknown expression type: %s" op))
;;

let install_sum_rule table =
  Hashtbl.replace table "+" (fun deriv args var ->
    match args with
    | [ a1; a2 ] -> make_sum (deriv a1 var) (deriv a2 var)
    | _ -> invalid_arg "+: deriv expects two operands")
;;

let install_product_rule table =
  Hashtbl.replace table "*" (fun deriv args var ->
    match args with
    | [ m1; m2 ] ->
      make_sum (make_product m1 (deriv m2 var)) (make_product (deriv m1 var) m2)
    | _ -> invalid_arg "*: deriv expects two operands")
;;

let install_expt_rule table =
  Hashtbl.replace table "**" (fun deriv args var ->
    match args with
    | [ base; Const n ] ->
      make_product (make_product (Const n) (make_expt base (n - 1))) (deriv base var)
    | _ -> invalid_arg "**: deriv expects a base and a constant exponent")
;;

(** [ex_2_73 ()] installs every rule this edition provides and
    differentiates [x + 3], [x * y], and [x**3] with respect to [x]:
    [1], [y], and [3 * x**2]. *)
let ex_2_73 () =
  let table = make_table () in
  install_sum_rule table;
  install_product_rule table;
  install_expt_rule table;
  ( deriv table (Compound ("+", [ Var "x"; Const 3 ])) "x"
  , deriv table (Compound ("*", [ Var "x"; Var "y" ])) "x"
  , deriv table (Compound ("**", [ Var "x"; Const 3 ])) "x" )
;;
