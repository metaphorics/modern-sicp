(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.72: why [disjoin] interleaves the disjunct streams rather
   than appending them.  The variant evaluator routes [And]/[Or] through
   its own clauses and leaves every other query to the stock [qeval];
   its [Or] joins the disjuncts with [stream_append_delayed], the same
   explicit delay the book's [disjoin] uses, so the demonstration
   isolates this exercise's append-versus-interleave choice from 4.71's
   delay question.  The demonstration query ors an infinite disjunct
   with a finite one: the swap rule over one [loves] assertion answers
   forever, while exactly three people supervise Ben.  The book's
   interleaved [Or] serves both disjuncts -- all three supervisor rows
   land inside the first six answers, drawn in proportion.  The appended
   variant starves the second disjunct: no supervisor row appears at
   all, whatever the bound, because the first stream never runs out.
   That is 3.5.3's lesson about [interleave] versus [append] verbatim. *)

open Sec_4_55.Kit

let assertions =
  [ l [ at "supervisor"; person "Hacker Alyssa P"; person "Bitdiddle Ben" ]
  ; l [ at "supervisor"; person "Fect Cy D"; person "Bitdiddle Ben" ]
  ; l [ at "supervisor"; person "Tweakit Lem E"; person "Bitdiddle Ben" ]
  ; l [ at "loves"; person "Minnie Mouse"; person "Mickey Mouse" ]
  ]
;;

let rules = [ l [ at "loves"; v "x"; v "y" ], p [ at "loves"; v "y"; v "x" ] ]

(* The variant evaluator: append instead of interleave in [Or]. *)
let rec qeval_append s q frames =
  match q with
  | Q.Or disjuncts -> disjoin_append s disjuncts frames
  | Q.And conjuncts ->
    List.fold_left (fun frames c -> qeval_append s c frames) frames conjuncts
  | Q.Pattern _ | Q.Not _ | Q.Holds _ | Q.Always_true | Q.Form _ -> Q.qeval s q frames

and disjoin_append s disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    Q.stream_append_delayed (qeval_append s first frames) (fun () ->
      disjoin_append s rest frames)
;;

let or_query =
  Q.Or
    [ p [ at "loves"; v "a"; v "b" ]
    ; p [ at "supervisor"; v "who"; person "Bitdiddle Ben" ]
    ]
;;

(* A supervisor answer instantiates the second disjunct's supervisee
   slot; the [loves] answers leave the unbound [?who]. *)
let is_supervisor_answer = function
  | Q.Or [ _; Q.Pattern (Q.Pair (_, Q.Pair (Q.Var _, _))) ] -> false
  | Q.Or [ _; Q.Pattern (Q.Pair (_, Q.Pair (_, _))) ] -> true
  | _ -> false
;;

let first_answers k frames = List.map (Q.instantiate_query or_query) (take k frames)

let summary answers =
  List.map Q.render_query answers, List.length (List.filter is_supervisor_answer answers)
;;

let ex_4_72 () =
  let s = session ~rules assertions in
  let start () = Q.singleton_stream [] in
  let interleave8, interleave8_supervisors =
    summary (first_answers 8 (Q.qeval s or_query (start ())))
  in
  let append8, append8_supervisors =
    summary (first_answers 8 (qeval_append s or_query (start ())))
  in
  let _, append20_supervisors =
    summary (first_answers 20 (qeval_append s or_query (start ())))
  in
  [ "interleave_first8" ]
  @ interleave8
  @ [ "interleave_supervisor_answers_in_first8=" ^ string_of_int interleave8_supervisors
    ; "append_first8"
    ]
  @ append8
  @ [ "append_supervisor_answers_in_first8=" ^ string_of_int append8_supervisors
    ; "append_supervisor_answers_in_first20=" ^ string_of_int append20_supervisors
    ]
;;
