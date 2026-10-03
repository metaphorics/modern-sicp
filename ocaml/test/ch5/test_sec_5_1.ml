(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the section 5.1 machines and the reference
   solutions' public contracts. Every exercise's demonstration is pinned
   to the exact observable outcomes the solutions produce; the
   simulator's own contract -- assembly-time checks, stack discipline,
   operation failures -- is pinned too, so a solution that leans on a
   broken clause cannot pass. *)

module M = Sicp_ch5.Sec_5_1
module Eval_error = Sicp_common.Eval_error
module Solutions = Sicp_ch5_solutions.Sec_5_1
module Sec_5_2 = Sicp_ch5_solutions.Sec_5_2
module Sec_5_3 = Sicp_ch5_solutions.Sec_5_3
module Sec_5_4 = Sicp_ch5_solutions.Sec_5_4
module Sec_5_5 = Sicp_ch5_solutions.Sec_5_5
module Sec_5_6 = Sicp_ch5_solutions.Sec_5_6

let ( let* ) = Result.bind

let show = function
  | Ok v -> M.value_to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string
let the_int = Alcotest.check Alcotest.int

let answers name expected computed =
  match computed with
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

let outcome_message = function
  | Ok _ -> "ran"
  | Error e -> Eval_error.to_string e
;;

let gcd_controller =
  M.
    [ Label "test-b"
    ; Test ("=", [ Reg "b"; Const (Int 0) ])
    ; Branch "gcd-done"
    ; Assign_op ("t", "rem", [ Reg "a"; Reg "b" ])
    ; Assign ("a", Reg "b")
    ; Assign ("b", Reg "t")
    ; Goto "test-b"
    ; Label "gcd-done"
    ]
;;

let run_gcd a b =
  M.run
    ~registers:[ "a"; "b"; "t" ]
    ~operations:M.arith_operations
    ~inputs:[ "a", M.Int a; "b", M.Int b ]
    ~controller:gcd_controller
    "a"
;;

let run_fib controller n =
  M.run
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:M.arith_operations
    ~inputs:[ "n", M.Int n ]
    ~controller
    "val"
;;

let started ~registers controller =
  let* m = M.make_machine ~registers ~operations:M.arith_operations ~controller in
  M.start m
;;

(* The section's machines run, and every failure mode is typed and
   named. *)
let simulator_gcd () =
  strings
    "the gcd machine answers euclid"
    [ "4"; "12"; "1"; "0" ]
    (List.map show [ run_gcd 12 8; run_gcd 48 36; run_gcd 17 5; run_gcd 0 0 ])
;;

let simulator_failures () =
  let check name expected computed =
    the_string name expected (outcome_message computed)
  in
  check
    "an unknown branch label fails at assembly"
    "unknown label nowhere"
    (started ~registers:[ "a" ] M.[ Branch "nowhere" ]);
  check
    "an unknown operation fails at assembly"
    "unknown operation foo"
    (started ~registers:[ "a" ] M.[ Assign_op ("a", "foo", []) ]);
  check
    "an undeclared register fails at assembly"
    "unknown register x"
    (started ~registers:[ "a" ] M.[ Assign ("x", Const (Int 1)) ]);
  check
    "a value operation used as a test fails at assembly"
    "bad instruction: operation + is not a test operation"
    (started ~registers:[ "a" ] M.[ Test ("+", [ Reg "a"; Reg "a" ]) ]);
  check
    "a duplicate label fails the assembly"
    "bad instruction: label a is defined twice"
    (M.assemble M.[ Label "a"; Assign ("x", Const (Int 1)); Label "a" ]);
  check
    "a branch without a test fails"
    "branch without test"
    (started ~registers:[] M.[ Branch "done"; Label "done" ]);
  check
    "a restore on an empty stack fails"
    "bad instruction: restore a from an empty stack"
    (started ~registers:[ "a" ] M.[ Restore "a" ]);
  check
    "a goto through a non-label register fails"
    "bad instruction: goto through continue, which holds 5"
    (started
       ~registers:[ "continue" ]
       M.[ Assign ("continue", Const (Int 5)); Goto_reg "continue" ]);
  check
    "a division by zero is the operation's typed failure"
    "division by zero"
    (started
       ~registers:[ "a" ]
       M.[ Assign_op ("a", "rem", [ Const (Int 1); Const (Int 0) ]) ])
;;

(* Every exercise's demonstration, pinned to its exact observable
   outcomes. *)
let ex_5_01_factorial_machine () =
  answers
    "the iterative factorial machine of the design"
    [ "1"; "1"; "120"; "3628800" ]
    (Solutions.ex_5_01 ())
;;

let ex_5_01a_driver_loop () =
  answers
    "two consecutive runs and the dried-up input"
    [ "120"
    ; "720"
    ; "end"
    ; "1"
    ; "3628800"
    ; "end"
    ; "120"
    ; "Error: read: the input is exhausted"
    ]
    (Solutions.ex_5_01a ())
;;

let ex_5_02_assembly () =
  answers
    "the machine language description assembles and runs"
    [ "5 instructions, 2 labels"; "fact-loop=0 fact-done=5"; "120"; "720" ]
    (Sec_5_2.ex_5_02 ())
;;

let ex_5_03_sqrt_stages () =
  answers
    "both sqrt stages print the same answers"
    [ "1.41421568627"; "3.00009155413"; "1.41421568627"; "3.00009155413" ]
    (Sec_5_3.ex_5_03 ())
;;

let ex_5_04_expt_machines () =
  answers
    "the recursive and iterative expt machines agree"
    [ "1024"; "243"; "1024"; "243" ]
    (Sec_5_4.ex_5_04 ())
;;

