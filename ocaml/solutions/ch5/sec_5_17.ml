(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.17: the trace prints the labels that precede each
    instruction, the information the assembler retained. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2
module Sim = Sec_5_15.Sim

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

(** [trace_line m i] renders one traced instruction: the labels the
    assembler attached to it, then the instruction. *)
let trace_line m i =
  let labels = Sim.instruction_labels m i in
  let text = Machine.instruction_to_string (Sim.instruction_text m i) in
  match labels with
  | [] -> text
  | _ -> String.concat " " labels ^ ": " ^ text
;;

(** [ex_5_17 ()] traces the Fibonacci machine on [n = 3]; every label
    crossing -- fib-loop, afterfib-n-1, afterfib-n-2, immediate-answer
    -- announces itself, and the instruction count is the traced
    run's, undisturbed by the printing. *)
let ex_5_17 () =
  Sim.make
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller:fib_controller
  >>= fun m ->
  let lines = ref [] in
  Sim.set_register m "n" (Machine.Int 3)
  >>= fun () ->
  Sim.start ~before:(fun i -> lines := !lines @ [ trace_line m i ]) m
  >>= fun _ ->
  Sim.get_register m "val"
  >>= fun answer ->
  let count = Sim.instruction_count m in
  Ok
    (!lines
     @ [ "fib 3 = "
         ^ Machine.value_to_string answer
         ^ " in "
         ^ string_of_int count
         ^ " instructions"
       ])
;;
