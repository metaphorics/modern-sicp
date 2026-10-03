(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the section 5.5 compiler and its reference
   solutions.  Every pin below is the exercise's own entry point run to
   completion: combinations of 5.31 listing which saves [preserving]
   keeps, the fast path of 5.32 answering with fewer pushes, the two
   factorials of 5.33 sharing their work but saving different
   registers, the hand-optimized 5.33a winning in executed steps, the
   tail-call branches of 5.34 with the constant stack of the iterative
   factorial, the 5.35 reconstruction matching the figure statement for
   statement, the two operand orders of 5.36 printing [321] against
   [123], the blind 5.37 compilation tripling in size, the three
   open-coded operations of 5.38, the address lookups of 5.39 through
   5.42 with the nested example answering [180], the grouped
   definitions of 5.43 answering under lexical addressing, the scoped
   5.44 compiler fixing the shadowed call, the three machines of 5.45
   and 5.46 measured under one monitored stack, the mixed session of
   5.47 answering [12] through the compound branch, the 5.48 session
   compiling then calling, the 5.49 loop printing its four bindings, the
   compiled metacircular of 5.50 answering under three engines, and the
   C evaluator of 5.51 and the C back end of 5.52 answering the book's
   factorial session. *)

module Sec_5_31 = Sicp_ch5_solutions.Sec_5_31
module Sec_5_32 = Sicp_ch5_solutions.Sec_5_32
module Sec_5_33 = Sicp_ch5_solutions.Sec_5_33
module Sec_5_34 = Sicp_ch5_solutions.Sec_5_34
module Sec_5_35 = Sicp_ch5_solutions.Sec_5_35
module Sec_5_36 = Sicp_ch5_solutions.Sec_5_36
module Sec_5_37 = Sicp_ch5_solutions.Sec_5_37
module Sec_5_38 = Sicp_ch5_solutions.Sec_5_38
module Sec_5_39 = Sicp_ch5_solutions.Sec_5_39
module Sec_5_40 = Sicp_ch5_solutions.Sec_5_40
module Sec_5_41 = Sicp_ch5_solutions.Sec_5_41
module Sec_5_42 = Sicp_ch5_solutions.Sec_5_42
module Sec_5_43 = Sicp_ch5_solutions.Sec_5_43
module Sec_5_44 = Sicp_ch5_solutions.Sec_5_44
module Sec_5_45 = Sicp_ch5_solutions.Sec_5_45
module Sec_5_46 = Sicp_ch5_solutions.Sec_5_46
module Sec_5_47 = Sicp_ch5_solutions.Sec_5_47
module Sec_5_48 = Sicp_ch5_solutions.Sec_5_48
module Sec_5_49 = Sicp_ch5_solutions.Sec_5_49
module Sec_5_50 = Sicp_ch5_solutions.Sec_5_50
module Sec_5_51 = Sicp_ch5_solutions.Sec_5_51
module Sec_5_52 = Sicp_ch5_solutions.Sec_5_52

let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string

let pin name expected = function
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Sicp_common.Eval_error.to_string e)
;;

let ex_5_31 () =
  pin
    "5.31"
    [ "f 1 2: "
    ; "(pick true) 1 2: "
    ; "f (g 1) y: save proc; save env; save argl; restore argl; restore env; restore proc"
    ; "f (g 1) 2: save proc; save argl; restore argl; restore proc"
    ]
    (Sec_5_31.ex_5_31 ())
;;

let ex_5_32 () =
  pin
    "5.32"
    [ "factorial 5: base answers 120 with 105 pushes, depth 18; fast path answers 120 \
       with 95 pushes, depth 18"
    ; "(fun y -> y + 1) 41: base answers 42 with 10 pushes, depth 3; fast path answers \
       42 with 10 pushes, depth 3"
    ]
    (Sec_5_32.ex_5_32 ())
;;

let ex_5_33 () =
  pin
    "5.33"
    [ "factorial: 40 statements; save continue; save env; restore env; restore continue; \
       answers 120 in 191 steps"
    ; "factorial_alt: 40 statements; save continue; save arg1; restore arg1; restore \
       continue; answers 120 in 191 steps"
    ]
    (Sec_5_33.ex_5_33 ())
;;

