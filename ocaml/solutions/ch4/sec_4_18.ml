(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.18 *)

(** Exercise 4.18: the alternative scan-out evaluates every
    initializer before any assignment runs. The solve program of
    3.5.4, with the streams abstracted away, works under the text's
    order and fails under this one, because the eager initializer
    touches a name that is still the [unassigned] marker. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

let unassigned_exp = Ast.quote (Ast.DSymbol "*unassigned*")

(* A fresh name the reader never produces, so it cannot collide with
   the program's own names. *)
let fresh_name index = "%init" ^ string_of_int index

(** [scan_text body] is the text's transformation: reserve every name,
      then assign in source order. *)
let scan_text body =
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

(** [scan_alternative body] is this exercise's transformation: the
      reserved names are assigned from an inner let that binds every
      initializer first, so no initializer runs after any assignment. *)
let scan_alternative body =
  let rec go defines rest = function
    | [] ->
      (match List.rev defines with
       | [] -> Ok (List.rev rest)
       | defines ->
         let reserve = List.map (fun (name, _) -> name, unassigned_exp) defines in
         let init_bindings =
           List.mapi (fun index (_, init) -> fresh_name index, init) defines
         in
         let sets =
           List.mapi
             (fun index (name, _) -> Ast.set name (Ast.variable (fresh_name index)))
             defines
         in
         Ast.let_ init_bindings sets
         >>= fun inner ->
         Ast.let_ reserve (inner :: List.rev rest) >>= fun outer -> Ok [ outer ])
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

module Make (Scan : sig
    val transform : Ast.expr list -> (Ast.expr list, Eval_error.t) result
  end) =
struct
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

    let eval exp env =
      match Ast.view exp with
      | Ast.Variable name -> lookup name env
      | Ast.Let (bindings, body) ->
        let names = List.map fst bindings in
        let inits = List.map snd bindings in
        Ast.lambda names body >>= fun proc -> C.eval (Ast.application proc inits) env
      | Ast.Lambda (parameters, body) ->
        Scan.transform body
        >>= fun scanned -> Ok (Value.compound ~name:None ~parameters ~body:scanned ~env)
      | Ast.Definition d ->
        (match Ast.view_definition d with
         | Ast.Define_function { name; parameters; body } ->
           Scan.transform body
           >>= fun scanned ->
           let proc = Value.compound ~name:(Some name) ~parameters ~body:scanned ~env in
           SE.define_variable_ name proc env
         | Ast.Define_variable _ -> C.eval exp env)
      | _ -> C.eval exp env
    ;;
  end

  let eval = Ev.eval
end

module Text = Make (struct
    let transform = scan_text
  end)

module Alternative = Make (struct
    let transform = scan_alternative
  end)

(* The solve program with the streams abstracted away: [y]'s
   initializer defers its read of [v] behind a lambda, the way
   [integral] defers [dy], and [v]'s initializer touches [y] eagerly,
   the way [stream-map] touches [y]; the probe keeps [v] at [2] either
   way, so only the read is observable. *)
let program =
  {|
(define (make)
  (define y (lambda () (+ v 1)))
  (define v (if (pair? y) 2 2))
  y)
(define it (make))
(it)
|}
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let run eval text =
  let env = SE.the_global_environment () in
  Reader.read_program text
  |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
  >>= fun exps ->
  let rec go = function
    | [] -> Ok (Value.symbol "ok")
    | [ exp ] -> eval exp env
    | exp :: rest -> eval exp env >>= fun _ -> go rest
  in
  go exps
;;

let strategy_trace eval = [ render (run eval program) ]

(** [eval_text_strategy ()] traces the program under the text's
      order: the call of the returned procedure answers [3]. *)
let eval_text_strategy () = strategy_trace Text.eval

(** [eval_alternative_strategy ()] traces the same program under this
      exercise's order: [v]'s eager initializer reads [y] before any
      assignment runs. *)
let eval_alternative_strategy () = strategy_trace Alternative.eval

(** [ex_4_18 ()] answers both traces in order. *)
let ex_4_18 () = eval_text_strategy () @ eval_alternative_strategy ()
