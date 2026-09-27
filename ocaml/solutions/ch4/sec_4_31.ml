(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.31: [lazy] and [lazy-memo] parameter declarations. The
    statement's extended [define] syntax is installed here by the "new
    syntax procedures" the statement asks for: a datum-level parse of
    the parameter list turns [a], [(b lazy)], and [(d lazy-memo)] into
    parameter declarations over the typed AST, and the application of an
    annotated compound procedure binds each parameter to a forced value,
    an unmemoized thunk, or a memoized thunk, in that order. Ordinary
    definitions keep Scheme's strictness, so the extension is upward
    compatible. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

(** How one parameter receives its argument. *)
type mode =
  | Strict
  | Lazy
  | Lazy_memo

(** One parameter declaration: the statement's [(name mode)] syntax. *)
type param_decl =
  { name : string
  ; mode : mode
  }

(** The annotated procedures of this evaluator, keyed by the procedure
    value; a physical scan, as in the 4.1 analyzer's body table. *)
let annotations : (Value.t * param_decl list) list ref = ref []

let annotations_of proc =
  let rec go = function
    | [] -> None
    | (k, decls) :: rest -> if Value.physical_equal k proc then Some decls else go rest
  in
  go !annotations
;;

let all_results results =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | r :: rest -> r >>= fun v -> go (v :: acc) rest
  in
  go [] results
;;

(** [datum_to_expr d] lifts the datum [d] into the typed AST, the small
    surface the exercise's syntax procedures need. *)
let rec datum_to_expr (d : Ast.datum) : (Ast.expr, Eval_error.t) result =
  match d with
  | Ast.DInt n -> Ok (Ast.int n)
  | Ast.DFloat f -> Ok (Ast.float f)
  | Ast.DBool b -> Ok (Ast.bool b)
  | Ast.DString s -> Ok (Ast.string s)
  | Ast.DSymbol s -> Ok (Ast.variable s)
  | Ast.DNil -> Error (Eval_error.Invalid_form "the empty combination () is not a form")
  | Ast.DPair (Ast.DSymbol "quote", Ast.DPair (quoted, Ast.DNil)) -> Ok (Ast.quote quoted)
  | Ast.DPair (Ast.DSymbol "if", operands) ->
    (match datum_list operands with
     | Some [ test; consequent; alternative ] ->
       datum_to_expr test
       >>= fun test ->
       datum_to_expr consequent
       >>= fun consequent ->
       datum_to_expr alternative
       >>= fun alternative -> Ok (Ast.if_ test consequent (Some alternative))
     | Some [ test; consequent ] ->
       datum_to_expr test
       >>= fun test ->
       datum_to_expr consequent >>= fun consequent -> Ok (Ast.if_ test consequent None)
     | _ -> Error (Eval_error.Invalid_form "if: expects a test and one or two branches"))
  | Ast.DPair (head, operands) ->
    (match datum_list operands with
     | Some ds ->
       all_results (List.map datum_to_expr (head :: ds))
       >>= (function
        | hd :: args -> Ok (Ast.application hd args)
        | [] -> Error (Eval_error.Invalid_form "the empty combination () is not a form"))
     | None -> Error (Eval_error.Invalid_form "a dotted list is not a form"))

and datum_list (d : Ast.datum) : Ast.datum list option =
  let rec go acc = function
    | Ast.DNil -> Some (List.rev acc)
    | Ast.DPair (car, cdr) -> go (car :: acc) cdr
    | _ -> None
  in
  go [] d
;;

(** [param_decl_of d] reads one declaration: a bare name is strict,
    [(name lazy)] delays without memoization, [(name lazy-memo)] delays
    with it. *)
let param_decl_of (d : Ast.datum) : (param_decl, Eval_error.t) result =
  match d with
  | Ast.DSymbol name -> Ok { name; mode = Strict }
  | Ast.DPair (Ast.DSymbol name, Ast.DPair (Ast.DSymbol "lazy-memo", Ast.DNil)) ->
    Ok { name; mode = Lazy_memo }
  | Ast.DPair (Ast.DSymbol name, Ast.DPair (Ast.DSymbol "lazy", Ast.DNil)) ->
    Ok { name; mode = Lazy }
  | _ ->
    Error
      (Eval_error.Invalid_form
         "the parameter declaration is not a name, (name lazy), or (name lazy-memo)")
;;

(** [install env text] reads the statement's extended [define] -- the
    driver cannot parse it, so the form is read as a quoted datum -- and
    installs the procedure with its declarations. *)
let install env text =
  match Reader.read ("'" ^ text) with
  | Error e -> Error (Eval_error.Invalid_form (Reader.to_string e))
  | Ok read_form ->
    (match Ast.view read_form with
     | Ast.Quote datum ->
       (match datum with
        | Ast.DPair
            (Ast.DSymbol "define", Ast.DPair (Ast.DPair (Ast.DSymbol name, params), body))
          ->
          (match datum_list params, datum_list body with
           | Some params, Some body ->
             all_results (List.map param_decl_of params)
             >>= fun decls ->
             all_results (List.map datum_to_expr body)
             >>= fun body ->
             let proc =
               Value.compound
                 ~name:(Some name)
                 ~parameters:(List.map (fun d -> d.name) decls)
                 ~body
                 ~env
             in
             annotations := (proc, decls) :: !annotations;
             Env.define env name proc;
             Ok (Value.symbol "ok")
           | _ ->
             Error (Eval_error.Invalid_form "install: a dotted parameter or body list"))
        | _ -> Error (Eval_error.Invalid_form "install: not an annotated define"))
     | _ -> Error (Eval_error.Invalid_form "install: not a quoted define"))
;;

(** The annotated evaluator: the section's dispatch with an application
    clause that binds per declaration. *)
module rec Annotated : sig
  val eval : Lazy_eval.eval_t
end = struct
  module C = Lazy_eval.Core (Annotated)

  let rec bind env decls operands acc =
    match decls, operands with
    | [], [] -> Ok (List.rev acc)
    | d :: ds, op :: ops ->
      (match d.mode with
       | Strict -> C.actual_value op env >>= fun v -> bind env ds ops (v :: acc)
       | Lazy ->
         bind
           env
           ds
           ops
           (Lazy_eval.delay_it ~memo:false ~eval:Annotated.eval op env :: acc)
       | Lazy_memo ->
         bind env ds ops (Lazy_eval.delay_it ~memo:true ~eval:Annotated.eval op env :: acc))
    | _ ->
      Error
        (Eval_error.Arity_mismatch
           { expected = List.length decls; given = List.length operands })
  ;;

  let apply_procedure proc operands env =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Lazy_eval.primitive_table with
       | Some f -> C.list_of_arg_values operands env >>= fun args -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; body; env = proc_env; _ } ->
      (match annotations_of proc with
       | Some decls ->
         bind env decls operands []
         >>= fun args ->
         Strict_eval.extend_environment parameters args proc_env
         >>= fun extended -> C.eval_sequence body extended
       | None ->
         C.list_of_arg_values operands env
         >>= fun args ->
         Strict_eval.extend_environment parameters args proc_env
         >>= fun extended -> C.eval_sequence body extended)
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> Strict_eval.lookup_variable_value name env
    | Ast.Quote datum -> Ok (Strict_eval.datum_to_value datum)
    | Ast.Definition d ->
      (match Ast.view_definition d with
       | Ast.Define_variable (name, e) ->
         Annotated.eval e env >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Annotated.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If _ -> C.eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Strict_eval.cond_to_if exp >>= fun rewritten -> Annotated.eval rewritten env
    | Ast.Application (operator, operands) ->
      C.actual_value operator env >>= fun proc -> apply_procedure proc operands env
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let run env text =
  match Reader.read text with
  | Ok exp -> render (Annotated.eval exp env)
  | Error e -> "Error: " ^ Reader.to_string e
;;

let counted = "(define count 0) (define (id x) (set! count (+ count 1)) x)"

(** [ex_4_31 ()] exercises the declarations: a [lazy] parameter never
    forced saves an armed error; the four-way declaration forces the
    strict parameters at the call, runs the [lazy] one once per use, and
    the [lazy-memo] one once in total; [count] reads 5 after the call;
    and the same arm under [lazy] alone runs twice where [lazy-memo]
    runs once. *)
let ex_4_31 () =
  let lazy_only = Lazy_eval.the_global_environment () in
  let four_way = Lazy_eval.the_global_environment () in
  let memo_pair = Lazy_eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Lazy_eval.run_program memo_pair counted in
  let (_ : (Value.t, Eval_error.t) result) = Lazy_eval.run_program four_way counted in
  let r1 = render (install lazy_only "(define (pick (b lazy)) (if #t 'taken b))") in
  let r2 = run lazy_only "(pick (car '()))" in
  let r3 =
    render (install four_way "(define (f a (b lazy) c (d lazy-memo)) (list a b b c d d))")
  in
  let r4 = run four_way "(f (id 1) (id (+ 2 3)) (id 4) (id (* 5 6)))" in
  let r5 = run four_way "count" in
  let r6 = render (install memo_pair "(define (twice (b lazy)) (list b b))") in
  let r7 = run memo_pair "(twice (id 10))" in
  let r8 = run memo_pair "count" in
  let r9 = render (install memo_pair "(define (twice-m (d lazy-memo)) (list d d))") in
  let r10 = run memo_pair "(twice-m (id 10))" in
  let r11 = run memo_pair "count" in
  [ r1; r2; r3; r4; r5; r6; r7; r8; r9; r10; r11 ]
;;
