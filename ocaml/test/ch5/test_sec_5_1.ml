(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the section 5.1 substrate and the reference
   solutions' public contracts. Every exercise's demonstration is pinned
   to the exact observable outcomes the solutions produce; the
   substrate's own contract -- parsing, assembly-time checks, stack
   discipline, operation failures -- is pinned too, so a solution that
   leans on a broken clause cannot pass. *)

module Machine = Sicp_ch5.Sec_5_1
module Solutions = Sicp_ch5_solutions.Sec_5_1
module Sec_5_2 = Sicp_ch5_solutions.Sec_5_2
module Sec_5_3 = Sicp_ch5_solutions.Sec_5_3
module Sec_5_4 = Sicp_ch5_solutions.Sec_5_4
module Sec_5_5 = Sicp_ch5_solutions.Sec_5_5
module Sec_5_6 = Sicp_ch5_solutions.Sec_5_6

let ( >>= ) = Result.bind

let show = function
  | Ok v -> Machine.value_to_string v
  | Error e -> "Error: " ^ Machine.error_to_string e
;;

let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string

let answers name expected computed =
  match computed with
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Machine.error_to_string e)
;;

let parse_message text =
  match Machine.parse_program text with
  | Ok _ -> "parsed"
  | Error e -> Machine.error_to_string e
;;

let outcome_message computed =
  match computed with
  | Ok _ -> "ran"
  | Error e -> Machine.error_to_string e
;;

let run_controller controller registers inputs result =
  Machine.make_machine ~registers ~operations:Machine.arith_operations ~controller
  >>= fun m ->
  List.fold_left
    (fun acc (name, v) ->
       match acc with
       | Error _ -> acc
       | Ok () -> Machine.set_register m name v)
    (Ok ())
    inputs
  >>= fun () -> Machine.start m >>= fun () -> Machine.get_register m result
;;

let gcd_controller =
  {|(controller
 test-b
   (test (op =) (reg b) (const 0))
   (branch (label gcd-done))
   (assign t (op rem) (reg a) (reg b))
   (assign a (reg b))
   (assign b (reg t))
   (goto (label test-b))
 gcd-done)|}
;;

let run_gcd a b =
  run_controller
    gcd_controller
    [ "a"; "b"; "t" ]
    [ "a", Machine.Int a; "b", Machine.Int b ]
    "a"
;;

(* The substrate: the section's machines run, and every failure mode is
   typed and named. *)
let substrate_gcd () =
  strings
    "the gcd machine answers euclid"
    [ "4"; "12"; "1"; "0" ]
    (List.map show [ run_gcd 12 8; run_gcd 48 36; run_gcd 17 5; run_gcd 0 0 ])
;;

