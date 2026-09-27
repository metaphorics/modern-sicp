(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.30: errors signaled inside the evaluator.

    (a) An unbound variable lookup answers a distinguished condition
    code -- a pair tagged with a reserved symbol no user symbol can
    spell -- and [ev-variable] tests for it and goes to
    [signal-error].  (b) Every primitive application is checked:
    [apply-primitive-procedure] answers the condition code when the
    primitive refuses the application (wrong operand count, [car] of a
    non-pair, division by zero), and [primitive-apply] tests for it.
    Both paths land in the base controller's [signal-error], which
    stops the machine with the object-level message. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4
module Ast = Sicp_common.Ast
module Value = Sicp_common.Value

(** The reserved tag character no object symbol can carry. *)
let condition_mark = "\000"

let condition_word tag detail =
  Eval.V (Value.pair (Value.symbol (condition_mark ^ tag)) (Value.string detail))
;;

let is_condition_word tag w =
  match w with
  | Eval.V v ->
    (match Value.view v with
     | Value.Pair (s, _) ->
       (match Value.view s with
        | Value.Symbol t -> t = condition_mark ^ tag
        | _ -> false)
     | _ -> false)
  | _ -> false
;;

(** [unwrap_condition w] is the detail a condition-code word carries;
    the overridden [signal-error] reports the detail, never the
    reserved tag a user symbol cannot spell. *)
let unwrap_condition w =
  match w with
  | Eval.V v ->
    (match Value.view v with
     | Value.Pair (_, rest) ->
       (match Value.view rest with
        | Value.String detail -> detail
        | _ -> Eval.word_to_string w)
     | _ -> Eval.word_to_string w)
  | w -> Eval.word_to_string w
;;

(** The exercise's operations: the checking lookup and the checking
    primitive application, plus the two condition-code tests. *)
let operations =
  [ ( "lookup-variable-value"
    , Eval.Value_op
        (function
          | [ Eval.Exp e; Eval.Env env ] ->
            (match Ast.view e with
             | Ast.Variable name ->
               (match Value.env_find_binding env name with
                | Some v -> Ok (Eval.V v)
                | None ->
                  Ok (condition_word "unbound-variable" ("unbound variable: " ^ name)))
             | _ -> Error (Eval.Op_failed "lookup-variable-value needs a variable"))
          | _ ->
            Error (Eval.Arity "lookup-variable-value needs a variable and an environment"))
    )
  ; ( "variable-lookup-failed?"
    , Eval.Value_op
        (function
          | [ w ] -> Ok (Eval.V (Value.bool (is_condition_word "unbound-variable" w)))
          | _ -> Error (Eval.Arity "variable-lookup-failed? needs one argument")) )
  ; ( "apply-primitive-procedure"
    , Eval.Value_op
        (function
          | [ Eval.V v; Eval.Args ws ] ->
            (match Value.view v with
             | Value.Primitive_procedure name ->
               Eval.word_values ws
               >>= fun vs ->
               (match Eval.apply_object_primitive name vs with
                | Ok r -> Ok (Eval.V r)
                | Error (Eval.Op_failed detail) ->
                  Ok (condition_word "primitive-failure" detail)
                | Error other ->
                  Ok (condition_word "primitive-failure" (Eval.error_to_string other)))
             | _ ->
               Error
                 (Eval.Op_failed "apply-primitive-procedure needs a primitive procedure"))
          | _ ->
            Error
              (Eval.Op_failed
                 "apply-primitive-procedure needs a procedure and an operand list")) )
  ; ( "primitive-application-failed?"
    , Eval.Value_op
        (function
          | [ w ] -> Ok (Eval.V (Value.bool (is_condition_word "primitive-failure" w)))
          | _ -> Error (Eval.Arity "primitive-application-failed? needs one argument")) )
  ; ( "signal-error"
    , Eval.Action_op
        (function
          | [ w ] -> Error (Eval.Op_failed (unwrap_condition w))
          | _ -> Error (Eval.Arity "signal-error needs one argument")) )
  ]
;;

(** [ev_variable_checking] tests the lookup's condition code before
    continuing. *)
let ev_variable_checking =
  {|ev-variable
  (assign val
          (op lookup-variable-value)
          (reg exp)
          (reg env))
  (test (op variable-lookup-failed?) (reg val))
  (branch (label variable-lookup-failed))
  (goto (reg continue))
variable-lookup-failed
  (goto (label signal-error))|}
;;

(** [primitive_apply_checking] tests the application's condition code
    before restoring [continue]. *)
let primitive_apply_checking =
  {|primitive-apply
  (assign val (op apply-primitive-procedure)
              (reg proc)
              (reg argl))
  (test (op primitive-application-failed?) (reg val))
  (branch (label primitive-application-failed))
  (restore continue)
  (goto (reg continue))
primitive-application-failed
  (goto (label signal-error))|}
;;

(** The exercise's controller: the base controller with the two
    checking entries. *)
let controller =
  String.concat
    "\n"
    (List.filter_map
       (fun (name, text) ->
          match name with
          | "ev-variable" -> Some ev_variable_checking
          | "primitive-apply" -> Some primitive_apply_checking
          | _ -> Some text)
       Eval.controller_fragments)
;;

let run source =
  Eval.make_evaluator ~controller ~operations ~source ()
  >>= fun m ->
  let note =
    match Eval.start m with
    | Ok () -> "end of input"
    | Error e when Eval.error_to_string e = "operation failed: " ^ Eval.input_exhausted ->
      "end of input"
    | Error e -> Eval.error_to_string e
  in
  Ok (note :: Eval.transcript m)
;;

(** [ex_5_30 ()] runs the checking evaluator over the caught failures
    and one clean computation: an unbound variable, [car] of a symbol,
    division by zero, a wrong operand count, and a factorial that must
    still answer [120]. *)
let ex_5_30 () =
  run "(car 5)"
  >>= fun car_failure ->
  run "(/ 1 0)"
  >>= fun division_failure ->
  run "(+ 1 no-such-variable)"
  >>= fun unbound_failure ->
  run "(remainder 7)"
  >>= fun arity_failure ->
  run
    {|
(define (factorial n)
  (if (= n 1)
      1
      (* n (factorial (- n 1)))))
(factorial 5)|}
  >>= fun clean ->
  Ok (car_failure @ division_failure @ unbound_failure @ arity_failure @ clean)
;;
