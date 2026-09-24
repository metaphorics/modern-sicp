(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.12 *)

(** Exercise 4.12: the three environment operations expressed in terms of
    one abstract traversal. [traverse] walks the frames newest first and
    answers the first frame inspection that binds the name; lookup
    inspects with [assoc], set inspects with a rewrite that mutates the
    frame in place, and define writes the newest frame through the same
    frame abstraction. The frame representation is the association list
    of 4.11. *)

module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** The environment of this representation: frames as association lists
    of name-value pairs, newest first. *)
type env = (string * Value.t) list ref list

(** [global_environment ()] is one empty frame, the base environment the
    demonstrations bootstrap into. *)
let global_environment () : env = [ ref [] ]

(** [walk frames inspect] is the one traversal: it applies [inspect] to
    each frame's association list, newest first, and answers the first
    [Some]. The inspect closure receives the frame itself, so an
    operation that rebinds can mutate in place as the traversal passes. *)
let rec walk frames inspect =
  match frames with
  | [] -> None
  | frame :: outer ->
    (match inspect frame !frame with
     | Some value -> Some value
     | None -> walk outer inspect)
;;

(** [traverse env name inspect_frame] walks the frames newest first and
      answers the first inspection that binds [name], or [None] when no
      frame does. *)
let traverse env _name inspect_frame = walk env (fun _frame -> inspect_frame)

(** [lookup_variable_value name env] is the value of the nearest binding
    of [name], the traversal inspected with [assoc]. *)
let lookup_variable_value name env =
  match traverse env name (List.assoc_opt name) with
  | Some value -> Ok value
  | None -> Error (Eval_error.Unbound_variable name)
;;

(** [set_variable_value_ name value env] rebinds the nearest binding of
    [name]: the traversal's inspection rewrites the first frame that
    carries the name and answers the value, so the walk stops there. *)
let set_variable_value_ name value env =
  match
    walk env (fun frame bindings ->
      if List.mem_assoc name bindings
      then (
        frame
        := List.map
             (fun (n, v) -> if String.equal n name then n, value else n, v)
             bindings;
        Some value)
      else None)
  with
  | Some _ -> Ok ()
  | None -> Error (Eval_error.Unbound_variable name)
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

(** [extend names values env] is [env] with one fresh frame, the demo
    helper standing for the evaluator's application extension. *)
let extend names values env = ref (List.combine names values) :: env

(** [render r] is the printed outcome of one demonstration step. *)
let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_12 ()] demonstrates the three operations sharing the one
    traversal: a shadowing lookup, a set of the nearest binding seen
    through lookup, a define creating a fresh binding, a set that
    reaches a binding in an outer frame past the newest one, and the
    unbound-variable error. *)
let ex_4_12 () =
  let env0 = global_environment () in
  let (_ : (unit, Eval_error.t) result) = define_variable_ "x" (Value.int 1) env0 in
  let env = extend [ "x" ] [ Value.int 10 ] env0 in
  let shadowing = render (lookup_variable_value "x" env) in
  let (_ : (unit, Eval_error.t) result) = set_variable_value_ "x" (Value.int 42) env in
  let after_set = render (lookup_variable_value "x" env) in
  let (_ : (unit, Eval_error.t) result) = define_variable_ "y" (Value.int 7) env in
  let fresh = render (lookup_variable_value "y" env) in
  let outer = extend [] [] env in
  let (_ : (unit, Eval_error.t) result) = set_variable_value_ "y" (Value.int 99) outer in
  let outer_set = render (lookup_variable_value "y" outer) in
  [ shadowing; after_set; fresh; outer_set; render (lookup_variable_value "z" outer) ]
;;
