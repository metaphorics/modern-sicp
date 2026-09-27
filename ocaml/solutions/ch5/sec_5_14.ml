(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.14: the monitored stack measures the factorial machine,
    and the measurements give the formulas. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2

(** The Figure 5.11 machine augmented for measurement: the controller
    clears the stack before the recursion and prints its statistics
    after it, through the two stack operations every machine carries. *)
let factorial_measured_controller =
  {|(controller
   (perform (op initialize-stack))
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
 fact-done
   (perform (op print-stack-statistics)))|}
;;

(** [measure n] runs the measured machine on [n] and reports the stack
    statistics line the machine itself printed. *)
let measure n =
  Machine.make_machine
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller:factorial_measured_controller
  >>= fun m ->
  Machine.set_register m "n" (Machine.Int n)
  >>= fun () ->
  Machine.start m
  >>= fun () ->
  match Machine.transcript m with
  | line :: _ -> Ok ("n = " ^ string_of_int n ^ ": " ^ line)
  | [] -> Ok ("n = " ^ string_of_int n ^ ": no statistics printed")
;;

(** [ex_5_14 ()] measures the machine for n = 1..7 and states the
    formulas the data determine: both counters are 2n - 2 for n > 1. *)
let ex_5_14 () =
  let rec over = function
    | [] -> Ok []
    | n :: ns -> measure n >>= fun line -> over ns >>= fun rest -> Ok (line :: rest)
  in
  over [ 1; 2; 3; 4; 5; 6; 7 ]
  >>= fun lines ->
  Ok
    (lines
     @ [ "total-pushes(n) = 2n - 2 and maximum-depth(n) = 2n - 2 for n > 1:"
         ^ " two pushes per recursive level, n - 1 levels, no pops before the base case"
       ])
;;
