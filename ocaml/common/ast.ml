(* SPDX-License-Identifier: GPL-3.0-only *)

type position =
  { line : int
  ; column : int
  }

type span =
  { start : position
  ; stop : position
  }

type scalar =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Unit

type arith =
  | Add
  | Sub
  | Mul
  | Div
  | Rem
  | Addf
  | Subf
  | Mulf
  | Divf

type comparison =
  | Eq
  | Ne
  | Lt
  | Le
  | Gt
  | Ge

type expr =
  { at : span
  ; form : view
  }

and view =
  | Scalar of scalar
  | Var of string
  | Let of bool * binding list * expr
  | Fun of string list * expr
  | Apply of expr * expr list
  | If of expr * expr * expr
  | Match of expr * (pattern * expr) list
  | Tuple of expr list
  | Construct of string * expr list
  | Record of (string * expr) list
  | Field of expr * string
  | Sequence of expr * expr
  | And of expr * expr
  | Or of expr * expr
  | Arith of arith * expr * expr
  | Compare of comparison * expr * expr
  | Nil
  | Cons of expr * expr
  | Concat of expr * expr
  | Not of expr
  | Neg of expr
  | Deref of expr
  | Assign of expr * expr
  | Make_ref of expr

and pattern =
  { pat_at : span
  ; pat_form : pattern_view
  }

and pattern_view =
  | PWildcard
  | PVar of string
  | PScalar of scalar
  | PTuple of pattern list
  | PConstruct of string * pattern list
  | PNil
  | PCons of pattern * pattern

and binding =
  { name : string option
  ; rhs : expr
  }

type type_shape =
  { type_name : string
  ; type_params : string list
  ; constructors : (string * int) list
  ; fields : string list
  }

type item =
  | Type_item of type_shape
  | Value_item of bool * binding list

let nowhere = { start = { line = 0; column = 0 }; stop = { line = 0; column = 0 } }
let view e = e.form
let view_pattern p = p.pat_form
let at e = e.at
let pattern_span p = p.pat_at
let node ?(at = nowhere) form = { at; form }
let pattern_node ?(at = nowhere) pat_form = { pat_at = at; pat_form }
let scalar ?at s = node ?at (Scalar s)
let var ?at name = node ?at (Var name)
let let_ ?at rec_ bindings body = node ?at (Let (rec_, bindings, body))
let fun_ ?at params body = node ?at (Fun (params, body))
let apply ?at operator operands = node ?at (Apply (operator, operands))
let if_ ?at c t f = node ?at (If (c, t, f))
let match_ ?at scrutinee cases = node ?at (Match (scrutinee, cases))
let tuple ?at parts = node ?at (Tuple parts)
let construct ?at name payload = node ?at (Construct (name, payload))
let record ?at fields = node ?at (Record fields)
let field ?at r name = node ?at (Field (r, name))
let sequence ?at first second = node ?at (Sequence (first, second))
let and_ ?at l r = node ?at (And (l, r))
let or_ ?at l r = node ?at (Or (l, r))
let arith ?at op l r = node ?at (Arith (op, l, r))
let compare_ ?at op l r = node ?at (Compare (op, l, r))
let nil ?at () = node ?at Nil
let cons ?at h t = node ?at (Cons (h, t))
let concat ?at l r = node ?at (Concat (l, r))
let not_ ?at e = node ?at (Not e)
let neg ?at e = node ?at (Neg e)
let deref ?at e = node ?at (Deref e)
let assign ?at r e = node ?at (Assign (r, e))
let make_ref ?at e = node ?at (Make_ref e)
let wildcard ?at () = pattern_node ?at PWildcard
let pvar ?at name = pattern_node ?at (PVar name)
let pscalar ?at s = pattern_node ?at (PScalar s)
let ptuple ?at parts = pattern_node ?at (PTuple parts)
let pconstruct ?at name payload = pattern_node ?at (PConstruct (name, payload))
let pnil ?at () = pattern_node ?at PNil
let pcons ?at head tail = pattern_node ?at (PCons (head, tail))

(* Every child is mapped in source order by sequenced [let]s: constructor
   arguments evaluate in an unspecified order, so an effectful [f] must
   never sit in argument position.  Lists go through [map_list], a
   tail-recursive left-to-right map that makes no use of [List.map]'s
   order. *)
let map_children f e =
  let rec map_list g acc = function
    | [] -> List.rev acc
    | x :: rest ->
      let y = g x in
      map_list g (y :: acc) rest
  in
  let map_list g xs = map_list g [] xs in
  let pair a b =
    let a = f a in
    let b = f b in
    a, b
  in
  let form =
    match e.form with
    | (Scalar _ | Var _ | Nil) as leaf -> leaf
    | Let (r, bindings, body) ->
      let bindings = map_list (fun b -> { b with rhs = f b.rhs }) bindings in
      Let (r, bindings, f body)
    | Fun (params, body) -> Fun (params, f body)
    | Apply (fn, args) ->
      let fn = f fn in
      Apply (fn, map_list f args)
    | If (c, t, e) ->
      let c = f c in
      let t = f t in
      If (c, t, f e)
    | Match (s, cases) ->
      let s = f s in
      Match (s, map_list (fun (p, b) -> p, f b) cases)
    | Tuple parts -> Tuple (map_list f parts)
    | Construct (name, parts) -> Construct (name, map_list f parts)
    | Record fields -> Record (map_list (fun (n, x) -> n, f x) fields)
    | Field (r, name) -> Field (f r, name)
    | Sequence (a, b) ->
      let a, b = pair a b in
      Sequence (a, b)
    | And (a, b) ->
      let a, b = pair a b in
      And (a, b)
    | Or (a, b) ->
      let a, b = pair a b in
      Or (a, b)
    | Arith (op, a, b) ->
      let a, b = pair a b in
      Arith (op, a, b)
    | Compare (op, a, b) ->
      let a, b = pair a b in
      Compare (op, a, b)
    | Cons (a, b) ->
      let a, b = pair a b in
      Cons (a, b)
    | Concat (a, b) ->
      let a, b = pair a b in
      Concat (a, b)
    | Not a -> Not (f a)
    | Neg a -> Neg (f a)
    | Deref a -> Deref (f a)
    | Assign (a, b) ->
      let a, b = pair a b in
      Assign (a, b)
    | Make_ref a -> Make_ref (f a)
  in
  { e with form }
;;
