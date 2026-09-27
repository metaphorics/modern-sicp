(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.24: [cond] as a basic controller form -- not reduced to
    [if]; and Exercise 5.24a, added by this edition: [and] and [or] as
    basic controller forms in the same style. *)

let ( >>= ) = Result.bind

module Eval = Sicp_ch5.Sec_5_4
module Ast = Sicp_common.Ast
module Value = Sicp_common.Value

(** The dispatch gains three tests, before the application test: one
    for [cond] (5.24) and one each for [and] and [or] (5.24a). *)
let derived_dispatch_tests =
  {|
  (test (op cond?) (reg exp))
  (branch (label ev-cond))
  (test (op and?) (reg exp))
  (branch (label ev-and))
  (test (op or?) (reg exp))
  (branch (label ev-or))|}
;;

(** [compose extra_dispatch extra_entries] concatenates the base
    fragments with [extra_dispatch]'s tests spliced into the dispatch
    and [extra_entries] appended before the error entries. *)
let compose ~extra_dispatch_tests ~extra_entries =
  let plain = List.assoc "eval-dispatch" Eval.controller_fragments in
  let lines = String.split_on_char '\n' plain in
  let core =
    List.filter
      (fun l ->
         not (String.starts_with ~prefix:"  (goto (label unknown-expression-type))" l))
      lines
    |> String.concat "\n"
  in
  String.concat
    "\n"
    (List.filter_map
       (fun (name, text) ->
          match name with
          | "eval-dispatch" ->
            Some
              (core ^ extra_dispatch_tests ^ "\n  (goto (label unknown-expression-type))")
          | "errors" -> Some (extra_entries ^ "\n" ^ text)
          | _ -> Some text)
       Eval.controller_fragments)
;;

(** The 5.24 cond loop: walk the clauses, test each predicate until one
    is true or the clause is an [else], then evaluate the clause's
    actions with [ev-sequence].  The clause under test rides in
    [proc], whose live value is stack-saved by any enclosing argument
    loop.  A selected clause with no actions returns the predicate's
    value, the book's answer for [(cond (p))].  No clause true answers
    [false], as [cond->if] would. *)
