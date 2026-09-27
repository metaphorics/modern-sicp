(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.33: quoted lists become true lazy lists. Once the 4.2.3
    definitions shadow the pair primitives, a quoted list is still an
    ordinary pair and Ben's [(car '(a b c))] applies a procedural [car]
    to it -- not applicable. The fix lifts the [Quote] clause: a quoted
    proper list is rebuilt as the application of the environment's own
    [cons] to the quoted elements and the quoted empty tail, so the list
    the driver hands out is the same lazy structure the program builds;
    any other datum stays ordinary data. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

(** [quote_lazy datum env] evaluates the [cons] spine of the quoted
    list through the current environment. *)
let rec build datum env =
  match datum with
  | Ast.DNil -> Ok (Ast.quote Ast.DNil)
  | Ast.DPair (car, cdr) ->
    build cdr env
    >>= fun tail -> Ok (Ast.application (Ast.variable "cons") [ Ast.quote car; tail ])
  | d -> Ok (Ast.quote d)
;;

module rec Quoted : sig
  val eval : Lazy_eval.eval_t
end = struct
  module C = Lazy_eval.Core (Quoted)

  let eval exp env =
    match Ast.view exp with
    | Ast.Quote datum ->
      (match datum with
       | Ast.DPair _ -> build datum env >>= fun rebuilt -> Quoted.eval rebuilt env
       | _ -> Ok (Strict_eval.datum_to_value datum))
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> Strict_eval.lookup_variable_value name env
    | Ast.Definition d ->
      (match Ast.view_definition d with
       | Ast.Define_variable (name, e) ->
         Quoted.eval e env >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Quoted.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If _ -> C.eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Strict_eval.cond_to_if exp >>= fun rewritten -> Quoted.eval rewritten env
    | Ast.Application (operator, operands) ->
      C.actual_value operator env >>= fun proc -> C.apply_procedure proc operands env
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [run eval env text] is the modified driver step: the value of the
    form, forced before printing. *)
let run eval env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (eval exp env >>= Lazy_eval.force_value)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let lazy_pairs =
  "(define (cons x y) (lambda (m) (m x y))) (define (car z) (z (lambda (p q) p))) \
   (define (cdr z) (z (lambda (p q) q))) (define (list-ref items n) (if (= n 0) (car \
   items) (list-ref (cdr items) (- n 1))))"
;;

(** [ex_4_33 ()] runs Ben's test: under the section evaluator with the
    4.2.3 definitions installed the quoted list is ordinary data and
    [car] goes not-applicable; under the lifted quote the same
    expression answers [a], the second element answers through the lazy
    spine, and [list-ref] walks the quoted lazy list. *)
let ex_4_33 () =
  let plain = Lazy_eval.the_global_environment () in
  let quoted = Lazy_eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Lazy_eval.run_program quoted lazy_pairs in
  let (_ : (Value.t, Eval_error.t) result) = Lazy_eval.run_program plain lazy_pairs in
  [ run Lazy_eval.eval plain "(car '(a b c))"
  ; run Quoted.eval quoted "(car '(a b c))"
  ; run Quoted.eval quoted "(car (cdr '(a b c)))"
  ; run Quoted.eval quoted "(list-ref '(a b c d e) 3)"
  ]
;;
