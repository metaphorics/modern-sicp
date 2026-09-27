(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the section 5.2 simulator and the reference
   solutions' public contracts. Every exercise's demonstration is
   pinned to the exact observable outcomes the solutions produce; the
   simulator's own contract -- assembly-time checks, the monitored
   stack, the trace and breakpoint machinery -- is pinned too, so a
   solution that leans on a broken clause cannot pass. *)

module Machine = Sicp_ch5.Sec_5_2
module Solutions = Sicp_ch5_solutions.Sec_5_7
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

let ( >>= ) = Result.bind
let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string
let the_int = Alcotest.check Alcotest.int

let strings_outcome name expected = function
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Machine.error_to_string e)
;;

(* The simulator: the book's GCD session runs, and the assembly-time
   checks are the typed failures the section names. *)
let simulator_gcd () =
  match
    Machine.make_machine
      ~registers:[ "a"; "b"; "t" ]
      ~operations:Machine.arith_operations
      ~controller:
        {|(controller
 test-b
   (test (op =) (reg b) (const 0))
   (branch (label gcd-done))
   (assign t (op rem) (reg a) (reg b))
   (assign a (reg b))
   (assign b (reg t))
   (goto (label test-b))
 gcd-done)|}
    >>= fun m ->
    Machine.set_register m "a" (Machine.Int 206)
    >>= fun () ->
    Machine.set_register m "b" (Machine.Int 40)
    >>= fun () -> Machine.start m >>= fun () -> Machine.get_register m "a"
  with
  | Ok (Int n) -> Alcotest.(check int) "gcd session" 2 n
  | Ok v -> Alcotest.fail (Machine.value_to_string v)
  | Error e -> Alcotest.fail (Machine.error_to_string e)
;;

let simulator_checks () =
  let assemble controller registers =
    Machine.make_machine ~registers ~operations:Machine.arith_operations ~controller
    |> function
    | Ok _ -> "assembled"
    | Error e -> "Error: " ^ Machine.error_to_string e
  in
  strings
    "assembly-time checks"
    [ "Error: bad instruction: the label again is used twice"
    ; "Error: unknown register nope"
    ; "Error: unknown label nowhere"
    ; "Error: unknown operation nosuch"
    ; "Error: bad instruction: the register a is declared twice"
    ]
    [ assemble
        {|(controller (assign a (const 1)) again (goto (label again)) again)|}
        [ "a" ]
    ; assemble {|(controller (assign a (reg nope)))|} [ "a" ]
    ; assemble {|(controller (goto (label nowhere)))|} [ "a" ]
    ; assemble {|(controller (assign a (op nosuch) (const 1)))|} [ "a" ]
    ; assemble {|(controller (assign a (const 1)))|} [ "a"; "a" ]
    ]
;;

(* A branch reached before any test is the typed failure; the stack
   underflow names its register; the monitored stack counts. *)
let branch_without_test () =
  match
    Machine.make_machine
      ~registers:[ "a" ]
      ~operations:[]
      ~controller:{|(controller (branch (label end)) end)|}
    >>= fun m -> Machine.start m
  with
  | Error Branch_without_test -> ()
  | _ -> Alcotest.fail "expected the branch-without-test failure"
;;

let underflow_names_register () =
  match
    Machine.make_machine
      ~registers:[ "x" ]
      ~operations:[]
      ~controller:{|(controller (restore x))|}
    >>= fun m -> Machine.start m
  with
  | Error (Stack_underflow "x") -> ()
  | _ -> Alcotest.fail "expected the typed underflow"
;;

let monitored_stack_counts () =
  match
    Machine.make_machine
      ~registers:[ "x" ]
      ~operations:Machine.arith_operations
      ~controller:
        {|(controller
   (perform (op initialize-stack))
   (save x)
   (save x)
   (restore x)
   (perform (op print-stack-statistics)))|}
    >>= fun m -> Machine.start m >>= fun () -> Ok m
  with
  | Error e -> Alcotest.fail (Machine.error_to_string e)
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

(* The monitoring core: the traced, counted, breakpointed machine of
   the later exercises. *)
