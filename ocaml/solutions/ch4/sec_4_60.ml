(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.60: Alyssa's [lives-near] pair query. The book's rule
    matches each pair of neighbors in both orders -- the two [address]
    conjuncts are independent, so binding [?person-1] to Alyssa and
    [?person-2] to Cy is one solution and the swapped binding is
    another -- and the chronological data base lists both, eight answers
    for four pairs. The demonstration runs the ride-share query, the
    full pair query, and a deduplicated variant [lives-near-unique]
    that keeps one order per pair.

    Deliberate deviation: the classic dedup orders each pair by a name
    comparison through [lisp-value], but this substrate's [lisp-value]
    resolves its predicate only in the section 4.1 primitive table,
    which installs no symbol or string comparison -- only the integer
    [<]. The edition therefore orders each pair by salary through
    [(lisp-value < ?salary-1 ?salary-2)]; the strict [<] also excludes
    the self-pair the book's [not (same ...)] clause removed. Two
    same-town neighbors with equal salaries would still need the name
    comparison the book's driver installs. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The address and salary assertions of Microshaft, in the book's
   order, plus the book's [lives-near]/[same] rules and the deduplicated
   variant. *)
let assertions =
  [ "(assert! (address (Bitdiddle Ben) (Slumerville (Ridge Road) 10)))"
  ; "(assert! (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))"
  ; "(assert! (address (Fect Cy D) (Cambridge (Ames Street) 3)))"
  ; "(assert! (address (Tweakit Lem E) (Boston (Bay State Road) 22)))"
  ; "(assert! (address (Reasoner Louis) (Slumerville (Pine Tree Road) 80)))"
  ; "(assert! (address (Warbucks Oliver) (Swellesley (Top Heap Road))))"
  ; "(assert! (address (Scrooge Eben) (Weston (Shady Lane) 10)))"
  ; "(assert! (address (Cratchet Robert) (Allston (N Harvard Street) 16)))"
  ; "(assert! (address (Aull DeWitt) (Slumerville (Onion Square) 5)))"
  ; "(assert! (salary (Bitdiddle Ben) 60000))"
  ; "(assert! (salary (Hacker Alyssa P) 40000))"
  ; "(assert! (salary (Fect Cy D) 35000))"
  ; "(assert! (salary (Tweakit Lem E) 25000))"
  ; "(assert! (salary (Reasoner Louis) 30000))"
  ; "(assert! (salary (Warbucks Oliver) 150000))"
  ; "(assert! (salary (Scrooge Eben) 75000))"
  ; "(assert! (salary (Cratchet Robert) 18000))"
  ; "(assert! (salary (Aull DeWitt) 25000))"
  ; "(assert! (rule (lives-near ?person-1 ?person-2)\n\
    \     (and (address ?person-1 (?town . ?rest-1))\n\
    \          (address ?person-2 (?town . ?rest-2))\n\
    \          (not (same ?person-1 ?person-2)))))"
  ; "(assert! (rule (same ?x ?x)))"
  ; "(assert! (rule (lives-near-unique ?person-1 ?person-2)\n\
    \     (and (address ?person-1 (?town . ?rest-1))\n\
    \          (address ?person-2 (?town . ?rest-2))\n\
    \          (salary ?person-1 ?salary-1)\n\
    \          (salary ?person-2 ?salary-2)\n\
    \          (lisp-value < ?salary-1 ?salary-2))))"
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

let answers_or_fail = function
  | Ok answers -> List.map Value.to_string answers
  | Error e -> failwith ("query failed: " ^ Eval_error.to_string e)
;;

(** [ex_4_60 ()] pins the ride-share query, the full pair query with its
    duplicated pairs, and the deduplicated formulation's answers. *)
let ex_4_60 () =
  let env = Eval.the_query_system () in
  load env;
  let ride = answers_or_fail (Eval.query env "(lives-near ?person (Hacker Alyssa P))") in
  let pairs = answers_or_fail (Eval.query env "(lives-near ?person-1 ?person-2)") in
  let unique =
    answers_or_fail (Eval.query env "(lives-near-unique ?person-1 ?person-2)")
  in
  [ "query: (lives-near ?person (Hacker Alyssa P))" ]
  @ ride
  @ [ "query: (lives-near ?person-1 ?person-2)"
    ; "note: every pair appears twice, once per binding order of the two address \
       conjuncts"
    ]
  @ pairs
  @ [ "query: (lives-near-unique ?person-1 ?person-2)"
    ; "note: one order per pair, chosen by (lisp-value < ?salary-1 ?salary-2)"
    ]
  @ unique
;;
