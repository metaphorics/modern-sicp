(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.13 *)

(** Exercise 4.13: [make-unbound!]. The specification decision: the
    binding leaves the newest frame only, so no enclosing binding is
    ever destroyed and the previous binding of the name, if any, becomes
    visible again -- the useful reading of unbinding in a lexically
    scoped language. Removing from every frame would punch holes through
    enclosing scopes. [make_unbound_] answers whether it removed one.
    The frame representation is the association list of 4.11. *)

module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** The environment of this representation: frames as association lists
    of name-value pairs, newest first. *)
type env = (string * Value.t) list ref list

(** [global_environment ()] is one empty frame, the base environment the
    demonstrations bootstrap into. *)
let global_environment () : env = [ ref [] ]

(** [make_unbound_ name env] removes the binding of [name] from the
    newest frame only and answers whether it removed one. *)
let make_unbound_ name env =
  match env with
  | [] -> Ok false
  | frame :: _ ->
    if List.mem_assoc name !frame
    then (
      frame := List.remove_assoc name !frame;
      Ok true)
    else Ok false
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

(** [render_removed r] is the printed outcome of one [make_unbound_]
    step. *)
let render_removed = function
  | Ok removed -> Value.to_string (Value.bool removed)
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_13 ()] demonstrates the specification decision: two defines in
    one frame leave a single binding that [make-unbound!] removes for
    good; over a shadowing frame the removal uncovers the outer binding;
    and removing a name the newest frame never bound answers false. *)
let ex_4_13 () =
  let env0 = global_environment () in
  let (_ : (unit, Eval_error.t) result) = define_variable_ "x" (Value.int 1) env0 in
  let (_ : (unit, Eval_error.t) result) = define_variable_ "x" (Value.int 2) env0 in
  let replaced = render (lookup_variable_value "x" env0) in
  let removed = render_removed (make_unbound_ "x" env0) in
  let gone = render (lookup_variable_value "x" env0) in
  let env1 = extend [ "x" ] [ Value.int 9 ] (global_environment ()) in
  let (_ : (unit, Eval_error.t) result) = define_variable_ "x" (Value.int 1) env1 in
  let shadowed = render (lookup_variable_value "x" env1) in
  let uncovered = render_removed (make_unbound_ "x" env1) in
  let outer_visible = render (lookup_variable_value "x" env1) in
  let unbound_newest = render_removed (make_unbound_ "q" env1) in
  [ replaced; removed; gone; shadowed; uncovered; outer_visible; unbound_newest ]
;;
