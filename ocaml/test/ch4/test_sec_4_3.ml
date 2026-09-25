(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 4.3. Every exercise's demonstration is pinned to the exact
   observable outcomes the solutions produce; the substrate amb
   evaluator's own contract -- the driver sample, the try-again
   protocol, and the undo trail of the parser's set! -- is pinned too,
   so a solution that leans on a broken clause cannot pass. *)

let check_strings = Alcotest.(check (list string))

let show = function
  | Ok v -> Sicp_common.Value.to_string v
  | Error e -> "Error: " ^ Sicp_common.Eval_error.to_string e
;;

(* The 4.3.1 driver sample: the first non-failing execution, the
   try-again protocol to exhaustion, and a fresh problem. *)
let substrate_driver () =
  let env = Sicp_ch4.Sec_4_3.the_global_environment () in
  let (_ : (Sicp_common.Value.t, Sicp_common.Eval_error.t) result) =
    Sicp_ch4.Sec_4_3.run_program
      env
      {|
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (prime? n)
  (define (divides? d) (= 0 (remainder n d)))
  (define (find-divisor d)
    (if (> (* d d) n) n (if (divides? d) d (find-divisor (+ d 1)))))
  (if (< n 2) #f (= n (find-divisor 2))))
(define (quotient x y) (if (< x y) 0 (+ 1 (quotient (- x y) y))))
(define (remainder x y) (- x (* y (quotient x y))))
(define (prime-sum-pair list1 list2)
  (let ((a (an-element-of list1)) (b (an-element-of list2)))
    (require (prime? (+ a b)))
    (list a b)))|}
  in
  Alcotest.(check string)
    "first answer"
    "(3 20)"
    (show (Sicp_ch4.Sec_4_3.run env "(prime-sum-pair '(1 3 5 8) '(20 35 110))"));
  Alcotest.(check string) "try again 1" "(3 110)" (show (Sicp_ch4.Sec_4_3.try_again ()));
  Alcotest.(check string) "try again 2" "(8 35)" (show (Sicp_ch4.Sec_4_3.try_again ()));
  Alcotest.(check string)
    "exhaustion"
    "Error: there are no more values"
    (show (Sicp_ch4.Sec_4_3.try_again ()));
  Alcotest.(check string)
    "fresh problem"
    "(30 11)"
    (show (Sicp_ch4.Sec_4_3.run env "(prime-sum-pair '(19 27 30) '(11 36 58))"));
  Alcotest.(check string)
    "fresh problem exhaustion"
    "Error: there are no more values"
    (show (Sicp_ch4.Sec_4_3.try_again ()));
  Alcotest.(check string)
    "no current problem"
    "Error: there is no current problem"
    (show (Sicp_ch4.Sec_4_3.try_again ()))
;;

let () =
  let open Alcotest in
  run
    "Sec_4_3"
    [ "substrate driver", [ test_case "driver sample" `Quick substrate_driver ]
    ; ( "4.35"
      , [ test_case "4.35" `Quick (fun () ->
            check_strings
              "4.35"
              [ "(3 4 5)"
              ; "(5 12 13)"
              ; "(6 8 10)"
              ; "(8 15 17)"
              ; "(9 12 15)"
              ; "(12 16 20)"
              ; "Error: there are no more values"
              ]
              (Sicp_ch4_solutions.Sec_4_35.ex_4_35 ()))
        ] )
    ; ( "4.36"
      , [ test_case "4.36" `Quick (fun () ->
            check_strings
              "4.36"
              [ "(3 4 5)"
              ; "(6 8 10)"
              ; "(5 12 13)"
              ; "(9 12 15)"
              ; "(8 15 17)"
              ; "(12 16 20)"
              ]
              (Sicp_ch4_solutions.Sec_4_36.ex_4_36 ()))
        ] )
    ; ( "4.37"
      , [ test_case "4.37" `Quick (fun () ->
            check_strings
              "4.37"
              [ "(3 4 5)"; "backtracks=918"; "(3 4 5)"; "backtracks=81" ]
              (Sicp_ch4_solutions.Sec_4_37.ex_4_37 ()))
        ] )
    ; ( "4.38"
      , [ test_case "4.38" `Quick (fun () ->
            check_strings
              "4.38"
              [ "((baker 1) (cooper 2) (fletcher 3) (miller 4) (smith 5))"
              ; "((baker 1) (cooper 3) (fletcher 2) (miller 4) (smith 5))"
              ; "((baker 1) (cooper 3) (fletcher 2) (miller 5) (smith 4))"
              ; "((baker 1) (cooper 3) (fletcher 4) (miller 5) (smith 2))"
              ; "((baker 2) (cooper 3) (fletcher 4) (miller 5) (smith 1))"
              ; "((baker 2) (cooper 4) (fletcher 3) (miller 5) (smith 1))"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "((baker 4) (cooper 2) (fletcher 3) (miller 5) (smith 1))"
              ; "solutions=8"
              ]
              (Sicp_ch4_solutions.Sec_4_38.ex_4_38 ()))
        ] )
    ; ( "4.39"
      , [ test_case "4.39" `Quick (fun () ->
            check_strings
              "4.39"
              [ "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "backtracks=1835"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "backtracks=310"
              ]
              (Sicp_ch4_solutions.Sec_4_39.ex_4_39 ()))
        ] )
    ; ( "4.40"
      , [ test_case "4.40" `Quick (fun () ->
            check_strings
              "4.40"
              [ "before distinct?: 3125"
              ; "after distinct?: 120"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "backtracks=1835"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "backtracks=1119"
              ]
              (Sicp_ch4_solutions.Sec_4_40.ex_4_40 ()))
        ] )
    ; ( "4.41"
      , [ test_case "4.41" `Quick (fun () ->
            check_strings
              "4.41"
              [ "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "solutions=1"
              ]
              (Sicp_ch4_solutions.Sec_4_41.ex_4_41 ()))
        ] )
    ; ( "4.42"
      , [ test_case "4.42" `Quick (fun () ->
            check_strings
              "4.42"
              [ "((betty 3) (ethel 5) (joan 2) (kitty 1) (mary 4))"
              ; "Error: there are no more values"
              ]
              (Sicp_ch4_solutions.Sec_4_42.ex_4_42 ()))
        ] )
    ; ( "4.43"
      , [ test_case "4.43" `Quick (fun () ->
            check_strings
              "4.43"
              [ "(lornas-father downing)"
              ; "Error: there are no more values"
              ; "(lornas-father downing)"
              ; "(lornas-father parker)"
              ; "Error: there are no more values"
              ]
              (Sicp_ch4_solutions.Sec_4_43.ex_4_43 ()))
        ] )
    ; ( "4.44"
      , [ test_case "4.44" `Quick (fun () ->
            check_strings
              "4.44"
              [ "(4 2 7 3 6 8 5 1)"; "(3 1 4 2)"; "(5 3 1 6 4 2)" ]
              (Sicp_ch4_solutions.Sec_4_44.ex_4_44 ()))
        ] )
    ; ( "4.44a"
      , [ test_case "4.44a" `Quick (fun () ->
            check_strings
              "4.44a"
              [ "(3 1 4 2)"
              ; "(4 2 5 3 1)"
              ; "(5 3 1 6 4 2)"
              ; "backtracks(4)=38"
              ; "backtracks(5)=10"
              ; "backtracks(6)=315"
              ]
              (Sicp_ch4_solutions.Sec_4_44.ex_4_44a ()))
        ] )
    ; ( "4.45"
      , [ test_case "4.45" `Quick (fun () ->
            check_strings
              "4.45"
              [ "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb-phrase (verb lectures) (prep-phrase \
                 (prep to) (simple-noun-phrase (article the) (noun student)))) \
                 (prep-phrase (prep in) (simple-noun-phrase (article the) (noun \
                 class)))) (prep-phrase (prep with) (simple-noun-phrase (article the) \
                 (noun cat)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 in) (noun-phrase (simple-noun-phrase (article the) (noun class)) \
                 (prep-phrase (prep with) (simple-noun-phrase (article the) (noun \
                 cat)))))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
                 (noun-phrase (simple-noun-phrase (article the) (noun student)) \
                 (prep-phrase (prep in) (simple-noun-phrase (article the) (noun \
                 class)))))) (prep-phrase (prep with) (simple-noun-phrase (article the) \
                 (noun cat)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase \
                 (noun-phrase (simple-noun-phrase (article the) (noun student)) \
                 (prep-phrase (prep in) (simple-noun-phrase (article the) (noun \
                 class)))) (prep-phrase (prep with) (simple-noun-phrase (article the) \
                 (noun cat)))))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase \
                 (simple-noun-phrase (article the) (noun student)) (prep-phrase (prep \
                 in) (noun-phrase (simple-noun-phrase (article the) (noun class)) \
                 (prep-phrase (prep with) (simple-noun-phrase (article the) (noun \
                 cat)))))))))"
              ; "Error: there are no more values"
              ]
              (Sicp_ch4_solutions.Sec_4_45.ex_4_45 ()))
        ] )
    ; ( "4.46"
      , [ test_case "4.46" `Quick (fun () ->
            check_strings
              "4.46"
              [ "(1 3)"; "(3 1)"; "(5 7)"; "(7 5)" ]
              (Sicp_ch4_solutions.Sec_4_46.ex_4_46 ()))
        ] )
    ; ( "4.47"
      , [ test_case "4.47" `Quick (fun () ->
            check_strings
              "4.47"
              [ "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase \
                 (simple-noun-phrase (article the) (noun student)) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))"
              ]
              (Sicp_ch4_solutions.Sec_4_47.ex_4_47 ()))
        ] )
    ; ( "4.48"
      , [ test_case "4.48" `Quick (fun () ->
            check_strings
              "4.48"
              [ "(sentence (simple-noun-phrase (article the) ((adj sleepy) (noun cat))) \
                 (verb eats))"
              ; "Error: there are no more values"
              ; "(sentence (simple-noun-phrase (article the) ((adj quick) (adj brown) \
                 (noun cat))) (verb sleeps))"
              ]
              (Sicp_ch4_solutions.Sec_4_48.ex_4_48 ()))
        ] )
    ; ( "4.49"
      , [ test_case "4.49" `Quick (fun () ->
            check_strings
              "4.49"
              [ "(sentence (simple-noun-phrase (article the) (noun student)) (verb \
                 studies))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb studies) (prep-phrase (prep for) (simple-noun-phrase \
                 (article the) (noun student)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb-phrase (verb studies) (prep-phrase (prep for) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 for) (simple-noun-phrase (article the) (noun student)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb-phrase (verb-phrase (verb studies) (prep-phrase \
                 (prep for) (simple-noun-phrase (article the) (noun student)))) \
                 (prep-phrase (prep for) (simple-noun-phrase (article the) (noun \
                 student)))) (prep-phrase (prep for) (simple-noun-phrase (article the) \
                 (noun student)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb-phrase (verb-phrase (verb-phrase (verb studies) \
                 (prep-phrase (prep for) (simple-noun-phrase (article the) (noun \
                 student)))) (prep-phrase (prep for) (simple-noun-phrase (article the) \
                 (noun student)))) (prep-phrase (prep for) (simple-noun-phrase (article \
                 the) (noun student)))) (prep-phrase (prep for) (simple-noun-phrase \
                 (article the) (noun student)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb-phrase (verb-phrase (verb-phrase (verb-phrase (verb \
                 studies) (prep-phrase (prep for) (simple-noun-phrase (article the) \
                 (noun student)))) (prep-phrase (prep for) (simple-noun-phrase (article \
                 the) (noun student)))) (prep-phrase (prep for) (simple-noun-phrase \
                 (article the) (noun student)))) (prep-phrase (prep for) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 for) (simple-noun-phrase (article the) (noun student)))))"
              ]
              (Sicp_ch4_solutions.Sec_4_49.ex_4_49 ()))
        ] )
    ; ( "4.50"
      , [ test_case "4.50" `Quick (fun () ->
            check_strings
              "4.50"
              [ "(sentence (simple-noun-phrase (article a) (noun class)) (verb sleeps))"
              ; "(sentence (simple-noun-phrase (article a) (noun class)) (verb-phrase \
                 (verb sleeps) (prep-phrase (prep with) (simple-noun-phrase (article a) \
                 (noun class)))))"
              ; "(sentence (simple-noun-phrase (article a) (noun class)) (verb-phrase \
                 (verb-phrase (verb sleeps) (prep-phrase (prep with) (simple-noun-phrase \
                 (article a) (noun class)))) (prep-phrase (prep with) \
                 (simple-noun-phrase (article a) (noun class)))))"
              ; "(sentence (simple-noun-phrase (article a) (noun class)) (verb-phrase \
                 (verb-phrase (verb-phrase (verb sleeps) (prep-phrase (prep with) \
                 (simple-noun-phrase (article a) (noun class)))) (prep-phrase (prep \
                 with) (simple-noun-phrase (article a) (noun class)))) (prep-phrase \
                 (prep with) (simple-noun-phrase (article a) (noun class)))))"
              ; "(sentence (simple-noun-phrase (article a) (noun class)) (verb-phrase \
                 (verb-phrase (verb-phrase (verb-phrase (verb sleeps) (prep-phrase (prep \
                 with) (simple-noun-phrase (article a) (noun class)))) (prep-phrase \
                 (prep with) (simple-noun-phrase (article a) (noun class)))) \
                 (prep-phrase (prep with) (simple-noun-phrase (article a) (noun \
                 class)))) (prep-phrase (prep with) (simple-noun-phrase (article a) \
                 (noun class)))))"
              ; "(sentence (simple-noun-phrase (article a) (noun class)) (verb-phrase \
                 (verb-phrase (verb-phrase (verb-phrase (verb-phrase (verb sleeps) \
                 (prep-phrase (prep with) (simple-noun-phrase (article a) (noun \
                 class)))) (prep-phrase (prep with) (simple-noun-phrase (article a) \
                 (noun class)))) (prep-phrase (prep with) (simple-noun-phrase (article \
                 a) (noun class)))) (prep-phrase (prep with) (simple-noun-phrase \
                 (article a) (noun class)))) (prep-phrase (prep with) \
                 (simple-noun-phrase (article a) (noun class)))))"
              ]
              (Sicp_ch4_solutions.Sec_4_50.ex_4_50 ()))
        ] )
    ; ( "4.51"
      , [ test_case "4.51" `Quick (fun () ->
            check_strings
              "4.51"
              [ "(a b 2)"; "(a c 3)"; "3"; "(a b 1)"; "(a c 2)" ]
              (Sicp_ch4_solutions.Sec_4_51.ex_4_51 ()))
        ] )
    ; ( "4.52"
      , [ test_case "4.52" `Quick (fun () ->
            check_strings
              "4.52"
              [ "all-odd"
              ; "Error: there are no more values"
              ; "8"
              ; "all-odd"
              ; "Error: there are no more values"
              ]
              (Sicp_ch4_solutions.Sec_4_52.ex_4_52 ()))
        ] )
    ; ( "4.53"
      , [ test_case "4.53" `Quick (fun () ->
            check_strings
              "4.53"
              [ "((8 35) (3 110) (3 20))"; "Error: there are no more values" ]
              (Sicp_ch4_solutions.Sec_4_53.ex_4_53 ()))
        ] )
    ; ( "4.54"
      , [ test_case "4.54" `Quick (fun () ->
            check_strings
              "4.54"
              [ "ok"
              ; "Error: there are no more values"
              ; "2"
              ; "4"
              ; "Error: there are no more values"
              ]
              (Sicp_ch4_solutions.Sec_4_54.ex_4_54 ()))
        ] )
    ]
;;
