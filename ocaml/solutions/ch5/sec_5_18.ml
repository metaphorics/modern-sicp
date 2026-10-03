(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Monitor = Sec_5_15.Monitor

let ex_5_18 () =
  let* m =
    Monitor.make
      ~registers:[ "n"; "val"; "continue" ]
      ~operations:M.arith_operations
      ~controller:Sec_5_5.factorial_recursive_controller
  in
  let* () = Monitor.trace_register m "n" true in
  let* () = Monitor.trace_register m "val" true in
  let* () = Monitor.set_register m "n" (M.Int 3) in
  let* _ = Monitor.start m in
  let* answer = Monitor.get_register m "val" in
  Ok (Monitor.traced_assignments m @ [ "fact 3 = " ^ M.value_to_string answer ])
;;
