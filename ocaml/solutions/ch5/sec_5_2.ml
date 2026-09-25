(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.2: the 5.1 machine described in the register-machine
    language as data, assembled, and run. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_1

(** The controller of Exercise 5.1, now as the machine-language
    description the exercise asks for. *)
let factorial_iterative_controller =
  {|(controller
 fact-loop
   (test (op >) (reg counter) (reg n))
   (branch (label fact-done))
   (assign product (op *) (reg counter) (reg product))
   (assign counter (op +) (reg counter) (const 1))
   (goto (label fact-loop))
 fact-done)|}
;;

(** [ex_5_02 ()] treats the 5.1 machine as data the way the exercise
    asks: the controller text is assembled -- 5 instructions, 2 labels,
    with [fact-done] naming the stop address -- and the assembled
    machine runs on 5 and 6. *)
let ex_5_02 () =
  Machine.parse_program factorial_iterative_controller
  >>= fun program ->
  let instructions = Array.length program.code in
  let labels = List.length program.labels in
  let label_table =
    String.concat " " (List.map (fun (l, i) -> l ^ "=" ^ string_of_int i) program.labels)
  in
  let run n =
    Machine.make_machine
      ~registers:[ "n"; "product"; "counter" ]
      ~operations:Machine.arith_operations
      ~controller:factorial_iterative_controller
    >>= fun m ->
    Machine.set_register m "n" (Machine.Int n)
    >>= fun () ->
    Machine.set_register m "product" (Machine.Int 1)
    >>= fun () ->
    Machine.set_register m "counter" (Machine.Int 1)
    >>= fun () -> Machine.start m >>= fun () -> Machine.get_register m "product"
  in
  run 5
  >>= fun five ->
  run 6
  >>= fun six ->
  Ok
    [ Printf.sprintf "%d instructions, %d labels" instructions labels
    ; label_table
    ; Machine.value_to_string five
    ; Machine.value_to_string six
    ]
;;
