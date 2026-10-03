(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Monitor = Sec_5_15.Monitor
module Eval_error = Sicp_common.Eval_error

let show_stop = function
  | Sec_5_15.Completed -> "completed"
  | Sec_5_15.Breakpoint (label, offset) -> "stop at " ^ label ^ " " ^ string_of_int offset
;;

let registers m names =
  List.map
    (fun r ->
       match Monitor.get_register m r with
       | Ok v -> r ^ " = " ^ M.value_to_string v
       | Error e -> r ^ " = ? " ^ Eval_error.to_string e)
    names
  |> String.concat ", "
;;

let load m a b =
  let* () = Monitor.set_register m "a" (M.Int a) in
  Monitor.set_register m "b" (M.Int b)
;;

(* The book's [(set-breakpoint gcd-machine 'test-b 4)] sits just before
   the assignment to [a], three instructions past the label. *)
let ex_5_19 () =
  let* m =
    Monitor.make
      ~registers:[ "a"; "b"; "t" ]
      ~operations:M.arith_operations
      ~controller:Sec_5_10.gcd_controller
  in
  let* () = Monitor.set_breakpoint m "test-b" 4 in
  let* () = load m 12 8 in
  let* stop1 = Monitor.start m in
  let at_stop1 = registers m [ "a"; "b" ] in
  let* stop2 = Monitor.proceed m in
  let at_stop2 = registers m [ "a"; "b" ] in
  let* stop3 = Monitor.proceed m in
  let* answer = Monitor.get_register m "a" in
  Monitor.cancel_breakpoint m "test-b" 4;
  let* () = load m 20 14 in
  let* stop4 = Monitor.start m in
  let* answer4 = Monitor.get_register m "a" in
  let* () = Monitor.set_breakpoint m "test-b" 4 in
  let* () = Monitor.set_breakpoint m "test-b" 2 in
  Monitor.cancel_all_breakpoints m;
  let* () = load m 25 15 in
  let* stop5 = Monitor.start m in
  Ok
    [ "breakpoint set at test-b 4"
    ; "start: " ^ show_stop stop1 ^ " (" ^ at_stop1 ^ ")"
    ; "proceed: " ^ show_stop stop2 ^ " (" ^ at_stop2 ^ ")"
    ; "proceed: " ^ show_stop stop3 ^ ", a = " ^ M.value_to_string answer
    ; "cancel test-b 4, rerun on (20, 14): "
      ^ show_stop stop4
      ^ ", a = "
      ^ M.value_to_string answer4
    ; "set test-b 4 and test-b 2, cancel all, run on (25, 15): " ^ show_stop stop5
    ]
;;
