(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind
let r name = M.Reg name

let rec from_label label = function
  | [] -> []
  | M.Label l :: _ as rest when l = label -> rest
  | _ :: rest -> from_label label rest
;;

let base_application = List.assoc "ev-application" W.controller_fragments

let ev_application_fast =
  [ M.Label "ev-application"
  ; M.Save "continue"
  ; M.Test ("symbol-operator?", [ r "exp" ])
  ; M.Branch "ev-appl-symbol-operator"
  ; M.Save "env"
  ; M.Assign_op ("unev", "operands", [ r "exp" ])
  ; M.Save "unev"
  ; M.Assign_op ("exp", "operator", [ r "exp" ])
  ; M.Assign ("continue", M.Label_ref "ev-appl-did-operator")
  ; M.Goto "eval-dispatch"
  ; M.Label "ev-appl-symbol-operator"
  ; M.Assign_op ("unev", "operands", [ r "exp" ])
  ; M.Assign_op ("exp", "operator", [ r "exp" ])
  ; M.Assign_op ("val", "lookup-variable-value", [ r "exp"; r "env" ])
  ; M.Goto "ev-appl-have-operator"
  ; M.Label "ev-appl-did-operator"
  ; M.Restore "unev"
  ; M.Restore "env"
  ; M.Label "ev-appl-have-operator"
  ; M.Assign ("argl", M.Const (W.Args []))
  ; M.Assign ("proc", r "val")
  ; M.Test ("no-operands?", [ r "unev" ])
  ; M.Branch "apply-dispatch"
  ; M.Save "proc"
  ]
  @ from_label "ev-appl-operand-loop" base_application
;;

let controller =
  List.concat_map
    (fun (name, fragment) ->
       if name = "ev-application" then ev_application_fast else fragment)
    W.controller_fragments
;;

let symbol_operator =
  ( "symbol-operator?"
  , M.Test_op
      (function
        | [ W.Exp e ] ->
          Ok
            (match Ast.view e with
             | Ast.Apply (f, _) ->
               (match Ast.view f with
                | Ast.Var _ -> true
                | _ -> false)
             | _ -> false)
        | [ w ] ->
          Error (Eval_error.Bad_instruction ("symbol-operator? of " ^ W.word_to_string w))
        | ws -> Error (Eval_error.Arity_mismatch { expected = 1; given = List.length ws }))
  )
;;

let measure controller source =
  let* p = Sec_5_33.program ~filename:"ex_5_32.ml" source in
  let* ev =
    W.make_evaluator ~operations:[ symbol_operator ] ~controller ~emit:ignore ()
  in
  let m = W.machine ev in
  M.initialize_stack m;
  let* v = W.run_program ev p in
  let pushes, depth = M.stack_statistics m in
  Ok (v, pushes, depth)
;;

let programs =
  [ "factorial 5", Sec_5_33.factorial ^ "\nlet result = factorial 5\n"
  ; "(fun y -> y + 1) 41", "let result = (fun y -> y + 1) 41\n"
  ]
;;

let ex_5_32 () =
  List.fold_right
    (fun (name, source) acc ->
       let* lines = acc in
       let* base_value, base_pushes, base_depth = measure W.base_controller source in
       let* fast_value, fast_pushes, fast_depth = measure controller source in
       Ok
         (Printf.sprintf
            "%s: base answers %s with %d pushes, depth %d; fast path answers %s with %d \
             pushes, depth %d"
            name
            (Value.to_string base_value)
            base_pushes
            base_depth
            (Value.to_string fast_value)
            fast_pushes
            fast_depth
          :: lines))
    programs
    (Ok [])
;;
