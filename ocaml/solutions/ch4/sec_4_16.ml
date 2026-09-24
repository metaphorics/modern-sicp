(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.16 *)

(** Exercise 4.16: internal definitions are scanned out of every
    procedure body. Each defined name becomes an [unassigned] let
    binding that the definitions' assignments fill in source order, and
    reading a name still carrying the marker is an invalid form. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(* The initializer that reserves a defined name: the book's
   ['*unassigned*], the section's [unassigned] marker value. *)
let unassigned_exp = Ast.quote (Ast.DSymbol "*unassigned*")

(** [scan_out_defines body] is [body] without internal definitions: the
      top-level defines, variable and function forms alike, become one
      [let] reserving every name with [unassigned] followed by the
      assignments in source order; a body without defines is
      unchanged. *)
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

(** [lookup name env] is the nearest binding of [name]; a binding that
      still holds the [unassigned] marker is the error of 4.16a. *)
let lookup name env =
  SE.lookup_variable_value name env
  >>= fun value ->
  if SE.is_unassigned value
  then Error (Eval_error.Invalid_form (name ^ " is read before it is assigned"))
  else Ok value
;;

module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  (* The Let clause of exercise 4.6, which the statement assumes: a
     let is the application of a lambda to the binding values. *)
  let lower_let bindings body env =
    let names = List.map fst bindings in
    let inits = List.map snd bindings in
    Ast.lambda names body >>= fun proc -> C.eval (Ast.application proc inits) env
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
    | Ast.Let (bindings, body) -> lower_let bindings body env
    | _ -> C.eval exp env
  ;;
end

(** [eval] is the standard dispatch with the scan-out installed at
      procedure creation -- the edition's [make-procedure] -- and the
      marker-checking lookup of part (a). *)
let eval = Ev.eval

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let run text =
  let env = SE.the_global_environment () in
  Reader.read_program text
  |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
  >>= fun exps ->
  let rec go = function
    | [] -> Ok (Value.symbol "ok")
    | [ exp ] -> Ev.eval exp env
    | exp :: rest -> Ev.eval exp env >>= fun _ -> go rest
  in
  go exps
;;

(** [ex_4_16 ()] runs the text's mutual recursion under the scan-out
      and then a program that reads a defined name inside another
      definition's initializer. *)
let ex_4_16 () =
  let mutual =
    run
      {|
(define (f x)
  (define (even? n)
    (if (= n 0) true (odd? (- n 1))))
  (define (odd? n)
    (if (= n 0) false (even? (- n 1))))
  (even? x))
(f 4)
|}
  in
  let premature =
    run
      {|
(define (g) (define a (* b 2)) (define b 3) a)
(g)
|}
  in
  [ render mutual; render premature ]
;;
