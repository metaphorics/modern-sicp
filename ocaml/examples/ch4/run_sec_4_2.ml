(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

let ( >>= ) = Result.bind

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let show_lazy env text expected = Replay.expect (show (Lazy_eval.run env text)) expected

(* The applicative-order contrast of 4.2.1: the 4.1 dispatch applied
   over the section's primitive table, where every operand is evaluated
   before the call, so the same try expression dies in the division the
   lazy evaluator never runs. *)
module rec Strict : sig
  val eval : Strict_eval.eval_t
end = struct
  module C = Strict_eval.Core (Strict)
  module Ast = Sicp_common.Ast

  let apply_procedure proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Lazy_eval.primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure cv ->
      Strict_eval.extend_environment cv.parameters args cv.env
      >>= fun extended -> C.eval_sequence cv.body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> Strict_eval.lookup_variable_value name env
    | Ast.Quote datum -> Ok (Strict_eval.datum_to_value datum)
    | Ast.Definition d ->
      (match Ast.view_definition d with
       | Ast.Define_variable (name, e) ->
         Strict.eval e env >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Strict.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If (predicate, consequent, alternative) ->
      Strict.eval predicate env
      >>= fun tested ->
      if Lazy_eval.true_ tested
      then Strict.eval consequent env
      else (
        match alternative with
        | Some branch -> Strict.eval branch env
        | None -> Ok (Value.bool false))
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Strict_eval.cond_to_if exp >>= fun rewritten -> Strict.eval rewritten env
    | Ast.Application (operator, operands) ->
      Strict.eval operator env
      >>= fun proc ->
      C.list_of_values operands env >>= fun args -> apply_procedure proc args
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let strict_env () =
  let env = Strict_eval.setup_environment () in
  List.iter
    (fun (name, f) -> Value.env_define env name (Value.primitive ~name f))
    [ "+", List.assoc "+" Lazy_eval.primitive_table
    ; "-", List.assoc "-" Lazy_eval.primitive_table
    ; "*", List.assoc "*" Lazy_eval.primitive_table
    ; "/", List.assoc "/" Lazy_eval.primitive_table
    ];
  env
;;

let show_strict env text expected =
  let parsed = Sicp_common.Reader.read text in
  let outcome =
    match parsed with
    | Ok exp -> show (Strict.eval exp env)
    | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
  in
  Replay.expect outcome expected
;;

(* The 4.2.3 lazy-list program: pairs as procedures, so cons is
   non-strict without a special form. *)
let lazy_lists =
  "(define (cons x y) (lambda (m) (m x y))) (define (car z) (z (lambda (p q) p))) \
   (define (cdr z) (z (lambda (p q) q))) (define (list-ref items n) (if (= n 0) (car \
   items) (list-ref (cdr items) (- n 1)))) (define (map proc items) (if (null? items) \
   '() (cons (proc (car items)) (map proc (cdr items))))) (define (scale-list items \
   factor) (map (lambda (x) (* x factor)) items)) (define (add-lists list1 list2) (cond \
   ((null? list1) list2) ((null? list2) list1) (else (cons (+ (car list1) (car list2)) \
   (add-lists (cdr list1) (cdr list2)))))) (define ones (cons 1 ones)) (define integers \
   (cons 1 (add-lists ones integers))) (define (integral integrand initial-value dt) \
   (define int (cons initial-value (add-lists (scale-list integrand dt) int))) int) \
   (define (solve f y0 dt) (define y (integral dy y0 dt)) (define dy (map f y)) y)"
;;

let () =
  (* 4.2.1: the try expression errors under applicative order and
     answers 1 under lazy evaluation. *)
  let strict = strict_env () in
  show_strict strict "(define (try a b) (if (= a 0) 1 b))" "ok";
  show_strict strict "(try 0 (/ 1 0))" "Error: division by zero";
  let lazy_env = Lazy_eval.the_global_environment () in
  show_lazy lazy_env "(define (try a b) (if (= a 0) 1 b))" "ok";
  show_lazy lazy_env "(try 0 (/ 1 0))" "1";
  (* 4.2.1: unless as a procedure, armed with a division in the
     unchosen arm. *)
  show_strict
    strict
    "(define (unless condition usual-value exceptional-value) (if condition \
     exceptional-value usual-value))"
    "ok";
  show_strict
    strict
    "(unless (= 0 0) (/ 1 0) (begin (display \"exception: returning 0\") 0))"
    "Error: division by zero";
  show_lazy
    lazy_env
    "(define (unless condition usual-value exceptional-value) (if condition \
     exceptional-value usual-value))"
    "ok";
  print_endline ";;; L-Eval: the display of the chosen arm prints:";
  show_lazy
    lazy_env
    "(unless (= 0 0) (/ 1 0) (begin (display \"exception: returning 0\") 0))"
    "0";
  (* 4.2.2: a delayed value reaching the driver is forced before it
     prints (exercise 4.27's sequence). *)
  show_lazy lazy_env "(define count 0)" "ok";
  show_lazy lazy_env "(define (id x) (set! count (+ count 1)) x)" "ok";
  show_lazy lazy_env "(define w (id (id 10)))" "ok";
  show_lazy lazy_env "count" "1";
  show_lazy lazy_env "w" "10";
  show_lazy lazy_env "count" "2";
  (* 4.2.3: the lazy lists of the text, infinite by construction. *)
  let lists = Lazy_eval.the_global_environment () in
  ignore (Lazy_eval.run_program lists lazy_lists);
  show_lazy lists "(list-ref integers 17)" "18";
  show_lazy lists "(car (add-lists ones ones))" "2";
  show_lazy lists "(list-ref (solve (lambda (x) x) 1 0.001) 1000)" "2.716923932235896"
;;
