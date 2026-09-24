(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.17 *)

(** Exercise 4.17: the scan-out of 4.16 wraps the procedure body in a
    let, and evaluating that let application pushes a frame of its own.
    Two evaluators count the frames live at the marked point of the
    text's two-define procedure: the sequential one pushes the
    parameter frame only, the scanned one pushes a second frame for the
    let. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(* One increment per frame pushed onto an environment. The global
   frame is built with defines and never with an extend, so the count
   over a whole run is the frames beyond it. *)
let frames_pushed = ref 0

let counting_extend names values outer =
  incr frames_pushed;
  Sicp_common.Env.extend names values outer
;;

(* The scan-out of 4.16a, duplicated locally: top-level defines become
   one unassigned let, assigned in source order. *)
let unassigned_exp = Ast.quote (Ast.DSymbol "*unassigned*")

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

(* The standard dispatch of 4.1.1-4.1.3, with every frame creation
   routed through [counting_extend]; frames are pushed nowhere else. *)
module Local_core (Eval : sig
    val eval : SE.eval_t
  end) =
struct
  let rec list_of_values exps env =
    match exps with
    | [] -> Ok []
    | exp :: rest ->
      Eval.eval exp env
      >>= fun value -> list_of_values rest env >>= fun values -> Ok (value :: values)
  ;;

  let rec eval_sequence exps env =
    match exps with
    | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
    | [ exp ] -> Eval.eval exp env
    | exp :: rest -> Eval.eval exp env >>= fun _ -> eval_sequence rest env
  ;;

  let apply_procedure proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name SE.primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; body; env; _ } ->
      counting_extend parameters args env >>= fun extended -> eval_sequence body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  let eval_if exp env =
    match Ast.view exp with
    | Ast.If (predicate, consequent, alternative) ->
      Eval.eval predicate env
      >>= fun tested ->
      if SE.true_ tested
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

  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> SE.lookup_variable_value name env
    | Ast.Quote datum -> Ok (SE.datum_to_value datum)
    | Ast.Definition d -> eval_definition d env
    | Ast.Set (name, exp) -> eval_assignment name exp env
    | Ast.If _ -> eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> eval_sequence body env
    | Ast.Cond _ -> SE.cond_to_if exp >>= fun rewritten -> Eval.eval rewritten env
    | Ast.Application (operator, operands) ->
      Eval.eval operator env
      >>= fun proc ->
      list_of_values operands env >>= fun args -> apply_procedure proc args
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

(* One evaluator per strategy: the sequential one is the plain
   dispatch, the scanned one scans every lambda and function define
   and lowers the resulting lets. *)
module Make_eval (Scan : sig
    val active : bool
  end) =
struct
  module rec Ev : sig
    val eval : SE.eval_t
  end = struct
    module C = Local_core (Ev)

    let lower_let bindings body env =
      let names = List.map fst bindings in
      let inits = List.map snd bindings in
      Ast.lambda names body >>= fun proc -> C.eval (Ast.application proc inits) env
    ;;

    let eval exp env =
      if not Scan.active
      then C.eval exp env
      else (
        match Ast.view exp with
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
        | _ -> C.eval exp env)
    ;;
  end

  let eval = Ev.eval
end

module Sequential = Make_eval (struct
    let active = false
  end)

module Scanned = Make_eval (struct
    let active = true
  end)

(* The text's procedure with ⟨e1⟩, ⟨e2⟩ free of ⟨e3⟩'s names, and the
   call that runs ⟨e3⟩. *)
let program =
  {|
(define (f x)
  (define u (* x 2))
  (define v (* x 3))
  (+ u v))
(f 1)
|}
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

let count_under eval =
  frames_pushed := 0;
  let (_ : (Value.t, Eval_error.t) result) = run eval program in
  !frames_pushed
;;

(** [frame_count_sequential ()] counts the frames live at ⟨e3⟩ when
      the definitions are interpreted sequentially. *)
let frame_count_sequential () = count_under Sequential.eval

(** [frame_count_scanned ()] counts the same frames under the
      scan-out, where the lowered let application adds its own. *)
let frame_count_scanned () = count_under Scanned.eval

(** [ex_4_17 ()] answers the two counts. *)
let ex_4_17 () =
  [ string_of_int (frame_count_sequential ()); string_of_int (frame_count_scanned ()) ]
;;
