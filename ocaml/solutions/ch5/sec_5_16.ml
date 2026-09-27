(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.16: instruction tracing, on and off. *)

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

(** [ex_5_16 ()] runs the GCD machine on (12, 8) with tracing on --
    one line per executed instruction -- then runs the same machine
    again with tracing off and shows the silence. *)
let ex_5_16 () =
  Sim.make
    ~registers:[ "a"; "b"; "t" ]
    ~operations:Machine.arith_operations
    ~controller:gcd_controller
  >>= fun m ->
  let lines = ref [] in
  Sim.set_register m "a" (Machine.Int 12)
  >>= fun () ->
  Sim.set_register m "b" (Machine.Int 8)
  >>= fun () ->
  Sim.start
    ~before:(fun i ->
      lines := !lines @ [ Machine.instruction_to_string (Sim.instruction_text m i) ])
    m
  >>= fun _ ->
  Sim.get_register m "a"
  >>= fun first_answer ->
  let traced = !lines in
  let before_len = List.length traced in
  Sim.set_register m "a" (Machine.Int 20)
  >>= fun () ->
  Sim.set_register m "b" (Machine.Int 14)
  >>= fun () ->
  Sim.start m
  >>= fun _ ->
  Sim.get_register m "a"
  >>= fun second_answer ->
  let traced_lines = List.length !lines - before_len in
  Ok
    (traced
     @ [ "gcd 12 8 = " ^ Machine.value_to_string first_answer
       ; "with tracing off the run on (20, 14) printed "
         ^ string_of_int traced_lines
         ^ " trace lines and answered "
         ^ Machine.value_to_string second_answer
       ])
;;
