(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the query system of section 4.4: the substrate's own
   contract (the book's pinned Microshaft interactions, the driver protocol,
   the matcher and unifier cases of 4.4.2, the query syntax procedures) and
   every exercise's demonstration pinned to the exact observable outcomes
   the reference solutions produce. *)

let check_strings = Alcotest.(check (list string))
let check_string = Alcotest.(check string)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(* The Microshaft data base of 4.4.1 and the prose rules, in insertion
   order. *)
let microshaft =
  List.map
    (fun s -> "(assert! " ^ s ^ ")")
    [ "(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10))"
    ; "(job (Bitdiddle Ben) (computer wizard))"
    ; "(salary (Bitdiddle Ben) 60000)"
    ; "(address (Hacker Alyssa P) (Cambridge (Mass Ave) 78))"
    ; "(job (Hacker Alyssa P) (computer programmer))"
    ; "(salary (Hacker Alyssa P) 40000)"
    ; "(supervisor (Hacker Alyssa P) (Bitdiddle Ben))"
    ; "(address (Fect Cy D) (Cambridge (Ames Street) 3))"
    ; "(job (Fect Cy D) (computer programmer))"
    ; "(salary (Fect Cy D) 35000)"
    ; "(supervisor (Fect Cy D) (Bitdiddle Ben))"
    ; "(address (Tweakit Lem E) (Boston (Bay State Road) 22))"
    ; "(job (Tweakit Lem E) (computer technician))"
    ; "(salary (Tweakit Lem E) 25000)"
    ; "(supervisor (Tweakit Lem E) (Bitdiddle Ben))"
    ; "(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80))"
    ; "(job (Reasoner Louis) (computer programmer trainee))"
    ; "(salary (Reasoner Louis) 30000)"
    ; "(supervisor (Reasoner Louis) (Hacker Alyssa P))"
    ; "(supervisor (Bitdiddle Ben) (Warbucks Oliver))"
    ; "(address (Warbucks Oliver) (Swellesley (Top Heap Road)))"
    ; "(job (Warbucks Oliver) (administration big wheel))"
    ; "(salary (Warbucks Oliver) 150000)"
    ; "(address (Scrooge Eben) (Weston (Shady Lane) 10))"
    ; "(job (Scrooge Eben) (accounting chief accountant))"
    ; "(salary (Scrooge Eben) 75000)"
    ; "(supervisor (Scrooge Eben) (Warbucks Oliver))"
    ; "(address (Cratchet Robert) (Allston (N Harvard Street) 16))"
    ; "(job (Cratchet Robert) (accounting scrivener))"
    ; "(salary (Cratchet Robert) 18000)"
    ; "(supervisor (Cratchet Robert) (Scrooge Eben))"
    ; "(address (Aull DeWitt) (Slumerville (Onion Square) 5))"
    ; "(job (Aull DeWitt) (administration secretary))"
    ; "(salary (Aull DeWitt) 25000)"
    ; "(supervisor (Aull DeWitt) (Warbucks Oliver))"
    ; "(can-do-job (computer wizard) (computer programmer))"
    ; "(can-do-job (computer wizard) (computer technician))"
    ; "(can-do-job (computer programmer) (computer programmer trainee))"
    ; "(can-do-job (administration secretary) (administration big wheel))"
    ]
;;

let prose_rules =
  List.map
    (fun s -> "(assert! " ^ s ^ ")")
    [ "(rule (lives-near ?person-1 ?person-2)\n\
      \      (and (address ?person-1 (?town . ?rest-1))\n\
      \           (address ?person-2 (?town . ?rest-2))\n\
      \           (not (same ?person-1 ?person-2))))"
    ; "(rule (same ?x ?x))"
    ; "(rule (wheel ?person)\n\
      \      (and (supervisor ?middle-manager ?person)\n\
      \           (supervisor ?x ?middle-manager)))"
    ; "(rule (outranked-by ?staff-person ?boss)\n\
      \      (or (supervisor ?staff-person ?boss)\n\
      \          (and (supervisor ?staff-person ?middle-manager)\n\
      \               (outranked-by ?middle-manager ?boss))))"
    ; "(rule (append-to-form () ?y ?y))"
    ; "(rule (append-to-form (?u . ?v) ?y (?u . ?z))\n      (append-to-form ?v ?y ?z))"
    ]
;;

let load env items =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "an assertion answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    items
;;

let answers_to_string answers = String.concat "\n" (List.map Value.to_string answers)

(* Run one query and pin its full output block; one expected line per
   answer. *)
let expect_query label env text expected =
  match Eval.query env text with
  | Ok answers ->
    check_string label (String.concat "\n" expected) (answers_to_string answers)
  | Error e ->
    check_string label (String.concat "\n" expected) ("Error: " ^ Eval_error.to_string e)
;;

(* The data-language reader and the syntax procedures of 4.4.4.7. *)
let test_syntax () =
  let show = function
    | Ok v -> Value.to_string v
    | Error m -> "read error: " ^ m
  in
  check_string
    "reader: assertion with bang"
    "(assert! (job (Bitdiddle Ben) (computer wizard)))"
    (show (Eval.read_query "(assert! (job (Bitdiddle Ben) (computer wizard)))"));
  check_string
    "reader: empty list datum"
    "(rule (append-to-form () ?y ?y))"
    (show (Eval.read_query "(rule (append-to-form () ?y ?y))"));
  check_string
    "reader: dotted pattern"
    "(computer . ?type)"
    (show (Eval.read_query "(computer . ?type)"));
  check_string "reader: symbol with digits" "9am" (show (Eval.read_query "9am"));
  check_string "reader: integer" "60000" (show (Eval.read_query "60000"));
  check_string
    "reader: comment then datum"
    "(job ?x ?y)"
    (show (Eval.read_query ";; the comment\n(job ?x ?y)"));
  (* query-syntax-process expands ?x to the internal (? x). *)
  let internal =
    match Eval.read_query "(job ?x ?y)" with
    | Ok v -> Value.to_string (Eval.query_syntax_process v)
    | Error m -> "read error: " ^ m
  in
  check_string "syntax process" "(job (? x) (? y))" internal;
  (* contract-question-mark rounds the internal forms back. *)
  let v name = Value.pair (Value.symbol "?") (Value.pair (Value.symbol name) Value.nil) in
  check_string "contract ?x" "?x" (Value.to_string (Eval.contract_question_mark (v "x")));
  let renamed = Eval.make_new_variable (v "x") 7 in
  check_string "make new variable" "(? 7 x)" (Value.to_string renamed);
  check_string
    "contract renamed"
    "?x-7"
    (Value.to_string (Eval.contract_question_mark renamed))
