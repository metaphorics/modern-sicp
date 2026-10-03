(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.16 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

(* A constructor name with a space cannot be written in source, so no
   guest value can impersonate the marker. *)
let marker = "unassigned value"
let unassigned = Ast.construct marker []

let is_unassigned v =
  match Value.view v with
  | Value.Constructor (name, []) -> String.equal name marker
  | _ -> false
;;

let rec pattern_names p =
  match Ast.view_pattern p with
  | Ast.PWildcard | Ast.PScalar _ | Ast.PNil -> []
  | Ast.PVar name -> [ name ]
  | Ast.PTuple ps | Ast.PConstruct (_, ps) -> List.concat_map pattern_names ps
  | Ast.PCons (head, tail) -> pattern_names head @ pattern_names tail
;;

let without bound names = List.filter (fun n -> not (List.mem n bound)) names
let binding_names bindings = List.filter_map (fun (b : Ast.binding) -> b.name) bindings

let rec deref_names names e =
  if names = []
  then e
  else (
    let at = Ast.at e in
    match Ast.view e with
    | Ast.Var name when List.mem name names -> Ast.deref ~at (Ast.var ~at name)
    | Ast.Fun (parameters, body) ->
      Ast.fun_ ~at parameters (deref_names (without parameters names) body)
    | Ast.Let (is_rec, bindings, body) ->
      let bound = binding_names bindings in
      let inside = without bound names in
      let in_rhs = if is_rec then inside else names in
      Ast.let_
        ~at
        is_rec
        (List.map
           (fun (b : Ast.binding) -> { b with rhs = deref_names in_rhs b.rhs })
           bindings)
        (deref_names inside body)
    | Ast.Match (scrutinee, cases) ->
      Ast.match_
        ~at
        (deref_names names scrutinee)
        (List.map
           (fun (p, body) -> p, deref_names (without (pattern_names p) names) body)
           cases)
    | Ast.Assign (target, rhs) ->
      (match Ast.view target with
       | Ast.Var _ -> Ast.assign ~at target (deref_names names rhs)
       | _ -> Ast.map_children (deref_names names) e)
    | _ -> Ast.map_children (deref_names names) e)
;;

let cells names =
  List.map (fun n -> { Ast.name = Some n; rhs = Ast.make_ref unassigned }) names
;;

let scan_out_let_rec e =
  match Ast.view e with
  | Ast.Let (true, bindings, body) ->
    let names = binding_names bindings in
    let at = Ast.at e in
    let assignment (b : Ast.binding) =
      let rhs = deref_names names b.rhs in
      match b.name with
      | Some name -> Ast.assign ~at (Ast.var ~at name) rhs
      | None -> rhs
    in
    let body =
      List.fold_right
        (fun b rest -> Ast.sequence ~at (assignment b) rest)
        bindings
        (deref_names names body)
    in
    Ast.let_ ~at false (cells names) body
  | _ -> e
;;

let scan_out_defines body = scan_out_let_rec body

let scanning scan ~(self : S.eval_t) e env =
  match Ast.view e with
  | Ast.Fun (parameters, body) ->
    S.open_eval ~self (Ast.fun_ ~at:(Ast.at e) parameters (scan body)) env
  | Ast.Deref reference ->
    let* v = S.open_eval ~self e env in
    if is_unassigned v
    then (
      let name =
        match Ast.view reference with
        | Ast.Var name -> name
        | _ -> "reference"
      in
      Error (Eval_error.User_error ("unassigned variable " ^ name)))
    else Ok v
  | _ -> S.open_eval ~self e env
;;

let eval =
  let rec self e env = scanning scan_out_defines ~self e env in
  self
;;

let parity =
  "let f x = let rec even n = if n = 0 then true else odd (n - 1) and odd n = if n = 0 \
   then false else even (n - 1) in even x in f 10"
;;

(* OCaml admits a group of statically constructive values, such as a
   cyclic list, whose right-hand sides read the group's names at once. *)
let cyclic =
  "(fun u -> let rec xs = 1 :: ys and ys = 2 :: xs in match xs with a :: b :: c :: _ -> \
   a + b + c | _ -> 0) ()"
;;

let ex_4_16 () =
  let* f =
    Sec_4_1.open_expression
      []
      "fun x -> let rec even n = if n = 0 then true else odd (n - 1) and odd n = if n = \
       0 then false else even (n - 1) in even x"
  in
  let cells =
    match Ast.view f with
    | Ast.Fun (_, body) ->
      (match Ast.view (scan_out_defines body) with
       | Ast.Let (false, bindings, _) ->
         "cells: " ^ String.concat ", " (binding_names bindings)
       | _ -> "not scanned")
    | _ -> "not a procedure"
  in
  Ok
    [ Sec_4_1.run_source eval parity
    ; Sec_4_1.run_source S.eval_expr parity
    ; Sec_4_1.run_source eval cyclic
    ; Sec_4_1.run_source S.eval_expr cyclic
    ; Sec_4_1.run_source eval "(fun u -> let rec a = b + 1 and b = 2 in a) ()"
    ; cells
    ]
;;
