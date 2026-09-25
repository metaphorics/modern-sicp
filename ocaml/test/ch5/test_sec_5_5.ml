(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the section 5.5 compiler and its reference
   solutions.  Every pin below was computed by running the exercise's
   own entry point; the brief-named pins (5.33a's 222/125 measurement,
   5.47's compound-call session answering 12, 5.35's Figure 5.18
   compilation, 5.41's lexical addresses, 5.50's compiled metacircular
   answering 120) are all pinned here.  5.51 and 5.52 build their C
   translations with the system compiler and run them; their outputs
   are the book's answers. *)

module Eval = Sicp_ch5.Sec_5_5
module Sec_5_31 = Sicp_ch5_solutions.Sec_5_31
module Sec_5_32 = Sicp_ch5_solutions.Sec_5_32
module Sec_5_33 = Sicp_ch5_solutions.Sec_5_33
module Sec_5_34 = Sicp_ch5_solutions.Sec_5_34
module Sec_5_35 = Sicp_ch5_solutions.Sec_5_35
module Sec_5_36 = Sicp_ch5_solutions.Sec_5_36
module Sec_5_37 = Sicp_ch5_solutions.Sec_5_37
module Sec_5_38 = Sicp_ch5_solutions.Sec_5_38
module Sec_5_40 = Sicp_ch5_solutions.Sec_5_40
module Sec_5_41 = Sicp_ch5_solutions.Sec_5_41
module Sec_5_44 = Sicp_ch5_solutions.Sec_5_44
module Sec_5_47 = Sicp_ch5_solutions.Sec_5_47
module Sec_5_50 = Sicp_ch5_solutions.Sec_5_50
module Sec_5_51 = Sicp_ch5_solutions.Sec_5_51
module Sec_5_52 = Sicp_ch5_solutions.Sec_5_52

let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string

let strings_outcome name expected = function
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

(* [has_prefix name index prefix lines] checks lines[index] begins with
   [prefix]; used where an outcome quotes a wall-clock measurement. *)
let has_prefix name index prefix lines =
  match List.nth_opt lines index with
  | Some line ->
    let n = String.length prefix in
    if String.length line >= n && String.sub line 0 n = prefix
    then ()
    else Alcotest.failf "%s: %S does not start with %S" name line prefix
  | None -> Alcotest.failf "%s: no line %d" name index
;;

(* 5.31: which combinations push which registers around their operand
   evaluations -- the book's three-case analysis. *)
let ex_5_31 () =
  strings_outcome
    "5.31"
    [ "(f 'x 'y): "
    ; "((f) 'x 'y): (save env); (restore env)"
    ; "(f (g 'x) y): (save proc); (save argl); (restore argl); (restore proc)"
    ; "(f (g 'x) 'y): (save proc); (save argl); (restore argl); (restore proc)"
    ]
    (Sec_5_31.ex_5_31 ())
;;

(* 5.32: the symbol-operator fast path answers as the base evaluator,
   and the base monitored factorial at n = 5 costs the book's 144
   pushes. *)
let ex_5_32 () =
  match Sec_5_32.ex_5_32 () with
  | Ok lines ->
    has_prefix "5.32 transcript" 0 ";;; EC-Eval input:" lines;
    has_prefix
      "5.32 fast-path answers"
      0
      ";;; EC-Eval input:\n\
       ;;; EC-Eval value:\n\
       ok\n\
       ;;; EC-Eval input:\n\
       ;;; EC-Eval value:\n\
       36"
      lines;
    the_string "5.32 base pushes" "base monitored pushes at n = 5: 144" (List.nth lines 1)
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

(* 5.33: the alternative factorial keeps an extra env save around the
   recursive argument; both compilations answer 120. *)
let ex_5_33 () =
  strings_outcome
    "5.33"
    [ "factorial saves: (save continue); (save env); (restore env); (restore continue); \
       (save continue); (save proc); (save argl); (save proc); (restore proc); (restore \
       argl); (restore proc); (restore continue)"
    ; "factorial-alt saves: (save continue); (save env); (restore env); (restore \
       continue); (save continue); (save proc); (save env); (save proc); (restore proc); \
       (restore env); (restore proc); (restore continue)"
    ; "factorial 5: ;;; EC-Eval value: 120 ;;; EC-Eval input:"
    ; "factorial-alt 5: ;;; EC-Eval value: 120 ;;; EC-Eval input:"
    ]
    (Sec_5_33.ex_5_33 ())
;;

(* 5.33a: the hand-optimized alternative drops the instruction count
   from 222 to 125, both answer 120, and the win is 97 of 222. *)
let ex_5_33a () =
  strings_outcome
    "5.33a"
    [ "naive steps: 222"
    ; "optimized steps: 125"
    ; "naive transcript: ;;; EC-Eval value: 120 ;;; EC-Eval input:"
    ; "optimized transcript: ;;; EC-Eval value: 120 ;;; EC-Eval input:"
    ; "instruction win: 97 of 222 (43.7%)"
    ]
    (Sec_5_33.ex_5_33a ())
;;

(* 5.34: the iterative factorial's call compiles to a compiled-procedure
   dispatch, and the monitored depth stays 0 at every measured n. *)
let ex_5_34 () =
  strings_outcome
    "5.34"
    [ "iter tail call: (test (op primitive-procedure?) (reg proc)); (branch (label \
       primitive-branch)); compiled-branch; (assign val (op compiled-procedure-entry) \
       (reg proc)); (goto (reg val))"
    ; "depth at n = 3: 0"
    ; "depth at n = 4: 0"
    ; "depth at n = 5: 0"
    ]
    (Sec_5_34.ex_5_34 ())
;;

(* 5.35: the compilation reproduces the book's Figure 5.18 listing
   statement for statement. *)
let ex_5_35 () =
  match Sec_5_35.ex_5_35 () with
  | Ok lines ->
    the_string
      "5.35 source"
      "compiled to the figure: (define (f x) (+ x (g (+ x 2))))"
      (List.nth lines 0);
    the_string "5.35 verdict" "figure matches: true" (List.nth (List.rev lines) 0)
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

(* 5.36: the compiler evaluates operands right to left; flipping
   construct-arglist's reverse flips the recorded order and leaves the
   instruction count untouched. *)
let ex_5_36 () =
  strings_outcome
    "5.36"
    [ "default order: 2 1"
    ; "left-to-right order: 1 2"
    ; "instruction counts: 42 = 42: true"
    ]
    (Sec_5_36.ex_5_36 ())
;;

(* 5.37: disabling preserving grows the factorial compilation from 79
   statements with 12 saves/restores to 149 with 82. *)
let ex_5_37 () =
  strings_outcome
    "5.37"
    [ "with preserving: 79 statements, 12 saves/restores"
    ; "without: 149 statements, 82 saves/restores"
    ; "monitored with: "
    ; "monitored without: "
    ]
    (Sec_5_37.ex_5_37 ())
;;

(* 5.38: open-coding the primitive compounds shrinks the compilation
   from 79 to 42 statements and every open-coded call answers. *)
let ex_5_38 () =
  strings_outcome
    "5.38"
    [ "plain compilation: 79 statements"
    ; "open-coded compilation: 42 statements"
    ; "factorial 5: ;;; EC-Eval value: ok ;;; EC-Eval input: ;;; EC-Eval value: 120 ;;; \
       EC-Eval input:"
    ; "(+ 1 2 3 4): ;;; EC-Eval value: ok ;;; EC-Eval input: ;;; EC-Eval value: 10 ;;; \
       EC-Eval input:"
    ; "(< 1 2): ;;; EC-Eval value: ok ;;; EC-Eval input: ;;; EC-Eval value: #t ;;; \
       EC-Eval input:"
    ]
    (Sec_5_38.ex_5_38 ())
;;

(* 5.40: the lexical-address mapping for the book's three-run example. *)
let ex_5_40 () =
  strings_outcome
    "5.40"
    [ "z in (y z) (a b c d e) (x y)"
    ; "y in (y z) (a b c d e) (x y)"
    ; "x in (y z) (a b c d e) (x y)"
    ; "+ in (y z) (a b c d e) (x y)"
    ]
    (Sec_5_40.ex_5_40 ())
;;

(* 5.41: the book's three lookup examples answer (1 2), (2 0), and
   not-found. *)
let ex_5_41 () =
  strings_outcome "5.41" [ "c: (1 2)"; "x: (2 0)"; "w: not-found" ] (Sec_5_41.ex_5_41 ())
;;

(* 5.44: the open-coding analysis of the compiled set!-procedure: no
   open-coded operation under either shadowing or free names. *)
let ex_5_44 () =
  strings_outcome
    "5.44"
    [ "shadowed parameters: 0 open-coded operations"
    ; "free names: 0 open-coded operations"
    ]
    (Sec_5_44.ex_5_44 ())
;;

(* 5.47: the compound-call branch rides through unev, and the book's
   session answers 12 through it. *)
let ex_5_47 () =
  strings_outcome
    "5.47"
    [ "compound branch instructions: (test (op compound-procedure?) (reg proc)); (branch \
       (label compound-branch6)); (assign unev (label compound-apply)); (goto (reg \
       unev)); (test (op compound-procedure?) (reg proc)); (branch (label \
       compound-branch10)); (assign unev (label compound-apply)); (goto (reg unev))"
    ; "session: ;;; EC-Eval value: ok ;;; EC-Eval input: ;;; EC-Eval value: ok ;;; \
       EC-Eval input: ;;; EC-Eval value: 12 ;;; EC-Eval input:"
    ]
    (Sec_5_47.ex_5_47 ())
;;

(* 5.50: the compiled metacircular answers the tick session and 120,
   and the three interpretation levels measure 292 machine steps for
   the plain compiled factorial and 52482 for the compiled
   metacircular. *)
let ex_5_50 () =
  match Sec_5_50.ex_5_50 () with
  | Ok lines ->
    strings
      "5.50 head"
      [ "compiled metacircular session: ;;; EC-Eval value: (tick tick tick) ;;; EC-Eval \
         input: ;;; EC-Eval value: 120 ;;; EC-Eval input:"
      ; "level 0 (compiled factorial), steps = 292"
      ; "level 1 (interpreted factorial), monitored pushes = 0"
      ]
      [ List.nth lines 0; List.nth lines 1; List.nth lines 2 ];
    has_prefix
      "5.50 level 2"
      3
      "level 2 (compiled metacircular), machine steps = 52482"
      lines;
    the_string
      "5.50 price"
      "interpretation price: level 2 over level 0 = 180 machine steps"
      (List.nth lines 4)
  | Error e -> Alcotest.fail (Eval.error_to_string e)
;;

(* 5.51: the C translation of the evaluator builds with the system
   compiler and answers the book's factorial session. *)
let ex_5_51 () = strings_outcome "5.51" [ "ok\n120\n" ] (Sec_5_51.ex_5_51 ())

(* 5.52: the compiler's C backend compiles the adapted metacircular
   into a C interpreter whose object factorial answers 120. *)
let ex_5_52 () = strings_outcome "5.52" [ "120\n" ] (Sec_5_52.ex_5_52 ())

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
        ; Alcotest.test_case "5.40" `Quick ex_5_40
        ; Alcotest.test_case "5.41" `Quick ex_5_41
        ; Alcotest.test_case "5.44" `Quick ex_5_44
        ; Alcotest.test_case "5.47" `Quick ex_5_47
        ; Alcotest.test_case "5.50" `Quick ex_5_50
        ; Alcotest.test_case "5.51" `Quick ex_5_51
        ; Alcotest.test_case "5.52" `Quick ex_5_52
        ] )
    ]
;;
