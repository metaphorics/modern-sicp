(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.57: the [can-replace] rule -- person 1 can replace person
    2 when their jobs coincide or person 1's job can do person 2's job
    per the data base's [can-do-job] assertions, and they are not the
    same person -- plus the two queries it exists for: everyone who can
    replace Cy D. Fect, and every replacement pair where the replaced
    person is paid more than the replacement, with both salaries. The
    rule needs the section's [same] rule for the identity exclusion. A
    person's own job never propagates transitively: [can-do-job] is a
    direct edge, so Ben replaces Hacker, Fect and Tweakit but not
    Reasoner's traineeship. With ?person-2 bound by the query, the
    [or]'s same-job disjunct streams before the [can-do-job] disjunct
    under the evaluator's interleaving, which is why Hacker's
    replacement of Fect precedes Ben's in the pinned output. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The Microshaft personnel data base of 4.4.1, in the book's order,
   then the exercise's additions: the section's [same] rule and the
   [can-replace] rule. *)
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
  ; "(assert! (rule (same ?x ?x)))"
  ; "(assert! (rule (can-replace ?person-1 ?person-2)\n\
     (and (job ?person-1 ?job-1)\n\
     (or (job ?person-2 ?job-1)\n\
     (and (can-do-job ?job-1 ?job-2)\n\
     (job ?person-2 ?job-2)))\n\
     (not (same ?person-1 ?person-2)))))"
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

(* One line per query -- the query text itself -- followed by one line
   per answer, the driver's display-stream. A query with no answers is
   just its own line. *)
let section env text =
  match Eval.query env text with
  | Ok answers -> text :: List.map Value.to_string answers
  | Error e -> [ text; "Error: " ^ Eval_error.to_string e ]
;;

let ex_4_57 () =
  let env = Eval.the_query_system () in
  load_microshaft env;
  List.concat_map
    (section env)
    [ "(can-replace ?x (Fect Cy D))"
    ; "(and (can-replace ?person-1 ?person-2) (salary ?person-1 ?salary-1) (salary \
       ?person-2 ?salary-2) (lisp-value < ?salary-1 ?salary-2))"
    ]
;;
