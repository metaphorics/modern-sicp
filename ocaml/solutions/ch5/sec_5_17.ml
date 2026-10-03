(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Monitor = Sec_5_15.Monitor

let trace_line m i =
  let text = M.instruction_to_string M.value_to_string (Monitor.instruction_at m i) in
  match Monitor.instruction_labels m i with
  | [] -> text
  | labels -> String.concat " " labels ^ ": " ^ text
;;

let ex_5_17 () =
  let* m =
    Monitor.make
      ~registers:[ "n"; "val"; "continue" ]
      ~operations:M.arith_operations
      ~controller:Sec_5_5.fib_controller
  in
  let lines = ref [] in
  let* () = Monitor.set_register m "n" (M.Int 3) in
  let* _ = Monitor.start ~before:(fun i -> lines := trace_line m i :: !lines) m in
  let* answer = Monitor.get_register m "val" in
  Ok
    (List.rev !lines
     @ [ Printf.sprintf
           "fib 3 = %s in %d instructions"
           (M.value_to_string answer)
           (Monitor.instruction_count m)
       ])
;;
