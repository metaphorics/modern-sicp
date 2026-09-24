(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.20 *)

(** Exercise 4.20: [letrec] as a derived expression. A letrec shape
    lowers to one [let] that reserves every name with the section's
    [unassigned] marker followed by the assignments, and the lowered
    form runs on the scan-out evaluator of 4.16, duplicated locally,
    whose lookup rejects a read of the marker. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** A letrec shape: the bindings and the body of one letrec form. The
    shared grammar carries no letrec, so the shape stands in for the
    syntax and the lowering uses the [Ast] smart constructors. *)
type shape = (string * Ast.expr) list * Ast.expr list

let unassigned_exp = Ast.quote (Ast.DSymbol "*unassigned*")

(** [letrec_to_lets shape] lowers one letrec shape to the [let] that
      reserves every name with [unassigned], followed by one [set!]
      per name, followed by the body. *)
let letrec_to_lets (bindings, body) =
  let reserve = List.map (fun (name, _) -> name, unassigned_exp) bindings in
  let sets = List.map (fun (name, init) -> Ast.set name init) bindings in
  Ast.let_ reserve (sets @ body)
;;

module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  let lookup name env =
    SE.lookup_variable_value name env
    >>= fun value ->
    if SE.is_unassigned value
    then Error (Eval_error.Invalid_form (name ^ " is read before it is assigned"))
    else Ok value
  ;;

  let scan_out_defines body =
    let rec go defines rest = function
      | [] ->
        (match List.rev defines with
         | [] -> Ok (List.rev rest)
         | defines ->
           let bindings = List.map (fun (name, _) -> name, unassigned_exp) defines in
           let sets = List.map (fun (name, init) -> Ast.set name init) defines in
           Ast.let_ bindings (sets @ List.rev rest) >>= fun let_exp -> Ok [ let_exp ])
      | exp :: tl ->
        (match Ast.view exp with
         | Ast.Definition d ->
           (match Ast.view_definition d with
            | Ast.Define_variable (name, init) -> go ((name, init) :: defines) rest tl
            | Ast.Define_function { name; parameters; body = fn_body } ->
              Ast.lambda parameters fn_body
              >>= fun proc -> go ((name, proc) :: defines) rest tl)
         | _ -> go defines (exp :: rest) tl)
    in
    go [] [] body
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.Variable name -> lookup name env
    | Ast.Lambda (parameters, body) ->
      scan_out_defines body
      >>= fun scanned -> Ok (Value.compound ~name:None ~parameters ~body:scanned ~env)
    | Ast.Definition d ->
      (match Ast.view_definition d with
       | Ast.Define_function { name; parameters; body } ->
         scan_out_defines body
         >>= fun scanned ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body:scanned ~env in
         SE.define_variable_ name proc env
       | Ast.Define_variable _ -> C.eval exp env)
    | Ast.Let (bindings, body) ->
      let names = List.map fst bindings in
      let inits = List.map snd bindings in
      Ast.lambda names body >>= fun proc -> C.eval (Ast.application proc inits) env
    | _ -> C.eval exp env
  ;;
end

(** [eval_shape shape env] lowers [shape] and evaluates the result in
      [env] with the scan-out evaluator. *)
let eval_shape shape env = letrec_to_lets shape >>= fun lowered -> Ev.eval lowered env

(* [call operator operands] is the call of the variable [operator]. *)
let call operator operands = Ast.application (Ast.variable operator) operands

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(* The statement's even?/odd? pair under letrec, applied to 4. *)
let even_odd : (shape, Eval_error.t) result =
  Ast.lambda
    [ "n" ]
    [ Ast.if_
        (call "=" [ Ast.variable "n"; Ast.int 0 ])
        (Ast.bool true)
        (Some (call "odd?" [ call "-" [ Ast.variable "n"; Ast.int 1 ] ]))
    ]
  >>= fun even_ ->
  Ast.lambda
    [ "n" ]
    [ Ast.if_
        (call "=" [ Ast.variable "n"; Ast.int 0 ])
        (Ast.bool false)
        (Some (call "even?" [ call "-" [ Ast.variable "n"; Ast.int 1 ] ]))
    ]
  >>= fun odd_ -> Ok ([ "even?", even_; "odd?", odd_ ], [ call "even?" [ Ast.int 4 ] ])
;;

(* A nested letrec: the inner one reserves and sets its own [x], so it
   shadows the outer binding and answers its own value. *)
let nested : (shape, Eval_error.t) result =
  letrec_to_lets ([ "x", Ast.int 2 ], [ Ast.variable "x" ])
  >>= fun inner -> Ok ([ "x", Ast.int 1 ], [ inner ])
;;

(** [ex_4_20 ()] runs the two demonstrations and answers their
      values. *)
let ex_4_20 () =
  let env = SE.the_global_environment () in
  let outcome = function
    | Ok shape -> render (eval_shape shape env)
    | Error e -> "Error: " ^ Eval_error.to_string e
  in
  [ outcome even_odd; outcome nested ]
;;
