(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.61: the book's [next-to] rules, verbatim, and the two
    queries of the statement. The engine tries the candidate rules in
    insertion order, so the base case answers first; rule 2's body then
    re-enters the same rule pair at the list tail, and the recursive
    descent reports each deeper adjacency on the way out -- the pairs
    of the whole list before the pairs of its tail, innermost last.
    The displayed answers instantiate the queried pattern, so every
    answer shows its adjacency in the original list. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let rules =
  [ "(assert! (rule (?x next-to ?y in (?x ?y . ?u))))"
  ; "(assert! (rule (?x next-to ?y in (?v . ?z))\n     (?x next-to ?y in ?z)))"
  ]
;;

let load env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "a rule answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    rules
;;

let answers_or_fail = function
  | Ok answers -> List.map Value.to_string answers
  | Error e -> failwith ("query failed: " ^ Eval_error.to_string e)
;;

(** [ex_4_61 ()] pins the two statement queries under the engine's rule
    order. *)
let ex_4_61 () =
  let env = Eval.the_query_system () in
  load env;
  let nested = answers_or_fail (Eval.query env "(?x next-to ?y in (1 (2 3) 4))") in
  let ones = answers_or_fail (Eval.query env "(?x next-to 1 in (2 1 3 1))") in
  [ "query: (?x next-to ?y in (1 (2 3) 4))" ]
  @ nested
  @ [ "query: (?x next-to 1 in (2 1 3 1))" ]
  @ ones
  @ [ "note: rules are tried in insertion order; rule 2 recurses on the list tail, so \
       the base case of the whole list answers first and the innermost adjacency last"
    ]
;;
