(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the query system of section 4.4: the engine's own
   contract (the matcher and unifier cases the book walks through in
   4.4.2, the occurs check, the driver over the Microshaft data base,
   the answer order of [Or], and the typed failures) and every
   exercise's demonstration pinned to the exact observable outcome the
   reference solutions produce. *)

module Q = Sicp_ch4.Sec_4_4
module Kit = Sicp_ch4_solutions.Sec_4_55.Kit

let check_strings = Alcotest.(check (list string))
let check_string = Alcotest.(check string)
let at = Kit.at
let v = Kit.v
let l = Kit.l
let p = Kit.p
let variable name = { Q.name; id = 0 }

let bound frame name =
  Q.render_term (Q.instantiate (Q.Var (variable name)) frame (fun w -> Q.Var w))
;;

(* The matcher and unifier cases of 4.4.2 and the occurs check of
   4.4.4.4. *)
let test_matcher () =
  let ab = l [ at "a"; at "b" ] in
  (match Q.pattern_match (l [ v "x"; at "c"; v "x" ]) (l [ ab; at "c"; ab ]) [] with
   | Some frame -> check_string "repeated variable binds once" "[a, b]" (bound frame "x")
   | None -> Alcotest.fail "[?x, c, ?x] should match [[a, b], c, [a, b]]");
  Alcotest.(check bool)
    "a repeated variable must see equal data"
    true
    (Option.is_none
       (Q.pattern_match (l [ v "x"; at "c"; v "x" ]) (l [ ab; at "c"; at "d" ]) []));
  (match Q.unify_match (l [ v "x"; at "a"; v "y" ]) (l [ v "y"; v "z"; at "a" ]) [] with
   | Some frame ->
     check_strings
       "unification chains three variables to a"
       [ "a"; "a"; "a" ]
       (List.map (bound frame) [ "x"; "y"; "z" ])
   | None -> Alcotest.fail "[?x, a, ?y] should unify with [?y, ?z, a]");
  (match
     Q.unify_match
       (l [ v "x"; v "x" ])
       (l [ l [ at "a"; v "y"; at "c" ]; l [ at "a"; at "b"; v "z" ] ])
       []
   with
   | Some frame ->
     check_strings
       "both sides contribute bindings"
       [ "[a, b, c]"; "b"; "c" ]
       (List.map (bound frame) [ "x"; "y"; "z" ])
   | None -> Alcotest.fail "[?x, ?x] should unify with [[a, ?y, c], [a, b, ?z]]");
  Alcotest.(check bool)
    "unification rejects conflicting constants"
    true
    (Option.is_none (Q.unify_match (l [ v "x"; at "a" ]) (l [ at "b"; at "b" ]) []));
  Alcotest.(check bool)
    "occurs check: ?x cannot unify with a term containing ?x"
    true
    (Option.is_none (Q.unify_match (v "x") (l [ at "f"; v "x" ]) []));
  Alcotest.(check bool)
    "occurs check through a binding chain"
    true
    (Option.is_none
       (Q.unify_match (l [ v "x"; v "x" ]) (l [ v "y"; l [ at "a"; v "y" ] ]) []));
  let chained = Q.extend (variable "y") (v "x") [] in
  Alcotest.(check bool)
    "depends_on sees through frame bindings"
    true
    (Q.depends_on (v "y") (variable "x") chained);
  Alcotest.(check bool)
    "depends_on is false for an unrelated variable"
    false
    (Q.depends_on (v "y") (variable "x") [])
;;

let microshaft_session () =
  Kit.session
    ~rules:
      [ l [ at "append-to-form"; Q.Nil; v "y"; v "y" ], Q.Always_true
      ; ( l
            [ at "append-to-form"
            ; Q.dotted [ v "u" ] (v "v")
            ; v "y"
            ; Q.dotted [ v "u" ] (v "z")
            ]
        , p [ at "append-to-form"; v "v"; v "y"; v "z" ] )
      ]
    Kit.microshaft
;;

(* The driver over the Microshaft data base: the book's pinned
   interactions and the answer-order contract. *)
let test_driver () =
  let s = microshaft_session () in
  check_strings
    "simple query in assertion order"
    [ "? [job, ?x, [computer, programmer]]"
    ; "[job, [Hacker, Alyssa, P], [computer, programmer]]"
    ; "[job, [Fect, Cy, D], [computer, programmer]]"
    ]
    (Kit.transcript s [ p [ at "job"; v "x"; Kit.atoms [ "computer"; "programmer" ] ] ]);
  check_strings
    "a dotted tail spans any number of elements"
    [ "? [job, ?x, [computer | ?type]]"
    ; "[job, [Bitdiddle, Ben], [computer, wizard]]"
    ; "[job, [Hacker, Alyssa, P], [computer, programmer]]"
    ; "[job, [Fect, Cy, D], [computer, programmer]]"
    ; "[job, [Tweakit, Lem, E], [computer, technician]]"
    ; "[job, [Reasoner, Louis], [computer, programmer, trainee]]"
    ]
    (Kit.transcript s [ p [ at "job"; v "x"; Q.dotted [ at "computer" ] (v "type") ] ]);
  check_strings
    "or interleaves its disjuncts"
    [ "or([supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]], [supervisor, [Hacker, \
       Alyssa, P], [Hacker, Alyssa, P]])"
    ; "or([supervisor, [Reasoner, Louis], [Bitdiddle, Ben]], [supervisor, [Reasoner, \
       Louis], [Hacker, Alyssa, P]])"
    ; "or([supervisor, [Fect, Cy, D], [Bitdiddle, Ben]], [supervisor, [Fect, Cy, D], \
       [Hacker, Alyssa, P]])"
    ; "or([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], [supervisor, [Tweakit, Lem, \
       E], [Hacker, Alyssa, P]])"
    ]
    (Kit.answers_all
       s
       (Q.Or
          [ p [ at "supervisor"; v "x"; Kit.person "Bitdiddle Ben" ]
          ; p [ at "supervisor"; v "x"; Kit.person "Hacker Alyssa P" ]
          ]));
  check_strings
    "holds filters bound frames"
    [ "and([salary, [Scrooge, Eben], 75000], holds(>, 75000, 70000))"
    ; "and([salary, [Warbucks, Oliver], 150000], holds(>, 150000, 70000))"
    ]
    (List.sort
       String.compare
       (Kit.answers_all
          s
          (Q.And
             [ p [ at "salary"; v "who"; v "amount" ]
             ; Q.Holds (">", [ v "amount"; Kit.n 70000 ])
             ])));
  check_strings
    "not on unbound variables filters every frame out"
    []
    (Kit.answers_all s (Q.Not (p [ at "supervisor"; Kit.person "Bitdiddle Ben"; v "b" ])));
  check_strings
    "append runs backwards through two rules"
    [ "[append-to-form, [], [a, b], [a, b]]"
    ; "[append-to-form, [a], [b], [a, b]]"
    ; "[append-to-form, [a, b], [], [a, b]]"
    ]
    (Kit.answers_all s (p [ at "append-to-form"; v "x"; v "y"; Kit.atoms [ "a"; "b" ] ]));
  check_strings
    "holds on an unbound variable is a typed failure"
    [ "error: unbound variable holds on unbound ?amount" ]
    (Kit.answers_all s (Q.Holds (">", [ v "amount"; Kit.n 1 ])));
  check_strings
    "an unknown special form is a typed failure"
    [ "error: invalid form: unknown query form unique" ]
    (Kit.answers_all s (Q.Form ("unique", [ Q.Always_true ])));
  Q.add_assertion s (Kit.atoms [ "meeting"; "whole-company"; "Wednesday" ]);
  check_strings
    "an added assertion answers later queries"
    [ "[meeting, whole-company, Wednesday]" ]
    (Kit.answers_all s (p [ at "meeting"; v "who"; v "when" ]))
