(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.19 *)

(** Exercise 4.19: one program under the three disciplines for internal
    definitions. Ben's sequential rule answers [16], Alyssa's scan-out
    rejects the program because [a] is still [unassigned] when [b]'s
    initializer runs, and Eva's simultaneous rule answers [20] by
    evaluating the initializers in dependency order against the shared
    scope. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

let unassigned_exp = Ast.quote (Ast.DSymbol "*unassigned*")

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

(* The statement's program. *)
let program =
  {|
(let ((a 1))
  (define (f x)
    (define b (+ a x))
    (define a 5)
    (+ a b))
  (f 10))
|}
;;

(* Ben's rule: the plain dispatch plus the let lowering of 4.6, which
   the statement's program needs; defines run where they appear. *)
module rec Ev_seq : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev_seq)

  let eval exp env =
    match Ast.view exp with
    | Ast.Let (bindings, body) ->
      let names = List.map fst bindings in
      let inits = List.map snd bindings in
      Ast.lambda names body >>= fun proc -> C.eval (Ast.application proc inits) env
    | _ -> C.eval exp env
  ;;
end

(** [occurs name exp] holds when [name] is read eagerly somewhere in
      [exp]: a lambda body or a quoted datum defers its reads, so the
      walk does not descend into them. *)
let rec occurs name exp =
  match Ast.view exp with
  | Ast.Variable n -> String.equal n name
  | Ast.Lambda _ | Ast.Quote _ -> false
  | Ast.Definition d ->
    (match Ast.view_definition d with
     | Ast.Define_variable (_, init) -> occurs name init
     | Ast.Define_function _ -> false)
  | Ast.Set (_, rhs) -> occurs name rhs
  | Ast.If (predicate, consequent, alternative) ->
    occurs name predicate
    || occurs name consequent
    ||
      (match alternative with
      | Some branch -> occurs name branch
      | None -> false)
  | Ast.Cond (clauses, else_body) ->
    List.exists
      (fun (test, body) -> occurs name test || List.exists (occurs name) body)
      clauses
    ||
      (match else_body with
      | Some body -> List.exists (occurs name) body
      | None -> false)
  | Ast.And exps | Ast.Or exps | Ast.Sequence exps -> List.exists (occurs name) exps
  | Ast.Let (bindings, body) ->
    List.exists (fun (_, init) -> occurs name init) bindings
    || List.exists (occurs name) body
  | Ast.Application (operator, operands) ->
    occurs name operator || List.exists (occurs name) operands
  | Ast.Int _ | Ast.Float _ | Ast.Bool _ | Ast.String _ -> false
;;

(** [eva_order defines] orders the definitions so every initializer
      runs after the names it eagerly reads: repeatedly the initializers
      that read no pending name are placed, in source order; a genuine
      cycle, a self-read included, falls back to source order, where
      the [unassigned] marker reports the deadlock. *)
let eva_order defines =
  let tagged = List.mapi (fun index definition -> index, definition) defines in
  let by_source (i, _) (j, _) = compare i j in
  let ready pending =
    List.partition
      (fun (_, (_, init)) ->
         List.for_all (fun (_, (other, _)) -> not (occurs other init)) pending)
      pending
  in
  let rec go placed pending =
    if pending = []
    then List.rev placed
    else (
      match ready pending with
      | [], _ -> List.rev placed @ List.map snd (List.sort by_source pending)
      | ready_items, blocked ->
        go
          (List.rev_append (List.map snd (List.sort by_source ready_items)) placed)
          blocked)
  in
  go [] tagged
;;

(* The scan-out of 4.16, with the assignments emitted in Eva's order:
   every initializer reads the final value of the names it needs. *)
let scan_eva body =
  let rec go defines rest = function
    | [] ->
      (match List.rev defines with
       | [] -> Ok (List.rev rest)
       | defines ->
         let bindings = List.map (fun (name, _) -> name, unassigned_exp) defines in
         let sets =
           List.map (fun (name, init) -> Ast.set name init) (eva_order defines)
         in
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

(* Alyssa's scan-out: the 4.16 transformation, reserve every name,
   assign in source order. *)
let scan_alyssa body =
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

module Alyssa = Make (struct
    let transform = scan_alyssa
  end)

module Eva = Make (struct
    let transform = scan_eva
  end)

let single_trace eval = [ render (run eval program) ]

(** [sequential_trace ()] is Ben's answer: the program under the
      sequential rule. *)
let sequential_trace () = single_trace Ev_seq.eval

(** [alyssa_trace ()] is Alyssa's answer: the program under the
      scan-out of 4.16. *)
let alyssa_trace () = single_trace Alyssa.eval

(** [eva_trace ()] is Eva's answer: the program under the simultaneous
      rule with dependency-ordered initializers. *)
let eva_trace () = single_trace Eva.eval

(** [ex_4_19 ()] answers the three traces in order. *)
let ex_4_19 () = sequential_trace () @ alyssa_trace () @ eva_trace ()
