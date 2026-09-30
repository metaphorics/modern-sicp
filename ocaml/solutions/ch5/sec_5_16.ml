(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Monitor = Sec_5_15.Monitor

let gcd m a b ?before () =
  let* () = Monitor.set_register m "a" (M.Int a) in
  let* () = Monitor.set_register m "b" (M.Int b) in
  let* _ = Monitor.start ?before m in
  Monitor.get_register m "a"
;;

let ex_5_16 () =
  let* m =
    Monitor.make
      ~registers:[ "a"; "b"; "t" ]
      ~operations:M.arith_operations
      ~controller:Sec_5_10.gcd_controller
  in
  let lines = ref [] in
  let trace i =
    lines
    := M.instruction_to_string M.value_to_string (Monitor.instruction_at m i) :: !lines
  in
  let* first_answer = gcd m 12 8 ~before:trace () in
  let traced = List.rev !lines in
  let* second_answer = gcd m 20 14 () in
  let silent_lines = List.length !lines - List.length traced in
  Ok
    (traced
     @ [ "gcd 12 8 = " ^ M.value_to_string first_answer
       ; Printf.sprintf
           "with tracing off the run on (20, 14) printed %d trace lines and answered %s"
           silent_lines
           (M.value_to_string second_answer)
       ])
;;
