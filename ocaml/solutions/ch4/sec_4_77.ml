(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.77: [not] and [lisp-value] as delayed filters. The
    section's filters are correct only on frames where the filter's
    variables are bound; applied earlier they give the wrong answer
    (the 4.4.3 example returns the empty stream) or an error. This
    solution attaches to each frame a promise to filter: the variant
    evaluator's frames carry the substrate frame plus the pending
    [not]/[lisp-value] filters accumulated so far. A filter whose
    query is fully bound in the current frame is applied immediately
    -- the efficiency the exercise asks for; one with unbound
    variables rides on the frame and is fulfilled in the driver's
    settle step, after all other operations have run and bound what
    they could. A promise still unfulfillable at settle time cannot
    decide its frame; the frame is kept and counted as unresolved. The
    substrate's own [negate]/[execute] do the settled filtering, so
    the fulfilled semantics are the section's exactly. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Streams = Eval.Streams
module Value = Sicp_common.Value

(* One filter promise: the negated query, or the host-predicate call. *)
type filter =
  | Not_filter of Value.t
  | Lisp_filter of Value.t

(* One frame of the delayed engine: the substrate frame plus the
   filters deferred so far, oldest first. *)
type dframe =
  { base : Eval.frame
  ; pending : filter list
  }

(* Typed errors escape the evaluator to the driver's rendering. *)
exception Run_error of Eval_error.t

(* The Microshaft assertions this demo needs, in the book's order. *)
let assertions =
  [ "(assert! (job (Bitdiddle Ben) (computer wizard)))"
  ; "(assert! (salary (Bitdiddle Ben) 60000))"
  ; "(assert! (job (Hacker Alyssa P) (computer programmer)))"
  ; "(assert! (salary (Hacker Alyssa P) 40000))"
  ; "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (job (Fect Cy D) (computer programmer)))"
  ; "(assert! (salary (Fect Cy D) 35000))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (job (Tweakit Lem E) (computer technician)))"
  ; "(assert! (salary (Tweakit Lem E) 25000))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (job (Reasoner Louis) (computer programmer trainee)))"
  ; "(assert! (salary (Reasoner Louis) 30000))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (salary (Warbucks Oliver) 150000))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (job (Scrooge Eben) (accounting chief accountant)))"
  ; "(assert! (salary (Scrooge Eben) 75000))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (job (Cratchet Robert) (accounting scrivener)))"
  ; "(assert! (salary (Cratchet Robert) 18000))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ; "(assert! (job (Aull DeWitt) (administration secretary)))"
  ; "(assert! (salary (Aull DeWitt) 25000))"
  ]
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

let stream_to_list s =
  let rec go s acc =
    if Streams.stream_null s
    then List.rev acc
    else go (Streams.stream_cdr s) (Streams.stream_car s :: acc)
  in
  go s []
;;

(* [vars_in exp] is every pattern variable the expression mentions. *)
let rec vars_in exp acc =
  if Eval.is_var exp
  then exp :: acc
  else (
    match Value.view exp with
    | Value.Pair (car, cdr) -> vars_in car (vars_in cdr acc)
    | _ -> acc)
;;

(* [filter_query filter] is the query whose bindings the filter reads. *)
let filter_query = function
  | Not_filter query -> query
  | Lisp_filter call -> call
;;

(* [fully_bound query frame] holds when the frame binds every variable
   the query mentions -- enough to make the filtering possible. *)
let fully_bound query frame =
  List.for_all
    (fun var -> Option.is_some (Eval.binding_in_frame var frame))
    (vars_in query [])
;;

(* The counters of the current delayed query: promises attached,
   promises fulfilled at settle time, promises still unfulfillable.
   Reset by [run_delayed], read by the demo after each query. *)
type stats =
  { deferred : int ref
  ; fulfilled : int ref
  ; unresolved : int ref
  }

let stats = { deferred = ref 0; fulfilled = ref 0; unresolved = ref 0 }

(** [dqeval env query frames] is the delayed evaluator: the section's
    dispatch with [not] and [lisp-value] filtered when possible and
    deferred otherwise. *)
let rec dqeval env query frames =
  match Value.view query with
  | Value.Pair (head, _) when Value.structural_equal head (Value.symbol "and") ->
    dconjoin env (contents query) frames
  | Value.Pair (head, _) when Value.structural_equal head (Value.symbol "or") ->
    ddisjoin env (contents query) frames
  | Value.Pair (head, _) when Value.structural_equal head (Value.symbol "not") ->
    dnot env (contents query) frames
  | Value.Pair (head, _) when Value.structural_equal head (Value.symbol "lisp-value") ->
    dlisp env (contents query) frames
  | _ -> dsimple env query frames

and contents query =
  match Value.view query with
  | Value.Pair (_, body) ->
    (match Eval.value_list body with
     | Ok items -> items
     | Error e -> raise (Run_error e))
  | _ -> raise (Run_error (Eval_error.Invalid_form (Value.to_string query)))

and dconjoin env conjuncts frames =
  match conjuncts with
  | [] -> frames
  | first :: rest -> dconjoin env rest (dqeval env first frames)

and ddisjoin env disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    Eval.interleave_delayed (dqeval env first frames) (fun () -> ddisjoin env rest frames)

and dsimple env pattern frames =
  Eval.stream_flatmap
    (fun d ->
       Streams.stream_map
         (fun base -> { base; pending = d.pending })
         (Eval.qeval env pattern (Eval.singleton_stream d.base)))
    frames

and dnot env operands frames =
  let query = Eval.first_operand operands in
  Eval.stream_flatmap
    (fun d ->
       if fully_bound query d.base
       then
         if Streams.stream_null (Eval.qeval env query (Eval.singleton_stream d.base))
         then Eval.singleton_stream d
         else Streams.the_empty_stream
       else (
         incr stats.deferred;
         Eval.singleton_stream { d with pending = d.pending @ [ Not_filter query ] }))
    frames

and dlisp env operands frames =
  (* The book's convention: the operands of (lisp-value ...) are the
     elements of the call. *)
  let call = List.fold_right Value.pair operands Value.nil in
  Eval.stream_flatmap
    (fun d ->
       if fully_bound call d.base
       then (
         let instantiated = Eval.instantiate call d.base (fun v _ -> v) in
         match Eval.execute env instantiated with
         | Ok true -> Eval.singleton_stream d
         | Ok false -> Streams.the_empty_stream
         | Error e -> raise (Run_error e))
       else (
         incr stats.deferred;
         Eval.singleton_stream { d with pending = d.pending @ [ Lisp_filter call ] }))
    frames

(** [settle env d] fulfills the frame's promises oldest first,
    now that every other operation has run: a decidable filter keeps
    or drops the frame, an undecidable one is counted unresolved and
    the frame is kept. *)
and settle env d =
  List.fold_left
    (fun acc filter ->
       match acc with
       | None -> None
       | Some base ->
         let query = filter_query filter in
         if fully_bound query base
         then (
           incr stats.fulfilled;
           match filter with
           | Not_filter q ->
             if Streams.stream_null (Eval.qeval env q (Eval.singleton_stream base))
             then Some base
             else None
           | Lisp_filter call ->
             let instantiated = Eval.instantiate call base (fun v _ -> v) in
             (match Eval.execute env instantiated with
              | Ok true -> Some base
              | Ok false -> None
              | Error e -> raise (Run_error e)))
         else (
           incr stats.unresolved;
           Some base))
    (Some d.base)
    d.pending
;;

(** [run_delayed env text] answers one query through the delayed
    engine: the rendered instantiations, or the typed error. The
    promise counters are reset for the query. *)
let run_delayed env text =
  stats.deferred := 0;
  stats.fulfilled := 0;
  stats.unresolved := 0;
  match Eval.read_query text with
  | Error message -> [ "Error: " ^ message ]
  | Ok raw ->
    let query = Eval.query_syntax_process raw in
    (try
       let frames =
         dqeval
           env
           query
           (Streams.cons_stream { base = Eval.the_empty_frame; pending = [] } (fun () ->
              Streams.the_empty_stream))
       in
       List.filter_map
         (fun d ->
            match settle env d with
            | Some base ->
              Some
                (Value.to_string
                   (Eval.instantiate query base (fun v _ -> Eval.contract_question_mark v)))
            | None -> None)
         (stream_to_list frames)
     with
     | Run_error e -> [ "Error: " ^ Eval_error.to_string e ])
;;

(** One query block: the naive section's answer (or error), the
    delayed engine's answer, and the promise counters. *)
let compare env text =
  let delayed = run_delayed env text in
  let naive =
    match Eval.query env text with
    | Ok answers -> List.map Value.to_string answers
    | Error e -> [ "Error: " ^ Eval_error.to_string e ]
  in
  [ text; "naive:" ]
  @ naive
  @ [ "delayed:" ]
  @ delayed
  @ [ "deferred="
      ^ string_of_int !(stats.deferred)
      ^ " fulfilled="
      ^ string_of_int !(stats.fulfilled)
      ^ " unresolved="
      ^ string_of_int !(stats.unresolved)
    ]
;;

let ex_4_77 () =
  let env = Eval.the_query_system () in
  load_microshaft env;
  List.concat_map
    (compare env)
    [ "(and (not (job ?x (computer programmer))) (supervisor ?x ?y))"
    ; "(and (lisp-value > ?amount 30000) (salary ?who ?amount))"
    ; "(and (salary ?who ?amount) (lisp-value > ?amount 30000))"
    ]
;;