;;

(* The pattern matcher and unifier cases the book walks through in
   4.4.2. *)
let test_matcher () =
  let sym s = Value.symbol s in
  let lst items = List.fold_right Value.pair items Value.nil in
  let var name =
    (* the internal variable form [(? name)] *)
    Value.pair (Value.symbol "?") (Value.pair (Value.symbol name) Value.nil)
  in
  (* ((a b) c (a b)) matches (?x c ?x) with ?x bound to (a b). *)
  let data = lst [ lst [ sym "a"; sym "b" ]; sym "c"; lst [ sym "a"; sym "b" ] ] in
  let pattern = lst [ var "x"; sym "c"; var "x" ] in
  (match Eval.pattern_match pattern data Eval.the_empty_frame with
   | Some frame ->
     (match Eval.binding_in_frame (var "x") frame with
      | Some (_, bound) ->
        check_string "matcher repeated var" "(a b)" (Value.to_string bound)
      | None -> Alcotest.fail "?x unbound")
   | None -> Alcotest.fail "match failed");
  (* ((a b) c (a b)) does not match (?x a ?y). *)
  let pattern2 = lst [ var "x"; sym "a"; var "y" ] in
  Alcotest.check
    Alcotest.bool
    "matcher rejects"
    false
    (match Eval.pattern_match pattern2 data Eval.the_empty_frame with
     | Some _ -> true
     | None -> false);
  (* Unifying (?x a ?y) with (?y ?z a) binds all three to a. *)
  let p1 = lst [ var "x"; sym "a"; var "y" ] in
  let p2 = lst [ var "y"; var "z"; sym "a" ] in
  (match Eval.unify_match p1 p2 Eval.the_empty_frame with
   | Some frame ->
     let bindings =
       List.sort
         String.compare
         (List.map
            (fun (v, value) ->
               Value.to_string (Eval.contract_question_mark v)
               ^ " = "
               ^
               if Eval.is_var value
               then Value.to_string (Eval.contract_question_mark value)
               else Value.to_string value)
            (Eval.frame_bindings frame))
     in
     (* ?x is bound to ?y, ?y and ?z to a: resolving the chain makes all
        three a, exactly the book's frame. *)
     check_string "unify frame" "?x = ?y, ?y = a, ?z = a" (String.concat ", " bindings)
   | None -> Alcotest.fail "unify failed");
  (* Unifying (?x ?y a) with (?x b ?y) fails. *)
  let p3 = lst [ var "x"; var "y"; sym "a" ] in
  let p4 = lst [ var "x"; sym "b"; var "y" ] in
  Alcotest.check
    Alcotest.bool
    "unify rejects"
    false
    (match Eval.unify_match p3 p4 Eval.the_empty_frame with
     | Some _ -> true
     | None -> false);
  (* depends_on sees through frame bindings: ?y is bound to ?x, so the
     expression ?y depends on ?x. *)
  let frame = Eval.extend (var "x") (var "y") Eval.the_empty_frame in
  Alcotest.check
    Alcotest.bool
    "depends on direct"
    false
    (Eval.depends_on (var "y") (var "x") frame);
  let chained = Eval.extend (var "y") (var "x") frame in
  Alcotest.check
    Alcotest.bool
    "depends on through frame"
    true
    (Eval.depends_on (var "y") (var "x") chained);
  ()
;;

(* The driver: the book's pinned Microshaft interactions. *)
let test_driver () =
  let env = Eval.the_query_system () in
  load env (microshaft @ prose_rules);
  expect_query
    "simple query"
    env
    "(job ?x (computer programmer))"
    [ "(job (Hacker Alyssa P) (computer programmer))"
    ; "(job (Fect Cy D) (computer programmer))"
    ];
  expect_query
    "pattern with nested var"
    env
    "(job ?x (computer ?type))"
    [ "(job (Bitdiddle Ben) (computer wizard))"
    ; "(job (Hacker Alyssa P) (computer programmer))"
    ; "(job (Fect Cy D) (computer programmer))"
    ; "(job (Tweakit Lem E) (computer technician))"
    ];
  expect_query "no self-supervision" env "(supervisor ?x ?x)" [];
  expect_query
    "and in series"
    env
    "(and (job ?person (computer programmer)) (address ?person ?where))"
    [ "(and (job (Hacker Alyssa P) (computer programmer)) (address (Hacker Alyssa P) \
       (Cambridge (Mass Ave) 78)))"
    ; "(and (job (Fect Cy D) (computer programmer)) (address (Fect Cy D) (Cambridge \
       (Ames Street) 3)))"
    ];
  expect_query
    "or interleaves"
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
    "not filters"
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
    "lisp-value filter"
    env
    "(and (salary ?person ?amount) (lisp-value > ?amount 30000))"
    [ "(and (salary (Bitdiddle Ben) 60000) (lisp-value > 60000 30000))"
    ; "(and (salary (Hacker Alyssa P) 40000) (lisp-value > 40000 30000))"
    ; "(and (salary (Fect Cy D) 35000) (lisp-value > 35000 30000))"
    ; "(and (salary (Warbucks Oliver) 150000) (lisp-value > 150000 30000))"
    ; "(and (salary (Scrooge Eben) 75000) (lisp-value > 75000 30000))"
    ];
  expect_query
    "rule over Ben"
    env
    "(lives-near ?x (Bitdiddle Ben))"
    [ "(lives-near (Reasoner Louis) (Bitdiddle Ben))"
    ; "(lives-near (Aull DeWitt) (Bitdiddle Ben))"
    ];
  expect_query
    "compound with rule"
    env
    "(and (job ?x (computer programmer)) (lives-near ?x (Bitdiddle Ben)))"
    [];
  expect_query
    "append forward"
    env
    "(append-to-form (a b) (c d) ?z)"
    [ "(append-to-form (a b) (c d) (a b c d))" ];
  expect_query
    "append backward"
    env
    "(append-to-form (a b) ?y (a b c d))"
    [ "(append-to-form (a b) (c d) (a b c d))" ];
  expect_query
    "append both"
    env
    "(append-to-form ?x ?y (a b c d))"
    [ "(append-to-form () (a b c d) (a b c d))"
    ; "(append-to-form (a) (b c d) (a b c d))"
    ; "(append-to-form (a b) (c d) (a b c d))"
    ; "(append-to-form (a b c) (d) (a b c d))"
    ; "(append-to-form (a b c d) () (a b c d))"
    ];
  (* An assert! adds instead of answering. *)
  (match Eval.run env "(assert! (meeting whole-company (Wednesday 4pm)))" with
   | Ok Eval.Asserted -> check_string "assert adds" "Asserted" "Asserted"
   | Ok (Eval.Answers _) -> check_string "assert adds" "Asserted" "answered"
   | Error e -> check_string "assert adds" "Asserted" ("Error: " ^ Eval_error.to_string e));
  (* A malformed query answers the typed error. *)
  match Eval.query env "(job ?x" with
  | Ok _ -> check_string "malformed" "error" "answered"
  | Error _ -> check_string "malformed" "error" "error"
;;

let test_cases =
  [ "syntax", [ Alcotest.test_case "query syntax procedures" `Quick test_syntax ]
  ; ( "matcher"
    , [ Alcotest.test_case "pattern matching and unification" `Quick test_matcher ] )
  ; "driver", [ Alcotest.test_case "driver and pinned interactions" `Quick test_driver ]
  ]
;;

(* Exercise pins: one group per exercise, appended as the reference
   solutions land; each group pins the exact string list the public
   ex_4_NN demonstration returns. *)
let exercise_cases : (string * unit Alcotest.test_case list) list =
  [ ( "4.55"
    , [ Alcotest.test_case "4.55" `Quick (fun () ->
          check_strings
            "4.55"
            [ "(supervisor ?x (Bitdiddle Ben))"
            ; "(supervisor (Hacker Alyssa P) (Bitdiddle Ben))"
            ; "(supervisor (Fect Cy D) (Bitdiddle Ben))"
            ; "(supervisor (Tweakit Lem E) (Bitdiddle Ben))"
            ; "(job ?name (accounting ?title))"
            ; "(job (Cratchet Robert) (accounting scrivener))"
            ; "(job ?name (accounting . ?title))"
            ; "(job (Scrooge Eben) (accounting chief accountant))"
            ; "(job (Cratchet Robert) (accounting scrivener))"
            ; "(address ?name (Slumerville . ?where))"
            ; "(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10))"
            ; "(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80))"
            ; "(address (Aull DeWitt) (Slumerville (Onion Square) 5))"
            ; "(supervisor ?x ?x)"
            ]
            (Sicp_ch4_solutions.Sec_4_55.ex_4_55 ()))
      ] )
  ; ( "4.56"
    , [ Alcotest.test_case "4.56" `Quick (fun () ->
          check_strings
            "4.56"
            [ "(and (supervisor ?person (Bitdiddle Ben)) (address ?person ?where))"
            ; "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (address (Hacker \
               Alyssa P) (Cambridge (Mass Ave) 78)))"
            ; "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (address (Fect Cy D) \
               (Cambridge (Ames Street) 3)))"
            ; "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (address (Tweakit Lem \
               E) (Boston (Bay State Road) 22)))"
            ; "(and (salary ?person ?amount) (salary (Bitdiddle Ben) ?ben-amount) \
               (lisp-value < ?amount ?ben-amount))"
            ; "(and (salary (Hacker Alyssa P) 40000) (salary (Bitdiddle Ben) 60000) \
               (lisp-value < 40000 60000))"
            ; "(and (salary (Fect Cy D) 35000) (salary (Bitdiddle Ben) 60000) \
               (lisp-value < 35000 60000))"
            ; "(and (salary (Tweakit Lem E) 25000) (salary (Bitdiddle Ben) 60000) \
               (lisp-value < 25000 60000))"
            ; "(and (salary (Reasoner Louis) 30000) (salary (Bitdiddle Ben) 60000) \
               (lisp-value < 30000 60000))"
            ; "(and (salary (Cratchet Robert) 18000) (salary (Bitdiddle Ben) 60000) \
               (lisp-value < 18000 60000))"
            ; "(and (salary (Aull DeWitt) 25000) (salary (Bitdiddle Ben) 60000) \
               (lisp-value < 25000 60000))"
            ; "(and (supervisor ?person ?supervisor) (job ?supervisor ?job) (not (job \
               ?supervisor (computer . ?type))))"
            ; "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (job (Warbucks \
               Oliver) (administration big wheel)) (not (job (Warbucks Oliver) (computer \
               . ?type))))"
            ; "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (job (Warbucks Oliver) \
               (administration big wheel)) (not (job (Warbucks Oliver) (computer . \
               ?type))))"
            ; "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (job (Scrooge Eben) \
               (accounting chief accountant)) (not (job (Scrooge Eben) (computer . \
               ?type))))"
            ; "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (job (Warbucks Oliver) \
               (administration big wheel)) (not (job (Warbucks Oliver) (computer . \
               ?type))))"
            ]
            (Sicp_ch4_solutions.Sec_4_56.ex_4_56 ()))
      ] )
  ; ( "4.57"
    , [ Alcotest.test_case "4.57" `Quick (fun () ->
          check_strings
            "4.57"
            [ "(can-replace ?x (Fect Cy D))"
            ; "(can-replace (Hacker Alyssa P) (Fect Cy D))"
            ; "(can-replace (Bitdiddle Ben) (Fect Cy D))"
            ; "(and (can-replace ?person-1 ?person-2) (salary ?person-1 ?salary-1) \
               (salary ?person-2 ?salary-2) (lisp-value < ?salary-1 ?salary-2))"
            ; "(and (can-replace (Fect Cy D) (Hacker Alyssa P)) (salary (Fect Cy D) \
               35000) (salary (Hacker Alyssa P) 40000) (lisp-value < 35000 40000))"
            ; "(and (can-replace (Aull DeWitt) (Warbucks Oliver)) (salary (Aull DeWitt) \
               25000) (salary (Warbucks Oliver) 150000) (lisp-value < 25000 150000))"
            ]
            (Sicp_ch4_solutions.Sec_4_57.ex_4_57 ()))
      ] )
  ; ( "4.58"
    , [ Alcotest.test_case "4.58" `Quick (fun () ->
          check_strings
            "4.58"
            [ "(big-shot ?person ?division)"
            ; "(big-shot (Warbucks Oliver) administration)"
            ; "(big-shot (Bitdiddle Ben) computer)"
            ; "(big-shot (Scrooge Eben) accounting)"
            ]
            (Sicp_ch4_solutions.Sec_4_58.ex_4_58 ()))
      ] )
  ; ( "4.59"
    , [ Alcotest.test_case "4.59" `Quick (fun () ->
          check_strings
            "4.59"
            [ "(meeting ?division (Friday ?time))"
            ; "(meeting administration (Friday 1pm))"
            ; "(meeting-time (Hacker Alyssa P) (Wednesday ?time))"
            ; "(meeting-time (Hacker Alyssa P) (Wednesday 4pm))"
            ; "(meeting-time (Hacker Alyssa P) (Wednesday 3pm))"
            ]
            (Sicp_ch4_solutions.Sec_4_59.ex_4_59 ()))
      ] )
  ; ( "4.60"
    , [ Alcotest.test_case "4.60" `Quick (fun () ->
          check_strings
            "4.60"
            [ "query: (lives-near ?person (Hacker Alyssa P))"
            ; "(lives-near (Fect Cy D) (Hacker Alyssa P))"
            ; "query: (lives-near ?person-1 ?person-2)"
            ; "note: every pair appears twice, once per binding order of the two address \
               conjuncts"
            ; "(lives-near (Bitdiddle Ben) (Reasoner Louis))"
            ; "(lives-near (Fect Cy D) (Hacker Alyssa P))"
            ; "(lives-near (Bitdiddle Ben) (Aull DeWitt))"
            ; "(lives-near (Hacker Alyssa P) (Fect Cy D))"
            ; "(lives-near (Reasoner Louis) (Bitdiddle Ben))"
            ; "(lives-near (Reasoner Louis) (Aull DeWitt))"
            ; "(lives-near (Aull DeWitt) (Bitdiddle Ben))"
            ; "(lives-near (Aull DeWitt) (Reasoner Louis))"
            ; "query: (lives-near-unique ?person-1 ?person-2)"
            ; "note: one order per pair, chosen by (lisp-value < ?salary-1 ?salary-2)"
            ; "(lives-near-unique (Fect Cy D) (Hacker Alyssa P))"
            ; "(lives-near-unique (Reasoner Louis) (Bitdiddle Ben))"
            ; "(lives-near-unique (Aull DeWitt) (Bitdiddle Ben))"
            ; "(lives-near-unique (Aull DeWitt) (Reasoner Louis))"
            ]
            (Sicp_ch4_solutions.Sec_4_60.ex_4_60 ()))
      ] )
  ; ( "4.61"
    , [ Alcotest.test_case "4.61" `Quick (fun () ->
          check_strings
            "4.61"
            [ "query: (?x next-to ?y in (1 (2 3) 4))"
            ; "(1 next-to (2 3) in (1 (2 3) 4))"
            ; "((2 3) next-to 4 in (1 (2 3) 4))"
            ; "query: (?x next-to 1 in (2 1 3 1))"
            ; "(2 next-to 1 in (2 1 3 1))"
            ; "(3 next-to 1 in (2 1 3 1))"
            ; "note: rules are tried in insertion order; rule 2 recurses on the list \
               tail, so the base case of the whole list answers first and the innermost \
               adjacency last"
            ]
            (Sicp_ch4_solutions.Sec_4_61.ex_4_61 ()))
      ] )
  ; ( "4.62"
    , [ Alcotest.test_case "4.62" `Quick (fun () ->
          check_strings
            "4.62"
            [ "query: (last-pair (3) ?x)"
            ; "(last-pair (3) (3))"
            ; "query: (last-pair (1 2 3) ?x)"
            ; "(last-pair (1 2 3) (3))"
            ; "query: (last-pair (2 ?x) (3))"
            ; "note: terminates; the step rule's recursive branch dies against the \
               query's already-bound one-element tail"
            ; "(last-pair (2 3) (3))"
            ; "query: (last-pair ?x (3))"
            ; "note: the full query never terminates; each step-rule application rebinds \
               its fresh tail variable and yields one more answer; first three answers:"
            ; "(last-pair (3) (3))"
            ; "(last-pair (?v-20 3) (3))"
            ; "(last-pair (?v-20 ?v-22 3) (3))"
            ]
            (Sicp_ch4_solutions.Sec_4_62.ex_4_62 ()))
      ] )
  ; ( "4.63"
    , [ Alcotest.test_case "4.63" `Quick (fun () ->
          check_strings
            "4.63"
            [ "query: (grandson ?x Cain)"
            ; "(grandson Irad Cain)"
            ; "query: (son Lamech ?x)"
            ; "(son Lamech Jabal)"
            ; "(son Lamech Jubal)"
            ; "query: (grandson ?x Methushael)"
            ; "(grandson Jabal Methushael)"
            ; "(grandson Jubal Methushael)"
            ]
            (Sicp_ch4_solutions.Sec_4_63.ex_4_63 ()))
      ] )
  ; ( "4.72"
    , [ Alcotest.test_case "4.72" `Quick (fun () ->
          check_strings
            "4.72"
            [ "interleave_first8"
            ; "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves ?a ?b) (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
            ; "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves ?a ?b) (supervisor (Fect Cy D) (Bitdiddle Ben)))"
            ; "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves ?a ?b) (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
            ; "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "interleave_supervisor_answers_in_first8=3"
            ; "append_first8"
            ; "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Minnie Mouse) (Mickey Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "(or (loves (Mickey Mouse) (Minnie Mouse)) (supervisor ?who (Bitdiddle \
               Ben)))"
            ; "append_supervisor_answers_in_first8=0"
            ; "append_supervisor_answers_in_first20=0"
            ]
            (Sicp_ch4_solutions.Sec_4_72.ex_4_72 ()))
      ] )
  ; ( "4.73"
    , [ Alcotest.test_case "4.73" `Quick (fun () ->
          check_strings
            "4.73"
            [ "delayed_and_first4"
            ; "(and (loves (Minnie Mouse) (Mickey Mouse)) (job (Hacker Alyssa P) \
               (computer programmer)))"
            ; "(and (loves (Mickey Mouse) (Minnie Mouse)) (job (Hacker Alyssa P) \
               (computer programmer)))"
            ; "(and (loves (Minnie Mouse) (Mickey Mouse)) (job (Fect Cy D) (computer \
               programmer)))"
            ; "(and (loves (Minnie Mouse) (Mickey Mouse)) (job (Hacker Alyssa P) \
               (computer programmer)))"
            ; "delayed_finite_first4=1 2 3 4"
            ; "undelayed_finite_first4=1 2 3 4"
            ; "finite_orders_agree=true"
            ; "delayed_sentinel_first=1"
            ; "undelayed_diverged_before_first_answer=true"
            ]
            (Sicp_ch4_solutions.Sec_4_73.ex_4_73 ()))
      ] )
  ; ( "4.74"
    , [ Alcotest.test_case "4.74" `Quick (fun () ->
          check_strings
            "4.74"
            [ "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (not (job (Tweakit Lem \
               E) (computer programmer))))"
            ; "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (not (job (Reasoner \
               Louis) (computer programmer))))"
            ; "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (not (job (Bitdiddle \
               Ben) (computer programmer))))"
            ; "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (not (job (Scrooge \
               Eben) (computer programmer))))"
            ; "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (not (job (Cratchet \
               Robert) (computer programmer))))"
            ; "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (not (job (Aull DeWitt) \
               (computer programmer))))"
            ; "book_flatmap_frames=14"
            ; "simple_flatmap_frames=14"
            ; "answers_equal=true"
            ]
            (Sicp_ch4_solutions.Sec_4_74.ex_4_74 ()))
      ] )
  ; ( "4.74a"
    , [ Alcotest.test_case "4.74a" `Quick (fun () ->
          check_strings
            "4.74a"
            [ "query=(and (supervisor ?x ?y) (not (job ?x (computer programmer))))"
            ; "old_frames=14"
            ; "simple_frames=14"
            ; "answers_equal=true"
            ; "counts_equal=true"
            ; "lisp_query=(and (salary ?person ?amount) (lisp-value > ?amount 30000))"
            ; "lisp_old_frames=14"
            ; "lisp_simple_frames=14"
            ; "lisp_answers_equal=true"
            ; "lisp_counts_equal=true"
            ]
            (Sicp_ch4_solutions.Sec_4_74.ex_4_74a ()))
      ] )
  ; ( "4.75"
    , [ Alcotest.test_case "4.75" `Quick (fun () ->
          check_strings
            "4.75"
            [ "unique_wizard"
            ; "(unique (job (Bitdiddle Ben) (computer wizard)))"
            ; "unique_wizard_answers=1"
            ; "unique_programmer"
            ; "unique_programmer_answers=0"
            ; "singly_filled_jobs"
            ; "(and (job (Bitdiddle Ben) (computer wizard)) (unique (job (Bitdiddle Ben) \
               (computer wizard))))"
            ; "(and (job (Tweakit Lem E) (computer technician)) (unique (job (Tweakit \
               Lem E) (computer technician))))"
            ; "(and (job (Reasoner Louis) (computer programmer trainee)) (unique (job \
               (Reasoner Louis) (computer programmer trainee))))"
            ; "(and (job (Warbucks Oliver) (administration big wheel)) (unique (job \
               (Warbucks Oliver) (administration big wheel))))"
            ; "(and (job (Scrooge Eben) (accounting chief accountant)) (unique (job \
               (Scrooge Eben) (accounting chief accountant))))"
            ; "(and (job (Cratchet Robert) (accounting scrivener)) (unique (job \
               (Cratchet Robert) (accounting scrivener))))"
            ; "(and (job (Aull DeWitt) (administration secretary)) (unique (job (Aull \
               DeWitt) (administration secretary))))"
            ; "singly_filled_jobs_answers=7"
            ; "supervises_precisely_one_person"
            ; "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (unique (supervisor \
               (Reasoner Louis) (Hacker Alyssa P))))"
            ; "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (unique (supervisor \
               (Cratchet Robert) (Scrooge Eben))))"
            ; "supervises_precisely_one_person_answers=2"
            ]
            (Sicp_ch4_solutions.Sec_4_75.ex_4_75 ()))
      ] )
  ; ( "4.64"
    , [ Alcotest.test_case "4.64" `Quick (fun () ->
          check_strings
            "4.64"
            [ "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ; "forcing past this one answer diverges: the recursion runs before the \
               supervisor test, re-enumerates every level forever, and the test can \
               never pass (Warbucks appears only in boss slots)"
            ; "bounded probe take(2) of the unanchored recursion: 2 answers, \
               (outranked-by (Hacker Alyssa P) (Bitdiddle Ben)) then (outranked-by \
               (Reasoner Louis) (Bitdiddle Ben)) -- the second already one deduction \
               level deep; deeper takes never return in bounded time"
            ; "the book's conjunct order answers the same query completely: "
            ; "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ]
            (Sicp_ch4_solutions.Sec_4_64.ex_4_64 ()))
      ] )
  ; ( "4.65"
    , [ Alcotest.test_case "4.65" `Quick (fun () ->
          check_strings
            "4.65"
            [ "(wheel (Bitdiddle Ben))"
            ; "(wheel (Warbucks Oliver))"
            ; "(wheel (Warbucks Oliver))"
            ; "(wheel (Warbucks Oliver))"
            ; "(wheel (Warbucks Oliver))"
            ; "Warbucks appears 4 times: 3 (middle-manager Ben) + 1 (middle-manager \
               Scrooge) + 0 (middle-manager Aull) routes"
            ; "Ben appears 1 time: middle-managers Alyssa, Cy, Lem with 1, 0, 0 \
               supervisees"
            ]
            (Sicp_ch4_solutions.Sec_4_65.ex_4_65 ()))
      ] )
  ; ( "4.66"
    , [ Alcotest.test_case "4.66" `Quick (fun () ->
          check_strings
            "4.66"
            [ "sum over the book's query = 75000 (from 2 frames)"
            ; "Ben's scheme on the wheel query = 660000 (from 5 frames): Warbucks's \
               150000 counted 4 times, 3 duplicate frames"
            ; "salvage, distinct answers only = 210000 (from 2 frames): the true payroll \
               of the wheels"
            ]
            (Sicp_ch4_solutions.Sec_4_66.ex_4_66 ()))
      ] )
  ; ( "4.67"
    , [ Alcotest.test_case "4.67" `Quick (fun () ->
          check_strings
            "4.67"
            [ "married Mickey ?who with the loop detector: 1 answer(s), 1 deduction \
               chain cut, terminates"
            ; "(married Mickey Minnie)"
            ; "stock evaluator, take(3) of the same query -- the loop re-derives the \
               answer forever:"
            ; "(married Mickey Minnie)"
            ; "(married Mickey Minnie)"
            ; "(married Mickey Minnie)"
            ; "wheel under the detector: 5 answers, identical to stock: true"
            ; "outranked-by under the detector: 1 answer(s), identical to stock: true"
            ]
            (Sicp_ch4_solutions.Sec_4_67.ex_4_67 ()))
      ] )
  ; ( "4.68"
    , [ Alcotest.test_case "4.68" `Quick (fun () ->
          check_strings
            "4.68"
            [ "(reverse (1 2 3) ?x) => (reverse (1 2 3) (3 2 1))"
            ; "(reverse (a b c d) ?x) => (reverse (a b c d) (d c b a))"
            ; "(reverse ?x (1 2 3)) first answer => (reverse (3 2 1) (1 2 3))"
            ; "backward: forcing a second answer does not return; with both ends of \
               (reverse ?v ?w) unbound the engine generates infinitely many candidate \
               pairs and only (reverse (3 2 1) (1 2 3)) survives the append-to-form \
               filter"
            ]
            (Sicp_ch4_solutions.Sec_4_68.ex_4_68 ()))
      ] )
  ; ( "4.69"
    , [ Alcotest.test_case "4.69" `Quick (fun () ->
          check_strings
            "4.69"
            [ "((great grandson) ?g ?ggs) => ((great grandson) Adam Irad)"
            ; "((great grandson) ?g ?ggs) => ((great grandson) Cain Mehujael)"
            ; "((great grandson) ?g ?ggs) => ((great grandson) Enoch Methushael)"
            ; "((great grandson) ?g ?ggs) => ((great grandson) Irad Lamech)"
            ; "((great grandson) ?g ?ggs) => ((great grandson) Mehujael Jabal)"
            ; "((great grandson) ?g ?ggs) => ((great grandson) Mehujael Jubal)"
            ; "(?relationship Adam Irad) first answer => ((great grandson) Adam Irad)"
            ; "note: with ?relationship unbound the ends-in-grandson generator produces \
               relationship lists of every depth; the first answer is the true one and a \
               full forcing would not return"
            ; "((great great great great great grandson) Adam ?d) => ((great great great \
               great great grandson) Adam Jabal)"
            ; "((great great great great great grandson) Adam ?d) => ((great great great \
               great great grandson) Adam Jubal)"
            ]
            (Sicp_ch4_solutions.Sec_4_69.ex_4_69 ()))
      ] )
  ; ( "4.70"
    , [ Alcotest.test_case "4.70" `Quick (fun () ->
          check_strings
            "4.70"
            [ "ones model (define ones (cons-stream 1 ones)): take(4) = 1 1 1 1 -- \
               uniform heads hide the cycle"
            ; "broken add-assertion! on (a b): add c => take(4) = c c c c"
            ; "broken: element 2 = element 1 = c -- the stream contains itself; the \
               stored (a b) is unreachable"
            ; "let-bound add-assertion! on (a b): add c => take(4) = c a b"
            ; "edition discipline: add_assertion binds old_assertions = !the_assertions, \
               then assigns old_assertions @ [assertion]; the right side is evaluated \
               before the cell is updated"
            ; "the book's hazard lives in a memoized-stream data base whose tail thunk \
               reads THE-ASSERTIONS at force time, after set! has rebound the name"
            ]
            (Sicp_ch4_solutions.Sec_4_70.ex_4_70 ()))
      ] )
  ; ( "4.71"
    , [ Alcotest.test_case "4.71" `Quick (fun () ->
          check_strings
            "4.71"
            [ "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ; "(outranked-by (Bitdiddle Ben) (Bitdiddle Ben))"
            ; "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ; "louis (plain stream-append in simple_query, plain interleave in disjoin): \
               no answer: constructing the first answer was still applying rules after \
               1001 steps -- the construction itself diverges"
            ; "the delay moves the rule body's evaluation from construction time to \
               answer-demand time; the cycle keeps the answer stream infinite and the \
               delayed engine still delivers it one answer at a time"
            ]
            (Sicp_ch4_solutions.Sec_4_71.ex_4_71 ()))
      ] )
  ; ( "4.76"
    , [ Alcotest.test_case "4.76" `Quick (fun () ->
          check_strings
            "4.76"
            [ "(and (job ?x (computer programmer)) (supervisor ?x ?boss))"
            ; "(and (job (Hacker Alyssa P) (computer programmer)) (supervisor (Hacker \
               Alyssa P) (Bitdiddle Ben)))"
            ; "(and (job (Fect Cy D) (computer programmer)) (supervisor (Fect Cy D) \
               (Bitdiddle Ben)))"
            ; "(and (job (Hacker Alyssa P) (computer programmer)) (supervisor (Hacker \
               Alyssa P) (Bitdiddle Ben)))"
            ; "(and (job (Fect Cy D) (computer programmer)) (supervisor (Fect Cy D) \
               (Bitdiddle Ben)))"
            ; "merge=series: true"
            ; "compatibility checks: 16"
            ; "(and (supervisor ?x ?y) (job ?x ?job))"
            ; "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (job (Hacker Alyssa \
               P) (computer programmer)))"
            ; "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (job (Fect Cy D) (computer \
               programmer)))"
            ; "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (job (Tweakit Lem E) \
               (computer technician)))"
            ; "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (job (Reasoner \
               Louis) (computer programmer trainee)))"
            ; "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (job (Bitdiddle Ben) \
               (computer wizard)))"
            ; "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (job (Scrooge Eben) \
               (accounting chief accountant)))"
            ; "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (job (Cratchet Robert) \
               (accounting scrivener)))"
            ; "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (job (Aull DeWitt) \
               (administration secretary)))"
            ; "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (job (Hacker Alyssa \
               P) (computer programmer)))"
            ; "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (job (Fect Cy D) (computer \
               programmer)))"
            ; "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (job (Tweakit Lem E) \
               (computer technician)))"
            ; "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (job (Reasoner \
               Louis) (computer programmer trainee)))"
            ; "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (job (Bitdiddle Ben) \
               (computer wizard)))"
            ; "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (job (Scrooge Eben) \
               (accounting chief accountant)))"
            ; "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (job (Cratchet Robert) \
               (accounting scrivener)))"
            ; "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (job (Aull DeWitt) \
               (administration secretary)))"
            ; "merge=series: true"
            ; "compatibility checks: 72"
            ]
            (Sicp_ch4_solutions.Sec_4_76.ex_4_76 ()))
      ] )
  ; ( "4.77"
    , [ Alcotest.test_case "4.77" `Quick (fun () ->
          check_strings
            "4.77"
            [ "(and (not (job ?x (computer programmer))) (supervisor ?x ?y))"
            ; "naive:"
            ; "delayed:"
            ; "(and (not (job (Tweakit Lem E) (computer programmer))) (supervisor \
               (Tweakit Lem E) (Bitdiddle Ben)))"
            ; "(and (not (job (Reasoner Louis) (computer programmer))) (supervisor \
               (Reasoner Louis) (Hacker Alyssa P)))"
            ; "(and (not (job (Bitdiddle Ben) (computer programmer))) (supervisor \
               (Bitdiddle Ben) (Warbucks Oliver)))"
            ; "(and (not (job (Scrooge Eben) (computer programmer))) (supervisor \
               (Scrooge Eben) (Warbucks Oliver)))"
            ; "(and (not (job (Cratchet Robert) (computer programmer))) (supervisor \
               (Cratchet Robert) (Scrooge Eben)))"
            ; "(and (not (job (Aull DeWitt) (computer programmer))) (supervisor (Aull \
               DeWitt) (Warbucks Oliver)))"
            ; "deferred=1 fulfilled=8 unresolved=0"
            ; "(and (lisp-value > ?amount 30000) (salary ?who ?amount))"
            ; "naive:"
            ; "Error: Unknown pat var LISP-VALUE: (? amount)"
            ; "delayed:"
            ; "(and (lisp-value > 60000 30000) (salary (Bitdiddle Ben) 60000))"
            ; "(and (lisp-value > 40000 30000) (salary (Hacker Alyssa P) 40000))"
            ; "(and (lisp-value > 35000 30000) (salary (Fect Cy D) 35000))"
            ; "(and (lisp-value > 150000 30000) (salary (Warbucks Oliver) 150000))"
            ; "(and (lisp-value > 75000 30000) (salary (Scrooge Eben) 75000))"
            ; "deferred=1 fulfilled=9 unresolved=0"
            ; "(and (salary ?who ?amount) (lisp-value > ?amount 30000))"
            ; "naive:"
            ; "(and (salary (Bitdiddle Ben) 60000) (lisp-value > 60000 30000))"
            ; "(and (salary (Hacker Alyssa P) 40000) (lisp-value > 40000 30000))"
            ; "(and (salary (Fect Cy D) 35000) (lisp-value > 35000 30000))"
            ; "(and (salary (Warbucks Oliver) 150000) (lisp-value > 150000 30000))"
            ; "(and (salary (Scrooge Eben) 75000) (lisp-value > 75000 30000))"
            ; "delayed:"
            ; "(and (salary (Bitdiddle Ben) 60000) (lisp-value > 60000 30000))"
            ; "(and (salary (Hacker Alyssa P) 40000) (lisp-value > 40000 30000))"
            ; "(and (salary (Fect Cy D) 35000) (lisp-value > 35000 30000))"
            ; "(and (salary (Warbucks Oliver) 150000) (lisp-value > 150000 30000))"
            ; "(and (salary (Scrooge Eben) 75000) (lisp-value > 75000 30000))"
            ; "deferred=0 fulfilled=0 unresolved=0"
            ]
            (Sicp_ch4_solutions.Sec_4_77.ex_4_77 ()))
      ] )
  ; ( "4.78"
    , [ Alcotest.test_case "4.78" `Quick (fun () ->
          check_strings
            "4.78"
            [ "(job ?x (computer programmer))"
            ; "(job (Hacker Alyssa P) (computer programmer))"
            ; "(job (Fect Cy D) (computer programmer))"
            ; "Error: there are no more values"
            ; "Error: there is no current problem"
            ; "after exhaustion:"
            ; "Error: there is no current problem"
            ; "(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))"
            ; "stream (interleaved):"
            ; "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker \
               Alyssa P) (Hacker Alyssa P)))"
            ; "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner \
               Louis) (Hacker Alyssa P)))"
            ; "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) \
               (Hacker Alyssa P)))"
            ; "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem \
               E) (Hacker Alyssa P)))"
            ; "amb (depth-first):"
            ; "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (supervisor (Hacker \
               Alyssa P) (Hacker Alyssa P)))"
            ; "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (supervisor (Fect Cy D) \
               (Hacker Alyssa P)))"
            ; "(or (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (supervisor (Tweakit Lem \
               E) (Hacker Alyssa P)))"
            ; "(or (supervisor (Reasoner Louis) (Bitdiddle Ben)) (supervisor (Reasoner \
               Louis) (Hacker Alyssa P)))"
            ; "Error: there are no more values"
            ; "(married Mickey ?who)"
            ; "stream (first 3):"
            ; "(married Mickey Minnie)"
            ; "(married Mickey Minnie)"
            ; "(married Mickey Minnie)"
            ; "amb (first 3):"
            ; "(married Mickey Minnie)"
            ; "(married Mickey Minnie)"
            ; "(married Mickey Minnie)"
            ; "(and (supervisor ?x ?y) (not (job ?x (computer programmer))))"
            ; "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (not (job (Tweakit Lem \
               E) (computer programmer))))"
            ; "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (not (job (Reasoner \
               Louis) (computer programmer))))"
            ; "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (not (job (Bitdiddle \
               Ben) (computer programmer))))"
            ; "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (not (job (Scrooge \
               Eben) (computer programmer))))"
            ; "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (not (job (Cratchet \
               Robert) (computer programmer))))"
            ; "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (not (job (Aull DeWitt) \
               (computer programmer))))"
            ; "Error: there are no more values"
            ]
            (Sicp_ch4_solutions.Sec_4_78.ex_4_78 ()))
      ] )
  ; ( "4.79"
    , [ Alcotest.test_case "4.79" `Quick (fun () ->
          check_strings
            "4.79"
            [ "(outranked-by (Bitdiddle Ben) ?who)"
            ; "renaming:"
            ; "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ; "scoped:"
            ; "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ; "renaming=scoped: true"
            ; "(outranked-by ?staff-person ?boss)"
            ; "renaming:"
            ; "(outranked-by (Hacker Alyssa P) (Bitdiddle Ben))"
            ; "(outranked-by (Hacker Alyssa P) (Warbucks Oliver))"
            ; "(outranked-by (Fect Cy D) (Bitdiddle Ben))"
            ; "(outranked-by (Fect Cy D) (Warbucks Oliver))"
            ; "(outranked-by (Tweakit Lem E) (Bitdiddle Ben))"
            ; "(outranked-by (Tweakit Lem E) (Warbucks Oliver))"
            ; "(outranked-by (Reasoner Louis) (Hacker Alyssa P))"
            ; "(outranked-by (Reasoner Louis) (Bitdiddle Ben))"
            ; "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ; "(outranked-by (Cratchet Robert) (Warbucks Oliver))"
            ; "(outranked-by (Scrooge Eben) (Warbucks Oliver))"
            ; "(outranked-by (Reasoner Louis) (Warbucks Oliver))"
            ; "(outranked-by (Cratchet Robert) (Scrooge Eben))"
            ; "(outranked-by (Aull DeWitt) (Warbucks Oliver))"
            ; "scoped:"
            ; "(outranked-by (Hacker Alyssa P) (Bitdiddle Ben))"
            ; "(outranked-by (Hacker Alyssa P) (Warbucks Oliver))"
            ; "(outranked-by (Fect Cy D) (Bitdiddle Ben))"
            ; "(outranked-by (Fect Cy D) (Warbucks Oliver))"
            ; "(outranked-by (Tweakit Lem E) (Bitdiddle Ben))"
            ; "(outranked-by (Tweakit Lem E) (Warbucks Oliver))"
            ; "(outranked-by (Reasoner Louis) (Hacker Alyssa P))"
            ; "(outranked-by (Reasoner Louis) (Bitdiddle Ben))"
            ; "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"
            ; "(outranked-by (Cratchet Robert) (Warbucks Oliver))"
            ; "(outranked-by (Scrooge Eben) (Warbucks Oliver))"
            ; "(outranked-by (Reasoner Louis) (Warbucks Oliver))"
            ; "(outranked-by (Cratchet Robert) (Scrooge Eben))"
            ; "(outranked-by (Aull DeWitt) (Warbucks Oliver))"
            ; "renaming=scoped: true"
            ; "(and (salary ?staff-person ?amount) (outranked-by ?staff-person ?boss))"
            ; "renaming:"
            ; "(and (salary (Hacker Alyssa P) 40000) (outranked-by (Hacker Alyssa P) \
               (Bitdiddle Ben)))"
            ; "(and (salary (Fect Cy D) 35000) (outranked-by (Fect Cy D) (Bitdiddle \
               Ben)))"
            ; "(and (salary (Hacker Alyssa P) 40000) (outranked-by (Hacker Alyssa P) \
               (Warbucks Oliver)))"
            ; "(and (salary (Tweakit Lem E) 25000) (outranked-by (Tweakit Lem E) \
               (Bitdiddle Ben)))"
            ; "(and (salary (Fect Cy D) 35000) (outranked-by (Fect Cy D) (Warbucks \
               Oliver)))"
            ; "(and (salary (Reasoner Louis) 30000) (outranked-by (Reasoner Louis) \
               (Hacker Alyssa P)))"
            ; "(and (salary (Tweakit Lem E) 25000) (outranked-by (Tweakit Lem E) \
               (Warbucks Oliver)))"
            ; "(and (salary (Bitdiddle Ben) 60000) (outranked-by (Bitdiddle Ben) \
               (Warbucks Oliver)))"
            ; "(and (salary (Reasoner Louis) 30000) (outranked-by (Reasoner Louis) \
               (Bitdiddle Ben)))"
            ; "(and (salary (Scrooge Eben) 75000) (outranked-by (Scrooge Eben) (Warbucks \
               Oliver)))"
            ; "(and (salary (Reasoner Louis) 30000) (outranked-by (Reasoner Louis) \
               (Warbucks Oliver)))"
            ; "(and (salary (Cratchet Robert) 18000) (outranked-by (Cratchet Robert) \
               (Scrooge Eben)))"
            ; "(and (salary (Aull DeWitt) 25000) (outranked-by (Aull DeWitt) (Warbucks \
               Oliver)))"
            ; "(and (salary (Cratchet Robert) 18000) (outranked-by (Cratchet Robert) \
               (Warbucks Oliver)))"
            ; "scoped:"
            ; "(and (salary (Hacker Alyssa P) 40000) (outranked-by (Hacker Alyssa P) \
               (Bitdiddle Ben)))"
            ; "(and (salary (Fect Cy D) 35000) (outranked-by (Fect Cy D) (Bitdiddle \
               Ben)))"
            ; "(and (salary (Hacker Alyssa P) 40000) (outranked-by (Hacker Alyssa P) \
               (Warbucks Oliver)))"
            ; "(and (salary (Tweakit Lem E) 25000) (outranked-by (Tweakit Lem E) \
               (Bitdiddle Ben)))"
            ; "(and (salary (Fect Cy D) 35000) (outranked-by (Fect Cy D) (Warbucks \
               Oliver)))"
            ; "(and (salary (Reasoner Louis) 30000) (outranked-by (Reasoner Louis) \
               (Hacker Alyssa P)))"
            ; "(and (salary (Tweakit Lem E) 25000) (outranked-by (Tweakit Lem E) \
               (Warbucks Oliver)))"
            ; "(and (salary (Bitdiddle Ben) 60000) (outranked-by (Bitdiddle Ben) \
               (Warbucks Oliver)))"
            ; "(and (salary (Reasoner Louis) 30000) (outranked-by (Reasoner Louis) \
               (Bitdiddle Ben)))"
            ; "(and (salary (Scrooge Eben) 75000) (outranked-by (Scrooge Eben) (Warbucks \
               Oliver)))"
            ; "(and (salary (Reasoner Louis) 30000) (outranked-by (Reasoner Louis) \
               (Warbucks Oliver)))"
            ; "(and (salary (Cratchet Robert) 18000) (outranked-by (Cratchet Robert) \
               (Scrooge Eben)))"
            ; "(and (salary (Aull DeWitt) 25000) (outranked-by (Aull DeWitt) (Warbucks \
               Oliver)))"
            ; "(and (salary (Cratchet Robert) 18000) (outranked-by (Cratchet Robert) \
               (Warbucks Oliver)))"
            ; "renaming=scoped: true"
            ]
            (Sicp_ch4_solutions.Sec_4_79.ex_4_79 ()))
      ] )
  ]
;;

let () = Alcotest.run "sec_4_4" (test_cases @ exercise_cases)
