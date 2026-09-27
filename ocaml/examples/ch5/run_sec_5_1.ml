(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Machine = Sicp_ch5.Sec_5_1
module Replay = Sicp_ch1.Replay

let ( >>= ) = Result.bind

let show = function
  | Ok v -> Machine.value_to_string v
  | Error e -> "Error: " ^ Machine.error_to_string e
;;

let run_machine machine_text registers a_name a b_name b result_name =
  Machine.make_machine
    ~registers
    ~operations:Machine.arith_operations
    ~controller:machine_text
  >>= fun m ->
  Machine.set_register m a_name a
  >>= fun () ->
  Machine.set_register m b_name b
  >>= fun () -> Machine.start m >>= fun () -> Machine.get_register m result_name
;;

(* The GCD machine of 5.1.1 as the edition's data: the same controller
   text both listings of the section show. *)
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
  run_machine gcd_controller [ "a"; "b"; "t" ] "a" (Machine.Int a) "b" (Machine.Int b) "a"
;;

(* Figure 5.6: the elaborated machine computes remainders by repeated
   subtraction in its own rem-loop. *)
let gcd_subtraction_controller =
  {|(controller
 test-b
   (test (op =) (reg b) (const 0))
   (branch (label gcd-done))
   (assign t (reg a))
 rem-loop
   (test (op <) (reg t) (reg b))
   (branch (label rem-done))
   (assign t (op -) (reg t) (reg b))
   (goto (label rem-loop))
 rem-done
   (assign a (reg b))
   (assign b (reg t))
   (goto (label test-b))
 gcd-done)|}
;;

let run_gcd_subtraction a b =
  run_machine
    gcd_subtraction_controller
    [ "a"; "b"; "t" ]
    "a"
    (Machine.Int a)
    "b"
    (Machine.Int b)
    "a"
;;

let driver_outcome controller registers inputs =
  let q = Queue.create () in
  List.iter (fun v -> Queue.push v q) inputs;
  let out = ref [] in
  let outcome =
    Machine.make_machine
      ~registers
      ~operations:(Machine.read_print ~inputs:q ~output:out @ Machine.arith_operations)
      ~controller
    >>= fun m -> Machine.start m
  in
  let stop =
    match outcome with
    | Ok () -> "ok"
    | Error e -> Machine.error_to_string e
  in
  !out, stop
;;

(* Figure 5.4: the GCD machine that reads inputs and prints results,
   looping like the driver loops of chapter 4. *)
let gcd_driver_controller =
  {|(controller
 gcd-loop
   (assign a (op read))
   (assign b (op read))
 test-b
   (test (op =) (reg b) (const 0))
   (branch (label gcd-done))
   (assign t (op rem) (reg a) (reg b))
   (assign a (reg b))
   (assign b (reg t))
   (goto (label test-b))
 gcd-done
   (perform (op print) (reg a))
   (goto (label gcd-loop)))|}
;;

(* Figure 5.11: the recursive factorial machine. *)
let factorial_recursive_controller =
  {|(controller
   (assign continue (label fact-done))
 fact-loop
   (test (op =) (reg n) (const 1))
   (branch (label base-case))
   (save continue)
   (save n)
   (assign n (op -) (reg n) (const 1))
   (assign continue (label after-fact))
   (goto (label fact-loop))
 after-fact
   (restore n)
   (restore continue)
   (assign val (op *) (reg n) (reg val))
   (goto (reg continue))
 base-case
   (assign val (const 1))
   (goto (reg continue))
 fact-done)|}
;;

let run_factorial_recursive n =
  run_machine
    factorial_recursive_controller
    [ "n"; "val"; "continue" ]
    "n"
    (Machine.Int n)
    "val"
    (Machine.Int 0)
    "val"
;;

(* Figure 5.12: the Fibonacci machine. *)
let fib_controller =
  {|(controller
   (assign continue (label fib-done))
 fib-loop
   (test (op <) (reg n) (const 2))
   (branch (label immediate-answer))
   (save continue)
   (assign continue (label afterfib-n-1))
   (save n)
   (assign n (op -) (reg n) (const 1))
   (goto (label fib-loop))
 afterfib-n-1
   (restore n)
   (restore continue)
   (assign n (op -) (reg n) (const 2))
   (save continue)
   (assign continue (label afterfib-n-2))
   (save val)
   (goto (label fib-loop))
 afterfib-n-2
   (assign n (reg val))
   (restore val)
   (restore continue)
   (assign val (op +) (reg val) (reg n))
   (goto (reg continue))
 immediate-answer
   (assign val (reg n))
   (goto (reg continue))
 fib-done)|}
;;

let run_fib n =
  run_machine
    fib_controller
    [ "n"; "val"; "continue" ]
    "n"
    (Machine.Int n)
    "val"
    (Machine.Int 0)
    "val"
;;

let () =
  (* 5.1.1: the GCD machine of Figure 5.3 and of the combined
     description. *)
  Replay.expect (show (run_gcd 12 8)) "4";
  Replay.expect (show (run_gcd 48 36)) "12";
  Replay.expect (show (run_gcd 17 5)) "1";
  (* 5.1.1 Actions: the driver-loop machine's transcript, and the read
     failure that stops it when the input runs dry. *)
  let transcript, stop =
    driver_outcome
      gcd_driver_controller
      [ "a"; "b"; "t" ]
      [ Machine.Int 12; Machine.Int 8; Machine.Int 20; Machine.Int 14 ]
  in
  Replay.expect (String.concat " " transcript) "4 2";
  Replay.expect stop "operation failed: read: the input is exhausted";
  (* 5.1.1 Actions: single instructions are data, and the notation
     round-trips through parse and print. *)
  let round_trip text =
    match Machine.parse_program text with
    | Ok p -> Machine.instruction_to_string p.code.(0)
    | Error e -> Machine.error_to_string e
  in
  Replay.expect
    (round_trip {|(controller (perform (op print) (reg a)))|})
    "(perform (op print) (reg a))";
  Replay.expect
    (round_trip {|(controller (assign t (op rem) (reg a) (reg b)))|})
    "(assign t (op rem) (reg a) (reg b))";
  (* 5.1.2: the elaborated machine without the primitive remainder. *)
  Replay.expect (show (run_gcd_subtraction 12 8)) "4";
  Replay.expect (show (run_gcd_subtraction 25 55)) "5";
  (* 5.1.3: the fragments of Figures 5.7 through 5.10 assemble; their
     instruction counts are the book's sequences as data. *)
  let fragment_count text =
    match Machine.parse_program text with
    | Ok p -> string_of_int (Array.length p.code)
    | Error e -> Machine.error_to_string e
  in
  Replay.expect
    (fragment_count
       {|(controller
 gcd-1
 (test (op =) (reg b) (const 0))
 (branch (label after-gcd-1))
 (assign t (op rem) (reg a) (reg b))
 (assign a (reg b))
 (assign b (reg t))
 (goto (label gcd-1))
after-gcd-1
gcd-2
 (test (op =) (reg d) (const 0))
 (branch (label after-gcd-2))
 (assign s (op rem) (reg c) (reg d))
 (assign c (reg d))
 (assign d (reg s))
 (goto (label gcd-2))
after-gcd-2)|})
    "12";
  Replay.expect
    (fragment_count
       {|(controller
 gcd-1
 (test (op =) (reg b) (const 0))
 (branch (label after-gcd-1))
 (assign t (op rem) (reg a) (reg b))
 (assign a (reg b))
 (assign b (reg t))
 (goto (label gcd-1))
after-gcd-1
gcd-2
 (test (op =) (reg b) (const 0))
 (branch (label after-gcd-2))
 (assign t (op rem) (reg a) (reg b))
 (assign a (reg b))
 (assign b (reg t))
 (goto (label gcd-2))
after-gcd-2)|})
    "12";
  Replay.expect
    (fragment_count
       {|(controller
 gcd
 (test (op =) (reg b) (const 0))
 (branch (label gcd-done))
 (assign t (op rem) (reg a) (reg b))
 (assign a (reg b))
 (assign b (reg t))
 (goto (label gcd))
gcd-done
 (test (op =) (reg continue) (const 0))
 (branch (label after-gcd-1))
 (goto (label after-gcd-2))
 ; before branching to gcd from the first place where it is needed,
 ; we place 0 in the continue register
 (assign continue (const 0))
 (goto (label gcd))
after-gcd-1
 ; before the second use of gcd, we place 1 in the continue register
 (assign continue (const 1))
 (goto (label gcd))
after-gcd-2)|})
    "13";
  Replay.expect
    (fragment_count
       {|(controller
 gcd
 (test (op =) (reg b) (const 0))
 (branch (label gcd-done))
 (assign t (op rem) (reg a) (reg b))
 (assign a (reg b))
 (assign b (reg t))
 (goto (label gcd))
gcd-done
 (goto (reg continue))
 ; before calling gcd, we assign to continue the label to which gcd
 ; should return
 (assign continue (label after-gcd-1))
 (goto (label gcd))
after-gcd-1
 ; here is the second call to gcd, with a different continuation
 (assign continue (label after-gcd-2))
 (goto (label gcd))
after-gcd-2)|})
    "11";
  (* 5.1.4: the recursive factorial machine of Figure 5.11. *)
  Replay.expect (show (run_factorial_recursive 5)) "120";
  Replay.expect (show (run_factorial_recursive 3)) "6";
  (* 5.1.4: the Fibonacci machine of Figure 5.12. *)
  Replay.expect (show (run_fib 6)) "8";
  Replay.expect (show (run_fib 10)) "55"
;;
