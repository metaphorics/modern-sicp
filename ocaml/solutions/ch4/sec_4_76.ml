(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.76: [And] as a merge join instead of a series combination.
   The book's series [And] scans the data base once per frame the first
   conjunct produced; the merge strategy runs the two conjuncts
   separately over the incoming frames and then looks for every pair of
   output frames whose bindings are compatible.  [merge_frames] is the
   compatibility check the exercise asks for: a fold of the engine's
   [unify_match] over one frame's bindings onto the other, so two frames
   merge when their common variables agree and each contributes its own
   bindings to the union.  Because a pair may come from different
   incoming frames, the join is exact for the driver's one empty input
   frame (the book's setting) and a superset filter otherwise; the demos
   run on the empty frame, where the answers are identical to the series
   [And] -- pinned both ways, with the number of compatibility checks
   the join performed. *)

open Sec_4_55.Kit

let jobs_and_supervisors =
  List.filter
    (function
      | Q.Pair (Q.Atom ("job" | "supervisor"), _) -> true
      | _ -> false)
    microshaft
;;

let merge_frames f1 f2 =
  List.fold_left
    (fun acc (variable, value) -> Option.bind acc (Q.unify_match (Q.Var variable) value))
    (Some f2)
    (List.rev f1)
;;

(* The conjuncts run separately over the incoming frames and every pair
   of their outputs is checked; [checks] counts the pairs. *)
let rec conjoin_merge s conjuncts frames checks =
  match conjuncts with
  | [] -> frames
  | [ only ] -> Q.qeval s only frames
  | first :: rest ->
    let left = Q.qeval s first frames in
    let right = conjoin_merge s rest frames checks in
    Q.stream_flatmap
      (fun f1 ->
         Q.stream_flatmap
           (fun f2 ->
              incr checks;
              match merge_frames f1 f2 with
              | Some merged -> Q.singleton_stream merged
              | None -> Streams.the_empty_stream)
           right)
      left
;;

let run_merge s conjuncts checks =
  let q = Q.And conjuncts in
  let answers = Dynarray.create () in
  Streams.stream_for_each
    (fun frame ->
       Dynarray.add_last answers (Q.render_query (Q.instantiate_query q frame)))
    (conjoin_merge s conjuncts (Q.singleton_stream []) checks);
  Dynarray.to_list answers
;;

let compare s conjuncts =
  let checks = ref 0 in
  let merged = run_merge s conjuncts checks in
  let series = answers_all s (Q.And conjuncts) in
  [ "? " ^ Q.render_query (Q.And conjuncts) ]
  @ series
  @ merged
  @ [ "merge=series: " ^ string_of_bool (List.equal String.equal series merged)
    ; "compatibility checks: " ^ string_of_int !checks
    ]
;;

let ex_4_76 () =
  let s = session jobs_and_supervisors in
  compare
    s
    [ p [ at "job"; v "x"; atoms [ "computer"; "programmer" ] ]
    ; p [ at "supervisor"; v "x"; v "boss" ]
    ]
  @ compare s [ p [ at "supervisor"; v "x"; v "y" ]; p [ at "job"; v "x"; v "job" ] ]
;;
