(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.11 *)

let ( let* ) = Result.bind

module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

type frame = { mutable bindings : (string * Value.t ref) list }
type env = frame list

let empty = []

let extend_environment names values env =
  let given = List.length values in
  let expected = List.length names in
  if given <> expected
  then Error (Eval_error.Arity_mismatch { expected; given })
  else Ok ({ bindings = List.map2 (fun n v -> n, ref v) names values } :: env)
;;

let rec find_cell name = function
  | [] -> None
  | frame :: enclosing ->
    (match List.assoc_opt name frame.bindings with
     | Some cell -> Some cell
     | None -> find_cell name enclosing)
;;

let lookup_variable_value name env =
  match find_cell name env with
  | Some cell -> Ok !cell
  | None -> Error (Eval_error.Unbound_variable name)
;;

let set_variable_value name value env =
  match find_cell name env with
  | Some cell ->
    cell := value;
    Ok ()
  | None -> Error (Eval_error.Unbound_variable name)
;;

let define_variable name value = function
  | [] ->
    Error (Eval_error.Invalid_form "define_variable: the empty environment has no frame")
  | frame :: _ ->
    (match List.assoc_opt name frame.bindings with
     | Some cell -> cell := value
     | None -> frame.bindings <- (name, ref value) :: frame.bindings);
    Ok ()
;;

let frames env =
  List.map (fun frame -> List.map (fun (name, cell) -> name, !cell) frame.bindings) env
;;

let show_frames env =
  frames env
  |> List.map (fun frame ->
    "["
    ^ String.concat "; " (List.map (fun (n, v) -> n ^ " = " ^ Value.to_string v) frame)
    ^ "]")
  |> String.concat " -> "
;;

let ex_4_11 () =
  let show = function
    | Ok v -> Value.to_string v
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  let int = Value.int in
  let* outer = extend_environment [ "x"; "y" ] [ int 1; int 2 ] empty in
  let* () = define_variable "z" (int 3) outer in
  let* () = set_variable_value "x" (int 10) outer in
  let* inner = extend_environment [ "x" ] [ int 100 ] outer in
  let* () = set_variable_value "y" (int 20) inner in
  let arity =
    match extend_environment [ "a" ] [] outer with
    | Ok _ -> "extended"
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  Ok
    [ show_frames inner
    ; show (lookup_variable_value "x" inner)
    ; show (lookup_variable_value "x" outer)
    ; show (lookup_variable_value "y" outer)
    ; show (lookup_variable_value "w" inner)
    ; arity
    ]
;;
