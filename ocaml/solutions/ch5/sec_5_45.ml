(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude
module Value = Sicp_common.Value
module C = Sicp_ch5.Sec_5_5
module M = Sicp_ch5.Sec_5_1
module W = Sicp_ch5.Sec_5_4

let ( let* ) = Result.bind
let bad detail = Error (Eval_error.Bad_instruction detail)

(* {1 The 5.5.7 interface} *)

let runtime =
  let s = C.compile_program (C.new_state ()) [] in
  List.filter
    (function
      | M.Goto "done" | M.Label "done" -> false
      | _ -> true)
    s.statements
;;

let block_statements (s : C.seq) =
  let keep = List.length s.statements - List.length runtime - 2 in
  List.filteri (fun i _ -> i < keep) s.statements
;;

let apply_dispatch =
  List.concat_map
    (function
      | M.Branch "compound-apply" as branch ->
        [ branch
        ; M.Test ("compiled-procedure?", [ M.Reg "proc" ])
        ; M.Branch "compiled-from-evaluator"
        ]
      | i -> [ i ])
    (List.assoc "apply-dispatch" W.controller_fragments)
  @ [ M.Label "compiled-from-evaluator"; M.Restore "continue"; M.Goto "compiled-apply" ]
;;

let controller ?(runtime = runtime) blocks =
  List.concat_map
    (fun (name, fragment) ->
       match name with
       | "done" -> []
       | "apply-dispatch" -> apply_dispatch
       | _ -> fragment)
    W.controller_fragments
  @ List.concat_map
      (fun (label, code) -> (M.Label label :: code) @ [ M.Goto "done" ])
      blocks
  @ runtime
  @ [ M.Label "done" ]
;;

let no_callbacks _ _ =
  Error (Eval_error.Invalid_form "compiled-procedure operations call no guest procedure")
;;

let compiled_operations =
  List.filter
    (fun (name, _) -> not (List.mem name W.base_operation_names))
    (C.runtime_operations ~apply:no_callbacks)
;;

let make_evaluator ?runtime ~emit blocks =
  W.make_evaluator
    ~operations:compiled_operations
    ~registers:[ "arg1"; "arg2" ]
    ~controller:(controller ?runtime blocks)
    ~emit
    ()
;;

let value_of what = function
  | W.V v -> Ok v
  | w -> bad (what ^ " holds " ^ W.word_to_string w)
;;

let run_block ev env label =
  let m = W.machine ev in
  M.restart m;
  let* () = M.set_register m "env" (W.Env env) in
  let* () = M.goto_label m label in
  let* () = M.start m in
  let* v = Result.bind (M.get_register m "val") (value_of "val") in
  let* env =
    match M.get_register m "env" with
    | Ok (W.Env env) -> Ok env
    | Ok w -> bad ("env holds " ^ W.word_to_string w)
    | Error e -> Error e
  in
  Ok (v, env)
;;

let named bindings values =
  List.filter_map
    (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
    (List.combine bindings values)
;;

let eval_item ev env = function
  | Ast.Type_item _ -> Ok (Value.unit, env)
  | Ast.Value_item (false, bindings) ->
    let* values =
      List.fold_left
        (fun acc (b : Ast.binding) ->
           let* vs = acc in
           let* v = W.eval ev env b.rhs in
           Ok (v :: vs))
        (Ok [])
        bindings
    in
    let values = List.rev values in
    let last =
      match List.rev values with
      | v :: _ -> v
      | [] -> Value.unit
    in
    Ok (last, Env.extend (named bindings values) env)
  | Ast.Value_item (true, bindings) ->
    let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
    let env, cells = Env.extend_recursive names env in
    let* last =
      List.fold_left
        (fun acc ((b : Ast.binding), cell) ->
           let* _ = acc in
           let* v = W.eval ev env b.rhs in
           Env.fill cell v;
           Ok v)
        (Ok Value.unit)
        (List.combine bindings cells)
    in
    Ok (last, env)
;;

let global_environment ~emit = Prelude.initial_env ~emit ()

(* {1 The measurements} *)

let interpreted source =
  let* p = Sec_5_33.program ~filename:"ex_5_45.ml" source in
  W.stack_statistics_after p
;;

let compiled source =
  let* p = Sec_5_33.program ~filename:"ex_5_45.ml" source in
  let state = C.new_state () in
  let blocks =
    List.mapi
      (fun i item ->
         Printf.sprintf "item-%d" i, block_statements (C.compile_program state [ item ]))
      (Check.items p)
  in
  let* ev = make_evaluator ~emit:ignore blocks in
  let* _, stats =
    List.fold_left
      (fun acc (label, _) ->
         let* env, _ = acc in
         let* _, env = run_block ev env label in
         Ok (env, M.stack_statistics (W.machine ev)))
      (Ok (global_environment ~emit:ignore, (0, 0)))
      blocks
  in
  Ok stats
;;

let special controller registers n =
  let* m = M.make_machine ~registers ~operations:M.arith_operations ~controller in
  let* () = M.set_register m "n" (M.Int n) in
  let* () = M.start m in
  Ok (M.stack_statistics m)
;;

let factorial_machine =
  [ M.Assign ("continue", M.Label_ref "fact-done")
  ; M.Label "fact-loop"
  ; M.Test ("=", [ M.Reg "n"; M.Const (M.Int 1) ])
  ; M.Branch "base-case"
  ; M.Save "continue"
  ; M.Save "n"
  ; M.Assign_op ("n", "-", [ M.Reg "n"; M.Const (M.Int 1) ])
  ; M.Assign ("continue", M.Label_ref "after-fact")
  ; M.Goto "fact-loop"
  ; M.Label "after-fact"
  ; M.Restore "n"
  ; M.Restore "continue"
  ; M.Assign_op ("val", "*", [ M.Reg "n"; M.Reg "val" ])
  ; M.Goto_reg "continue"
  ; M.Label "base-case"
  ; M.Assign ("val", M.Const (M.Int 1))
  ; M.Goto_reg "continue"
  ; M.Label "fact-done"
  ]
;;

let ratio a b = if b = 0 then 0.0 else float_of_int a /. float_of_int b

let comparison ~definition ~call ~machine n =
  let source = definition ^ Printf.sprintf "\nlet result = %s %d\n" call n in
  let* ip, id = interpreted source in
  let* cp, cd = compiled source in
  let* sp, sd = special machine [ "n"; "val"; "continue" ] n in
  Ok
    (Printf.sprintf
       "n = %d: interpreted %d/%d, compiled %d/%d, special %d/%d; ratios compiled \
        %.3f/%.3f, special %.3f/%.3f"
       n
       ip
       id
       cp
       cd
       sp
       sd
       (ratio cp ip)
       (ratio cd id)
       (ratio sp ip)
       (ratio sd id))
;;

let ex_5_45 () =
  List.fold_right
    (fun n acc ->
       let* lines = acc in
       let* line =
         comparison
           ~definition:Sec_5_33.factorial
           ~call:"factorial"
           ~machine:factorial_machine
           n
       in
       Ok (line :: lines))
    [ 5; 10 ]
    (Ok [])
;;