let ex_5_33a () =
  pin
    "5.33a"
    [ "naive steps: 191"
    ; "optimized steps: 111"
    ; "naive answer: 120"
    ; "optimized answer: 120"
    ; "instruction win: 80 of 191 (41.9%)"
    ]
    (Sec_5_33.ex_5_33a ())
;;

let contains haystack needle =
  let n = String.length haystack
  and m = String.length needle in
  let rec go i =
    if i + m > n
    then false
    else if String.sub haystack i m = needle
    then true
    else go (i + 1)
  in
  go 0
;;

let ex_5_34 () =
  match Sec_5_34.ex_5_34 () with
  | Ok lines ->
    Alcotest.check Alcotest.int "5.34 line count" 7 (List.length lines);
    strings
      "5.34 calls"
      [ "iterative call: Label \"compiled-branch12\"; Goto \"compiled-apply\""
      ; "iterative call: Label \"compiled-branch4\"; Goto \"compiled-apply\""
      ; "recursive call: Label \"compiled-branch7\"; Assign (\"continue\", Label_ref \
         \"proc-return9\"); Goto \"compiled-apply\""
      ]
      (List.filteri (fun i _ -> i < 3) lines);
    Alcotest.check
      Alcotest.bool
      "5.34 tail calls assign no continue"
      true
      ((not (contains (List.nth lines 0) "Assign (\"continue\""))
       && not (contains (List.nth lines 1) "Assign (\"continue\""));
    Alcotest.check
      Alcotest.bool
      "5.34 recursive call assigns continue"
      true
      (contains (List.nth lines 2) "Assign (\"continue\"");
    the_string "5.34 saves" "saves in the iterative compilation: 0" (List.nth lines 3);
    strings
      "5.34 depths"
      [ "depth at n = 3: iterative 2, recursive 6"
      ; "depth at n = 4: iterative 2, recursive 8"
      ; "depth at n = 5: iterative 2, recursive 10"
      ]
      (List.filteri (fun i _ -> i >= 4) lines)
  | Error e -> Alcotest.fail (Sicp_common.Eval_error.to_string e)
;;

let ex_5_35 () =
  match Sec_5_35.ex_5_35 () with
  | Ok lines ->
    the_string
      "5.35 source"
      "compiled to the figure: let f x = x + g (x + 2)"
      (List.nth lines 0);
    the_string "5.35 verdict" "figure matches: true" (List.nth (List.rev lines) 0);
    strings
      "5.35 figure"
      Sec_5_35.figure
      (List.filteri (fun i _ -> i > 0 && i < List.length lines - 1) lines)
  | Error e -> Alcotest.fail (Sicp_common.Eval_error.to_string e)
;;

let ex_5_36 () =
  pin
    "5.36"
    [ "default order: 123"
    ; "right-to-left order: 321"
    ; "statements: 154 and 154; steps: 166 and 166"
    ; "answers: 6 and 6"
    ]
    (Sec_5_36.ex_5_36 ())
;;

let ex_5_37 () =
  pin
    "5.37"
    [ "f (g 1) 2 with preserving: Save \"proc\"; Save \"argl\"; Restore \"argl\"; \
       Restore \"proc\""
    ; "f (g 1) 2 without: Save \"continue\"; Save \"env\"; Save \"continue\"; Restore \
       \"continue\"; Restore \"env\"; Restore \"continue\"; Save \"continue\"; Save \
       \"proc\"; Save \"env\"; Save \"env\"; Restore \"env\"; Save \"argl\"; Save \
       \"continue\"; Save \"env\"; Save \"continue\"; Restore \"continue\"; Restore \
       \"env\"; Restore \"continue\"; Save \"continue\"; Save \"proc\"; Save \"env\"; \
       Restore \"env\"; Save \"argl\"; Save \"continue\"; Restore \"continue\"; Restore \
       \"argl\"; Restore \"proc\"; Restore \"continue\"; Save \"continue\"; Restore \
       \"continue\"; Restore \"argl\"; Restore \"env\"; Save \"argl\"; Save \
       \"continue\"; Restore \"continue\"; Restore \"argl\"; Restore \"proc\"; Restore \
       \"continue\"; Save \"continue\"; Restore \"continue\""
    ; "factorial with preserving: 40 statements, 4 saves/restores"
    ; "factorial without: 88 statements, 52 saves/restores"
    ; "factorial 5 with preserving: answers 120 with 10 pushes, depth 10"
    ; "factorial 5 without: answers 120 with 111 pushes, depth 16"
    ]
    (Sec_5_37.ex_5_37 ())
