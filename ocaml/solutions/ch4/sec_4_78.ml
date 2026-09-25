(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.78: the query language as a nondeterministic program on
    the evaluator of 4.3. The matcher, the unifier, and the data base
    stay the section's ([Sec_4_4]); what changes is the control
    strategy: every enumeration point is a choice point of the amb
    engine, so a query produces one answer and [try_again] produces
    the next by chronological backtracking -- no frame streams, no
    flatmap, no interleave. A simple query is [choice] over its
    matching assertions and, after them, its rule applications, in the
    stream system's order; [and] chains through the frame
    continuation; [or] is [choice] over its disjuncts; [not] and
    [lisp-value] are require-style filters, where the [not] test runs
    its sub-search behind a [Fail] boundary so an exhausted inner
    search reports "no match" instead of backtracking the outer one.
    Rule bodies evaluate as nested searches inside their candidate's
    continuation, so backtracking out of a body falls to the enclosing
    candidate loop with no extra machinery -- much of 4.4.4 is
    subsumed, as the exercise predicts. The behavioral difference the
    exercise asks for is pinned: [or] answers come out in depth-first
    order (all of the first disjunct, then the second), where the
    stream system interleaves the disjuncts, and the recursive
    [married] rule yields its duplicate answers one demandable
    [try-again] at a time. *)

module Amb = Sicp_ch4.Sec_4_3
module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Streams = Eval.Streams
module Value = Sicp_common.Value

