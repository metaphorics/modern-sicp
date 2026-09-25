(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.25: normal-order evaluation in the controller, based on
    the lazy evaluator of 4.2.

    The plan, followed by the code below and by [controller]:

    - Thunks are machine words -- [Thunk of expression * environment] --
      built only by the [make-thunk] operation.
    - Bindings of thunked operands go through the shared environment
      with placeholder symbols: [extend-environment] binds each
      parameter to a fresh reserved symbol and records the symbol's
      thunk in a side table; [lookup-variable-value] answers the
      recorded thunk when it finds a placeholder.  The base evaluator's
      frames never change shape, so [define-variable!] and
      [set-variable-value!] work unchanged.
    - The controller changes in exactly three places:
      1. the argument loop no longer evaluates operands; it makes a
         thunk per operand and adjoins it (nothing is saved, nothing
         recursed -- that is the laziness);
      2. [ev-variable] forces a thunked binding the first time the
         variable is read, and memoizes by storing the forced value
         over the placeholder with [set-variable-value!];
      3. [primitive-apply] forces any thunks left in [argl] before the
         primitive sees the arguments, since a primitive consumes
         values.
    - A thunked operand in tail position, or forced twice through two
      different variables, is recomputed: memoization is per binding,
      not per thunk object, because the edition's values are immutable;
      the book memoizes by mutating the thunk pair.  Every session
      below observes single-variable references, where the two agree. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4
module Ast = Sicp_common.Ast
module Value = Sicp_common.Value

(** The reserved first character a placeholder symbol starts with; no
    object-language symbol can carry it. *)
let placeholder_mark = "\000"

let is_placeholder_symbol s =
  String.length s > 0 && String.contains s placeholder_mark.[0]
;;

(** The side table: placeholder symbol to the thunk word it stands
    for.  One table per lazy machine. *)
type laziness =
  { placeholders : (string, Eval.word) Hashtbl.t
  ; counter : int ref
  }

let fresh_placeholder laziness =
  let n = !(laziness.counter) + 1 in
  laziness.counter := n;
  placeholder_mark ^ "param" ^ string_of_int n
;;

(** [lazy_operations ()] builds the exercise's operations: the base
    table with [make-thunk], the thunk selectors, and the placeholder
    [extend-environment]/[lookup-variable-value] overrides. *)
let lazy_operations () =
  let laziness = { placeholders = Hashtbl.create 16; counter = ref 0 } in
  let operations =
    [ ( "make-thunk"
      , Eval.Value_op
          (function
            | [ Eval.Exp e; Eval.Env env ] -> Ok (Eval.Thunk (e, env))
            | _ -> Error (Eval.Arity "make-thunk needs an expression and an environment"))
      )
    ; ( "thunk?"
      , Eval.Value_op
          (function
            | [ Eval.Thunk _ ] -> Ok (Eval.V (Value.bool true))
            | [ _ ] -> Ok (Eval.V (Value.bool false))
            | _ -> Error (Eval.Arity "thunk? needs one argument")) )
    ; ( "thunk-expression"
      , Eval.Value_op
          (function
            | [ Eval.Thunk (e, _) ] -> Ok (Eval.Exp e)
            | [ _ ] -> Error (Eval.Op_failed "thunk-expression needs a thunk")
            | _ -> Error (Eval.Arity "thunk-expression needs one argument")) )
    ; ( "thunk-environment"
      , Eval.Value_op
          (function
            | [ Eval.Thunk (_, env) ] -> Ok (Eval.Env env)
            | [ _ ] -> Error (Eval.Op_failed "thunk-environment needs a thunk")
            | _ -> Error (Eval.Arity "thunk-environment needs one argument")) )
    ; ( "extend-environment"
      , Eval.Value_op
          (function
            | [ Eval.Seq params; Eval.Args args; Eval.Env base ] ->
              Eval.result_all (List.map Eval.variable_name params)
              >>= fun names ->
              let bound_value arg =
                match arg with
                | Eval.V v -> Ok v
                | Eval.Thunk _ ->
                  (* a placeholder the lookup will resolve *)
                  let slot = fresh_placeholder laziness in
                  Hashtbl.replace laziness.placeholders slot arg;
                  Ok (Value.symbol slot)
                | w ->
                  Error
                    (Eval.Op_failed
                       ("a bound operand must be a value or a thunk, found "
                        ^ Eval.word_to_string w))
              in
              Eval.result_all (List.map bound_value args)
              >>= fun values ->
              (match Value.env_extend names values base with
               | Ok env -> Ok (Eval.Env env)
               | Error e -> Error (Eval.eval_error e))
            | _ ->
              Error
                (Eval.Arity
                   "extend-environment needs parameters, arguments, and an environment"))
      )
    ; ( "lookup-variable-value"
      , Eval.Value_op
          (function
            | [ Eval.Exp e; Eval.Env env ] ->
              (match Ast.view e with
               | Ast.Variable name ->
                 (match Value.env_find_binding env name with
                  | Some v ->
                    (match Value.view v with
                     | Value.Symbol s when is_placeholder_symbol s ->
                       (match Hashtbl.find_opt laziness.placeholders s with
                        | Some thunk -> Ok thunk
                        | None -> Error (Eval.Op_failed ("unbound variable: " ^ name)))
                     | _ -> Ok (Eval.V v))
                  | None -> Error (Eval.Op_failed ("unbound variable: " ^ name)))
               | _ -> Error (Eval.Op_failed "lookup-variable-value needs a variable"))
            | _ ->
              Error
                (Eval.Arity "lookup-variable-value needs a variable and an environment"))
      )
    ]
  in
  operations
;;

(** The three controller changes.  The lazy argument loop replaces the
    book's save-heavy loop: making a thunk evaluates nothing. *)
let lazy_argument_loop =
  {|ev-appl-did-operator
  (restore unev)
  (restore env)
  (assign argl (op empty-arglist))
  (assign proc (reg val))
ev-appl-operand-loop
  (test (op no-operands?) (reg unev))
  (branch (label ev-appl-args-done))
  (assign exp
          (op first-operand)
          (reg unev))
  (assign val
          (op make-thunk)
          (reg exp)
          (reg env))
  (assign argl
          (op adjoin-arg)
          (reg val)
          (reg argl))
  (assign unev
          (op rest-operands)
          (reg unev))
  (goto (label ev-appl-operand-loop))
ev-appl-args-done
  (goto (label apply-dispatch))|}
;;

(** Variable reads force and memoize: the forced value is stored over
    the placeholder, so the second read of the same variable is a
    plain value. *)
let lazy_ev_variable =
  {|ev-variable
  (assign val
          (op lookup-variable-value)
          (reg exp)
          (reg env))
  (test (op thunk?) (reg val))
  (branch (label ev-variable-thunk))
  (goto (reg continue))
ev-variable-thunk
  (save exp)
  (save env)
  (save continue)
  (assign exp (op thunk-expression) (reg val))
  (assign env (op thunk-environment) (reg val))
  (assign continue (label ev-variable-forced))
  (goto (label eval-dispatch))
ev-variable-forced
  (restore continue)
  (restore env)
  (restore exp)
  (perform
   (op set-variable-value!) (reg exp) (reg val) (reg env))
  (goto (reg continue))|}
;;

(** Primitive application forces the arguments first: values are
    rebuilt in order into [unev], then moved back to [argl]. *)
let lazy_primitive_apply =
  {|primitive-apply
  (assign unev (op empty-arglist))
force-args-loop
  (test (op no-args?) (reg argl))
  (branch (label force-args-done))
  (assign val (op first-arg) (reg argl))
  (assign argl (op rest-args) (reg argl))
  (test (op thunk?) (reg val))
  (branch (label force-args-one))
  (goto (label force-args-keep))
force-args-one
  (save proc)
  (save unev)
  (save argl)
  (save continue)
  (assign exp (op thunk-expression) (reg val))
  (assign env (op thunk-environment) (reg val))
  (assign continue (label force-args-back))
  (goto (label eval-dispatch))
force-args-back
  (restore continue)
  (restore argl)
  (restore unev)
  (restore proc)
force-args-keep
  (assign unev
          (op adjoin-arg)
          (reg val)
          (reg unev))
  (goto (label force-args-loop))
force-args-done
  (assign argl (reg unev))
  (assign val (op apply-primitive-procedure)
              (reg proc)
              (reg argl))
  (restore continue)
  (goto (reg continue))|}
;;

(** [controller ()] assembles the lazy evaluator from the base
    fragments with the three replaced blocks. *)
let controller =
  String.concat
    "\n"
    (List.filter_map
       (fun (name, text) ->
          match name with
          | "ev-variable" -> Some lazy_ev_variable
          | "ev-appl-did-operator" | "argument-loop" -> None
          | "primitive-apply" -> Some lazy_primitive_apply
          | "apply-dispatch" ->
            (* the lazy argument loop carries its own did-operator
               entry, so nothing else changes here *)
            Some text
          | _ -> Some text)
       Eval.controller_fragments
     @ [ lazy_argument_loop ])
;;

(** [run source] builds a fresh lazy machine over [source] and answers
    its transcript. *)
let run source =
  Eval.make_evaluator ~controller ~operations:(lazy_operations ()) ~source ()
  >>= fun m ->
  let ended =
    match Eval.start m with
    | Ok () -> Ok ()
    | Error e when Eval.error_to_string e = "operation failed: " ^ Eval.input_exhausted ->
      Ok ()
    | Error e -> Error e
  in
  ended >>= fun () -> Ok (Eval.transcript m)
;;

let lazy_factorial_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))
(factorial 5)|}
;;

(** [ex_5_25 ()] runs the lazy evaluator and answers three transcript
    groups: the factorial session (whose answer must still be the
    strict evaluator's [120]), the laziness proof -- a procedure whose
    argument is never used never evaluates it, so [(always-42 (car
    '()))] answers [42] where the strict evaluator would crash -- and
    the memoization proof: [(use-twice (bump))] answers [(1 1)] and
    [count] reads [1], so the thunked [(bump)] was evaluated once, not
    once per reference. *)
let ex_5_25 () =
  run lazy_factorial_source
  >>= fun factorial_lines ->
  run
    {|
(define (always-42 ignored) 42)
(always-42 (car '()))|}
  >>= fun lazy_lines ->
  run
    {|
(define count 0)
(define (bump) (set! count (+ count 1)) count)
(define (use-twice x) (list x x))
(use-twice (bump))
count|}
  >>= fun memo_lines -> Ok (factorial_lines @ lazy_lines @ memo_lines)
;;
