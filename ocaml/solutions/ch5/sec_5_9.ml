(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.9: machine operations take registers and constants only;
    a label is not an operand. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2

(** [ex_5_09 ()] contrasts the two assemblies: an operation fed from
    registers and constants assembles and runs, and the same
    controller with a label in an operand position is refused by the
    instruction grammar before any execution procedure is built. *)
let ex_5_09 () =
  let legal_controller =
    {|(controller
   (assign a (op +) (reg b) (reg c)))|}
  in
  let label_operand_controller =
    {|(controller
   (assign a (op +) (reg b) (label there)))|}
  in
  let outcome controller =
    Machine.make_machine
      ~registers:[ "a"; "b"; "c" ]
      ~operations:Machine.arith_operations
      ~controller
    |> function
    | Ok m ->
      (match
         Machine.set_register m "b" (Machine.Int 2)
         >>= fun () -> Machine.set_register m "c" (Machine.Int 3)
       with
       | Error e -> "Error: " ^ Machine.error_to_string e
       | Ok () ->
         (match Machine.start m >>= fun () -> Machine.get_register m "a" with
          | Ok v -> Machine.value_to_string v
          | Error e -> "Error: " ^ Machine.error_to_string e))
    | Error e -> "Error: " ^ Machine.error_to_string e
  in
  Ok [ outcome legal_controller; outcome label_operand_controller ]
;;
