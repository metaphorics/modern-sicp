(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the simulator's machine of 5.2 with the stack
   monitors of 5.2.4, results proved with [expect]. *)

module Machine = Sicp_ch5.Sec_5_2
module M = Sicp_ch5.Sec_5_1
module Replay = Sicp_ch1.Replay

let ( let* ) = Result.bind

(* Two pushes and one restore: the monitors count pushes and the
   deepest the stack reached, not its final depth. *)
let stack_demo =
  let* m =
    Machine.make_machine
      ~registers:[ "x" ]
      ~operations:M.arith_operations
      ~controller:
        [ M.Perform ("initialize-stack", [])
        ; M.Save "x"
        ; M.Save "x"
        ; M.Restore "x"
        ; M.Perform ("print-stack-statistics", [])
        ; M.Perform ("print", [ M.Reg "x" ])
        ]
  in
  let* () = Machine.set_register m "x" (Machine.Int 7) in
  let* () = Machine.start m in
  Ok (Machine.transcript m)
;;

let () =
  match stack_demo with
  | Ok lines ->
    Replay.expect (String.concat " | " lines) "total-pushes = 2 maximum-depth = 2 | 7"
  | Error e -> Replay.expect ("error: " ^ Sicp_common.Eval_error.to_string e) "no error"
;;
