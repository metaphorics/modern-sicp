(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.67: a loop detector for the query system.  The variant
   rule application carries a history of the current chain of
   deductions: one key per rule body now being processed on the branch.
   Before a renamed rule's body is evaluated in its unified frame, the
   body is instantiated in that frame with every unbound variable
   numbered by first occurrence; if the resulting shape already occurs
   in the history, the system would begin processing a query it is
   already working on, so the branch fails.  Numbering keeps distinct
   shapes distinct where one shared wildcard would conflate them, and
   makes the comparison immune to rule-variable renaming, which is what
   defeats a literal (pattern, frame) equality: the frames of a looping
   chain grow (fresh renamed bindings each round) while the chain's
   deduction -- the query the system is effectively working on --
   repeats exactly.  The detector numbers its rule applications below
   zero, where the session counter can never collide with them, and a
   [Not] runs through the detector so a loop inside one is cut too.

   The demo is the text's married Minnie Mickey data base with its
   symmetric rule.  The stock evaluator answers
   [[married, Mickey, Minnie]] and then re-derives the same answer
   forever (a take of any size keeps returning it); the detector answers
   it once, cuts the repeated chain once, and terminates.  Two finite
   regressions show the answers still come: [wheel] and [outranked-by]
   return exactly the stock answers under the detector -- the detector
   only prunes chains that repeat a deduction already on the stack. *)

open Sec_4_55.Kit

let supervisors =
  List.filter
    (function
      | Q.Pair (Q.Atom "supervisor", _) -> true
      | _ -> false)
    microshaft
;;

let rules =
  [ l [ at "married"; v "x"; v "y" ], p [ at "married"; v "y"; v "x" ]
  ; ( l [ at "wheel"; v "person" ]
    , Q.And
        [ p [ at "supervisor"; v "middle-manager"; v "person" ]
        ; p [ at "supervisor"; v "x"; v "middle-manager" ]
        ] )
  ; ( l [ at "outranked-by"; v "staff-person"; v "boss" ]
    , Q.Or
        [ p [ at "supervisor"; v "staff-person"; v "boss" ]
        ; Q.And
            [ p [ at "supervisor"; v "staff-person"; v "middle-manager" ]
            ; p [ at "outranked-by"; v "middle-manager"; v "boss" ]
            ]
        ] )
  ]
;;

(* The deduction key: the body instantiated in its frame, with each
   unbound variable numbered by first occurrence.  Numbering keeps
   distinct shapes distinct: a body binding two different variables is
   a more constrained deduction than one binding the same variable
   twice, and a single shared wildcard would conflate them. *)
let deduction_key body frame =
  canonical_query
    (let rec instantiate = function
       | Q.Pattern t -> Q.Pattern (Q.instantiate t frame (fun v -> Q.Var v))
       | Q.And qs -> Q.And (List.map instantiate qs)
       | Q.Or qs -> Q.Or (List.map instantiate qs)
       | Q.Not q -> Q.Not (instantiate q)
       | Q.Holds (name, ts) ->
         Q.Holds (name, List.map (fun t -> Q.instantiate t frame (fun v -> Q.Var v)) ts)
       | Q.Always_true -> Q.Always_true
       | Q.Form (name, qs) -> Q.Form (name, List.map instantiate qs)
     in
     instantiate body)
;;

(* [history] holds the deduction keys of the rule bodies on the current
   chain; [cuts] counts the chains the detector refused; [ids] numbers
   the detector's rule applications below zero, where the session
   counter -- which only counts up -- can never collide with them. *)
let rec qeval_loop_safe s ids cuts history q frames =
  match q with
  | Q.Pattern pattern ->
    Q.stream_flatmap
      (fun frame ->
         Q.stream_append_delayed (find_assertions s pattern frame) (fun () ->
           apply_rules s ids cuts history pattern frame))
      frames
  | Q.And conjuncts ->
    List.fold_left
      (fun frames c -> qeval_loop_safe s ids cuts history c frames)
      frames
      conjuncts
  | Q.Or disjuncts -> disjoin s ids cuts history disjuncts frames
  | Q.Not inner ->
    Streams.stream_filter
      (fun frame ->
         Streams.stream_null
           (qeval_loop_safe s ids cuts history inner (Q.singleton_stream frame)))
      frames
  | Q.Always_true -> frames
  | Q.Holds _ | Q.Form _ -> Q.qeval s q frames

and disjoin s ids cuts history disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    Q.interleave_delayed (qeval_loop_safe s ids cuts history first frames) (fun () ->
      disjoin s ids cuts history rest frames)

and apply_rules s ids cuts history pattern frame =
  Q.stream_flatmap
    (fun rule -> apply_a_rule s ids cuts history rule pattern frame)
    (Q.fetch_rules s pattern)

(* The variant rule application: after unification, the history check on
   the body; a repeated deduction fails the branch, a fresh one is pushed
   before the body is evaluated. *)
and apply_a_rule s ids cuts history rule pattern frame =
  decr ids;
  let conclusion, body = Q.rename_variables_in rule !ids in
  match Q.unify_match pattern conclusion frame with
  | None -> Streams.the_empty_stream
  | Some unified ->
    let key = deduction_key body unified in
    if List.mem key history
    then (
      incr cuts;
      Streams.the_empty_stream)
    else qeval_loop_safe s ids cuts (key :: history) body (Q.singleton_stream unified)
;;

let run_loop_safe s q =
  let ids = ref 0 in
  let cuts = ref 0 in
  let answers = Dynarray.create () in
  Streams.stream_for_each
    (fun frame ->
       Dynarray.add_last answers (Q.render_query (Q.instantiate_query q frame)))
    (qeval_loop_safe s ids cuts [] q (Q.singleton_stream []));
  Dynarray.to_list answers, !cuts
;;

let ex_4_67 () =
  let s = session ~rules (supervisors @ [ atoms [ "married"; "Minnie"; "Mickey" ] ]) in
  let married = p [ at "married"; at "Mickey"; v "who" ] in
  let detected, cuts = run_loop_safe s married in
  let stock = answers_upto 3 s married in
  let wheel = p [ at "wheel"; v "who" ] in
  let detector_wheel, _ = run_loop_safe s wheel in
  let outranked = p [ at "outranked-by"; person "Bitdiddle Ben"; v "who" ] in
  let detector_outranked, _ = run_loop_safe s outranked in
  let same a b = List.equal String.equal a b in
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
  @ stock
  @ [ Printf.sprintf
        "wheel under the detector: %d answers, identical to stock: %b"
        (List.length detector_wheel)
        (same detector_wheel (answers_all s wheel))
    ; Printf.sprintf
        "outranked-by under the detector: %d answer(s), identical to stock: %b"
        (List.length detector_outranked)
        (same detector_outranked (answers_all s outranked))
    ]
;;
