(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.54: [require] as a special form. This edition's choice:
    the section keeps [require] as the user-level procedure of 4.3.1 --
    [an-amb]-based, definable in the object language -- and this
    solution shows the special-form alternative the statement sketches.
    The variant dispatch recognizes [require] at an application head,
    evaluates the predicate, and answers [ok] only when it is true,
    performing [Fail] otherwise -- the completed [analyze-require].
    Failures performed by the special form are indistinguishable to the
    search from failures of the procedure version. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Ast = Sicp_common.Ast
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

module rec Require_eval : sig
  val eval_k : Eval.eval_k
end = struct
  module C = Eval.Core (Require_eval)

  let eval_k exp env succeed =
    match Ast.view exp with
    | Ast.Application (operator, operands) ->
      (match Ast.view operator with
       | Ast.Variable "require" ->
         (match operands with
          | [ predicate ] ->
            Require_eval.eval_k predicate env (fun value ->
              if Eval.true_ value
              then succeed (Value.symbol "ok")
              else Effect.perform Eval.Fail)
          | _ -> C.eval_k exp env succeed)
       | _ -> C.eval_k exp env succeed)
    | _ -> C.eval_k exp env succeed
  ;;
end

let eval exp env = Eval.drive (fun () -> Require_eval.eval_k exp env Eval.report)

let run env text =
  match Reader.read text with
  | Ok exp -> eval exp env
  | Error e -> Error (Eval_error.Invalid_form (Reader.to_string e))
;;

let program =
  {|
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))|}
;;

let ex_4_54 () =
  let env = Eval.setup_environment () in
  let (_ : (Value.t, Eval_error.t) result) = run env program in
  let true_case = run env "(require (= 1 1))" in
  let false_case = run env "(require (= 1 2))" in
  let filter_case =
    run env "(let ((x (an-element-of '(1 2 3 4)))) (require (even? x)) x)"
  in
  let filter_again = Eval.try_again () in
  let filter_exhausted = Eval.try_again () in
  List.map show [ true_case; false_case; filter_case; filter_again; filter_exhausted ]
;;
