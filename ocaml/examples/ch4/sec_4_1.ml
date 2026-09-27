(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 *)

(** The metacircular evaluator of section 4.1, written in OCaml against
    the shared substrate. The evaluator's syntax is the typed [Ast]
    produced by [Reader]: the core never parses text. [eval] and [apply]
    return [(value, eval_error) result]; primitive procedures report
    through the same channel.

    [Core] is the standard dispatch of 4.1.1 to 4.1.3, parameterized by
    the [eval] it recurses through. [Base] instantiates it with itself,
    which is the evaluator as the book presents it. An exercise that adds
    a clause to [eval] instantiates [Core] with its own recursive module,
    so every nested evaluation crosses the new clause.

    [Analyze] is the analyzed evaluator of 4.1.7: [analyze] compiles an
    expression once into an execution procedure, a host closure from
    environments to results, and [Analyze.eval] calls it immediately. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

(* The [t] and [view] families share constructor names; every pattern
   match goes through [Value.view], whose constructors pick themselves. *)

type eval_t = Ast.expr -> Value.env -> (Value.t, Eval_error.t) result

(** [true_ v] holds for every value except the false object, the book's
    [true?]. *)
let true_ v = not (Value.physical_equal v (Value.bool false))

(** [false_ v] holds exactly for the false object, the book's [false?]. *)
let false_ v = Value.physical_equal v (Value.bool false)

(** [datum_to_value d] is the runtime value of the quoted datum [d]. *)
let rec datum_to_value = function
  | Ast.DInt n -> Value.int n
  | Ast.DFloat f -> Value.float f
  | Ast.DBool b -> Value.bool b
  | Ast.DString s -> Value.string s
  | Ast.DSymbol s -> Value.symbol s
  | Ast.DNil -> Value.nil
  | Ast.DPair (car, cdr) -> Value.pair (datum_to_value car) (datum_to_value cdr)
;;

(** The symbol marking a scanned-out internal definition that has not
    been assigned yet (4.16); the environment never uses [None] or a
    default value for it. *)
let unassigned = Value.symbol "*unassigned*"

let is_unassigned v = Value.physical_equal v unassigned

(** {2 4.1.3: the four environment operations}

    The representations live in [Env] and [Value] since 3.2: an
    environment is a chain of mutable frames, newest first, and a
    compound procedure captures the environment it was created in. The
    operations wrap that representation with the evaluator's error
    channel. *)

let lookup_variable_value name env =
  match Env.find_binding env name with
  | Some value -> Ok value
  | None -> Error (Eval_error.Unbound_variable name)
;;

let extend_environment names values base_env = Env.extend names values base_env
let set_variable_value_ name value env = Env.set env name value

let define_variable_ name value env =
  Env.define env name value;
  Ok (Value.symbol "ok")
;;

(** Arity checks over the evaluated operands; a mismatch answers a typed
    error naming expected and given. *)
let arity0 = function
  | [] -> Ok ()
  | args -> Error (Eval_error.Arity_mismatch { expected = 0; given = List.length args })
;;

let arity1 = function
  | [ v ] -> Ok v
  | args -> Error (Eval_error.Arity_mismatch { expected = 1; given = List.length args })
;;

let arity2 = function
  | [ a; b ] -> Ok (a, b)
  | args -> Error (Eval_error.Arity_mismatch { expected = 2; given = List.length args })
;;

let need_pair name (v : Value.t) =
  match Value.view v with
  | Value.Pair (car, cdr) -> Ok (car, cdr)
  | _ -> Error (Eval_error.Type_error (name ^ ": not a pair: " ^ Value.to_string v))
;;

let need_list name (v : Value.t) =
  match Value.view v with
  | Value.Nil | Value.Pair _ -> Ok ()
  | _ -> Error (Eval_error.Type_error (name ^ ": not a list: " ^ Value.to_string v))
;;

let rec value_list (v : Value.t) =
  match Value.view v with
  | Value.Nil -> Ok []
  | Value.Pair (car, cdr) -> value_list cdr >>= fun rest -> Ok (car :: rest)
  | _ -> Error (Eval_error.Type_error ("not a list: " ^ Value.to_string v))
;;

let list_values values = List.fold_right Value.pair values Value.nil

let int2 name f =
  ( name
  , fun args ->
      arity2 args
      >>= fun (a, b) ->
      match Value.view a, Value.view b with
      | Value.Int a, Value.Int b -> Ok (Value.int (f a b))
      | _ -> Error (Eval_error.Type_error (name ^ ": the operands are not both integers"))
  )
;;

let int_cmp name f =
  ( name
  , fun args ->
      arity2 args
      >>= fun (a, b) ->
      match Value.view a, Value.view b with
      | Value.Int a, Value.Int b -> Ok (Value.bool (f a b))
      | _ -> Error (Eval_error.Type_error (name ^ ": the operands are not both integers"))
  )
;;

(** {2 4.1.4: the primitive table}

    The book's sample primitives plus the arithmetic and output the
    section's programs use; every primitive answers through the one
    error channel. *)
let primitive_table : (string * Value.primitive) list =
  [ ( "car"
    , fun args -> arity1 args >>= fun v -> need_pair "car" v >>= fun (car, _) -> Ok car )
  ; ( "cdr"
    , fun args -> arity1 args >>= fun v -> need_pair "cdr" v >>= fun (_, cdr) -> Ok cdr )
  ; ("cons", fun args -> arity2 args >>= fun (car, cdr) -> Ok (Value.pair car cdr))
  ; ("list", fun args -> Ok (list_values args))
  ; ( "null?"
    , fun args ->
        arity1 args >>= fun v -> Ok (Value.bool (Value.physical_equal v Value.nil)) )
  ; ( "pair?"
    , fun args ->
        arity1 args
        >>= fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Pair _ -> true
              | _ -> false)) )
  ; ( "number?"
    , fun args ->
        arity1 args
        >>= fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Int _ | Value.Float _ -> true
              | _ -> false)) )
  ; ( "symbol?"
    , fun args ->
        arity1 args
        >>= fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Symbol _ -> true
              | _ -> false)) )
  ; ( "eq?"
    , fun args -> arity2 args >>= fun (a, b) -> Ok (Value.bool (Value.physical_equal a b))
    )
  ; ( "equal?"
    , fun args ->
        arity2 args >>= fun (a, b) -> Ok (Value.bool (Value.structural_equal a b)) )
  ; ("not", fun args -> arity1 args >>= fun v -> Ok (Value.bool (false_ v)))
  ; ( "assoc"
    , fun args ->
        arity2 args
        >>= fun (key, entries) ->
        need_list "assoc" entries
        >>= fun () ->
        let rec go (entries : Value.t) =
          match Value.view entries with
          | Value.Nil -> Ok (Value.bool false)
          | Value.Pair (entry, rest) ->
            (match value_list entry with
             | Ok (k :: _ :: _) when Value.structural_equal key k -> Ok entry
             | Ok _ -> go rest
             | Error _ ->
               Error
                 (Eval_error.Type_error "assoc: the entries are not two-element lists"))
          | _ ->
            Error
              (Eval_error.Type_error ("assoc: not a list: " ^ Value.to_string entries))
        in
        go entries )
  ; ( "memq"
    , fun args ->
        arity2 args
        >>= fun (item, l2) ->
        need_list "memq" l2
        >>= fun () ->
        let rec go (l : Value.t) =
          match Value.view l with
          | Value.Nil -> Ok (Value.bool false)
          | Value.Pair (car, _) when Value.physical_equal item car -> Ok l
          | Value.Pair (_, cdr) -> go cdr
          | _ -> Error (Eval_error.Type_error ("memq: not a list: " ^ Value.to_string l))
        in
        go l2 )
  ; ( "cadr"
    , fun args ->
        arity1 args
        >>= fun v ->
        need_pair "cadr" v
        >>= fun (_, cdr) -> need_pair "cadr" cdr >>= fun (second, _) -> Ok second )
  ; int2 "+" ( + )
  ; int2 "-" ( - )
  ; int2 "*" ( * )
  ; int_cmp "=" ( = )
  ; int_cmp "<" ( < )
  ; int_cmp ">" ( > )
  ; ( "display"
    , fun args ->
        arity1 args
        >>= fun v ->
        print_string (Value.display v);
        Ok (Value.symbol "ok") )
  ; ( "newline"
    , fun args ->
        arity0 args
        >>= fun () ->
        print_newline ();
        Ok (Value.symbol "ok") )
  ; ( "error"
    , fun args ->
        match args with
        | [] -> Error (Eval_error.User_error "error")
        | message :: irritants ->
          let rendered =
            List.fold_left
              (fun acc v -> acc ^ " " ^ Value.display v)
              (Value.display message)
              irritants
          in
          Error (Eval_error.User_error rendered) )
  ]
