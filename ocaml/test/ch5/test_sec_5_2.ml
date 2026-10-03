(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the section 5.2 machines and the reference
   solutions' public contracts. Every exercise's demonstration is
   pinned to the exact observable outcomes the solutions produce; the
   simulator's own contract -- assembly-time checks, the monitored
   stack -- and the solutions' monitor, trace, and breakpoint
   machinery are pinned too, so a solution that leans on a broken
   clause cannot pass. *)

module M = Sicp_ch5.Sec_5_1
module Machine = Sicp_ch5.Sec_5_2
module Eval_error = Sicp_common.Eval_error
module Solutions = Sicp_ch5_solutions.Sec_5_7
module Sec_5_5 = Sicp_ch5_solutions.Sec_5_5
module Sec_5_8 = Sicp_ch5_solutions.Sec_5_8
module Sec_5_9 = Sicp_ch5_solutions.Sec_5_9
module Sec_5_10 = Sicp_ch5_solutions.Sec_5_10
module Sec_5_11 = Sicp_ch5_solutions.Sec_5_11
module Sec_5_12 = Sicp_ch5_solutions.Sec_5_12
module Sec_5_13 = Sicp_ch5_solutions.Sec_5_13
module Sec_5_14 = Sicp_ch5_solutions.Sec_5_14
module Sec_5_15 = Sicp_ch5_solutions.Sec_5_15
module Sec_5_16 = Sicp_ch5_solutions.Sec_5_16
module Sec_5_17 = Sicp_ch5_solutions.Sec_5_17
module Sec_5_18 = Sicp_ch5_solutions.Sec_5_18
module Sec_5_19 = Sicp_ch5_solutions.Sec_5_19
module Monitor = Sec_5_15.Monitor

let ( let* ) = Result.bind
let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string
let the_int = Alcotest.check Alcotest.int

let strings_outcome name expected = function
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

let outcome = function
  | Ok _ -> "ok"
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(* The simulator: the book's GCD session runs, and the assembly-time
   checks are the typed failures the section names. *)
let simulator_gcd () =
  match
    let* m =
      Machine.make_machine
        ~registers:[ "a"; "b"; "t" ]
        ~operations:M.arith_operations
        ~controller:Sec_5_10.gcd_controller
    in
    let* () = Machine.set_register m "a" (M.Int 206) in
    let* () = Machine.set_register m "b" (M.Int 40) in
    let* () = Machine.start m in
    Machine.get_register m "a"
  with
  | Ok (M.Int n) -> the_int "gcd session" 2 n
  | Ok v -> Alcotest.fail (M.value_to_string v)
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

let simulator_checks () =
  let assemble controller registers =
    outcome (Machine.make_machine ~registers ~operations:M.arith_operations ~controller)
  in
  strings
    "assembly-time checks"
    [ "Error: bad instruction: label again is defined twice"
    ; "Error: unknown register nope"
    ; "Error: unknown label nowhere"
    ; "Error: unknown operation nosuch"
    ; "Error: bad instruction: register a is declared twice"
    ]
    [ assemble
        M.[ Assign ("a", Const (Int 1)); Label "again"; Goto "again"; Label "again" ]
        [ "a" ]
    ; assemble M.[ Assign ("a", Reg "nope") ] [ "a" ]
    ; assemble M.[ Goto "nowhere" ] [ "a" ]
    ; assemble M.[ Assign_op ("a", "nosuch", [ Const (Int 1) ]) ] [ "a" ]
    ; assemble M.[ Assign ("a", Const (Int 1)) ] [ "a"; "a" ]
    ]
;;

(* A branch reached before any test is the typed failure; the monitored
   stack counts pushes and depth since the last initialization. *)
let branch_without_test () =
  match
    let* m =
      Machine.make_machine
        ~registers:[ "a" ]
        ~operations:[]
        ~controller:M.[ Branch "end"; Label "end" ]
    in
    Machine.start m
  with
  | Error Eval_error.Branch_without_test -> ()
  | _ -> Alcotest.fail "expected the branch-without-test failure"
;;

let monitored_stack_counts () =
  match
    let* m =
      Machine.make_machine
        ~registers:[ "x" ]
        ~operations:M.arith_operations
        ~controller:
          M.
            [ Save "x"
            ; Perform ("initialize-stack", [])
            ; Save "x"
            ; Save "x"
            ; Restore "x"
            ; Perform ("print-stack-statistics", [])
            ]
    in
    let* () = Machine.set_register m "x" (M.Int 1) in
    let* () = Machine.start m in
    Ok m
  with
  | Error e -> Alcotest.fail (Eval_error.to_string e)
  | Ok m ->
    the_string
      "stack statistics"
      "total-pushes = 2 maximum-depth = 2"
      (Machine.print_stack_statistics m);
    strings
      "stack transcript"
      [ "total-pushes = 2 maximum-depth = 2" ]
      (Machine.transcript m)
