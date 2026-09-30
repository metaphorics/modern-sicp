(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.79: rule application with environments instead of
   renaming.  Where the section's engine renames every rule variable
   under a fresh application id, this engine applies a rule in a layer
   of its own, shaped like a procedure-call frame: the layer's [params]
   hold the unrenamed conclusion's bindings, fixed for the application,
   and its [local] collects what the body's matches add.  Variable
   lookup is local, then parameters, then the parent chain outwards,
   exactly as a procedure body sees its own parameters and, through the
   chain, its caller's bindings; a recursive application's parameters
   shadow their same-named ancestors only inside the call, because on
   return the layer is squashed into its parent -- the body's
   caller-visible bindings join the parent's locals and the parameters
   vanish, as a call's frame is discarded while the constraints it
   recorded on the caller's variables remain.  A rule application is a
   directed parameter binding (a procedure call), not a symmetric
   unification: parameters bind fresh, argument variables are read
   through the chain -- a bound argument contributes its value, an
   unbound one is passed by reference.  The data base and the stream
   combinators are the engine's, and the evaluator mirrors its clause
   structure, so answer order is the stream system's.  The demo pins
   equivalence with the renaming engine on the recursive [outranked-by]
   rule over Microshaft, including a query whose bound variable must
   flow into the rule through the chain.  The exercise's open-ended
   reaches -- block-structured rule environments, deduction in a
   supposed context -- are beyond this edition's scope: it implements
   the scoped-frame core. *)

open Sec_4_55.Kit

(* One scope is one frame of bindings over its parent's.  The driver's
   query lives in the root; a rule application binds its conclusion in a
   fresh child of the querying scope and evaluates its body there. *)
type scope =
  { params : (Q.variable * Q.term) list
  ; local : (Q.variable * Q.term) list
  ; owns : Q.variable list
  ; share : (Q.variable * Q.variable) list
  ; parent : scope option
  }

let the_root = { params = []; local = []; owns = []; share = []; parent = None }

let extend_scope variable value scope =
  { scope with local = (variable, value) :: scope.local }
;;

