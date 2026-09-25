(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.69: ``greats'' relationships over the Genesis data base of
    4.63. A relationship is a list ending in the word [grandson]:
    [ends-in-grandson] recognizes exactly those lists, the bridge rule
    ties the one-element relationship [(grandson)] to the two-slot
    [grandson] relation of 4.63, and the greats rule derives
    [((great . ?rel) ?x ?y)] by peeling one [great] off the relationship
    and one generation off the [son] chain. Without the
    [ends-in-grandson] guard the recursion could bind the relationship to
    a dotted tail, so the guard is the rule's first conjunct. Queries
    with a ground relationship answer finitely; [(?relationship Adam
    Irad)] leaves [?rel] unbound, the guard then generates relationship
    lists of every depth, and the query is read with [query_upto]: the
    true answer is first, the deeper candidates all fail their [son]
    check in bounded time, and the generation never runs out. *)

module Eval = Sicp_ch4.Sec_4_4
module Streams = Eval.Streams
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let assertions =
  [ "(assert! (son Adam Cain))"
  ; "(assert! (son Cain Enoch))"
  ; "(assert! (son Enoch Irad))"
  ; "(assert! (son Irad Mehujael))"
  ; "(assert! (son Mehujael Methushael))"
  ; "(assert! (son Methushael Lamech))"
  ; "(assert! (wife Lamech Ada))"
  ; "(assert! (son Ada Jabal))"
  ; "(assert! (son Ada Jubal))"
  ; "(assert! (rule (son ?m ?s)\n(and (wife ?m ?w)\n(son ?w ?s))))"
  ; "(assert! (rule (grandson ?g ?s)\n(and (son ?g ?f)\n(son ?f ?s))))"
  ; "(assert! (rule (ends-in-grandson (grandson))))"
  ; "(assert! (rule (ends-in-grandson (great . ?rest))\n(ends-in-grandson ?rest)))"
  ; "(assert! (rule ((grandson) ?x ?y)\n(grandson ?x ?y)))"
  ; "(assert! (rule ((great . ?rel) ?x ?y)\n\
     (and (ends-in-grandson ?rel)\n\
     (?rel ?x ?z)\n\
     (son ?z ?y))))"
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
  | Ok answers -> List.map Value.to_string answers
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
;;

(** The first answer of a stream that diverges after it: [query_upto]
    would force one answer past the prefix (its [stream_take] walks one
    [stream_cdr] before the count runs out), and with [?relationship]
    unbound the ends-in-grandson generator keeps producing deeper
    relationship lists, so a second answer never arrives. [run] hands
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

let ex_4_69 () =
  let env = Eval.the_query_system () in
  load env;
  let greats = render (Eval.query env "((great grandson) ?g ?ggs)") in
  let irad = first_answer env "(?relationship Adam Irad)" in
  let fifth =
    render (Eval.query env "((great great great great great grandson) Adam ?d)")
  in
  List.map (fun line -> "((great grandson) ?g ?ggs) => " ^ line) greats
  @ [ "(?relationship Adam Irad) first answer => " ^ irad
    ; "note: with ?relationship unbound the ends-in-grandson generator produces \
       relationship lists of every depth; the first answer is the true one and a full \
       forcing would not return"
    ]
  @ List.map
      (fun line -> "((great great great great great grandson) Adam ?d) => " ^ line)
      fifth
;;