;;

(** [setup_environment ()] is the global environment: one frame with the
    primitives and the bindings of [true] and [false]. *)
let setup_environment () =
  let env = Env.empty () in
  Env.define env "true" (Value.bool true);
  Env.define env "false" (Value.bool false);
  List.iter
    (fun (name, f) -> Env.define env name (Value.primitive ~name f))
    primitive_table;
  env
;;

(** [the_global_environment ()] is a fresh global environment, the
    book's [the-global-environment]. *)
let the_global_environment = setup_environment

(** [sequence_to_exp exps] packs a clause body into one expression, a
    [begin] when more than one expression remains. *)
let sequence_to_exp exps =
  match exps with
  | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
  | [ exp ] -> Ok exp
  | exps -> Ast.sequence exps
;;

(** [cond_to_if exp] is the derived-expression rewrite of a [cond]:
    each clause becomes an [if], the trailing [else] body or the false
    object when every predicate fails. *)
let cond_to_if exp =
  match Ast.view exp with
  | Ast.Cond (clauses, else_body) ->
    let rec expand = function
      | [] ->
        (match else_body with
         | Some body -> sequence_to_exp body
         | None -> Ok (Ast.bool false))
      | (test, actions) :: rest ->
        expand rest
        >>= fun alternative ->
        sequence_to_exp actions
        >>= fun consequent -> Ok (Ast.if_ test consequent (Some alternative))
    in
    expand clauses
  | _ -> Error (Eval_error.Invalid_form "cond_to_if: not a cond")
;;

(** {2 4.1.1: the core of the evaluator}

    [Core] holds the procedures of the [eval]/[apply] cycle. Every
    recursive step goes through the functor argument's [eval], so an
    instantiation with extra clauses catches the whole recursion. *)
module Core (Eval : sig
    val eval : eval_t
  end) =
struct
  (** [list_of_values exps env] evaluates the operands of a combination.
      The recursion evaluates the first operand, then the rest, so the
      order is left to right by construction; exercise 4.1 pins both
      orders down explicitly. *)
  let rec list_of_values exps env =
    match exps with
    | [] -> Ok []
    | exp :: rest ->
      Eval.eval exp env
      >>= fun value -> list_of_values rest env >>= fun values -> Ok (value :: values)
  ;;

  (** [list_of_values_right_to_left] is exercise 4.1's second version:
      the same operands evaluated from right to left. *)
  let rec list_of_values_right_to_left exps env =
    match exps with
    | [] -> Ok []
    | exp :: rest ->
      list_of_values_right_to_left rest env
      >>= fun values -> Eval.eval exp env >>= fun value -> Ok (value :: values)
  ;;

  (** [eval_sequence exps env] evaluates the expressions of a body or a
      [begin] in order and returns the value of the last one. *)
  let rec eval_sequence exps env =
    match exps with
    | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
    | [ exp ] -> Eval.eval exp env
    | exp :: rest -> Eval.eval exp env >>= fun _ -> eval_sequence rest env
  ;;

  (** [apply_procedure proc args] is the book's [apply]: a primitive is
      looked up in the section's table by the name the value carries and
      applied directly; a compound procedure extends the captured
      environment with a frame binding the parameters to the arguments
      and evaluates the body there. *)
  let apply_procedure proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; body; env; _ } ->
      Env.extend parameters args env >>= fun extended -> eval_sequence body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  (** [eval_if exp env] evaluates the predicate in the object language
      and translates the value with [true_] before branching. A missing
      alternative yields the false object. *)
  let eval_if exp env =
    match Ast.view exp with
    | Ast.If (predicate, consequent, alternative) ->
      Eval.eval predicate env
      >>= fun tested ->
      if true_ tested
      then Eval.eval consequent env
      else (
        match alternative with
        | Some branch -> Eval.eval branch env
        | None -> Ok (Value.bool false))
    | _ -> Error (Eval_error.Invalid_form "eval_if: not an if")
  ;;

  (** [eval_assignment name exp env] changes the nearest binding of
      [name] and answers the symbol [ok]. *)
  let eval_assignment name exp env =
    Eval.eval exp env
    >>= fun value ->
    set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
  ;;

  (** [eval_definition d env] binds the variable, or the procedure the
      function form names, in the newest frame, and answers [ok]. *)
  let eval_definition d env =
    match Ast.view_definition d with
    | Ast.Define_variable (name, exp) ->
      Eval.eval exp env >>= fun value -> define_variable_ name value env
    | Ast.Define_function { name; parameters; body } ->
      let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
      define_variable_ name proc env
  ;;

  (** The standard dispatch of the metacircular evaluator: one clause
      per syntactic type, [cond] reduced to [if] as a derived
      expression, and a final clause that treats the remainder as a
      procedure application. [And], [Or], and [Let] are not in the
      language of this section; exercises 4.4 to 4.6 add them. *)
  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> lookup_variable_value name env
    | Ast.Quote datum -> Ok (datum_to_value datum)
    | Ast.Definition d -> eval_definition d env
    | Ast.Set (name, exp) -> eval_assignment name exp env
    | Ast.If _ -> eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> eval_sequence body env
    | Ast.Cond _ -> cond_to_if exp >>= fun rewritten -> Eval.eval rewritten env
    | Ast.Application (operator, operands) ->
      Eval.eval operator env
      >>= fun proc ->
      list_of_values operands env >>= fun args -> apply_procedure proc args
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

