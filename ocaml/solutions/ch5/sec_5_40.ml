(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check

let ( let* ) = Result.bind

module Node = Hashtbl.Make (struct
    type t = Ast.expr

    let equal = ( == )
    let hash = Hashtbl.hash
  end)

type frames = string list list

type t =
  { table : frames Node.t
  ; mutable variables : (string * frames) list
  }

let rec pattern_variables p =
  match Ast.view_pattern p with
  | Ast.PWildcard | Ast.PScalar _ | Ast.PNil -> []
  | Ast.PVar x -> [ x ]
  | Ast.PTuple ps | Ast.PConstruct (_, ps) -> List.concat_map pattern_variables ps
  | Ast.PCons (h, tl) -> pattern_variables h @ pattern_variables tl
;;

let parameter_name (b : Ast.binding) = Option.value b.name ~default:"_"
let bound_names bindings = List.filter_map (fun (b : Ast.binding) -> b.name) bindings

let rec walk t ctenv e =
  Node.replace t.table e ctenv;
  let go = walk t ctenv in
  match Ast.view e with
  | Ast.Scalar _ | Ast.Nil -> ()
  | Ast.Var x -> t.variables <- (x, ctenv) :: t.variables
  | Ast.Fun (parameters, body) -> walk t (parameters :: ctenv) body
  | Ast.Let (false, bindings, body) ->
    List.iter (fun (b : Ast.binding) -> go b.rhs) bindings;
    walk t (List.map parameter_name bindings :: ctenv) body
  | Ast.Let (true, bindings, body) ->
    let inner = bound_names bindings :: ctenv in
    List.iter (fun (b : Ast.binding) -> walk t inner b.rhs) bindings;
    walk t inner body
  | Ast.Match (scrutinee, cases) ->
    go scrutinee;
    List.iter (fun (p, body) -> walk t (pattern_variables p :: ctenv) body) cases
  | Ast.Apply (f, args) ->
    go f;
    List.iter go args
  | Ast.If (a, b, c) ->
    go a;
    go b;
    go c
  | Ast.Tuple es | Ast.Construct (_, es) -> List.iter go es
  | Ast.Record fields -> List.iter (fun (_, e) -> go e) fields
  | Ast.Field (a, _) | Ast.Not a | Ast.Neg a | Ast.Deref a | Ast.Make_ref a -> go a
  | Ast.Sequence (a, b)
  | Ast.And (a, b)
  | Ast.Or (a, b)
  | Ast.Arith (_, a, b)
  | Ast.Compare (_, a, b)
  | Ast.Cons (a, b)
  | Ast.Concat (a, b)
  | Ast.Assign (a, b) ->
    go a;
    go b
;;

let environments items =
  let t = { table = Node.create 64; variables = [] } in
  let _ : frames =
    List.fold_left
      (fun ctenv item ->
         match item with
         | Ast.Type_item _ -> ctenv
         | Ast.Value_item (false, bindings) ->
           List.iter (fun (b : Ast.binding) -> walk t ctenv b.rhs) bindings;
           bound_names bindings :: ctenv
         | Ast.Value_item (true, bindings) ->
           let inner = bound_names bindings :: ctenv in
           List.iter (fun (b : Ast.binding) -> walk t inner b.rhs) bindings;
           inner)
      []
      items
  in
  t.variables <- List.rev t.variables;
  t
;;

let find t e = Node.find_opt t.table e
let variables t = t.variables

let frames_to_string frames =
  String.concat " " (List.map (fun f -> "[" ^ String.concat "; " f ^ "]") frames)
;;

let nested_example =
  "let result =\n\
  \  (fun x y -> fun a b c d e -> (fun y z -> x * y * z) (a * b * x) (c + d + x)) 3 4 1 \
   2 3 4 5\n"
;;

let ex_5_40 () =
  let* p = Sec_5_33.program ~filename:"ex_5_40.ml" nested_example in
  let t = environments (Check.items p) in
  Ok (List.map (fun (x, frames) -> x ^ " in " ^ frames_to_string frames) (variables t))
;;
