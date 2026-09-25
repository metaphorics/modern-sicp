(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.6: the redundant save/restore pair of the Fibonacci
    machine, removed and proved by counts. The counts come from the
    hand-simulation transcription of Sec_5_5, so the before/after
    comparison measures the same semantics on the two controllers. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_1
module Handsim = Sec_5_5.Handsim

(** The Figure 5.12 controller with the redundant pair removed: the
    [(restore continue)] at [afterfib-n-1] and the [(save continue)] in
    the second call's setup. Each level's caller return address is
    pushed once (before the first call) and popped once (at
    [afterfib-n-2]); the middle push/pop carried the same value twice. *)
let fib_modified_controller =
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
   (assign n (op -) (reg n) (const 2))
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

let run_fib_text text n =
  Machine.make_machine
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller:text
  >>= fun m ->
  Machine.set_register m "n" (Machine.Int n)
  >>= fun () -> Machine.start m >>= fun () -> Machine.get_register m "val"
;;

let counts program n =
  Handsim.run program (Handsim.initial [ "n", Machine.Int n; "val", Machine.Int 0 ]) []
  >>= fun (_, st) -> Ok (Printf.sprintf "steps=%d saves=%d" st.steps st.saves)
;;

(** [ex_5_06 ()] asserts the removal: both machines answer alike on 10,
    and on [fib 6] the transcription counts -- 257 steps against 281,
    36 saves against 48 -- show the twelve internal calls of the
    [fib 6] tree each shedding one save and one restore. *)
let ex_5_06 () =
  run_fib_text Sec_5_5.fib_controller 10
  >>= fun before ->
  run_fib_text fib_modified_controller 10
  >>= fun after ->
  Machine.parse_program Sec_5_5.fib_controller
  >>= fun before_program ->
  Machine.parse_program fib_modified_controller
  >>= fun after_program ->
  counts before_program 6
  >>= fun before_counts ->
  counts after_program 6
  >>= fun after_counts ->
  Ok
    [ Machine.value_to_string before
    ; Machine.value_to_string after
    ; before_counts
    ; after_counts
    ]
;;
