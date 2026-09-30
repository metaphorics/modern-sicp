(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Machine = Sicp_ch5.Sec_5_2

let factorial_measured_controller =
  (M.Perform ("initialize-stack", []) :: Sec_5_5.factorial_recursive_controller)
  @ [ M.Perform ("print-stack-statistics", []) ]
;;

let measure n =
  let* m =
    Machine.make_machine
      ~registers:[ "n"; "val"; "continue" ]
      ~operations:M.arith_operations
      ~controller:factorial_measured_controller
  in
  let* () = Machine.set_register m "n" (M.Int n) in
  let* () = Machine.start m in
  let printed = String.concat "; " (Machine.transcript m) in
  Ok ("n = " ^ string_of_int n ^ ": " ^ printed)
;;

let ex_5_14 () =
  let rec over = function
    | [] -> Ok []
    | n :: ns ->
      let* line = measure n in
      let* rest = over ns in
      Ok (line :: rest)
  in
  let* lines = over [ 1; 2; 3; 4; 5; 6; 7 ] in
  Ok
    (lines
     @ [ "total-pushes(n) = 2n - 2 and maximum-depth(n) = 2n - 2 for n > 1:"
         ^ " two pushes per recursive level, n - 1 levels, no pops before the base case"
       ])
;;