let ev_cond =
  {|ev-cond
  (save continue)
  (assign unev (op cond-clauses) (reg exp))
ev-cond-loop
  (test (op no-clauses?) (reg unev))
  (branch (label ev-cond-no-true-clause))
  (assign val (op first-clause) (reg unev))
  (assign unev (op rest-clauses) (reg unev))
  (test (op cond-else-clause?) (reg val))
  (branch (label ev-cond-else))
  (assign exp (op cond-predicate) (reg val))
  (save val)
  (save unev)
  (save env)
  (save continue)
  (assign continue (label ev-cond-decide))
  (goto (label eval-dispatch))
ev-cond-decide
  (restore continue)
  (restore env)
  (restore unev)
  (restore proc)
  (test (op true?) (reg val))
  (branch (label ev-cond-selected))
  (goto (label ev-cond-loop))
ev-cond-selected
  (assign unev (op cond-actions) (reg proc))
  (goto (label ev-cond-actions))
ev-cond-else
  (assign unev (op cond-actions) (reg val))
  (goto (label ev-cond-actions))
ev-cond-actions
  (test (op no-more-exps?) (reg unev))
  (branch (label ev-cond-empty-actions))
  (goto (label ev-sequence))
ev-cond-empty-actions
  (restore continue)
  (goto (reg continue))
ev-cond-no-true-clause
  (restore continue)
  (assign val (const #f))
  (goto (reg continue))|}
;;

(** The 5.24a [and] loop: evaluate the operands until one is [false]
    -- the answer -- or the last one, whose value is the answer in tail
    position.  [(and)] is [true]. *)
let ev_and =
  {|ev-and
  (save continue)
  (assign unev (op and-operands) (reg exp))
  (test (op no-more-exps?) (reg unev))
  (branch (label ev-and-empty))
  (goto (label ev-and-inner))
ev-and-inner
  (assign exp (op first-exp) (reg unev))
  (test (op last-exp?) (reg unev))
  (branch (label ev-and-last))
  (save unev)
  (save env)
  (assign continue (label ev-and-inner-continue))
  (goto (label eval-dispatch))
ev-and-inner-continue
  (restore env)
  (restore unev)
  (assign unev (op rest-exps) (reg unev))
  (test (op true?) (reg val))
  (branch (label ev-and-inner))
  (restore continue)
  (assign val (const #f))
  (goto (reg continue))
ev-and-last
  (restore continue)
  (goto (label eval-dispatch))
ev-and-empty
  (restore continue)
  (assign val (const #t))
  (goto (reg continue))|}
;;

(** The 5.24a [or] loop: evaluate the operands until one is true --
    its value is the answer -- or the last one, whose value is the
    answer in tail position.  [(or)] is [false]. *)
let ev_or =
  {|ev-or
  (save continue)
  (assign unev (op or-operands) (reg exp))
  (test (op no-more-exps?) (reg unev))
  (branch (label ev-or-empty))
  (goto (label ev-or-inner))
ev-or-inner
  (assign exp (op first-exp) (reg unev))
  (test (op last-exp?) (reg unev))
  (branch (label ev-or-last))
  (save unev)
  (save env)
  (assign continue (label ev-or-inner-continue))
  (goto (label eval-dispatch))
ev-or-inner-continue
  (restore env)
  (restore unev)
  (assign unev (op rest-exps) (reg unev))
  (test (op true?) (reg val))
  (branch (label ev-or-return))
  (goto (label ev-or-inner))
ev-or-return
  (restore continue)
  (goto (reg continue))
ev-or-last
  (restore continue)
  (goto (label eval-dispatch))
ev-or-empty
  (restore continue)
  (assign val (const #f))
  (goto (reg continue))|}
;;

(** The exercise's controller: cond, and, or, all as basic forms. *)
let controller =
  compose
    ~extra_dispatch_tests:derived_dispatch_tests
    ~extra_entries:(ev_cond ^ "\n" ^ ev_and ^ "\n" ^ ev_or)
;;

(** The clause and operand operations the basic forms need.  A
    [cond-clauses] word is a sequence of [Clause] words: one per
    normal clause and, when present, the trailing else body. *)
let clause_operations =
  [ ( "cond?"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            let is_cond =
              match Ast.view e with
              | Ast.Cond _ -> true
              | _ -> false
            in
            Ok (Eval.V (Value.bool is_cond))
          | _ -> Error (Eval.Arity "cond? needs one argument")) )
  ; ( "and?"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            let is_and =
              match Ast.view e with
              | Ast.And _ -> true
              | _ -> false
            in
            Ok (Eval.V (Value.bool is_and))
          | _ -> Error (Eval.Arity "and? needs one argument")) )
  ; ( "or?"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            let is_or =
              match Ast.view e with
              | Ast.Or _ -> true
              | _ -> false
            in
            Ok (Eval.V (Value.bool is_or))
          | _ -> Error (Eval.Arity "or? needs one argument")) )
  ; ( "cond-clauses"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            (match Ast.view e with
             | Ast.Cond (clauses, else_body) ->
               let normal = List.map (fun (t, b) -> Eval.Clause (Some t, b)) clauses in
               let els =
                 match else_body with
                 | Some body -> [ Eval.Clause (None, body) ]
                 | None -> []
               in
               Ok (Eval.Args (normal @ els))
             | _ -> Error (Eval.Op_failed "cond-clauses needs a cond"))
          | _ -> Error (Eval.Arity "cond-clauses needs one argument")) )
  ; ( "no-clauses?"
    , Eval.Value_op
        (function
          | [ Eval.Args [] ] -> Ok (Eval.V (Value.bool true))
          | [ Eval.Args _ ] -> Ok (Eval.V (Value.bool false))
          | _ -> Error (Eval.Arity "no-clauses? needs a clause sequence")) )
  ; ( "first-clause"
    , Eval.Value_op
        (function
          | [ Eval.Args (c :: _) ] -> Ok c
          | [ Eval.Args [] ] ->
            Error (Eval.Op_failed "first-clause of an empty clause list")
          | _ -> Error (Eval.Arity "first-clause needs a clause sequence")) )
  ; ( "rest-clauses"
    , Eval.Value_op
        (function
          | [ Eval.Args (_ :: rest) ] -> Ok (Eval.Args rest)
          | [ Eval.Args [] ] ->
            Error (Eval.Op_failed "rest-clauses of an empty clause list")
          | _ -> Error (Eval.Arity "rest-clauses needs a clause sequence")) )
  ; ( "cond-else-clause?"
    , Eval.Value_op
        (function
          | [ Eval.Clause (None, _) ] -> Ok (Eval.V (Value.bool true))
          | [ Eval.Clause (Some _, _) ] -> Ok (Eval.V (Value.bool false))
          | _ -> Error (Eval.Arity "cond-else-clause? needs a clause")) )
  ; ( "cond-predicate"
    , Eval.Value_op
        (function
          | [ Eval.Clause (Some t, _) ] -> Ok (Eval.Exp t)
          | [ Eval.Clause (None, _) ] ->
            Error (Eval.Op_failed "the else clause has no predicate")
          | _ -> Error (Eval.Arity "cond-predicate needs a clause")) )
  ; ( "cond-actions"
    , Eval.Value_op
        (function
          | [ Eval.Clause (_, body) ] -> Ok (Eval.Seq body)
          | _ -> Error (Eval.Arity "cond-actions needs a clause")) )
  ; ( "and-operands"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            (match Ast.view e with
             | Ast.And es -> Ok (Eval.Seq es)
             | _ -> Error (Eval.Op_failed "and-operands needs an and"))
          | _ -> Error (Eval.Arity "and-operands needs one argument")) )
  ; ( "or-operands"
    , Eval.Value_op
        (function
          | [ Eval.Exp e ] ->
            (match Ast.view e with
             | Ast.Or es -> Ok (Eval.Seq es)
             | _ -> Error (Eval.Op_failed "or-operands needs an or"))
          | _ -> Error (Eval.Arity "or-operands needs one argument")) )
  ]
;;

let run source =
  Eval.make_evaluator ~controller ~operations:clause_operations ~source ()
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

(** [ex_5_24 ()] runs the cond sessions: a three-clause classify with
    an [else], a bodyless clause whose value is its test, and a cond
    with no true clause. *)
let ex_5_24 () =
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
(cond ((= 1 1)))|}
;;

(** [ex_5_24a ()] runs the and/or sessions: values of the last and
    first-true operand, the empty forms, short-circuit both ways -- a
    false cut before an unbound variable, a true cut before a false --
    and nesting. *)
let ex_5_24a () =
  run
    {|
(and 1 2 3)
(and 1 #f 3)
(and)
(and #f no-such-variable)
(or #f 2 3)
(or #f #f)
(or)
(or 1 no-such-variable)
(or #f (and #f (or #f #t)))
(define (within n) (and (> n 0) (< n 10)))
(within 5)
(within 50)|}
;;
