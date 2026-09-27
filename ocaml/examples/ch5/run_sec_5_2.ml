(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the machine the section's listings build runs,
   and every result comment the adapted prose displays is proved with
   [expect]. *)

module Machine = Sicp_ch5.Sec_5_2
module Replay = Sicp_ch1.Replay

let ( >>= ) = Result.bind

(* The GCD machine of 5.1.1 as the section assembles it: registers,
   the shared arithmetic table, and the controller text. *)
let gcd_demo =
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
;;

(* The monitored stack of 5.2.4: two pushes and one restore leave
   their counters on the transcript. *)
let stack_demo =
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
  >>= fun m ->
  Machine.set_register m "x" (Machine.Int 7)
  >>= fun () -> Machine.start m >>= fun () -> Ok (Machine.transcript m)
;;

let () =
  Replay.expect
    (match gcd_demo with
     | Ok v -> Machine.value_to_string v
     | Error e -> "Error: " ^ Machine.error_to_string e)
    "2";
  Replay.expect
    (match stack_demo with
     | Ok [ line ] -> line
     | Ok lines -> String.concat " | " lines
     | Error e -> "Error: " ^ Machine.error_to_string e)
    "total-pushes = 2 maximum-depth = 2"
;;
