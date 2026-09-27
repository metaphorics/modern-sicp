(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.23: derived expressions -- [cond] and [let] enter the
    evaluator through transformer machine operations. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4
module Ast = Sicp_common.Ast

(** The dispatch grows two tests, one per derived form, before the
    application test; each entry transforms [exp] and re-enters
    [eval-dispatch]. *)
let dispatch_with_derived =
  {|
  (test (op cond?) (reg exp))
  (branch (label ev-cond))
  (test (op let?) (reg exp))
  (branch (label ev-let))|}
;;

(** The two transformer entries: the book's cheat -- [cond->if] and
    [let->combination] are machine operations. *)
let ev_derived =
  {|ev-cond
  (assign exp (op cond->if) (reg exp))
  (goto (label eval-dispatch))
ev-let
  (assign exp (op let->combination) (reg exp))
  (goto (label eval-dispatch))|}
;;

(** The exercise's controller: the base fragments with the dispatch
    replaced and the transformer entries appended. *)
let controller =
  let plain = List.assoc "eval-dispatch" Eval.controller_fragments in
  (* the base dispatch minus its final goto, the derived-form tests,
     then the goto *)
  let core =
    let lines = String.split_on_char '\n' plain in
    String.concat
      "\n"
      (List.filter
         (fun l ->
            not (String.starts_with ~prefix:"  (goto (label unknown-expression-type))" l))
         lines)
  in
  String.concat
    "\n"
    (List.filter_map
       (fun (name, text) ->
          match name with
          | "eval-dispatch" ->
            Some
              (core ^ dispatch_with_derived ^ "\n  (goto (label unknown-expression-type))")
          | "errors" -> Some (ev_derived ^ "\n" ^ text)
          | _ -> Some text)
       Eval.controller_fragments)
;;

(** [cond_to_if clauses else_body] is the book's [cond->if]: a chain
    of [if]s ending in the else body, in [false] when there is none,
    and with a bodyless clause's value its test. *)
let rec cond_to_if clauses else_body =
  match clauses with
  | [] ->
    (match else_body with
     | Some body -> Ast.sequence body
     | None -> Ok (Ast.bool false))
  | (test, body) :: rest ->
    let alternative = cond_to_if rest else_body in
    let consequent =
      match body with
      | [] -> Ok test
      | exprs -> Ast.sequence exprs
    in
    alternative
    >>= fun alternative ->
    consequent >>= fun consequent -> Ok (Ast.if_ test consequent (Some alternative))
;;

(** [let_to_combination] is the book's [let->combination]: the body as
    a lambda over the binding names, applied to the binding
    initializers. *)
let let_to_combination e =
  match Ast.view e with
  | Ast.Let (bindings, body) ->
    let names = List.map fst bindings in
    let inits = List.map snd bindings in
    Ast.lambda names body >>= fun lam -> Ok (Ast.application lam inits)
  | _ -> Error (Sicp_common.Eval_error.Invalid_form "let->combination needs a let")
;;

(** The exercise's operations: the two syntax tests and the two
    transformers. *)
let operations =
  [ ( "cond?"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            (match Ast.view e with
             | Ast.Cond _ -> Ok (Eval.V (Sicp_common.Value.bool true))
             | _ -> Ok (Eval.V (Sicp_common.Value.bool false)))
          | _ -> Error (Eval.Arity "cond? needs one argument")) )
  ; ( "let?"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            (match Ast.view e with
             | Ast.Let _ -> Ok (Eval.V (Sicp_common.Value.bool true))
             | _ -> Ok (Eval.V (Sicp_common.Value.bool false)))
          | _ -> Error (Eval.Arity "let? needs one argument")) )
  ; ( "cond->if"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            (match Ast.view e with
             | Ast.Cond (clauses, else_body) ->
               Eval.expr_word (cond_to_if clauses else_body)
             | _ -> Error (Eval.Op_failed "cond->if needs a cond"))
          | _ -> Error (Eval.Arity "cond->if needs one argument")) )
  ; ( "let->combination"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] -> Eval.expr_word (let_to_combination e)
          | _ -> Error (Eval.Arity "let->combination needs one argument")) )
  ]
;;

let run source =
  Eval.make_evaluator ~controller ~operations ~source ()
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

(** [ex_5_23 ()] runs [cond] and [let] sessions through the extended
    evaluator: a three-clause classify with an [else], a bodyless
    clause, and a [let] whose body is a lambda application. *)
let ex_5_23 () =
  run
    {|
(define (classify n)
  (cond ((= n 0) 'zero)
        ((= n 1) 'one)
        (else 'many)))
(classify 0)
(classify 1)
(classify 7)
(cond ((= 1 2)))
(let ((a 2) (b 3)) (* a b))|}
;;
