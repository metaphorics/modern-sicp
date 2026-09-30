(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module M = Sicp_ch5.Sec_5_1
module Eval = Sicp_ch5.Sec_5_4

type controller = Eval.word M.instruction list
type operations = (string * Eval.word M.op) list

(* {1 The section's session harness} *)

let admit source =
  match Check.check ~filename:"session.ml" source with
  | Ok program -> Ok program
  | Error d ->
    Error (Eval_error.Invalid_form ("rejected: " ^ Check.diagnostic_to_string d))
;;

let lines_of text =
  match List.rev (String.split_on_char '\n' text) with
  | "" :: rest -> List.rev rest
  | lines -> List.rev lines
;;

let session ?operations ~controller source =
  let* program = admit source in
  let out = Buffer.create 64 in
  let* ev =
    Eval.make_evaluator ?operations ~controller ~emit:(Buffer.add_string out) ()
  in
  let* _ = Eval.run_program ev program in
  Ok (lines_of (Buffer.contents out))
;;

type stats =
  { pushes : int
  ; depth : int
  ; instructions : int
  }

let statistics ?operations ~controller source =
  let* program = admit source in
  let* ev = Eval.make_evaluator ?operations ~controller ~emit:ignore () in
  let* _ = Eval.run_program ev program in
  let m = Eval.machine ev in
  let pushes, depth = M.stack_statistics m in
  Ok { pushes; depth; instructions = M.executed m }
;;

let extend ~dispatch ~entries =
  let tests =
    List.concat_map
      (fun (test, label) -> [ M.Test (test, [ M.Reg "exp" ]); M.Branch label ])
      dispatch
  in
  List.concat_map
    (fun (name, fragment) ->
       match name with
       | "eval-dispatch" ->
         let rest =
           List.filter
             (function
               | M.Label "eval-dispatch" -> false
               | _ -> true)
             fragment
         in
         (M.Label "eval-dispatch" :: tests) @ rest
       | "done" -> entries @ fragment
       | _ -> fragment)
    Eval.controller_fragments
;;

let splice ~from ~until replacement controller =
  let is label = function
    | M.Label l -> l = label
    | _ -> false
  in
  match List.find_index (is from) controller, List.find_index (is until) controller with
  | Some i, Some j when i < j ->
    List.filteri (fun k _ -> k < i) controller
    @ replacement
    @ List.filteri (fun k _ -> k >= j) controller
  | _ -> invalid_arg ("splice: label " ^ from ^ " does not precede label " ^ until)
;;

(* {1 The derived forms} *)

let scrutinee_name = "cond scrutinee"

let literal_case (pattern, _) =
  match Ast.view_pattern pattern with
  | Ast.PScalar _ | Ast.PWildcard | Ast.PVar _ -> true
  | _ -> false
;;

let is_cond e =
  match Ast.view e with
  | Ast.Match (_, cases) -> List.for_all literal_case cases
  | _ -> false
;;

(* The case bodies see the scrutinee through [scrutinee_name], a name no
   guest identifier can spell, so no body variable is captured. *)
let rec clauses_to_if = function
  | [] -> Error (Eval_error.Invalid_form "cond->if needs a case")
  | [ (pattern, body) ] -> selected pattern body
  | (pattern, body) :: rest ->
    (match Ast.view_pattern pattern with
     | Ast.PScalar literal ->
       let* alternative = clauses_to_if rest in
       Ok
         (Ast.if_
            (Ast.compare_ Ast.Eq (Ast.var scrutinee_name) (Ast.scalar literal))
            body
            alternative)
     | _ -> selected pattern body)

and selected pattern body =
  match Ast.view_pattern pattern with
  | Ast.PVar name -> Ok (Ast.apply (Ast.fun_ [ name ] body) [ Ast.var scrutinee_name ])
  | _ -> Ok body
;;

let cond_to_if e =
  match Ast.view e with
  | Ast.Match (scrutinee, cases) when List.for_all literal_case cases ->
    let* chain = clauses_to_if cases in
    Ok (Ast.apply (Ast.fun_ [ scrutinee_name ] chain) [ scrutinee ])
  | _ -> Error (Eval_error.Invalid_form "cond->if needs a literal match")
;;

let let_to_combination e =
  match Ast.view e with
  | Ast.Let (false, bindings, body) ->
    let parameter i (b : Ast.binding) =
      match b.name with
      | Some name -> name
      | None -> Printf.sprintf "let discarded %d" i
    in
    let parameters = List.mapi parameter bindings in
    Ok
      (Ast.apply
         (Ast.fun_ parameters body)
         (List.map (fun (b : Ast.binding) -> b.rhs) bindings))
  | _ -> Error (Eval_error.Invalid_form "let->combination needs a parallel let")
;;

let transformer name f =
  ( name
  , M.Value_op
      (function
        | [ Eval.Exp e ] -> Result.map (fun e -> Eval.Exp e) (f e)
        | ws -> Error (Eval_error.Arity_mismatch { expected = 1; given = List.length ws }))
  )
;;

let operations =
  [ ( "cond?"
    , M.Test_op
        (function
          | [ Eval.Exp e ] -> Ok (is_cond e)
          | ws ->
            Error (Eval_error.Arity_mismatch { expected = 1; given = List.length ws })) )
  ; transformer "cond->if" cond_to_if
  ; transformer "let->combination" let_to_combination
  ]
;;

let controller =
  extend
    ~dispatch:[ "cond?", "ev-cond"; "let?", "ev-derived-let" ]
    ~entries:
      [ M.Label "ev-cond"
      ; M.Assign_op ("exp", "cond->if", [ M.Reg "exp" ])
      ; M.Goto "eval-dispatch"
      ; M.Label "ev-derived-let"
      ; M.Assign_op ("exp", "let->combination", [ M.Reg "exp" ])
      ; M.Goto "eval-dispatch"
      ]
;;

let run source = session ~operations ~controller source

let classify_session =
  {|let classify n = match n with 0 -> "zero" | 1 -> "one" | _ -> "many"
let () = print_endline (classify 0)
let () = print_endline (classify 1)
let () = print_endline (classify 7)
let () = print_endline (match 3 < 2 with true -> "yes" | false -> "no")
let () = print_endline (match 7 * 2 with 0 -> "zero" | m -> string_of_int m)
let () = print_endline (string_of_int (let a = 2 and b = 3 in a * b))
|}
;;

let classify_cost =
  {|let classify n = match n with 0 -> "zero" | 1 -> "one" | _ -> "many"
let v = classify 7
|}
;;

let ex_5_23 () =
  let* answers = run classify_session in
  let* derived = statistics ~operations ~controller classify_cost in
  let* basic = statistics ~controller:Eval.base_controller classify_cost in
  Ok
    (answers
     @ [ Printf.sprintf
           "classify 7 through cond->if: pushes = %d, instructions = %d"
           derived.pushes
           derived.instructions
       ; Printf.sprintf
           "classify 7 through ev-match: pushes = %d, instructions = %d"
           basic.pushes
           basic.instructions
       ])
;;
