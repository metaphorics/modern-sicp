(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.78: the query language as a nondeterministic program in the
   style of the amb evaluator of 4.3.  Deliberate deviation: the
   exercise wants the 4.3 evaluator, but [Sec_4_3] exports only [run]
   over guest programs, so this is a host-side driver with the same
   search order, not the 4.3 evaluator itself.  The matcher, the
   unifier, and the data base stay the section's; what changes is the
   control strategy.  Every enumeration point is a choice point
   threaded through two continuations -- [succeed] receives a frame and
   the failure continuation that resumes the search, [fail] backtracks
  to the most recent choice point -- so a query produces one answer and
  [try_again] produces the next by chronological backtracking: no frame
  streams, no flatmap, no interleave.  A simple query is a choice over
  its matching assertions and, after them, its rule applications, in
  the stream system's order; [And] chains through the success
   continuation; [Or] is a choice over its disjuncts; [Not] and [Holds]
   are require-style filters, where the [Not] test runs its sub-search
   with its own continuations so an exhausted inner search reports "no
   match" instead of backtracking the outer one.  Rule bodies evaluate
   as nested searches inside their candidate's continuation, so
   backtracking out of a body falls to the enclosing candidate loop with
   no extra machinery -- much of 4.4.4 is subsumed, as the exercise
   predicts.  The behavioral difference the exercise asks for is pinned:
   [Or] answers come out in depth-first order (all of the first
   disjunct, then the second), where the stream system interleaves the
   disjuncts, and the recursive [married] rule yields its duplicate
   answers one demandable [try_again] at a time. *)

open Sec_4_55.Kit

(* The driver's view of a search: an answer with the failure
   continuation that resumes the search behind it, or exhaustion. *)
type outcome =
  | Answer of Q.query * (unit -> outcome)
  | Exhausted

(* The choice point: try each alternative in order, each receiving the
   failure continuation that moves on to the next. *)
let rec amb alternatives fail =
  match alternatives with
  | [] -> fail ()
  | first :: rest -> first (fun () -> amb rest fail)
;;

let rec nqeval s ids q frame succeed fail =
  match q with
  | Q.Pattern pattern -> nsimple s ids pattern frame succeed fail
  | Q.And conjuncts -> nconjoin s ids conjuncts frame succeed fail
  | Q.Or disjuncts ->
    amb
      (List.map (fun disjunct fail -> nqeval s ids disjunct frame succeed fail) disjuncts)
      fail
  | Q.Not inner ->
    (match
       nqeval s ids inner frame (fun _ _ -> Answer (inner, fail)) (fun () -> Exhausted)
     with
     | Answer _ -> fail ()
     | Exhausted -> succeed frame fail)
  | Q.Holds _ | Q.Form _ ->
    let rec each = function
      | Streams.Empty -> fail ()
      | Streams.Cons (extended, rest) ->
        succeed extended (fun () -> each (Lazy.force rest))
    in
    each (Q.qeval s q (Q.singleton_stream frame))
  | Q.Always_true -> succeed frame fail

and nconjoin s ids conjuncts frame succeed fail =
  match conjuncts with
  | [] -> succeed frame fail
  | first :: rest ->
    nqeval
      s
      ids
      first
      frame
      (fun extended fail -> nconjoin s ids rest extended succeed fail)
      fail

and nsimple s ids pattern frame succeed fail =
  let from_assertions =
    List.map
      (fun datum fail ->
         match Q.pattern_match pattern datum frame with
         | Some extended -> succeed extended fail
         | None -> fail ())
      (take max_int (Q.fetch_assertions s pattern))
  in
  let from_rules =
    List.map
      (fun rule fail ->
         decr ids;
         let conclusion, body = Q.rename_variables_in rule !ids in
         match Q.unify_match pattern conclusion frame with
         | Some unified -> nqeval s ids body unified succeed fail
         | None -> fail ())
      (take max_int (Q.fetch_rules s pattern))
  in
  amb (from_assertions @ from_rules) fail
;;

(* The driver loop's state: the failure continuation of the problem in
   progress, if any. *)
type driver =
  { session : Q.session
  ; ids : int ref
  ; mutable problem : (unit -> outcome) option
  }

let report driver outcome =
  match outcome with
  | Answer (answer, resume) ->
    driver.problem <- Some resume;
    Q.render_query answer
  | Exhausted ->
    driver.problem <- None;
    "error: there are no more values"
  | exception Q.Query_error e ->
    driver.problem <- None;
    "error: " ^ Sicp_common.Eval_error.to_string e
;;

let ask driver q =
  report
    driver
    (nqeval
       driver.session
       driver.ids
       q
       []
       (fun frame fail -> Answer (Q.instantiate_query q frame, fail))
       (fun () -> Exhausted))
;;

let try_again driver =
  match driver.problem with
  | None -> "error: there is no current problem"
  | Some resume -> report driver (resume ())
;;

(* [try_agains driver k] is the protocol's next [k] answers, in order. *)
let try_agains driver k = List.init k (fun _ -> try_again driver)

let demo_data =
  List.filter
    (function
      | Q.Pair (Q.Atom ("job" | "supervisor"), _) -> true
      | _ -> false)
    microshaft
  @ [ atoms [ "married"; "Minnie"; "Mickey" ] ]
;;

let rules = [ l [ at "married"; v "x"; v "y" ], p [ at "married"; v "y"; v "x" ] ]

let ex_4_78 () =
  let s = session ~rules demo_data in
  (* Rule applications are numbered below zero, where the session
   counter can never collide with them. *)
  let driver = { session = s; ids = ref 0; problem = None } in
  let header q = "? " ^ Q.render_query q in
  (* Every protocol step is bound in order: list literals assemble
     already-computed strings. *)
  let one_q = p [ at "job"; v "x"; atoms [ "computer"; "programmer" ] ] in
  let one_first = ask driver one_q in
  let one_more = try_agains driver 3 in
  let one_after = try_again driver in
  let or_q =
    Q.Or
      [ p [ at "supervisor"; v "x"; person "Bitdiddle Ben" ]
      ; p [ at "supervisor"; v "x"; person "Hacker Alyssa P" ]
      ]
  in
  let two_stream = answers_upto 4 s or_q in
  let two_first = ask driver or_q in
  let two_more = try_agains driver 4 in
  let married_q = p [ at "married"; at "Mickey"; v "who" ] in
  let three_stream = answers_upto 3 s married_q in
  let three_first = ask driver married_q in
  let three_more = try_agains driver 2 in
  let not_q =
    Q.And
      [ p [ at "supervisor"; v "x"; v "y" ]
      ; Q.Not (p [ at "job"; v "x"; atoms [ "computer"; "programmer" ] ])
      ]
  in
  let four_first = ask driver not_q in
  let four_more = try_agains driver 6 in
  [ header one_q; one_first ]
  @ one_more
  @ [ "after exhaustion:"; one_after; header or_q; "stream (interleaved):" ]
  @ two_stream
  @ [ "amb (depth-first):"; two_first ]
  @ two_more
  @ [ header married_q; "stream (first 3):" ]
  @ three_stream
  @ [ "amb (first 3):"; three_first ]
  @ three_more
  @ [ header not_q; four_first ]
  @ four_more
;;
