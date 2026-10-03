(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 4.3. Each exercise's demonstration is pinned to the exact
   transcript the search experiment produces: the successful branches'
   output in answer order, then the answer, choice, and failure counts
   of the complete search. The invariants below hold across
   demonstrations: reordering a search never changes its size, and the
   binary choice tree ties the three counts together. *)

let check_strings = Alcotest.(check (list string))

let count name lines =
  let prefix = name ^ ": " in
  let prefix_length = String.length prefix in
  match
    List.find_opt
      (fun line ->
         String.length line > prefix_length && String.sub line 0 prefix_length = prefix)
      lines
  with
  | Some line ->
    int_of_string (String.sub line prefix_length (String.length line - prefix_length))
  | None -> Alcotest.failf "no %s count in the transcript" name
;;

let counts lines = count "answers" lines, count "choices" lines, count "failures" lines

(* Every attempt of the search ends as an answer or a failure, and each
   binary choice point turns one pending path into two. *)
let binary_tree_identity () =
  List.iter
    (fun lines ->
       let answers, choices, failures = counts lines in
       Alcotest.(check int)
         "answers + failures = choices + 1"
         (choices + 1)
         (answers + failures))
    [ Sicp_ch4_solutions.Sec_4_35.ex_4_35 ()
    ; Sicp_ch4_solutions.Sec_4_38.ex_4_38 ()
    ; Sicp_ch4_solutions.Sec_4_42.ex_4_42 ()
    ; Sicp_ch4_solutions.Sec_4_45.ex_4_45 ()
    ; Sicp_ch4_solutions.Sec_4_53.ex_4_53 ()
    ]
;;

(* [ramb] reorders the capped generation of 4.49 without changing its
   size: the same answers, choice points, and failures. *)
let ramb_preserves_search_size () =
  let amb_counts = counts (Sicp_ch4_solutions.Sec_4_49.ex_4_49 ()) in
  let ramb_lines = Sicp_ch4_solutions.Sec_4_50.ex_4_50 () in
  Alcotest.(check (triple int int int)) "ramb search size" amb_counts (counts ramb_lines)
;;

