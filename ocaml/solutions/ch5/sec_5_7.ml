(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Machine = Sicp_ch5.Sec_5_2

let rec load m = function
  | [] -> Ok ()
  | (name, v) :: rest ->
    let* () = Machine.set_register m name v in
    load m rest
;;

let run controller ~registers ~inputs ~result =
  let* m = Machine.make_machine ~registers ~operations:M.arith_operations ~controller in
  let* () = load m inputs in
  let* () = Machine.start m in
  Machine.get_register m result
;;

let ex_5_07 () =
  let recursive b n =
    run
      Sec_5_4.expt_recursive_controller
      ~registers:[ "b"; "n"; "val"; "continue" ]
      ~inputs:[ "b", M.Int b; "n", M.Int n ]
      ~result:"val"
  in
  let iterative b n =
    run
      Sec_5_4.expt_iterative_controller
      ~registers:[ "b"; "counter"; "product" ]
      ~inputs:[ "b", M.Int b; "product", M.Int 1; "counter", M.Int n ]
      ~result:"product"
  in
  let* r1 = recursive 2 10 in
  let* r2 = recursive 3 5 in
  let* r3 = iterative 2 10 in
  let* r4 = iterative 3 5 in
  Ok (List.map M.value_to_string [ r1; r2; r3; r4 ])
;;

(* Exercise 5.1's machine with its registers initialized by the
   controller, so the run needs [n] alone. *)
let factorial_iterative_controller =
  M.Assign ("product", M.Const (M.Int 1))
  :: M.Assign ("counter", M.Const (M.Int 1))
  :: Sec_5_1.factorial_iterative_controller
;;

let rec host_fib = function
  | 0 -> 0
  | 1 -> 1
  | n -> host_fib (n - 1) + host_fib (n - 2)
;;

let host_factorial n =
  let rec loop product counter =
    if counter > n then product else loop (counter * product) (counter + 1)
  in
  loop 1 1
;;

let report machine direct =
  let verdict = if String.equal machine direct then "ok" else "MISMATCH" in
  machine ^ " (direct " ^ direct ^ ") " ^ verdict
;;

let oracle_line n =
  let* fib_v =
    run
      Sec_5_5.fib_controller
      ~registers:[ "n"; "val"; "continue" ]
      ~inputs:[ "n", M.Int n ]
      ~result:"val"
  in
  let* fact_v =
    run
      factorial_iterative_controller
      ~registers:[ "n"; "product"; "counter" ]
      ~inputs:[ "n", M.Int n ]
      ~result:"product"
  in
  Ok
    (Printf.sprintf
       "n=%d: fib %s; factorial %s"
       n
       (report (M.value_to_string fib_v) (string_of_int (host_fib n)))
       (report (M.value_to_string fact_v) (string_of_int (host_factorial n))))
;;

let ex_5_07a () =
  let rec over = function
    | [] -> Ok []
    | n :: ns ->
      let* line = oracle_line n in
      let* rest = over ns in
      Ok (line :: rest)
  in
  over [ 0; 1; 2; 3; 4; 5; 6 ]
;;
