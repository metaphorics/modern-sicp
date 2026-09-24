(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.11 *)

(** Exercise 4.11: the environment operations over the alternative frame
    representation. A frame is one mutable association list of
    name-value pairs instead of the section's pair of lists; an
    environment is the list of frames, newest first. The operations obey
    the section's contract unchanged: [extend_environment] arity-checks
    the two lists, [lookup_variable_value] walks frames newest first,
    [set_variable_value_] rebinds the nearest binding, and
    [define_variable_] binds in the newest frame. *)

module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** The environment of this representation: frames as association lists
    of name-value pairs, newest first; a frame is mutable the way the
    section's frames are. *)
type env = (string * Value.t) list ref list

(** [global_environment ()] is one empty frame, the base environment the
    demonstrations bootstrap into. *)
let global_environment () : env = [ ref [] ]

(** [extend_environment names values base_env] is [base_env] with one
    fresh frame binding each name to the value at the same position, or
    an arity error when the lists differ. *)
let extend_environment names values base_env =
  if List.length names = List.length values
  then Ok (ref (List.combine names values) :: base_env)
  else
    Error
      (Eval_error.Arity_mismatch
         { expected = List.length names; given = List.length values })
;;

(** [lookup_variable_value name env] is the value of the nearest binding
    of [name], or [Error (Unbound_variable name)] when no frame binds
    it. *)
let rec lookup_variable_value name env =
  match env with
  | [] -> Error (Eval_error.Unbound_variable name)
  | frame :: outer ->
    (match List.assoc_opt name !frame with
     | Some value -> Ok value
     | None -> lookup_variable_value name outer)
;;

(** [set_variable_value_ name value env] rebinds the nearest binding of
    [name] in place, or answers [Error (Unbound_variable name)] when no
    frame binds it. *)
let rec set_variable_value_ name value env =
  match env with
  | [] -> Error (Eval_error.Unbound_variable name)
  | frame :: outer ->
    if List.mem_assoc name !frame
    then (
      frame
      := List.map (fun (n, v) -> if String.equal n name then n, value else n, v) !frame;
      Ok ())
    else set_variable_value_ name value outer
;;

(** [define_variable_ name value env] binds [name] in the newest frame,
    replacing any binding it already carries. *)
let define_variable_ name value env =
  match env with
  | [] -> Error (Eval_error.Invalid_form "define_variable_: the environment has no frame")
  | frame :: _ ->
    frame := (name, value) :: List.remove_assoc name !frame;
    Ok ()
;;

(** [render r] is the printed outcome of one demonstration step. *)
let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_11 ()] demonstrates the four operations over the alternative
    representation: a shadowing lookup, a set of the nearest binding
    seen through lookup, a define creating a fresh binding, and the
    unbound-variable error. *)
let ex_4_11 () =
  let env0 = global_environment () in
  let (_ : (unit, Eval_error.t) result) = define_variable_ "x" (Value.int 1) env0 in
  match extend_environment [ "x" ] [ Value.int 10 ] env0 with
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
  | Ok env ->
    let shadowing = render (lookup_variable_value "x" env) in
    let (_ : (unit, Eval_error.t) result) = set_variable_value_ "x" (Value.int 42) env in
    let after_set = render (lookup_variable_value "x" env) in
    let (_ : (unit, Eval_error.t) result) = define_variable_ "y" (Value.int 7) env in
    let fresh = render (lookup_variable_value "y" env) in
    [ shadowing; after_set; fresh; render (lookup_variable_value "z" env) ]
;;
