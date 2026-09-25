(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.79: rule application with environments instead of
    renaming. Where the section's engine renames every rule variable
    under a fresh application id, this engine applies a rule in a
    layer of its own, shaped like a procedure-call frame: the layer's
    [params] hold the unrenamed conclusion's bindings, fixed for the
    application, and its [local] collects what the body's matches add.
    Variable lookup is local, then parameters, then the parent chain
    outwards, exactly as a procedure body sees its own parameters
    and, through the chain, its caller's bindings; a recursive
    application's parameters shadow their same-named ancestors only
    inside the call, because on return the layer is squashed into its
    parent -- the body's caller-visible bindings join the parent's
    locals and the parameters vanish, as a call's frame is discarded
    while the constraints it recorded on the caller's variables
    remain. A rule application is a directed parameter binding (a
    procedure call), not a symmetric unification: parameters bind
    fresh, argument variables are read through the chain -- a bound
    argument contributes its value, an unbound one is passed by
    reference. The data base, the index, and the stream combinators
    are the substrate's, and the evaluator mirrors its clause
    structure, so answer order is the stream system's. The demo pins
    equivalence with the renaming engine on the recursive
    [outranked-by] rule over Microshaft, including a query whose
    bound variable must flow into the rule through the chain. The
    exercise's open-ended reaches -- block-structured rule
    environments, deduction in a supposed context -- are beyond this
    edition's scope: it implements the scoped-frame core. *)

module Eval = Sicp_ch4.Sec_4_4
module Eval_error = Sicp_common.Eval_error
module Streams = Eval.Streams
module Value = Sicp_common.Value

(* Typed errors escape the evaluator to the driver's rendering. *)
exception Run_error of Eval_error.t

(* The Microshaft assertions and the section's outranked-by rule, in
   the book's order. *)
let assertions =
  [ "(assert! (job (Bitdiddle Ben) (computer wizard)))"
  ; "(assert! (job (Hacker Alyssa P) (computer programmer)))"
  ; "(assert! (salary (Hacker Alyssa P) 40000))"
  ; "(assert! (supervisor (Hacker Alyssa P) (Bitdiddle Ben)))"
  ; "(assert! (job (Fect Cy D) (computer programmer)))"
  ; "(assert! (salary (Fect Cy D) 35000))"
  ; "(assert! (supervisor (Fect Cy D) (Bitdiddle Ben)))"
  ; "(assert! (job (Tweakit Lem E) (computer technician)))"
  ; "(assert! (salary (Tweakit Lem E) 25000))"
  ; "(assert! (supervisor (Tweakit Lem E) (Bitdiddle Ben)))"
  ; "(assert! (job (Reasoner Louis) (computer programmer trainee)))"
  ; "(assert! (salary (Reasoner Louis) 30000))"
  ; "(assert! (supervisor (Reasoner Louis) (Hacker Alyssa P)))"
  ; "(assert! (supervisor (Bitdiddle Ben) (Warbucks Oliver)))"
  ; "(assert! (salary (Bitdiddle Ben) 60000))"
  ; "(assert! (supervisor (Scrooge Eben) (Warbucks Oliver)))"
  ; "(assert! (job (Scrooge Eben) (accounting chief accountant)))"
  ; "(assert! (salary (Scrooge Eben) 75000))"
  ; "(assert! (supervisor (Cratchet Robert) (Scrooge Eben)))"
  ; "(assert! (job (Cratchet Robert) (accounting scrivener)))"
  ; "(assert! (salary (Cratchet Robert) 18000))"
  ; "(assert! (supervisor (Aull DeWitt) (Warbucks Oliver)))"
  ; "(assert! (job (Aull DeWitt) (administration secretary)))"
  ; "(assert! (salary (Aull DeWitt) 25000))"
  ; "(assert! (rule (outranked-by ?staff-person ?boss)\n\
     (or (supervisor ?staff-person ?boss)\n\
     (and (supervisor ?staff-person ?middle-manager)\n\
     (outranked-by ?middle-manager ?boss)))))"
  ]
;;

let load_microshaft env =
  List.iter
    (fun text ->
       match Eval.run env text with
       | Ok Eval.Asserted -> ()
       | Ok (Eval.Answers _) -> failwith "an assertion answered as a query"
       | Error e -> failwith ("assertion failed: " ^ Eval_error.to_string e))
    assertions
;;

(* {2 Scopes}

    One scope is one frame of bindings over its parent's. The driver's
    query lives in the root; a rule application unifies its conclusion
    in a fresh child of the querying scope and evaluates its body
    there. *)

type scope =
  { params : (Value.t * Value.t) list
  ; local : (Value.t * Value.t) list
  ; owns : Value.t list
  ; share : (Value.t * Value.t) list
  ; parent : scope option
  }

let the_root = { params = []; local = []; owns = []; share = []; parent = None }

let extend_scope variable value scope =
  { scope with local = (variable, value) :: scope.local }
;;

(* [extend_var scope variable value] binds [variable]: a variable the
   conclusion shares with its caller (the query passed the same
   unbound variable, the by-reference case) is bound at the layer that
   owns the caller's variable, chasing the share chain upwards -- the
   environment form of renaming's pointer binding, so a body's match
   on the rule's parameter is visible as the caller's variable. Any
   other variable is bound in the layer's own locals. *)
let rec extend_var scope variable value =
  match List.find_opt (fun (p, _) -> Value.structural_equal p variable) scope.share with
  | Some (_, caller) ->
    (match scope.parent with
     | Some parent ->
       let relinked = extend_var parent caller value in
       { scope with parent = Some relinked }
     | None -> extend_scope variable value scope)
  | None -> extend_scope variable value scope
;;

(* [lookup_scope scope var] is the nearest binding of [var]: the layer's
   locals, then its parameters, then -- for a shared parameter -- the
   caller's binding of the paired variable; a variable the rule owns
   but no binding resolved is absence, never the caller's same-named
   binding, which is the environment discipline renaming substituted
   for. *)
let rec lookup_scope scope variable =
  let found table =
    List.find_opt (fun (v, _) -> Value.structural_equal v variable) table
  in
  let shared =
    List.find_opt (fun (p, _) -> Value.structural_equal p variable) scope.share
  in
  let owned vars = List.exists (fun v -> Value.structural_equal v variable) vars in
  match found scope.local with
  | Some (_, value) -> Some value
  | None ->
    (match found scope.params with
     | Some (_, value) -> Some value
     | None ->
       (match shared, scope.parent with
        | Some (_, caller), Some parent -> lookup_scope parent caller
        | _, Some _ when owned scope.owns -> None
        | _, Some parent -> lookup_scope parent variable
        | _, None -> None))
;;

(* {2 The scoped matcher and unifier}

    The substrate's matcher over substrate frames, re-owned over
    scopes: the only change is where a binding is stored and where a
    stored value is found. *)

let rec match_scoped pattern datum scope =
  if Value.structural_equal pattern datum
  then Some scope
  else if Eval.is_var pattern
  then extend_if_consistent pattern datum scope
  else (
    match Value.view pattern, Value.view datum with
    | Value.Pair (p_car, p_cdr), Value.Pair (d_car, d_cdr) ->
      (match match_scoped p_car d_car scope with
       | None -> None
       | Some scope -> match_scoped p_cdr d_cdr scope)
    | _ -> None)

and extend_if_consistent variable datum scope =
  match lookup_scope scope variable with
  | Some stored -> match_scoped stored datum scope
  | None -> Some (extend_var scope variable datum)
;;

(* {2 Applying a rule: parameters bind fresh, arguments read the chain}

    A rule application is a procedure call, not a symmetric unification:
    the conclusion's parameters are the rule's own, one application's
    bindings live in a fresh child and shadow their same-named
    ancestors (the environment discipline renaming substituted for),
    while the query's arguments are the caller's and are read through
    the chain -- a bound argument contributes its value, an unbound one
    is passed by reference so the body's matches constrain it. *)

type binder =
  { kid : (Value.t * Value.t) list
  ; owns : Value.t list
  ; share : (Value.t * Value.t) list
  ; outer : scope
  }

let rec bind_conclusion params args st =
  (* No whole-tree fast path: an identical subtree still carries the
     by-reference pairs the body's matches must write through. Only
     equal atomic constants bind nothing. *)
  if Eval.is_var params
  then bind_param params args st
  else (
    match Value.view params, Value.view args with
    | Value.Pair (p_car, p_cdr), Value.Pair (a_car, a_cdr) ->
      (match bind_conclusion p_car a_car st with
       | None -> None
       | Some st -> bind_conclusion p_cdr a_cdr st)
    | _ -> if Value.structural_equal params args then Some st else None)

and bind_param param arg st =
  (* A variable argument is a by-reference pass: the parameter and the
     caller's variable are aliased (share), whoever binds one binds
     both, at the layer that owns the caller's variable. A constant
     argument contributes a value binding, checked against any earlier
     binding of the parameter. *)
  if Eval.is_var arg
  then Some { st with share = (param, arg) :: st.share }
  else add_kid param arg st

(* [body_vars exp] is every rule variable the body mentions that the
   conclusion did not already bind or share. *)
and body_vars exp acc st =
  if Eval.is_var exp
  then
    if
      List.exists (fun (v, _) -> Value.structural_equal v exp) st.kid
      || List.exists (fun (v, _) -> Value.structural_equal v exp) st.share
      || List.exists (fun v -> Value.structural_equal v exp) acc
    then acc
    else exp :: acc
  else (
    match Value.view exp with
    | Value.Pair (car, cdr) -> body_vars cdr (body_vars car acc st) st
    | _ -> acc)

and add_kid param value st =
  match List.find_opt (fun (v, _) -> Value.structural_equal v param) st.kid with
  | Some (_, stored) ->
    (match
       match_scoped
         stored
         value
         { params = []; local = st.kid; owns = []; share = []; parent = Some st.outer }
     with
     | Some extended ->
       Some { kid = extended.local; owns = st.owns; share = st.share; outer = st.outer }
     | None -> None)
  | None ->
    Some
      { kid = (param, value) :: st.kid
      ; owns = st.owns
      ; share = st.share
      ; outer = st.outer
      }
;;

(* [instantiate_scoped exp scope unbound] copies [exp], resolving every
   variable through the chain. *)
let rec instantiate_scoped exp scope unbound =
  if Eval.is_var exp
  then (
    match lookup_scope scope exp with
    | Some value -> instantiate_scoped value scope unbound
    | None -> unbound exp scope)
  else (
    match Value.view exp with
    | Value.Pair (car, cdr) ->
      Value.pair
        (instantiate_scoped car scope unbound)
        (instantiate_scoped cdr scope unbound)
    | _ -> exp)
;;

let stream_to_list s =
  let rec go s acc =
    if Streams.stream_null s
    then List.rev acc
    else go (Streams.stream_cdr s) (Streams.stream_car s :: acc)
  in
  go s []
;;

let head_symbol query =
  match Value.view query with
  | Value.Pair (head, _) ->
    (match Value.view head with
     | Value.Symbol name -> Some name
     | _ -> None)
  | _ -> None
;;

let contents query =
  match Value.view query with
  | Value.Pair (_, body) ->
    (match Eval.value_list body with
     | Ok items -> items
     | Error e -> raise (Run_error e))
  | _ -> raise (Run_error (Eval_error.Invalid_form (Value.to_string query)))
;;

(* {2 The scoped evaluator}

    The section's clause structure unchanged; only frames are scopes. *)

let rec qeval_scoped env query frames =
  match head_symbol query with
  | Some "and" -> conjoin_scoped env (contents query) frames
  | Some "or" -> disjoin_scoped env (contents query) frames
  | Some "not" -> negate_scoped env (contents query) frames
  | Some "lisp-value" -> lisp_value_scoped env (contents query) frames
  | Some "always-true" -> frames
  | _ -> simple_query_scoped env query frames

and conjoin_scoped env conjuncts frames =
  match conjuncts with
  | [] -> frames
  | first :: rest -> conjoin_scoped env rest (qeval_scoped env first frames)

and disjoin_scoped env disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    Eval.interleave_delayed (qeval_scoped env first frames) (fun () ->
      disjoin_scoped env rest frames)

and simple_query_scoped env pattern frames =
  Eval.stream_flatmap
    (fun scope ->
       Eval.stream_append_delayed (find_assertions_scoped pattern scope) (fun () ->
         apply_rules_scoped env pattern scope))
    frames

and find_assertions_scoped pattern scope =
  Eval.stream_flatmap
    (fun assertion ->
       match match_scoped pattern assertion scope with
       | Some extended -> Eval.singleton_stream extended
       | None -> Streams.the_empty_stream)
    (* The fetchers ignore the frame; the book's signature. *)
    (Eval.fetch_assertions pattern Eval.the_empty_frame)

and apply_rules_scoped env pattern scope =
  Eval.stream_flatmap
    (fun rule -> apply_a_rule_scoped env rule pattern scope)
    (Eval.fetch_rules pattern Eval.the_empty_frame)

and apply_a_rule_scoped env rule pattern scope =
  (* No renaming: the conclusion's parameters bind in a fresh
     application layer over the querying scope, and the body is
     evaluated there -- the rule's variables shadow their same-named
     ancestors inside the call and read the caller's bindings through
     the chain. On return the layer is squashed into its parent: the
     body's caller-visible bindings join the parent's locals and the
     parameters disappear, exactly as a call's frame is discarded
     while the constraints it recorded on the caller's variables
     remain. *)
  match
    bind_conclusion
      (Eval.rule_conclusion rule)
      pattern
      { kid = []; owns = []; share = []; outer = scope }
  with
  | None -> Streams.the_empty_stream
  | Some st ->
    let owns = body_vars (Eval.rule_body rule) st.owns st @ List.map fst st.kid in
    let child =
      { params = st.kid; local = []; owns; share = st.share; parent = Some scope }
    in
    let body_frames =
      qeval_scoped env (Eval.rule_body rule) (Eval.singleton_stream child)
    in
    Streams.stream_map squash_return body_frames

(* [squash_return frame] discards the frame's innermost application
   layer, merging its locals into the parent's; by construction a
   layer's locals never mention its own parameters, so nothing of the
   call's bindings survives above it. *)
and squash_return frame =
  match frame.parent with
  | None -> frame
  | Some parent ->
    { params = parent.params
    ; local = frame.local @ parent.local
    ; owns = parent.owns
    ; share = parent.share
    ; parent = parent.parent
    }

and negate_scoped env operands frames =
  let query = Eval.first_operand operands in
  Eval.stream_flatmap
    (fun scope ->
       if Streams.stream_null (qeval_scoped env query (Eval.singleton_stream scope))
       then Eval.singleton_stream scope
       else Streams.the_empty_stream)
    frames

and lisp_value_scoped env operands frames =
  let call = Eval.first_operand operands in
  Eval.stream_flatmap
    (fun scope ->
       let instantiated =
         instantiate_scoped call scope (fun v _ ->
           raise
             (Run_error
                (Eval_error.User_error ("Unknown pat var LISP-VALUE: " ^ Value.to_string v))))
       in
       match Eval.execute env instantiated with
       | Ok true -> Eval.singleton_stream scope
       | Ok false -> Streams.the_empty_stream
       | Error e -> raise (Run_error e))
    frames
;;

(** [query_scoped env text] is one query through the scoped engine:
    every rendered answer, or the typed error. *)
let query_scoped env text =
  match Eval.read_query text with
  | Error message -> [ "Error: " ^ message ]
  | Ok raw ->
    let query = Eval.query_syntax_process raw in
    (try
       let frames =
         qeval_scoped
           env
           query
           (Streams.cons_stream the_root (fun () -> Streams.the_empty_stream))
       in
       List.map
         (fun scope ->
            Value.to_string
              (instantiate_scoped query scope (fun v _ -> Eval.contract_question_mark v)))
         (stream_to_list frames)
     with
     | Run_error e -> [ "Error: " ^ Eval_error.to_string e ])
;;

(** [stream_answers env text] is the renaming engine's rendering. *)
let stream_answers env text =
  match Eval.query env text with
  | Ok answers -> List.map Value.to_string answers
  | Error e -> [ "Error: " ^ Eval_error.to_string e ]
;;

(* One block: the query, the renaming engine's answers, the scoped
   engine's answers, and the equality verdict with the answer count. *)
let compare env text =
  let renaming = stream_answers env text in
  let scoped = query_scoped env text in
  let identical = List.equal String.equal renaming scoped in
  [ text; "renaming:" ]
  @ renaming
  @ [ "scoped:" ]
  @ scoped
  @ [ ("renaming=scoped: "
       ^
       if identical
       then "true"
       else "false" ^ " (" ^ string_of_int (List.length scoped) ^ " answers)")
    ]
;;

let ex_4_79 () =
  let env = Eval.the_query_system () in
  load_microshaft env;
  List.concat_map
    (compare env)
    [ "(outranked-by (Bitdiddle Ben) ?who)"
    ; "(outranked-by ?staff-person ?boss)"
    ; "(and (salary ?staff-person ?amount) (outranked-by ?staff-person ?boss))"
    ]
;;