;;

(* The monitor: counted, reset, and stopped by breakpoints. *)
let fib_monitor () =
  Monitor.make
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:M.arith_operations
    ~controller:Sec_5_5.fib_controller
;;

let monitored_counts () =
  match
    let* m = fib_monitor () in
    let* () = Monitor.set_register m "n" (M.Int 6) in
    let* _ = Monitor.start m in
    let* v = Monitor.get_register m "val" in
    Ok (m, v)
  with
  | Error e -> Alcotest.fail (Eval_error.to_string e)
  | Ok (m, v) ->
    the_string "fib 6" "8" (M.value_to_string v);
    the_int "instruction count" 281 (Monitor.take_instruction_count m);
    the_int "count after reset" 0 (Monitor.take_instruction_count m);
    (match
       let* () = Monitor.set_register m "n" (M.Int 3) in
       Monitor.start m
     with
     | Error e -> Alcotest.fail (Eval_error.to_string e)
     | Ok _ -> the_int "fib 3 count" 51 (Monitor.instruction_count m))
;;

let breakpoint_boundaries () =
  match fib_monitor () with
  | Error e -> Alcotest.fail (Eval_error.to_string e)
  | Ok m ->
    strings
      "breakpoint placement"
      [ "ok"
      ; "Error: bad instruction: the breakpoint at fib-loop 0 is past the code"
      ; "Error: bad instruction: the breakpoint at immediate-answer 3 is past the code"
      ; "Error: unknown label nowhere"
      ]
      [ outcome (Monitor.set_breakpoint m "immediate-answer" 2)
      ; outcome (Monitor.set_breakpoint m "fib-loop" 0)
      ; outcome (Monitor.set_breakpoint m "immediate-answer" 3)
      ; outcome (Monitor.set_breakpoint m "nowhere" 1)
      ];
    let stops =
      let* () = Monitor.set_register m "n" (M.Int 2) in
      let rec run acc stop =
        match stop with
        | Sec_5_15.Completed -> Ok (List.rev acc)
        | Sec_5_15.Breakpoint _ ->
          let* next = Monitor.proceed m in
          run (Sec_5_19.show_stop stop :: acc) next
      in
      let* first = Monitor.start m in
      run [] first
    in
    strings_outcome
      "fib 2 returns through immediate-answer twice"
      [ "stop at immediate-answer 2"; "stop at immediate-answer 2" ]
      stops
;;

(* Every exercise's demonstration, pinned to its exact observable
   outcomes. *)
let ex_5_07_expt_machines () =
  strings_outcome "ex 5.7" [ "1024"; "243"; "1024"; "243" ] (Solutions.ex_5_07 ())
;;

let ex_5_07a_oracle () =
  strings_outcome
    "ex 5.7a"
    [ "n=0: fib 0 (direct 0) ok; factorial 1 (direct 1) ok"
    ; "n=1: fib 1 (direct 1) ok; factorial 1 (direct 1) ok"
    ; "n=2: fib 1 (direct 1) ok; factorial 2 (direct 2) ok"
    ; "n=3: fib 2 (direct 2) ok; factorial 6 (direct 6) ok"
    ; "n=4: fib 3 (direct 3) ok; factorial 24 (direct 24) ok"
    ; "n=5: fib 5 (direct 5) ok; factorial 120 (direct 120) ok"
    ; "n=6: fib 8 (direct 8) ok; factorial 720 (direct 720) ok"
    ]
    (Solutions.ex_5_07a ())
;;

let ex_5_08_duplicate_label () =
  strings_outcome
    "ex 5.8"
    [ "Error: bad instruction: label again is defined twice" ]
    (Sec_5_8.ex_5_08 ())
;;

let ex_5_09_label_operands () =
  strings_outcome
    "ex 5.9"
    [ "5"
    ; "Error: bad instruction: an operation input is a register or a constant, not the \
       label there"
    ]
    (Sec_5_9.ex_5_09 ());
  strings
    "every operation position is checked"
    [ "ok"
    ; "Error: bad instruction: an operation input is a register or a constant, not the \
       label x"
    ; "Error: bad instruction: an operation input is a register or a constant, not the \
       label y"
    ]
    [ outcome
        (Sec_5_9.check_operands
           M.[ Assign ("a", Label_ref "x"); Test ("=", [ Reg "a"; Const (Int 0) ]) ])
    ; outcome (Sec_5_9.check_operands M.[ Test ("=", [ Label_ref "x"; Reg "a" ]) ])
    ; outcome (Sec_5_9.check_operands M.[ Perform ("print", [ Label_ref "y" ]) ])
    ]
