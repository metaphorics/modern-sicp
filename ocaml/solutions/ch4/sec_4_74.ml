(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.74: Alyssa P. Hacker's [simple-stream-flatmap] beside the
    book's [stream-flatmap], each with a frame counter, run through two
    variant evaluators over one shared query.  Alyssa's version fills
    the missing expressions literally: [simple-flatten] is
    [stream-map] of [stream-car] over [stream-filter] of the non-empty
    element streams, in [Eval.Streams] style.  The book's version is the
    substrate's interleave-based [stream_flatmap] with the counting
    placed at its output conses.

    Both variant evaluators are built from the exported layers and
    replace exactly the three paths the exercise names --
    [find_assertions], [negate], and [lisp_value] use the flatmap under
    test -- while the frame stream's own combination ([simple_query],
    [apply_rules], the [and]/[or] plumbing) uses the book's flatmap
    untouched on both sides, because those procs do return multi-frame
    streams and Alyssa's change does not apply to them.  The inner
    queries of [negate] go through the substrate's [qeval], as in the
    book.

    The shared query is the 4.4.1 compound query
    [(and (supervisor ?x ?y) (not (job ?x (computer programmer))))].
    4.74b's answer, measured: the behavior does not change -- identical
    answer lists and identical frame counts, because the proc mapped
    over the frame stream in these three paths always returns either
    the empty stream or a singleton (a match either exists or not), so
    interleaving and first-survivor selection combine the same frames.
    [ex_4_74a] reports the measured counts and re-runs the measurement
    on a [lisp-value] query to cover the third replaced path. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The Microshaft personnel data base of 4.4.1, in the book's order. *)