;;

(* [take] reads a prefix without forcing the tail behind it. *)
let test_take () =
  let forced = ref 0 in
  let s =
    Q.Streams.cons_stream 1 (fun () ->
      incr forced;
      Q.Streams.cons_stream 2 (fun () ->
        incr forced;
        Q.Streams.the_empty_stream))
  in
  Alcotest.(check (list int)) "prefix of one" [ 1 ] (Kit.take 1 s);
  Alcotest.(check int) "no tail forced" 0 !forced;
  Alcotest.(check (list int)) "past the end" [ 1; 2 ] (Kit.take 5 s)
;;

let pin label actual expected =
  Alcotest.test_case label `Quick (fun () -> check_strings label expected (actual ()))
;;

let test_cases =
  [ "matcher", [ Alcotest.test_case "4.4.2 matching and unification" `Quick test_matcher ]
  ; "driver", [ Alcotest.test_case "Microshaft interactions" `Quick test_driver ]
  ; "take", [ Alcotest.test_case "bounded prefix" `Quick test_take ]
  ]
;;

(* Exercise pins: one group per exercise, each pinning the exact string
   list the public ex_4_NN demonstration returns. *)
let exercise_cases : (string * unit Alcotest.test_case list) list =
  [ ( "4.55"
    , [ pin
          "4.55"
          Sicp_ch4_solutions.Sec_4_55.ex_4_55
          [ "? [supervisor, ?x, [Bitdiddle, Ben]]"
          ; "[supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]]"
          ; "[supervisor, [Fect, Cy, D], [Bitdiddle, Ben]]"
          ; "[supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]]"
          ; "? [job, ?name, [accounting, ?title]]"
          ; "[job, [Cratchet, Robert], [accounting, scrivener]]"
          ; "? [job, ?name, [accounting | ?title]]"
          ; "[job, [Scrooge, Eben], [accounting, chief, accountant]]"
          ; "[job, [Cratchet, Robert], [accounting, scrivener]]"
          ; "? [address, ?name, [Slumerville | ?where]]"
          ; "[address, [Bitdiddle, Ben], [Slumerville, [Ridge, Road], 10]]"
          ; "[address, [Reasoner, Louis], [Slumerville, [Pine, Tree, Road], 80]]"
          ; "[address, [Aull, DeWitt], [Slumerville, [Onion, Square], 5]]"
          ; "? [supervisor, ?x, ?x]"
          ]
      ] )
  ; ( "4.56"
    , [ pin
          "4.56"
          Sicp_ch4_solutions.Sec_4_56.ex_4_56
          [ "? and([supervisor, ?person, [Bitdiddle, Ben]], [address, ?person, ?where])"
          ; "and([supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]], [address, [Hacker, \
             Alyssa, P], [Cambridge, [Mass, Ave], 78]])"
          ; "and([supervisor, [Fect, Cy, D], [Bitdiddle, Ben]], [address, [Fect, Cy, D], \
             [Cambridge, [Ames, Street], 3]])"
          ; "and([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], [address, [Tweakit, \
             Lem, E], [Boston, [Bay, State, Road], 22]])"
          ; "? and([salary, ?person, ?amount], [salary, [Bitdiddle, Ben], ?ben-amount], \
             holds(<, ?amount, ?ben-amount))"
          ; "and([salary, [Hacker, Alyssa, P], 40000], [salary, [Bitdiddle, Ben], \
             60000], holds(<, 40000, 60000))"
          ; "and([salary, [Fect, Cy, D], 35000], [salary, [Bitdiddle, Ben], 60000], \
             holds(<, 35000, 60000))"
          ; "and([salary, [Tweakit, Lem, E], 25000], [salary, [Bitdiddle, Ben], 60000], \
             holds(<, 25000, 60000))"
          ; "and([salary, [Reasoner, Louis], 30000], [salary, [Bitdiddle, Ben], 60000], \
             holds(<, 30000, 60000))"
          ; "and([salary, [Cratchet, Robert], 18000], [salary, [Bitdiddle, Ben], 60000], \
             holds(<, 18000, 60000))"
          ; "and([salary, [Aull, DeWitt], 25000], [salary, [Bitdiddle, Ben], 60000], \
             holds(<, 25000, 60000))"
          ; "? and([supervisor, ?person, ?supervisor], [job, ?supervisor, ?job], \
             not([job, ?supervisor, [computer | ?type]]))"
          ; "and([supervisor, [Bitdiddle, Ben], [Warbucks, Oliver]], [job, [Warbucks, \
             Oliver], [administration, big, wheel]], not([job, [Warbucks, Oliver], \
             [computer | ?type]]))"
          ; "and([supervisor, [Scrooge, Eben], [Warbucks, Oliver]], [job, [Warbucks, \
             Oliver], [administration, big, wheel]], not([job, [Warbucks, Oliver], \
             [computer | ?type]]))"
          ; "and([supervisor, [Cratchet, Robert], [Scrooge, Eben]], [job, [Scrooge, \
             Eben], [accounting, chief, accountant]], not([job, [Scrooge, Eben], \
             [computer | ?type]]))"
          ; "and([supervisor, [Aull, DeWitt], [Warbucks, Oliver]], [job, [Warbucks, \
             Oliver], [administration, big, wheel]], not([job, [Warbucks, Oliver], \
             [computer | ?type]]))"
          ]
      ] )
  ; ( "4.57"
    , [ pin
          "4.57"
          Sicp_ch4_solutions.Sec_4_57.ex_4_57
          [ "? [can-replace, ?x, [Fect, Cy, D]]"
          ; "[can-replace, [Hacker, Alyssa, P], [Fect, Cy, D]]"
          ; "[can-replace, [Bitdiddle, Ben], [Fect, Cy, D]]"
          ; "? and([can-replace, ?person-1, ?person-2], [salary, ?person-1, ?salary-1], \
             [salary, ?person-2, ?salary-2], holds(<, ?salary-1, ?salary-2))"
          ; "and([can-replace, [Fect, Cy, D], [Hacker, Alyssa, P]], [salary, [Fect, Cy, \
             D], 35000], [salary, [Hacker, Alyssa, P], 40000], holds(<, 35000, 40000))"
          ; "and([can-replace, [Aull, DeWitt], [Warbucks, Oliver]], [salary, [Aull, \
             DeWitt], 25000], [salary, [Warbucks, Oliver], 150000], holds(<, 25000, \
             150000))"
          ]
      ] )
  ; ( "4.58"
    , [ pin
          "4.58"
          Sicp_ch4_solutions.Sec_4_58.ex_4_58
          [ "? [big-shot, ?person, ?division]"
          ; "[big-shot, [Warbucks, Oliver], administration]"
          ; "[big-shot, [Bitdiddle, Ben], computer]"
          ; "[big-shot, [Scrooge, Eben], accounting]"
          ]
      ] )
  ; ( "4.59"
    , [ pin
          "4.59"
          Sicp_ch4_solutions.Sec_4_59.ex_4_59
          [ "? [meeting, ?division, [Friday, ?time]]"
          ; "[meeting, administration, [Friday, 1pm]]"
          ; "? [meeting-time, [Hacker, Alyssa, P], [Wednesday, ?time]]"
          ; "[meeting-time, [Hacker, Alyssa, P], [Wednesday, 4pm]]"
          ; "[meeting-time, [Hacker, Alyssa, P], [Wednesday, 3pm]]"
          ]
      ] )
  ; ( "4.60"
    , [ pin
          "4.60"
          Sicp_ch4_solutions.Sec_4_60.ex_4_60
          [ "? [lives-near, ?person, [Hacker, Alyssa, P]]"
          ; "[lives-near, [Fect, Cy, D], [Hacker, Alyssa, P]]"
          ; "note: every pair appears twice, once per binding order of the two address \
             conjuncts"
          ; "? [lives-near, ?person-1, ?person-2]"
          ; "[lives-near, [Bitdiddle, Ben], [Reasoner, Louis]]"
          ; "[lives-near, [Fect, Cy, D], [Hacker, Alyssa, P]]"
          ; "[lives-near, [Bitdiddle, Ben], [Aull, DeWitt]]"
          ; "[lives-near, [Hacker, Alyssa, P], [Fect, Cy, D]]"
          ; "[lives-near, [Reasoner, Louis], [Bitdiddle, Ben]]"
          ; "[lives-near, [Reasoner, Louis], [Aull, DeWitt]]"
          ; "[lives-near, [Aull, DeWitt], [Bitdiddle, Ben]]"
          ; "[lives-near, [Aull, DeWitt], [Reasoner, Louis]]"
          ; "note: one order per pair, chosen by holds(<, ?salary-1, ?salary-2)"
          ; "? [lives-near-unique, ?person-1, ?person-2]"
          ; "[lives-near-unique, [Fect, Cy, D], [Hacker, Alyssa, P]]"
          ; "[lives-near-unique, [Reasoner, Louis], [Bitdiddle, Ben]]"
          ; "[lives-near-unique, [Aull, DeWitt], [Bitdiddle, Ben]]"
          ; "[lives-near-unique, [Aull, DeWitt], [Reasoner, Louis]]"
          ]
      ] )
  ; ( "4.61"
    , [ pin
          "4.61"
          Sicp_ch4_solutions.Sec_4_61.ex_4_61
          [ "? [?x, next-to, ?y, in, [1, [2, 3], 4]]"
          ; "[1, next-to, [2, 3], in, [1, [2, 3], 4]]"
          ; "[[2, 3], next-to, 4, in, [1, [2, 3], 4]]"
          ; "? [?x, next-to, 1, in, [2, 1, 3, 1]]"
          ; "[2, next-to, 1, in, [2, 1, 3, 1]]"
          ; "[3, next-to, 1, in, [2, 1, 3, 1]]"
          ; "note: rules are tried in insertion order; rule 2 recurses on the list tail, \
             so the base case of the whole list answers first and the innermost \
             adjacency last"
          ]
      ] )
  ; ( "4.62"
    , [ pin
          "4.62"
          Sicp_ch4_solutions.Sec_4_62.ex_4_62
          [ "? [last-pair, [3], ?x]"
          ; "[last-pair, [3], [3]]"
          ; "? [last-pair, [1, 2, 3], ?x]"
          ; "[last-pair, [1, 2, 3], [3]]"
          ; "? [last-pair, [2, ?x], [3]]"
          ; "[last-pair, [2, 3], [3]]"
          ; "note: the third query terminates; the step rule's recursive branch dies \
             against the query's already-bound one-element tail"
          ; "? [last-pair, ?x, [3]]"
          ; "note: the full query never terminates; each step-rule application rebinds \
             its fresh tail variable and yields one more answer; first three answers:"
          ; "[last-pair, [3], [3]]"
          ; "[last-pair, [?v.1, 3], [3]]"
          ; "[last-pair, [?v.1, ?v.2, 3], [3]]"
          ]
      ] )
  ; ( "4.63"
    , [ pin
          "4.63"
          Sicp_ch4_solutions.Sec_4_63.ex_4_63
          [ "? [grandson, ?x, Cain]"
          ; "[grandson, Irad, Cain]"
          ; "? [son, Lamech, ?x]"
          ; "[son, Lamech, Jabal]"
          ; "[son, Lamech, Jubal]"
          ; "? [grandson, ?x, Methushael]"
          ; "[grandson, Jabal, Methushael]"
          ; "[grandson, Jubal, Methushael]"
          ]
      ] )
  ; ( "4.64"
    , [ pin
          "4.64"
          Sicp_ch4_solutions.Sec_4_64.ex_4_64
          [ "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ; "forcing past this one answer diverges: the recursion runs before the \
             supervisor test, re-enumerates every level forever, and the test can never \
             pass (Warbucks appears only in boss slots)"
          ; "bounded probe take(2) of the unanchored recursion: 2 answers, \
             [outranked-by, [Hacker, Alyssa, P], [Bitdiddle, Ben]] then [outranked-by, \
             [Reasoner, Louis], [Bitdiddle, Ben]] -- the second already one deduction \
             level deep; deeper takes never return in bounded time"
          ; "the book's conjunct order answers the same query completely:"
          ; "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ]
      ] )
  ; ( "4.65"
    , [ pin
          "4.65"
          Sicp_ch4_solutions.Sec_4_65.ex_4_65
          [ "? [wheel, ?who]"
          ; "[wheel, [Bitdiddle, Ben]]"
          ; "[wheel, [Warbucks, Oliver]]"
          ; "[wheel, [Warbucks, Oliver]]"
          ; "[wheel, [Warbucks, Oliver]]"
          ; "[wheel, [Warbucks, Oliver]]"
          ; "Warbucks appears 4 times (observed): 3 (middle-manager Ben) + 1 \
             (middle-manager Scrooge) + 0 (middle-manager Aull) routes"
          ; "Ben appears 1 time (observed): middle-managers Alyssa, Cy, Lem with 1, 0, 0 \
             supervisees"
          ]
      ] )
  ; ( "4.66"
    , [ pin
          "4.66"
          Sicp_ch4_solutions.Sec_4_66.ex_4_66
          [ "sum over the book's query = 75000 (from 2 frames)"
          ; "Ben's scheme on the wheel query = 660000 (from 5 frames): Warbucks's 150000 \
             counted 4 times, 3 duplicate frames"
          ; "salvage, distinct answers only = 210000 (from 2 frames): the true payroll \
             of the wheels"
          ]
      ] )
  ; ( "4.67"
    , [ pin
          "4.67"
          Sicp_ch4_solutions.Sec_4_67.ex_4_67
          [ "married Mickey ?who with the loop detector: 1 answer(s), 1 deduction chain \
             cut, terminates"
          ; "[married, Mickey, Minnie]"
          ; "stock evaluator, take(3) of the same query -- the loop re-derives the \
             answer forever:"
          ; "[married, Mickey, Minnie]"
          ; "[married, Mickey, Minnie]"
          ; "[married, Mickey, Minnie]"
          ; "wheel under the detector: 5 answers, identical to stock: true"
          ; "outranked-by under the detector: 1 answer(s), identical to stock: true"
          ]
      ] )
  ; ( "4.68"
    , [ pin
          "4.68"
          Sicp_ch4_solutions.Sec_4_68.ex_4_68
          [ "[reverse, [1, 2, 3], ?x] => [reverse, [1, 2, 3], [3, 2, 1]]"
          ; "[reverse, [a, b, c, d], ?x] => [reverse, [a, b, c, d], [d, c, b, a]]"
          ; "[reverse, ?x, [1, 2, 3]] first answer => [reverse, [3, 2, 1], [1, 2, 3]]"
          ; "backward: forcing a second answer does not return; with both ends of \
             [reverse, ?v, ?w] unbound the engine generates infinitely many candidate \
             pairs and only [reverse, [3, 2, 1], [1, 2, 3]] survives the append-to-form \
             filter"
          ]
      ] )
  ; ( "4.69"
    , [ pin
          "4.69"
          Sicp_ch4_solutions.Sec_4_69.ex_4_69
          [ "[[great, grandson], ?g, ?ggs] => [[great, grandson], Adam, Irad]"
          ; "[[great, grandson], ?g, ?ggs] => [[great, grandson], Cain, Mehujael]"
          ; "[[great, grandson], ?g, ?ggs] => [[great, grandson], Enoch, Methushael]"
          ; "[[great, grandson], ?g, ?ggs] => [[great, grandson], Irad, Lamech]"
          ; "[[great, grandson], ?g, ?ggs] => [[great, grandson], Mehujael, Jabal]"
          ; "[[great, grandson], ?g, ?ggs] => [[great, grandson], Mehujael, Jubal]"
          ; "[?relationship, Adam, Irad] first answer => [[great, grandson], Adam, Irad]"
          ; "note: with ?relationship unbound the ends-in-grandson generator produces \
             relationship lists of every depth; the first answer is the true one and a \
             full forcing would not return"
          ; "[[great, great, great, great, great, grandson], Adam, ?d] => [[great, \
             great, great, great, great, grandson], Adam, Jabal]"
          ; "[[great, great, great, great, great, grandson], Adam, ?d] => [[great, \
             great, great, great, great, grandson], Adam, Jubal]"
          ]
      ] )
  ; ( "4.70"
    , [ pin
          "4.70"
          Sicp_ch4_solutions.Sec_4_70.ex_4_70
          [ "ones model (define ones (cons-stream 1 ones)): take(4) = 1 1 1 1 -- uniform \
             heads hide the cycle"
          ; "broken add-assertion! on a b: add c => take(4) = c c c c"
          ; "broken: element 2 = element 1 = c -- the stream contains itself; the stored \
             a b is unreachable"
          ; "let-bound add-assertion! on a b: add c => take(4) = c a b"
          ; "edition discipline: add_assertion assigns session.assertions @ [t]; the \
             right side is evaluated before the field is updated"
          ; "the book's hazard lives in a memoized-stream data base whose tail thunk \
             reads THE-ASSERTIONS at force time, after set! has rebound the name"
          ]
      ] )
  ; ( "4.71"
    , [ pin
          "4.71"
          Sicp_ch4_solutions.Sec_4_71.ex_4_71
          [ "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ; "[outranked-by, [Bitdiddle, Ben], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ; "louis (plain stream-append in simple_query, plain interleave in disjoin): \
             no answer: constructing the first answer was still applying rules after \
             1001 steps -- the construction itself diverges"
          ; "the delay moves the rule body's evaluation from construction time to \
             answer-demand time; the cycle keeps the answer stream infinite and the \
             delayed engine still delivers it one answer at a time"
          ]
      ] )
  ; ( "4.72"
    , [ pin
          "4.72"
          Sicp_ch4_solutions.Sec_4_72.ex_4_72
          [ "interleave_first8"
          ; "or([loves, [Minnie, Mouse], [Mickey, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, ?a, ?b], [supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]])"
          ; "or([loves, [Mickey, Mouse], [Minnie, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, ?a, ?b], [supervisor, [Fect, Cy, D], [Bitdiddle, Ben]])"
          ; "or([loves, [Minnie, Mouse], [Mickey, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, ?a, ?b], [supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]])"
          ; "or([loves, [Mickey, Mouse], [Minnie, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Minnie, Mouse], [Mickey, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "interleave_supervisor_answers_in_first8=3"
          ; "append_first8"
          ; "or([loves, [Minnie, Mouse], [Mickey, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Mickey, Mouse], [Minnie, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Minnie, Mouse], [Mickey, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Mickey, Mouse], [Minnie, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Minnie, Mouse], [Mickey, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Mickey, Mouse], [Minnie, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Minnie, Mouse], [Mickey, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "or([loves, [Mickey, Mouse], [Minnie, Mouse]], [supervisor, ?who, \
             [Bitdiddle, Ben]])"
          ; "append_supervisor_answers_in_first8=0"
          ; "append_supervisor_answers_in_first20=0"
          ]
      ] )
  ; ( "4.73"
    , [ pin
          "4.73"
          Sicp_ch4_solutions.Sec_4_73.ex_4_73
          [ "delayed_and_first4"
          ; "and([loves, [Minnie, Mouse], [Mickey, Mouse]], [job, [Hacker, Alyssa, P], \
             [computer, programmer]])"
          ; "and([loves, [Mickey, Mouse], [Minnie, Mouse]], [job, [Hacker, Alyssa, P], \
             [computer, programmer]])"
          ; "and([loves, [Minnie, Mouse], [Mickey, Mouse]], [job, [Fect, Cy, D], \
             [computer, programmer]])"
          ; "and([loves, [Minnie, Mouse], [Mickey, Mouse]], [job, [Hacker, Alyssa, P], \
             [computer, programmer]])"
          ; "delayed_finite_first4=1 2 3 4"
          ; "undelayed_finite_first4=1 2 3 4"
          ; "finite_orders_agree=true"
          ; "delayed_sentinel_first=1"
          ; "undelayed_diverged_before_first_answer=true"
          ]
      ] )
  ; ( "4.74"
    , [ pin
          "4.74"
          Sicp_ch4_solutions.Sec_4_74.ex_4_74
          [ "and([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], not([job, [Tweakit, \
             Lem, E], [computer, programmer]]))"
          ; "and([supervisor, [Reasoner, Louis], [Hacker, Alyssa, P]], not([job, \
             [Reasoner, Louis], [computer, programmer]]))"
          ; "and([supervisor, [Bitdiddle, Ben], [Warbucks, Oliver]], not([job, \
             [Bitdiddle, Ben], [computer, programmer]]))"
          ; "and([supervisor, [Scrooge, Eben], [Warbucks, Oliver]], not([job, [Scrooge, \
             Eben], [computer, programmer]]))"
          ; "and([supervisor, [Cratchet, Robert], [Scrooge, Eben]], not([job, [Cratchet, \
             Robert], [computer, programmer]]))"
          ; "and([supervisor, [Aull, DeWitt], [Warbucks, Oliver]], not([job, [Aull, \
             DeWitt], [computer, programmer]]))"
          ; "book_flatmap_frames=14"
          ; "simple_flatmap_frames=14"
          ; "answers_equal=true"
          ]
      ] )
  ; ( "4.74a"
    , [ pin
          "4.74a"
          Sicp_ch4_solutions.Sec_4_74.ex_4_74a
          [ "query=and([supervisor, ?x, ?y], not([job, ?x, [computer, programmer]]))"
          ; "old_frames=14"
          ; "simple_frames=14"
          ; "answers_equal=true"
          ; "counts_equal=true"
          ; "holds_query=and([salary, ?person, ?amount], holds(>, ?amount, 30000))"
          ; "holds_old_frames=14"
          ; "holds_simple_frames=14"
          ; "holds_answers_equal=true"
          ; "holds_counts_equal=true"
          ]
      ] )
  ; ( "4.75"
    , [ pin
          "4.75"
          Sicp_ch4_solutions.Sec_4_75.ex_4_75
          [ "unique_wizard"
          ; "unique([job, [Bitdiddle, Ben], [computer, wizard]])"
          ; "unique_wizard_answers=1"
          ; "unique_programmer"
          ; "unique_programmer_answers=0"
          ; "singly_filled_jobs"
          ; "and([job, [Bitdiddle, Ben], [computer, wizard]], unique([job, [Bitdiddle, \
             Ben], [computer, wizard]]))"
          ; "and([job, [Tweakit, Lem, E], [computer, technician]], unique([job, \
             [Tweakit, Lem, E], [computer, technician]]))"
          ; "and([job, [Reasoner, Louis], [computer, programmer, trainee]], unique([job, \
             [Reasoner, Louis], [computer, programmer, trainee]]))"
          ; "and([job, [Warbucks, Oliver], [administration, big, wheel]], unique([job, \
             [Warbucks, Oliver], [administration, big, wheel]]))"
          ; "and([job, [Scrooge, Eben], [accounting, chief, accountant]], unique([job, \
             [Scrooge, Eben], [accounting, chief, accountant]]))"
          ; "and([job, [Cratchet, Robert], [accounting, scrivener]], unique([job, \
             [Cratchet, Robert], [accounting, scrivener]]))"
          ; "and([job, [Aull, DeWitt], [administration, secretary]], unique([job, [Aull, \
             DeWitt], [administration, secretary]]))"
          ; "singly_filled_jobs_answers=7"
          ; "supervises_precisely_one_person"
          ; "and([supervisor, [Reasoner, Louis], [Hacker, Alyssa, P]], \
             unique([supervisor, [Reasoner, Louis], [Hacker, Alyssa, P]]))"
          ; "and([supervisor, [Cratchet, Robert], [Scrooge, Eben]], unique([supervisor, \
             [Cratchet, Robert], [Scrooge, Eben]]))"
          ; "supervises_precisely_one_person_answers=2"
          ]
      ] )
  ; ( "4.76"
    , [ pin
          "4.76"
          Sicp_ch4_solutions.Sec_4_76.ex_4_76
          [ "? and([job, ?x, [computer, programmer]], [supervisor, ?x, ?boss])"
          ; "and([job, [Hacker, Alyssa, P], [computer, programmer]], [supervisor, \
             [Hacker, Alyssa, P], [Bitdiddle, Ben]])"
          ; "and([job, [Fect, Cy, D], [computer, programmer]], [supervisor, [Fect, Cy, \
             D], [Bitdiddle, Ben]])"
          ; "and([job, [Hacker, Alyssa, P], [computer, programmer]], [supervisor, \
             [Hacker, Alyssa, P], [Bitdiddle, Ben]])"
          ; "and([job, [Fect, Cy, D], [computer, programmer]], [supervisor, [Fect, Cy, \
             D], [Bitdiddle, Ben]])"
          ; "merge=series: true"
          ; "compatibility checks: 16"
          ; "? and([supervisor, ?x, ?y], [job, ?x, ?job])"
          ; "and([supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]], [job, [Hacker, \
             Alyssa, P], [computer, programmer]])"
          ; "and([supervisor, [Fect, Cy, D], [Bitdiddle, Ben]], [job, [Fect, Cy, D], \
             [computer, programmer]])"
          ; "and([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], [job, [Tweakit, Lem, \
             E], [computer, technician]])"
          ; "and([supervisor, [Reasoner, Louis], [Hacker, Alyssa, P]], [job, [Reasoner, \
             Louis], [computer, programmer, trainee]])"
          ; "and([supervisor, [Bitdiddle, Ben], [Warbucks, Oliver]], [job, [Bitdiddle, \
             Ben], [computer, wizard]])"
          ; "and([supervisor, [Scrooge, Eben], [Warbucks, Oliver]], [job, [Scrooge, \
             Eben], [accounting, chief, accountant]])"
          ; "and([supervisor, [Cratchet, Robert], [Scrooge, Eben]], [job, [Cratchet, \
             Robert], [accounting, scrivener]])"
          ; "and([supervisor, [Aull, DeWitt], [Warbucks, Oliver]], [job, [Aull, DeWitt], \
             [administration, secretary]])"
          ; "and([supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]], [job, [Hacker, \
             Alyssa, P], [computer, programmer]])"
          ; "and([supervisor, [Fect, Cy, D], [Bitdiddle, Ben]], [job, [Fect, Cy, D], \
             [computer, programmer]])"
          ; "and([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], [job, [Tweakit, Lem, \
             E], [computer, technician]])"
          ; "and([supervisor, [Reasoner, Louis], [Hacker, Alyssa, P]], [job, [Reasoner, \
             Louis], [computer, programmer, trainee]])"
          ; "and([supervisor, [Bitdiddle, Ben], [Warbucks, Oliver]], [job, [Bitdiddle, \
             Ben], [computer, wizard]])"
          ; "and([supervisor, [Scrooge, Eben], [Warbucks, Oliver]], [job, [Scrooge, \
             Eben], [accounting, chief, accountant]])"
          ; "and([supervisor, [Cratchet, Robert], [Scrooge, Eben]], [job, [Cratchet, \
             Robert], [accounting, scrivener]])"
          ; "and([supervisor, [Aull, DeWitt], [Warbucks, Oliver]], [job, [Aull, DeWitt], \
             [administration, secretary]])"
          ; "merge=series: true"
          ; "compatibility checks: 72"
          ]
      ] )
  ; ( "4.77"
    , [ pin
          "4.77"
          Sicp_ch4_solutions.Sec_4_77.ex_4_77
          [ "? and(not([job, ?x, [computer, programmer]]), [supervisor, ?x, ?y])"
          ; "naive:"
          ; "delayed:"
          ; "and(not([job, [Tweakit, Lem, E], [computer, programmer]]), [supervisor, \
             [Tweakit, Lem, E], [Bitdiddle, Ben]])"
          ; "and(not([job, [Reasoner, Louis], [computer, programmer]]), [supervisor, \
             [Reasoner, Louis], [Hacker, Alyssa, P]])"
          ; "and(not([job, [Bitdiddle, Ben], [computer, programmer]]), [supervisor, \
             [Bitdiddle, Ben], [Warbucks, Oliver]])"
          ; "and(not([job, [Scrooge, Eben], [computer, programmer]]), [supervisor, \
             [Scrooge, Eben], [Warbucks, Oliver]])"
          ; "and(not([job, [Cratchet, Robert], [computer, programmer]]), [supervisor, \
             [Cratchet, Robert], [Scrooge, Eben]])"
          ; "and(not([job, [Aull, DeWitt], [computer, programmer]]), [supervisor, [Aull, \
             DeWitt], [Warbucks, Oliver]])"
          ; "deferred=1 fulfilled=8"
          ; "? and(holds(>, ?amount, 30000), [salary, ?who, ?amount])"
          ; "naive:"
          ; "error: unbound variable holds on unbound ?amount"
          ; "delayed:"
          ; "and(holds(>, 60000, 30000), [salary, [Bitdiddle, Ben], 60000])"
          ; "and(holds(>, 40000, 30000), [salary, [Hacker, Alyssa, P], 40000])"
          ; "and(holds(>, 35000, 30000), [salary, [Fect, Cy, D], 35000])"
          ; "and(holds(>, 150000, 30000), [salary, [Warbucks, Oliver], 150000])"
          ; "and(holds(>, 75000, 30000), [salary, [Scrooge, Eben], 75000])"
          ; "deferred=1 fulfilled=9"
          ; "? and([salary, ?who, ?amount], holds(>, ?amount, 30000))"
          ; "naive:"
          ; "and([salary, [Bitdiddle, Ben], 60000], holds(>, 60000, 30000))"
          ; "and([salary, [Hacker, Alyssa, P], 40000], holds(>, 40000, 30000))"
          ; "and([salary, [Fect, Cy, D], 35000], holds(>, 35000, 30000))"
          ; "and([salary, [Warbucks, Oliver], 150000], holds(>, 150000, 30000))"
          ; "and([salary, [Scrooge, Eben], 75000], holds(>, 75000, 30000))"
          ; "delayed:"
          ; "and([salary, [Bitdiddle, Ben], 60000], holds(>, 60000, 30000))"
          ; "and([salary, [Hacker, Alyssa, P], 40000], holds(>, 40000, 30000))"
          ; "and([salary, [Fect, Cy, D], 35000], holds(>, 35000, 30000))"
          ; "and([salary, [Warbucks, Oliver], 150000], holds(>, 150000, 30000))"
          ; "and([salary, [Scrooge, Eben], 75000], holds(>, 75000, 30000))"
          ; "deferred=0 fulfilled=0"
          ]
      ] )
  ; ( "4.78"
    , [ pin
          "4.78"
          Sicp_ch4_solutions.Sec_4_78.ex_4_78
          [ "? [job, ?x, [computer, programmer]]"
          ; "[job, [Hacker, Alyssa, P], [computer, programmer]]"
          ; "[job, [Fect, Cy, D], [computer, programmer]]"
          ; "error: there are no more values"
          ; "error: there is no current problem"
          ; "after exhaustion:"
          ; "error: there is no current problem"
          ; "? or([supervisor, ?x, [Bitdiddle, Ben]], [supervisor, ?x, [Hacker, Alyssa, \
             P]])"
          ; "stream (interleaved):"
          ; "or([supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]], [supervisor, \
             [Hacker, Alyssa, P], [Hacker, Alyssa, P]])"
          ; "or([supervisor, [Reasoner, Louis], [Bitdiddle, Ben]], [supervisor, \
             [Reasoner, Louis], [Hacker, Alyssa, P]])"
          ; "or([supervisor, [Fect, Cy, D], [Bitdiddle, Ben]], [supervisor, [Fect, Cy, \
             D], [Hacker, Alyssa, P]])"
          ; "or([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], [supervisor, \
             [Tweakit, Lem, E], [Hacker, Alyssa, P]])"
          ; "amb (depth-first):"
          ; "or([supervisor, [Hacker, Alyssa, P], [Bitdiddle, Ben]], [supervisor, \
             [Hacker, Alyssa, P], [Hacker, Alyssa, P]])"
          ; "or([supervisor, [Fect, Cy, D], [Bitdiddle, Ben]], [supervisor, [Fect, Cy, \
             D], [Hacker, Alyssa, P]])"
          ; "or([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], [supervisor, \
             [Tweakit, Lem, E], [Hacker, Alyssa, P]])"
          ; "or([supervisor, [Reasoner, Louis], [Bitdiddle, Ben]], [supervisor, \
             [Reasoner, Louis], [Hacker, Alyssa, P]])"
          ; "error: there are no more values"
          ; "? [married, Mickey, ?who]"
          ; "stream (first 3):"
          ; "[married, Mickey, Minnie]"
          ; "[married, Mickey, Minnie]"
          ; "[married, Mickey, Minnie]"
          ; "amb (first 3):"
          ; "[married, Mickey, Minnie]"
          ; "[married, Mickey, Minnie]"
          ; "[married, Mickey, Minnie]"
          ; "? and([supervisor, ?x, ?y], not([job, ?x, [computer, programmer]]))"
          ; "and([supervisor, [Tweakit, Lem, E], [Bitdiddle, Ben]], not([job, [Tweakit, \
             Lem, E], [computer, programmer]]))"
          ; "and([supervisor, [Reasoner, Louis], [Hacker, Alyssa, P]], not([job, \
             [Reasoner, Louis], [computer, programmer]]))"
          ; "and([supervisor, [Bitdiddle, Ben], [Warbucks, Oliver]], not([job, \
             [Bitdiddle, Ben], [computer, programmer]]))"
          ; "and([supervisor, [Scrooge, Eben], [Warbucks, Oliver]], not([job, [Scrooge, \
             Eben], [computer, programmer]]))"
          ; "and([supervisor, [Cratchet, Robert], [Scrooge, Eben]], not([job, [Cratchet, \
             Robert], [computer, programmer]]))"
          ; "and([supervisor, [Aull, DeWitt], [Warbucks, Oliver]], not([job, [Aull, \
             DeWitt], [computer, programmer]]))"
          ; "error: there are no more values"
          ]
      ] )
  ; ( "4.79"
    , [ pin
          "4.79"
          Sicp_ch4_solutions.Sec_4_79.ex_4_79
          [ "? [outranked-by, [Bitdiddle, Ben], ?who]"
          ; "renaming:"
          ; "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ; "scoped:"
          ; "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ; "renaming=scoped: true"
          ; "? [outranked-by, ?staff-person, ?boss]"
          ; "renaming:"
          ; "[outranked-by, [Hacker, Alyssa, P], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Hacker, Alyssa, P], [Warbucks, Oliver]]"
          ; "[outranked-by, [Fect, Cy, D], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Fect, Cy, D], [Warbucks, Oliver]]"
          ; "[outranked-by, [Tweakit, Lem, E], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Tweakit, Lem, E], [Warbucks, Oliver]]"
          ; "[outranked-by, [Reasoner, Louis], [Hacker, Alyssa, P]]"
          ; "[outranked-by, [Reasoner, Louis], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ; "[outranked-by, [Cratchet, Robert], [Warbucks, Oliver]]"
          ; "[outranked-by, [Scrooge, Eben], [Warbucks, Oliver]]"
          ; "[outranked-by, [Reasoner, Louis], [Warbucks, Oliver]]"
          ; "[outranked-by, [Cratchet, Robert], [Scrooge, Eben]]"
          ; "[outranked-by, [Aull, DeWitt], [Warbucks, Oliver]]"
          ; "scoped:"
          ; "[outranked-by, [Hacker, Alyssa, P], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Hacker, Alyssa, P], [Warbucks, Oliver]]"
          ; "[outranked-by, [Fect, Cy, D], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Fect, Cy, D], [Warbucks, Oliver]]"
          ; "[outranked-by, [Tweakit, Lem, E], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Tweakit, Lem, E], [Warbucks, Oliver]]"
          ; "[outranked-by, [Reasoner, Louis], [Hacker, Alyssa, P]]"
          ; "[outranked-by, [Reasoner, Louis], [Bitdiddle, Ben]]"
          ; "[outranked-by, [Bitdiddle, Ben], [Warbucks, Oliver]]"
          ; "[outranked-by, [Cratchet, Robert], [Warbucks, Oliver]]"
          ; "[outranked-by, [Scrooge, Eben], [Warbucks, Oliver]]"
          ; "[outranked-by, [Reasoner, Louis], [Warbucks, Oliver]]"
          ; "[outranked-by, [Cratchet, Robert], [Scrooge, Eben]]"
          ; "[outranked-by, [Aull, DeWitt], [Warbucks, Oliver]]"
          ; "renaming=scoped: true"
          ; "? and([salary, ?staff-person, ?amount], [outranked-by, ?staff-person, \
             ?boss])"
          ; "renaming:"
          ; "and([salary, [Bitdiddle, Ben], 60000], [outranked-by, [Bitdiddle, Ben], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Hacker, Alyssa, P], 40000], [outranked-by, [Hacker, Alyssa, \
             P], [Bitdiddle, Ben]])"
          ; "and([salary, [Fect, Cy, D], 35000], [outranked-by, [Fect, Cy, D], \
             [Bitdiddle, Ben]])"
          ; "and([salary, [Hacker, Alyssa, P], 40000], [outranked-by, [Hacker, Alyssa, \
             P], [Warbucks, Oliver]])"
          ; "and([salary, [Tweakit, Lem, E], 25000], [outranked-by, [Tweakit, Lem, E], \
             [Bitdiddle, Ben]])"
          ; "and([salary, [Fect, Cy, D], 35000], [outranked-by, [Fect, Cy, D], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Reasoner, Louis], 30000], [outranked-by, [Reasoner, Louis], \
             [Hacker, Alyssa, P]])"
          ; "and([salary, [Tweakit, Lem, E], 25000], [outranked-by, [Tweakit, Lem, E], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Scrooge, Eben], 75000], [outranked-by, [Scrooge, Eben], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Reasoner, Louis], 30000], [outranked-by, [Reasoner, Louis], \
             [Bitdiddle, Ben]])"
          ; "and([salary, [Cratchet, Robert], 18000], [outranked-by, [Cratchet, Robert], \
             [Scrooge, Eben]])"
          ; "and([salary, [Reasoner, Louis], 30000], [outranked-by, [Reasoner, Louis], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Aull, DeWitt], 25000], [outranked-by, [Aull, DeWitt], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Cratchet, Robert], 18000], [outranked-by, [Cratchet, Robert], \
             [Warbucks, Oliver]])"
          ; "scoped:"
          ; "and([salary, [Bitdiddle, Ben], 60000], [outranked-by, [Bitdiddle, Ben], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Hacker, Alyssa, P], 40000], [outranked-by, [Hacker, Alyssa, \
             P], [Bitdiddle, Ben]])"
          ; "and([salary, [Fect, Cy, D], 35000], [outranked-by, [Fect, Cy, D], \
             [Bitdiddle, Ben]])"
          ; "and([salary, [Hacker, Alyssa, P], 40000], [outranked-by, [Hacker, Alyssa, \
             P], [Warbucks, Oliver]])"
          ; "and([salary, [Tweakit, Lem, E], 25000], [outranked-by, [Tweakit, Lem, E], \
             [Bitdiddle, Ben]])"
          ; "and([salary, [Fect, Cy, D], 35000], [outranked-by, [Fect, Cy, D], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Reasoner, Louis], 30000], [outranked-by, [Reasoner, Louis], \
             [Hacker, Alyssa, P]])"
          ; "and([salary, [Tweakit, Lem, E], 25000], [outranked-by, [Tweakit, Lem, E], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Scrooge, Eben], 75000], [outranked-by, [Scrooge, Eben], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Reasoner, Louis], 30000], [outranked-by, [Reasoner, Louis], \
             [Bitdiddle, Ben]])"
          ; "and([salary, [Cratchet, Robert], 18000], [outranked-by, [Cratchet, Robert], \
             [Scrooge, Eben]])"
          ; "and([salary, [Reasoner, Louis], 30000], [outranked-by, [Reasoner, Louis], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Aull, DeWitt], 25000], [outranked-by, [Aull, DeWitt], \
             [Warbucks, Oliver]])"
          ; "and([salary, [Cratchet, Robert], 18000], [outranked-by, [Cratchet, Robert], \
             [Warbucks, Oliver]])"
          ; "renaming=scoped: true"
          ]
      ] )
  ]
;;

let () = Alcotest.run "sec_4_4" (test_cases @ exercise_cases)