(* A variable the conclusion shares with its caller (the query passed an
   unbound variable, the by-reference case) is bound at the layer that
   owns the caller's variable, chasing the share chain upwards; any other
   variable is bound in the layer's own locals. *)
let rec extend_var scope variable value =
  match List.assoc_opt variable scope.share, scope.parent with
  | Some caller, Some parent ->
    { scope with parent = Some (extend_var parent caller value) }
  | _ -> extend_scope variable value scope
;;

(* The nearest binding of [variable]: the layer's locals, then its
   parameters, then -- for a shared parameter -- the caller's binding of
   the paired variable; a variable the rule owns but no binding resolved
   is absence, never the caller's same-named binding. *)
let rec lookup_scope scope variable =
  match List.assoc_opt variable scope.local with
  | Some value -> Some value
  | None ->
    (match List.assoc_opt variable scope.params with
     | Some value -> Some value
     | None ->
       (match List.assoc_opt variable scope.share, scope.parent with
        | Some caller, Some parent -> lookup_scope parent caller
        | None, Some _ when List.mem variable scope.owns -> None
        | None, Some parent -> lookup_scope parent variable
        | _, None -> None))
;;

(* The engine's matcher re-owned over scopes: the only change is where a
   binding is stored and where a stored value is found. *)
let rec match_scoped pattern datum scope =
  if pattern = datum
  then Some scope
  else (
    match pattern, datum with
    | Q.Var variable, _ ->
      (match lookup_scope scope variable with
       | Some stored -> match_scoped stored datum scope
       | None -> Some (extend_var scope variable datum))
    | Q.Pair (p1, p2), Q.Pair (d1, d2) ->
      Option.bind (match_scoped p1 d1 scope) (match_scoped p2 d2)
    | _ -> None)
;;

(* The binder of one application: the parameters' values, the
   by-reference pairs, and the querying scope. *)
type binder =
  { kid : (Q.variable * Q.term) list
  ; kid_share : (Q.variable * Q.variable) list
  ; outer : scope
  }

(* No whole-tree fast path: an identical subtree still carries the
   by-reference pairs the body's matches must write through.  Only equal
   atomic constants bind nothing. *)
let rec bind_conclusion params args st =
  match params, args with
  | Q.Var param, Q.Var arg -> Some { st with kid_share = (param, arg) :: st.kid_share }
  | Q.Var param, _ -> add_kid param args st
  | Q.Pair (p1, p2), Q.Pair (a1, a2) ->
    Option.bind (bind_conclusion p1 a1 st) (bind_conclusion p2 a2)
  | _ -> if params = args then Some st else None

(* A constant argument contributes a value binding, checked against any
   earlier binding of the parameter. *)
and add_kid param value st =
  match List.assoc_opt param st.kid with
  | Some stored ->
    Option.map
      (fun extended -> { st with kid = extended.local })
      (match_scoped
         stored
         value
         { params = []; local = st.kid; owns = []; share = []; parent = Some st.outer })
  | None -> Some { st with kid = (param, value) :: st.kid }
;;

let rec term_vars acc = function
  | Q.Var variable -> if List.mem variable acc then acc else variable :: acc
  | Q.Pair (a, b) -> term_vars (term_vars acc a) b
  | Q.Atom _ | Q.Num _ | Q.Str _ | Q.Nil -> acc
;;

let rec query_vars acc = function
  | Q.Pattern t -> term_vars acc t
  | Q.And qs | Q.Or qs | Q.Form (_, qs) -> List.fold_left query_vars acc qs
  | Q.Not q -> query_vars acc q
  | Q.Holds (_, ts) -> List.fold_left term_vars acc ts
  | Q.Always_true -> acc
;;

(* Every rule variable the body mentions that the conclusion did not
   already bind or share. *)
let body_vars body st =
  List.filter
    (fun variable ->
       not (List.mem_assoc variable st.kid || List.mem_assoc variable st.kid_share))
    (query_vars [] body)
;;

let rec instantiate_scoped t scope =
  match t with
  | Q.Var variable ->
    (match lookup_scope scope variable with
     | Some value -> instantiate_scoped value scope
     | None -> t)
  | Q.Pair (a, b) -> Q.Pair (instantiate_scoped a scope, instantiate_scoped b scope)
  | Q.Atom _ | Q.Num _ | Q.Str _ | Q.Nil -> t
;;

let rec map_terms f = function
  | Q.Pattern t -> Q.Pattern (f t)
  | Q.And qs -> Q.And (List.map (map_terms f) qs)
  | Q.Or qs -> Q.Or (List.map (map_terms f) qs)
  | Q.Not q -> Q.Not (map_terms f q)
  | Q.Holds (name, ts) -> Q.Holds (name, List.map f ts)
  | Q.Always_true -> Q.Always_true
  | Q.Form (name, qs) -> Q.Form (name, List.map (map_terms f) qs)
;;

(* The section's clause structure unchanged; only frames are scopes. *)
let rec qeval_scoped s q frames =
  match q with
  | Q.Pattern pattern ->
    Q.stream_flatmap
      (fun scope ->
         Q.stream_append_delayed (find_assertions_scoped s pattern scope) (fun () ->
           apply_rules_scoped s pattern scope))
      frames
  | Q.And conjuncts ->
    List.fold_left (fun frames c -> qeval_scoped s c frames) frames conjuncts
  | Q.Or disjuncts -> disjoin_scoped s disjuncts frames
  | Q.Not inner ->
    Streams.stream_filter
      (fun scope -> Streams.stream_null (qeval_scoped s inner (Q.singleton_stream scope)))
      frames
  | Q.Holds (name, args) ->
    (* The engine's predicate decides the instantiated test; an argument
       still unbound is the engine's own unbound-variable error. *)
    Streams.stream_filter
      (fun scope ->
         let test = Q.Holds (name, List.map (fun t -> instantiate_scoped t scope) args) in
         not (Streams.stream_null (Q.qeval s test (Q.singleton_stream []))))
      frames
  | Q.Always_true -> frames
  | Q.Form (name, _) ->
    raise
      (Q.Query_error
         (Sicp_common.Eval_error.Invalid_form ("the scoped engine has no form " ^ name)))

and disjoin_scoped s disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    Q.interleave_delayed (qeval_scoped s first frames) (fun () ->
      disjoin_scoped s rest frames)

and find_assertions_scoped s pattern scope =
  Q.stream_flatmap
    (fun assertion ->
       match match_scoped pattern assertion scope with
       | Some extended -> Q.singleton_stream extended
       | None -> Streams.the_empty_stream)
    (Q.fetch_assertions s pattern)

and apply_rules_scoped s pattern scope =
  Q.stream_flatmap
    (fun rule -> apply_a_rule_scoped s rule pattern scope)
    (Q.fetch_rules s pattern)

(* No renaming: the conclusion's parameters bind in a fresh application
   layer over the querying scope, and the body is evaluated there. *)
and apply_a_rule_scoped s (conclusion, body) pattern scope =
  match
    bind_conclusion conclusion pattern { kid = []; kid_share = []; outer = scope }
  with
  | None -> Streams.the_empty_stream
  | Some st ->
    let child =
      { params = st.kid
      ; local = []
      ; owns = body_vars body st @ List.map fst st.kid
      ; share = st.kid_share
      ; parent = Some scope
      }
    in
    Streams.stream_map squash_return (qeval_scoped s body (Q.singleton_stream child))

(* The frame's innermost application layer is discarded and its locals
   merged into the parent's; by construction a layer's locals never
   mention its own parameters, so nothing of the call's bindings
   survives above it. *)
and squash_return frame =
  match frame.parent with
  | None -> frame
  | Some parent -> { parent with local = frame.local @ parent.local }
;;

let query_scoped s q =
  let answers = Dynarray.create () in
  match
    Streams.stream_for_each
      (fun scope ->
         Dynarray.add_last
           answers
           (Q.render_query (map_terms (fun t -> instantiate_scoped t scope) q)))
      (qeval_scoped s q (Q.singleton_stream the_root))
  with
  | () -> Dynarray.to_list answers
  | exception Q.Query_error e ->
    Dynarray.to_list answers @ [ "error: " ^ Sicp_common.Eval_error.to_string e ]
;;

let compare s q =
  let renaming = answers_all s q in
  let scoped = query_scoped s q in
  [ "? " ^ Q.render_query q; "renaming:" ]
  @ renaming
  @ [ "scoped:" ]
  @ scoped
  @ [ (if List.equal String.equal renaming scoped
       then "renaming=scoped: true"
       else Printf.sprintf "renaming=scoped: false (%d answers)" (List.length scoped))
    ]
;;

let demo_data =
  List.filter
    (function
      | Q.Pair (Q.Atom ("job" | "salary" | "supervisor"), _) -> true
      | _ -> false)
    microshaft
;;

let rules =
  [ ( l [ at "outranked-by"; v "staff-person"; v "boss" ]
    , Q.Or
        [ p [ at "supervisor"; v "staff-person"; v "boss" ]
        ; Q.And
            [ p [ at "supervisor"; v "staff-person"; v "middle-manager" ]
            ; p [ at "outranked-by"; v "middle-manager"; v "boss" ]
            ]
        ] )
  ]
;;

let ex_4_79 () =
  let s = session ~rules demo_data in
  List.concat_map
    (compare s)
    [ p [ at "outranked-by"; person "Bitdiddle Ben"; v "who" ]
    ; p [ at "outranked-by"; v "staff-person"; v "boss" ]
    ; Q.And
        [ p [ at "salary"; v "staff-person"; v "amount" ]
        ; p [ at "outranked-by"; v "staff-person"; v "boss" ]
        ]
    ]
;;
