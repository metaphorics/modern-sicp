(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.3: the square-root machine in two stages -- first with
    [good-enough?] and [improve] as primitives, then expanded into
    arithmetic operations. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_1

(** The stage-1 controller: [good-enough?] and [improve] are primitives
    of the operations table, exactly as the statement's first ask. *)
let sqrt_stage1_controller =
  {|(controller
 sqrt-loop
   (test (op good-enough?) (reg guess) (reg x))
   (branch (label sqrt-done))
   (assign guess (op improve) (reg guess) (reg x))
   (goto (label sqrt-loop))
 sqrt-done
   (perform (op print) (reg guess)))|}
;;

(** The stage-2 controller: good-enough? and improve are expanded into
    the arithmetic operations. Two temporaries carry the intermediate
    values, since an operation's inputs are registers and constants
    only. *)
let sqrt_stage2_controller =
  {|(controller
 sqrt-loop
   (assign t (op *) (reg guess) (reg guess))
   (assign t (op -) (reg t) (reg x))
   (assign t (op abs) (reg t))
   (test (op <) (reg t) (const 0.001))
   (branch (label sqrt-done))
   (assign t (op /) (reg x) (reg guess))
   (assign u (op +) (reg t) (reg guess))
   (assign u (op /) (reg u) (const 2))
   (assign guess (reg u))
   (goto (label sqrt-loop))
 sqrt-done
   (perform (op print) (reg guess)))|}
;;

let good_enough_operation =
  ( "good-enough?"
  , Machine.Value_op
      (function
        | [ Machine.Float g; Machine.Float x ] ->
          Ok (Machine.Bool (Float.abs ((g *. g) -. x) < 0.001))
        | _ -> Error (Machine.Arity "good-enough? needs two floats")) )
;;

let improve_operation =
  ( "improve"
  , Machine.Value_op
      (function
        | [ Machine.Float g; Machine.Float x ] ->
          Ok (Machine.Float ((g +. (x /. g)) /. 2.0))
        | _ -> Error (Machine.Arity "improve needs two floats")) )
;;

(** [run_sqrt controller operations x] starts guess at 1.0 and reads the
    printed transcript the machine leaves at [sqrt-done]. *)
let run_sqrt controller operations x =
  let q = Queue.create () in
  let out = ref [] in
  Machine.make_machine
    ~registers:[ "guess"; "x"; "t"; "u" ]
    ~operations:(operations @ Machine.read_print ~inputs:q ~output:out)
    ~controller
  >>= fun m ->
  Machine.set_register m "guess" (Machine.Float 1.0)
  >>= fun () ->
  Machine.set_register m "x" (Machine.Float x)
  >>= fun () -> Machine.start m >>= fun () -> Ok (String.concat " " !out)
;;

(** [ex_5_03 ()] runs both stages on 2 and 9; the transcript of each
    stage is the same, which is the point of the two-stage design. *)
let ex_5_03 () =
  let stage1 = [ good_enough_operation; improve_operation ] in
  run_sqrt sqrt_stage1_controller stage1 2.0
  >>= fun a2 ->
  run_sqrt sqrt_stage1_controller stage1 9.0
  >>= fun a9 ->
  run_sqrt sqrt_stage2_controller Machine.arith_operations 2.0
  >>= fun b2 ->
  run_sqrt sqrt_stage2_controller Machine.arith_operations 9.0
  >>= fun b9 -> Ok [ a2; a9; b2; b9 ]
;;
