(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.5 *)

(** Cond arrow clauses. In a clause written [(test => recipient)] the
    reader delivers [=>] as an ordinary variable expression, so the
    dispatch detects the two-element action list whose first element is
    that variable: the test is evaluated once, and when it holds, the
    recipient is evaluated and applied to the test's value through the
    evaluator, so a compound recipient works like any procedure. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  let rec eval_cond clauses else_body env =
    match clauses with
    | [] ->
      (match else_body with
       | Some body -> C.eval_sequence body env
       | None -> Ok (Value.bool false))
    | (test, actions) :: rest ->
      Ev.eval test env
      >>= fun tested ->
      if SE.false_ tested
      then eval_cond rest else_body env
      else (
        match actions with
        | [ arrow; recipient ] ->
          (match Ast.view arrow with
           | Ast.Variable "=>" ->
             Ev.eval recipient env >>= fun proc -> C.apply_procedure proc [ tested ]
           | _ -> C.eval_sequence actions env)
        | _ -> C.eval_sequence actions env)
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.Cond (clauses, else_body) -> eval_cond clauses else_body env
    | _ -> C.eval exp env
  ;;
end

let eval = Ev.eval

let run env text =
  Reader.read text
  |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
  >>= fun exp -> Ev.eval exp env
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_05 ()] evaluates the statement's [assoc] example, an arrow
      clause whose recipient is a compound procedure, and an arrow
      clause whose test fails and falls through to the [else]. *)
let ex_4_05 () =
  let env = SE.the_global_environment () in
  [ run env "(cond ((assoc 'b '((a 1) (b 2))) => cadr) (else false))"
  ; run env "(cond ((memq 'c '(a b c)) => (lambda (l) (cons 'x l))) (else 'none))"
  ; run env "(cond ((assoc 'z '((a 1))) => cadr) (else 'missing))"
  ]
  |> List.map render
;;
