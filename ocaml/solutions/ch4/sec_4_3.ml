(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.3 *)

(** Data-directed dispatch. The syntactic type of an expression indexes
    a handler table; [eval] consults the table for every compound form
    and falls back to the application clause for forms with no entry.
    Every handler recurses through [Ev], so a [put] made after the fact
    changes the behavior of the whole evaluator, at every depth, which
    is the extensibility the exercise asks for. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** One handler: the whole expression and the environment, to a value
    or an error. *)
type handler = Ast.expr -> Value.env -> (Value.t, Eval_error.t) result

let table : (string, handler) Hashtbl.t = Hashtbl.create 16

(** [put tag handler] installs the handler of one expression type. *)
let put (tag : string) (handler : Ast.expr -> Value.env -> (Value.t, Eval_error.t) result)
  : unit
  =
  Hashtbl.replace table tag handler
;;

(** [get tag] is the installed handler of [tag], or [None]. *)
let get (tag : string) : (Ast.expr -> Value.env -> (Value.t, Eval_error.t) result) option =
  Hashtbl.find_opt table tag
;;

(** [type_name shape] is the table key of one syntactic type, or [None]
    for the self-evaluating expressions and the application clause. *)
let type_name = function
  | Ast.Variable _ -> Some "variable"
  | Ast.Quote _ -> Some "quote"
  | Ast.Definition _ -> Some "definition"
  | Ast.Set _ -> Some "set!"
  | Ast.If _ -> Some "if"
  | Ast.Lambda _ -> Some "lambda"
  | Ast.Sequence _ -> Some "sequence"
  | Ast.Cond _ -> Some "cond"
  | Ast.Int _
  | Ast.Float _
  | Ast.Bool _
  | Ast.String _
  | Ast.And _
  | Ast.Or _
  | Ast.Let _
  | Ast.Application _ -> None
;;

module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Application _ -> C.eval exp env
    | shape ->
      (match type_name shape with
       | None -> C.eval exp env
       | Some tag ->
         (match get tag with
          | Some handler -> handler exp env
          | None -> C.eval exp env))
  ;;
end

let eval = Ev.eval

let rec eval_sequence_exps exps env =
  match exps with
  | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
  | [ last ] -> Ev.eval last env
  | next :: rest -> Ev.eval next env >>= fun _ -> eval_sequence_exps rest env
;;

let variable_handler exp env =
  match Ast.view exp with
  | Ast.Variable name -> SE.lookup_variable_value name env
  | _ -> Error (Eval_error.Invalid_form "variable: not a variable")
;;

let quote_handler exp _env =
  match Ast.view exp with
  | Ast.Quote datum -> Ok (SE.datum_to_value datum)
  | _ -> Error (Eval_error.Invalid_form "quote: not a quote")
;;

let definition_handler exp env =
  match Ast.view exp with
  | Ast.Definition d ->
    (match Ast.view_definition d with
     | Ast.Define_variable (name, value_exp) ->
       Ev.eval value_exp env >>= fun value -> SE.define_variable_ name value env
     | Ast.Define_function { name; parameters; body } ->
       let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
       SE.define_variable_ name proc env)
  | _ -> Error (Eval_error.Invalid_form "definition: not a definition")
;;

let set_handler exp env =
  match Ast.view exp with
  | Ast.Set (name, value_exp) ->
    Ev.eval value_exp env
    >>= fun value ->
    SE.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
  | _ -> Error (Eval_error.Invalid_form "set!: not an assignment")
;;

let if_handler exp env =
  match Ast.view exp with
  | Ast.If (predicate, consequent, alternative) ->
    Ev.eval predicate env
    >>= fun tested ->
    if SE.true_ tested
    then Ev.eval consequent env
    else (
      match alternative with
      | Some branch -> Ev.eval branch env
      | None -> Ok (Value.bool false))
  | _ -> Error (Eval_error.Invalid_form "if: not an if")
;;

let lambda_handler exp env =
  match Ast.view exp with
  | Ast.Lambda (parameters, body) -> Ok (Value.compound ~name:None ~parameters ~body ~env)
  | _ -> Error (Eval_error.Invalid_form "lambda: not a lambda")
;;

let sequence_handler exp env =
  match Ast.view exp with
  | Ast.Sequence body -> eval_sequence_exps body env
  | _ -> Error (Eval_error.Invalid_form "sequence: not a sequence")
;;

let cond_handler exp env = SE.cond_to_if exp >>= fun rewritten -> Ev.eval rewritten env

let install_default_handlers () =
  List.iter
    (fun (tag, handler) -> put tag handler)
    [ "variable", variable_handler
    ; "quote", quote_handler
    ; "definition", definition_handler
    ; "set!", set_handler
    ; "if", if_handler
    ; "lambda", lambda_handler
    ; "sequence", sequence_handler
    ; "cond", cond_handler
    ]
;;

let () = install_default_handlers ()

let run env text =
  Reader.read text
  |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
  >>= fun exp -> Ev.eval exp env
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [custom_quote_handler] answers a fixed symbol, distinguishable from
    every ordinary quote result. *)
let custom_quote_handler exp _env =
  match Ast.view exp with
  | Ast.Quote _ -> Ok (Value.symbol "custom")
  | _ -> Error (Eval_error.Invalid_form "custom quote: not a quote")
;;

(** [ex_4_03 ()] evaluates a quote and an [if] through the table's
      handlers, replaces the quote handler and evaluates the same quote
      under the replacement, restores the standard handler, and applies
      a primitive, which has no table entry. *)
let ex_4_03 () =
  let env = SE.the_global_environment () in
  let through_table = run env "'(a b c)" in
  let nested = run env "(if #t 'yes 'no)" in
  put "quote" custom_quote_handler;
  let replaced = run env "'marked" in
  put "quote" quote_handler;
  let restored = run env "'marked" in
  let application = run env "(+ 1 2)" in
  List.map render [ through_table; nested; replaced; restored; application ]
;;
