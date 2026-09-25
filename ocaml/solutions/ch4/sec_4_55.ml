(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.55: the three simple queries over the Microshaft data base
    -- everyone supervised by Ben Bitdiddle, the accounting division, and
    the Slumerville residents -- each answered from a fresh data base and
    pinned in the data base's insertion order. The demo also pins the
    book's [same] observation [(supervisor ?x ?x)] as empty.

    One deliberate deviation: the prescribed [(job ?name (accounting
    ?title))] matches only Cratchet, because the book's matcher is a
    strict pairwise walk -- [(accounting ?title)] cannot span the
    three-element [(accounting chief accountant)] exactly as [(computer
    ?type)] misses Reasoner's three-element job in the section's pinned
    transcript. The dotted form [(job ?name (accounting . ?title))]
    retrieves the whole division, so the demo pins both queries and the
    rationale spells out the difference. [(address ?name (Slumerville
    . ?where))] uses the same dotted form; the plain form would match
    nothing, every Slumerville address having a street and a number. *)

module Eval = Sicp_ch4.Sec_4_4
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

let ex_4_55 () =
  let env = Eval.the_query_system () in
  load_microshaft env;
  List.concat_map
    (section env)
    [ "(supervisor ?x (Bitdiddle Ben))"
    ; "(job ?name (accounting ?title))"
    ; "(job ?name (accounting . ?title))"
    ; "(address ?name (Slumerville . ?where))"
    ; "(supervisor ?x ?x)"
    ]
;;
