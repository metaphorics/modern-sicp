(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.67: a loop detector for the query system. The variant rule
    application carries a history of the current chain of deductions: one
    (pattern, frame) pair per rule body now being processed on the branch.
    Before a renamed rule's body is evaluated in its unified frame, the
    body is instantiated in that frame with every unbound variable
    contracted to one wildcard; if the resulting shape already occurs in
    the history, the system would begin processing a query it is already
    working on, so the branch fails. The wildcard makes the comparison
    immune to rule-variable renaming, which is what defeats a literal
    (pattern, frame) equality: the frames of a looping chain grow (fresh
    renamed bindings each round) while the chain's deduction -- the query
    the system is effectively working on -- repeats exactly.

    The demo is the text's married Minnie Mickey data base with its
    symmetric rule. The stock evaluator answers
    [(married Mickey Minnie)] and then re-derives the same answer forever
    (a [take] of any size keeps returning it); the detector answers it
    once, cuts the repeated chain once, and terminates. Two finite
    regressions show the answers still come: [wheel] and [outranked-by]
    return exactly the stock answers under the detector -- the detector
    only prunes chains that repeat a deduction already on the stack. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The facts and rules of the demo: the married pair and its symmetric
   rule, plus the Microshaft supervisor data with the wheel and
   outranked-by rules for the finite regressions. *)
let demo_assertions =
  [ "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ; "(assert! (married Minnie Mickey))"
  ]
;;

let demo_rules =
  [ "(assert! (rule (married ?x ?y)\n     (married ?y ?x)))"
  ; "(assert! (rule (wheel ?person)\n\
    \  (and (supervisor ?middle-manager ?person)\n\
    \       (supervisor ?x ?middle-manager))))"
  ; "(assert! (rule (outranked-by ?staff-person ?boss)\n\
    \  (or (supervisor ?staff-person ?boss)\n\
    \      (and (supervisor ?staff-person ?middle-manager)\n\
    \           (outranked-by ?middle-manager ?boss)))))"
  ]
;;

let assert_all env texts = List.iter (fun text -> ignore (Eval.run env text)) texts

let show_result = function
  | Ok answers -> List.map Value.to_string answers
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
;;

(* {2 The loop detector} *)

(** One (pattern, frame) entry per rule body on the current deduction
    chain. *)
type history = (Value.t * Eval.frame) list

(** The wildcard every unbound variable contracts to, so two spellings of
    the same pending query compare equal however their renamed variables
    differ. *)
let wildcard = Value.symbol "?_"

(** [deduction_key pattern frame] is the query the system is effectively
    working on: [pattern] with the frame's bindings applied and its
    unbound variables anonymous. *)
let deduction_key pattern frame = Eval.instantiate pattern frame (fun _ _ -> wildcard)

(** [chain_repeats history pattern frame] holds when processing (pattern,
    frame) would re-run a deduction already on the chain. *)
let chain_repeats history pattern frame =
  let key = deduction_key pattern frame in
  List.exists (fun (p, f) -> Value.structural_equal key (deduction_key p f)) history
;;

(* Cuts observed in the current demo run. *)
let loop_cuts = ref 0

let rec qeval_loop_safe env query frames history =
  match Value.view query with
  | Value.Pair (tag, _) when Value.structural_equal tag (Value.symbol "and") ->
    conjoin env (contents query) frames history
  | Value.Pair (tag, _) when Value.structural_equal tag (Value.symbol "or") ->
    disjoin env (contents query) frames history
  | Value.Pair (tag, _) when Value.structural_equal tag (Value.symbol "always-true") ->
    frames
  | _ -> simple_query env query frames history

and contents query =
  match Value.view query with
  | Value.Pair (_, cdr) ->
    (match Eval.value_list cdr with
     | Ok items -> items
     | Error _ -> [])
  | _ -> []

and simple_query env pattern frames history =
  Eval.stream_flatmap
    (fun frame ->
       Eval.stream_append_delayed (Eval.find_assertions pattern frame) (fun () ->
         apply_rules env pattern frame history))
    frames

and apply_rules env pattern frame history =
  Eval.stream_flatmap
    (fun rule -> apply_a_rule env rule pattern frame history)
    (Eval.fetch_rules pattern frame)

(* The variant rule application: after unification, the history check on
    the body; a repeated deduction fails the branch, a fresh one is
    pushed before the body is evaluated. *)
and apply_a_rule env rule pattern frame (history : history) =
  let clean_rule = Eval.rename_variables_in rule in
  match Eval.unify_match pattern (Eval.rule_conclusion clean_rule) frame with
  | None -> Eval.Streams.the_empty_stream
  | Some unified ->
    let body = Eval.rule_body clean_rule in
    if chain_repeats history body unified
    then (
      incr loop_cuts;
      Eval.Streams.the_empty_stream)
    else
      qeval_loop_safe env body (Eval.singleton_stream unified) ((body, unified) :: history)

and conjoin env conjuncts frames history =
  match conjuncts with
  | [] -> frames
  | first :: rest -> conjoin env rest (qeval_loop_safe env first frames history) history

and disjoin env disjuncts frames history =
  match disjuncts with
  | [] -> Eval.Streams.the_empty_stream
  | first :: rest ->
    Eval.interleave_delayed (qeval_loop_safe env first frames history) (fun () ->
      disjoin env rest frames history)
;;

(** [run_loop_safe env text] evaluates a query through the detector and
    answers the shown instantiations, as the driver would. *)
let run_loop_safe env text =
  match Eval.read_query text with
  | Error message -> [ "Error: " ^ message ]
  | Ok raw ->
    let q = Eval.query_syntax_process raw in
    let frame_stream =
      qeval_loop_safe env q (Eval.singleton_stream Eval.the_empty_frame) []
    in
    List.map
      (fun f ->
         Value.to_string (Eval.instantiate q f (fun v _ -> Eval.contract_question_mark v)))
      (Eval.Streams.stream_take 1000 frame_stream)
;;

let ex_4_67 () =
  loop_cuts := 0;
  let env = Eval.the_query_system () in
  assert_all env demo_assertions;
  assert_all env demo_rules;
  let detected = run_loop_safe env "(married Mickey ?who)" in
  let cuts = !loop_cuts in
  loop_cuts := 0;
  (* The stock evaluator re-derives the same answer endlessly; three
     forced answers show the loop at work. *)
  let stock = Eval.query_upto 3 env "(married Mickey ?who)" in
  let detector_wheel = run_loop_safe env "(wheel ?who)" in
  let stock_wheel = Eval.query env "(wheel ?who)" in
  let detector_outranked = run_loop_safe env "(outranked-by (Bitdiddle Ben) ?who)" in
  let stock_outranked = Eval.query env "(outranked-by (Bitdiddle Ben) ?who)" in
  let same answers1 answers2 = List.equal String.equal answers1 answers2 in
  [ Printf.sprintf
      "married Mickey ?who with the loop detector: %d answer(s), %d deduction chain cut, \
       terminates"
      (List.length detected)
      cuts
  ]
  @ detected
  @ [ "stock evaluator, take(3) of the same query -- the loop re-derives the answer \
       forever:"
    ]
  @ show_result stock
  @ [ Printf.sprintf
        "wheel under the detector: %d answers, identical to stock: %b"
        (List.length detector_wheel)
        (same detector_wheel (show_result stock_wheel))
    ; Printf.sprintf
        "outranked-by under the detector: %d answer(s), identical to stock: %b"
        (List.length detector_outranked)
        (same detector_outranked (show_result stock_outranked))
    ]
;;
