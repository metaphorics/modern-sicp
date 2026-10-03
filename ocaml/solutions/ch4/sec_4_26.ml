(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Core = Sicp_ch4.Sec_4_1

let is_unless operator =
  match Ast.view operator with
  | Ast.Var "unless" -> true
  | _ -> false
;;

(* Ben's rewrite, run before evaluation: the program's definition of
   [unless] declares the syntax to the pinned checker and is consumed
   here instead of binding a value, and every saturated
   [unless c u e] becomes [if c then e else u]. *)
let rec lower e =
  match Ast.view e with
  | Ast.Let (false, [ { Ast.name = Some "unless"; _ } ], body) -> lower body
  | Ast.Apply (operator, [ condition; usual; exceptional ]) when is_unless operator ->
    Ast.if_ ~at:(Ast.at e) (lower condition) (lower exceptional) (lower usual)
  | _ -> Ast.map_children lower e
;;

let run_special source =
  match Core.expression source with
  | Error rejection -> "rejected: " ^ rejection
  | Ok e ->
    let out = Buffer.create 64 in
    let env = Core.the_global_environment ~emit:(Buffer.add_string out) () in
    (match Core.eval_expr (lower e) env with
     | Ok _ -> Buffer.contents out
     | Error err -> Buffer.contents out ^ "error: " ^ Eval_error.to_string err)
;;

let run_lazy source =
  Core.transcript ~experiment:Check.Lazy Sicp_ch4.Sec_4_2.run ("let () =\n" ^ source)
;;

let unless_definition =
  "let unless condition usual_value exceptional_value =\n\
  \  if condition then exceptional_value else usual_value\n\
   in\n"
;;

let armed_call =
  unless_definition ^ "print_int (unless (1 = 1) (1 / 0) 42);\n print_newline ()"
;;

let value_use =
  unless_definition
  ^ "let triple c u e = (c, u, e) in\n\
     let rec map3 f l =\n\
    \  match l with\n\
    \  | [] -> []\n\
    \  | (c, u, e) :: rest -> f c u e :: map3 f rest\n\
     in\n\
     let rec print_all l =\n\
    \  match l with\n\
    \  | [] -> ()\n\
    \  | x :: rest ->\n\
    \    print_int x;\n\
    \    print_string \" \";\n\
    \    print_all rest\n\
     in\n\
     print_all (map3 unless [ triple true (1 / 0) 42; triple false 7 (1 / 0) ]);\n\
     print_newline ()"
;;

let ex_4_26 () =
  [ run_special armed_call
  ; run_lazy armed_call
  ; run_lazy value_use
  ; run_special value_use
  ]
;;
