(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 4.3 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude
module Value = Sicp_common.Value
(* One deterministic run attempt against a decision list.  An [amb]
   consumes the next decision; an exhausted list suspends the attempt
   with its arity so the driver can extend the path. *)

type stop =
  | Failed
  | Incomplete of int
  | Broke of Eval_error.t

type 'a step = ('a, stop) result

(* The permanent store outlives every attempt: the survived value of
   each named top-level reference a permanent assignment wrote, and the
   assignments already applied, each keyed by the decisions taken
   before it and its ordinal among the attempt's permanent assignments
   at that point.  Attempts sharing a decision prefix run identically up
   to its end, so a key names one assignment of the search tree, and a
   re-executed prefix never applies it twice. *)
type store =
  { survived : (string, Value.t) Hashtbl.t
  ; applied : (int list * int, unit) Hashtbl.t
  }

type search =
  { mutable decisions : int list
  ; mutable taken : int list
  ; permanents : int ref
  ; mutable top : (string * Value.t) list
  ; store : store
  }

type eval_t = search -> Env.t -> Ast.expr -> Value.t step

let broke e = Error (Broke e)
let type_error detail = broke (Eval_error.Type_error detail)
let not_applicable v = broke (Eval_error.Not_applicable (Value.to_string v))
let unbound name = broke (Eval_error.Unbound_variable name)

let as_bool what v =
  match Value.view v with
  | Value.Bool b -> Ok b
  | _ -> type_error (what ^ ": condition is not a bool")
;;

let lift = function
  | Ok v -> Ok v
  | Error e -> broke e
;;

let fail = Error Failed

let choose search n =
  if n <= 0
  then type_error "a choice point needs at least one alternative"
  else (
    match search.decisions with
    | decision :: rest when decision >= 0 && decision < n ->
      search.decisions <- rest;
      search.taken <- decision :: search.taken;
      Ok decision
    | _ :: _ -> fail
    | [] -> Error (Incomplete n))
;;

let permanent_assign search name value =
  let key = search.taken, !(search.permanents) in
  incr search.permanents;
  match Option.map Value.view (List.assoc_opt name search.top) with
  | Some (Value.Ref cell) ->
    if not (Hashtbl.mem search.store.applied key)
    then (
      Hashtbl.replace search.store.applied key ();
      Hashtbl.replace search.store.survived name value;
      cell := value);
    Ok Value.unit
  | Some _ -> type_error (name ^ " is not a reference")
  | None -> type_error (name ^ " is not a top-level reference")
;;

let scalar_value = Sec_4_1.scalar_value
let arithmetic op left right = lift (Sec_4_1.arithmetic op left right)
let negate v = lift (Sec_4_1.negate v)
let comparison op left right = lift (Sec_4_1.comparison op left right)

let rec step self state env (e : Ast.expr) : Value.t step =
  match Ast.view e with
  | Ast.Scalar s -> Ok (scalar_value s)
  | Ast.Var name ->
    (match Env.find env name with
     | Some v -> Ok v
     | None -> unbound name)
  | Ast.Let (is_rec, bindings, body) -> eval_let self state env is_rec bindings body
  | Ast.Fun (parameters, body) -> Ok (Value.closure ~name:None ~parameters ~body ~env)
  | Ast.Apply (fn, args) -> apply_from_expr self state env fn args
  | Ast.If (condition, consequent, alternative) ->
    let* condition = self state env condition in
    let* b = as_bool "if" condition in
    if b then self state env consequent else self state env alternative
  | Ast.Match (scrutinee, cases) ->
    let* v = self state env scrutinee in
    eval_cases self state env v cases
  | Ast.Tuple parts ->
    let* parts = eval_list self state env parts in
    Ok (Value.tuple parts)
  | Ast.Construct (name, fields) ->
    let* fields = eval_list self state env fields in
    Ok (Value.construct name fields)
  | Ast.Record fields ->
    let rec gather acc = function
      | [] -> Ok (Value.record (List.rev acc))
      | (name, field) :: rest ->
        let* v = self state env field in
        gather ((name, v) :: acc) rest
    in
    gather [] fields
  | Ast.Field (record, name) ->
    let* record = self state env record in
    (match Value.view record with
     | Value.Record fields ->
       (match List.assoc_opt name fields with
        | Some v -> Ok v
        | None -> type_error ("record field " ^ name ^ " is absent"))
     | _ -> type_error "field access target is not a record")
  | Ast.Sequence (first, second) ->
    let* _ = self state env first in
    self state env second
  | Ast.And (left, right) ->
    let* left = self state env left in
    let* b = as_bool "&&" left in
    if not b then Ok (Value.bool false) else self state env right
  | Ast.Or (left, right) ->
    let* left = self state env left in
    let* b = as_bool "||" left in
    if b then Ok (Value.bool true) else self state env right
  | Ast.Arith (op, left, right) ->
    let* left = self state env left in
    let* right = self state env right in
    arithmetic op left right
  | Ast.Compare (op, left, right) ->
    let* left = self state env left in
    let* right = self state env right in
    comparison op left right
  | Ast.Nil -> Ok Value.nil
  | Ast.Cons (head, tail) ->
    let* head = self state env head in
    let* tail = self state env tail in
    Ok (Value.cons head tail)
  | Ast.Concat (left, right) ->
    let* left = self state env left in
    let* right = self state env right in
    (match Value.view left, Value.view right with
     | Value.String a, Value.String b -> Ok (Value.string (a ^ b))
     | _ -> type_error "^ operands are not strings")
  | Ast.Not operand ->
    let* operand = self state env operand in
    let* b = as_bool "not" operand in
    Ok (Value.bool (not b))
  | Ast.Neg operand ->
    let* operand = self state env operand in
    negate operand
  | Ast.Deref reference ->
    let* reference = self state env reference in
    (match Value.view reference with
     | Value.Ref cell -> Ok !cell
     | _ -> type_error "! target is not a reference")
  | Ast.Assign (reference, rhs) ->
    let* reference = self state env reference in
    let* value = self state env rhs in
    (match Value.view reference with
     | Value.Ref cell ->
       cell := value;
       Ok Value.unit
     | _ -> type_error ":= target is not a reference")
  | Ast.Make_ref operand ->
    let* operand = self state env operand in
    Ok (Value.ref_value operand)

