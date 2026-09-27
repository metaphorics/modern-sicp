(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.1 and this edition's 5.1a: the iterative factorial
    machine, designed in the book's register-machine language and
    pinned by running it, then wrapped in the driver loop of 5.1.1's
    Actions over repeated inputs. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_1

let rec all f = function
  | [] -> Ok []
  | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
;;

(** [run_controller controller ~registers ~inputs ~result] assembles the
    controller text, loads the input registers in order, starts the
    machine, and reads the result register. *)
let run_controller controller ~registers ~inputs ~result =
  Machine.make_machine ~registers ~operations:Machine.arith_operations ~controller
  >>= fun m ->
  all (fun (name, v) -> Machine.set_register m name v) inputs
  >>= fun _ -> Machine.start m >>= fun () -> Machine.get_register m result
;;

(** The controller of Exercise 5.1: three registers ([n], [product],
    [counter]), one test, and the two data-path buttons of the
    iteration step. The drawings the statement asks for are the
    data-path and controller diagrams in solutions/ch5/ex_5_01.md, in
    the notation of the book's Figures 5.1 and 5.2. *)
let factorial_iterative_controller =
  {|(controller
 fact-loop
   (test (op >) (reg counter) (reg n))
   (branch (label fact-done))
   (assign product (op *) (reg counter) (reg product))
   (assign counter (op +) (reg counter) (const 1))
   (goto (label fact-loop))
 fact-done)|}
;;

(** [ex_5_01 ()] runs the designed machine on 0, 1, 5, and 10. *)
let ex_5_01 () =
  all
    (fun n ->
       run_controller
         factorial_iterative_controller
         ~registers:[ "n"; "product"; "counter" ]
         ~inputs:
           [ "n", Machine.Int n; "product", Machine.Int 1; "counter", Machine.Int 1 ]
         ~result:"product")
    [ 0; 1; 5; 10 ]
  >>= fun answers -> Ok (List.map Machine.value_to_string answers)
;;

(** The 5.1 machine wrapped in the driver loop of 5.1.1's Actions: each
    pass reads [n], computes its factorial, prints it, and starts over;
    the symbol [end], written [(const end)] and compared by the
    machine's [=] test, is the only way out. *)
let factorial_driver_controller =
  {|(controller
 factorial-loop
   (assign n (op read))
   (test (op =) (reg n) (const end))
   (branch (label machine-done))
   (assign product (const 1))
   (assign counter (const 1))
 fact-loop
   (test (op >) (reg counter) (reg n))
   (branch (label fact-done))
   (assign product (op *) (reg counter) (reg product))
   (assign counter (op +) (reg counter) (const 1))
   (goto (label fact-loop))
 fact-done
   (perform (op print) (reg product))
   (goto (label factorial-loop))
 machine-done)|}
;;

(** [drive_factorial inputs] runs the driver-loop machine over [inputs];
    the transcript collects in order, and the stop report names either
    the [end] symbol or the read failure that stopped the machine. *)
let drive_factorial inputs =
  let q = Queue.create () in
  List.iter (fun v -> Queue.push v q) inputs;
  let out = ref [] in
  let outcome =
    Machine.make_machine
      ~registers:[ "n"; "product"; "counter" ]
      ~operations:(Machine.read_print ~inputs:q ~output:out @ Machine.arith_operations)
      ~controller:factorial_driver_controller
    >>= fun m -> Machine.start m
  in
  let stop =
    match outcome with
    | Ok () -> "end"
    | Error e -> "Error: " ^ Machine.error_to_string e
  in
  Ok (!out, stop)
;;

(** [ex_5_01a ()] is the transcript of two consecutive runs -- inputs 5
    then 6, and inputs 1 then 10, each closed by [end] -- then a run
    whose input dries up, where the typed read failure is what stops
    the unbounded loop. Each run contributes its printed lines and a
    final stop report. *)
let ex_5_01a () =
  drive_factorial [ Machine.Int 5; Machine.Int 6; Machine.Symbol "end" ]
  >>= fun (transcript1, stop1) ->
  drive_factorial [ Machine.Int 1; Machine.Int 10; Machine.Symbol "end" ]
  >>= fun (transcript2, stop2) ->
  drive_factorial [ Machine.Int 5 ]
  >>= fun (transcript3, stop3) ->
  Ok (transcript1 @ [ stop1 ] @ transcript2 @ [ stop2 ] @ transcript3 @ [ stop3 ])
;;
