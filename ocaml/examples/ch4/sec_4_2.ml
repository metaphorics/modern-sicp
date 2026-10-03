(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 4.2 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude
module Value = Sicp_common.Value

type counts =
  { allocations : int
  ; forces : int
  ; recomputations : int
  ; memo_hits : int
  }

type counters =
  { mutable allocations : int
  ; mutable forces : int
  ; mutable recomputations : int
  ; mutable memo_hits : int
  ; memoize : bool
  }

type state = counters
type eval_t = Ast.expr -> Env.t -> (Value.t, Eval_error.t) result

let state ?(memoize = true) () =
  { allocations = 0; forces = 0; recomputations = 0; memo_hits = 0; memoize }
;;

let type_error detail = Error (Eval_error.Type_error detail)
let not_applicable v = Error (Eval_error.Not_applicable (Value.to_string v))
let unbound name = Error (Eval_error.Unbound_variable name)

let scalar_value = function
  | Ast.Int n -> Value.int n
  | Ast.Float f -> Value.float f
  | Ast.Bool b -> Value.bool b
  | Ast.String s -> Value.string s
  | Ast.Unit -> Value.unit
;;

let as_bool what v =
  match Value.view v with
  | Value.Bool b -> Ok b
  | _ -> type_error (what ^ ": condition is not a bool")
;;

let arithmetic op left right =
  match op, Value.view left, Value.view right with
  | Ast.Add, Value.Int x, Value.Int y -> Ok (Value.int (x + y))
  | Ast.Sub, Value.Int x, Value.Int y -> Ok (Value.int (x - y))
  | Ast.Mul, Value.Int x, Value.Int y -> Ok (Value.int (x * y))
  | Ast.Div, Value.Int _, Value.Int 0 -> Error Eval_error.Division_by_zero
  | Ast.Div, Value.Int x, Value.Int y -> Ok (Value.int (x / y))
  | Ast.Rem, Value.Int _, Value.Int 0 -> Error Eval_error.Division_by_zero
  | Ast.Rem, Value.Int x, Value.Int y -> Ok (Value.int (x mod y))
  | Ast.Addf, Value.Float x, Value.Float y -> Ok (Value.float (x +. y))
  | Ast.Subf, Value.Float x, Value.Float y -> Ok (Value.float (x -. y))
  | Ast.Mulf, Value.Float x, Value.Float y -> Ok (Value.float (x *. y))
  | Ast.Divf, Value.Float x, Value.Float y -> Ok (Value.float (x /. y))
  | _ -> type_error "arithmetic operands do not match the operator"
;;

let negate v =
  match Value.view v with
  | Value.Int n -> Ok (Value.int (Int.neg n))
  | Value.Float f -> Ok (Value.float (Float.neg f))
  | _ -> type_error "negation operand is not a number"
;;

let comparison op left right =
  match op with
  | Ast.Eq -> Result.map Value.bool (Value.equal_scalars left right)
  | Ast.Ne -> Result.map (fun b -> Value.bool (not b)) (Value.equal_scalars left right)
  | Ast.Lt -> Result.map (fun n -> Value.bool (n < 0)) (Value.compare_scalars left right)
  | Ast.Le -> Result.map (fun n -> Value.bool (n <= 0)) (Value.compare_scalars left right)
  | Ast.Gt -> Result.map (fun n -> Value.bool (n > 0)) (Value.compare_scalars left right)
  | Ast.Ge -> Result.map (fun n -> Value.bool (n >= 0)) (Value.compare_scalars left right)
;;

(* The experiment: [eval] answers values that may be thunks;
   [actual_value] forces them under the counting rules; closure
   arguments and let right-hand sides are delayed. *)

let rec step self counters env (e : Ast.expr) : (Value.t, Eval_error.t) result =
  match Ast.view e with
  | Ast.Scalar s -> Ok (scalar_value s)
  | Ast.Var name ->
    (match Env.find env name with
     | Some v -> Ok v
     | None -> unbound name)
  | Ast.Let (is_rec, bindings, body) -> eval_let self counters env is_rec bindings body
  | Ast.Fun (parameters, body) -> Ok (Value.closure ~name:None ~parameters ~body ~env)
  | Ast.Apply (fn, args) -> apply_from_expr self counters env fn args
  | Ast.If (condition, consequent, alternative) ->
    let* condition = self env condition in
    let* condition = actual_value self counters condition in
    let* b = as_bool "if" condition in
    if b then self env consequent else self env alternative
  | Ast.Match (scrutinee, cases) ->
    let* v = self env scrutinee in
    let* v = actual_value self counters v in
    eval_cases self counters env v cases
  | Ast.Tuple parts ->
    let* parts = eval_list self counters env parts in
    Ok (Value.tuple parts)
  | Ast.Construct (name, fields) ->
    let* fields = eval_list self counters env fields in
    Ok (Value.construct name fields)
  | Ast.Record fields ->
    let* fields = eval_fields self counters env fields in
    Ok (Value.record fields)
  | Ast.Field (record, name) ->
    let* record = self env record in
    let* record = actual_value self counters record in
    (match Value.view record with
     | Value.Record fields ->
       (match List.assoc_opt name fields with
        | Some v -> Ok v
        | None -> type_error ("record field " ^ name ^ " is absent"))
     | _ -> type_error "field access target is not a record")
  | Ast.Sequence (first, second) ->
    let* first = self env first in
    let* _ = actual_value self counters first in
    self env second
  | Ast.And (left, right) ->
    let* left = self env left in
    let* left = actual_value self counters left in
    let* b = as_bool "&&" left in
    if not b then Ok (Value.bool false) else self env right
  | Ast.Or (left, right) ->
    let* left = self env left in
    let* left = actual_value self counters left in
    let* b = as_bool "||" left in
    if b then Ok (Value.bool true) else self env right
  | Ast.Arith (op, left, right) ->
    let* left = self env left in
    let* left = actual_value self counters left in
    let* right = self env right in
    let* right = actual_value self counters right in
    arithmetic op left right
  | Ast.Compare (op, left, right) ->
    let* left = self env left in
    let* left = actual_value self counters left in
    let* right = self env right in
    let* right = actual_value self counters right in
    comparison op left right
  | Ast.Nil -> Ok Value.nil
  | Ast.Cons (head, tail) ->
    let* head = self env head in
    let* tail = self env tail in
    Ok (Value.cons head tail)
  | Ast.Concat (left, right) ->
    let* left = self env left in
    let* left = actual_value self counters left in
    let* right = self env right in
    let* right = actual_value self counters right in
    (match Value.view left, Value.view right with
     | Value.String a, Value.String b -> Ok (Value.string (a ^ b))
     | _ -> type_error "^ operands are not strings")
  | Ast.Not operand ->
    let* operand = self env operand in
    let* operand = actual_value self counters operand in
    let* b = as_bool "not" operand in
    Ok (Value.bool (not b))
  | Ast.Neg operand ->
    let* operand = self env operand in
    let* operand = actual_value self counters operand in
    negate operand
  | Ast.Deref reference ->
    let* reference = self env reference in
    let* reference = actual_value self counters reference in
    (match Value.view reference with
     | Value.Ref cell -> Ok !cell
     | _ -> type_error "! target is not a reference")
  | Ast.Assign (reference, rhs) ->
    let* reference = self env reference in
    let* reference = actual_value self counters reference in
    let* value = self env rhs in
    let* value = actual_value self counters value in
    (match Value.view reference with
     | Value.Ref cell ->
       cell := value;
       Ok Value.unit
     | _ -> type_error ":= target is not a reference")
  | Ast.Make_ref operand ->
    let* operand = self env operand in
    let* operand = actual_value self counters operand in
    Ok (Value.ref_value operand)

and actual_value self counters v =
  match Value.view v with
  | Value.Thunk cell ->
    counters.forces <- counters.forces + 1;
    (match !cell with
     | Value.Forced memo ->
       counters.memo_hits <- counters.memo_hits + 1;
       Ok memo
     | Value.Delayed (expr, env) ->
       counters.recomputations <- counters.recomputations + 1;
       let* forced = self env expr in
       let* forced = actual_value self counters forced in
       if counters.memoize then Value.set_thunk_state cell (Value.Forced forced);
       Ok forced)
  | _ -> Ok v

and eval_list self _counters env exprs =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | e :: rest ->
      let* v = self env e in
      go (v :: acc) rest
  in
  go [] exprs

and eval_strict_list self counters env exprs =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | e :: rest ->
      let* v = self env e in
      let* v = actual_value self counters v in
      go (v :: acc) rest
  in
  go [] exprs

and eval_fields self _counters env fields =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | (name, e) :: rest ->
      let* v = self env e in
      go ((name, v) :: acc) rest
  in
  go [] fields

and eval_let self counters env is_rec bindings body =
  if is_rec
  then (
    let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
    let env, cells = Env.extend_recursive names env in
    let rec go cells = function
      | [] -> Ok ()
      | (b : Ast.binding) :: rest ->
        let* v =
          match b.name with
          | None ->
            let* v = self env b.rhs in
            actual_value self counters v
          | Some _ ->
            counters.allocations <- counters.allocations + 1;
            Ok (Value.thunk ~expr:b.rhs ~env)
        in
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
    self env body)
  else (
    let rec go acc = function
      | [] -> Ok (List.rev acc)
      | (b : Ast.binding) :: rest ->
        (* An unnamed binding ([_ = e] or [() = e]) exists for its
           effects, so its right-hand side is forced at once. *)
        (match b.name with
         | None ->
           let* v = self env b.rhs in
           let* v = actual_value self counters v in
           go (v :: acc) rest
         | Some _ ->
           counters.allocations <- counters.allocations + 1;
           go (Value.thunk ~expr:b.rhs ~env :: acc) rest)
    in
    let* values = go [] bindings in
    let named =
      List.filter_map
        (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
        (List.combine bindings values)
    in
    self (Env.extend named env) body)

and eval_cases self counters env v cases =
  match cases with
  | [] -> type_error "no case matched"
  | (pattern, body) :: rest ->
    let* bound =
      Sec_4_1.bind_pattern_with ~force:(actual_value self counters) pattern v
    in
    (match bound with
     | None -> eval_cases self counters env v rest
     | Some bindings -> self (Env.extend bindings env) body)

and apply_from_expr self counters env fn args =
  match Ast.view fn with
  | Ast.Var "delay" ->
    (match args with
     | [ expr ] ->
       counters.allocations <- counters.allocations + 1;
       Ok (Value.thunk ~expr ~env)
     | _ -> Error (Eval_error.Arity_mismatch { expected = 1; given = List.length args }))
  | Ast.Var "force" ->
    (match args with
     | [ expr ] ->
       let* v = self env expr in
       actual_value self counters v
     | _ -> Error (Eval_error.Arity_mismatch { expected = 1; given = List.length args }))
  | _ ->
    let* fn = self env fn in
    let* fn = actual_value self counters fn in
    apply_value self counters env fn args

and apply_value self counters caller_env fn args =
  match args with
  | [] -> Ok fn
  | _ ->
    (match Value.view fn with
     | Value.Closure { parameters = []; _ } -> not_applicable fn
     | Value.Closure { parameters; body; env; _ } ->
       let rec take bound parameters args =
         match parameters, args with
         | [], rest -> `Done (List.rev bound, rest)
         | _, [] -> `Partial (List.rev bound, parameters)
         | parameter :: parameters, arg :: rest ->
           counters.allocations <- counters.allocations + 1;
           take
             ((parameter, Value.thunk ~expr:arg ~env:caller_env) :: bound)
             parameters
             rest
       in
       (match take [] parameters args with
        | `Done (bound, rest) ->
          let* result = self (Env.extend bound env) body in
          (match rest with
           | [] -> Ok result
           | _ -> apply_value self counters caller_env result rest)
        | `Partial (bound, remaining) ->
          Ok
            (Value.closure
               ~name:None
               ~parameters:remaining
               ~body
               ~env:(Env.extend bound env)))
     | Value.Primitive p ->
       let* values = eval_strict_list self counters caller_env args in
       apply_primitive_values self counters p [] values
     | Value.Partial (p, gathered) ->
       let* values = eval_strict_list self counters caller_env args in
       apply_primitive_values self counters p gathered values
     | _ -> not_applicable fn)

and apply_primitive_values self counters p gathered values =
  let missing = p.prim_arity - List.length gathered in
  if List.length values < missing
  then Ok (Value.partial p (gathered @ values))
  else (
    let rec split n acc = function
      | rest when n = 0 -> List.rev acc, rest
      | x :: rest -> split (n - 1) (x :: acc) rest
      | [] -> List.rev acc, []
    in
    let taken, rest = split missing [] values in
    let* result = p.prim_apply (apply_already_valued self counters) (gathered @ taken) in
    apply_already_valued self counters result rest)

and apply_already_valued self counters fn values =
  match values with
  | [] -> Ok fn
  | value :: rest ->
    (match Value.view fn with
     | Value.Closure { parameters = parameter :: parameters; body; env; _ } ->
       let env = Env.extend [ parameter, Value.forced value ] env in
       if parameters = []
       then
         let* result = self env body in
         apply_already_valued self counters result rest
       else
         apply_already_valued
           self
           counters
           (Value.closure ~name:None ~parameters ~body ~env)
           rest
     | Value.Primitive p -> apply_primitive_values self counters p [] values
     | Value.Partial (p, gathered) ->
       apply_primitive_values self counters p gathered values
     | _ -> not_applicable fn)
;;

let counts (c : counters) : counts =
  { allocations = c.allocations
  ; forces = c.forces
  ; recomputations = c.recomputations
  ; memo_hits = c.memo_hits
  }
;;

let flip (self : eval_t) env e = self e env
let open_eval ~self counters : eval_t = fun e env -> step (flip self) counters env e
let actual_value ~self counters v = actual_value (flip self) counters v

let delay counters expr env =
  counters.allocations <- counters.allocations + 1;
  Value.thunk ~expr ~env
;;

let apply ~self counters env fn args = apply_value (flip self) counters env fn args

let fix open_ =
  let rec self e env = open_ ~self e env in
  self
;;

let run_with ~(self : eval_t) counters ~emit program =
  let actual_value = actual_value ~self in
  let self env e = self e env in
  let initial = Prelude.initial_env ~emit () in
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
               let* v = self env b.rhs in
               let* v = actual_value counters v in
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
         else (
           let rec eval_all acc = function
             | [] -> Ok (List.rev acc)
             | (b : Ast.binding) :: rest ->
               let* v = self env b.rhs in
               let* v = actual_value counters v in
               eval_all (v :: acc) rest
           in
           let* values = eval_all [] bindings in
           let named =
             List.filter_map
               (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
               (List.combine bindings values)
           in
           let env = Env.extend named env in
           let outcome =
             match List.rev named with
             | (_, v) :: _ -> v
             | [] -> outcome
           in
           go env outcome rest))
  in
  let* outcome = go initial Value.unit (Check.items program) in
  emit
    (Printf.sprintf
       "thunk-allocations: %d\n\
        thunk-forces: %d\n\
        thunk-recomputations: %d\n\
        thunk-memo-hits: %d\n"
       counters.allocations
       counters.forces
       counters.recomputations
       counters.memo_hits);
  Ok outcome
;;

let run ~emit program =
  let st = state () in
  run_with ~self:(fix (fun ~self -> open_eval ~self st)) st ~emit program
;;
