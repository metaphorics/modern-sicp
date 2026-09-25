(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.2 *)

(** The lazy evaluator of section 4.2 on the 4.1 substrate. The value
    domain gains delayed computations: applying a compound procedure
    delays every operand into a thunk instead of evaluating it, while
    primitives stay strict and quoted data stays ordinary data. A thunk
    is a primitive-shaped value under the reserved name [thunk]; its
    host closure produces the value when the thunk is forced, and a
    memoized thunk remembers that value in a host [Lazy] cell, which is
    the book's evaluated thunk. Every failure travels through
    [Eval_error]; a failure answered while a memoized cell fills is
    cached by the [Lazy] cell and re-answered on every later forcing.
    [Core] is the lazy dispatch, parameterized by the [eval] it recurses
    through exactly as in 4.1; the driver evaluates through
    [actual_value], so a delayed value is forced before it reaches the
    surface. *)

let ( >>= ) = Result.bind

module SE = Sec_4_1
module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

type eval_t = SE.eval_t

(** [true_ v] holds for every value except the false object. *)
let true_ = SE.true_

(* {2 4.2.2: representing thunks}

   A thunk packages an expression with the environment of the
   application that delayed it. The shared [Value] type carries no
   thunk constructor, so the thunk is a primitive-shaped value under
   the reserved name [thunk], a name no primitive table installs; the
   forcing closure is registered beside the value at creation and found
   later by a physical scan, the same shape the 4.1 analyzer uses for
   procedure bodies. *)

(** The reserved name marking a thunk value. *)
let thunk_tag = "thunk"

