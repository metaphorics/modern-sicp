(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.76: [and] as a merge join instead of a series combination.
    The book's series [and] scans the data base once per frame the first
    conjunct produced; the merge strategy runs the two conjuncts
    separately over the incoming frames and then looks for every pair of
    output frames whose bindings are compatible -- [k] fewer matcher
    calls when each conjunct produces [n/k] frames. [merge_frames] is
    the compatibility check the exercise asks for: a fold of the
    substrate's exported [unify_match] over one frame's bindings onto
    the other, so two frames merge when their common variables agree
    and each contributes its own bindings to the union. The variant
    conjoin splits the incoming frame stream through the conjuncts
    independently and merges every pair. Because a pair may come from
    different incoming frames, the join is exact for the driver's one
    empty input frame (the book's setting) and a superset filter
    otherwise; the demos run on the empty frame, where the answers are
    identical to the series [and] -- pinned both ways. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Streams = Eval.Streams
module Value = Sicp_common.Value

(* The Microshaft assertions this demo needs: jobs and supervisors, in
   the book's order. *)
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
  ; "(assert! (job (Warbucks Oliver) (administration big wheel)))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (job (Scrooge Eben) (accounting chief accountant)))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (job (Cratchet Robert) (accounting scrivener)))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ; "(assert! (job (Aull DeWitt) (administration secretary)))"
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

(** [merge_frames f1 f2] is the union of the two frames' bindings when
    they are compatible, or [None]: every binding of [f1] is unified
    into [f2] with the substrate's [unify_match], so an [f1] variable
    already bound in [f2] must agree and the others are added. *)
let merge_frames f1 f2 =
  List.fold_left
    (fun acc (variable, value) ->
       match acc with
       | None -> None
       | Some frame -> Eval.unify_match variable value frame)
    (Some f2)
    (Eval.frame_bindings f1)
;;

(** [conjoin_merge env conjuncts frames checks] evaluates the conjuncts
    separately over the incoming frames and merges compatible pairs,
    counting one compatibility check per pair in [checks]. One
    conjunct degenerates to the plain evaluator; the empty conjunction
    passes the frames through. *)
let rec conjoin_merge env conjuncts frames checks =
  match conjuncts with
  | [] -> frames
  | [ only ] -> Eval.qeval env only frames
  | first :: rest ->
    let left = Eval.qeval env first frames in
    let right = conjoin_merge env rest frames checks in
    Eval.stream_flatmap
      (fun f1 ->
         Eval.stream_flatmap
           (fun f2 ->
              incr checks;
              match merge_frames f1 f2 with
              | Some merged -> Eval.singleton_stream merged
              | None -> Streams.the_empty_stream)
           right)
      left
;;

(** [conjunction_of query] is the conjunct list of an [and] query, or
    the typed error for any other shape. *)
let conjunction_of query =
  match Value.view query with
  | Value.Pair (head, body) when Value.structural_equal head (Value.symbol "and") ->
    Eval.value_list body
  | _ -> Error (Eval_error.Invalid_form "the merge engine answers and-queries")
;;

(** [run_merge env text checks] answers one query through the merge
    [and]: the rendered instantiations, or the typed error. *)
let run_merge env text checks =
  match Eval.read_query text with
  | Error message -> [ "Error: " ^ message ]
  | Ok raw ->
    let query = Eval.query_syntax_process raw in
    (match conjunction_of query with
     | Error e -> [ "Error: " ^ Eval_error.to_string e ]
     | Ok conjuncts ->
       let frames =
         conjoin_merge
           env
           conjuncts
           (Streams.cons_stream Eval.the_empty_frame (fun () -> Streams.the_empty_stream))
           checks
       in
       List.map
         (fun frame ->
            Value.to_string
              (Eval.instantiate query frame (fun v _ -> Eval.contract_question_mark v)))
         (stream_to_list frames))
;;

(** One query block: the query text, the series-[and] answers, the
    merge answers, and the equality verdict with the pair count. *)
let compare env text =
  let checks = ref 0 in
  let merged = run_merge env text checks in
  let series =
    match Eval.query env text with
    | Ok answers -> List.map Value.to_string answers
    | Error e -> [ "Error: " ^ Eval_error.to_string e ]
  in
  let identical = List.equal String.equal series merged in
  [ text ]
  @ series
  @ merged
  @ [ ("merge=series: " ^ if identical then "true" else "false")
    ; "compatibility checks: " ^ string_of_int !checks
    ]
;;

let ex_4_76 () =
  let env = Eval.the_query_system () in
  load_microshaft env;
  List.concat_map
    (compare env)
    [ "(and (job ?x (computer programmer)) (supervisor ?x ?boss))"
    ; "(and (supervisor ?x ?y) (job ?x ?job))"
    ]
;;
