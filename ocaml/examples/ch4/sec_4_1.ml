(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 4.1 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude
module Value = Sicp_common.Value

type outcome = (Value.t, Eval_error.t) result

(* The semantic core shared by the direct and analyzed evaluators: a
   strategy only answers how a closure body runs in its environment. *)

type strategy = Value.env -> Ast.expr -> outcome

let scalar_value = function
  | Ast.Int n -> Value.int n
  | Ast.Float f -> Value.float f
  | Ast.Bool b -> Value.bool b
  | Ast.String s -> Value.string s
  | Ast.Unit -> Value.unit
;;

let type_error detail = Error (Eval_error.Type_error detail)
let not_applicable v = Error (Eval_error.Not_applicable (Value.to_string v))
let unbound name = Error (Eval_error.Unbound_variable name)

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

(* The host relation of [op]: Stdlib's comparison, which on floats is
   the IEEE relation (a NaN is unequal and unordered). *)
let relation op x y =
  match op with
  | Ast.Eq -> x = y
  | Ast.Ne -> x <> y
  | Ast.Lt -> x < y
  | Ast.Le -> x <= y
  | Ast.Gt -> x > y
  | Ast.Ge -> x >= y
;;

let comparison op left right =
  let ordered =
    match op with
    | Ast.Eq | Ast.Ne -> false
    | _ -> true
  in
  match Value.view left, Value.view right with
  | Value.Int x, Value.Int y -> Ok (Value.bool (relation op x y))
  | Value.Float x, Value.Float y -> Ok (Value.bool (relation op x y))
  | Value.String x, Value.String y -> Ok (Value.bool (relation op x y))
  | Value.Bool x, Value.Bool y when not ordered -> Ok (Value.bool (relation op x y))
  | Value.Unit, Value.Unit when not ordered -> Ok (Value.bool (relation op () ()))
  | _ -> type_error "comparison operands are outside the admitted comparison types"
;;

(* [force] runs on a subject exactly where a pattern tests its shape:
   variables and wildcards bind the subject as it is. *)
let bind_pattern_with ~force (p : Ast.pattern) v =
  let rec bind (p : Ast.pattern) v =
    match Ast.view_pattern p with
    | Ast.PWildcard -> Ok (Some [])
    | Ast.PVar name -> Ok (Some [ name, v ])
    | shape ->
      let* v = force v in
      (match shape, Value.view v with
       | Ast.PScalar expected, _ ->
         (match Value.equal_scalars (scalar_value expected) v with
          | Ok true -> Ok (Some [])
          | Ok false | Error _ -> Ok None)
       | Ast.PTuple patterns, Value.Tuple values -> bind_many patterns values
       | Ast.PConstruct (expected, patterns), Value.Constructor (name, fields)
         when String.equal expected name -> bind_fields patterns fields
       | Ast.PNil, Value.Nil -> Ok (Some [])
       | Ast.PCons (head_pattern, tail_pattern), Value.Cons (head, tail) ->
         bind_many [ head_pattern; tail_pattern ] [ head; tail ]
       | _, _ -> Ok None)
  and bind_many patterns values =
    if List.length patterns <> List.length values
    then Ok None
    else (
      let rec go acc patterns values =
        match patterns, values with
        | [], [] -> Ok (Some (List.concat (List.rev acc)))
        | pattern :: patterns, value :: values ->
          let* bound = bind pattern value in
          (match bound with
           | None -> Ok None
           | Some bindings -> go (bindings :: acc) patterns values)
        | _, _ -> Ok None
      in
      go [] patterns values)
  and bind_fields patterns fields =
    let* bound = bind_many patterns fields in
    match bound with
    | Some _ -> Ok bound
    | None ->
      (* Grammar section 3: a constructor with multiple payload fields
         uses a tuple payload, so one variable payload binds the whole
         tuple and a multi-pattern payload destructures one tuple
         payload. *)
      (match patterns, fields with
       | [ _ ], [ _ ] -> Ok None
       | [ pattern ], values -> bind pattern (Value.tuple values)
       | patterns, [ value ] ->
         let* value = force value in
         (match Value.view value with
          | Value.Tuple values -> bind_many patterns values
          | _ -> Ok None)
       | _, _ -> Ok None)
  in
  bind p v
;;

let bind_pattern p v =
  match bind_pattern_with ~force:Result.ok p v with
  | Ok bound -> bound
  | Error _ -> None
;;

let rec eval (strategy : strategy) env (e : Ast.expr) : outcome =
  match Ast.view e with
  | Ast.Scalar s -> Ok (scalar_value s)
  | Ast.Var name ->
    (match Env.find env name with
     | Some v -> Ok v
     | None -> unbound name)
  | Ast.Let (is_rec, bindings, body) -> eval_let strategy env is_rec bindings body
  | Ast.Fun (parameters, body) -> Ok (Value.closure ~name:None ~parameters ~body ~env)
  | Ast.Apply (fn, args) ->
    let* fn = strategy env fn in
    let* args = eval_list strategy env args in
    apply strategy fn args
  | Ast.If (condition, consequent, alternative) ->
    let* condition = strategy env condition in
    let* b = as_bool "if" condition in
    if b then strategy env consequent else strategy env alternative
  | Ast.Match (scrutinee, cases) ->
    let* v = strategy env scrutinee in
    eval_cases strategy env v cases
  | Ast.Tuple parts ->
    let* parts = eval_list strategy env parts in
    Ok (Value.tuple parts)
  | Ast.Construct (name, fields) ->
    let* fields = eval_list strategy env fields in
    Ok (Value.construct name fields)
  | Ast.Record fields ->
    let* fields = eval_fields strategy env fields in
    Ok (Value.record fields)
  | Ast.Field (record, name) ->
    let* record = strategy env record in
    (match Value.view record with
     | Value.Record fields ->
       (match List.assoc_opt name fields with
        | Some v -> Ok v
        | None -> type_error ("record field " ^ name ^ " is absent"))
     | _ -> type_error "field access target is not a record")
  | Ast.Sequence (first, second) ->
    let* _ = strategy env first in
    strategy env second
  | Ast.And (left, right) ->
    let* left = strategy env left in
    let* b = as_bool "&&" left in
    if not b then Ok (Value.bool false) else strategy env right
  | Ast.Or (left, right) ->
    let* left = strategy env left in
    let* b = as_bool "||" left in
    if b then Ok (Value.bool true) else strategy env right
  | Ast.Arith (op, left, right) ->
    let* left = strategy env left in
    let* right = strategy env right in
    arithmetic op left right
  | Ast.Compare (op, left, right) ->
    let* left = strategy env left in
    let* right = strategy env right in
    comparison op left right
  | Ast.Nil -> Ok Value.nil
  | Ast.Cons (head, tail) ->
    let* head = strategy env head in
    let* tail = strategy env tail in
    Ok (Value.cons head tail)
  | Ast.Concat (left, right) ->
    let* left = strategy env left in
    let* right = strategy env right in
    (match Value.view left, Value.view right with
     | Value.String a, Value.String b -> Ok (Value.string (a ^ b))
     | _ -> type_error "^ operands are not strings")
  | Ast.Not operand ->
    let* operand = strategy env operand in
    let* b = as_bool "not" operand in
    Ok (Value.bool (not b))
  | Ast.Neg operand ->
    let* operand = strategy env operand in
    negate operand
  | Ast.Deref reference ->
    let* reference = strategy env reference in
    (match Value.view reference with
     | Value.Ref cell -> Ok !cell
     | _ -> type_error "! target is not a reference")
  | Ast.Assign (reference, rhs) ->
    let* reference = strategy env reference in
    let* value = strategy env rhs in
    (match Value.view reference with
     | Value.Ref cell ->
       cell := value;
       Ok Value.unit
     | _ -> type_error ":= target is not a reference")
  | Ast.Make_ref operand ->
    let* operand = strategy env operand in
    Ok (Value.ref_value operand)

and eval_list strategy env exprs =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | e :: rest ->
      let* v = strategy env e in
      go (v :: acc) rest
  in
  go [] exprs

and eval_fields strategy env fields =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | (name, e) :: rest ->
      let* v = strategy env e in
      go ((name, v) :: acc) rest
  in
  go [] fields

and eval_let strategy env is_rec bindings body =
  if is_rec
  then (
    let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
    let env, cells = Env.extend_recursive names env in
    let* () = eval_recursive_bindings strategy env cells bindings in
    strategy env body)
  else
    let* values =
      eval_list strategy env (List.map (fun (b : Ast.binding) -> b.rhs) bindings)
    in
    let named =
      List.filter_map
        (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
        (List.combine bindings values)
    in
    strategy (Env.extend named env) body

and eval_recursive_bindings strategy env cells bindings =
  let rec go cells bindings =
    match bindings with
    | [] -> Ok ()
    | (b : Ast.binding) :: rest ->
      let* v = strategy env b.rhs in
      let cells =
        match b.name, cells with
        | Some _, cell :: cells ->
          Env.fill cell v;
          cells
        | _, cells -> cells
      in
      go cells rest
  in
  go cells bindings

and eval_cases strategy env v cases =
  match cases with
  | [] -> type_error "no case matched"
  | (pattern, body) :: rest ->
    (match bind_pattern pattern v with
     | None -> eval_cases strategy env v rest
     | Some bindings -> strategy (Env.extend bindings env) body)

and apply strategy fn args =
  match args with
  | [] -> Ok fn
  | arg :: rest ->
    (match Value.view fn with
     | Value.Closure { parameters = parameter :: parameters; body; env; _ } ->
       let env = Env.extend [ parameter, arg ] env in
       if parameters = []
       then
         let* v = strategy env body in
         apply strategy v rest
       else apply strategy (Value.closure ~name:None ~parameters ~body ~env) rest
     | Value.Closure { parameters = []; _ } -> not_applicable fn
     | Value.Primitive p -> apply_primitive strategy p [] (arg :: rest)
     | Value.Partial (p, gathered) -> apply_primitive strategy p gathered (arg :: rest)
     | _ -> not_applicable fn)

and apply_primitive strategy p gathered args =
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
    let* result = p.prim_apply (apply strategy) (gathered @ taken) in
    apply strategy result rest)
;;

(* Program execution: type declarations introduce runtime data shapes
   (their constructors are values by name) and value declarations extend
   the global environment in source order. *)

let run_items strategy initial items =
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
           let* () = eval_recursive_bindings strategy env cells bindings in
           let last =
             List.fold_left
               (fun acc (b : Ast.binding) ->
                  match b.name with
                  | Some name -> Some name
                  | None -> acc)
               None
               bindings
           in
           let outcome =
             match last with
             | Some name ->
               (match Env.find env name with
                | Some v -> v
                | None -> outcome)
             | None -> outcome
           in
           go env outcome rest)
         else
           let* values =
             eval_list strategy env (List.map (fun (b : Ast.binding) -> b.rhs) bindings)
           in
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
           go env outcome rest)
  in
  go initial Value.unit items
