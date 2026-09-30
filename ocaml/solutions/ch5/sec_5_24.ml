(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Env = Sicp_common.Env
module M = Sicp_ch5.Sec_5_1
module Eval = Sicp_ch5.Sec_5_4
module Sec_4_1 = Sicp_ch4.Sec_4_1

let r name = M.Reg name
let dispatch = M.Goto "eval-dispatch"

(* The literal match runs its scrutinee once and then walks its cases,
   comparing the value with each literal; the selected body runs in
   tail position.  The final case is selected without a test because
   admission proved the match exhaustive. *)
let ev_cond =
  [ M.Label "ev-cond"
  ; M.Save "exp"
  ; M.Save "env"
  ; M.Save "continue"
  ; M.Assign ("continue", M.Label_ref "ev-cond-clauses")
  ; M.Assign_op ("exp", "match-scrutinee", [ r "exp" ])
  ; dispatch
  ; M.Label "ev-cond-clauses"
  ; M.Restore "continue"
  ; M.Restore "env"
  ; M.Restore "exp"
  ; M.Assign_op ("unev", "match-cases", [ r "exp" ])
  ; M.Label "ev-cond-loop"
  ; M.Test ("last-case?", [ r "unev" ])
  ; M.Branch "ev-cond-selected"
  ; M.Test ("case-selects?", [ r "unev"; r "val" ])
  ; M.Branch "ev-cond-selected"
  ; M.Assign_op ("unev", "rest-cases", [ r "unev" ])
  ; M.Goto "ev-cond-loop"
  ; M.Label "ev-cond-selected"
  ; M.Assign_op ("env", "case-environment", [ r "unev"; r "val"; r "env" ])
  ; M.Assign_op ("exp", "first-case-body", [ r "unev" ])
  ; dispatch
  ]
;;

(* A chain of [&&] (or [||]) is one operand sequence: [continue] is
   saved once for the whole chain, each operand but the last runs with
   the rest of the chain and the environment saved, and the last
   operand runs in tail position. *)
let chain name operands decided =
  let label suffix = "ev-" ^ name ^ "-chain" ^ suffix in
  [ M.Label (label "")
  ; M.Save "continue"
  ; M.Assign_op ("unev", operands, [ r "exp" ])
  ; M.Label (label "-loop")
  ; M.Assign_op ("exp", "first-operand", [ r "unev" ])
  ; M.Test ("last-operand?", [ r "unev" ])
  ; M.Branch (label "-last")
  ; M.Save "unev"
  ; M.Save "env"
  ; M.Assign ("continue", M.Label_ref (label "-decide"))
  ; dispatch
  ; M.Label (label "-decide")
  ; M.Restore "env"
  ; M.Restore "unev"
  ; M.Test (decided, [ r "val" ])
  ; M.Branch (label "-done")
  ; M.Assign_op ("unev", "rest-operands", [ r "unev" ])
  ; M.Goto (label "-loop")
  ; M.Label (label "-last")
  ; M.Restore "continue"
  ; dispatch
  ; M.Label (label "-done")
  ; M.Restore "continue"
  ; M.Goto_reg "continue"
  ]
;;

let controller =
  Sec_5_23.extend
    ~dispatch:[ "cond?", "ev-cond"; "and?", "ev-and-chain"; "or?", "ev-or-chain" ]
    ~entries:
      (List.concat
         [ ev_cond
         ; chain "and" "and-operands" "false?"
         ; chain "or" "or-operands" "true?"
         ])
;;

let arity expected ws =
  Error (Eval_error.Arity_mismatch { expected; given = List.length ws })
;;

let bad detail = Error (Eval_error.Bad_instruction detail)

let selects pattern v =
  match Ast.view_pattern pattern with
  | Ast.PScalar literal -> Value.equal_scalars (Sec_4_1.scalar_value literal) v
  | _ -> Ok true
;;

let rec flatten_and e =
  match Ast.view e with
  | Ast.And (a, b) -> a :: flatten_and b
  | _ -> [ e ]
;;

let rec flatten_or e =
  match Ast.view e with
  | Ast.Or (a, b) -> a :: flatten_or b
  | _ -> [ e ]
;;

let operands name flatten =
  ( name
  , M.Value_op
      (function
        | [ Eval.Exp e ] -> Ok (Eval.Exps (flatten e))
        | [ w ] -> bad (name ^ " of " ^ Eval.word_to_string w)
        | ws -> arity 1 ws) )
;;

let operations =
  List.filter (fun (name, _) -> name = "cond?") Sec_5_23.operations
  @ [ ( "last-case?"
      , M.Test_op
          (function
            | [ Eval.Cases cases ] -> Ok (List.length cases = 1)
            | ws -> arity 1 ws) )
    ; ( "case-selects?"
      , M.Test_op
          (function
            | [ Eval.Cases ((pattern, _) :: _); Eval.V v ] -> selects pattern v
            | ws -> arity 2 ws) )
    ; ( "case-environment"
      , M.Value_op
          (function
            | [ Eval.Cases ((pattern, _) :: _); Eval.V v; Eval.Env env ] ->
              (match Ast.view_pattern pattern with
               | Ast.PVar name -> Ok (Eval.Env (Env.bind name v env))
               | _ -> Ok (Eval.Env env))
            | ws -> arity 3 ws) )
    ; operands "and-operands" flatten_and
    ; operands "or-operands" flatten_or
    ]
;;

let run source = Sec_5_23.session ~operations ~controller source

let cost_line route (s : Sec_5_23.stats) =
  Printf.sprintf "%s: pushes = %d, instructions = %d" route s.pushes s.instructions
;;

let ex_5_24 () =
  let* answers = run Sec_5_23.classify_session in
  let* basic = Sec_5_23.statistics ~operations ~controller Sec_5_23.classify_cost in
  let* derived =
    Sec_5_23.statistics
      ~operations:Sec_5_23.operations
      ~controller:Sec_5_23.controller
      Sec_5_23.classify_cost
  in
  Ok
    (answers
     @ [ cost_line "classify 7 through ev-cond" basic
       ; cost_line "classify 7 through cond->if" derived
       ])
;;

let logic_session =
  {|let show b = if b then "true" else "false"
let within n = 0 < n && n < 10
let () = print_endline (show (1 < 2 && 2 < 3 && 3 < 4))
let () = print_endline (show (1 < 2 && 3 < 2 && 1 / 0 = 0))
let () = print_endline (show (3 < 2 || 1 < 2 || 1 / 0 = 0))
let () = print_endline (show (3 < 2 || 5 < 4))
let () = print_endline (show (3 < 2 || (1 < 2 && (5 < 4 || 2 < 3))))
let () = print_endline (show (within 5))
let () = print_endline (show (within 50))
|}
;;

let logic_cost = "let v = 1 < 2 && 2 < 3 && 3 < 4 && 4 < 5\n"

let ex_5_24a () =
  let* answers = run logic_session in
  let* chain = Sec_5_23.statistics ~operations ~controller logic_cost in
  let* nested = Sec_5_23.statistics ~controller:Eval.base_controller logic_cost in
  Ok
    (answers
     @ [ cost_line "four-operand && through the chain loop" chain
       ; cost_line "four-operand && through the base ev-and" nested
       ])
;;
