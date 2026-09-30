(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.12 *)

let ( let* ) = Result.bind

module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

type frame =
  { mutable variables : string list
  ; mutable values : Value.t ref list
  }

type env = frame list

let empty = []

let extend_environment names values env =
  let given = List.length values in
  let expected = List.length names in
  if given <> expected
  then Error (Eval_error.Arity_mismatch { expected; given })
  else Ok ({ variables = names; values = List.map ref values } :: env)
;;

let find_in_frame name frame =
  let rec scan variables values =
    match variables, values with
    | variable :: _, cell :: _ when String.equal variable name -> Some cell
    | _ :: variables, _ :: values -> scan variables values
    | _, _ -> None
  in
  scan frame.variables frame.values
;;

let rec find_binding name = function
  | [] -> None
  | frame :: enclosing ->
    (match find_in_frame name frame with
     | Some cell -> Some cell
     | None -> find_binding name enclosing)
;;

let lookup_variable_value name env =
  match find_binding name env with
  | Some cell -> Ok !cell
  | None -> Error (Eval_error.Unbound_variable name)
;;

let set_variable_value name value env =
  match find_binding name env with
  | Some cell ->
    cell := value;
    Ok ()
  | None -> Error (Eval_error.Unbound_variable name)
;;

let define_variable name value = function
  | [] ->
    Error (Eval_error.Invalid_form "define_variable: the empty environment has no frame")
  | frame :: _ ->
    (match find_in_frame name frame with
     | Some cell -> cell := value
     | None ->
       frame.variables <- name :: frame.variables;
       frame.values <- ref value :: frame.values);
    Ok ()
;;

let ex_4_12 () =
  let show = function
    | Ok v -> Value.to_string v
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  let int = Value.int in
  let* outer = extend_environment [ "a"; "b" ] [ int 1; int 2 ] empty in
  let* inner = extend_environment [ "a" ] [ int 10 ] outer in
  let* () = define_variable "c" (int 3) inner in
  let* () = define_variable "a" (int 11) inner in
  let* () = set_variable_value "b" (int 20) inner in
  Ok
    [ show (lookup_variable_value "a" inner)
    ; show (lookup_variable_value "a" outer)
    ; show (lookup_variable_value "b" outer)
    ; show (lookup_variable_value "c" inner)
    ; show (lookup_variable_value "c" outer)
    ; show (Result.map (fun () -> Value.unit) (set_variable_value "d" (int 0) inner))
    ]
;;
