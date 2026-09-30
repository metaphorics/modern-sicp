(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Machine = Sicp_ch5.Sec_5_2

let derive_registers controller =
  List.fold_left
    (fun seen inst ->
       List.fold_left
         (fun seen r -> if List.mem r seen then seen else seen @ [ r ])
         seen
         (M.instruction_registers inst))
    []
    controller
;;

let make_controller_machine ~operations ~controller =
  Machine.make_machine ~registers:(derive_registers controller) ~operations ~controller
;;

let run ~controller ~inputs result =
  let* m = make_controller_machine ~operations:M.arith_operations ~controller in
  let* () =
    List.fold_left
      (fun acc (r, v) ->
         let* () = acc in
         Machine.set_register m r v)
      (Ok ())
      inputs
  in
  let* () = Machine.start m in
  Machine.get_register m result
;;

let ex_5_13 () =
  let* gcd =
    run ~controller:Sec_5_10.gcd_controller ~inputs:[ "a", M.Int 206; "b", M.Int 40 ] "a"
  in
  let* fib = run ~controller:Sec_5_5.fib_controller ~inputs:[ "n", M.Int 6 ] "val" in
  Ok [ "gcd 206 40 = " ^ M.value_to_string gcd; "fib 6 = " ^ M.value_to_string fib ]
;;
