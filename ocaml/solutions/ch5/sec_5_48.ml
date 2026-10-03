(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind

type command =
  | Evaluate
  | Compile_and_run

type session =
  { state : C.state
  ; blocks : (string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list) list
  ; evaluator : W.evaluator
  ; env : Env.t
  ; transcript : string list
  }

let bound_names = function
  | Ast.Type_item _ -> []
  | Ast.Value_item (_, bindings) ->
    List.filter_map (fun (b : Ast.binding) -> b.name) bindings
;;

let describe env item value =
  match bound_names item with
  | [] -> "- = " ^ Value.to_string value
  | names ->
    String.concat
      "; "
      (List.map
         (fun name ->
            let shown = Option.fold ~none:"?" ~some:Value.to_string (Env.find env name) in
            name ^ " = " ^ shown)
         names)
;;

let compile_and_run ~emit s item =
  let label = Printf.sprintf "block-%d" (List.length s.blocks) in
  let block = label, Sec_5_45.block_statements (C.compile_program s.state [ item ]) in
  let blocks = s.blocks @ [ block ] in
  let* evaluator = Sec_5_45.make_evaluator ~emit blocks in
  let* v, env = Sec_5_45.run_block evaluator s.env label in
  Ok
    { s with
      blocks
    ; evaluator
    ; env
    ; transcript = s.transcript @ [ "compile-and-run: " ^ describe env item v ]
    }
;;

let evaluate s item =
  let* v, env = Sec_5_45.eval_item s.evaluator s.env item in
  Ok { s with env; transcript = s.transcript @ [ "evaluate: " ^ describe env item v ] }
;;

let run_session ~emit commands =
  let* evaluator = Sec_5_45.make_evaluator ~emit [] in
  let start =
    { state = C.new_state ()
    ; blocks = []
    ; evaluator
    ; env = Sec_5_45.global_environment ~emit
    ; transcript = []
    }
  in
  let* s =
    List.fold_left
      (fun acc (command, item) ->
         let* s = acc in
         match command with
         | Evaluate -> evaluate s item
         | Compile_and_run -> compile_and_run ~emit s item)
      (Ok start)
      commands
  in
  Ok s.transcript
;;

let source =
  Sec_5_33.factorial
  ^ "\nlet double x = x + x\nlet result = double (factorial 5) - factorial 5\n"
;;

let ex_5_48 () =
  let* p = Sec_5_33.program ~filename:"ex_5_48.ml" source in
  match Check.items p with
  | [ factorial; double; result ] ->
    run_session
      ~emit:ignore
      [ Compile_and_run, factorial; Evaluate, double; Evaluate, result ]
  | _ -> Error (Eval_error.Invalid_form "three items")
;;