;;

let ex_5_10_new_syntax () =
  strings_outcome
    "ex 5.10"
    [ "4"
    ; "4"
    ; "Assign_op (\"t\", \"rem\", [Reg \"a\"; Reg \"b\"])"
    ; "Assign_op (\"t\", \"rem\", [Reg \"a\"; Reg \"b\"])"
    ]
    (Sec_5_10.ex_5_10 ());
  strings
    "the syntax procedures refuse what the machine cannot run"
    [ "Error: bad instruction: the call + cannot be an operand"
    ; "Error: bad instruction: a jump goes to a label or through a register"
    ]
    [ outcome
        (Sec_5_10.syntax
           Sec_5_10.[ Set ("a", Call ("*", [ R "b"; Call ("+", [ R "b"; N 1 ]) ])) ])
    ; outcome (Sec_5_10.syntax Sec_5_10.[ Jump (N 3) ])
    ]
;;

let ex_5_11_disciplines () =
  strings_outcome
    "ex 5.11"
    [ "(a) untagged: Save \"y\"; Save \"x\"; Restore \"y\" leaves y = 8"
    ; "(a) fib with afterfib-n-2's exchange replaced by one Restore \"n\": answers \
       n=0..9 match the original"
    ; "(b) tagged: Save \"y\"; Save \"x\"; Restore \"y\" reports -- Error: bad \
       instruction: restore y but the stack holds x"
    ; "(b) tagged fib 6 = 8"
    ; "(c) per-register: Save \"y\"; Save \"x\"; Restore \"y\" leaves y = 7"
    ; "(c) restore from an empty per-register stack reports -- Error: bad instruction: \
       restore x from an empty stack"
    ]
    (Sec_5_11.ex_5_11 ())
;;

(* The disciplined machines replace one instruction by one, so they
   execute exactly as many instructions as the untagged machine. *)
let ex_5_11_counts_unchanged () =
  let executed discipline =
    let* m =
      Sec_5_11.make
        ~discipline
        ~registers:[ "n"; "val"; "continue" ]
        ~operations:M.arith_operations
        ~controller:Sec_5_5.fib_controller
    in
    let* () = M.set_register m "n" (M.Int 6) in
    let* () = M.start m in
    Ok (M.executed m)
  in
  List.iter
    (fun (name, discipline) ->
       match executed discipline with
       | Ok n -> the_int (name ^ " fib 6 instructions") 281 n
       | Error e -> Alcotest.fail (Eval_error.to_string e))
    [ "untagged", Sec_5_11.Untagged
    ; "tagged", Sec_5_11.Tagged
    ; "per-register", Sec_5_11.Per_register
    ]
;;

let ex_5_11_one_fewer () =
  the_int
    "the one-fewer machine has one instruction fewer"
    (List.length Sec_5_11.fib_one_fewer_controller)
    (List.length Sec_5_5.fib_controller - 1)
;;

let syntax_covers_whole_program () =
  match Sec_5_10.syntax Sec_5_10.gcd_new_syntax with
  | Error e -> Alcotest.fail (Eval_error.to_string e)
  | Ok translated ->
    let assembled controller =
      match M.assemble controller with
      | Ok program -> program
      | Error e -> Alcotest.fail (Eval_error.to_string e)
    in
    let old_program = assembled Sec_5_10.gcd_controller in
    let new_program = assembled translated in
    strings
      "the translation covers the whole controller"
      [ string_of_bool (old_program.code = new_program.code)
      ; string_of_bool (old_program.labels = new_program.labels)
      ]
      [ "true"; "true" ]
;;