and eval_list self state env exprs =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | e :: rest ->
      let* v = self state env e in
      go (v :: acc) rest
  in
  go [] exprs

and eval_let self state env is_rec bindings body =
  if is_rec
  then (
    let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
    let env, cells = Env.extend_recursive names env in
    let rec go cells = function
      | [] -> Ok ()
      | (b : Ast.binding) :: rest ->
        let* v = self state env b.rhs in
        let cells =
          match b.name, cells with
          | Some _, cell :: cells ->
            Env.fill cell v;
            cells
          | _, cells -> cells
        in
        go cells rest
    in
    let* () = go cells bindings in
    self state env body)
  else
    let* values =
      eval_list self state env (List.map (fun (b : Ast.binding) -> b.rhs) bindings)
    in
    let named =
      List.filter_map
        (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
        (List.combine bindings values)
    in
    self state (Env.extend named env) body

and eval_cases self state env v cases =
  match cases with
  | [] -> type_error "no case matched"
  | (pattern, body) :: rest ->
    (match Sec_4_1.bind_pattern pattern v with
     | None -> eval_cases self state env v rest
     | Some bindings -> self state (Env.extend bindings env) body)

and apply_from_expr self state env fn args =
  match Ast.view fn with
  | Ast.Var "amb" ->
    let* decision = choose state (List.length args) in
    self state env (List.nth args decision)
  | Ast.Var "require" ->
    (match args with
     | [ condition ] ->
       let* v = self state env condition in
       let* b = as_bool "require" v in
       if b then Ok Value.unit else fail
     | _ ->
       Error
         (Broke (Eval_error.Arity_mismatch { expected = 1; given = List.length args })))
  | _ ->
    let* fn = self state env fn in
    let* args = eval_list self state env args in
    apply_value self state fn args

and apply_value self state fn args =
  match args with
  | [] -> Ok fn
  | arg :: rest ->
    (match Value.view fn with
     | Value.Closure { parameters = parameter :: parameters; body; env; _ } ->
       let env = Env.extend [ parameter, arg ] env in
       if parameters = []
       then
         let* v = self state env body in
         apply_value self state v rest
       else apply_value self state (Value.closure ~name:None ~parameters ~body ~env) rest
     | Value.Closure { parameters = []; _ } -> not_applicable fn
     | Value.Primitive p -> apply_primitive self state p [] (arg :: rest)
     | Value.Partial (p, gathered) -> apply_primitive self state p gathered (arg :: rest)
     | _ -> not_applicable fn)

and apply_primitive self state p gathered args =
  let missing = p.prim_arity - List.length gathered in
  if List.length args < missing
  then Ok (Value.partial p (gathered @ args))
  else (
    let rec split n acc = function
      | rest when n = 0 -> List.rev acc, rest
      | x :: rest -> split (n - 1) (x :: acc) rest
      | [] -> List.rev acc, []
    in
    let taken, rest = split missing [] args in
    (* Primitives are deterministic: a procedure a primitive calls runs
       with an empty decision list, so amb beneath a primitive is
       outside the experiment. *)
    let deterministic fn values = beneath_primitive self state fn values in
    match p.prim_apply deterministic (gathered @ taken) with
    | Error e -> broke e
    | Ok result -> apply_value self state result rest)

and beneath_primitive self state fn values =
  let search = { state with decisions = [] } in
  match apply_value self search fn values with
  | Ok v -> Ok v
  | Error Failed ->
    Error (Eval_error.User_error "a search branch failed beneath a primitive")
  | Error (Incomplete _) ->
    Error
      (Eval_error.User_error
         "a choice point beneath a primitive is outside the experiment")
  | Error (Broke e) -> Error e
;;

let attempt eval state ~emit program =
  let rec go env outcome = function
    | [] -> Ok outcome
    | (item : Ast.item) :: rest ->
      (match item with
       | Ast.Type_item _ -> go env outcome rest
       | Ast.Value_item (is_rec, bindings) ->
         if is_rec
         then (
           let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
           let env, cells = Env.extend_recursive names env in
           let rec fill_all cells = function
             | [] -> Ok ()
             | (b : Ast.binding) :: rest ->
               let* v = eval state env b.rhs in
               let cells =
                 match b.name, cells with
                 | Some _, cell :: cells ->
                   Env.fill cell v;
                   cells
                 | _, cells -> cells
               in
               fill_all cells rest
           in
           let* () = fill_all cells bindings in
           let outcome =
             match
               List.rev (List.filter_map (fun (b : Ast.binding) -> b.name) bindings)
             with
             | name :: _ ->
               (match Env.find env name with
                | Some v -> v
                | None -> outcome)
             | [] -> outcome
           in
           go env outcome rest)
         else
           let* values =
             eval_list eval state env (List.map (fun (b : Ast.binding) -> b.rhs) bindings)
           in
           let named =
             List.filter_map
               (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
               (List.combine bindings values)
           in
           (* A named reference a permanent assignment wrote starts
              each later attempt at its survived value. *)
           List.iter
             (fun (name, v) ->
                match Value.view v, Hashtbl.find_opt state.store.survived name with
                | Value.Ref cell, Some survived -> cell := survived
                | _ -> ())
             named;
           state.top <- List.rev_append named state.top;
           let env = Env.extend named env in
           let outcome =
             match List.rev named with
             | (_, v) :: _ -> v
             | [] -> outcome
           in
           go env outcome rest)
  in
  go (Prelude.initial_env ~emit ()) Value.unit (Check.items program)
;;

let drive ~eval ~emit program =
  let store = { survived = Hashtbl.create 8; applied = Hashtbl.create 8 } in
  let rec loop pending answers choices failures =
    match pending with
    | [] -> Ok (answers, choices, failures)
    | path :: rest ->
      let state = { decisions = path; taken = []; permanents = ref 0; top = []; store } in
      let branch = Buffer.create 128 in
      (match attempt eval state ~emit:(Buffer.add_string branch) program with
       | Ok _ ->
         emit (Buffer.contents branch);
         loop rest (answers + 1) choices failures
       | Error Failed -> loop rest answers choices (failures + 1)
       | Error (Incomplete arity) ->
         let branches = List.init arity (fun i -> path @ [ i ]) in
         loop (branches @ rest) answers (choices + 1) failures
       | Error (Broke e) -> Error e)
  in
  match loop [ [] ] 0 0 0 with
  | Error e -> Error e
  | Ok (answers, choices, failures) ->
    emit
      (Printf.sprintf "answers: %d\nchoices: %d\nfailures: %d\n" answers choices failures);
    Ok Value.unit
;;

let open_eval ~self : eval_t = fun search env e -> step self search env e
let rec eval : eval_t = fun search env e -> step eval search env e
let run ~emit program = drive ~eval ~emit program

let run_with ~eval ~forms source =
  match
    Check.check_experiment_with
      ~experiment:Check.Search
      ~forms
      ~filename:"program.ml"
      source
  with
  | Error d -> "rejected: " ^ Check.kind_to_string d.kind
  | Ok program ->
    let out = Buffer.create 64 in
    (match drive ~eval ~emit:(Buffer.add_string out) program with
     | Ok _ -> Buffer.contents out
     | Error e -> Buffer.contents out ^ "error: " ^ Eval_error.to_string e)
;;
