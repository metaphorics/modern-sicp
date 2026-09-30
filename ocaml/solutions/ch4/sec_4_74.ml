(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.74: Alyssa P. Hacker's [simple-stream-flatmap] beside the
   book's [stream-flatmap], each with a frame counter, run through two
   variant evaluators over one shared query.  Alyssa's version fills the
   missing expressions literally: [simple-flatten] is [stream_map] of
   [stream_car] over [stream_filter] of the non-empty element streams.
   The book's version is the engine's interleave-based [stream_flatmap]
   with the counting placed at its output conses.

   Both variant evaluators replace exactly the three paths the exercise
   names -- [find_assertions], [Not], and [Holds] (the book's
   [lisp-value]) use the flatmap under test -- while the frame stream's
   own combination (the simple query over its input frames, rule
   application, the [And]/[Or] plumbing) uses the book's flatmap
   untouched on both sides, because those procedures do return
   multi-frame streams and Alyssa's change does not apply to them.  The
   inner query of [Not] and the single-frame [Holds] test go through the
   stock [qeval], as in the book.

   The shared query is the 4.4.1 compound query
   [and([supervisor, ?x, ?y], not([job, ?x, [computer, programmer]]))].
   4.74b's answer, measured: the behavior does not change -- identical
   answer lists and identical frame counts, because the procedure mapped
   over the frame stream in these three paths always returns either the
   empty stream or a singleton (a match either exists or not), so
   interleaving and first-survivor selection combine the same frames.
   [ex_4_74a] reports the measured counts and re-runs the measurement on
   a [Holds] query to cover the third replaced path. *)

open Sec_4_55.Kit

(* Counting rule: a frame is counted when it is emitted into the
   flatmap's output stream; the pinned queries are read to the end, so
   the total is exactly the output's size. *)
let rec counted_output count = function
  | Streams.Empty -> Streams.the_empty_stream
  | Streams.Cons (head, tail) ->
    incr count;
    Streams.cons_stream head (fun () -> counted_output count (Lazy.force tail))
;;

type flatmap_choice =
  | Book
  | Simple

(* Alyssa's [simple-stream-flatmap]: the procedure's results are empty
   or singleton streams, so the first element of each non-empty one is
   all there is to keep. *)
let simple_stream_flatmap proc s =
  Streams.stream_map
    Streams.stream_car
    (Streams.stream_filter
       (fun element -> not (Streams.stream_null element))
       (Streams.stream_map proc s))
;;

let flatmap choice count proc s =
  match choice with
  | Book -> counted_output count (Q.stream_flatmap proc s)
  | Simple -> counted_output count (simple_stream_flatmap proc s)
;;

(* The variant evaluator for one flatmap choice. *)
(* Variant rule applications are numbered below zero, where the
   session counter -- which only counts up -- can never collide. *)
let make_qeval choice count =
  let ids = ref 0 in
  let find_assertions s pattern frame =
    flatmap
      choice
      count
      (fun datum ->
         match Q.pattern_match pattern datum frame with
         | Some extended -> Q.singleton_stream extended
         | None -> Streams.the_empty_stream)
      (Q.fetch_assertions s pattern)
  in
  let rec qeval s q frames =
    match q with
    | Q.Pattern pattern ->
      Q.stream_flatmap
        (fun frame ->
           Q.stream_append_delayed (find_assertions s pattern frame) (fun () ->
             apply_rules s pattern frame))
        frames
    | Q.And conjuncts ->
      List.fold_left (fun frames c -> qeval s c frames) frames conjuncts
    | Q.Or disjuncts -> disjoin s disjuncts frames
    | Q.Not inner ->
      flatmap
        choice
        count
        (fun frame ->
           if Streams.stream_null (Q.qeval s inner (Q.singleton_stream frame))
           then Q.singleton_stream frame
           else Streams.the_empty_stream)
        frames
    | Q.Holds _ ->
      flatmap choice count (fun frame -> Q.qeval s q (Q.singleton_stream frame)) frames
    | Q.Always_true -> frames
    | Q.Form _ -> Q.qeval s q frames
  and disjoin s disjuncts frames =
    match disjuncts with
    | [] -> Streams.the_empty_stream
    | first :: rest ->
      Q.interleave_delayed (qeval s first frames) (fun () -> disjoin s rest frames)
  and apply_rules s pattern frame =
    Q.stream_flatmap
      (fun rule ->
         decr ids;
         let conclusion, body = Q.rename_variables_in rule !ids in
         match Q.unify_match pattern conclusion frame with
         | Some unified -> qeval s body (Q.singleton_stream unified)
         | None -> Streams.the_empty_stream)
      (Q.fetch_rules s pattern)
  in
  qeval
;;

(* One query through one variant evaluator: its answers and its measured
   frame count in the three replaced paths. *)
let measure choice s q =
  let count = ref 0 in
  let answers = Dynarray.create () in
  Streams.stream_for_each
    (fun frame ->
       Dynarray.add_last answers (Q.render_query (Q.instantiate_query q frame)))
    (make_qeval choice count s q (Q.singleton_stream []));
  Dynarray.to_list answers, !count
;;

let shared_query =
  Q.And
    [ p [ at "supervisor"; v "x"; v "y" ]
    ; Q.Not (p [ at "job"; v "x"; atoms [ "computer"; "programmer" ] ])
    ]
;;

let holds_query =
  Q.And
    [ p [ at "salary"; v "person"; v "amount" ]; Q.Holds (">", [ v "amount"; n 30000 ]) ]
;;

let ex_4_74 () =
  let s = session microshaft in
  let book_answers, book_count = measure Book s shared_query in
  let simple_answers, simple_count = measure Simple s shared_query in
  book_answers
  @ [ "book_flatmap_frames=" ^ string_of_int book_count
    ; "simple_flatmap_frames=" ^ string_of_int simple_count
    ; "answers_equal=" ^ string_of_bool (book_answers = simple_answers)
    ]
;;

let ex_4_74a () =
  let s = session microshaft in
  let book_answers, book_count = measure Book s shared_query in
  let simple_answers, simple_count = measure Simple s shared_query in
  let holds_book, holds_book_count = measure Book s holds_query in
  let holds_simple, holds_simple_count = measure Simple s holds_query in
  [ "query=" ^ Q.render_query shared_query
  ; "old_frames=" ^ string_of_int book_count
  ; "simple_frames=" ^ string_of_int simple_count
  ; "answers_equal=" ^ string_of_bool (book_answers = simple_answers)
  ; "counts_equal=" ^ string_of_bool (book_count = simple_count)
  ; "holds_query=" ^ Q.render_query holds_query
  ; "holds_old_frames=" ^ string_of_int holds_book_count
  ; "holds_simple_frames=" ^ string_of_int holds_simple_count
  ; "holds_answers_equal=" ^ string_of_bool (holds_book = holds_simple)
  ; "holds_counts_equal=" ^ string_of_bool (holds_book_count = holds_simple_count)
  ]
;;