let monitored_counts () =
  match
    Sec_5_15.Sim.make
      ~registers:[ "n"; "val"; "continue" ]
      ~operations:Machine.arith_operations
      ~controller:Sec_5_15.fib_controller
    >>= fun m ->
    Sec_5_15.Sim.set_register m "n" (Machine.Int 6)
    >>= fun () ->
    Sec_5_15.Sim.start m
    >>= fun _ -> Sec_5_15.Sim.get_register m "val" >>= fun v -> Ok (m, v)
  with
  | Error e -> Alcotest.fail (Machine.error_to_string e)
  | Ok (m, v) ->
    the_string "fib 6" "8" (Machine.value_to_string v);
    the_int "instruction count" 281 (Sec_5_15.Sim.take_instruction_count m);
    the_int "count after reset" 0 (Sec_5_15.Sim.take_instruction_count m);
    (match
       Sec_5_15.Sim.set_register m "n" (Machine.Int 3) >>= fun () -> Sec_5_15.Sim.start m
     with
     | Error e -> Alcotest.fail (Machine.error_to_string e)
     | Ok _ -> the_int "fib 3 count" 51 (Sec_5_15.Sim.instruction_count m))
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
    [ "Error: bad instruction: the label again is used twice" ]
    (Sec_5_8.ex_5_08 ())
;;

let ex_5_09_label_operands () =
  strings_outcome
    "ex 5.9"
    [ "5"; "Error: bad instruction: an operation input is written (reg r) or (const c)" ]
    (Sec_5_9.ex_5_09 ())
;;

let ex_5_10_new_syntax () =
  strings_outcome
    "ex 5.10"
    [ "4"
    ; "4"
    ; "(assign t (op rem) (reg a) (reg b))"
    ; "(assign t (op rem) (reg a) (reg b))"
    ]
    (Sec_5_10.ex_5_10 ())
;;

let ex_5_11_disciplines () =
  strings_outcome
    "ex 5.11"
    [ "(a) untagged: (save y) (save x) (restore y) leaves y = 8"
    ; "(a) fib with afterfib-n-2's exchange replaced by one (restore n): answers n=0..9 \
       match the original"
    ; "(b) tagged: (save y) (save x) (restore y) reports -- Error: bad instruction: \
       restore y but the stack holds x"
    ; "(b) tagged fib 6 = 8"
    ; "(c) per-register: (save y) (save x) (restore y) leaves y = 7"
    ; "(c) restore from an empty per-register stack reports -- Error: stack underflow \
       restoring x"
    ]
    (Sec_5_11.ex_5_11 ())
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
    ; "sources of continue: (label fib-done), (label afterfib-n-1), (label afterfib-n-2)"
    ; "sources of n: (reg n), (const 1), (const 2), (reg val)"
    ; "sources of val: (reg val), (reg n)"
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
    ; "sources of continue: (label fact-done), (label after-fact)"
    ; "sources of n: (reg n), (const 1)"
    ; "sources of val: (reg n), (reg val), (const 1)"
    ]
    (Sec_5_12.ex_5_12 ())
;;

let ex_5_13_derived_registers () =
  strings_outcome "ex 5.13" [ "gcd 206 40 = 2"; "fib 6 = 8" ] (Sec_5_13.ex_5_13 ())
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

let ex_5_16_tracing () =
  strings_outcome
    "ex 5.16"
    [ "(test (op =) (reg b) (const 0))"
    ; "(branch (label gcd-done))"
    ; "(assign t (op rem) (reg a) (reg b))"
    ; "(assign a (reg b))"
    ; "(assign b (reg t))"
    ; "(goto (label test-b))"
    ; "(test (op =) (reg b) (const 0))"
    ; "(branch (label gcd-done))"
    ; "(assign t (op rem) (reg a) (reg b))"
    ; "(assign a (reg b))"
    ; "(assign b (reg t))"
    ; "(goto (label test-b))"
    ; "(test (op =) (reg b) (const 0))"
    ; "(branch (label gcd-done))"
    ; "gcd 12 8 = 4"
    ; "with tracing off the run on (20, 14) printed 0 trace lines and answered 2"
    ]
    (Sec_5_16.ex_5_16 ())