let ex_5_12_analysis () =
  strings_outcome
    "ex 5.12"
    [ "fib:"
    ; "unique instructions, sorted by type: 18"
    ; "assign: 8"
    ; "test: 1"
    ; "branch: 1"
    ; "goto: 2"
    ; "save: 3"
    ; "restore: 3"
    ; "perform: 0"
    ; "entry-point registers: continue"
    ; "saved/restored registers: continue, n, val"
    ; "sources of continue: Label_ref \"fib-done\", Label_ref \"afterfib-n-1\", \
       Label_ref \"afterfib-n-2\""
    ; "sources of n: Reg \"n\", Const (1), Const (2), Reg \"val\""
    ; "sources of val: Reg \"val\", Reg \"n\""
    ; "factorial:"
    ; "unique instructions, sorted by type: 13"
    ; "assign: 5"
    ; "test: 1"
    ; "branch: 1"
    ; "goto: 2"
    ; "save: 2"
    ; "restore: 2"
    ; "perform: 0"
    ; "entry-point registers: continue"
    ; "saved/restored registers: continue, n"
    ; "sources of continue: Label_ref \"fact-done\", Label_ref \"after-fact\""
    ; "sources of n: Reg \"n\", Const (1)"
    ; "sources of val: Reg \"n\", Reg \"val\", Const (1)"
    ]
    (Sec_5_12.ex_5_12 ())
;;

let ex_5_13_derived_registers () =
  strings_outcome "ex 5.13" [ "gcd 206 40 = 2"; "fib 6 = 8" ] (Sec_5_13.ex_5_13 ());
  strings
    "first-use order, no duplicates"
    [ "b"; "t"; "a" ]
    (Sec_5_13.derive_registers Sec_5_10.gcd_controller)
;;

let ex_5_14_stack_formulas () =
  strings_outcome
    "ex 5.14"
    [ "n = 1: total-pushes = 0 maximum-depth = 0"
    ; "n = 2: total-pushes = 2 maximum-depth = 2"
    ; "n = 3: total-pushes = 4 maximum-depth = 4"
    ; "n = 4: total-pushes = 6 maximum-depth = 6"
    ; "n = 5: total-pushes = 8 maximum-depth = 8"
    ; "n = 6: total-pushes = 10 maximum-depth = 10"
    ; "n = 7: total-pushes = 12 maximum-depth = 12"
    ; "total-pushes(n) = 2n - 2 and maximum-depth(n) = 2n - 2 for n > 1:"
      ^ " two pushes per recursive level, n - 1 levels, no pops before the base case"
    ]
    (Sec_5_14.ex_5_14 ())
;;

let ex_5_15_instruction_count () =
  strings_outcome
    "ex 5.15"
    [ "fib 3 = 2, instructions = 51"
    ; "fib 6 = 8, instructions = 281"
    ; "after the reset the count is 0"
    ]
    (Sec_5_15.ex_5_15 ())
;;

let gcd_pass =
  [ "Test (\"=\", [Reg \"b\"; Const (0)])"
  ; "Branch \"gcd-done\""
  ; "Assign_op (\"t\", \"rem\", [Reg \"a\"; Reg \"b\"])"
  ; "Assign (\"a\", Reg \"b\")"
  ; "Assign (\"b\", Reg \"t\")"
  ; "Goto \"test-b\""
  ]
;;

let ex_5_16_tracing () =
  strings_outcome
    "ex 5.16"
    (gcd_pass
     @ gcd_pass
     @ [ "Test (\"=\", [Reg \"b\"; Const (0)])"
       ; "Branch \"gcd-done\""
       ; "gcd 12 8 = 4"
       ; "with tracing off the run on (20, 14) printed 0 trace lines and answered 2"
       ])
    (Sec_5_16.ex_5_16 ())
;;

let fib_call =
  [ "fib-loop: Test (\"<\", [Reg \"n\"; Const (2)])"; "Branch \"immediate-answer\"" ]
;;

let fib_descend =
  fib_call
  @ [ "Save \"continue\""
    ; "Assign (\"continue\", Label_ref \"afterfib-n-1\")"
    ; "Save \"n\""
    ; "Assign_op (\"n\", \"-\", [Reg \"n\"; Const (1)])"
    ; "Goto \"fib-loop\""
    ]
;;

let fib_leaf =
  fib_call @ [ "immediate-answer: Assign (\"val\", Reg \"n\")"; "Goto_reg \"continue\"" ]
;;

let fib_second_call =
  [ "afterfib-n-1: Restore \"n\""
  ; "Restore \"continue\""
  ; "Assign_op (\"n\", \"-\", [Reg \"n\"; Const (2)])"
  ; "Save \"continue\""
  ; "Assign (\"continue\", Label_ref \"afterfib-n-2\")"
  ; "Save \"val\""
  ; "Goto \"fib-loop\""
  ]
;;

let fib_sum =
  [ "afterfib-n-2: Assign (\"n\", Reg \"val\")"
  ; "Restore \"val\""
  ; "Restore \"continue\""
  ; "Assign_op (\"val\", \"+\", [Reg \"val\"; Reg \"n\"])"
  ; "Goto_reg \"continue\""
  ]
