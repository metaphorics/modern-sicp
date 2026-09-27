(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.18: register tracing -- every write to a traced
    register reports the register, the old contents, and the new. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2
module Sim = Sec_5_15.Sim

(** The recursive factorial machine of Figure 5.11. *)
let factorial_controller =
  {|(controller
   (assign continue (label fact-done))
 fact-loop
   (test (op =) (reg n) (const 1))
   (branch (label base-case))
   (save continue)
   (save n)
   (assign n (op -) (reg n) (const 1))
   (assign continue (label after-fact))
   (goto (label fact-loop))
 after-fact
   (restore n)
   (restore continue)
   (assign val (op *) (reg n) (reg val))
   (goto (reg continue))
 base-case
   (assign val (const 1))
   (goto (reg continue))
 fact-done)|}
;;

(** [ex_5_18 ()] traces [n] and [val] through the factorial machine on
    [n = 3]: the host's input load, the recursion's assigns, and the
    restores all report, then the answer. *)
let ex_5_18 () =
  Sim.make
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller:factorial_controller
  >>= fun m ->
  Sim.trace_register m "n" true
  >>= fun () ->
  Sim.trace_register m "val" true
  >>= fun () ->
  Sim.set_register m "n" (Machine.Int 3)
  >>= fun () ->
  Sim.start m
  >>= fun _ ->
  Sim.get_register m "val"
  >>= fun answer ->
  Ok (Sim.traced_assignments m @ [ "fact 3 = " ^ Machine.value_to_string answer ])
;;
