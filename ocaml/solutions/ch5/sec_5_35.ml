(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

let arith_symbol = function
  | Ast.Add -> "+"
  | Ast.Sub -> "-"
  | Ast.Mul -> "*"
  | Ast.Div -> "/"
  | Ast.Rem -> "mod"
  | Ast.Addf -> "+."
  | Ast.Subf -> "-."
  | Ast.Mulf -> "*."
  | Ast.Divf -> "/."
;;

let comparison_symbol = function
  | Ast.Eq -> "="
  | Ast.Ne -> "<>"
  | Ast.Lt -> "<"
  | Ast.Le -> "<="
  | Ast.Gt -> ">"
  | Ast.Ge -> ">="
;;

let binding_name (b : Ast.binding) = Option.value b.name ~default:"_"

let rec pattern_to_string p =
  match Ast.view_pattern p with
  | Ast.PWildcard -> "_"
  | Ast.PVar x -> x
  | Ast.PScalar s -> Value.to_string (Sicp_ch4.Sec_4_1.scalar_value s)
  | Ast.PTuple ps -> "(" ^ String.concat ", " (List.map pattern_to_string ps) ^ ")"
  | Ast.PConstruct (c, []) -> c
  | Ast.PConstruct (c, ps) ->
    c ^ " (" ^ String.concat ", " (List.map pattern_to_string ps) ^ ")"
  | Ast.PNil -> "[]"
  | Ast.PCons (h, t) -> pattern_to_string h ^ " :: " ^ pattern_to_string t
;;

let descriptor e =
  match Ast.view e with
  | Ast.Var x -> x
  | Ast.Scalar s -> Value.to_string (Sicp_ch4.Sec_4_1.scalar_value s)
  | Ast.Fun (parameters, _) -> "fun " ^ String.concat " " parameters
  | Ast.Arith (op, _, _) -> "(" ^ arith_symbol op ^ ")"
  | Ast.Compare (op, _, _) -> "(" ^ comparison_symbol op ^ ")"
  | Ast.Concat _ -> "(^)"
  | Ast.Cons _ -> "(::)"
  | Ast.Assign _ -> "(:=)"
  | Ast.Not _ -> "not"
  | Ast.Neg _ -> "(~-)"
  | Ast.Deref _ -> "(!)"
  | Ast.Make_ref _ -> "ref"
  | Ast.Field (_, f) -> "." ^ f
  | Ast.Tuple parts -> Printf.sprintf "tuple/%d" (List.length parts)
  | Ast.Construct (c, _) -> c
  | Ast.Record fields -> "{" ^ String.concat "; " (List.map fst fields) ^ "}"
  | Ast.Let (false, bindings, _) ->
    "let " ^ String.concat " and " (List.map binding_name bindings)
  | Ast.Let (true, bindings, _) ->
    "let rec " ^ String.concat " and " (List.map binding_name bindings)
  | Ast.Apply _ | Ast.If _ | Ast.Match _ | Ast.Sequence _ | Ast.And _ | Ast.Or _ | Ast.Nil
    -> "<expression>"
;;

let word_to_string = function
  | W.Exp e -> descriptor e
  | W.Pat p -> pattern_to_string p
  | w -> W.word_to_string w
;;

let statement_to_string i = M.instruction_to_string word_to_string i
let listing (seq : C.seq) = List.map statement_to_string seq.statements
let answer = "let f x = x + g (x + 2)"

let figure =
  [ "Assign_op (\"val\", \"make-compiled-procedure\", [Label_ref \"entry1\"; Const (fun \
     x); Reg \"env\"])"
  ; "Goto \"after-lambda2\""
  ; "Label \"entry1\""
  ; "Assign_op (\"env\", \"compiled-procedure-bind\", [Reg \"proc\"; Reg \"argl\"])"
  ; "Save \"continue\""
  ; "Assign_op (\"arg1\", \"lookup-variable-value\", [Const (x); Reg \"env\"])"
  ; "Save \"arg1\""
  ; "Assign_op (\"proc\", \"lookup-variable-value\", [Const (g); Reg \"env\"])"
  ; "Assign (\"argl\", Const ([]))"
  ; "Assign_op (\"arg1\", \"lookup-variable-value\", [Const (x); Reg \"env\"])"
  ; "Assign (\"arg2\", Const (2))"
  ; "Assign_op (\"val\", \"apply-binary\", [Const ((+)); Reg \"arg1\"; Reg \"arg2\"])"
  ; "Assign_op (\"argl\", \"adjoin-arg\", [Reg \"val\"; Reg \"argl\"])"
  ; "Test (\"primitive-exact?\", [Reg \"proc\"; Reg \"argl\"])"
  ; "Branch \"primitive-branch3\""
  ; "Label \"compiled-branch4\""
  ; "Assign (\"continue\", Label_ref \"proc-return6\")"
  ; "Goto \"compiled-apply\""
  ; "Label \"proc-return6\""
  ; "Assign (\"arg2\", Reg \"val\")"
  ; "Goto \"after-call5\""
  ; "Label \"primitive-branch3\""
  ; "Assign_op (\"arg2\", \"apply-primitive-procedure\", [Reg \"proc\"; Reg \"argl\"])"
  ; "Goto \"after-call5\""
  ; "Label \"after-call5\""
  ; "Restore \"arg1\""
  ; "Assign_op (\"val\", \"apply-binary\", [Const ((+)); Reg \"arg1\"; Reg \"arg2\"])"
  ; "Restore \"continue\""
  ; "Goto_reg \"continue\""
  ; "Label \"after-lambda2\""
  ]
;;

let ex_5_35 () =
  let* program =
    Sec_5_33.program ~filename:"ex_5_35.ml" ("let g y = y * 3\n" ^ answer ^ "\n")
  in
  match Check.items program with
  | [ _; Ast.Value_item (false, [ binding ]) ] ->
    let compiled = listing (C.compile (C.new_state ()) binding.rhs "val" C.Next) in
    Ok
      ([ "compiled to the figure: " ^ answer ]
       @ compiled
       @ [ Printf.sprintf "figure matches: %b" (compiled = figure) ])
  | _ -> Error (Sicp_common.Eval_error.Invalid_form "two top-level bindings")
;;
