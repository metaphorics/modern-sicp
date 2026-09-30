(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.10 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

type term =
  | Lit of Ast.scalar
  | Name of string
  | Fn of string list * term
  | Do of term list
  | Call of term * term list
  | Op of string * term * term
  | When of term * term * term
  | Rec of string * term * term

let invalid detail = Error (Eval_error.Invalid_form detail)

let operator = function
  | "+" -> Ok (fun a b -> Ast.arith Ast.Add a b)
  | "-" -> Ok (fun a b -> Ast.arith Ast.Sub a b)
  | "*" -> Ok (fun a b -> Ast.arith Ast.Mul a b)
  | "/" -> Ok (fun a b -> Ast.arith Ast.Div a b)
  | "=" -> Ok (fun a b -> Ast.compare_ Ast.Eq a b)
  | "<" -> Ok (fun a b -> Ast.compare_ Ast.Lt a b)
  | ">" -> Ok (fun a b -> Ast.compare_ Ast.Gt a b)
  | "^" -> Ok (fun a b -> Ast.concat a b)
  | other -> invalid ("unknown operator " ^ other)
;;

let rec from_new_syntax = function
  | Lit s -> Ok (Ast.scalar s)
  | Name n -> Ok (Ast.var n)
  | Fn ([], _) -> invalid "fn needs a parameter"
  | Fn (parameters, body) ->
    let* body = from_new_syntax body in
    Ok (Ast.fun_ parameters body)
  | Do [] -> invalid "do needs a term"
  | Do [ last ] -> from_new_syntax last
  | Do (first :: rest) ->
    let* first = from_new_syntax first in
    let* rest = from_new_syntax (Do rest) in
    Ok (Ast.sequence first rest)
  | Call (_, []) -> invalid "call needs an argument"
  | Call (f, args) ->
    let* f = from_new_syntax f in
    let* args = all args in
    Ok (Ast.apply f args)
  | Op (name, a, b) ->
    let* build = operator name in
    let* a = from_new_syntax a in
    let* b = from_new_syntax b in
    Ok (build a b)
  | When (c, t, f) ->
    let* c = from_new_syntax c in
    let* t = from_new_syntax t in
    let* f = from_new_syntax f in
    Ok (Ast.if_ c t f)
  | Rec (name, definition, body) ->
    let* definition = from_new_syntax definition in
    let* body = from_new_syntax body in
    Ok (Ast.let_ true [ { Ast.name = Some name; rhs = definition } ] body)

and all = function
  | [] -> Ok []
  | t :: rest ->
    let* e = from_new_syntax t in
    let* es = all rest in
    Ok (e :: es)
;;

let eval_term term env =
  let* e = from_new_syntax term in
  S.eval_expr e env
;;

let factorial =
  Rec
    ( "fact"
    , Fn
        ( [ "n" ]
        , When
            ( Op ("=", Name "n", Lit (Ast.Int 0))
            , Lit (Ast.Int 1)
            , Op
                ( "*"
                , Name "n"
                , Call (Name "fact", [ Op ("-", Name "n", Lit (Ast.Int 1)) ]) ) ) )
    , Call (Name "fact", [ Lit (Ast.Int 6) ]) )
;;

let ex_4_10 () =
  let run term =
    let out = Buffer.create 16 in
    let env = S.the_global_environment ~emit:(Buffer.add_string out) () in
    match eval_term term env with
    | Ok v -> Buffer.contents out ^ Value.to_string v
    | Error err -> Buffer.contents out ^ "error: " ^ Eval_error.to_string err
  in
  [ run factorial
  ; run
      (Do
         [ Call (Name "print_string", [ Lit (Ast.String "new ") ])
         ; Op ("^", Lit (Ast.String "syntax"), Lit (Ast.String "!"))
         ])
  ; run
      (Call
         ( Fn ([ "x"; "y" ], Op ("-", Name "x", Name "y"))
         , [ Lit (Ast.Int 10); Lit (Ast.Int 3) ] ))
  ; run (Op ("%", Lit (Ast.Int 1), Lit (Ast.Int 2)))
  ]
;;
