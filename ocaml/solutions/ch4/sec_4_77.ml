(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.77: [Not] and [Holds] (the book's [lisp-value]) as delayed
   filters.  The section's filters are correct only on frames where the
   filter's variables are bound; applied earlier they give the wrong
   answer (the 4.4.3 example returns the empty stream) or an error.
   This solution attaches to each frame a promise to filter: the variant
   evaluator's frames carry the engine frame plus the pending filters
   accumulated so far.  A filter whose query is fully bound in the
   current frame is applied immediately -- the efficiency the exercise
   asks for; one with unbound variables rides on the frame and is
   fulfilled in the driver's settle step, after all other operations
   have run and bound what they could.  Settle runs the section's own
   filter on the one frame: a [Not] still unbound then matches and
   drops its frame, an unbound [Holds] is the typed error. *)

open Sec_4_55.Kit

(* One frame of the delayed engine: the engine frame plus the filter
   queries deferred so far, oldest first. *)
type dframe =
  { base : Q.frame
  ; pending : Q.query list
  }

type stats =
  { mutable deferred : int
  ; mutable fulfilled : int
  }

let rec term_vars acc = function
  | Q.Var variable -> variable :: acc
  | Q.Pair (a, b) -> term_vars (term_vars acc a) b
  | Q.Atom _ | Q.Num _ | Q.Str _ | Q.Nil -> acc
;;

let rec query_vars acc = function
  | Q.Pattern t -> term_vars acc t
  | Q.And qs | Q.Or qs | Q.Form (_, qs) -> List.fold_left query_vars acc qs
  | Q.Not q -> query_vars acc q
  | Q.Holds (_, ts) -> List.fold_left term_vars acc ts
  | Q.Always_true -> acc
;;

(* [fully_bound q frame] holds when the frame binds every variable the
   filter mentions -- enough to make the filtering possible. *)
let fully_bound q frame =
  List.for_all
    (fun variable -> Option.is_some (Q.binding_in_frame variable frame))
    (query_vars [] q)
;;

(* The section's filter on one frame. *)
let passes s filter frame =
  not (Streams.stream_null (Q.qeval s filter (Q.singleton_stream frame)))
;;

let rec dqeval s stats q frames =
  match q with
  | Q.And conjuncts ->
    List.fold_left (fun frames c -> dqeval s stats c frames) frames conjuncts
  | Q.Or disjuncts -> ddisjoin s stats disjuncts frames
  | Q.Not _ | Q.Holds _ -> dfilter s stats q frames
  | Q.Always_true -> frames
  | Q.Pattern _ | Q.Form _ ->
    Q.stream_flatmap
      (fun d ->
         Streams.stream_map
           (fun base -> { d with base })
           (Q.qeval s q (Q.singleton_stream d.base)))
      frames

and ddisjoin s stats disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    Q.interleave_delayed (dqeval s stats first frames) (fun () ->
      ddisjoin s stats rest frames)

and dfilter s stats filter frames =
  Q.stream_flatmap
    (fun d ->
       if fully_bound filter d.base
       then
         if passes s filter d.base then Q.singleton_stream d else Streams.the_empty_stream
       else (
         stats.deferred <- stats.deferred + 1;
         Q.singleton_stream { d with pending = d.pending @ [ filter ] }))
    frames
;;

(* [settle] fulfills the frame's promises oldest first, now that every
   other operation has run, with the section's own filter semantics: a
   [Not] is decidable on any frame -- its unbound variables simply
   match -- so a lone [Not] still answers empty; an unbound [Holds]
   raises the stock engine's typed error. *)
let settle s stats d =
  List.for_all
    (fun filter ->
       stats.fulfilled <- stats.fulfilled + 1;
       passes s filter d.base)
    d.pending
;;

let run_delayed s q =
  let stats = { deferred = 0; fulfilled = 0 } in
  let answers = Dynarray.create () in
  let outcome =
    match
      Streams.stream_for_each
        (fun d ->
           if settle s stats d
           then Dynarray.add_last answers (Q.render_query (Q.instantiate_query q d.base)))
        (dqeval s stats q (Q.singleton_stream { base = []; pending = [] }))
    with
    | () -> []
    | exception Q.Query_error e -> [ "error: " ^ Sicp_common.Eval_error.to_string e ]
  in
  Dynarray.to_list answers @ outcome, stats
;;

let compare s q =
  let delayed, stats = run_delayed s q in
  [ "? " ^ Q.render_query q; "naive:" ]
  @ answers_all s q
  @ [ "delayed:" ]
  @ delayed
  @ [ Printf.sprintf "deferred=%d fulfilled=%d" stats.deferred stats.fulfilled ]
;;

let demo_data =
  List.filter
    (function
      | Q.Pair (Q.Atom ("job" | "salary" | "supervisor"), _) -> true
      | _ -> false)
    microshaft
;;

let ex_4_77 () =
  let s = session demo_data in
  List.concat_map
    (compare s)
    [ Q.And
        [ Q.Not (p [ at "job"; v "x"; atoms [ "computer"; "programmer" ] ])
        ; p [ at "supervisor"; v "x"; v "y" ]
        ]
    ; Q.And
        [ Q.Holds (">", [ v "amount"; n 30000 ]); p [ at "salary"; v "who"; v "amount" ] ]
    ; Q.And
        [ p [ at "salary"; v "who"; v "amount" ]; Q.Holds (">", [ v "amount"; n 30000 ]) ]
    ]
;;