;;

let ex_5_17_labels_in_trace () =
  strings_outcome
    "ex 5.17"
    [ "(assign continue (label fib-done))"
    ; "fib-loop: (test (op <) (reg n) (const 2))"
    ; "(branch (label immediate-answer))"
    ; "(save continue)"
    ; "(assign continue (label afterfib-n-1))"
    ; "(save n)"
    ; "(assign n (op -) (reg n) (const 1))"
    ; "(goto (label fib-loop))"
    ; "fib-loop: (test (op <) (reg n) (const 2))"
    ; "(branch (label immediate-answer))"
    ; "(save continue)"
    ; "(assign continue (label afterfib-n-1))"
    ; "(save n)"
    ; "(assign n (op -) (reg n) (const 1))"
    ; "(goto (label fib-loop))"
    ; "fib-loop: (test (op <) (reg n) (const 2))"
    ; "(branch (label immediate-answer))"
    ; "immediate-answer: (assign val (reg n))"
    ; "(goto (reg continue))"
    ; "afterfib-n-1: (restore n)"
    ; "(restore continue)"
    ; "(assign n (op -) (reg n) (const 2))"
    ; "(save continue)"
    ; "(assign continue (label afterfib-n-2))"
    ; "(save val)"
    ; "(goto (label fib-loop))"
    ; "fib-loop: (test (op <) (reg n) (const 2))"
    ; "(branch (label immediate-answer))"
    ; "immediate-answer: (assign val (reg n))"
    ; "(goto (reg continue))"
    ; "afterfib-n-2: (assign n (reg val))"
    ; "(restore val)"
    ; "(restore continue)"
    ; "(assign val (op +) (reg val) (reg n))"
    ; "(goto (reg continue))"
    ; "afterfib-n-1: (restore n)"
    ; "(restore continue)"
    ; "(assign n (op -) (reg n) (const 2))"
    ; "(save continue)"
    ; "(assign continue (label afterfib-n-2))"
    ; "(save val)"
    ; "(goto (label fib-loop))"
    ; "fib-loop: (test (op <) (reg n) (const 2))"
    ; "(branch (label immediate-answer))"
    ; "immediate-answer: (assign val (reg n))"
    ; "(goto (reg continue))"
    ; "afterfib-n-2: (assign n (reg val))"
    ; "(restore val)"
    ; "(restore continue)"
    ; "(assign val (op +) (reg val) (reg n))"
    ; "(goto (reg continue))"
    ; "fib 3 = 2 in 51 instructions"
    ]
    (Sec_5_17.ex_5_17 ())
;;

let ex_5_18_register_trace () =
  strings_outcome
    "ex 5.18"
    [ "n: *unassigned* -> 3"
    ; "n: 3 -> 2"
    ; "n: 2 -> 1"
    ; "val: *unassigned* -> 1"
    ; "n: 1 -> 2"
    ; "val: 1 -> 2"
    ; "n: 2 -> 3"
    ; "val: 2 -> 6"
    ; "fact 3 = 6"
    ]
    (Sec_5_18.ex_5_18 ())
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
        ; Alcotest.test_case "typed underflow" `Quick underflow_names_register
        ; Alcotest.test_case "monitored stack" `Quick monitored_stack_counts
        ; Alcotest.test_case "monitored counts" `Quick monitored_counts
        ] )
    ; ( "exercises"
      , [ Alcotest.test_case "5.7" `Quick ex_5_07_expt_machines
        ; Alcotest.test_case "5.7a" `Quick ex_5_07a_oracle
        ; Alcotest.test_case "5.8" `Quick ex_5_08_duplicate_label
        ; Alcotest.test_case "5.9" `Quick ex_5_09_label_operands
        ; Alcotest.test_case "5.10" `Quick ex_5_10_new_syntax
        ; Alcotest.test_case "5.11" `Quick ex_5_11_disciplines
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
