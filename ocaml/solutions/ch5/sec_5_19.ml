(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.19: breakpoints -- set before the [n]th instruction
    after a label, proceed past them, cancel them. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2
module Sim = Sec_5_15.Sim

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

let show_stop = function
  | Sec_5_15.Completed -> "completed"
  | Sec_5_15.Breakpoint (label, offset) -> "stop at " ^ label ^ " " ^ string_of_int offset
;;

let registers m names =
  List.map
    (fun r ->
       Sim.get_register m r
       |> function
       | Error e -> r ^ "=?" ^ Machine.error_to_string e
       | Ok v -> r ^ " = " ^ Machine.value_to_string v)
    names
  |> String.concat ", "
;;

(** [ex_5_19 ()] installs the book's breakpoint -- [(set-breakpoint
    gcd-machine 'test-b 4)], just before the assignment to [a] -- runs
    the machine, examines it at each stop, proceeds to the answer,
    then cancels the breakpoint and runs through. *)
let ex_5_19 () =
  Sim.make
    ~registers:[ "a"; "b"; "t" ]
    ~operations:Machine.arith_operations
    ~controller:gcd_controller
  >>= fun m ->
  Sim.set_breakpoint m "test-b" 4
  >>= fun () ->
  Sim.set_register m "a" (Machine.Int 12)
  >>= fun () ->
  Sim.set_register m "b" (Machine.Int 8)
  >>= fun () ->
  Sim.start m
  >>= fun stop1 ->
  let at_stop1 = registers m [ "a"; "b" ] in
  Sim.proceed m
  >>= fun stop2 ->
  let at_stop2 = registers m [ "a"; "b" ] in
  Sim.proceed m
  >>= fun stop3 ->
  Sim.get_register m "a"
  >>= fun answer ->
  Sim.cancel_breakpoint m "test-b" 4
  >>= fun () ->
  Sim.set_register m "a" (Machine.Int 20)
  >>= fun () ->
  Sim.set_register m "b" (Machine.Int 14)
  >>= fun () ->
  Sim.start m
  >>= fun stop4 ->
  Sim.get_register m "a"
  >>= fun answer4 ->
  Sim.set_breakpoint m "test-b" 4
  >>= fun () ->
  Sim.set_breakpoint m "test-b" 2
  >>= fun () ->
  Sim.cancel_all_breakpoints m
  >>= fun () ->
  Sim.set_register m "a" (Machine.Int 25)
  >>= fun () ->
  Sim.set_register m "b" (Machine.Int 15)
  >>= fun () ->
  Sim.start m
  >>= fun stop5 ->
  Ok
    [ "breakpoint set at test-b 4"
    ; "start: " ^ show_stop stop1 ^ " (" ^ at_stop1 ^ ")"
    ; "proceed: " ^ show_stop stop2 ^ " (" ^ at_stop2 ^ ")"
    ; "proceed: " ^ show_stop stop3 ^ ", a = " ^ Machine.value_to_string answer
    ; "cancel test-b 4, rerun on (20, 14): "
      ^ show_stop stop4
      ^ ", a = "
      ^ Machine.value_to_string answer4
    ; "set test-b 4 and test-b 2, cancel all, run on (25, 15): " ^ show_stop stop5
    ]
;;