;;

let ex_5_38 () =
  pin
    "5.38"
    [ "plain: 181 statements, 0 open-coded operations; answers \"35\" in 397 steps"
    ; "open-coded: 134 statements, 3 open-coded operations; answers \"35\" in 284 steps"
    ; "Assign_op (\"arg2\", \"Array.length\", [Reg \"arg1\"])"
    ; "Assign_op (\"arg1\", \"Array.get\", [Reg \"arg1\"; Reg \"arg2\"])"
    ]
    (Sec_5_38.ex_5_38 ())
;;

let ex_5_39 () =
  pin
    "5.39"
    [ "c at (1, 2): 13"
    ; "x at (2, 0): 1"
    ; "y at (0, 0): 21"
    ; "y at (2, 1): 2"
    ; "loop before its group is filled: error: bad instruction: lexical address (0, 0) \
       of loop is unassigned"
    ]
    (Sec_5_39.ex_5_39 ())
;;

let ex_5_40 () =
  pin
    "5.40"
    [ "x in [y; z] [a; b; c; d; e] [x; y]"
    ; "y in [y; z] [a; b; c; d; e] [x; y]"
    ; "z in [y; z] [a; b; c; d; e] [x; y]"
    ; "a in [a; b; c; d; e] [x; y]"
    ; "b in [a; b; c; d; e] [x; y]"
    ; "x in [a; b; c; d; e] [x; y]"
    ; "c in [a; b; c; d; e] [x; y]"
    ; "d in [a; b; c; d; e] [x; y]"
    ; "x in [a; b; c; d; e] [x; y]"
    ]
    (Sec_5_40.ex_5_40 ())
;;

let ex_5_41 () =
  pin "5.41" [ "c: (1, 2)"; "x: (2, 0)"; "w: not found" ] (Sec_5_41.ex_5_41 ())
;;

let ex_5_42 () =
  match Sec_5_42.ex_5_42 () with
  | Ok lines ->
    the_string "5.42 run" "lexical run: 180" (List.nth (List.rev lines) 0);
    Alcotest.check Alcotest.int "5.42 lookups" 9 (List.length lines - 1)
  | Error e -> Alcotest.fail (Sicp_common.Eval_error.to_string e)
;;

let ex_5_43 () =
  pin
    "5.43"
    [ "group code in order: let-rec-group, group-environment, fill-first-pending, \
       fill-first-pending"
    ; "plain run: 3"
    ; "lexical run: 3"
    ]
    (Sec_5_43.ex_5_43 ())
;;

let ex_5_44 () =
  pin
    "5.44"
    [ "shadowed parameter: 5.38 compiler: 1 open-coded, answers \"42\"; scoped: 0 \
       open-coded, answers \"large\""
    ; "free name: 5.38 compiler: 1 open-coded, answers \"42\"; scoped: 1 open-coded, \
       answers \"42\""
    ]
    (Sec_5_44.ex_5_44 ())
;;

let ex_5_45 () =
  pin
    "5.45"
    [ "n = 5: interpreted 105/18, compiled 10/10, special 8/8; ratios compiled \
       0.095/0.556, special 0.076/0.444"
    ; "n = 10: interpreted 220/33, compiled 20/20, special 18/18; ratios compiled \
       0.091/0.606, special 0.082/0.545"
    ]
    (Sec_5_45.ex_5_45 ())
;;

let ex_5_46 () =
  pin
    "5.46"
    [ "n = 5: interpreted 300/18, compiled 23/10, special 28/8; ratios compiled \
       0.077/0.556, special 0.093/0.444"
    ; "n = 6: interpreted 505/21, compiled 38/12, special 48/10; ratios compiled \
       0.075/0.571, special 0.095/0.476"
    ; "n = 7: interpreted 833/24, compiled 62/14, special 80/12; ratios compiled \
       0.074/0.583, special 0.096/0.500"
    ]
    (Sec_5_46.ex_5_46 ())
;;