let assertions =
  [ "(assert! (address (Bitdiddle Ben) (Slumerville (Ridge Road) 10)))"
  ; "(assert! (job (Bitdiddle Ben) (computer wizard)))"
  ; "(assert! (salary (Bitdiddle Ben) 60000))"
  ; "(assert! (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))"
  ; "(assert! (job (Hacker Alyssa P) (computer programmer)))"
  ; "(assert! (salary (Hacker Alyssa P) 40000))"
  ; "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (address (Fect Cy D) (Cambridge (Ames Street) 3)))"
  ; "(assert! (job (Fect Cy D) (computer programmer)))"
  ; "(assert! (salary (Fect Cy D) 35000))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (address (Tweakit Lem E) (Boston (Bay State Road) 22)))"
  ; "(assert! (job (Tweakit Lem E) (computer technician)))"
  ; "(assert! (salary (Tweakit Lem E) 25000))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (address (Reasoner Louis) (Slumerville (Pine Tree Road) 80)))"
  ; "(assert! (job (Reasoner Louis) (computer programmer trainee)))"
  ; "(assert! (salary (Reasoner Louis) 30000))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (address (Warbucks Oliver) (Swellesley (Top Heap Road))))"
  ; "(assert! (job (Warbucks Oliver) (administration big wheel)))"
  ; "(assert! (salary (Warbucks Oliver) 150000))"
  ; "(assert! (address (Scrooge Eben) (Weston (Shady Lane) 10)))"
  ; "(assert! (job (Scrooge Eben) (accounting chief accountant)))"
  ; "(assert! (salary (Scrooge Eben) 75000))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (address (Cratchet Robert) (Allston (N Harvard Street) 16)))"
  ; "(assert! (job (Cratchet Robert) (accounting scrivener)))"
  ; "(assert! (salary (Cratchet Robert) 18000))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (address (Aull DeWitt) (Slumerville (Onion Square) 5)))"
  ; "(assert! (job (Aull DeWitt) (administration secretary)))"
  ; "(assert! (salary (Aull DeWitt) 25000))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ; "(assert! (can-do-job (computer wizard) (computer programmer)))"
  ; "(assert! (can-do-job (computer wizard) (computer technician)))"
  ; "(assert! (can-do-job (computer programmer) (computer programmer trainee)))"
  ; "(assert! (can-do-job (administration secretary) (administration big wheel)))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | _ -> failwith "load: expected an assertion")
    assertions
;;

(* Counting rule: a frame is counted when it is emitted into the
   flatmap's output stream -- one increment per frame delivered on the
   output.  The wrapper adds one cell per delivered frame; the pinned
   queries are finite and read to the end, so the total is exactly the
   output's size, once per frame, in emission order. *)
let rec counted_output count s =
  if Streams.stream_null s
  then Streams.the_empty_stream
  else (
    incr count;
    Streams.cons_stream (Streams.stream_car s) (fun () ->
      counted_output count (Streams.stream_cdr s)))
;;

(* The book's [stream-flatmap] -- the substrate's interleave-based
   definition -- with the counter on its output. *)
let stream_flatmap_book count proc s = counted_output count (Eval.stream_flatmap proc s)

(* Alyssa's [simple-stream-flatmap] with the same counter:
   [stream-map] of [stream-car] over [stream-filter] of the non-empty
   element streams, exactly the missing expressions filled in. *)
let stream_flatmap_simple count proc s =
  let mapped = Streams.stream_map proc s in
  let kept =
    Streams.stream_filter (fun element -> not (Streams.stream_null element)) mapped
  in
  counted_output count (Streams.stream_map Streams.stream_car kept)
;;

let check_an_assertion assertion query_pattern query_frame =
  match Eval.pattern_match query_pattern assertion query_frame with
  | None -> Streams.the_empty_stream
  | Some frame -> Eval.singleton_stream frame
;;

let contents_of query =
  match Value.view query with
  | Value.Pair (_, cdr) -> Eval.value_list cdr
  | _ -> Error (Eval_error.Invalid_form "a compound query must be a list")
;;

(* The local twin of the substrate's internal [Raised] error, which is
   not exported; [lisp_value]'s error branches raise it, exactly where
   the book's evaluator raises its error out of the flatmap.  The pinned
   queries never take these branches. *)
exception Variant_error of Eval_error.t

(* Which flatmap a variant evaluator drives its three replaced paths
   with.  The dispatcher stays polymorphic, so every use site --
   assertions in, frames out; frames in, frames out -- gets its own
   instantiation, which a shared function parameter would not. *)
type flatmap_choice =
  | Book
  | Simple

let apply_flatmap choice count proc s =
  match choice with
  | Book -> stream_flatmap_book count proc s
  | Simple -> stream_flatmap_simple count proc s
;;

(* The variant evaluator: the flatmap under test drives the three
   replaced paths; the frame stream's own combination keeps the book's
   flatmap, whose procs are genuinely multi-frame. *)
let make_qeval choice count =
  let flatmap proc s = apply_flatmap choice count proc s in
  let find_assertions pattern frame =
    flatmap
      (fun assertion -> check_an_assertion assertion pattern frame)
      (Eval.fetch_assertions pattern frame)
  in
  let apply_rules env pattern frame =
    Eval.stream_flatmap
      (fun rule -> Eval.apply_a_rule env rule pattern frame)
      (Eval.fetch_rules pattern frame)
  in
  let simple_query env pattern frames =
    Eval.stream_flatmap
      (fun frame ->
         Eval.stream_append_delayed (find_assertions pattern frame) (fun () ->
           apply_rules env pattern frame))
      frames
  in
  let negate env operands frames =
    flatmap
      (fun frame ->
         if
           Streams.stream_null
             (Eval.qeval env (Eval.first_operand operands) (Eval.singleton_stream frame))
         then Eval.singleton_stream frame
         else Streams.the_empty_stream)
      frames
  in
  let lisp_value env operands frames =
    let call = List.fold_right Value.pair operands Value.nil in
    flatmap
      (fun frame ->
         let instantiated =
           Eval.instantiate call frame (fun v _ ->
             raise
               (Variant_error
                  (Eval_error.User_error
                     ("Unknown pat var LISP-VALUE: " ^ Value.to_string v))))
         in
         match Eval.execute env instantiated with
         | Ok true -> Eval.singleton_stream frame
         | Ok false -> Streams.the_empty_stream
         | Error e -> raise (Variant_error e))
      frames
  in
  let rec conjoin env conjuncts frames =
    match conjuncts with
    | [] -> frames
    | first :: rest -> conjoin env rest (qeval env first frames)
  and disjoin env disjuncts frames =
    match disjuncts with
    | [] -> Streams.the_empty_stream
    | first :: rest ->
      Eval.interleave_delayed (qeval env first frames) (fun () -> disjoin env rest frames)
  and qeval env query frames =
    match Value.view query with
    | Value.Pair (tag, _) ->
      let name = Value.to_string tag in
      (match contents_of query with
       | Ok operands when String.equal name "and" -> conjoin env operands frames
       | Ok operands when String.equal name "or" -> disjoin env operands frames
       | Ok operands when String.equal name "not" -> negate env operands frames
       | Ok operands when String.equal name "lisp-value" -> lisp_value env operands frames
       | _ ->
         (* another registered special form goes to the substrate; an
           untagged pattern is a simple query, the counted one *)
         (match Eval.get name "qeval" with
          | Some _ -> Eval.qeval env query frames
          | None -> simple_query env query frames))
    | _ -> simple_query env query frames
  in
  qeval
;;

(* Run one query through one variant evaluator: its answers plus its
   measured frame count in the three replaced paths. *)
let measure choice env n text =
  let count = ref 0 in
  let qeval_v = make_qeval choice count in
  match Eval.read_query text with
  | Error message -> [ "Error: " ^ message ], 0
  | Ok raw ->
    let q = Eval.query_syntax_process raw in
    let frames = qeval_v env q (Eval.singleton_stream Eval.the_empty_frame) in
    let answers =
      Streams.stream_take
        n
        (Streams.stream_map
           (fun frame ->
              Eval.instantiate q frame (fun v _ -> Eval.contract_question_mark v))
           frames)
    in
    List.map Value.to_string answers, !count
;;

let shared_query = "(and (supervisor ?x ?y) (not (job ?x (computer programmer))))"
let lisp_query = "(and (salary ?person ?amount) (lisp-value > ?amount 30000))"

let run_both env n text =
  let old_answers, old_count = measure Book env n text in
  let simple_answers, simple_count = measure Simple env n text in
  old_answers, old_count, simple_answers, simple_count
;;

let ex_4_74 () =
  let env = Eval.the_query_system () in
  load env;
  let old_answers, old_count, simple_answers, simple_count =
    run_both env 100 shared_query
  in
  old_answers
  @ [ "book_flatmap_frames=" ^ string_of_int old_count
    ; "simple_flatmap_frames=" ^ string_of_int simple_count
    ; "answers_equal=" ^ string_of_bool (old_answers = simple_answers)
    ]
;;

let ex_4_74a () =
  let env = Eval.the_query_system () in
  load env;
  let old_answers, old_count, simple_answers, simple_count =
    run_both env 100 shared_query
  in
  let lisp_old, lisp_old_count, lisp_simple, lisp_simple_count =
    run_both env 100 lisp_query
  in
  [ "query=" ^ shared_query
  ; "old_frames=" ^ string_of_int old_count
  ; "simple_frames=" ^ string_of_int simple_count
  ; "answers_equal=" ^ string_of_bool (old_answers = simple_answers)
  ; "counts_equal=" ^ string_of_bool (old_count = simple_count)
  ; "lisp_query=" ^ lisp_query
  ; "lisp_old_frames=" ^ string_of_int lisp_old_count
  ; "lisp_simple_frames=" ^ string_of_int lisp_simple_count
  ; "lisp_answers_equal=" ^ string_of_bool (lisp_old = lisp_simple)
  ; "lisp_counts_equal=" ^ string_of_bool (lisp_old_count = lisp_simple_count)
  ]
;;
