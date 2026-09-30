(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

let ( let* ) = Result.bind

module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module M = Sicp_ch5.Sec_5_1
module Eval = Sicp_ch5.Sec_5_4

let r name = M.Reg name
let dispatch = M.Goto "eval-dispatch"

(* A variable read forces a delayed binding through the controller and
   memoizes the result in the thunk cell.  [force-val] is also the
   entry [primitive-apply] uses for each delayed argument. *)
let lazy_ev_variable =
  [ M.Label "ev-variable"
  ; M.Assign_op ("val", "lookup-variable-value", [ r "exp"; r "env" ])
  ; M.Label "force-val"
  ; M.Test ("delayed?", [ r "val" ])
  ; M.Branch "force-delayed"
  ; M.Assign_op ("val", "thunk-result", [ r "val" ])
  ; M.Goto_reg "continue"
  ; M.Label "force-delayed"
  ; M.Save "continue"
  ; M.Save "val"
  ; M.Assign_op ("env", "thunk-environment", [ r "val" ])
  ; M.Assign_op ("exp", "thunk-expression", [ r "val" ])
  ; M.Assign ("continue", M.Label_ref "force-memoize")
  ; dispatch
  ; M.Label "force-memoize"
  ; M.Restore "unev"
  ; M.Perform ("memoize-thunk", [ r "unev"; r "val" ])
  ; M.Restore "continue"
  ; M.Goto_reg "continue"
  ]
;;

(* Operands are delayed, not evaluated: no save, no dispatch. *)
let lazy_argument_loop =
  [ M.Label "ev-appl-did-operator"
  ; M.Restore "unev"
  ; M.Restore "env"
  ; M.Assign ("argl", M.Const (Eval.Args []))
  ; M.Assign ("proc", r "val")
  ; M.Label "ev-appl-delay-loop"
  ; M.Test ("no-operands?", [ r "unev" ])
  ; M.Branch "apply-dispatch"
  ; M.Assign_op ("val", "delay-operand", [ r "unev"; r "env" ])
  ; M.Assign_op ("argl", "adjoin-arg", [ r "val"; r "argl" ])
  ; M.Assign_op ("unev", "rest-operands", [ r "unev" ])
  ; M.Goto "ev-appl-delay-loop"
  ]
;;

(* Primitives are strict: every delayed argument is forced, one at a
   time, before the primitive sees the list. *)
let lazy_primitive_apply =
  [ M.Label "primitive-apply"
  ; M.Test ("arguments-delayed?", [ r "argl" ])
  ; M.Branch "primitive-force-argument"
  ; M.Assign_op ("argl", "settle-arguments", [ r "argl" ])
  ; M.Assign_op ("val", "apply-primitive-procedure", [ r "proc"; r "argl" ])
  ; M.Assign_op ("argl", "primitive-excess-arguments", [ r "proc"; r "argl" ])
  ; M.Restore "continue"
  ; M.Goto "apply-excess"
  ; M.Label "primitive-force-argument"
  ; M.Save "proc"
  ; M.Save "argl"
  ; M.Assign_op ("val", "first-delayed-argument", [ r "argl" ])
  ; M.Assign ("continue", M.Label_ref "primitive-forced")
  ; M.Goto "force-val"
  ; M.Label "primitive-forced"
  ; M.Restore "argl"
  ; M.Restore "proc"
  ; M.Assign_op ("argl", "settle-first-argument", [ r "argl"; r "val" ])
  ; M.Goto "primitive-apply"
  ]
;;

let controller =
  Eval.base_controller
  |> Sec_5_23.splice ~from:"ev-variable" ~until:"ev-lambda" lazy_ev_variable
  |> Sec_5_23.splice ~from:"ev-appl-did-operator" ~until:"apply-entry" lazy_argument_loop
  |> Sec_5_23.splice ~from:"primitive-apply" ~until:"compound-apply" lazy_primitive_apply
;;

let arity expected ws =
  Error (Eval_error.Arity_mismatch { expected; given = List.length ws })
;;

let delayed v =
  match Value.thunk_state_of v with
  | Some { contents = Value.Delayed (e, env) } -> Some (e, env)
  | _ -> None
;;

let settled v =
  match Value.thunk_state_of v with
  | Some { contents = Value.Forced v } -> v
  | _ -> v
;;

let is_delayed v = Option.is_some (delayed v)

let thunk_part name part =
  ( name
  , M.Value_op
      (function
        | [ Eval.V v ] ->
          (match delayed v with
           | Some d -> Ok (part d)
           | None ->
             Error (Eval_error.Bad_instruction (name ^ " of a value that is not delayed")))
        | ws -> arity 1 ws) )
;;

let operations =
  [ ( "delay-operand"
    , M.Value_op
        (function
          | [ Eval.Exps (e :: _); Eval.Env env ] -> Ok (Eval.V (Value.thunk ~expr:e ~env))
          | ws -> arity 2 ws) )
  ; ( "delayed?"
    , M.Test_op
        (function
          | [ Eval.V v ] -> Ok (is_delayed v)
          | [ _ ] -> Ok false
          | ws -> arity 1 ws) )
  ; thunk_part "thunk-expression" (fun (e, _) -> Eval.Exp e)
  ; thunk_part "thunk-environment" (fun (_, env) -> Eval.Env env)
  ; ( "thunk-result"
    , M.Value_op
        (function
          | [ Eval.V v ] -> Ok (Eval.V (settled v))
          | [ w ] -> Ok w
          | ws -> arity 1 ws) )
  ; ( "memoize-thunk"
    , M.Action_op
        (function
          | [ Eval.V thunk; Eval.V v ] ->
            (match Value.thunk_state_of thunk with
             | Some cell ->
               Value.set_thunk_state cell (Value.Forced v);
               Ok ()
             | None ->
               Error
                 (Eval_error.Bad_instruction
                    "memoize-thunk of a value that is not a thunk"))
          | ws -> arity 2 ws) )
  ; ( "arguments-delayed?"
    , M.Test_op
        (function
          | [ Eval.Args vs ] -> Ok (List.exists is_delayed vs)
          | ws -> arity 1 ws) )
  ; ( "first-delayed-argument"
    , M.Value_op
        (function
          | [ Eval.Args vs ] ->
            (match List.find_opt is_delayed vs with
             | Some v -> Ok (Eval.V v)
             | None -> Error (Eval_error.Bad_instruction "no delayed argument"))
          | ws -> arity 1 ws) )
  ; ( "settle-first-argument"
    , M.Value_op
        (function
          | [ Eval.Args vs; Eval.V v ] ->
            let rec replace seen = function
              | [] -> Ok (Eval.Args vs)
              | head :: rest ->
                if is_delayed head
                then Ok (Eval.Args (List.rev_append seen (settled v :: rest)))
                else replace (head :: seen) rest
            in
            replace [] vs
          | ws -> arity 2 ws) )
  ; ( "settle-arguments"
    , M.Value_op
        (function
          | [ Eval.Args vs ] -> Ok (Eval.Args (List.map settled vs))
          | ws -> arity 1 ws) )
  ]
;;

(* The same operations with memoization disabled: a thunk is forced
   again at every reference. *)
let without_memoization =
  List.map
    (fun (name, op) ->
       if name = "memoize-thunk"
       then
         ( name
         , M.Action_op
             (function
               | [ _; _ ] -> Ok ()
               | ws -> arity 2 ws) )
       else name, op)
    operations
;;

let run source = Sec_5_23.session ~operations ~controller source

let factorial_session =
  {|let rec factorial n = if n = 1 then 1 else n * factorial (n - 1)
let () = print_endline (string_of_int (factorial 5))
|}
;;

let unused_session =
  {|let always_42 _ = 42
let () = print_endline (string_of_int (always_42 (1 / 0)))
|}
;;

let memo_session =
  {|let count = ref 0
let bump step = count := !count + step; !count
let use_twice x = (x, x)
let () = match use_twice (bump 1) with (a, b) -> print_endline (string_of_int a ^ " " ^ string_of_int b)
let () = print_endline (string_of_int !count)
|}
;;

let ex_5_25 () =
  let* factorial = run factorial_session in
  let* unused = run unused_session in
  let* memo = run memo_session in
  let* unmemoized =
    Sec_5_23.session ~operations:without_memoization ~controller memo_session
  in
  let strict =
    match Sec_5_23.session ~controller:Eval.base_controller unused_session with
    | Ok lines -> "strict evaluator: " ^ String.concat " " lines
    | Error e -> "strict evaluator: error: " ^ Eval_error.to_string e
  in
  Ok (factorial @ unused @ [ strict ] @ memo @ ("without memoization:" :: unmemoized))
;;
