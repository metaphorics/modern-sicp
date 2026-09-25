(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.13: the controller text, not a register list, tells the
    simulator which registers the machine has. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2

(** [derive_registers program] scans the parsed controller and names
    every register the instructions read or write, in first-use order,
    without duplicates. *)
let derive_registers (program : Machine.program) =
  let rec scan seen = function
    | [] -> List.rev seen
    | inst :: rest ->
      let fresh =
        Machine.instruction_registers inst |> List.filter (fun r -> not (List.mem r seen))
      in
      scan (fresh @ seen) rest
  in
  scan [] (Array.to_list program.code)
;;

(** [make_controller_machine ~operations controller] is the
    register-deriving constructor: no register list is supplied; the
    scan of the controller determines the machine's registers. *)
let make_controller_machine ~operations ~controller =
  Machine.parse_program controller
  >>= fun (program : Machine.program) ->
  Machine.make_machine_from_program
    ~registers:(derive_registers program)
    ~operations
    program
;;

(** [ex_5_13 ()] assembles the GCD and Fibonacci machines with no
    register list at all and runs them; the scan found every register
    the machines touch. *)
let ex_5_13 () =
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
  in
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
  in
  make_controller_machine ~operations:Machine.arith_operations ~controller:gcd_controller
  >>= fun gcd_machine ->
  Machine.set_register gcd_machine "a" (Machine.Int 206)
  >>= fun () ->
  Machine.set_register gcd_machine "b" (Machine.Int 40)
  >>= fun () ->
  Machine.start gcd_machine
  >>= fun () ->
  Machine.get_register gcd_machine "a"
  >>= fun gcd_answer ->
  make_controller_machine ~operations:Machine.arith_operations ~controller:fib_controller
  >>= fun fib_machine ->
  Machine.set_register fib_machine "n" (Machine.Int 6)
  >>= fun () ->
  Machine.start fib_machine
  >>= fun () ->
  Machine.get_register fib_machine "val"
  >>= fun fib_answer ->
  Ok
    [ "gcd 206 40 = " ^ Machine.value_to_string gcd_answer
    ; "fib 6 = " ^ Machine.value_to_string fib_answer
    ]
;;
