(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.71: why [simple_query] and [disjoin] carry explicit
    [delay]s. The demonstration adds one supervisor cycle, Ben ->
    Warbucks -> Ben, to a Microshaft chain and asks the recursive
    [outranked-by] rule of 4.4.1. In the book's engine the rule body's
    [or] is an [interleave_delayed]: constructing the answer stream
    stops at the delay, and [query_upto] delivers the answers one at a
    time even though the cycle makes the stream infinite. Louis's
    undelayed variants -- plain [stream-append] in [simple_query],
    plain [interleave] in [disjoin] -- evaluate the rest of the
    disjunction while the stream is still being built, so constructing
    even the first answer descends the infinite chain of derivations
    and never returns; the observation bounds that construction with a
    rule-application budget, under which the book's engine has long
    since answered. Louis's two procedures are substituted verbatim
    into a variant evaluator assembled from the exported layers
    ([find_assertions], [fetch_rules], [unify_match],
    [rename_variables_in], [negate], [lisp_value], [stream_flatmap]
    stay the book's); the delay, not the answer set, is the only
    difference. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let assertions =
  [ "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (supervisor (Warbucks Oliver) (Bitdiddle Ben)))"
  ; "(assert! (rule (outranked-by ?staff-person ?boss)\n\
     (or (supervisor ?staff-person ?boss)\n\
     (and (supervisor ?staff-person ?middle-manager)\n\
     (outranked-by ?middle-manager ?boss)))))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "an assertion answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    assertions
;;

(* Louis's building blocks: the 3.5.3 operations without the explicit
   delay. *)
let rec stream_append s1 s2 =
  if Streams.stream_null s1
  then s2
  else
    Streams.cons_stream (Streams.stream_car s1) (fun () ->
      stream_append (Streams.stream_cdr s1) s2)
;;

let rec interleave s1 s2 =
  if Streams.stream_null s1
  then s2
  else
    Streams.cons_stream (Streams.stream_car s1) (fun () ->
      interleave (Streams.stream_cdr s1) s2)
;;

(* The divergence meter: the undelayed construction never returns, so the
   observation bounds it. One rule application is one derivation step; the
   book's delayed engine answers the same query with a handful. *)
exception Budget_exhausted of int

let louis_steps = ref 0
let louis_budget = 1000

let rec louis_qeval env query frames =
  let is_tag tag =
    match Value.view query with
    | Value.Pair (car, _) -> Value.structural_equal car (Value.symbol tag)
    | _ -> false
  in
  let operands =
    match
      Eval.value_list
        (match Value.view query with
         | Value.Pair (_, cdr) -> cdr
         | _ -> Value.nil)
    with
    | Ok l -> l
    | Error _ -> []
  in
  if is_tag "and"
  then louis_conjoin env operands frames
  else if is_tag "or"
  then louis_disjoin env operands frames
  else if is_tag "not"
  then Eval.negate env operands frames
  else if is_tag "lisp-value"
  then Eval.lisp_value env operands frames
  else louis_simple_query env query frames

(** The book's conjoin, with Louis's [qeval] as the evaluator: the series
    composition evaluates each conjunct's stream while the combination is
    still being built, which is where the undelayed rule evaluation
    bites. *)
and louis_conjoin env conjuncts frames =
  match conjuncts with
  | [] -> frames
  | first :: rest -> louis_conjoin env rest (louis_qeval env first frames)

and louis_simple_query env pattern frames =
  Eval.stream_flatmap
    (fun frame ->
       stream_append
         (Eval.find_assertions pattern frame)
         (louis_apply_rules env pattern frame))
    frames

and louis_apply_rules env pattern frame =
  Eval.stream_flatmap
    (fun rule -> louis_apply_a_rule env rule pattern frame)
    (Eval.fetch_rules pattern frame)

and louis_apply_a_rule env rule pattern frame =
  incr louis_steps;
  if !louis_steps > louis_budget then raise (Budget_exhausted !louis_steps);
  let clean_rule = Eval.rename_variables_in rule in
  match Eval.unify_match pattern (Eval.rule_conclusion clean_rule) frame with
  | None -> Streams.the_empty_stream
  | Some frame ->
    louis_qeval env (Eval.rule_body clean_rule) (Eval.singleton_stream frame)

and louis_disjoin env disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    interleave (louis_qeval env first frames) (louis_disjoin env rest frames)
;;

let book_answers env =
  match Eval.query_upto 3 env "(outranked-by (Bitdiddle Ben) ?boss)" with
  | Ok answers -> List.map Value.to_string answers
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
;;

let louis_first env =
  louis_steps := 0;
  match Eval.read_query "(outranked-by (Bitdiddle Ben) ?boss)" with
  | Error message -> "Error: " ^ message
  | Ok raw ->
    (try
       let q = Eval.query_syntax_process raw in
       let frames = louis_qeval env q (Eval.singleton_stream Eval.the_empty_frame) in
       if Streams.stream_null frames
       then "no answers"
       else (
         let frame = Streams.stream_car frames in
         Value.to_string
           (Eval.instantiate q frame (fun v _ -> Eval.contract_question_mark v)))
     with
     | Budget_exhausted steps ->
       "no answer: constructing the first answer was still applying rules after "
       ^ string_of_int steps
       ^ " steps -- the construction itself diverges")
;;

let ex_4_71 () =
  let env = Eval.the_query_system () in
  load env;
  let answers = book_answers env in
  let louis = louis_first env in
  answers
  @ [ "louis (plain stream-append in simple_query, plain interleave in disjoin): " ^ louis
    ; "the delay moves the rule body's evaluation from construction time to \
       answer-demand time; the cycle keeps the answer stream infinite and the delayed \
       engine still delivers it one answer at a time"
    ]
;;
