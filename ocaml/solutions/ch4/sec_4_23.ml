(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.23 *)

(** Exercise 4.23: the two [analyze_sequence]s, counted. The text's
    version combines the execution procedures of a sequence into one
    procedure at analysis time; Alyssa's analyzes the individual
    expressions and loops over the resulting procedures at every
    execution. Two otherwise identical analyzers of the statement's
    program count the sequencing work: the text's version spends it
    once per analysis pass, Alyssa's once per call. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** One execution procedure: the analyzed form of one expression. *)
type execution = Value.env -> (Value.t, Eval_error.t) result

let constant value = fun _ -> Ok value
let constant_error error = fun _ -> Error error
let ok_constant value = Ok (constant value)

(** The statement's program -- [f] of one parameter [n], whose [begin]
    sets [n] to [n] plus 1 and answers [n] times 2 -- built with the
    [Ast] smart constructors. *)
let f_program () =
  Ast.sequence
    [ Ast.set "n" (Ast.application (Ast.variable "+") [ Ast.variable "n"; Ast.int 1 ])
    ; Ast.application (Ast.variable "*") [ Ast.variable "n"; Ast.int 2 ]
    ]
  >>= fun body ->
  Ast.define_function "f" [ "n" ] [ body ] >>= fun d -> Ok (Ast.definition d)
;;

(** One call of the program's procedure: [(f 1)]. *)
let f_call = Ast.application (Ast.variable "f") [ Ast.int 1 ]

(** The text's analyzer. A sequence of more than one expression is
    combined into one execution procedure during analysis -- the work
    [count] measures -- and execution runs the combined procedure, which
    adds no sequencing of its own. *)
module Book = struct
  (** How many times a multi-expression sequence has been processed so
      far, in analysis or in execution. *)
  let count = ref 0

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
    | Ast.Application (operator, operands) -> analyze_application operator operands
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Ok (constant_error (Eval_error.Invalid_form "unknown expression type: ANALYZE"))

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
    match body with
    | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
    (* A one-expression sequence is answered by that expression's own
       execution procedure in both versions; there is no sequencing to
       count. *)
    | [ exp ] -> analyze exp
    | _ ->
      (* The text's combining work: every execution procedure is built
         into the chain here, once, at analysis time. *)
      incr count;
      let rec each = function
        | [] -> Ok []
        | exp :: rest ->
          analyze exp >>= fun exec -> each rest >>= fun execs -> Ok (exec :: execs)
      in
      each body
      >>= fun procs ->
      let rec combine = function
        | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
        | [ exec ] -> Ok exec
        | exec :: rest ->
          combine rest
          >>= fun rest_exec -> Ok (fun env -> exec env >>= fun _ -> rest_exec env)
      in
      combine procs

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

  and apply proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Sicp_ch4.Sec_4_1.primitive_table with
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
end

(** Alyssa's analyzer. The individual expressions are analyzed, but the
    sequence itself is not: the execution procedure loops over the
    procedures at every execution, and that walk is the work [count]
    measures. *)
module Alyssa = struct
  (** How many times the sequence walk of a multi-expression sequence
      has run. *)
  let count = ref 0

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
    | Ast.Application (operator, operands) -> analyze_application operator operands
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Ok (constant_error (Eval_error.Invalid_form "unknown expression type: ANALYZE"))

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
    match body with
    | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
    | [ exp ] -> analyze exp
    | _ ->
      let rec each = function
        | [] -> Ok []
        | exp :: rest ->
          analyze exp >>= fun exec -> each rest >>= fun execs -> Ok (exec :: execs)
      in
      each body
      >>= fun procs ->
      Ok
        (fun env ->
          (* The sequencing itself happens here, at every execution:
             the walk over the analyzed procedures that the text's
             version built into a closure at analysis time. *)
          incr count;
          let rec execute = function
            | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
            | [ exec ] -> exec env
            | exec :: rest -> exec env >>= fun _ -> execute rest
          in
          execute procs)

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

  and apply proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Sicp_ch4.Sec_4_1.primitive_table with
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
end

(** [define_and_call analyze] analyzes the statement's program with
    [analyze], installs it in a fresh global environment, and calls
    [(f 1)] twice. *)
let define_and_call analyze =
  f_program ()
  >>= analyze
  >>= fun define_exec ->
  let env = Sicp_ch4.Sec_4_1.the_global_environment () in
  define_exec env
  >>= fun _ ->
  analyze f_call >>= fun call_exec -> call_exec env >>= fun _ -> call_exec env
;;

(** [analysis_count_book ()] is the sequencing work the text's analyzer
    spends when the statement's program runs once: one analysis pass
    over the [begin], nothing per call. *)
let analysis_count_book () =
  Book.count := 0;
  Book.bodies := [];
  let (_ : (Value.t, Eval_error.t) result) = define_and_call Book.analyze in
  !Book.count
;;

(** [analysis_count_alyssa ()] is the same experiment under Alyssa's
    analyzer: the analysis pass adds no sequencing count, and each of
    the two calls walks the [begin] once. *)
let analysis_count_alyssa () =
  Alyssa.count := 0;
  Alyssa.bodies := [];
  let (_ : (Value.t, Eval_error.t) result) = define_and_call Alyssa.analyze in
  !Alyssa.count
;;

(** [ex_4_23 ()] is the two counts, the text's analyzer first. *)
let ex_4_23 () =
  [ string_of_int (analysis_count_book ()); string_of_int (analysis_count_alyssa ()) ]
;;
