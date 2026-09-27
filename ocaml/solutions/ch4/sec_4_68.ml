(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.68: rules for [reverse] over the [append-to-form] rules of
    4.4.1. The base rule covers the empty list; the recursive rule says
    the reverse of [(?u . ?v)] is the append of the reverse of [?v] with
    the one-element list [(?u)]. A forward query grounds the recursion,
    so it answers finitely. The backward query [(reverse ?x (1 2 3))]
    leaves both ends of the recursive subquery [(reverse ?v ?w)] unbound:
    the engine wades through the infinite stream of candidate pairs, the
    [append-to-form] conjunct filters them against the ground target, and
    the one true answer arrives first -- after which forcing further
    answers never returns. The backward direction is therefore observed
    with [query_upto]. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let assertions =
  [ "(assert! (rule (append-to-form () ?y ?y)))"
  ; "(assert! (rule (append-to-form (?u . ?v) ?y (?u . ?z))\n(append-to-form ?v ?y ?z)))"
  ; "(assert! (rule (reverse () ())))"
  ; "(assert! (rule (reverse (?u . ?v) ?z)\n\
     (and (reverse ?v ?w)\n\
     (append-to-form ?w (?u) ?z))))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "an assertion answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    assertions
;;

let render = function
  | Ok answers -> String.concat "; " (List.map Value.to_string answers)
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** The first answer of a stream that diverges after it:
    [query_upto] would force one answer past the prefix (its
    [stream_take] walks one [stream_cdr] before the count runs out), and
    on a stream with no second answer that never returns. [run] hands
    back the lazy stream; [stream_car] forces exactly one answer. *)
let first_answer env text =
  match Eval.run env text with
  | Ok (Eval.Answers stream) ->
    if Streams.stream_null stream
    then "no answers"
    else Value.to_string (Streams.stream_car stream)
  | Ok Eval.Asserted -> "asserted"
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let ex_4_68 () =
  let env = Eval.the_query_system () in
  load env;
  let forward3 = render (Eval.query env "(reverse (1 2 3) ?x)") in
  let forward4 = render (Eval.query env "(reverse (a b c d) ?x)") in
  let backward = first_answer env "(reverse ?x (1 2 3))" in
  [ "(reverse (1 2 3) ?x) => " ^ forward3
  ; "(reverse (a b c d) ?x) => " ^ forward4
  ; "(reverse ?x (1 2 3)) first answer => " ^ backward
  ; "backward: forcing a second answer does not return; with both ends of (reverse ?v \
     ?w) unbound the engine generates infinitely many candidate pairs and only (reverse \
     (3 2 1) (1 2 3)) survives the append-to-form filter"
  ]
;;
