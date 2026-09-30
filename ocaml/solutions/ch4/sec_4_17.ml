(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.17 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

type strategy =
  | Scanned
  | Shared_frame

(* Frames are counted where they are made: one per [let] node and one
   per call that binds arguments, counted at the application site from
   the closure's parameter list. *)
type counter = { mutable frames : int }

let apply_counted counter (self : S.eval_t) procedure arguments =
  (match Value.view procedure with
   | Value.Closure { parameters; _ } ->
     let consumed = Int.min (List.length arguments) (List.length parameters) in
     counter.frames <- counter.frames + consumed
   | _ -> ());
  S.apply_with ~self procedure arguments
;;

let rec fill_in (self : S.eval_t) env cells bindings =
  match cells, bindings with
  | cell :: cells, (b : Ast.binding) :: bindings ->
    let* v = self b.rhs env in
    Env.fill cell v;
    fill_in self env cells bindings
  | _, _ -> Ok ()
;;

(* The simultaneous scope without the extra frame: the parameter and
   every internal name get their cells in the call's one frame before
   any right-hand side runs. *)
let shared_call counter (self : S.eval_t) procedure argument =
  match Value.view procedure with
  | Value.Closure { parameters = [ parameter ]; body; env; _ } ->
    (match Ast.view body with
     | Ast.Let (true, bindings, inner) ->
       let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
       let env, cells = Env.extend_recursive (parameter :: names) env in
       counter.frames <- counter.frames + 1;
       (match cells with
        | parameter_cell :: cells ->
          Env.fill parameter_cell argument;
          let* () = fill_in self env cells bindings in
          self inner env
        | [] -> Error (Eval_error.Invalid_form "shared_call: no parameter cell"))
     | _ -> apply_counted counter self procedure [ argument ])
  | _ -> apply_counted counter self procedure [ argument ]
;;

let evaluator strategy counter =
  let rec self e env =
    match strategy, Ast.view e with
    | _, Ast.Let _ ->
      counter.frames <- counter.frames + 1;
      Sec_4_16.scanning Fun.id ~self e env
    | Scanned, Ast.Fun (parameters, body) ->
      S.open_eval
        ~self
        (Ast.fun_ ~at:(Ast.at e) parameters (Sec_4_16.scan_out_defines body))
        env
    | Shared_frame, Ast.Apply (operator, [ operand ]) ->
      let* procedure = self operator env in
      let* argument = self operand env in
      shared_call counter self procedure argument
    | _, Ast.Apply (operator, operands) ->
      let* procedure = self operator env in
      let* arguments = Sec_4_1.list_of_values_left_to_right self operands env in
      apply_counted counter self procedure arguments
    | _ -> Sec_4_16.scanning Fun.id ~self e env
  in
  self
;;

let measure strategy source =
  let* e = Sec_4_1.open_expression [] source in
  let counter = { frames = 0 } in
  let* v = evaluator strategy counter e (S.the_global_environment ()) in
  Ok (v, counter.frames)
;;

let program =
  "let f x = let rec double y = y + y and square y = y * y in square (double x) in f 3"
;;

let ex_4_17 () =
  let line name strategy =
    match measure strategy program with
    | Ok (v, frames) ->
      Printf.sprintf "%s: %s in %d frames" name (Value.to_string v) frames
    | Error err -> name ^ ": error: " ^ Eval_error.to_string err
  in
  [ line "scanned" Scanned; line "shared frame" Shared_frame ]
;;
