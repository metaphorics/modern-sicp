(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.62: rules for [last-pair], the operation of exercise
    2.17. The base case reads "the last pair of a one-element list is
    that list"; the step strips the first element and recurses. The
    first three statement queries behave differently, and the
    demonstration reports each as the engine actually answers it.

    [(last-pair (3) ?x)] and [(last-pair (1 2 3) ?x)] answer once and
    run dry: the step rule's body walks down to [()] where no rule
    matches. [(last-pair (2 ?x) (3))] also terminates -- with exactly
    the intended answer. Its step rule binds the tail variable to the
    query's structured one-element tail; re-applying the step rule to
    that body would unify the conclusion's [(?v . ?w)] with the bound
    tail, and the required [()] against the already-bound list fails,
    so the recursive branch dies after the one answer.

    [(last-pair ?x (3))] does not run dry: [?x] is free, so every
    application of the step rule rebinds its fresh tail variable and
    yields one more answer one level deeper -- [(last-pair (3) (3))],
    then answers carrying renamed variables the driver prints as
    [?name-<id>], forever. The full query never terminates; the
    demonstration pins its finite prefix through [Eval.query_upto]. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let rules =
  [ "(assert! (rule (last-pair (?x) (?x))))"
  ; "(assert! (rule (last-pair (?v . ?w) ?y)\n     (last-pair ?w ?y)))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "a rule answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    rules
;;

let answers_or_fail = function
  | Ok answers -> List.map Value.to_string answers
  | Error e -> failwith ("query failed: " ^ Eval_error.to_string e)
;;

(** [ex_4_62 ()] pins the four statement queries; the reverse query is
    read through [query_upto] because its full answer stream never
    terminates. *)
let ex_4_62 () =
  let env = Eval.the_query_system () in
  load env;
  let one = answers_or_fail (Eval.query env "(last-pair (3) ?x)") in
  let three = answers_or_fail (Eval.query env "(last-pair (1 2 3) ?x)") in
  let template = answers_or_fail (Eval.query env "(last-pair (2 ?x) (3))") in
  let reverse = answers_or_fail (Eval.query_upto 3 env "(last-pair ?x (3))") in
  [ "query: (last-pair (3) ?x)" ]
  @ one
  @ [ "query: (last-pair (1 2 3) ?x)" ]
  @ three
  @ [ "query: (last-pair (2 ?x) (3))"
    ; "note: terminates; the step rule's recursive branch dies against the query's \
       already-bound one-element tail"
    ]
  @ template
  @ [ "query: (last-pair ?x (3))"
    ; "note: the full query never terminates; each step-rule application rebinds its \
       fresh tail variable and yields one more answer; first three answers:"
    ]
  @ reverse
;;
