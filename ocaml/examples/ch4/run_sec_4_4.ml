(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error

let expect = Sicp_ch1.Replay.expect

(* The Microshaft personnel data base of 4.4.1, in the book's order, and
   the rules the section's prose introduces. *)
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
  ; "(assert! (rule (lives-near ?person-1 ?person-2)\n\
     (and (address ?person-1 (?town . ?rest-1))\n\
     (address ?person-2 (?town . ?rest-2))\n\
     (not (same ?person-1 ?person-2)))))"
  ; "(assert! (rule (same ?x ?x)))"
  ; "(assert! (rule (wheel ?person)\n\
     (and (supervisor ?middle-manager ?person)\n\
     (supervisor ?x ?middle-manager))))"
  ; "(assert! (rule (outranked-by ?staff-person ?boss)\n\
     (or (supervisor ?staff-person ?boss)\n\
     (and (supervisor ?staff-person ?middle-manager)\n\
     (outranked-by ?middle-manager ?boss)))))"
  ; "(assert! (rule (append-to-form () ?y ?y)))"
  ; "(assert! (rule (append-to-form (?u . ?v) ?y (?u . ?z))\n(append-to-form ?v ?y ?z)))"
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

(* One line per answer, the driver's display-stream; the expected block is
   written one answer per list element. *)
let expect_query env text expected =
  let got =
    match Eval.query env text with
    | Ok answers -> String.concat "\n" (List.map Sicp_common.Value.to_string answers)
    | Error e -> "Error: " ^ Eval_error.to_string e
  in
  expect got (String.concat "\n" expected)
;;

let () =
  let env = Eval.the_query_system () in
  load_microshaft env;
  (* 4.4.1: simple queries, the book's pinned results in the data base's
     insertion order. *)
  expect_query
    env
    "(job ?x (computer programmer))"
    [ "(job (Hacker Alyssa P) (computer programmer))"
    ; "(job (Fect Cy D) (computer programmer))"
    ];
  expect_query
    env
    "(job ?x (computer ?type))"
    [ "(job (Bitdiddle Ben) (computer wizard))"
    ; "(job (Hacker Alyssa P) (computer programmer))"
    ; "(job (Fect Cy D) (computer programmer))"
    ; "(job (Tweakit Lem E) (computer technician))"
    ];
  expect_query env "(supervisor ?x ?x)" [];
  (* Compound queries: and in series, or with its pairwise interleave,
     not as a filter, lisp-value against the host predicate. *)
  expect_query
    env
    "(and (job ?person (computer programmer)) (address ?person ?where))"
    [ "(and (job (Hacker Alyssa P) (computer programmer)) (address (Hacker Alyssa P) \
       (Cambridge (Mass Ave) 78)))"
    ; "(and (job (Fect Cy D) (computer programmer)) (address (Fect Cy D) (Cambridge \
       (Ames Street) 3)))"
    ];
  (* The or transcripts: the book's prose sample lists the first
     disjunct's answers before the second's, but the implementation the
     book presents -- and exercises 4.71 to 4.73 analyze -- interleaves
     the disjunct streams, so Reasoner (Hacker's only supervisee) lands
     in the second slot. *)
  expect_query
    env
    "(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))"
    [ "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker Alyssa P) \
       (Hacker Alyssa P)))"
    ; "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner Louis) \
       (Hacker Alyssa P)))"
    ; "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) (Hacker \
       Alyssa P)))"
    ; "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem E) \
       (Hacker Alyssa P)))"
    ];
  expect_query
    env
    "(and (supervisor ?x ?y) (not (job ?x (computer programmer))))"
    [ "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (not (job (Tweakit Lem E) \
       (computer programmer))))"
    ; "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (not (job (Reasoner Louis) \
       (computer programmer))))"
    ; "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (not (job (Bitdiddle Ben) \
       (computer programmer))))"
    ; "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (not (job (Scrooge Eben) \
       (computer programmer))))"
    ; "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (not (job (Cratchet Robert) \
       (computer programmer))))"
    ; "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (not (job (Aull DeWitt) \
       (computer programmer))))"
    ];
  expect_query
    env
    "(and (salary ?person ?amount) (lisp-value > ?amount 30000))"
    [ "(and (salary (Bitdiddle Ben) 60000) (lisp-value > 60000 30000))"
    ; "(and (salary (Hacker Alyssa P) 40000) (lisp-value > 40000 30000))"
    ; "(and (salary (Fect Cy D) 35000) (lisp-value > 35000 30000))"
    ; "(and (salary (Warbucks Oliver) 150000) (lisp-value > 150000 30000))"
    ; "(and (salary (Scrooge Eben) 75000) (lisp-value > 75000 30000))"
    ];
  (* Rules: lives-near over Ben, append-to-form in all three directions. *)
  expect_query
    env
    "(lives-near ?x (Bitdiddle Ben))"
    [ "(lives-near (Reasoner Louis) (Bitdiddle Ben))"
    ; "(lives-near (Aull DeWitt) (Bitdiddle Ben))"
    ];
  expect_query
    env
    "(and (job ?x (computer programmer)) (lives-near ?x (Bitdiddle Ben)))"
    [];
  expect_query
    env
    "(append-to-form (a b) (c d) ?z)"
    [ "(append-to-form (a b) (c d) (a b c d))" ];
  expect_query
    env
    "(append-to-form (a b) ?y (a b c d))"
    [ "(append-to-form (a b) (c d) (a b c d))" ];
  expect_query
    env
    "(append-to-form ?x ?y (a b c d))"
    [ "(append-to-form () (a b c d) (a b c d))"
    ; "(append-to-form (a) (b c d) (a b c d))"
    ; "(append-to-form (a b) (c d) (a b c d))"
    ; "(append-to-form (a b c) (d) (a b c d))"
    ; "(append-to-form (a b c d) () (a b c d))"
    ];
  (* The driver's other input: an assertion is added, not answered. *)
  match Eval.run env "(assert! (meeting whole-company (Wednesday 4pm)))" with
  | Ok Eval.Asserted ->
    expect "Assertion added to data base." "Assertion added to data base."
  | Ok (Eval.Answers _) -> expect "unexpected answers" "Assertion added to data base."
  | Error e -> expect ("Error: " ^ Eval_error.to_string e) "Assertion added to data base."
;;
