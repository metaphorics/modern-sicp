(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.32: chapter 3 streams versus the lazier lazy lists. The
    text's procedural [cons] delays the car as well as the cdr, so the
    elements of a lazy list are produced only when something forces
    them -- never, for a skipped element. The demonstration takes the
    second element of lists whose other arms are armed with a division
    by zero: under the lazy evaluator the armed elements stay thunks and
    the answer comes out, while the strict evaluator's eager [car] --
    the chapter 3 [cons-stream] behavior at construction time -- dies in
    the armed arm before the list is even finished. *)

let render = function
  | Ok v -> Sicp_common.Value.to_string v
  | Error e -> "Error: " ^ Sicp_common.Eval_error.to_string e
;;

let ( >>= ) = Result.bind

let run_lazy env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Sicp_ch4.Sec_4_2.actual_value exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

(** The strict global environment with the section's arithmetic
    installed, so the armed operands evaluate. *)
let strict_env () =
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  List.iter
    (fun (name, f) ->
       Sicp_common.Value.env_define env name (Sicp_common.Value.primitive ~name f))
    [ "+", List.assoc "+" Sicp_ch4.Sec_4_2.primitive_table
    ; "-", List.assoc "-" Sicp_ch4.Sec_4_2.primitive_table
    ; "*", List.assoc "*" Sicp_ch4.Sec_4_2.primitive_table
    ; "/", List.assoc "/" Sicp_ch4.Sec_4_2.primitive_table
    ];
  env
;;

(** The applicative-order evaluator over the section's primitive table:
    the host of the chapter 3 style [cons-stream], whose [car] is
    evaluated at construction time. *)
module rec Strict : sig
  val eval : Sicp_ch4.Sec_4_1.eval_t
end = struct
  module C = Sicp_ch4.Sec_4_1.Core (Strict)
  module Ast = Sicp_common.Ast
  module Eval_error = Sicp_common.Eval_error
  module Value = Sicp_common.Value

  let apply_procedure proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Sicp_ch4.Sec_4_2.primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure cv ->
      Sicp_ch4.Sec_4_1.extend_environment cv.parameters args cv.env
      >>= fun extended -> C.eval_sequence cv.body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> Sicp_ch4.Sec_4_1.lookup_variable_value name env
    | Ast.Quote datum -> Ok (Sicp_ch4.Sec_4_1.datum_to_value datum)
    | Ast.Definition d ->
      (match Ast.view_definition d with
       | Ast.Define_variable (name, e) ->
         Strict.eval e env
         >>= fun value -> Sicp_ch4.Sec_4_1.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Sicp_ch4.Sec_4_1.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Strict.eval e env
      >>= fun value ->
      Sicp_ch4.Sec_4_1.set_variable_value_ name value env
      >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If (predicate, consequent, alternative) ->
      Strict.eval predicate env
      >>= fun tested ->
      if Sicp_ch4.Sec_4_2.true_ tested
      then Strict.eval consequent env
      else (
        match alternative with
        | Some branch -> Strict.eval branch env
        | None -> Ok (Value.bool false))
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Sicp_ch4.Sec_4_1.cond_to_if exp >>= fun rewritten -> Strict.eval rewritten env
    | Ast.Application (operator, operands) ->
      Strict.eval operator env
      >>= fun proc ->
      C.list_of_values operands env >>= fun args -> apply_procedure proc args
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let run_strict env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Strict.eval exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let lazy_pairs =
  "(define (cons x y) (lambda (m) (m x y))) (define (car z) (z (lambda (p q) p))) \
   (define (cdr z) (z (lambda (p q) q)))"
;;

(** [ex_4_32 ()] shows the extra laziness: skipping an armed first
    element works under the lazy lists; the same construction with the
    chapter 3 eager car errors at once; and the armed tail of a
    two-element lazy list is skipped just as freely. *)
let ex_4_32 () =
  let lazy_env = Sicp_ch4.Sec_4_2.the_global_environment () in
  let strict = strict_env () in
  let (_ : (Sicp_common.Value.t, Sicp_common.Eval_error.t) result) =
    Sicp_ch4.Sec_4_2.run_program lazy_env lazy_pairs
  in
  let (_ : (Sicp_common.Value.t, Sicp_common.Eval_error.t) result) =
    Sicp_ch4.Sec_4_1.run_program strict "(define (cons-stream a b) (cons a b))"
  in
  let r1 = run_lazy lazy_env "(car (cdr (cons (/ 1 0) (cons 42 '()))))" in
  let r2 = run_strict strict "(car (cdr (cons-stream (/ 1 0) (cons-stream 42 '()))))" in
  let r3 = run_lazy lazy_env "(car (cdr (cons (/ 1 0) (cons 7 (/ 1 0)))))" in
  [ r1; r2; r3 ]
;;