let ex_5_47 () =
  pin
    "5.47"
    [ "compound branch: Test (\"compound-procedure?\", [Reg \"proc\"]); Branch \
       \"ca-compound\"; Perform (\"signal-not-applicable\", [Reg \"proc\"]); Label \
       \"ca-compound\"; Goto \"apply-entry\""
    ; "without the branch: error: not applicable: closure is not a procedure"
    ; "with the branch: 12"
    ; "all compiled with the branch: 12"
    ]
    (Sec_5_47.ex_5_47 ())
;;

let ex_5_48 () =
  pin
    "5.48"
    [ "compile-and-run: factorial = compiled procedure entry1"
    ; "evaluate: double = closure"
    ; "evaluate: result = 120"
    ]
    (Sec_5_48.ex_5_48 ())
;;

let ex_5_49 () =
  pin
    "5.49"
    [ "fib = compiled procedure entry1"
    ; "a = 144"
    ; "double = compiled procedure entry17"
    ; "b = 882"
    ]
    (Sec_5_49.ex_5_49 ())
;;

let ex_5_50 () =
  pin
    "5.50"
    [ "metacircular answers: compiled 120, explicit-control 120, direct 120; counter 3"
    ; "native oracle agrees: 120"
    ; "native counter agrees: 3"
    ; "level 0 (compiled factorial): 120 in 1707 machine steps"
    ; "level 1 (factorial on the explicit-control evaluator): 120 in 11082 machine steps"
    ; "level 2 (factorial on the compiled metacircular evaluator): 120 in 55149 machine \
       steps"
    ; "interpretation price: level 1 is 6 times level 0, level 2 is 32 times level 0"
    ]
    (Sec_5_50.ex_5_50 ())
;;

let ex_5_51 () =
  pin
    "5.51"
    [ "factorial: C evaluator \"120\", direct \"120\"; pushes 110, maximum depth 21"
    ; "list length: C evaluator \"3\", direct \"3\"; pushes 67, maximum depth 15"
    ; "counter: C evaluator \"5 12\", direct \"5 12\"; pushes 78, maximum depth 17"
    ]
    (Sec_5_51.ex_5_51 ())
;;

let ex_5_52 () =
  match Sec_5_52.ex_5_52 () with
  | Ok lines ->
    the_string
      "5.52 outputs"
      "compiled metacircular in C: \"120\\n\" (machine \"120\\n\"); counter \"3\\n\""
      (List.nth lines 0);
    the_string "5.52 native" "native oracle: \"120\\n\"" (List.nth lines 1);
    Alcotest.check Alcotest.int "5.52 lines" 3 (List.length lines)
  | Error e -> Alcotest.fail (Sicp_common.Eval_error.to_string e)
;;

let () =
  Alcotest.run
    "sec_5_5"
    [ ( "5.5"
      , [ Alcotest.test_case "5.31" `Quick ex_5_31
        ; Alcotest.test_case "5.32" `Quick ex_5_32
        ; Alcotest.test_case "5.33" `Quick ex_5_33
        ; Alcotest.test_case "5.33a" `Quick ex_5_33a
        ; Alcotest.test_case "5.34" `Quick ex_5_34
        ; Alcotest.test_case "5.35" `Quick ex_5_35
        ; Alcotest.test_case "5.36" `Quick ex_5_36
        ; Alcotest.test_case "5.37" `Quick ex_5_37
        ; Alcotest.test_case "5.38" `Quick ex_5_38
        ; Alcotest.test_case "5.39" `Quick ex_5_39
        ; Alcotest.test_case "5.40" `Quick ex_5_40
        ; Alcotest.test_case "5.41" `Quick ex_5_41
        ; Alcotest.test_case "5.42" `Quick ex_5_42
        ; Alcotest.test_case "5.43" `Quick ex_5_43
        ; Alcotest.test_case "5.44" `Quick ex_5_44
        ; Alcotest.test_case "5.45" `Quick ex_5_45
        ; Alcotest.test_case "5.46" `Quick ex_5_46
        ; Alcotest.test_case "5.47" `Quick ex_5_47
        ; Alcotest.test_case "5.48" `Quick ex_5_48
        ; Alcotest.test_case "5.49" `Quick ex_5_49
        ; Alcotest.test_case "5.50" `Quick ex_5_50
        ; Alcotest.test_case "5.51" `Quick ex_5_51
        ; Alcotest.test_case "5.52" `Quick ex_5_52
        ] )
    ]
;;
