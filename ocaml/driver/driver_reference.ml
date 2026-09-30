(* SPDX-License-Identifier: GPL-3.0-only *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude

(** Independent finite reference models for the named experiments and
    domain fixtures (grammar section 10).  Each model executes the
    artifact itself with its own evaluator code and its own scanner for
    the s-expression data languages; none of them calls a teaching
    engine.  The rules they implement are the documented experiment
    rules, so their transcripts are the expectations the teaching
    engines are compared against.

    Lazy rules: compound arguments and named [let] right-hand sides
    allocate delayed cells; unnamed bindings ([_ = e], [() = e]) are
    forced at once for their effects; primitives are strict; a demand
    forces a thunk once and memoizes.  [thunk-allocations],
    [thunk-forces], [thunk-recomputations], and [thunk-memo-hits] count
    the run and follow the guest output.

    Search rules: depth-first over choice points, left alternative
    first; a branch is one root-to-outcome path; failed branches leave
    no effects; the transcript carries the printing of successful
    branches in answer order, then [answers], [choices], and
    [failures].

    Query rules: answers of one query render the query pattern with its
    variables substituted.  A simple query appends its assertion
    answers (assertion order) before its rule-derived answers (rule
    order); [and] threads conjuncts left to right with interleaved
    element streams; [or] interleaves its disjuncts' streams; [not]
    answers once iff its inner query answers nothing.  Each query is
    announced with a line starting [? ].

    Machine rules: the controller's instruction semantics over explicit
    state; [print] operations write their operand and a newline; the
    transcript then carries one [name: value] line per declared
    register, in declaration order. *)
module Value = Sicp_common.Value

let ( let* ) = Result.bind
let fail message = Error message

let read_file path =
  let channel = open_in_bin path in
  Fun.protect
    ~finally:(fun () -> close_in channel)
    (fun () -> really_input_string channel (in_channel_length channel))
;;

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
  | _ -> fail ("reference: " ^ what ^ " condition is not a bool")
;;

let arithmetic op left right =
  match op, Value.view left, Value.view right with
  | Ast.Add, Value.Int x, Value.Int y -> Ok (Value.int (x + y))
  | Ast.Sub, Value.Int x, Value.Int y -> Ok (Value.int (x - y))
  | Ast.Mul, Value.Int x, Value.Int y -> Ok (Value.int (x * y))
  | Ast.Div, Value.Int _, Value.Int 0 -> fail "reference: division by zero"
  | Ast.Div, Value.Int x, Value.Int y -> Ok (Value.int (x / y))
  | Ast.Rem, Value.Int _, Value.Int 0 -> fail "reference: division by zero"
  | Ast.Rem, Value.Int x, Value.Int y -> Ok (Value.int (x mod y))
  | Ast.Addf, Value.Float x, Value.Float y -> Ok (Value.float (x +. y))
  | Ast.Subf, Value.Float x, Value.Float y -> Ok (Value.float (x -. y))
  | Ast.Mulf, Value.Float x, Value.Float y -> Ok (Value.float (x *. y))
  | Ast.Divf, Value.Float x, Value.Float y -> Ok (Value.float (x /. y))
  | _ -> fail "reference: arithmetic operands do not match the operator"
;;

let negate v =
  match Value.view v with
  | Value.Int n -> Ok (Value.int (Int.neg n))
  | Value.Float f -> Ok (Value.float (Float.neg f))
  | _ -> fail "reference: negation operand is not a number"
;;

let comparison op left right =
  (* IEEE on floats: every ordered or equality test with a NaN operand
     is false except [<>]. *)
  let floats x y =
    match op with
    | Ast.Eq -> Float.equal x y && not (Float.is_nan x)
    | Ast.Ne -> (not (Float.equal x y)) || Float.is_nan x
    | Ast.Lt -> Float.compare x y < 0 && not (Float.is_nan x || Float.is_nan y)
    | Ast.Le -> Float.compare x y <= 0 && not (Float.is_nan x || Float.is_nan y)
    | Ast.Gt -> Float.compare x y > 0 && not (Float.is_nan x || Float.is_nan y)
    | Ast.Ge -> Float.compare x y >= 0 && not (Float.is_nan x || Float.is_nan y)
  in
  let by_order c =
    match op with
    | Ast.Eq -> c = 0
    | Ast.Ne -> c <> 0
    | Ast.Lt -> c < 0
    | Ast.Le -> c <= 0
    | Ast.Gt -> c > 0
    | Ast.Ge -> c >= 0
  in
  let equality_only =
    match op with
    | Ast.Eq | Ast.Ne -> true
    | _ -> false
  in
  match Value.view left, Value.view right with
  | Value.Int x, Value.Int y -> Ok (Value.bool (by_order (Int.compare x y)))
  | Value.Float x, Value.Float y -> Ok (Value.bool (floats x y))
  | Value.String x, Value.String y -> Ok (Value.bool (by_order (String.compare x y)))
  | Value.Bool x, Value.Bool y when equality_only ->
    Ok (Value.bool (by_order (Bool.compare x y)))
  | Value.Unit, Value.Unit when equality_only -> Ok (Value.bool (by_order 0))
  | _ -> fail "reference: comparison operands are not admitted scalars"
;;

(* The reference models' own pattern matcher: a constructor pattern
   with one payload pattern binds the whole payload tuple, several
   payload patterns destructure it (grammar section 3). *)
let rec match_pattern (p : Ast.pattern) v =
  match Ast.view_pattern p, Value.view v with
  | Ast.PWildcard, _ -> Some []
  | Ast.PVar name, _ -> Some [ name, v ]
  | Ast.PScalar s, _ ->
    (match comparison Ast.Eq (scalar_value s) v with
     | Ok b when Value.view b = Value.Bool true -> Some []
     | _ -> None)
  | Ast.PTuple ps, Value.Tuple vs -> match_all ps vs
  | Ast.PNil, Value.Nil -> Some []
  | Ast.PCons (hp, tp), Value.Cons (h, t) -> match_all [ hp; tp ] [ h; t ]
  | Ast.PConstruct (name, ps), Value.Constructor (tag, fields) when String.equal name tag
    ->
    (* The payload as one value: its single field, or the tuple of its
       fields; several payload patterns destructure that tuple. *)
    let payload =
      match fields with
      | [ field ] -> field
      | _ -> Value.tuple fields
    in
    (match ps, Value.view payload with
     | [], _ -> if fields = [] then Some [] else None
     | [ single ], _ -> match_pattern single payload
     | _, Value.Tuple parts -> match_all ps parts
     | _, _ -> None)
  | _ -> None

and match_all ps vs =
  if List.length ps <> List.length vs
  then None
  else
    List.fold_left2
      (fun acc p v ->
         match acc with
         | None -> None
         | Some bound -> Option.map (fun more -> bound @ more) (match_pattern p v))
      (Some [])
      ps
      vs
;;

(* [match_pattern_forcing ~demand] is [match_pattern] for delayed values:
   [demand] runs on each subject whose shape the pattern tests (a scalar,
   tuple, constructor, [[]] or [::] pattern) before the test, while
   variables and wildcards bind their subject undemanded.  A demand
   error stops the match.  It mirrors [Sec_4_1.bind_pattern_with]. *)
let match_pattern_forcing ~demand (p : Ast.pattern) v =
  let ( let* ) = Result.bind in
  let rec go (p : Ast.pattern) v =
    match Ast.view_pattern p with
    | Ast.PWildcard -> Ok (Some [])
    | Ast.PVar name -> Ok (Some [ name, v ])
    | shape ->
      let* v = demand v in
      (match shape, Value.view v with
       | Ast.PScalar s, _ ->
         (match comparison Ast.Eq (scalar_value s) v with
          | Ok b when Value.view b = Value.Bool true -> Ok (Some [])
          | _ -> Ok None)
       | Ast.PTuple ps, Value.Tuple vs -> go_all ps vs
       | Ast.PNil, Value.Nil -> Ok (Some [])
       | Ast.PCons (hp, tp), Value.Cons (h, t) -> go_all [ hp; tp ] [ h; t ]
       | Ast.PConstruct (name, ps), Value.Constructor (tag, fields)
         when String.equal name tag ->
         let* bound = go_all ps fields in
         (match bound with
          | Some _ -> Ok bound
          | None ->
            (* Grammar section 3: a constructor with multiple payload
               fields uses a tuple payload, so one payload pattern binds
               the tuple of the fields and several payload patterns
               destructure one tuple payload.  A single pattern against a
               single field already failed above, so it answers [None]
               without demanding twice. *)
            (match ps, fields with
             | [ _ ], [ _ ] -> Ok None
             | [ single ], _ -> go single (Value.tuple fields)
             | _, [ payload ] ->
               let* payload = demand payload in
               (match Value.view payload with
                | Value.Tuple parts -> go_all ps parts
                | _ -> Ok None)
             | _, _ -> Ok None))
       | _, _ -> Ok None)
  and go_all ps vs =
    if List.length ps <> List.length vs
    then Ok None
    else (
      let rec loop acc ps vs =
        match ps, vs with
        | [], [] -> Ok (Some (List.concat (List.rev acc)))
        | p :: ps, v :: vs ->
          let* bound = go p v in
          (match bound with
           | None -> Ok None
           | Some bindings -> loop (bindings :: acc) ps vs)
        | _, _ -> Ok None
      in
      loop [] ps vs)
  in
  go p v
;;

(* {1 Lazy model} *)

module Lazy_model = struct
  type counts =
    { mutable allocations : int
    ; mutable forces : int
    ; mutable recomputations : int
    ; mutable memo_hits : int
    }

  let counts = { allocations = 0; forces = 0; recomputations = 0; memo_hits = 0 }

  let reset () =
    counts.allocations <- 0;
    counts.forces <- 0;
    counts.recomputations <- 0;
    counts.memo_hits <- 0
  ;;

  let rec eval env (e : Ast.expr) =
    match Ast.view e with
    | Ast.Scalar s -> Ok (scalar_value s)
    | Ast.Var name ->
      (match Env.find env name with
       | Some v -> Ok v
       | None -> fail ("reference: unbound " ^ name))
    | Ast.Let (is_rec, bindings, body) -> eval_let env is_rec bindings body
    | Ast.Fun (parameters, body) -> Ok (Value.closure ~name:None ~parameters ~body ~env)
    | Ast.Apply (fn, args) -> apply_expr env fn args
    | Ast.If (condition, consequent, alternative) ->
      let* condition = eval env condition in
      let* condition = demand condition in
      let* b = as_bool "if" condition in
      if b then eval env consequent else eval env alternative
    | Ast.Match (scrutinee, cases) ->
      let* v = eval env scrutinee in
      let* v = demand v in
      eval_cases env v cases
    | Ast.Tuple parts ->
      let rec gather acc = function
        | [] -> Ok (Value.tuple (List.rev acc))
        | e :: rest ->
          let* v = eval env e in
          gather (v :: acc) rest
      in
      gather [] parts
    | Ast.Construct (name, fields) ->
      let rec gather acc = function
        | [] -> Ok (Value.construct name (List.rev acc))
        | e :: rest ->
          let* v = eval env e in
          gather (v :: acc) rest
      in
      gather [] fields
    | Ast.Record fields ->
      let rec gather acc = function
        | [] -> Ok (Value.record (List.rev acc))
        | (name, e) :: rest ->
          let* v = eval env e in
          gather ((name, v) :: acc) rest
      in
      gather [] fields
    | Ast.Field (record, name) ->
      let* record = eval env record in
      let* record = demand record in
      (match Value.view record with
       | Value.Record fields ->
         (match List.assoc_opt name fields with
          | Some v -> Ok v
          | None -> fail ("reference: absent field " ^ name))
       | _ -> fail "reference: field target is not a record")
    | Ast.Sequence (first, second) ->
      let* first = eval env first in
      let* _ = demand first in
      eval env second
    | Ast.And (left, right) ->
      let* left = eval env left in
      let* left = demand left in
      let* b = as_bool "&&" left in
      if not b then Ok (Value.bool false) else eval env right
    | Ast.Or (left, right) ->
      let* left = eval env left in
      let* left = demand left in
      let* b = as_bool "||" left in
      if b then Ok (Value.bool true) else eval env right
    | Ast.Arith (op, left, right) ->
      let* left = eval env left in
      let* left = demand left in
      let* right = eval env right in
      let* right = demand right in
      arithmetic op left right
    | Ast.Compare (op, left, right) ->
      let* left = eval env left in
      let* left = demand left in
      let* right = eval env right in
      let* right = demand right in
      comparison op left right
    | Ast.Nil -> Ok Value.nil
    | Ast.Cons (head, tail) ->
      let* head = eval env head in
      let* tail = eval env tail in
      Ok (Value.cons head tail)
    | Ast.Concat (left, right) ->
      let* left = eval env left in
      let* left = demand left in
      let* right = eval env right in
      let* right = demand right in
      (match Value.view left, Value.view right with
       | Value.String a, Value.String b -> Ok (Value.string (a ^ b))
       | _ -> fail "reference: ^ operands are not strings")
    | Ast.Not operand ->
      let* operand = eval env operand in
      let* operand = demand operand in
      let* b = as_bool "not" operand in
      Ok (Value.bool (not b))
    | Ast.Neg operand ->
      let* operand = eval env operand in
      let* operand = demand operand in
      negate operand
    | Ast.Deref reference ->
      let* reference = eval env reference in
      let* reference = demand reference in
      (match Value.view reference with
       | Value.Ref cell -> Ok !cell
       | _ -> fail "reference: ! target is not a reference")
    | Ast.Assign (reference, rhs) ->
      let* reference = eval env reference in
      let* reference = demand reference in
      let* value = eval env rhs in
      let* value = demand value in
      (match Value.view reference with
       | Value.Ref cell ->
         cell := value;
         Ok Value.unit
       | _ -> fail "reference: := target is not a reference")
    | Ast.Make_ref operand ->
      let* operand = eval env operand in
      let* operand = demand operand in
      Ok (Value.ref_value operand)

  and demand v =
    match Value.view v with
    | Value.Thunk cell ->
      counts.forces <- counts.forces + 1;
      (match !cell with
       | Value.Forced memo ->
         counts.memo_hits <- counts.memo_hits + 1;
         Ok memo
       | Value.Delayed (expr, env) ->
         counts.recomputations <- counts.recomputations + 1;
         let* forced = eval env expr in
         let* forced = demand forced in
         Value.set_thunk_state cell (Value.Forced forced);
         Ok forced)
    | _ -> Ok v

  and eval_let env is_rec bindings body =
    if is_rec
    then (
      let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
      let env, cells = Env.extend_recursive names env in
      let rec fill cells = function
        | [] -> Ok ()
        | (b : Ast.binding) :: rest ->
          let* v =
            match b.name with
            | None ->
              let* v = eval env b.rhs in
              demand v
            | Some _ ->
              counts.allocations <- counts.allocations + 1;
              Ok (Value.thunk ~expr:b.rhs ~env)
          in
          let cells =
            match b.name, cells with
            | Some _, cell :: cells ->
              Env.fill cell v;
              cells
            | _, cells -> cells
          in
          fill cells rest
      in
      let* () = fill cells bindings in
      eval env body)
    else (
      let rec gather acc = function
        | [] -> Ok (List.rev acc)
        | (b : Ast.binding) :: rest ->
          let* v =
            match b.name with
            | None ->
              let* v = eval env b.rhs in
              demand v
            | Some _ ->
              counts.allocations <- counts.allocations + 1;
              Ok (Value.thunk ~expr:b.rhs ~env)
          in
          gather (v :: acc) rest
      in
      let* values = gather [] bindings in
      let named =
        List.filter_map
          (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
          (List.combine bindings values)
      in
      eval (Env.extend named env) body)

  and eval_cases env v cases =
    match cases with
    | [] -> fail "reference: no case matched"
    | (pattern, body) :: rest ->
      (match match_pattern_forcing ~demand pattern v with
       | Ok (Some bindings) -> eval (Env.extend bindings env) body
       | Ok None -> eval_cases env v rest
       | Error e -> Error e)

  and apply_expr env fn args =
    match Ast.view fn with
    | Ast.Var "delay" ->
      (match args with
       | [ expr ] ->
         counts.allocations <- counts.allocations + 1;
         Ok (Value.thunk ~expr ~env)
       | _ -> fail "reference: delay takes one argument")
    | Ast.Var "force" ->
      (match args with
       | [ expr ] ->
         let* v = eval env expr in
         demand v
       | _ -> fail "reference: force takes one argument")
    | _ ->
      let* fn = eval env fn in
      let* fn = demand fn in
      apply_value env fn args

  and apply_value caller_env fn args =
    match args with
    | [] -> Ok fn
    | arg :: rest ->
      (match Value.view fn with
       | Value.Closure { parameters = parameter :: parameters; body; env; _ } ->
         counts.allocations <- counts.allocations + 1;
         let env = Env.extend [ parameter, Value.thunk ~expr:arg ~env:caller_env ] env in
         if parameters = []
         then
           let* result = eval env body in
           apply_value caller_env result rest
         else
           apply_value caller_env (Value.closure ~name:None ~parameters ~body ~env) rest
       | Value.Primitive p ->
         let* values = strict_args caller_env (arg :: rest) in
         apply_strict p [] values
       | Value.Partial (p, gathered) ->
         let* values = strict_args caller_env (arg :: rest) in
         apply_strict p gathered values
       | _ -> fail "reference: not applicable")

  and strict_args caller_env exprs =
    let rec go acc = function
      | [] -> Ok (List.rev acc)
      | e :: rest ->
        let* v = eval caller_env e in
        let* v = demand v in
        go (v :: acc) rest
    in
    go [] exprs

  and apply_strict p gathered values =
    let missing = p.Value.prim_arity - List.length gathered in
    if List.length values < missing
    then Ok (Value.partial p (gathered @ values))
    else (
      let rec split n acc = function
        | rest when n = 0 -> List.rev acc, rest
        | x :: rest -> split (n - 1) (x :: acc) rest
        | [] -> List.rev acc, []
      in
      let taken, rest = split missing [] values in
      let rec apply_done fn values =
        match values with
        | [] -> Ok fn
        | value :: rest ->
          (match Value.view fn with
           | Value.Closure { parameters = parameter :: parameters; body; env; _ } ->
             let env = Env.extend [ parameter, Value.forced value ] env in
             if parameters = []
             then
               let* result = eval env body in
               apply_done result rest
             else apply_done (Value.closure ~name:None ~parameters ~body ~env) rest
           | Value.Primitive p -> apply_strict p [] values
           | Value.Partial (p, gathered) -> apply_strict p gathered values
           | _ -> fail "reference: not applicable beneath a primitive")
      in
      let callback f vs =
        Result.map_error (fun m -> Eval_error.User_error m) (apply_done f vs)
      in
      let* result =
        Result.map_error
          Eval_error.to_string
          (p.Value.prim_apply callback (gathered @ taken))
      in
      apply_done result rest)
  ;;

  let execute ~emit program =
    reset ();
    let rec go env = function
      | [] -> Ok ()
      | (item : Ast.item) :: rest ->
        (match item with
         | Ast.Type_item _ -> go env rest
         | Ast.Value_item (is_rec, bindings) ->
           if is_rec
           then (
             let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
             let env, cells = Env.extend_recursive names env in
             let rec fill cells = function
               | [] -> Ok ()
               | (b : Ast.binding) :: rest ->
                 let* v = eval env b.rhs in
                 let* v = demand v in
                 let cells =
                   match b.name, cells with
                   | Some _, cell :: cells ->
                     Env.fill cell v;
                     cells
                   | _, cells -> cells
                 in
                 fill cells rest
             in
             let* () = fill cells bindings in
             go env rest)
           else (
             let rec strict acc = function
               | [] -> Ok (List.rev acc)
               | (b : Ast.binding) :: rest ->
                 let* v = eval env b.rhs in
                 let* v = demand v in
                 strict (v :: acc) rest
             in
             let* values = strict [] bindings in
             let named =
               List.filter_map
                 (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
                 (List.combine bindings values)
             in
             go (Env.extend named env) rest))
    in
    let* () = go (Prelude.initial_env ~emit ()) (Check.items program) in
    emit
      (Printf.sprintf
         "thunk-allocations: %d\n\
          thunk-forces: %d\n\
          thunk-recomputations: %d\n\
          thunk-memo-hits: %d\n"
         counts.allocations
         counts.forces
         counts.recomputations
         counts.memo_hits);
    Ok ()
  ;;
end

(* {1 Search model: decision-tree enumeration with per-branch effects} *)

module Search_model = struct
  type stop =
    | Dead
    | Open of int
    | Broke of Eval_error.t

  type 'a step = ('a, stop) result
  type walk = { mutable decisions : int list }

  let broke detail = Error (Broke (Eval_error.Type_error ("reference: " ^ detail)))

  let truth what v =
    match Value.view v with
    | Value.Bool b -> Ok b
    | _ -> broke (what ^ " condition is not a bool")
  ;;

  let lift = function
    | Ok v -> Ok v
    | Error detail -> broke detail
  ;;

  let rec eval walk env (e : Ast.expr) : Value.t step =
    match Ast.view e with
    | Ast.Scalar s -> Ok (scalar_value s)
    | Ast.Var name ->
      (match Env.find env name with
       | Some v -> Ok v
       | None -> Error (Broke (Eval_error.Unbound_variable name)))
    | Ast.Let (is_rec, bindings, body) ->
      if is_rec
      then (
        let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
        let env, cells = Env.extend_recursive names env in
        let rec fill cells = function
          | [] -> Ok ()
          | (b : Ast.binding) :: rest ->
            let* v = eval walk env b.rhs in
            let cells =
              match b.name, cells with
              | Some _, cell :: cells ->
                Env.fill cell v;
                cells
              | _, cells -> cells
            in
            fill cells rest
        in
        let* () = fill cells bindings in
        eval walk env body)
      else (
        let rec gather acc = function
          | [] -> Ok (List.rev acc)
          | (b : Ast.binding) :: rest ->
            let* v = eval walk env b.rhs in
            gather (v :: acc) rest
        in
        let* values = gather [] bindings in
        let named =
          List.filter_map
            (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
            (List.combine bindings values)
        in
        eval walk (Env.extend named env) body)
    | Ast.Fun (parameters, body) -> Ok (Value.closure ~name:None ~parameters ~body ~env)
    | Ast.Apply (fn, args) -> apply_from_expr walk env fn args
    | Ast.If (condition, consequent, alternative) ->
      let* condition = eval walk env condition in
      let* b = truth "if" condition in
      if b then eval walk env consequent else eval walk env alternative
    | Ast.Match (scrutinee, cases) ->
      let* v = eval walk env scrutinee in
      (match
         List.find_map
           (fun (pattern, body) ->
              match match_pattern pattern v with
              | None -> None
              | Some bindings -> Some (Env.extend bindings env, body))
           cases
       with
       | Some (env, body) -> eval walk env body
       | None -> broke "no case matched")
    | Ast.Tuple parts ->
      let rec gather acc = function
        | [] -> Ok (Value.tuple (List.rev acc))
        | e :: rest ->
          let* v = eval walk env e in
          gather (v :: acc) rest
      in
      gather [] parts
    | Ast.Construct (name, fields) ->
      let rec gather acc = function
        | [] -> Ok (Value.construct name (List.rev acc))
        | e :: rest ->
          let* v = eval walk env e in
          gather (v :: acc) rest
      in
      gather [] fields
    | Ast.Record fields ->
      let rec gather acc = function
        | [] -> Ok (Value.record (List.rev acc))
        | (name, e) :: rest ->
          let* v = eval walk env e in
          gather ((name, v) :: acc) rest
      in
      gather [] fields
    | Ast.Field (record, name) ->
      let* record = eval walk env record in
      (match Value.view record with
       | Value.Record fields ->
         (match List.assoc_opt name fields with
          | Some v -> Ok v
          | None -> broke ("absent field " ^ name))
       | _ -> broke "field target is not a record")
    | Ast.Sequence (first, second) ->
      let* _ = eval walk env first in
      eval walk env second
    | Ast.And (left, right) ->
      let* left = eval walk env left in
      let* b = truth "&&" left in
      if not b then Ok (Value.bool false) else eval walk env right
    | Ast.Or (left, right) ->
      let* left = eval walk env left in
      let* b = truth "||" left in
      if b then Ok (Value.bool true) else eval walk env right
    | Ast.Arith (op, left, right) ->
      let* left = eval walk env left in
      let* right = eval walk env right in
      lift (arithmetic op left right)
    | Ast.Compare (op, left, right) ->
      let* left = eval walk env left in
      let* right = eval walk env right in
      lift (comparison op left right)
    | Ast.Nil -> Ok Value.nil
    | Ast.Cons (head, tail) ->
      let* head = eval walk env head in
      let* tail = eval walk env tail in
      Ok (Value.cons head tail)
    | Ast.Concat (left, right) ->
      let* left = eval walk env left in
      let* right = eval walk env right in
      (match Value.view left, Value.view right with
       | Value.String a, Value.String b -> Ok (Value.string (a ^ b))
       | _ -> broke "^ operands are not strings")
    | Ast.Not operand ->
      let* operand = eval walk env operand in
      let* b = truth "not" operand in
      Ok (Value.bool (not b))
    | Ast.Neg operand ->
      let* operand = eval walk env operand in
      lift (negate operand)
    | Ast.Deref reference ->
      let* reference = eval walk env reference in
      (match Value.view reference with
       | Value.Ref cell -> Ok !cell
       | _ -> broke "! target is not a reference")
    | Ast.Assign (reference, rhs) ->
      let* reference = eval walk env reference in
      let* value = eval walk env rhs in
      (match Value.view reference with
       | Value.Ref cell ->
         cell := value;
         Ok Value.unit
       | _ -> broke ":= target is not a reference")
    | Ast.Make_ref operand ->
      let* operand = eval walk env operand in
      Ok (Value.ref_value operand)

  and apply_from_expr walk env fn args =
    match Ast.view fn with
    | Ast.Var "amb" ->
      let arity = List.length args in
      if arity = 0
      then broke "amb takes alternatives"
      else (
        match walk.decisions with
        | choice :: rest when choice >= 0 && choice < arity ->
          walk.decisions <- rest;
          eval walk env (List.nth args choice)
        | _ :: _ -> Error Dead
        | [] -> Error (Open arity))
    | Ast.Var "require" ->
      (match args with
       | [ condition ] ->
         let* v = eval walk env condition in
         let* b = truth "require" v in
         if b then Ok Value.unit else Error Dead
       | _ ->
         Error
           (Broke (Eval_error.Arity_mismatch { expected = 1; given = List.length args })))
    | _ ->
      let* fn = eval walk env fn in
      let rec gather acc = function
        | [] -> Ok (List.rev acc)
        | e :: rest ->
          let* v = eval walk env e in
          gather (v :: acc) rest
      in
      let* values = gather [] args in
      apply walk fn values

  and apply walk fn values =
    match values with
    | [] -> Ok fn
    | value :: rest ->
      (match Value.view fn with
       | Value.Closure { parameters = parameter :: parameters; body; env; _ } ->
         let env = Env.extend [ parameter, value ] env in
         if parameters = []
         then
           let* result = eval walk env body in
           apply walk result rest
         else apply walk (Value.closure ~name:None ~parameters ~body ~env) rest
       | Value.Primitive p -> strict walk p [] (value :: rest)
       | Value.Partial (p, gathered) -> strict walk p gathered (value :: rest)
       | _ -> Error (Broke (Eval_error.Not_applicable (Value.to_string fn))))

  and strict walk p gathered values =
    let missing = p.Value.prim_arity - List.length gathered in
    if List.length values < missing
    then Ok (Value.partial p (gathered @ values))
    else (
      let rec split n acc = function
        | rest when n = 0 -> List.rev acc, rest
        | x :: rest -> split (n - 1) (x :: acc) rest
        | [] -> List.rev acc, []
      in
      let taken, rest = split missing [] values in
      let deterministic fn values =
        match apply { decisions = [] } fn values with
        | Ok v -> Ok v
        | Error Dead ->
          Error
            (Eval_error.User_error "reference: a search branch failed beneath a primitive")
        | Error (Open _) ->
          Error
            (Eval_error.User_error
               "reference: a choice point beneath a primitive is outside the experiment")
        | Error (Broke e) -> Error e
      in
      let* result =
        match p.Value.prim_apply deterministic (gathered @ taken) with
        | Ok v -> Ok v
        | Error e -> Error (Broke e)
      in
      apply walk result rest)
  ;;

  let attempt decisions ~emit program =
    let walk = { decisions } in
    let rec go env = function
      | [] -> Ok ()
      | (item : Ast.item) :: rest ->
        (match item with
         | Ast.Type_item _ -> go env rest
         | Ast.Value_item (is_rec, bindings) ->
           if is_rec
           then (
             let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
             let env, cells = Env.extend_recursive names env in
             let rec fill cells = function
               | [] -> Ok ()
               | (b : Ast.binding) :: rest ->
                 let* v = eval walk env b.rhs in
                 let cells =
                   match b.name, cells with
                   | Some _, cell :: cells ->
                     Env.fill cell v;
                     cells
                   | _, cells -> cells
                 in
                 fill cells rest
             in
             let* () = fill cells bindings in
             go env rest)
           else (
             let rec gather acc = function
               | [] -> Ok (List.rev acc)
               | (b : Ast.binding) :: rest ->
                 let* v = eval walk env b.rhs in
                 gather (v :: acc) rest
             in
             let* values = gather [] bindings in
             let named =
               List.filter_map
                 (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
                 (List.combine bindings values)
             in
             go (Env.extend named env) rest))
    in
    go (Prelude.initial_env ~emit ()) (Check.items program)
  ;;

  let execute ~emit program =
    let rec loop pending answers choices failures =
      match pending with
      | [] -> Ok (answers, choices, failures)
      | path :: rest ->
        let branch = Buffer.create 128 in
        (match attempt path ~emit:(Buffer.add_string branch) program with
         | Ok () ->
           emit (Buffer.contents branch);
           loop rest (answers + 1) choices failures
         | Error Dead -> loop rest answers choices (failures + 1)
         | Error (Open arity) ->
           let branches = List.init arity (fun i -> path @ [ i ]) in
           loop (branches @ rest) answers (choices + 1) failures
         | Error (Broke e) -> Error e)
    in
    match loop [ [] ] 0 0 0 with
    | Error e -> Error (Eval_error.to_string e)
    | Ok (answers, choices, failures) ->
      emit
        (Printf.sprintf
           "answers: %d\nchoices: %d\nfailures: %d\n"
           answers
           choices
           failures);
      Ok ()
  ;;
end

(* {1 Query model} *)

module Query_model = struct
  (* An independent query model: its own term type decoded straight from
     the constructor-literal fixture, its own frames and unifier with
     the occurs check, and its own memoized lazy answer lists.  It shares
     the literal reader and the documented order and rendering rules
     with the teaching engine, nothing else. *)
  module D = Sicp_common.Constructor_data

  type t =
    | Sym of string
    | Int of int
    | Text of string
    | V of string * int
    | Empty
    | Cell of t * t

  type q =
    | Match of t
    | All of q list
    | Any of q list
    | Without of q
    | Test of string * t list
    | Yes

  type 'a answers =
    | Stop
    | Next of 'a * 'a answers Lazy.t

  let error fmt = Printf.ksprintf (fun s -> Error ("reference query: " ^ s)) fmt

  exception Broken of string

  let rec each f = function
    | [] -> Ok []
    | x :: rest ->
      let* y = f x in
      let* ys = each f rest in
      Ok (y :: ys)
  ;;

  let rec term = function
    | D.Ctor ("Atom", [ D.String a ]) -> Ok (Sym a)
    | D.Ctor ("Num", [ D.Int n ]) -> Ok (Int n)
    | D.Ctor ("Str", [ D.String s ]) -> Ok (Text s)
    | D.Ctor ("Var", [ D.String v ]) -> Ok (V (v, 0))
    | D.Ctor ("List", [ D.List items ]) ->
      let* items = each term items in
      Ok (List.fold_right (fun h t -> Cell (h, t)) items Empty)
    | D.Ctor ("Dotted", [ D.List items; tail ]) ->
      let* items = each term items in
      let* tail = term tail in
      Ok (List.fold_right (fun h t -> Cell (h, t)) items tail)
    | d -> error "not a term: %s" (D.describe d)
  ;;

  let rec query = function
    | D.Ctor ("Pattern", [ x ]) -> Result.map (fun x -> Match x) (term x)
    | D.Ctor ("And", [ D.List qs ]) -> Result.map (fun qs -> All qs) (each query qs)
    | D.Ctor ("Or", [ D.List qs ]) -> Result.map (fun qs -> Any qs) (each query qs)
    | D.Ctor ("Not", [ x ]) -> Result.map (fun x -> Without x) (query x)
    | D.Ctor ("Holds", [ D.String p; D.List ts ]) ->
      Result.map (fun ts -> Test (p, ts)) (each term ts)
    | D.Ctor ("Always_true", []) -> Ok Yes
    | d -> error "not a query: %s" (D.describe d)
  ;;

  let lookup (name, id) frame =
    List.find_map
      (fun ((n, i), x) -> if i = id && String.equal n name then Some x else None)
      frame
  ;;

  let rec walk x frame =
    match x with
    | V (n, i) ->
      (match lookup (n, i) frame with
       | Some bound -> walk bound frame
       | None -> x)
    | _ -> x
  ;;

  let rec occurs (n, i) x frame =
    match walk x frame with
    | V (m, j) -> j = i && String.equal m n
    | Cell (a, b) -> occurs (n, i) a frame || occurs (n, i) b frame
    | _ -> false
  ;;

  let rec unify a b frame =
    match walk a frame, walk b frame with
    | V (n, i), V (m, j) when i = j && String.equal n m -> Some frame
    | V (n, i), other | other, V (n, i) ->
      if occurs (n, i) other frame then None else Some (((n, i), other) :: frame)
    | Cell (a1, a2), Cell (b1, b2) -> Option.bind (unify a1 b1 frame) (unify a2 b2)
    | Sym x, Sym y -> if String.equal x y then Some frame else None
    | Int x, Int y -> if x = y then Some frame else None
    | Text x, Text y -> if String.equal x y then Some frame else None
    | Empty, Empty -> Some frame
    | _ -> None
  ;;

  let rec resolve x frame =
    match walk x frame with
    | Cell (a, b) -> Cell (resolve a frame, resolve b frame)
    | other -> other
  ;;

  let rec concat_later a later =
    match a with
    | Stop -> Lazy.force later
    | Next (x, rest) -> Next (x, lazy (concat_later (Lazy.force rest) later))
  ;;

  let rec alternate a later =
    match a with
    | Stop -> Lazy.force later
    | Next (x, rest) -> Next (x, lazy (alternate (Lazy.force later) rest))
  ;;

  let rec flatten = function
    | Stop -> Stop
    | Next (inner, rest) -> alternate inner (lazy (flatten (Lazy.force rest)))
  ;;

  let rec map f = function
    | Stop -> Stop
    | Next (x, rest) -> Next (f x, lazy (map f (Lazy.force rest)))
  ;;

  let rec keep p = function
    | Stop -> Stop
    | Next (x, rest) ->
      if p x then Next (x, lazy (keep p (Lazy.force rest))) else keep p (Lazy.force rest)
  ;;

  let rec of_list = function
    | [] -> Stop
    | x :: rest -> Next (x, lazy (of_list rest))
  ;;

  let one x = Next (x, lazy Stop)

  let rec rename id = function
    | V (n, _) -> V (n, id)
    | Cell (a, b) -> Cell (rename id a, rename id b)
    | x -> x
  ;;

  let rec rename_q id = function
    | Match x -> Match (rename id x)
    | All qs -> All (List.map (rename_q id) qs)
    | Any qs -> Any (List.map (rename_q id) qs)
    | Without x -> Without (rename_q id x)
    | Test (p, ts) -> Test (p, List.map (rename id) ts)
    | Yes -> Yes
  ;;

  let holds p args =
    let order a b =
      match a, b with
      | Int x, Int y -> compare x y
      | Text x, Text y -> compare x y
      | _ -> raise (Broken ("holds " ^ p ^ " compares numbers or strings"))
    in
    match p, args with
    | "<", [ a; b ] -> order a b < 0
    | ">", [ a; b ] -> order a b > 0
    | "<=", [ a; b ] -> order a b <= 0
    | ">=", [ a; b ] -> order a b >= 0
    | "=", [ a; b ] -> order a b = 0
    | "<>", [ a; b ] -> order a b <> 0
    | _ -> raise (Broken ("unknown predicate " ^ p))
  ;;

  let rec ground = function
    | V (n, _) -> raise (Broken ("holds on unbound ?" ^ n))
    | Cell (a, b) ->
      ground a;
      ground b
    | _ -> ()
  ;;

  (* [solve] follows the documented order: assertions before rules for a
     simple query, conjuncts in series, disjuncts interleaved, and the
     book's interleaved flattening wherever a stream of streams arises. *)
  let solve facts rules =
    let counter = ref 0 in
    let rec eval q frames =
      match q with
      | Match p -> flatten (map (fun frame -> simple p frame) frames)
      | All qs -> List.fold_left (fun frames q -> eval q frames) frames qs
      | Any qs -> disjoin qs frames
      | Without x -> keep (fun frame -> eval x (one frame) = Stop) frames
      | Test (p, ts) ->
        keep
          (fun frame ->
             let args = List.map (fun x -> resolve x frame) ts in
             List.iter ground args;
             holds p args)
          frames
      | Yes -> frames
    and disjoin qs frames =
      match qs with
      | [] -> Stop
      | q :: rest -> alternate (eval q frames) (lazy (disjoin rest frames))
    and simple p frame =
      let from_facts =
        flatten
          (map
             (fun fact ->
                match unify p fact frame with
                | Some f -> one f
                | None -> Stop)
             (of_list facts))
      in
      concat_later
        from_facts
        (lazy
          (flatten
             (map
                (fun (head, body) ->
                   incr counter;
                   let id = !counter in
                   match unify p (rename id head) frame with
                   | Some f -> eval (rename_q id body) (one f)
                   | None -> Stop)
                (of_list rules))))
    in
    eval
  ;;

  let rec show = function
    | Sym a -> a
    | Int n -> string_of_int n
    | Text s -> Printf.sprintf "%S" s
    | V (n, 0) -> "?" ^ n
    | V (n, i) -> Printf.sprintf "?%s.%d" n i
    | Empty -> "[]"
    | Cell _ as x ->
      let rec go acc = function
        | Cell (h, t) -> go (show h :: acc) t
        | Empty -> "[" ^ String.concat ", " (List.rev acc) ^ "]"
        | tail -> "[" ^ String.concat ", " (List.rev acc) ^ " | " ^ show tail ^ "]"
      in
      go [] x
  ;;

  let rec show_q = function
    | Match x -> show x
    | All qs -> "and(" ^ String.concat ", " (List.map show_q qs) ^ ")"
    | Any qs -> "or(" ^ String.concat ", " (List.map show_q qs) ^ ")"
    | Without x -> "not(" ^ show_q x ^ ")"
    | Test (p, ts) -> "holds(" ^ String.concat ", " (p :: List.map show ts) ^ ")"
    | Yes -> "always-true"
  ;;

  (* Rule variables left unbound in an answer print numbered by first
     occurrence, per the documented rendering rule. *)
  let answer_text q frame =
    let seen = ref [] in
    let rec fix x =
      match resolve x frame with
      | V (n, i) when i <> 0 ->
        (match List.assoc_opt (n, i) !seen with
         | Some k -> V (n, k)
         | None ->
           let k = List.length !seen + 1 in
           seen := !seen @ [ (n, i), k ];
           V (n, k))
      | Cell (a, b) ->
        let a = fix a in
        Cell (a, fix b)
      | other -> other
    in
    let rec fix_q = function
      | Match x -> Match (fix x)
      | All qs -> All (List.map fix_q qs)
      | Any qs -> Any (List.map fix_q qs)
      | Without x -> Without (fix_q x)
      | Test (p, ts) -> Test (p, List.map fix ts)
      | Yes -> Yes
    in
    show_q (fix_q q)
  ;;

  let execute ~emit source =
    let* data =
      Result.map_error
        (fun s -> "reference query: " ^ s)
        (D.read ~filename:"query" source)
    in
    let* commands =
      match data with
      | D.List commands -> Ok commands
      | d -> error "not a command list: %s" (D.describe d)
    in
    let facts = ref [] in
    let rules = ref [] in
    let rec go = function
      | [] -> Ok ()
      | D.Ctor ("Assert", [ x ]) :: rest ->
        let* x = term x in
        facts := !facts @ [ x ];
        go rest
      | D.Ctor ("Rule", [ head; body ]) :: rest ->
        let* head = term head in
        let* body = query body in
        rules := !rules @ [ head, body ];
        go rest
      | D.Ctor ("Query", [ q ]) :: rest ->
        let* q = query q in
        emit ("? " ^ show_q q ^ "\n");
        let rec print = function
          | Stop -> ()
          | Next (frame, more) ->
            emit (answer_text q frame ^ "\n");
            print (Lazy.force more)
        in
        (match print (solve !facts !rules q (one [])) with
         | () -> go rest
         | exception Broken message -> error "%s" message)
      | d :: _ -> error "not a command: %s" (D.describe d)
    in
    go commands
  ;;
end

(* {1 Machine model} *)

module Machine_model = struct
  (* An independent register-machine model: its own decoding of the
     constructor-literal fixture, its own register file and stack, and
     its own operation meanings.  It shares only the literal reader and
     the observation format with the teaching simulator. *)
  module D = Sicp_common.Constructor_data

  type word =
    | W_int of int
    | W_float of float
    | W_bool of bool
    | W_string of string
    | W_label of string
    | W_unset

  let show = function
    | W_int n -> string_of_int n
    | W_float f -> string_of_float f
    | W_bool b -> if b then "true" else "false"
    | W_string s -> Printf.sprintf "%S" s
    | W_label l -> l
    | W_unset -> "unassigned"
  ;;

  type operand =
    | O_const of word
    | O_reg of string
    | O_label of string

  type step =
    | S_label of string
    | S_assign of string * operand
    | S_compute of string * string * operand list
    | S_test of string * operand list
    | S_branch of string
    | S_goto of string
    | S_goto_reg of string
    | S_save of string
    | S_restore of string
    | S_perform of string * operand list

  let error fmt = Printf.ksprintf (fun s -> Error ("reference machine: " ^ s)) fmt

  let rec each f = function
    | [] -> Ok []
    | x :: rest ->
      let* y = f x in
      let* ys = each f rest in
      Ok (y :: ys)
  ;;

  let word = function
    | D.Ctor ("Int", [ D.Int n ]) -> Ok (W_int n)
    | D.Ctor ("Float", [ D.Float f ]) -> Ok (W_float f)
    | D.Ctor ("Bool", [ D.Ctor ("true", []) ]) -> Ok (W_bool true)
    | D.Ctor ("Bool", [ D.Ctor ("false", []) ]) -> Ok (W_bool false)
    | D.Ctor ("Str", [ D.String s ]) -> Ok (W_string s)
    | d -> error "not a word: %s" (D.describe d)
  ;;

  let operand = function
    | D.Ctor ("Const", [ w ]) -> Result.map (fun w -> O_const w) (word w)
    | D.Ctor ("Reg", [ D.String r ]) -> Ok (O_reg r)
    | D.Ctor ("Label_ref", [ D.String l ]) -> Ok (O_label l)
    | d -> error "not an operand: %s" (D.describe d)
  ;;

  let operands = function
    | D.List items -> each operand items
    | d -> error "not an operand list: %s" (D.describe d)
  ;;

  let instruction = function
    | D.Ctor ("Label", [ D.String l ]) -> Ok (S_label l)
    | D.Ctor ("Assign", [ D.String r; o ]) ->
      Result.map (fun o -> S_assign (r, o)) (operand o)
    | D.Ctor ("Assign_op", [ D.String r; D.String op; os ]) ->
      Result.map (fun os -> S_compute (r, op, os)) (operands os)
    | D.Ctor ("Test", [ D.String op; os ]) ->
      Result.map (fun os -> S_test (op, os)) (operands os)
    | D.Ctor ("Branch", [ D.String l ]) -> Ok (S_branch l)
    | D.Ctor ("Goto", [ D.String l ]) -> Ok (S_goto l)
    | D.Ctor ("Goto_reg", [ D.String r ]) -> Ok (S_goto_reg r)
    | D.Ctor ("Save", [ D.String r ]) -> Ok (S_save r)
    | D.Ctor ("Restore", [ D.String r ]) -> Ok (S_restore r)
    | D.Ctor ("Perform", [ D.String op; os ]) ->
      Result.map (fun os -> S_perform (op, os)) (operands os)
    | d -> error "not an instruction: %s" (D.describe d)
  ;;

  type kind =
    | K_int
    | K_float
    | K_bool
    | K_unit

  let kind = function
    | D.Ctor ("Int_type", []) -> Ok K_int
    | D.Ctor ("Float_type", []) -> Ok K_float
    | D.Ctor ("Bool_type", []) -> Ok K_bool
    | D.Ctor ("Unit_type", []) -> Ok K_unit
    | d -> error "not an operation type: %s" (D.describe d)
  ;;

  let declaration = function
    | D.Tuple [ D.String name; D.List args; result ] ->
      let* args = each kind args in
      let* result = kind result in
      Ok (name, (args, result))
    | d -> error "not an operation declaration: %s" (D.describe d)
  ;;

  let fits k w =
    match k, w with
    | K_int, W_int _ | K_float, W_float _ | K_bool, W_bool _ -> true
    | _ -> false
  ;;

  (* The operation meanings of grammar section 6 over the declared
     types; a result is checked against the declaration. *)
  let meaning ~emit name args =
    match name, args with
    | "+", [ W_int a; W_int b ] -> Ok (W_int (a + b))
    | "-", [ W_int a; W_int b ] -> Ok (W_int (a - b))
    | "*", [ W_int a; W_int b ] -> Ok (W_int (a * b))
    | ("/" | "rem"), [ W_int _; W_int 0 ] -> error "division by zero"
    | "/", [ W_int a; W_int b ] -> Ok (W_int (a / b))
    | "rem", [ W_int a; W_int b ] -> Ok (W_int (a mod b))
    | "+.", [ W_float a; W_float b ] -> Ok (W_float (a +. b))
    | "-.", [ W_float a; W_float b ] -> Ok (W_float (a -. b))
    | "*.", [ W_float a; W_float b ] -> Ok (W_float (a *. b))
    | "/.", [ W_float a; W_float b ] -> Ok (W_float (a /. b))
    | "=", [ W_int a; W_int b ] -> Ok (W_bool (a = b))
    | "<", [ W_int a; W_int b ] -> Ok (W_bool (a < b))
    | ">", [ W_int a; W_int b ] -> Ok (W_bool (a > b))
    | "<=", [ W_int a; W_int b ] -> Ok (W_bool (a <= b))
    | ">=", [ W_int a; W_int b ] -> Ok (W_bool (a >= b))
    | "=", [ W_float a; W_float b ] -> Ok (W_bool (Float.equal a b))
    | "<", [ W_float a; W_float b ] -> Ok (W_bool (a < b))
    | ">", [ W_float a; W_float b ] -> Ok (W_bool (a > b))
    | "abs", [ W_int a ] -> Ok (W_int (abs a))
    | "sqrt", [ W_float a ] -> Ok (W_float (sqrt a))
    | "float_of_int", [ W_int a ] -> Ok (W_float (float_of_int a))
    | "print", [ w ] ->
      emit (show w ^ "\n");
      Ok W_unset
    | _ -> error "no meaning for %s on %s" name (String.concat ", " (List.map show args))
  ;;

  let execute ~emit source =
    let* data =
      Result.map_error
        (fun s -> "reference machine: " ^ s)
        (D.read ~filename:"machine" source)
    in
    let* fields =
      match data with
      | D.Ctor ("Machine", [ D.Record fields ]) -> Ok fields
      | d -> error "not a machine: %s" (D.describe d)
    in
    let get name =
      match List.assoc_opt name fields with
      | Some (D.List items) -> Ok items
      | _ -> error "missing list field %s" name
    in
    let* registers =
      Result.bind
        (get "registers")
        (each (function
           | D.String r -> Ok r
           | d -> error "register %s" (D.describe d)))
    in
    let* declared = Result.bind (get "operations") (each declaration) in
    let* inputs =
      Result.bind
        (get "inputs")
        (each (function
           | D.Tuple [ D.String r; w ] -> Result.map (fun w -> r, w) (word w)
           | d -> error "input %s" (D.describe d)))
    in
    let* steps = Result.bind (get "controller") (each instruction) in
    let code = Array.of_list steps in
    let target l =
      let rec find i =
        if i >= Array.length code
        then error "unknown label %s" l
        else (
          match code.(i) with
          | S_label m when String.equal m l -> Ok i
          | _ -> find (i + 1))
      in
      find 0
    in
    let file = Hashtbl.create 8 in
    List.iter (fun r -> Hashtbl.replace file r W_unset) registers;
    let read r =
      match Hashtbl.find_opt file r with
      | Some w -> Ok w
      | None -> error "unknown register %s" r
    in
    let write r w =
      if Hashtbl.mem file r
      then Ok (Hashtbl.replace file r w)
      else error "unknown register %s" r
    in
    let* () =
      List.fold_left
        (fun acc (r, w) -> Result.bind acc (fun () -> write r w))
        (Ok ())
        inputs
    in
    let value = function
      | O_const w -> Ok w
      | O_reg r -> read r
      | O_label l -> Result.map (fun _ -> W_label l) (target l)
    in
    let call name os =
      match List.assoc_opt name declared with
      | None -> error "undeclared operation %s" name
      | Some (kinds, result) ->
        let* args = each value os in
        if List.length kinds <> List.length args || not (List.for_all2 fits kinds args)
        then error "%s applied outside its declared types" name
        else
          let* w = meaning ~emit name args in
          if result = K_unit || fits result w
          then Ok w
          else error "%s answered outside its type" name
    in
    let rec run pc stack flag =
      if pc >= Array.length code
      then Ok ()
      else (
        match code.(pc) with
        | S_label _ -> run (pc + 1) stack flag
        | S_assign (r, o) ->
          let* w = value o in
          let* () = write r w in
          run (pc + 1) stack flag
        | S_compute (r, op, os) ->
          let* w = call op os in
          let* () = write r w in
          run (pc + 1) stack flag
        | S_test (op, os) ->
          let* w = call op os in
          (match w with
           | W_bool b -> run (pc + 1) stack (Some b)
           | _ -> error "test %s is not a predicate" op)
        | S_branch l ->
          (match flag with
           | Some true -> Result.bind (target l) (fun pc -> run pc stack flag)
           | Some false -> run (pc + 1) stack flag
           | None -> error "branch without test")
        | S_goto l -> Result.bind (target l) (fun pc -> run pc stack flag)
        | S_goto_reg r ->
          (match read r with
           | Ok (W_label l) -> Result.bind (target l) (fun pc -> run pc stack flag)
           | Ok w -> error "goto through %s" (show w)
           | Error e -> Error e)
        | S_save r ->
          let* w = read r in
          run (pc + 1) (w :: stack) flag
        | S_restore r ->
          (match stack with
           | w :: rest ->
             let* () = write r w in
             run (pc + 1) rest flag
           | [] -> error "restore from an empty stack")
        | S_perform (op, os) ->
          let* _ = call op os in
          run (pc + 1) stack flag)
    in
    let* () = run 0 [] None in
    List.iter
      (fun r ->
         emit
           (r
            ^ ": "
            ^ show (Option.value (Hashtbl.find_opt file r) ~default:W_unset)
            ^ "\n"))
      registers;
    Ok ()
  ;;
end

let run ~emit ~capability source_path =
  let source = read_file source_path in
  match capability with
  | "lazy" ->
    (match Check.check_experiment ~experiment:Lazy ~filename:source_path source with
     | Error diagnostic -> fail (Check.diagnostic_to_string diagnostic)
     | Ok program -> Lazy_model.execute ~emit program)
  | "amb" ->
    (match Check.check_experiment ~experiment:Search ~filename:source_path source with
     | Error diagnostic -> fail (Check.diagnostic_to_string diagnostic)
     | Ok program -> Search_model.execute ~emit program)
  | "query" -> Query_model.execute ~emit source
  | "machine" -> Machine_model.execute ~emit source
  | _ -> fail ("reference: no model for capability " ^ capability)
;;
