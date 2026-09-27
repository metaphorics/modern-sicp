(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)
let ( >>= ) = Result.bind

(** Exercise 5.32: (a) the evaluator's symbol-operator fast path; (b)
    the answer to Alyssa's suggestion lives in the rationale.

    The modified [ev-application] tests whether the operator is a
    symbol; if it is, the lookup happens in place, without saving
    [env] or [unev] and without a round trip through
    [eval-dispatch], and control joins the ordinary path at
    [ev-appl-after-operator] with [proc] already set.  The operand
    loop is the base fragment. *)

(** The symbol-operator dispatch: inserted before the ordinary
    operator evaluation. *)
let ev_application_fast =
  {|ev-application
  (save continue)
  (assign unev (op operands) (reg exp))
  (test (op symbol-operator?) (reg exp))
  (branch (label ev-appl-symbol-operator))
  (save env)
  (save unev)
  (assign exp (op operator) (reg exp))
  (assign
   continue (label ev-appl-did-operator))
  (goto (label eval-dispatch))
ev-appl-symbol-operator
  (assign exp (op operator) (reg exp))
  (assign val (op lookup-variable-value) (reg exp) (reg env))
  (assign argl (op empty-arglist))
  (assign proc (reg val))
  (test (op no-operands?) (reg unev))
  (branch (label apply-dispatch))
  (save proc)
  (goto (label ev-appl-operand-loop))|}
;;

(** [controller] is the base evaluator with the fast-path application
    fragments. *)
let controller =
  String.concat
    "\n"
    (List.map
       (fun (n, text) ->
          match n with
          | "ev-application" -> ev_application_fast
          | _ -> text)
       Sicp_ch5.Sec_5_4.controller_fragments)
;;

(** The extra operation the dispatch names. *)
let symbol_operator_op =
  ( "symbol-operator?"
  , Sicp_ch5.Sec_5_4.Value_op
      (function
        | [ Sicp_ch5.Sec_5_4.Exp e ] ->
          (match Sicp_common.Ast.view e with
           | Sicp_common.Ast.Application (op, _) ->
             (match Sicp_common.Ast.view op with
              | Sicp_common.Ast.Variable _ ->
                Ok (Sicp_ch5.Sec_5_4.V (Sicp_common.Value.bool true))
              | _ -> Ok (Sicp_ch5.Sec_5_4.V (Sicp_common.Value.bool false)))
           | _ ->
             Error (Sicp_ch5.Sec_5_4.Op_failed "symbol-operator? needs an expression"))
        | _ -> Error (Sicp_ch5.Sec_5_4.Arity "symbol-operator? needs one argument")) )
;;

(** [run source] evaluates [source] on the fast-path evaluator and
    answers the transcript. *)
let run source =
  Sicp_ch5.Sec_5_4.make_evaluator
    ~controller
    ~operations:[ symbol_operator_op ]
    ~source
    ()
  >>= fun m ->
  (match Sicp_ch5.Sec_5_4.start m with
   | Ok () -> Ok ()
   | Error (Sicp_ch5.Sec_5_4.Op_failed m) when m = Sicp_ch5.Sec_5_4.input_exhausted ->
     Ok ()
   | Error e -> Error e)
  >>= fun () -> Ok (Sicp_ch5.Sec_5_4.transcript m)
;;

(** [ex_5_32 ()] pins the fast path: symbol-operator calls answer as
    before, a compound operator still evaluates through
    [eval-dispatch], and the base monitored factorial of 5 costs the
    book's 144 pushes; the fast path removes that call's [env] and
    [unev] saves, as the fragment shows. *)
let ex_5_32 () =
  run "(define (f x) (* x x))\n(f 6)\n((lambda (y) (+ y 1)) 41)"
  >>= fun transcript ->
  Sec_5_26.measure
    "(define (factorial n)\n(if (= n 1) 1 (* n (factorial (- n 1)))))"
    [ 5 ]
  >>= fun stats ->
  let pushes =
    match stats with
    | [ s ] -> Sec_5_26.pushes_of s
    | _ -> 0
  in
  Ok
    [ String.concat "\n" transcript
    ; Printf.sprintf "base monitored pushes at n = 5: %d" pushes
    ]
;;