let substrate_failures () =
  let check name expected computed =
    the_string name expected (outcome_message computed)
  in
  check
    "an unknown branch label fails at assembly"
    "unknown label nowhere"
    (Machine.make_machine
       ~registers:[ "a" ]
       ~operations:Machine.arith_operations
       ~controller:{|(controller (branch (label nowhere)))|}
     >>= fun m -> Machine.start m);
  check
    "an unknown operation fails at assembly"
    "unknown operation foo"
    (Machine.make_machine
       ~registers:[ "a" ]
       ~operations:Machine.arith_operations
       ~controller:{|(controller (assign a (op foo)))|}
     >>= fun m -> Machine.start m);
  check
    "an undeclared register fails at assembly"
    "unknown register x"
    (Machine.make_machine
       ~registers:[ "a" ]
       ~operations:Machine.arith_operations
       ~controller:{|(controller (assign x (const 1)))|}
     >>= fun m -> Machine.start m);
  check
    "a duplicate label fails the parse"
    "bad instruction: the label a is used twice"
    (Machine.parse_program {|(controller a (assign x (const 1)) a)|});
  check
    "a branch without a test fails"
    "branch without a preceding test"
    (Machine.make_machine
       ~registers:[]
       ~operations:Machine.arith_operations
       ~controller:{|(controller (branch (label done)) done)|}
     >>= fun m -> Machine.start m);
  check
    "a restore on an empty stack fails"
    "stack underflow restoring a"
    (Machine.make_machine
       ~registers:[ "a" ]
       ~operations:Machine.arith_operations
       ~controller:{|(controller (restore a))|}
     >>= fun m -> Machine.start m);
  check
    "a goto through a non-label register fails"
    "bad instruction: goto reads 5 from continue, not a label"
    (Machine.make_machine
       ~registers:[ "continue" ]
       ~operations:Machine.arith_operations
       ~controller:{|(controller (assign continue (const 5)) (goto (reg continue)))|}
     >>= fun m -> Machine.start m);
  the_string
    "a non-controller form fails the parse"
    "machine parse error: the controller is written (controller label-or-instruction ...)"
    (parse_message {|(machine)|});
  the_string
    "an unbalanced form fails the reader"
    "machine parse error: 1:14: unexpected end of input"
    (parse_message {|(controller ((|});
  the_string
    "a non-instruction form fails the grammar"
    "bad instruction: the form is not one of the instructions of 5.1.5"
    (parse_message {|(controller (frobnicate (const 1)))|});
  the_string
    "an assign without a source fails the grammar"
    "bad instruction: an assign needs one source or an operation with its inputs"
    (parse_message {|(controller (assign a))|})
;;

(* The driver-loop operations of 5.1.1's Actions: read consumes the
   queue, print appends, and the exhausted read stops the machine. *)
let substrate_driver () =
  let show_unit = function
    | Ok () -> "ok"
    | Error e -> "Error: " ^ Machine.error_to_string e
  in
  let q = Queue.create () in
  Queue.push (Machine.Int 12) q;
  let out = ref [] in
  Machine.make_machine
    ~registers:[ "a" ]
    ~operations:(Machine.read_print ~inputs:q ~output:out @ Machine.arith_operations)
    ~controller:{|(controller (assign a (op read)) (perform (op print) (reg a)))|}
  >>= fun m ->
  let first_run = Machine.start m in
  let second_run = Machine.start m in
  let () = strings "the driver transcript is in order" [ "12" ] !out in
  the_string "the first pass answers ok" "ok" (show_unit first_run);
  the_string
    "the exhausted read is a typed failure"
    "Error: operation failed: read: the input is exhausted"
    (show_unit second_run);
  Ok ()
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
    ; "Error: operation failed: read: the input is exhausted"
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
    [ "1.4142156862745097"; "3.00009155413138"; "1.4142156862745097"; "3.00009155413138" ]
    (Sec_5_3.ex_5_03 ())
;;

let ex_5_04_expt_machines () =
  answers
    "the recursive and iterative expt machines agree"
    [ "1024"; "243"; "1024"; "243" ]
    (Sec_5_4.ex_5_04 ())
;;

let fact_trace =
  [ "(save continue) stack=(fact-done)"
  ; "(save n) stack=(3 fact-done)"
  ; "(save continue) stack=(after-fact 3 fact-done)"
  ; "(save n) stack=(2 after-fact 3 fact-done)"
  ; "branch taken to base-case"
  ; "return to after-fact"
  ; "(restore n) n=2 stack=(after-fact 3 fact-done)"
  ; "(restore continue) continue=after-fact stack=(3 fact-done)"
  ; "return to after-fact"
  ; "(restore n) n=3 stack=(fact-done)"
  ; "(restore continue) continue=fact-done stack=()"
  ; "return to fact-done"
  ; "answer 6"
  ]
;;

let fib_trace =
  [ "(save continue) stack=(fib-done)"
  ; "(save n) stack=(3 fib-done)"
  ; "(save continue) stack=(afterfib-n-1 3 fib-done)"
  ; "(save n) stack=(2 afterfib-n-1 3 fib-done)"
  ; "branch taken to immediate-answer"
  ; "return to afterfib-n-1"
  ; "(restore n) n=2 stack=(afterfib-n-1 3 fib-done)"
  ; "(restore continue) continue=afterfib-n-1 stack=(3 fib-done)"
  ; "(save continue) stack=(afterfib-n-1 3 fib-done)"
  ; "(save val) stack=(1 afterfib-n-1 3 fib-done)"
  ; "branch taken to immediate-answer"
  ; "return to afterfib-n-2"
  ; "(restore val) val=1 stack=(afterfib-n-1 3 fib-done)"
  ; "(restore continue) continue=afterfib-n-1 stack=(3 fib-done)"
  ; "return to afterfib-n-1"
  ; "(restore n) n=3 stack=(fib-done)"
  ; "(restore continue) continue=fib-done stack=()"
  ; "(save continue) stack=(fib-done)"
  ; "(save val) stack=(1 fib-done)"
  ; "branch taken to immediate-answer"
  ; "return to afterfib-n-2"
  ; "(restore val) val=1 stack=(fib-done)"
  ; "(restore continue) continue=fib-done stack=()"
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
   Figure 5.11 and Figure 5.12. *)
let hand_agrees_with_simulator () =
  let simulated controller n =
    run_controller controller [ "n"; "val"; "continue" ] [ "n", Machine.Int n ] "val"
  in
  the_string
    "the factorial hand model answers what the machine answers"
    "6"
    (show (simulated Sec_5_5.factorial_recursive_controller 3));
  the_string
    "the fib hand model answers what the machine answers"
    "2"
    (show (simulated Sec_5_5.fib_controller 3));
  strings
    "the modified fib machine answers like the original on ten inputs"
    (List.init 10 (fun n -> show (simulated Sec_5_5.fib_controller n)))
    (List.init 10 (fun n -> show (simulated Sec_5_6.fib_modified_controller n)))
;;

let () =
  let open Alcotest in
  run
    "sicp section 5.1"
    [ ( "substrate"
      , [ test_case "gcd machines run" `Quick substrate_gcd
        ; test_case "every failure is typed and named" `Quick substrate_failures
        ; test_case "the driver operations read, print, and fail typed" `Quick (fun () ->
            ignore (substrate_driver ()))
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