(** The forcing closure of every thunk created so far, keyed by the
    value itself. A physical scan, because the environment mutates
    under the thunk's captured frame and a hash would change with it. *)
let thunks : (Value.t * (unit -> (Value.t, Eval_error.t) result)) list ref = ref []

(** [is_thunk v] holds exactly for the thunk values this evaluator
    creates. *)
let is_thunk v =
  match Value.view v with
  | Value.Primitive_procedure name -> String.equal name thunk_tag
  | _ -> false
;;

(** [thunk_of force] is the thunk whose forcing runs [force ()] and
    keeps forcing until a non-thunk value comes back. *)
let thunk_of (force : unit -> (Value.t, Eval_error.t) result) : Value.t =
  let v = Value.primitive ~name:thunk_tag (fun _ -> force ()) in
  thunks := (v, force) :: !thunks;
  v
;;

(** [force_value v] is the book's [force-it]: a thunk is forced, and the
    result is forced again in case the expression's value is itself a
    thunk; anything else answers unchanged. *)
let rec force_value (v : Value.t) : (Value.t, Eval_error.t) result =
  if is_thunk v
  then (
    let rec go = function
      | [] -> Error (Eval_error.Type_error "force_value: not a thunk of this evaluator")
      | (w, force) :: rest -> if Value.physical_equal w v then force () else go rest
    in
    go !thunks >>= force_value)
  else Ok v
;;

(** {2 4.1.4: the primitive table of the section}

    The 4.1 table with the arithmetic the section's programs need: [+],
    [-], and [*] take exact integers or inexact floats as the book's
    [scale-list] and [solve] demand, and [/] joins the table with its
    typed zero-divisor error. *)

(** The number of one operand value: the exact integer or inexact float
    the section's arithmetic folds over. *)
let as_number name (v : Value.t) =
  match Value.view v with
  | Value.Int x -> Ok (`Int x)
  | Value.Float x -> Ok (`Float x)
  | _ -> Error (Eval_error.Type_error (name ^ ": the operand is not a number"))
;;

let num2 name (a : Value.t) (b : Value.t) iop fop =
  as_number name a
  >>= fun x ->
  as_number name b
  >>= fun y ->
  match x, y with
  | `Int x, `Int y -> Ok (Value.int (iop x y))
  | `Float x, `Float y -> Ok (Value.float (fop x y))
  | `Int x, `Float y -> Ok (Value.float (fop (float_of_int x) y))
  | `Float x, `Int y -> Ok (Value.float (fop x (float_of_int y)))
;;

let arith name iop fop init =
  ( name
  , fun args ->
      match args with
      | [] -> Ok (init ())
      | first :: rest ->
        as_number name first
        >>= fun _first ->
        List.fold_left
          (fun acc v -> acc >>= fun total -> num2 name total v iop fop)
          (Ok first)
          rest )
;;

let division =
  ( "/"
  , fun args ->
      match args with
      | [ a; b ] ->
        (match Value.view a, Value.view b with
         | Value.Int _, Value.Int 0
         | Value.Float _, Value.Float 0.0
         | Value.Float _, Value.Int 0 -> Error Eval_error.Division_by_zero
         | Value.Int x, Value.Int y -> Ok (Value.int (x / y))
         | Value.Float x, Value.Float y -> Ok (Value.float (x /. y))
         | Value.Float x, Value.Int y -> Ok (Value.float (x /. float_of_int y))
         | Value.Int x, Value.Float y -> Ok (Value.float (float_of_int x /. y))
         | _ -> Error (Eval_error.Type_error "/: the operands are not both numbers"))
      | args ->
        Error (Eval_error.Arity_mismatch { expected = 2; given = List.length args }) )
;;

(** [primitive_table] is the section's table: the 4.1 entries with the
    arithmetic above in front of them. [+], [-], and [*] fold over any
    number of operands, Scheme style: an empty [+] answers 0 and an
    empty [*] answers 1, a one-operand [-] negates. *)
let primitive_table : (string * Value.primitive) list =
  [ arith "+" ( + ) ( +. ) (fun () -> Value.int 0)
  ; ( "-"
    , fun args ->
        match args with
        | [ single ] -> num2 "-" (Value.int 0) single ( - ) ( -. )
        | first :: rest ->
          as_number "-" first
          >>= fun _first ->
          List.fold_left
            (fun acc v -> acc >>= fun total -> num2 "-" total v ( - ) ( -. ))
            (Ok first)
            rest
        | [] -> Error (Eval_error.Arity_mismatch { expected = 1; given = 0 }) )
  ; arith "*" ( * ) ( *. ) (fun () -> Value.int 1)
  ; division
  ]
  @ SE.primitive_table
;;

(** [delay_it ~memo ~eval exp env] is the book's [delay-it] at module
    level, so an exercise evaluator delays its operands through this
    machinery; [Core]'s operands use it with their own [eval]. *)
let delay_it ?(memo = true) ~(eval : eval_t) exp env =
  if memo
  then (
    let cell = lazy (eval exp env >>= force_value) in
    thunk_of (fun () -> Lazy.force cell))
  else thunk_of (fun () -> eval exp env >>= force_value)
;;

(** {2 4.2.2: the evaluator changes}

    [Core (Eval)] holds the lazy dispatch. Only application and [if]
    differ from 4.1: the application clause forces the operator and
    delays the operands, and the [if] clause forces its predicate. *)
module Core (Eval : sig
    val eval : eval_t
  end) =
struct
  (** [actual_value exp env] is the book's [actual-value]: [eval]
      followed by [force_value], used wherever an actual value is
      needed instead of a thunk. *)
  let actual_value exp env = Eval.eval exp env >>= force_value

  (** [delay_it ~memo ~eval exp env] is the book's [delay-it], the thunk
      of [exp] in [env]. With the default [memo = true] the thunk
      memoizes through a host [Lazy] cell: the first forcing fills the
      cell, later forcings read it, and the book's [set-car!] mutation
      of a thunk into an evaluated thunk becomes the cell's one-time
      fill. With [memo = false] every forcing re-evaluates [exp].
      [~eval] names the evaluator the thunk body runs under, so a
      variant dispatch delays its operands through its own clauses. *)
  let delay_it ?(memo = true) ~(eval : eval_t) exp env =
    if memo
    then (
      let cell = lazy (eval exp env >>= force_value) in
      thunk_of (fun () -> Lazy.force cell))
    else thunk_of (fun () -> eval exp env >>= force_value)
  ;;

  (** [list_of_arg_values exps env] forces the operands left to right:
      the strict primitives see values, never thunks. *)
  let rec list_of_arg_values exps env =
    match exps with
    | [] -> Ok []
    | exp :: rest ->
      actual_value exp env
      >>= fun value -> list_of_arg_values rest env >>= fun values -> Ok (value :: values)
  ;;

  (** [list_of_delayed_args exps env] delays the operands left to
      right, through the module-level [delay_it] under this functor's
      evaluator. *)
  let rec list_of_delayed_args exps env =
    match exps with
    | [] -> Ok []
    | exp :: rest ->
      let first = delay_it ~eval:Eval.eval exp env in
      list_of_delayed_args rest env >>= fun values -> Ok (first :: values)
  ;;

  (** [eval_sequence exps env] is 4.1's sequence: every expression is
      evaluated and the value of the last one is answered. Sequence
      positions are evaluated, not forced; exercise 4.30 debates the
      choice. *)
  let rec eval_sequence exps env =
    match exps with
    | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
    | [ exp ] -> Eval.eval exp env
    | exp :: rest -> Eval.eval exp env >>= fun _ -> eval_sequence rest env
  ;;

  (** [apply_procedure proc operands env] is the book's lazy [apply]:
      the operands arrive unevaluated. A primitive is strict, so its
      operands are forced before the call; a compound procedure is
      non-strict, so every operand is delayed into a thunk and the body
      runs in the extended environment. *)
  let apply_procedure proc operands env =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name primitive_table with
       | Some f -> list_of_arg_values operands env >>= fun args -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; body; env = proc_env; _ } ->
      list_of_delayed_args operands env
      >>= fun args ->
      SE.extend_environment parameters args proc_env
      >>= fun extended -> eval_sequence body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  (** [eval_if exp env] forces the predicate before testing it. *)
  let eval_if exp env =
    match Ast.view exp with
    | Ast.If (predicate, consequent, alternative) ->
      actual_value predicate env
      >>= fun tested ->
      if true_ tested
      then Eval.eval consequent env
      else (
        match alternative with
        | Some branch -> Eval.eval branch env
        | None -> Ok (Value.bool false))
    | _ -> Error (Eval_error.Invalid_form "eval_if: not an if")
  ;;

  let eval_assignment name exp env =
    Eval.eval exp env
    >>= fun value ->
    SE.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
  ;;

  let eval_definition d env =
    match Ast.view_definition d with
    | Ast.Define_variable (name, exp) ->
      Eval.eval exp env >>= fun value -> SE.define_variable_ name value env
    | Ast.Define_function { name; parameters; body } ->
      let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
      SE.define_variable_ name proc env
  ;;

  (** [eval exp env] is the lazy dispatch: one clause per syntactic type,
      [cond] reduced to [if] as a derived expression, and an application
      clause that forces the operator and delays the operands. Quoted
      data stays ordinary data; the section's only 4.1 changes are the
      application clause and the forcing in [eval_if]. *)
  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> SE.lookup_variable_value name env
    | Ast.Quote datum -> Ok (SE.datum_to_value datum)
    | Ast.Definition d -> eval_definition d env
    | Ast.Set (name, e) -> eval_assignment name e env
    | Ast.If _ -> eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> eval_sequence body env
    | Ast.Cond _ -> SE.cond_to_if exp >>= fun rewritten -> Eval.eval rewritten env
    | Ast.Application (operator, operands) ->
      actual_value operator env >>= fun proc -> apply_procedure proc operands env
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

module rec Base : sig
  val eval : eval_t
end = struct
  module C = Core (Base)

  let eval = C.eval
end

(** [eval exp env] evaluates one expression in one environment under the
    lazy language. *)
let eval = Base.eval

(** [actual_value exp env] is the driver's evaluation step: the value of
    [exp], forced if it is a thunk. *)
let actual_value exp env = eval exp env >>= force_value

let read_error e = Eval_error.Invalid_form (Reader.to_string e)

(** [run env text] reads one form from [text] and evaluates it with
    [actual_value]: the lazy driver loop, which forces a delayed value
    before printing it. *)
let run env text =
  Reader.read text |> Result.map_error read_error >>= fun exp -> actual_value exp env
;;

(** [run_program env text] reads a whole program of forms and evaluates
    them in order, forcing the value of the last one. *)
let run_program env text =
  Reader.read_program text
  |> Result.map_error read_error
  >>= fun exps ->
  let rec go = function
    | [] -> Ok (Value.symbol "ok")
    | [ exp ] -> actual_value exp env
    | exp :: rest -> eval exp env >>= fun _ -> go rest
  in
  go exps
;;

(** [setup_environment ()] is a fresh global environment with the
    section's primitive table and the bindings of [true] and [false]. *)
let setup_environment () =
  let env = Sicp_common.Env.empty () in
  Sicp_common.Env.define env "true" (Value.bool true);
  Sicp_common.Env.define env "false" (Value.bool false);
  List.iter
    (fun (name, f) -> Sicp_common.Env.define env name (Value.primitive ~name f))
    primitive_table;
  env
;;

(** [the_global_environment ()] is a fresh global environment, the
    book's [the-global-environment] of this section. *)
let the_global_environment = setup_environment

(** {2 4.2.1: the applicative-order contrast}

    [Strict_eval] is the applicative-order evaluator of 4.1 applied
    over the section's primitive table: the dispatch is 4.1's [Core]
    recursing through [eval], and application -- the one clause the
    section changes -- evaluates every operand before the call and
    resolves the operator against [primitive_table], the table that
    installs [/]. The book's [try] example dies in the operand under
    [Strict_eval] where the lazy driver answers [1]. *)

module rec Strict_eval : sig
  (** [eval exp env] evaluates one expression in one environment under
      the applicative-order dispatch. *)
  val eval : eval_t

  (** [run env text] reads one object-language form from [text] and
      evaluates it in [env]: the strict counterpart of the lazy
      driver. *)
  val run
    :  Sicp_common.Value.env
    -> string
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result
end = struct
  module C = SE.Core (Strict_eval)

  (** [apply_procedure proc args] is 4.1's [apply] over the section's
      table: a primitive is looked up in [primitive_table] by the name
      its value carries; a compound procedure extends its captured
      environment. *)
  let apply_procedure proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; body; env = proc_env; _ } ->
      SE.extend_environment parameters args proc_env
      >>= fun extended -> C.eval_sequence body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  (** [eval exp env] is 4.1's dispatch with the application clause
      resolved over the section's table; every other clause is
      [Core]'s. *)
  let eval exp env =
    match Ast.view exp with
    | Ast.Application (operator, operands) ->
      Strict_eval.eval operator env
      >>= fun proc ->
      C.list_of_values operands env >>= fun args -> apply_procedure proc args
    | _ -> C.eval exp env
  ;;

  let run env text =
    Reader.read text |> Result.map_error read_error >>= fun exp -> eval exp env
  ;;
end