;;

let rec direct env body = eval direct env body

let run ~emit program =
  run_items direct (Prelude.initial_env ~emit ()) (Check.items program)
;;

(* The analyzed evaluator: every syntax node is analyzed once per run
   into an environment application; execution never re-reads syntax
   structure except through the memoized analysis of closure bodies. *)

(* [analyzer ()] is a fresh analyzer: its memo table maps each closure
   body analyzed during one run to its execution procedure. *)
(* Analyses are memoized per body node, keyed physically: a closure
   call finds its body's analysis in constant time, never comparing
   expression trees; the hash is the body's source position, four
   integers. *)
module Body_table = Hashtbl.Make (struct
    type t = Ast.expr

    let equal = ( == )

    let hash e =
      let ({ start; stop } : Ast.span) = Ast.at e in
      Hashtbl.hash (start.line, start.column, stop.line, stop.column)
    ;;
  end)

let analyzer () =
  let memo : (Value.env -> outcome) Body_table.t = Body_table.create 64 in
  let rec analyze (e : Ast.expr) : Value.env -> outcome =
    match Ast.view e with
    | Ast.Scalar s -> fun _ -> Ok (scalar_value s)
    | Ast.Var name ->
      fun env ->
        (match Env.find env name with
         | Some v -> Ok v
         | None -> unbound name)
    | Ast.Let (is_rec, bindings, body) ->
      let rhss = List.map (fun (b : Ast.binding) -> analyze b.rhs) bindings in
      let body = analyze body in
      fun env -> eval_let_analyzed env is_rec bindings rhss body
    | Ast.Fun (parameters, body) ->
      let (_ : Value.env -> outcome) = analysis_of body in
      fun env -> Ok (Value.closure ~name:None ~parameters ~body ~env)
    | Ast.Apply (fn, args) ->
      let fn = analyze fn in
      let args = List.map analyze args in
      fun env ->
        let* fn = fn env in
        let* args = eval_all env args in
        apply analyzed_strategy fn args
    | Ast.If (condition, consequent, alternative) ->
      let condition = analyze condition in
      let consequent = analyze consequent in
      let alternative = analyze alternative in
      fun env ->
        let* condition = condition env in
        let* b = as_bool "if" condition in
        if b then consequent env else alternative env
    | Ast.Match (scrutinee, cases) ->
      let scrutinee = analyze scrutinee in
      let cases = List.map (fun (pattern, body) -> pattern, analyze body) cases in
      fun env ->
        let* v = scrutinee env in
        eval_cases_analyzed env v cases
    | Ast.Tuple parts ->
      let parts = List.map analyze parts in
      fun env -> Result.map Value.tuple (eval_all env parts)
    | Ast.Construct (name, fields) ->
      let fields = List.map analyze fields in
      fun env ->
        Result.map (fun fields -> Value.construct name fields) (eval_all env fields)
    | Ast.Record fields ->
      let fields = List.map (fun (name, e) -> name, analyze e) fields in
      fun env ->
        let rec go acc = function
          | [] -> Ok (Value.record (List.rev acc))
          | (name, proc) :: rest ->
            let* v = proc env in
            go ((name, v) :: acc) rest
        in
        go [] fields
    | Ast.Field (record, name) ->
      let record = analyze record in
      fun env ->
        let* record = record env in
        (match Value.view record with
         | Value.Record fields ->
           (match List.assoc_opt name fields with
            | Some v -> Ok v
            | None -> type_error ("record field " ^ name ^ " is absent"))
         | _ -> type_error "field access target is not a record")
    | Ast.Sequence (first, second) ->
      let first = analyze first in
      let second = analyze second in
      fun env ->
        let* _ = first env in
        second env
    | Ast.And (left, right) ->
      let left = analyze left in
      let right = analyze right in
      fun env ->
        let* left = left env in
        let* b = as_bool "&&" left in
        if not b then Ok (Value.bool false) else right env
    | Ast.Or (left, right) ->
      let left = analyze left in
      let right = analyze right in
      fun env ->
        let* left = left env in
        let* b = as_bool "||" left in
        if b then Ok (Value.bool true) else right env
    | Ast.Arith (op, left, right) ->
      let left = analyze left in
      let right = analyze right in
      fun env ->
        let* left = left env in
        let* right = right env in
        arithmetic op left right
    | Ast.Compare (op, left, right) ->
      let left = analyze left in
      let right = analyze right in
      fun env ->
        let* left = left env in
        let* right = right env in
        comparison op left right
    | Ast.Nil -> fun _ -> Ok Value.nil
    | Ast.Cons (head, tail) ->
      let head = analyze head in
      let tail = analyze tail in
      fun env ->
        let* head = head env in
        let* tail = tail env in
        Ok (Value.cons head tail)
    | Ast.Concat (left, right) ->
      let left = analyze left in
      let right = analyze right in
      fun env ->
        let* left = left env in
        let* right = right env in
        (match Value.view left, Value.view right with
         | Value.String a, Value.String b -> Ok (Value.string (a ^ b))
         | _ -> type_error "^ operands are not strings")
    | Ast.Not operand ->
      let operand = analyze operand in
      fun env ->
        let* operand = operand env in
        let* b = as_bool "not" operand in
        Ok (Value.bool (not b))
    | Ast.Neg operand ->
      let operand = analyze operand in
      fun env ->
        let* operand = operand env in
        negate operand
    | Ast.Deref reference ->
      let reference = analyze reference in
      fun env ->
        let* reference = reference env in
        (match Value.view reference with
         | Value.Ref cell -> Ok !cell
         | _ -> type_error "! target is not a reference")
    | Ast.Assign (reference, rhs) ->
      let reference = analyze reference in
      let rhs = analyze rhs in
      fun env ->
        let* reference = reference env in
        let* value = rhs env in
        (match Value.view reference with
         | Value.Ref cell ->
           cell := value;
           Ok Value.unit
         | _ -> type_error ":= target is not a reference")
    | Ast.Make_ref operand ->
      let operand = analyze operand in
      fun env ->
        let* operand = operand env in
        Ok (Value.ref_value operand)
  and analysis_of body =
    match Body_table.find_opt memo body with
    | Some proc -> proc
    | None ->
      let proc = analyze body in
      Body_table.replace memo body proc;
      proc
  and analyzed_strategy env body = analysis_of body env
  and eval_all env procs =
    let rec go acc = function
      | [] -> Ok (List.rev acc)
      | proc :: rest ->
        let* v = proc env in
        go (v :: acc) rest
    in
    go [] procs
  and eval_let_analyzed env is_rec bindings rhss body =
    if is_rec
    then (
      let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
      let env, cells = Env.extend_recursive names env in
      let rec go cells bindings rhss =
        match bindings, rhss with
        | [], [] -> Ok ()
        | (b : Ast.binding) :: bindings, rhs :: rhss ->
          let* v = rhs env in
          let cells =
            match b.name, cells with
            | Some _, cell :: cells ->
              Env.fill cell v;
              cells
            | _, cells -> cells
          in
          go cells bindings rhss
        | _, _ -> Error (Eval_error.Invalid_form "binding group mismatch")
      in
      let* () = go cells bindings rhss in
      body env)
    else
      let* values = eval_all env rhss in
      let named =
        List.filter_map
          (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
          (List.combine bindings values)
      in
      body (Env.extend named env)
  and eval_cases_analyzed env v cases =
    match cases with
    | [] -> type_error "no case matched"
    | (pattern, body) :: rest ->
      (match bind_pattern pattern v with
       | None -> eval_cases_analyzed env v rest
       | Some bindings -> body (Env.extend bindings env))
  in
  analyze
;;

let analyze e = analyzer () e

let run_analyzed ~emit program =
  let analyze = analyzer () in
  let eval_all env procs =
    let rec go acc = function
      | [] -> Ok (List.rev acc)
      | proc :: rest ->
        let* v = proc env in
        go (v :: acc) rest
    in
    go [] procs
  in
  let items = Check.items program in
  let procs =
    List.map
      (fun (item : Ast.item) ->
         match item with
         | Ast.Type_item _ -> `Type
         | Ast.Value_item (is_rec, bindings) ->
           `Value
             (is_rec, bindings, List.map (fun (b : Ast.binding) -> analyze b.rhs) bindings))
      items
  in
  let rec go env outcome = function
    | [] -> Ok outcome
    | `Type :: rest -> go env outcome rest
    | `Value (is_rec, bindings, rhss) :: rest ->
      if is_rec
      then (
        let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
        let env, cells = Env.extend_recursive names env in
        let rec fill_all cells bindings rhss =
          match bindings, rhss with
          | [], [] -> Ok ()
          | (b : Ast.binding) :: bindings, rhs :: rhss ->
            let* v = rhs env in
            let cells =
              match b.name, cells with
              | Some _, cell :: cells ->
                Env.fill cell v;
                cells
              | _, cells -> cells
            in
            fill_all cells bindings rhss
          | _, _ -> Error (Eval_error.Invalid_form "binding group mismatch")
        in
        let* () = fill_all cells bindings rhss in
        let outcome =
          match List.rev (List.filter_map (fun (b : Ast.binding) -> b.name) bindings) with
          | name :: _ ->
            (match Env.find env name with
             | Some v -> v
             | None -> outcome)
          | [] -> outcome
        in
        go env outcome rest)
      else
        let* values = eval_all env rhss in
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
        go env outcome rest
  in
  go (Prelude.initial_env ~emit ()) Value.unit procs
;;

(* {1 The evaluators as functions over one expression} *)

type eval_t = Ast.expr -> Env.t -> outcome

let eval_expr : eval_t = fun e env -> eval direct env e
let apply_procedure proc args = apply direct proc args
let the_global_environment ?(emit = print_string) () = Prelude.initial_env ~emit ()

let expression source =
  match Check.check ~filename:"expression.ml" ("let it = (" ^ source ^ ")") with
  | Error d -> Error (Check.diagnostic_to_string d)
  | Ok program ->
    (match Check.items program with
     | [ Ast.Value_item (false, [ { Ast.rhs; _ } ]) ] -> Ok rhs
     | _ -> Error "expression: one binding expected")
;;

let transcript ?(experiment = Check.Core) run source =
  match Check.check_experiment ~experiment ~filename:"program.ml" source with
  | Error d -> "rejected: " ^ Check.kind_to_string d.kind
  | Ok program ->
    let out = Buffer.create 64 in
    (match run ~emit:(Buffer.add_string out) program with
     | Ok _ -> Buffer.contents out
     | Error e -> Buffer.contents out ^ "error: " ^ Eval_error.to_string e)
;;

let open_eval ~(self : eval_t) : eval_t =
  fun e env -> eval (fun env e -> self e env) env e
;;

let apply_with ~(self : eval_t) proc args = apply (fun env e -> self e env) proc args
