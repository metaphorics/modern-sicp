(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.54: [require] as a special form. The book's [require] is a
   user procedure over [(amb)], the choice point with no alternatives.
   The subset's [amb] always has two alternatives, so failure cannot be
   written in the guest language, and [require] must be a special form.
   The clause below is the completed [analyze-require]: it evaluates the
   predicate and fails the branch when the predicate is false, the
   [⟨??⟩] being [(not pred-value)] and [(fail2)]. Otherwise it answers
   [()] and the branch goes on.

   The clause is recognized before the experiment's own [require], which
   it replaces. Running the same programs under the experiment's
   evaluator gives transcripts identical to this clause's: a search
   cannot tell the two apart. A true requirement answers, a false one
   fails its only branch, and a requirement filtering [1] to [4] for
   even numbers answers [2] and [4]. *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let rec eval : Eval.eval_t =
  fun search env e ->
  match Ast.view e with
  | Ast.Apply (head, [ predicate ]) ->
    (match Ast.view head with
     | Ast.Var "require" ->
       let* value = eval search env predicate in
       (match Value.view value with
        | Value.Bool true -> Ok Value.unit
        | Value.Bool false -> Eval.fail
        | _ ->
          Eval.lift (Error (Eval_error.Type_error "require: condition is not a bool")))
     | _ -> Eval.open_eval ~self:eval search env e)
  | _ -> Eval.open_eval ~self:eval search env e
;;

let preamble =
  {|
let rec an_element_of items =
  match items with
  | [] -> require false; 0
  | x :: rest -> amb x (an_element_of rest)

let even n = n mod 2 = 0
|}
;;

let programs =
  [ preamble ^ "let () = require (1 = 1); print_endline \"ok\"\n"
  ; preamble ^ "let () = require (1 = 2); print_endline \"ok\"\n"
  ; preamble
    ^ "let () = let x = an_element_of [ 1; 2; 3; 4 ] in require (even x); print_int x; \
       print_newline ()\n"
  ]
;;

let lines transcript =
  transcript |> String.split_on_char '\n' |> List.filter (fun line -> line <> "")
;;

let ex_4_54 () =
  List.concat_map
    (fun program ->
       let special_form = Eval.run_with ~eval ~forms:[] program in
       let experiment = Eval.run_with ~eval:Eval.eval ~forms:[] program in
       lines special_form
       @ [ (if special_form = experiment
            then "same as the experiment's require"
            else "differs from the experiment's require")
         ])
    programs
;;
