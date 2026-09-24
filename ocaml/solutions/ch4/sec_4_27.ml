(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.27: lazy [id] with [set!]. The outer call applies [id]
    immediately, so its [set!] runs and [count] answers 1, while the
    operand [(id 10)] stays a thunk; the definition binds [w] to that
    thunk, the driver's forcing walks it and answers 10, and the walk
    runs the inner [set!] exactly once, leaving [count] at 2. *)

let render = function
  | Ok v -> Sicp_common.Value.to_string v
  | Error e -> "Error: " ^ Sicp_common.Eval_error.to_string e
;;

let run env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Sicp_ch4.Sec_4_2.actual_value exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

(** [ex_4_27 ()] answers the interaction's sequence, with [w] and
    [count] displayed a second time to show that the memoized thunk
    re-forces without further effect. The runs are bound in order, one
    [let] each, because a list literal evaluates its elements right to
    left. *)
let ex_4_27 () =
  let env = Sicp_ch4.Sec_4_2.the_global_environment () in
  let r1 = run env "(define count 0)" in
  let r2 = run env "(define (id x) (set! count (+ count 1)) x)" in
  let r3 = run env "(define w (id (id 10)))" in
  let r4 = run env "count" in
  let r5 = run env "w" in
  let r6 = run env "count" in
  let r7 = run env "w" in
  let r8 = run env "count" in
  [ r1; r2; r3; r4; r5; r6; r7; r8 ]
;;
