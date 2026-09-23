(* SPDX-License-Identifier: GPL-3.0-only *)

type expr = Expr of view
and definition = Def of definition_view

and datum =
  | DInt of int
  | DFloat of float
  | DBool of bool
  | DString of string
  | DSymbol of string
  | DNil
  | DPair of datum * datum

and view =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Variable of string
  | Quote of datum
  | Definition of definition
  | Set of string * expr
  | If of expr * expr * expr option
  | Cond of (expr * expr list) list * expr list option
  | And of expr list
  | Or of expr list
  | Sequence of expr list
  | Let of (string * expr) list * expr list
  | Lambda of string list * expr list
  | Application of expr * expr list

and definition_view =
  | Define_variable of string * expr
  | Define_function of
      { name : string
      ; parameters : string list
      ; body : expr list
      }

let view (Expr v) = v
let view_definition (Def d) = d
let int n = Expr (Int n)
let float f = Expr (Float f)
let bool b = Expr (Bool b)
let string s = Expr (String s)
let variable name = Expr (Variable name)
let quote datum = Expr (Quote datum)
let definition d = Expr (Definition d)
let set name e = Expr (Set (name, e))
let if_ c t a = Expr (If (c, t, a))
let and_ operands = Expr (And operands)
let or_ operands = Expr (Or operands)
let application operator operands = Expr (Application (operator, operands))

let nonempty_body what body =
  match body with
  | [] -> Error (Eval_error.Invalid_form (what ^ " needs a nonempty body"))
  | _ -> Ok body
;;

let sequence body =
  nonempty_body "begin" body |> Result.map (fun body -> Expr (Sequence body))
;;

let lambda parameters body =
  nonempty_body "lambda" body |> Result.map (fun body -> Expr (Lambda (parameters, body)))
;;

let let_ bindings body =
  nonempty_body "let" body |> Result.map (fun body -> Expr (Let (bindings, body)))
;;

let cond clauses else_body =
  let bad detail = Error (Eval_error.Invalid_form detail) in
  if clauses = []
  then bad "cond needs clauses"
  else (
    match else_body with
    | Some [] -> bad "cond else clause needs a body"
    | Some _ | None -> Ok (Expr (Cond (clauses, else_body))))
;;

let define_variable name e = Def (Define_variable (name, e))

let define_function name parameters body =
  nonempty_body ("define " ^ name) body
  |> Result.map (fun body -> Def (Define_function { name; parameters; body }))
;;