let () =
  let open Alcotest in
  run
    "Sec_4_3"
    [ ( "invariants"
      , [ test_case "binary choice tree" `Quick binary_tree_identity
        ; test_case "ramb preserves the search size" `Quick ramb_preserves_search_size
        ] )
    ; ( "4.35"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.35"
              [ "(3 4 5)"
              ; "(5 12 13)"
              ; "(6 8 10)"
              ; "(8 15 17)"
              ; "(9 12 15)"
              ; "(12 16 20)"
              ; "answers: 6"
              ; "choices: 1770"
              ; "failures: 1765"
              ]
              (Sicp_ch4_solutions.Sec_4_35.ex_4_35 ()))
        ] )
    ; ( "4.36"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.36"
              [ "fair_triple_from ceiling 20"
              ; "(3 4 5)"
              ; "(6 8 10)"
              ; "(5 12 13)"
              ; "(9 12 15)"
              ; "(8 15 17)"
              ; "(12 16 20)"
              ; "answers: 6"
              ; "choices: 1540"
              ; "failures: 1535"
              ; "fair_triple_from ceiling 30"
              ; "(3 4 5)"
              ; "(6 8 10)"
              ; "(5 12 13)"
              ; "(9 12 15)"
              ; "(8 15 17)"
              ; "(12 16 20)"
              ; "(7 24 25)"
              ; "(15 20 25)"
              ; "(10 24 26)"
              ; "(20 21 29)"
              ; "(18 24 30)"
              ; "answers: 11"
              ; "choices: 4960"
              ; "failures: 4950"
              ; "naive_triple_from ceiling 20"
              ; "(3 4 5)"
              ; "(5 12 13)"
              ; "(6 8 10)"
              ; "(8 15 17)"
              ; "(9 12 15)"
              ; "(12 16 20)"
              ; "answers: 6"
              ; "choices: 1770"
              ; "failures: 1765"
              ; "naive_triple_from ceiling 30"
              ; "(3 4 5)"
              ; "(5 12 13)"
              ; "(6 8 10)"
              ; "(7 24 25)"
              ; "(8 15 17)"
              ; "(9 12 15)"
              ; "(10 24 26)"
              ; "(12 16 20)"
              ; "(15 20 25)"
              ; "(18 24 30)"
              ; "(20 21 29)"
              ; "answers: 11"
              ; "choices: 5455"
              ; "failures: 5445"
              ]
              (Sicp_ch4_solutions.Sec_4_36.ex_4_36 ()))
        ] )
    ; ( "4.37"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.37"
              [ "(3 4 5)"
              ; "(5 12 13)"
              ; "(6 8 10)"
              ; "(8 15 17)"
              ; "(9 12 15)"
              ; "(12 16 20)"
              ; "answers: 6"
              ; "choices: 1770"
              ; "failures: 1765"
              ; "(3 4 5)"
              ; "(5 12 13)"
              ; "(6 8 10)"
              ; "(8 15 17)"
              ; "(9 12 15)"
              ; "(12 16 20)"
              ; "answers: 6"
              ; "choices: 230"
              ; "failures: 225"
              ]
              (Sicp_ch4_solutions.Sec_4_37.ex_4_37 ()))
        ] )
    ; ( "4.38"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.38"
              [ "((baker 1) (cooper 2) (fletcher 4) (miller 3) (smith 5))"
              ; "((baker 1) (cooper 2) (fletcher 4) (miller 5) (smith 3))"
              ; "((baker 1) (cooper 4) (fletcher 2) (miller 5) (smith 3))"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "((baker 3) (cooper 4) (fletcher 2) (miller 5) (smith 1))"
              ; "answers: 5"
              ; "choices: 3124"
              ; "failures: 3120"
              ]
              (Sicp_ch4_solutions.Sec_4_38.ex_4_38 ()))
        ] )
    ; ( "4.39"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.39"
              [ "multiple_dwelling"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "answers: 1"
              ; "choices: 3124"
              ; "failures: 3124"
              ; "multiple_dwelling_cheap_first"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "answers: 1"
              ; "choices: 3124"
              ; "failures: 3124"
              ; "multiple_dwelling_reordered"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "answers: 1"
              ; "choices: 308"
              ; "failures: 308"
              ]
              (Sicp_ch4_solutions.Sec_4_39.ex_4_39 ()))
        ] )
    ; ( "4.40"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.40"
              [ "before distinct"
              ; "answers: 3125"
              ; "choices: 3905"
              ; "failures: 781"
              ; "after distinct"
              ; "answers: 120"
              ; "choices: 3905"
              ; "failures: 3786"
              ; "multiple_dwelling"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "answers: 1"
              ; "choices: 3905"
              ; "failures: 3905"
              ; "multiple_dwelling_pruned"
              ; "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "answers: 1"
              ; "choices: 34"
              ; "failures: 34"
              ]
              (Sicp_ch4_solutions.Sec_4_40.ex_4_40 ()))
        ] )
    ; ( "4.41"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.41"
              [ "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
              ; "solutions=1"
              ]
              (Sicp_ch4_solutions.Sec_4_41.ex_4_41 ()))
        ] )
    ; ( "4.42"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.42"
              [ "((betty 3) (ethel 5) (joan 2) (kitty 1) (mary 4))"
              ; "answers: 1"
              ; "choices: 3905"
              ; "failures: 3905"
              ]
              (Sicp_ch4_solutions.Sec_4_42.ex_4_42 ()))
        ] )
    ; ( "4.43"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.43"
              [ "told"
              ; "(lornas-father downing)"
              ; "answers: 1"
              ; "choices: 40"
              ; "failures: 40"
              ; "open"
              ; "(lornas-father downing)"
              ; "(lornas-father parker)"
              ; "answers: 2"
              ; "choices: 152"
              ; "failures: 151"
              ]
              (Sicp_ch4_solutions.Sec_4_43.ex_4_43 ()))
        ] )
    ; ( "4.44"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.44"
              [ "(4 2 7 3 6 8 5 1)"
              ; "answers: 92"
              ; "choices: 15720"
              ; "failures: 15629"
              ; "(3 1 4 2)"
              ; "answers: 2"
              ; "choices: 60"
              ; "failures: 59"
              ; "(5 3 1 6 4 2)"
              ; "answers: 4"
              ; "choices: 894"
              ; "failures: 891"
              ]
              (Sicp_ch4_solutions.Sec_4_44.ex_4_44 ()))
        ] )
    ; ( "4.44a"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.44a"
              [ "board 4"
              ; "answers: 2"
              ; "choices: 60"
              ; "failures: 59"
              ; "board 5"
              ; "answers: 10"
              ; "choices: 220"
              ; "failures: 211"
              ; "board 6"
              ; "answers: 4"
              ; "choices: 894"
              ; "failures: 891"
              ]
              (Sicp_ch4_solutions.Sec_4_44.ex_4_44a ()))
        ] )
    ; ( "4.45"
      , [ test_case "transcript" `Quick (fun () ->
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
              ; "answers: 5"
              ; "choices: 23"
              ; "failures: 19"
              ]
              (Sicp_ch4_solutions.Sec_4_45.ex_4_45 ()))
        ] )
    ; ( "4.46"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.46"
              [ "operand order"
              ; "(1 3)"
              ; "(1 4)"
              ; "(2 3)"
              ; "(2 4)"
              ; "answers: 4"
              ; "choices: 3"
              ; "failures: 0"
              ; "left to right"
              ; "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))"
              ; "answers: 1"
              ; "choices: 0"
              ; "failures: 0"
              ; "right to left"
              ; "answers: 0"
              ; "choices: 0"
              ; "failures: 1"
              ]
              (Sicp_ch4_solutions.Sec_4_46.ex_4_46 ()))
        ] )
    ; ( "4.47"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.47"
              [ "louis cat-eats depth_limit 4"
              ; "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))"
              ; "answers: 1"
              ; "choices: 6"
              ; "failures: 6"
              ; "louis cat-eats depth_limit 8"
              ; "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))"
              ; "answers: 1"
              ; "choices: 10"
              ; "failures: 10"
              ; "louis professor depth_limit 4"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase \
                 (simple-noun-phrase (article the) (noun student)) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))"
              ; "answers: 2"
              ; "choices: 21"
              ; "failures: 20"
              ; "louis professor depth_limit 8"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase \
                 (simple-noun-phrase (article the) (noun student)) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))"
              ; "answers: 2"
              ; "choices: 41"
              ; "failures: 40"
              ; "interchanged professor depth_limit 4"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase \
                 (simple-noun-phrase (article the) (noun student)) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))))"
              ; "answers: 2"
              ; "choices: 21"
              ; "failures: 20"
              ; "interchanged professor depth_limit 8"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
                 (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun professor)) \
                 (verb-phrase (verb lectures) (prep-phrase (prep to) (noun-phrase \
                 (simple-noun-phrase (article the) (noun student)) (prep-phrase (prep \
                 with) (simple-noun-phrase (article the) (noun cat)))))))"
              ; "answers: 2"
              ; "choices: 41"
              ; "failures: 40"
              ]
              (Sicp_ch4_solutions.Sec_4_47.ex_4_47 ()))
        ] )
    ; ( "4.48"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.48"
              [ "(sentence (simple-noun-phrase (article the) ((adj sleepy) (noun cat))) \
                 (verb eats))"
              ; "answers: 1"
              ; "choices: 4"
              ; "failures: 4"
              ; "(sentence (simple-noun-phrase (article the) ((adj quick) (adj brown) \
                 (noun cat))) (verb sleeps))"
              ; "answers: 1"
              ; "choices: 5"
              ; "failures: 5"
              ]
              (Sicp_ch4_solutions.Sec_4_48.ex_4_48 ()))
        ] )
    ; ( "4.49"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.49"
              [ "(sentence (simple-noun-phrase (article the) (noun student)) (verb \
                 studies))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb studies) (prep-phrase (prep for) (simple-noun-phrase \
                 (article the) (noun student)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb studies) (prep-phrase (prep for) (simple-noun-phrase \
                 (article the) (noun professor)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb studies) (prep-phrase (prep for) (simple-noun-phrase \
                 (article the) (noun cat)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb studies) (prep-phrase (prep for) (simple-noun-phrase \
                 (article the) (noun class)))))"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb studies) (prep-phrase (prep for) (simple-noun-phrase \
                 (article a) (noun student)))))"
              ; "answers: 2592"
              ; "choices: 8042"
              ; "failures: 5451"
              ]
              (Sicp_ch4_solutions.Sec_4_49.ex_4_49 ()))
        ] )
    ; ( "4.50"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.50"
              [ "seed 20260925"
              ; "(sentence (simple-noun-phrase (article the) (noun student)) \
                 (verb-phrase (verb eats) (prep-phrase (prep in) (simple-noun-phrase \
                 (article the) (noun class)))))"
              ; "answers: 2592"
              ; "choices: 8042"
              ; "failures: 5451"
              ; "seed 20260926"
              ; "(sentence (noun-phrase (simple-noun-phrase (article a) (noun student)) \
                 (prep-phrase (prep for) (simple-noun-phrase (article the) (noun \
                 student)))) (verb eats))"
              ; "answers: 2592"
              ; "choices: 8042"
              ; "failures: 5451"
              ; "seed 20260927"
              ; "(sentence (noun-phrase (simple-noun-phrase (article the) (noun \
                 student)) (prep-phrase (prep for) (simple-noun-phrase (article the) \
                 (noun class)))) (verb studies))"
              ; "answers: 2592"
              ; "choices: 8042"
              ; "failures: 5451"
              ; "seed 20260928"
              ; "(sentence (noun-phrase (simple-noun-phrase (article the) (noun cat)) \
                 (prep-phrase (prep to) (simple-noun-phrase (article a) (noun cat)))) \
                 (verb studies))"
              ; "answers: 2592"
              ; "choices: 8042"
              ; "failures: 5451"
              ]
              (Sicp_ch4_solutions.Sec_4_50.ex_4_50 ()))
        ] )
    ; ( "4.51"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.51"
              [ "permanent_set"
              ; "(a b 2)"
              ; "(a c 3)"
              ; "(b a 4)"
              ; "(b c 6)"
              ; "(c a 7)"
              ; "(c b 8)"
              ; "answers: 6"
              ; "choices: 12"
              ; "failures: 7"
              ; ":="
              ; "(a b 1)"
              ; "(a c 1)"
              ; "(b a 1)"
              ; "(b c 1)"
              ; "(c a 1)"
              ; "(c b 1)"
              ; "answers: 6"
              ; "choices: 12"
              ; "failures: 7"
              ]
              (Sicp_ch4_solutions.Sec_4_51.ex_4_51 ()))
        ] )
    ; ( "4.52"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.52"
              [ "all-odd"
              ; "answers: 1"
              ; "choices: 4"
              ; "failures: 4"
              ; "8"
              ; "all-odd"
              ; "answers: 2"
              ; "choices: 5"
              ; "failures: 4"
              ]
              (Sicp_ch4_solutions.Sec_4_52.ex_4_52 ()))
        ] )
    ; ( "4.53"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.53"
              [ "((8 35) (3 110) (3 20))"; "answers: 1"; "choices: 17"; "failures: 17" ]
              (Sicp_ch4_solutions.Sec_4_53.ex_4_53 ()))
        ] )
    ; ( "4.54"
      , [ test_case "transcript" `Quick (fun () ->
            check_strings
              "4.54"
              [ "ok"
              ; "answers: 1"
              ; "choices: 0"
              ; "failures: 0"
              ; "same as the experiment's require"
              ; "answers: 0"
              ; "choices: 0"
              ; "failures: 1"
              ; "same as the experiment's require"
              ; "2"
              ; "4"
              ; "answers: 2"
              ; "choices: 4"
              ; "failures: 3"
              ; "same as the experiment's require"
              ]
              (Sicp_ch4_solutions.Sec_4_54.ex_4_54 ()))
        ] )
    ]
;;