;;

let ex_5_17_labels_in_trace () =
  strings_outcome
    "ex 5.17"
    ([ "Assign (\"continue\", Label_ref \"fib-done\")" ]
     @ fib_descend
     @ fib_descend
     @ fib_leaf
     @ fib_second_call
     @ fib_leaf
     @ fib_sum
     @ fib_second_call
     @ fib_leaf
     @ fib_sum
     @ [ "fib 3 = 2 in 51 instructions" ])
    (Sec_5_17.ex_5_17 ())
;;

let ex_5_18_register_trace () =
  strings_outcome
    "ex 5.18"
    [ "n: unassigned -> 3"
    ; "n: 3 -> 2"
    ; "n: 2 -> 1"
    ; "val: unassigned -> 1"
    ; "n: 1 -> 2"
    ; "val: 1 -> 2"
    ; "n: 2 -> 3"
    ; "val: 2 -> 6"
    ; "fact 3 = 6"
    ]
    (Sec_5_18.ex_5_18 ())
;;

(* A traced register reports every write, including one that stores the
   word it already holds; turning tracing off silences it. *)
let register_trace_transitions () =
  match
    let* m =
      Monitor.make
        ~registers:[ "a" ]
        ~operations:M.arith_operations
        ~controller:M.[ Assign ("a", Const (Int 1)); Assign ("a", Const (Int 1)) ]
    in
    let* () = Monitor.trace_register m "a" true in
    let* _ = Monitor.start m in
    let* () = Monitor.trace_register m "a" false in
    let* _ = Monitor.start m in
    Ok (Monitor.traced_assignments m)
  with
  | Ok lines ->
    strings "same-word writes report" [ "a: unassigned -> 1"; "a: 1 -> 1" ] lines
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

let ex_5_19_breakpoints () =
  strings_outcome
    "ex 5.19"
    [ "breakpoint set at test-b 4"
    ; "start: stop at test-b 4 (a = 12, b = 8)"
    ; "proceed: stop at test-b 4 (a = 8, b = 4)"
    ; "proceed: completed, a = 4"
    ; "cancel test-b 4, rerun on (20, 14): completed, a = 2"
    ; "set test-b 4 and test-b 2, cancel all, run on (25, 15): completed"
    ]
    (Sec_5_19.ex_5_19 ())
;;

let () =
  Alcotest.run
    "section 5.2"
    [ ( "simulator"
      , [ Alcotest.test_case "gcd session" `Quick simulator_gcd
        ; Alcotest.test_case "assembly-time checks" `Quick simulator_checks
        ; Alcotest.test_case "branch without test" `Quick branch_without_test
        ; Alcotest.test_case "monitored stack" `Quick monitored_stack_counts
        ; Alcotest.test_case "monitored counts" `Quick monitored_counts
        ; Alcotest.test_case "breakpoint boundaries" `Quick breakpoint_boundaries
        ; Alcotest.test_case
            "register trace transitions"
            `Quick
            register_trace_transitions
        ] )
    ; ( "exercises"
      , [ Alcotest.test_case "5.7" `Quick ex_5_07_expt_machines
        ; Alcotest.test_case "5.7a" `Quick ex_5_07a_oracle
        ; Alcotest.test_case "5.8" `Quick ex_5_08_duplicate_label
        ; Alcotest.test_case "5.9" `Quick ex_5_09_label_operands
        ; Alcotest.test_case "5.10" `Quick ex_5_10_new_syntax
        ; Alcotest.test_case "5.10 whole program" `Quick syntax_covers_whole_program
        ; Alcotest.test_case "5.11" `Quick ex_5_11_disciplines
        ; Alcotest.test_case "5.11 counts" `Quick ex_5_11_counts_unchanged
        ; Alcotest.test_case "5.11 one fewer" `Quick ex_5_11_one_fewer
        ; Alcotest.test_case "5.12" `Quick ex_5_12_analysis
        ; Alcotest.test_case "5.13" `Quick ex_5_13_derived_registers
        ; Alcotest.test_case "5.14" `Quick ex_5_14_stack_formulas
        ; Alcotest.test_case "5.15" `Quick ex_5_15_instruction_count
        ; Alcotest.test_case "5.16" `Quick ex_5_16_tracing
        ; Alcotest.test_case "5.17" `Quick ex_5_17_labels_in_trace
        ; Alcotest.test_case "5.18" `Quick ex_5_18_register_trace
        ; Alcotest.test_case "5.19" `Quick ex_5_19_breakpoints
        ] )
    ]
;;
