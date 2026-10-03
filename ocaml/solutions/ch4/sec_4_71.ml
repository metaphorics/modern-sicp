(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.71: why [simple_query] and [disjoin] carry explicit
   delays.  The demonstration adds one supervisor cycle, Ben -> Warbucks
   -> Ben, to a Microshaft chain and asks the recursive [outranked-by]
   rule of 4.4.1.  In the book's engine the rule body's [Or] is an
   [interleave_delayed]: constructing the answer stream stops at the
   delay, and a bounded read delivers the answers one at a time even
   though the cycle makes the stream infinite.  Louis's undelayed
   variants -- plain [stream_append] in [simple_query], plain
   [interleave] in [disjoin] -- evaluate the rest of the disjunction
   while the stream is still being built, so constructing even the
   first answer descends the infinite chain of derivations and never
   returns; the observation bounds that construction with a
   rule-application budget, under which the book's engine has long since
   answered.  Louis's two procedures are substituted into a variant
   evaluator assembled from the exported layers (the matcher, the
   unifier, [rename_variables_in], [stream_flatmap], and the stock
   evaluator for [Not] and [Holds] stay the book's); the delay, not the
   answer set, is the only difference. *)

open Sec_4_55.Kit

let rules =
  [ ( l [ at "outranked-by"; v "staff-person"; v "boss" ]
    , Q.Or
        [ p [ at "supervisor"; v "staff-person"; v "boss" ]
        ; Q.And
            [ p [ at "supervisor"; v "staff-person"; v "middle-manager" ]
            ; p [ at "outranked-by"; v "middle-manager"; v "boss" ]
            ]
        ] )
  ]
;;

let assertions =
  [ l [ at "supervisor"; person "Hacker Alyssa P"; person "Bitdiddle Ben" ]
  ; l [ at "supervisor"; person "Bitdiddle Ben"; person "Warbucks Oliver" ]
  ; l [ at "supervisor"; person "Warbucks Oliver"; person "Bitdiddle Ben" ]
  ]
;;

(* Louis's building blocks: the 3.5.3 operations without the explicit
   delay; the second stream is an ordinary, already evaluated argument. *)
let rec stream_append s1 s2 =
  match s1 with
  | Streams.Empty -> s2
  | Streams.Cons (head, tail) ->
    Streams.cons_stream head (fun () -> stream_append (Lazy.force tail) s2)
;;

let rec interleave s1 s2 =
  match s1 with
  | Streams.Empty -> s2
  | Streams.Cons (head, tail) ->
    Streams.cons_stream head (fun () -> interleave s2 (Lazy.force tail))
;;

(* The divergence meter: one rule application is one derivation step. *)
exception Budget_exhausted of int

let budget = 1000

let rec louis_qeval s steps q frames =
  match q with
  | Q.Pattern pattern ->
    Q.stream_flatmap
      (fun frame ->
         stream_append
           (find_assertions s pattern frame)
           (louis_apply_rules s steps pattern frame))
      frames
  | Q.And conjuncts ->
    List.fold_left (fun frames c -> louis_qeval s steps c frames) frames conjuncts
  | Q.Or disjuncts -> louis_disjoin s steps disjuncts frames
  | Q.Always_true -> frames
  | Q.Not _ | Q.Holds _ | Q.Form _ -> Q.qeval s q frames

and louis_disjoin s steps disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    interleave (louis_qeval s steps first frames) (louis_disjoin s steps rest frames)

and louis_apply_rules s steps pattern frame =
  Q.stream_flatmap
    (fun rule -> louis_apply_a_rule s steps rule pattern frame)
    (Q.fetch_rules s pattern)

and louis_apply_a_rule s steps rule pattern frame =
  incr steps;
  if !steps > budget then raise (Budget_exhausted !steps);
  let conclusion, body = Q.rename_variables_in rule (- !steps) in
  match Q.unify_match pattern conclusion frame with
  | None -> Streams.the_empty_stream
  | Some unified -> louis_qeval s steps body (Q.singleton_stream unified)
;;

let louis_first s q =
  let steps = ref 0 in
  match louis_qeval s steps q (Q.singleton_stream []) with
  | Streams.Empty -> "no answers"
  | Streams.Cons (frame, _) -> Q.render_query (Q.instantiate_query q frame)
  | exception Budget_exhausted steps ->
    "no answer: constructing the first answer was still applying rules after "
    ^ string_of_int steps
    ^ " steps -- the construction itself diverges"
;;

let ex_4_71 () =
  let s = session ~rules assertions in
  let q = p [ at "outranked-by"; person "Bitdiddle Ben"; v "boss" ] in
  answers_upto 3 s q
  @ [ "louis (plain stream-append in simple_query, plain interleave in disjoin): "
      ^ louis_first s q
    ; "the delay moves the rule body's evaluation from construction time to \
       answer-demand time; the cycle keeps the answer stream infinite and the delayed \
       engine still delivers it one answer at a time"
    ]
;;
