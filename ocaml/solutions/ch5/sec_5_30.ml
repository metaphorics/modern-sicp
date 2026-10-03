(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module M = Sicp_ch5.Sec_5_1
module Eval = Sicp_ch5.Sec_5_4

(* A constructor no guest type can declare: guest constructors start
   with a capital letter. *)
let condition_tag = "#condition"

(* A signaled interaction does not finish its program: the report is
   already on the output, and [run] goes on to the next interaction. *)
let signaled = "the interaction was signaled"

let condition_detail = function
  | Eval.V v ->
    (match Value.view v with
     | Value.Constructor (tag, [ detail ]) when tag = condition_tag ->
       (match Value.view detail with
        | Value.String s -> Some s
        | _ -> None)
     | _ -> None)
  | _ -> None
;;

let r name = M.Reg name

let checked_primitive_apply =
  [ M.Label "primitive-apply"
  ; M.Assign_op ("val", "apply-primitive-procedure", [ r "proc"; r "argl" ])
  ; M.Test ("condition?", [ r "val" ])
  ; M.Branch "signal-error"
  ; M.Assign_op ("argl", "primitive-excess-arguments", [ r "proc"; r "argl" ])
  ; M.Restore "continue"
  ; M.Goto "apply-excess"
  ]
;;

let checked_binary_apply =
  [ M.Label "ev-binary-apply"
  ; M.Restore "argl"
  ; M.Restore "exp"
  ; M.Restore "continue"
  ; M.Assign_op ("val", "apply-binary", [ r "exp"; r "argl"; r "val" ])
  ; M.Test ("condition?", [ r "val" ])
  ; M.Branch "signal-error"
  ; M.Goto_reg "continue"
  ]
;;

let signal_error = [ M.Label "signal-error"; M.Perform ("signal-error", [ r "val" ]) ]

let controller =
  Sec_5_23.extend ~dispatch:[] ~entries:signal_error
  |> Sec_5_23.splice ~from:"ev-binary-apply" ~until:"ev-collect" checked_binary_apply
  |> Sec_5_23.splice
       ~from:"primitive-apply"
       ~until:"compound-apply"
       checked_primitive_apply
;;

(* Only the runtime failures of a checked program become condition
   codes; a failure of the machine itself still stops it. *)
let caught = function
  | Eval_error.Division_by_zero
  | Eval_error.Bounds_error _
  | Eval_error.Type_error _
  | Eval_error.User_error _ -> true
  | _ -> false
;;

let with_condition = function
  | Error e when caught e ->
    Ok (Eval.V (Value.construct condition_tag [ Value.string (Eval_error.to_string e) ]))
  | outcome -> outcome
;;

(* A primitive's guest callback runs on the base evaluator: a callback
   failure escapes as an error of the primitive, so the primitive's
   application becomes the condition on the signaling machine. *)
let guest_apply ~emit proc vs =
  let* ev = Eval.make_evaluator ~controller:Eval.base_controller ~emit () in
  let names = List.mapi (fun i _ -> Printf.sprintf "argument %d" i) vs in
  let env = Env.extend (("procedure", proc) :: List.combine names vs) Env.empty in
  Eval.eval
    ev
    env
    (Ast.apply (Ast.var "procedure") (List.map (fun name -> Ast.var name) names))
;;

let arity expected ws =
  Error (Eval_error.Arity_mismatch { expected; given = List.length ws })
;;

let operations ~emit =
  let base = Eval.operation_table ~apply:(guest_apply ~emit) in
  let checked name =
    match List.assoc_opt name base with
    | Some (M.Value_op f) -> Some (name, M.Value_op (fun ws -> with_condition (f ws)))
    | _ -> None
  in
  List.filter_map checked [ "apply-primitive-procedure"; "apply-binary" ]
  @ [ ( "condition?"
      , M.Test_op
          (function
            | [ w ] -> Ok (Option.is_some (condition_detail w))
            | ws -> arity 1 ws) )
    ; ( "signal-error"
      , M.Action_op
          (function
            | [ w ] ->
              (match condition_detail w with
               | Some detail ->
                 emit ("error: " ^ detail ^ "\n");
                 Error (Eval_error.Invalid_form signaled)
               | None ->
                 Error (Eval_error.Bad_instruction "signal-error without a condition"))
            | ws -> arity 1 ws) )
    ]
;;

let interactions =
  [ "let a = Array.make 3 0\nlet () = print_endline (string_of_int (Array.get a 5))\n"
  ; "let () = print_endline (string_of_int (10 / 0))\n"
  ; "let () = print_endline (string_of_int (7 mod 0))\n"
  ; "let () = print_endline (string_of_int (List.length (List.map (fun x -> 10 / x) [ 1; \
     0 ])))\n"
  ; "let v = 10 / 0\nlet () = print_endline \"after\"\n"
  ; "let rec factorial n = if n = 1 then 1 else factorial (n - 1) * n\n\
     let () = print_endline (string_of_int (factorial 5))\n"
  ]
;;

let rejected =
  [ "let () = print_endline (string_of_int no_such_variable)\n"
  ; "let f x y = x + y\nlet () = print_endline (string_of_int (f 1))\n"
  ]
;;

let is_signaled = function
  | Eval_error.Invalid_form detail -> detail = signaled
  | _ -> false
;;

let run sources =
  let out = Buffer.create 64 in
  let emit = Buffer.add_string out in
  let* ev = Eval.make_evaluator ~operations:(operations ~emit) ~controller ~emit () in
  let* () =
    List.fold_left
      (fun acc source ->
         let* () = acc in
         let* program = Sec_5_23.admit source in
         match Eval.run_program ev program with
         | Ok _ -> Ok ()
         | Error e when is_signaled e -> Ok ()
         | Error e -> Error e)
      (Ok ())
      sources
  in
  Ok (Sec_5_23.lines_of (Buffer.contents out))
;;

(* The saved words a failed call leaves on the stack: the driver did
   not restore them, because the interaction stopped mid-call. *)
let depth_after_failure () =
  let* ev =
    Eval.make_evaluator ~operations:(operations ~emit:ignore) ~controller ~emit:ignore ()
  in
  let* program = Sec_5_23.admit (List.nth interactions 1) in
  let* () =
    match Eval.run_program ev program with
    | Ok _ -> Error (Eval_error.Invalid_form "the failing interaction did not fail")
    | Error e when is_signaled e -> Ok ()
    | Error e -> Error e
  in
  Ok (M.stack_depth (Eval.machine ev))
;;

let admission source =
  match Check.check ~filename:"session.ml" source with
  | Ok _ -> "admitted"
  | Error d -> "rejected before evaluation: " ^ Check.kind_to_string d.Check.kind
;;

let ex_5_30 () =
  let* caught = run interactions in
  let* depth = depth_after_failure () in
  let base =
    match Sec_5_23.session ~controller:Eval.base_controller (List.nth interactions 1) with
    | Ok lines -> "base evaluator: " ^ String.concat " " lines
    | Error e -> "base evaluator stops: " ^ Eval_error.to_string e
  in
  Ok
    (caught
     @ [ Printf.sprintf "saved frames left by the failed interaction: %d" depth; base ]
     @ List.map admission rejected)
;;
