(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.13 *)

let ( let* ) = Result.bind

module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

type frame = { mutable bindings : (string * Value.t ref) list }
type env = frame list

let empty = []
let extend bindings env = { bindings = List.map (fun (n, v) -> n, ref v) bindings } :: env

let rec lookup_variable_value name = function
  | [] -> Error (Eval_error.Unbound_variable name)
  | frame :: enclosing ->
    (match List.assoc_opt name frame.bindings with
     | Some cell -> Ok !cell
     | None -> lookup_variable_value name enclosing)
;;

let define_variable name value = function
  | [] ->
    Error (Eval_error.Invalid_form "define_variable: the empty environment has no frame")
  | frame :: _ ->
    frame.bindings <- (name, ref value) :: List.remove_assoc name frame.bindings;
    Ok ()
;;

let make_unbound name = function
  | [] -> Error (Eval_error.Unbound_variable name)
  | frame :: _ ->
    if List.mem_assoc name frame.bindings
    then (
      frame.bindings <- List.remove_assoc name frame.bindings;
      Ok ())
    else Error (Eval_error.Unbound_variable name)
;;

let ex_4_13 () =
  let show = function
    | Ok v -> Value.to_string v
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  let unbind name env =
    match make_unbound name env with
    | Ok () -> "unbound " ^ name
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  let global = extend [ "x", Value.int 1; "y", Value.int 2 ] empty in
  let first_call = extend [ "x", Value.int 10 ] global in
  let second_call = extend [ "z", Value.int 30 ] global in
  let* () = define_variable "w" (Value.int 40) first_call in
  let removed_inner = unbind "x" first_call in
  let removed_outer = unbind "y" first_call in
  let removed_defined = unbind "w" first_call in
  Ok
    [ removed_inner
    ; show (lookup_variable_value "x" first_call)
    ; removed_outer
    ; show (lookup_variable_value "y" second_call)
    ; removed_defined
    ; show (lookup_variable_value "w" first_call)
    ]
;;
