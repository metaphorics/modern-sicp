(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Eval_error = Sicp_common.Eval_error

let rec all f = function
  | [] -> Ok []
  | x :: xs ->
    let* y = f x in
    let* ys = all f xs in
    Ok (y :: ys)
;;

let factorial_iterative_controller =
  M.
    [ Label "fact-loop"
    ; Test (">", [ Reg "counter"; Reg "n" ])
    ; Branch "fact-done"
    ; Assign_op ("product", "*", [ Reg "counter"; Reg "product" ])
    ; Assign_op ("counter", "+", [ Reg "counter"; Const (Int 1) ])
    ; Goto "fact-loop"
    ; Label "fact-done"
    ]
;;

let ex_5_01 () =
  let* answers =
    all
      (fun n ->
         M.run
           ~registers:[ "n"; "product"; "counter" ]
           ~operations:M.arith_operations
           ~inputs:[ "n", M.Int n; "product", M.Int 1; "counter", M.Int 1 ]
           ~controller:factorial_iterative_controller
           "product")
      [ 0; 1; 5; 10 ]
  in
  Ok (List.map M.value_to_string answers)
;;

let arity expected args =
  Error (Eval_error.Arity_mismatch { expected; given = List.length args })
;;

(* [read] pops the next input word; an exhausted queue is the typed
   failure that stops a driver loop the sentinel never reached. *)
let read_operation inputs =
  ( "read"
  , M.Value_op
      (function
        | [] ->
          (match Queue.take_opt inputs with
           | Some v -> Ok v
           | None -> Error (Eval_error.User_error "read: the input is exhausted"))
        | args -> arity 0 args) )
;;

let end_test =
  ( "end?"
  , M.Test_op
      (function
        | [ M.Str "end" ] -> Ok true
        | [ _ ] -> Ok false
        | args -> arity 1 args) )
;;

let factorial_driver_controller =
  M.
    [ Label "factorial-loop"
    ; Assign_op ("n", "read", [])
    ; Test ("end?", [ Reg "n" ])
    ; Branch "machine-done"
    ; Assign ("product", Const (Int 1))
    ; Assign ("counter", Const (Int 1))
    ; Label "fact-loop"
    ; Test (">", [ Reg "counter"; Reg "n" ])
    ; Branch "fact-done"
    ; Assign_op ("product", "*", [ Reg "counter"; Reg "product" ])
    ; Assign_op ("counter", "+", [ Reg "counter"; Const (Int 1) ])
    ; Goto "fact-loop"
    ; Label "fact-done"
    ; Perform ("print", [ Reg "product" ])
    ; Goto "factorial-loop"
    ; Label "machine-done"
    ]
;;

let drive_factorial inputs =
  let queue = Queue.of_seq (List.to_seq inputs) in
  let printed = Buffer.create 64 in
  let outcome =
    let* m =
      M.make_machine
        ~registers:[ "n"; "product"; "counter" ]
        ~operations:
          ((read_operation queue :: end_test :: M.arith_operations)
           @ [ M.print_operation (Buffer.add_string printed) ])
        ~controller:factorial_driver_controller
    in
    M.start m
  in
  let stop =
    match outcome with
    | Ok () -> "end"
    | Error e -> "Error: " ^ Eval_error.to_string e
  in
  let lines =
    String.split_on_char '\n' (Buffer.contents printed)
    |> List.filter (fun line -> not (String.equal line ""))
  in
  lines @ [ stop ]
;;

let ex_5_01a () =
  Ok
    (drive_factorial [ M.Int 5; M.Int 6; M.Str "end" ]
     @ drive_factorial [ M.Int 1; M.Int 10; M.Str "end" ]
     @ drive_factorial [ M.Int 5 ])
;;
