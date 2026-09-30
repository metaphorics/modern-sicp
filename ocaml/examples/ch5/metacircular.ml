(* SPDX-License-Identifier: GPL-3.0-only *)

let evaluator =
  {|type expr =
  | EInt of int
  | EBool of bool
  | EUnit
  | EVar of string
  | EIf of expr * expr * expr
  | ELambda of string * expr
  | EApply of expr * expr
  | ELet of string * expr * expr
  | ELetRec of string * string * expr * expr
  | EAdd of expr * expr
  | ESub of expr * expr
  | EEqual of expr * expr
  | ELess of expr * expr
  | ETuple of expr list
  | EConstructor of string * expr list
  | EMatch of expr * (pattern * expr) list
  | EListNil
  | EListCons of expr * expr
  | ERef of expr
  | EDeref of expr
  | EAssign of expr * expr
  | EArrayMake of expr * expr
  | EArrayGet of expr * expr
  | EArraySet of expr * expr * expr
  | ESequence of expr * expr

and pattern =
  | PInt of int
  | PBool of bool
  | PUnit
  | PVar of string
  | PWildcard
  | PTuple of pattern list
  | PConstructor of string * pattern list
  | PListNil
  | PListCons of pattern * pattern

type value =
  | VInt of int
  | VBool of bool
  | VUnit
  | VTuple of value list
  | VConstructor of string * value list
  | VListNil
  | VListCons of value * value
  | VClosure of string * expr * env
  | VRef of value ref
  | VArray of value array
  | VError

and env = (string * value) list

let rec extend bindings environment =
  match bindings with
  | [] -> environment
  | (name, value) :: rest -> (name, value) :: extend rest environment

let rec append left right =
  match left with
  | [] -> right
  | item :: rest -> item :: append rest right

let rec bind_patterns patterns values =
  match patterns, values with
  | [], [] -> Some []
  | pattern :: remaining_patterns, value :: remaining_values ->
      (match bind_pattern pattern value with
       | None -> None
       | Some bindings ->
           (match bind_patterns remaining_patterns remaining_values with
            | None -> None
            | Some rest -> Some (append bindings rest)))
  | _, _ -> None
and bind_pattern pattern value =
  match pattern, value with
  | PWildcard, _ -> Some []
  | PVar name, value -> Some [(name, value)]
  | PUnit, VUnit -> Some []
  | PInt expected, VInt actual -> if expected = actual then Some [] else None
  | PBool expected, VBool actual -> if expected = actual then Some [] else None
  | PTuple patterns, VTuple values -> bind_patterns patterns values
  | PConstructor (expected_name, patterns), VConstructor (name, values) ->
      if expected_name = name then bind_patterns patterns values else None
  | PListNil, VListNil -> Some []
  | PListCons (head_pattern, tail_pattern), VListCons (head, tail) ->
      bind_patterns [head_pattern; tail_pattern] [head; tail]
  | _, _ -> None

let rec eval environment expression =
  match expression with
  | EInt n -> VInt n
  | EBool b -> VBool b
  | EUnit -> VUnit
  | EVar name ->
      (match lookup name environment with Some value -> value | None -> VError)
  | EIf (condition, consequent, alternative) ->
      (match eval environment condition with
       | VBool true -> eval environment consequent
       | VBool false -> eval environment alternative
       | _ -> VError)
  | ELambda (parameter, body) -> VClosure (parameter, body, environment)
  | EApply (procedure, argument) ->
      let procedure_value = eval environment procedure in
      let argument_value = eval environment argument in
      (match procedure_value with
       | VClosure (parameter, body, saved_environment) ->
           eval ((parameter, argument_value) :: saved_environment) body
       | _ -> VError)
  | ELet (name, right_hand_side, body) ->
      let value = eval environment right_hand_side in
      eval ((name, value) :: environment) body
  | ELetRec (name, parameter, function_body, body) ->
      let rec recursive_closure =
        VClosure (parameter, function_body, (name, recursive_closure) :: environment)
      in
      eval ((name, recursive_closure) :: environment) body
  | EAdd (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VInt (x + y)
       | _, _ -> VError)
  | ESub (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VInt (x - y)
       | _, _ -> VError)
  | EEqual (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VBool (x = y)
       | VBool x, VBool y -> VBool (x = y)
       | VUnit, VUnit -> VBool true
       | _, _ -> VError)
  | ELess (left, right) ->
      let left_value = eval environment left in
      let right_value = eval environment right in
      (match left_value, right_value with
       | VInt x, VInt y -> VBool (x < y)
       | _, _ -> VError)
  | ETuple expressions -> VTuple (eval_many environment expressions)
  | EConstructor (name, expressions) ->
      VConstructor (name, eval_many environment expressions)
  | EMatch (scrutinee, cases) -> eval_cases environment (eval environment scrutinee) cases
  | EListNil -> VListNil
  | EListCons (head, tail) ->
      let head_value = eval environment head in
      let tail_value = eval environment tail in
      VListCons (head_value, tail_value)
  | ERef initial_expression -> VRef (ref (eval environment initial_expression))
  | EDeref reference ->
      (match eval environment reference with VRef cell -> !cell | _ -> VError)
  | EAssign (reference, right_hand_side) ->
      let reference_value = eval environment reference in
      let assigned_value = eval environment right_hand_side in
      (match reference_value with
       | VRef cell -> cell := assigned_value; VUnit
       | _ -> VError)
  | EArrayMake (length_expression, initial_expression) ->
      let length_value = eval environment length_expression in
      let initial_value = eval environment initial_expression in
      (match length_value with
       | VInt length -> if length < 0 then VError else VArray (Array.make length initial_value)
       | _ -> VError)
  | EArrayGet (array_expression, index_expression) ->
      let array_value = eval environment array_expression in
      let index_value = eval environment index_expression in
      (match array_value, index_value with
       | VArray array, VInt index ->
           if index < 0 || index >= Array.length array then VError else Array.get array index
       | _, _ -> VError)
  | EArraySet (array_expression, index_expression, value_expression) ->
      let array_value = eval environment array_expression in
      let index_value = eval environment index_expression in
      let new_value = eval environment value_expression in
      (match array_value, index_value with
       | VArray array, VInt index ->
           if index < 0 || index >= Array.length array then VError
           else (Array.set array index new_value; VUnit)
       | _, _ -> VError)
  | ESequence (first, second) ->
      let _ = eval environment first in
      eval environment second
and eval_many environment expressions =
  match expressions with
  | [] -> []
  | expression :: rest ->
      let value = eval environment expression in
      value :: eval_many environment rest
and eval_cases environment value cases =
  match cases with
  | [] -> VError
  | (pattern, body) :: rest ->
      (match bind_pattern pattern value with
       | None -> eval_cases environment value rest
       | Some bindings -> eval (extend bindings environment) body)
and lookup name environment =
  match environment with
  | [] -> None
  | (bound_name, value) :: rest ->
      if name = bound_name then Some value else lookup name rest
and show value =
  match value with
  | VInt n -> string_of_int n
  | VBool b -> if b then "true" else "false"
  | VUnit -> "unit"
  | VTuple values -> "tuple"
  | VConstructor (name, _) -> name
  | VListNil -> "list"
  | VListCons _ -> "list"
  | VClosure _ -> "closure"
  | VRef _ -> "ref"
  | VArray _ -> "array"
  | VError -> "error"
|}
;;

let with_guest guest =
  evaluator
  ^ "\nlet guest_program =\n"
  ^ guest
  ^ "\n\nlet () = print_endline (show (eval [] guest_program))\n"
;;