(** [eval exp env] evaluates one expression in one environment. *)
let eval = Base.eval

(** {2 4.1.4: the driver}

    The driver reads object-language text with the shared [Reader] --
    parsing lives at the surface, never in the core -- and evaluates it
    in the given environment. Defines and assignments mutate that
    environment, so a sequence of [run] calls shares state, exactly as
    the book's driver loop does. *)

let read_error e = Eval_error.Invalid_form (Reader.to_string e)

(** [run env text] reads one form from [text] and evaluates it in [env]. *)
let run env text =
  Reader.read text |> Result.map_error read_error >>= fun exp -> eval exp env
;;

(** [run_program env text] reads a whole program of forms and evaluates
    them in order, answering the value of the last one. *)
let run_program env text =
  Reader.read_program text
  |> Result.map_error read_error
  >>= fun exps ->
  let rec go = function
    | [] -> Ok (Value.symbol "ok")
    | [ exp ] -> eval exp env
    | exp :: rest -> eval exp env >>= fun _ -> go rest
  in
  go exps
;;

(** {2 4.1.7: separating syntactic analysis from execution}

    [Analyze] splits [eval] in two: [analyze] walks the expression once
    and returns an execution procedure, a host closure from environments
    to results, with every dispatch decision already made.
    [Analyze.eval] analyzes and executes immediately, the book's
    [(define (eval exp env) ((analyze exp) env))]. *)
module Analyze = struct
  (** One execution procedure: the analyzed form of one expression. *)
  type execution = Value.env -> (Value.t, Eval_error.t) result

  let constant value = fun _ -> Ok value
  let constant_error error = fun _ -> Error error
  let ok_constant value = Ok (constant value)

  (* The analyzed body of every compound procedure, keyed by the
     procedure's identity. The book's [make-procedure] stores the
     execution procedure inside the procedure object; [Value.compound]
     is the shared runtime value, so the analyzed body lives beside it
     here, registered once at creation. A physical scan, because a
     hash over the value would change as the captured frames mutate. *)
  let bodies : (Value.t * execution) list ref = ref []
  let register proc body_exec = bodies := (proc, body_exec) :: !bodies

  let registered proc =
    let rec go = function
      | [] -> None
      | (k, exec) :: rest -> if Value.physical_equal k proc then Some exec else go rest
    in
    go !bodies
  ;;

  let rec analyze exp =
    match Ast.view exp with
    | Ast.Int n -> ok_constant (Value.int n)
    | Ast.Float f -> ok_constant (Value.float f)
    | Ast.Bool b -> ok_constant (Value.bool b)
    | Ast.String s -> ok_constant (Value.string s)
    | Ast.Variable name -> Ok (fun env -> lookup_variable_value name env)
    | Ast.Quote datum -> ok_constant (datum_to_value datum)
    | Ast.Definition d -> analyze_definition d
    | Ast.Set (name, exp) -> analyze_assignment name exp
    | Ast.If (predicate, consequent, alternative) ->
      analyze_if predicate consequent alternative
    | Ast.Lambda (parameters, body) -> analyze_lambda parameters body
    | Ast.Sequence body -> analyze_sequence body
    | Ast.Cond _ -> cond_to_if exp >>= analyze
    | Ast.Application (operator, operands) -> analyze_application operator operands
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Ok (constant_error (Eval_error.Invalid_form "unknown expression type: ANALYZE"))

  and analyze_definition d =
    match Ast.view_definition d with
    | Ast.Define_variable (name, exp) ->
      analyze exp
      >>= fun get ->
      Ok (fun env -> get env >>= fun value -> define_variable_ name value env)
    | Ast.Define_function { name; parameters; body } ->
      analyze_sequence body
      >>= fun body_exec ->
      Ok
        (fun env ->
          let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
          register proc body_exec;
          define_variable_ name proc env)

  and analyze_assignment name exp =
    analyze exp
    >>= fun get ->
    Ok
      (fun env ->
        get env
        >>= fun value ->
        set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok"))

  and analyze_lambda parameters body =
    analyze_sequence body
    >>= fun body_exec ->
    Ok
      (fun env ->
        let proc = Value.compound ~name:None ~parameters ~body ~env in
        register proc body_exec;
        Ok proc)

  and analyze_if predicate consequent alternative =
    analyze predicate
    >>= fun get ->
    analyze consequent
    >>= fun when_true ->
    let when_false =
      match alternative with
      | Some branch -> analyze branch
      | None -> ok_constant (Value.bool false)
    in
    when_false
    >>= fun otherwise ->
    Ok
      (fun env ->
        get env >>= fun tested -> if true_ tested then when_true env else otherwise env)

  and analyze_sequence body =
    let rec go = function
      | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
      | [ exp ] -> analyze exp
      | exp :: rest ->
        analyze exp
        >>= fun first ->
        go rest >>= fun others -> Ok (fun env -> first env >>= fun _ -> others env)
    in
    go body

  and analyze_application operator operands =
    analyze operator
    >>= fun get_proc ->
    let rec collect = function
      | [] -> Ok []
      | exp :: rest ->
        analyze exp >>= fun get -> collect rest >>= fun gets -> Ok (get :: gets)
    in
    collect operands
    >>= fun gets ->
    Ok
      (fun env ->
        get_proc env
        >>= fun proc ->
        let rec evaluate acc = function
          | [] -> Ok (List.rev acc)
          | get :: rest -> get env >>= fun value -> evaluate (value :: acc) rest
        in
        evaluate [] gets >>= fun args -> apply proc args)

  (** The analyzed evaluator's [apply]: primitives run through the
      section's table, and a compound procedure extends the captured
      environment with the arguments and executes its registered
      analyzed body. *)
  and apply proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; env; _ } ->
      Env.extend parameters args env
      >>= fun extended ->
      (match registered proc with
       | Some body_exec -> body_exec extended
       | None -> Error (Eval_error.Invalid_form "the analyzed body is missing"))
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  (** [eval exp env] analyzes [exp] and runs the resulting execution
      procedure in [env]. *)
  let eval exp env = analyze exp >>= fun proc -> proc env
end
