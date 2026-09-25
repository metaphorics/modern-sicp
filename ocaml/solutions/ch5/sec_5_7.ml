(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.7 and this edition's 5.7a: the machines of Exercise 5.4
    run on the section's simulator, and each simulated run paired with
    a direct host computation as its oracle. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2

let rec all f = function
  | [] -> Ok []
  | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
;;

(** [run controller ~registers ~inputs ~result] assembles the
    controller with the section's simulator, loads the inputs, starts
    the machine, and reads the result register. *)
let run controller ~registers ~inputs ~result =
  Machine.make_machine ~registers ~operations:Machine.arith_operations ~controller
  >>= fun m ->
  all (fun (name, v) -> Machine.set_register m name v) inputs
  >>= fun _ -> Machine.start m >>= fun () -> Machine.get_register m result
;;

(** The recursive exponentiation machine of Exercise 5.4. *)
let expt_recursive_controller =
  {|(controller
   (assign continue (label expt-done))
 expt-loop
   (test (op =) (reg n) (const 0))
   (branch (label base-case))
   (save continue)
   (assign n (op -) (reg n) (const 1))
   (assign continue (label after-expt))
   (goto (label expt-loop))
 after-expt
   (restore continue)
   (assign val (op *) (reg b) (reg val))
   (goto (reg continue))
 base-case
   (assign val (const 1))
   (goto (reg continue))
 expt-done)|}
;;

(** The iterative exponentiation machine of Exercise 5.4. *)
let expt_iterative_controller =
  {|(controller
 expt-iter
   (test (op =) (reg counter) (const 0))
   (branch (label expt-done))
   (assign counter (op -) (reg counter) (const 1))
   (assign product (op *) (reg b) (reg product))
   (goto (label expt-iter))
 expt-done)|}
;;

(** [ex_5_07 ()] runs both 5.4 machines on (2, 10) and (3, 5) and
    reads the answers the simulator produces. *)
let ex_5_07 () =
  let b = "b", Machine.Int 2
  and b5 = "b", Machine.Int 3 in
  run
    expt_recursive_controller
    ~registers:[ "b"; "n"; "val"; "continue" ]
    ~inputs:[ b; "n", Machine.Int 10 ]
    ~result:"val"
  >>= fun r1 ->
  run
    expt_recursive_controller
    ~registers:[ "b"; "n"; "val"; "continue" ]
    ~inputs:[ b5; "n", Machine.Int 5 ]
    ~result:"val"
  >>= fun r2 ->
  run
    expt_iterative_controller
    ~registers:[ "b"; "counter"; "product" ]
    ~inputs:[ b; "product", Machine.Int 1; "counter", Machine.Int 10 ]
    ~result:"product"
  >>= fun r3 ->
  run
    expt_iterative_controller
    ~registers:[ "b"; "counter"; "product" ]
    ~inputs:[ b5; "product", Machine.Int 1; "counter", Machine.Int 5 ]
    ~result:"product"
  >>= fun r4 -> Ok (List.map Machine.value_to_string [ r1; r2; r3; r4 ])
;;

(** The Fibonacci machine of Figure 5.12, the recursive process whose
    direct counterpart is the host's recursive [fib]. *)
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

(** The iterative factorial machine of Exercise 5.1, whose direct
    counterpart is the host's loop over product and counter. *)
let factorial_iterative_controller =
  {|(controller
   (assign product (const 1))
   (assign counter (const 1))
 fact-loop
   (test (op >) (reg counter) (reg n))
   (branch (label fact-done))
   (assign product (op *) (reg counter) (reg product))
   (assign counter (op +) (reg counter) (const 1))
   (goto (label fact-loop))
 fact-done)|}
;;

(** The oracle side of 5.7a: the direct host computations the machine
    runs are paired with. *)
let rec host_fib = function
  | 0 -> 0
  | 1 -> 1
  | n -> host_fib (n - 1) + host_fib (n - 2)
;;

let host_factorial n =
  let rec loop product counter =
    if counter > n then product else loop (counter * product) (counter + 1)
  in
  loop 1 1
;;

(** [ex_5_07a ()] is the transcript of the oracle pairing: for every
    [n] in the shared argument set 0..6 the simulated run and the
    direct host computation appear side by side, and the run is
    correct exactly when the two agree. *)
let ex_5_07a () =
  let report machine direct =
    if String.equal machine direct
    then machine ^ " (direct " ^ direct ^ ") ok"
    else machine ^ " (direct " ^ direct ^ ") MISMATCH"
  in
  let one n =
    run
      fib_controller
      ~registers:[ "n"; "val"; "continue" ]
      ~inputs:[ "n", Machine.Int n ]
      ~result:"val"
    >>= fun fib_v ->
    run
      factorial_iterative_controller
      ~registers:[ "n"; "product"; "counter" ]
      ~inputs:[ "n", Machine.Int n ]
      ~result:"product"
    >>= fun fact_v ->
    let fib_line = report (Machine.value_to_string fib_v) (string_of_int (host_fib n)) in
    let fact_line =
      report (Machine.value_to_string fact_v) (string_of_int (host_factorial n))
    in
    Ok [ "n=" ^ string_of_int n ^ ": fib " ^ fib_line ^ "; factorial " ^ fact_line ]
  in
  let rec over = function
    | [] -> Ok []
    | n :: ns -> one n >>= fun line -> over ns >>= fun rest -> Ok (line @ rest)
  in
  over [ 0; 1; 2; 3; 4; 5; 6 ]
;;