(* Raised by the [not] test's sub-search when it finds a match. *)
exception Found_match

(* The Microshaft assertions and the famous-marriage rule this demo
   needs, in the book's order. *)
let assertions =
  [ "(assert! (job (Bitdiddle Ben) (computer wizard)))"
  ; "(assert! (job (Hacker Alyssa P) (computer programmer)))"
  ; "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (job (Fect Cy D) (computer programmer)))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (job (Tweakit Lem E) (computer technician)))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (job (Reasoner Louis) (computer programmer trainee)))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ; "(assert! (married Minnie Mickey))"
  ; "(assert! (rule (married ?x ?y)\n(married ?y ?x)))"
  ]
;;

let stream_to_list s =
  let rec go s acc =
    if Streams.stream_null s
    then List.rev acc
    else go (Streams.stream_cdr s) (Streams.stream_car s :: acc)
  in
  go s []
;;

let head_symbol query =
  match Value.view query with
  | Value.Pair (head, _) ->
    (match Value.view head with
     | Value.Symbol name -> Some name
     | _ -> None)
  | _ -> None
;;

let contents query =
  match Value.view query with
  | Value.Pair (_, body) ->
    (match Eval.value_list body with
     | Ok items -> items
     | Error e -> raise (Amb.Raised e))
  | _ -> raise (Amb.Raised (Eval_error.Invalid_form (Value.to_string query)))
;;

(* [swallow_fail thunk] runs [thunk] to completion, absorbing the [Fail]
   that an exhausted inner search performs so it is not mistaken for a
   dead end of the enclosing search. *)
let swallow_fail (thunk : unit -> unit) : unit =
  Effect.Deep.match_with
    thunk
    ()
    { retc = (fun () -> ())
    ; exnc = raise
    ; effc =
        (fun (type a) (eff : a Effect.t) ->
          match eff with
          | Amb.Fail -> Some (fun (_dead : (a, unit) continuation) -> ())
          | _ -> None)
    }
;;

(* The nondeterministic evaluator: [succeed] is the frame continuation
   the book threads as the success procedure; failure is the [Fail]
   effect unwinding to the innermost choice point. *)
let rec nqeval env query frame succeed =
  match head_symbol query with
  | Some "and" -> nconjoin env (contents query) frame succeed
  | Some "or" -> ndisjoin env (contents query) frame succeed
  | Some "not" -> nnot env (contents query) frame succeed
  | Some "lisp-value" -> nlisp env (contents query) frame succeed
  | Some "always-true" -> succeed frame
  | _ -> nsimple env query frame succeed

and nconjoin env conjuncts frame succeed =
  match conjuncts with
  | [] -> succeed frame
  | first :: rest ->
    nqeval env first frame (fun extended -> nconjoin env rest extended succeed)

and ndisjoin env disjuncts frame succeed =
  (* Depth-first: each disjunct is one alternative that enumerates all
     of its answers before the next disjunct is tried. *)
  let alternatives =
    List.map (fun disjunct _env _value -> nqeval env disjunct frame succeed) disjuncts
  in
  Amb.choice alternatives env (fun _ -> ())

and nnot env operands frame succeed =
  let query = Eval.first_operand operands in
  if inner_has_match env query frame then Effect.perform Amb.Fail else succeed frame

and nlisp env operands frame succeed =
  let call = Eval.first_operand operands in
  let instantiated =
    Eval.instantiate call frame (fun v _ ->
      raise
        (Amb.Raised
           (Eval_error.User_error ("Unknown pat var LISP-VALUE: " ^ Value.to_string v))))
  in
  match Eval.execute env instantiated with
  | Ok true -> succeed frame
  | Ok false -> Effect.perform Amb.Fail
  | Error e -> raise (Amb.Raised e)

and nsimple env pattern frame succeed =
  let assertion_alternatives =
    List.filter_map
      (fun assertion ->
         match Eval.pattern_match pattern assertion frame with
         | Some extended -> Some (fun _env _value -> succeed extended)
         | None -> None)
      (stream_to_list (Eval.fetch_assertions pattern frame))
  in
  let rule_alternatives =
    List.filter_map
      (fun rule ->
         let clean_rule = Eval.rename_variables_in rule in
         match Eval.unify_match pattern (Eval.rule_conclusion clean_rule) frame with
         | Some unified ->
           Some
             (fun _env _value -> nqeval env (Eval.rule_body clean_rule) unified succeed)
         | None -> None)
      (stream_to_list (Eval.fetch_rules pattern frame))
  in
  Amb.choice (assertion_alternatives @ rule_alternatives) env (fun _ -> ())

(* [inner_has_match env query frame] decides the [not] filter
   deterministically: the first extension of the frame raises
   [Found_match]; exhaustion without a match is absorbed at the
   boundary instead of backtracking the enclosing search. *)
and inner_has_match env query frame =
  let found = ref false in
  (try
     swallow_fail (fun () ->
       nqeval env query frame (fun _ ->
         found := true;
         raise Found_match))
   with
   | Found_match -> ());
  !found
;;

(** [ask env text] is one driver interaction: the first answer of the
    query, or the typed error. The search stays suspended behind the
    answer for [try_again]. *)
let ask env text =
  match Eval.read_query text with
  | Error message -> Error (Eval_error.Invalid_form message)
  | Ok raw ->
    let query = Eval.query_syntax_process raw in
    Amb.drive (fun () ->
      nqeval env query Eval.the_empty_frame (fun frame ->
        Amb.report
          (Eval.instantiate query frame (fun v _ -> Eval.contract_question_mark v))))
;;

let show = function
  | Ok value -> Value.to_string value
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let load_microshaft env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "an assertion answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    assertions
;;

(* [try_agains n] is the protocol's next [n] answers, one line each;
   the calls run left to right, in protocol order. *)
let try_agains n =
  let rec go k acc =
    if k = 0 then List.rev acc else go (k - 1) (show (Amb.try_again ()) :: acc)
  in
  go n []
;;

(* The stream engine's rendering of the same query, first [n] answers. *)
let stream_answers n env text =
  match Eval.query_upto n env text with
  | Ok answers -> List.map Value.to_string answers
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
;;

let ex_4_78 () =
  let env = Eval.the_query_system () in
  load_microshaft env;
  (* Every protocol step is bound in order: list literals assemble
     already-computed strings, never interleaving effects. *)
  (* 1: a simple query; the protocol walks Alyssa, Fect, exhaustion,
     and then reports that no problem is in progress. *)
  let one_text = "(job ?x (computer programmer))" in
  let one_first = show (ask env one_text) in
  let one_more = try_agains 3 in
  let one_after = show (Amb.try_again ()) in
  let one = [ one_text; one_first ] @ one_more @ [ "after exhaustion:"; one_after ] in
  (* 2: [or] is depth-first here; the stream system interleaves. *)
  let or_text =
    "(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))"
  in
  let two_stream = stream_answers 4 env or_text in
  let two_first = show (ask env or_text) in
  let two_more = try_agains 4 in
  let two =
    [ or_text; "stream (interleaved):" ]
    @ two_stream
    @ [ "amb (depth-first):"; two_first ]
    @ two_more
  in
  (* 3: the recursive married rule: every answer is the same
     duplicate, one rule application deeper each demand. *)
  let married_text = "(married Mickey ?who)" in
  let three_stream = stream_answers 3 env married_text in
  let three_first = show (ask env married_text) in
  let three_more = try_agains 2 in
  let three =
    [ married_text; "stream (first 3):" ]
    @ three_stream
    @ [ "amb (first 3):"; three_first ]
    @ three_more
  in
  (* 4: [not] as a require filter over bound frames. *)
  let not_text = "(and (supervisor ?x ?y) (not (job ?x (computer programmer))))" in
  let four_first = show (ask env not_text) in
  let four_more = try_agains 6 in
  let four = not_text :: four_first :: four_more in
  one @ two @ three @ four
;;