let fact_trace =
  [ "Save \"continue\" stack=[fact-done]"
  ; "Save \"n\" stack=[3; fact-done]"
  ; "Save \"continue\" stack=[after-fact; 3; fact-done]"
  ; "Save \"n\" stack=[2; after-fact; 3; fact-done]"
  ; "branch taken to base-case"
  ; "return to after-fact"
  ; "Restore \"n\" n=2 stack=[after-fact; 3; fact-done]"
  ; "Restore \"continue\" continue=after-fact stack=[3; fact-done]"
  ; "return to after-fact"
  ; "Restore \"n\" n=3 stack=[fact-done]"
  ; "Restore \"continue\" continue=fact-done stack=[]"
  ; "return to fact-done"
  ; "answer 6"
  ]
;;

let fib_trace =
  [ "Save \"continue\" stack=[fib-done]"
  ; "Save \"n\" stack=[3; fib-done]"
  ; "Save \"continue\" stack=[afterfib-n-1; 3; fib-done]"
  ; "Save \"n\" stack=[2; afterfib-n-1; 3; fib-done]"
  ; "branch taken to immediate-answer"
  ; "return to afterfib-n-1"
  ; "Restore \"n\" n=2 stack=[afterfib-n-1; 3; fib-done]"
  ; "Restore \"continue\" continue=afterfib-n-1 stack=[3; fib-done]"
  ; "Save \"continue\" stack=[afterfib-n-1; 3; fib-done]"
  ; "Save \"val\" stack=[1; afterfib-n-1; 3; fib-done]"
  ; "branch taken to immediate-answer"
  ; "return to afterfib-n-2"
  ; "Restore \"val\" val=1 stack=[afterfib-n-1; 3; fib-done]"
  ; "Restore \"continue\" continue=afterfib-n-1 stack=[3; fib-done]"
  ; "return to afterfib-n-1"
  ; "Restore \"n\" n=3 stack=[fib-done]"
  ; "Restore \"continue\" continue=fib-done stack=[]"
  ; "Save \"continue\" stack=[fib-done]"
  ; "Save \"val\" stack=[1; fib-done]"
  ; "branch taken to immediate-answer"
  ; "return to afterfib-n-2"
  ; "Restore \"val\" val=1 stack=[fib-done]"
  ; "Restore \"continue\" continue=fib-done stack=[]"
  ; "return to fib-done"
  ; "answer 2"
  ]
;;

let ex_5_05_hand_simulation () =
  answers
    "the hand simulations of factorial 3 and fib 3"
    (fact_trace @ fib_trace)
    (Sec_5_5.ex_5_05 ())
;;

let ex_5_06_redundant_pair () =
  answers
    "the removal keeps the answers and sheds the counts"
    [ "55"; "55"; "steps=281 saves=48"; "steps=257 saves=36" ]
    (Sec_5_6.ex_5_06 ())
;;

(* The hand model and the simulator must agree on the machines of
   Figure 5.11 and Figure 5.12: the same answers, and for fib 6 the
   same number of executed instructions and pushes. *)
let hand_agrees_with_simulator () =
  the_string
    "the factorial hand model answers what the machine answers"
    "6"
    (show (run_fib Sec_5_5.factorial_recursive_controller 3));
  the_string
    "the fib hand model answers what the machine answers"
    "2"
    (show (run_fib Sec_5_5.fib_controller 3));
  strings
    "the modified fib machine answers like the original on ten inputs"
    (List.init 10 (fun n -> show (run_fib Sec_5_5.fib_controller n)))
    (List.init 10 (fun n -> show (run_fib Sec_5_6.fib_modified_controller n)));
  let counts controller =
    let* m =
      M.make_machine
        ~registers:[ "n"; "val"; "continue" ]
        ~operations:M.arith_operations
        ~controller
    in
    let* () = M.set_register m "n" (M.Int 6) in
    let* () = M.start m in
    let pushes, _ = M.stack_statistics m in
    let* program = M.assemble controller in
    let* _, st =
      Sec_5_5.Handsim.run
        program
        (Sec_5_5.Handsim.initial [ "n", M.Int 6; "val", M.Int 0 ])
        []
    in
    Ok ((M.executed m, st.steps), (pushes, st.saves))
  in
  List.iter
    (fun (name, controller) ->
       match counts controller with
       | Error e -> Alcotest.fail (Eval_error.to_string e)
       | Ok ((executed, steps), (pushes, saves)) ->
         the_int (name ^ ": executed instructions") executed steps;
         the_int (name ^ ": pushes") pushes saves)
    [ "fib", Sec_5_5.fib_controller; "modified fib", Sec_5_6.fib_modified_controller ]
;;

let () =
  let open Alcotest in
  run
    "sicp section 5.1"
    [ ( "simulator"
      , [ test_case "gcd machines run" `Quick simulator_gcd
        ; test_case "every failure is typed and named" `Quick simulator_failures
        ] )
    ; ( "exercises"
      , [ test_case "5.1 the designed factorial machine" `Quick ex_5_01_factorial_machine
        ; test_case
            "5.1a the driver loop over repeated inputs"
            `Quick
            ex_5_01a_driver_loop
        ; test_case "5.2 the machine language description" `Quick ex_5_02_assembly
        ; test_case "5.3 the sqrt machine in two stages" `Quick ex_5_03_sqrt_stages
        ; test_case "5.4 the two exponentiation machines" `Quick ex_5_04_expt_machines
        ; test_case "5.5 the hand simulations" `Quick ex_5_05_hand_simulation
        ; test_case "5.6 the redundant save and restore" `Quick ex_5_06_redundant_pair
        ; test_case
            "the hand model agrees with the simulator"
            `Quick
            hand_agrees_with_simulator
        ] )
    ]
;;
