(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.22 *)

(** Exercise 4.22: [let] in the analyzed evaluator. The analyzer of
    4.1.7 gains a [let] clause the way a direct evaluator would, with
    the difference the exercise turns on: the rewrite to a lambda
    application happens once, at analysis time, so the execution
    procedure of a [let] is the analyzed application itself and no
    rewrite runs at execution. The analyzer is the section's,
    duplicated locally and extended with the one clause. The direct
    comparison runs the standard [Core] dispatch under the same
    rewrite, which it redoes at every execution. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

(** The analyzed evaluator of 4.1.7 with [let]: every clause is the
    section's, and [analyze_let] is the one addition. *)
module Analyze = struct
  (** One execution procedure: the analyzed form of one expression, an
      environment to result closure with the dispatch already decided. *)
  type execution = Value.env -> (Value.t, Eval_error.t) result

  let constant value = fun _ -> Ok value
  let constant_error error = fun _ -> Error error
  let ok_constant value = Ok (constant value)

  (* The analyzed body of every compound procedure, keyed by the
     procedure's identity; the same registry the section's analyzer
     keeps, because [Value.compound] is the shared runtime value. *)
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
    | Ast.Variable name -> Ok (fun env -> Sicp_ch4.Sec_4_1.lookup_variable_value name env)
    | Ast.Quote datum -> ok_constant (Sicp_ch4.Sec_4_1.datum_to_value datum)
    | Ast.Definition d -> analyze_definition d
    | Ast.Set (name, exp) -> analyze_assignment name exp
    | Ast.If (predicate, consequent, alternative) ->
      analyze_if predicate consequent alternative
    | Ast.Lambda (parameters, body) -> analyze_lambda parameters body
    | Ast.Sequence body -> analyze_sequence body
    | Ast.Cond _ -> Sicp_ch4.Sec_4_1.cond_to_if exp >>= analyze
    | Ast.Let (bindings, body) -> analyze_let bindings body
    | Ast.Application (operator, operands) -> analyze_application operator operands
    | Ast.And _ | Ast.Or _ ->
      Ok (constant_error (Eval_error.Invalid_form "unknown expression type: ANALYZE"))

  and analyze_let bindings body =
    (* The whole rewrite happens here, once, at analysis time: the
       lambda over the parameters and body, applied to the inits, is
       analyzed by this same [analyze]. *)
    Ast.lambda (List.map fst bindings) body
    >>= fun procedure -> analyze (Ast.application procedure (List.map snd bindings))

  and analyze_definition d =
    match Ast.view_definition d with
    | Ast.Define_variable (name, exp) ->
      analyze exp
      >>= fun get ->
      Ok
        (fun env ->
          get env >>= fun value -> Sicp_ch4.Sec_4_1.define_variable_ name value env)
    | Ast.Define_function { name; parameters; body } ->
      analyze_sequence body
      >>= fun body_exec ->
      Ok
        (fun env ->
          let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
          register proc body_exec;
          Sicp_ch4.Sec_4_1.define_variable_ name proc env)

  and analyze_assignment name exp =
    analyze exp
    >>= fun get ->
    Ok
      (fun env ->
        get env
        >>= fun value ->
        Sicp_ch4.Sec_4_1.set_variable_value_ name value env
        >>= fun () -> Ok (Value.symbol "ok"))

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
        get env
        >>= fun tested ->
        if Sicp_ch4.Sec_4_1.true_ tested then when_true env else otherwise env)

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

  (** The analyzed [apply]: a primitive runs through the section's
      table; a compound procedure runs its registered analyzed body in
      the extended environment. *)
  and apply proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Sicp_ch4.Sec_4_1.primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; env; _ } ->
      Sicp_common.Env.extend parameters args env
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

(** [analyze exp] is the analyzed form of [exp], [let] included. *)
let analyze exp = Analyze.analyze exp

(** [eval exp env] analyzes [exp] and runs it in [env]. *)
let eval = Analyze.eval

(** The direct evaluator of the exercises 4.4 to 4.6 pattern: the
    standard [Core] dispatch under a recursive module whose [Let] clause
    lowers to a lambda application. The lowering runs at every
    execution, which is the contrast the demonstration draws. *)
module rec Direct : sig
  val eval : Sicp_ch4.Sec_4_1.eval_t
end = struct
  module C = Sicp_ch4.Sec_4_1.Core (Direct)

  let eval exp env =
    match Ast.view exp with
    | Ast.Let (bindings, body) ->
      Ast.lambda (List.map fst bindings) body
      >>= fun procedure ->
      Direct.eval (Ast.application procedure (List.map snd bindings)) env
    | _ -> C.eval exp env
  ;;
end

(** [run_eval eval env text] reads one form from [text] and evaluates
    it in [env] with [eval]. *)
let run_eval eval env text =
  match Reader.read text with
  | Ok exp -> eval exp env
  | Error e -> Error (Eval_error.Invalid_form (Reader.to_string e))
;;

(** [ex_4_22 ()] evaluates [(let ((x 3) (y 4)) (+ x y))] through the
    analyzed evaluator and through the direct evaluator extended with
    the same special form. Both answer 7; the trace is the two printed
    values in that order. *)
let ex_4_22 () =
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  let render = function
    | Ok v -> Value.to_string v
    | Error e -> "Error: " ^ Eval_error.to_string e
  in
  let text = "(let ((x 3) (y 4)) (+ x y))" in
  let analyzed = render (run_eval eval env text) in
  let direct = render (run_eval Direct.eval env text) in
  [ analyzed; direct ]
;;
