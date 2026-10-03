(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 4.4 *)

module Eval_error = Sicp_common.Eval_error
module Data = Sicp_common.Constructor_data
module Streams = Sicp_ch3.Sec_3_5.Streams

let ( let* ) = Result.bind

type variable =
  { name : string
  ; id : int
  }

type term =
  | Atom of string
  | Num of int
  | Str of string
  | Var of variable
  | Nil
  | Pair of term * term

type query =
  | Pattern of term
  | And of query list
  | Or of query list
  | Not of query
  | Holds of string * term list
  | Always_true
  | Form of string * query list

type command =
  | Assert of term
  | Rule of term * query
  | Query of query

type frame = (variable * term) list

exception Query_error of Eval_error.t

let list items = List.fold_right (fun head tail -> Pair (head, tail)) items Nil
let dotted items tail = List.fold_right (fun head tail -> Pair (head, tail)) items tail
let var name = Var { name; id = 0 }

let render_variable v =
  if v.id = 0 then "?" ^ v.name else Printf.sprintf "?%s.%d" v.name v.id
;;

let rec render_term = function
  | Atom a -> a
  | Num n -> string_of_int n
  | Str s -> Printf.sprintf "%S" s
  | Var v -> render_variable v
  | Nil -> "[]"
  | Pair (head, tail) ->
    let rec items acc = function
      | Pair (h, t) -> items (render_term h :: acc) t
      | Nil -> List.rev acc, None
      | tail -> List.rev acc, Some (render_term tail)
    in
    let parts, tail = items [ render_term head ] tail in
    (match tail with
     | None -> "[" ^ String.concat ", " parts ^ "]"
     | Some t -> "[" ^ String.concat ", " parts ^ " | " ^ t ^ "]")
;;

let rec render_query = function
  | Pattern t -> render_term t
  | And qs -> "and(" ^ String.concat ", " (List.map render_query qs) ^ ")"
  | Or qs -> "or(" ^ String.concat ", " (List.map render_query qs) ^ ")"
  | Not q -> "not(" ^ render_query q ^ ")"
  | Holds (p, ts) -> "holds(" ^ String.concat ", " (p :: List.map render_term ts) ^ ")"
  | Always_true -> "always-true"
  | Form (name, qs) -> name ^ "(" ^ String.concat ", " (List.map render_query qs) ^ ")"
;;

(* {1 4.4.4.8: frames} *)

let same_variable a b = a.id = b.id && String.equal a.name b.name

let binding_in_frame v frame =
  List.find_map (fun (w, t) -> if same_variable v w then Some t else None) frame
;;

let extend v t frame = (v, t) :: frame

(* {1 4.4.4.3 and 4.4.4.4: matching and unification} *)

let rec pattern_match pattern datum frame =
  match pattern, datum with
  | Var v, _ -> extend_if_consistent v datum frame
  | Pair (p1, p2), Pair (d1, d2) ->
    Option.bind (pattern_match p1 d1 frame) (pattern_match p2 d2)
  | Atom a, Atom b when String.equal a b -> Some frame
  | Num a, Num b when a = b -> Some frame
  | Str a, Str b when String.equal a b -> Some frame
  | Nil, Nil -> Some frame
  | _ -> None

and extend_if_consistent v datum frame =
  match binding_in_frame v frame with
  | Some bound -> pattern_match bound datum frame
  | None -> Some (extend v datum frame)
;;

let rec depends_on t v frame =
  match t with
  | Var w when same_variable v w -> true
  | Var w ->
    (match binding_in_frame w frame with
     | Some bound -> depends_on bound v frame
     | None -> false)
  | Pair (a, b) -> depends_on a v frame || depends_on b v frame
  | Atom _ | Num _ | Str _ | Nil -> false
;;

let rec unify_match a b frame =
  match a, b with
  | Var v, Var w when same_variable v w -> Some frame
  | Var v, _ -> extend_if_possible v b frame
  | _, Var w -> extend_if_possible w a frame
  | Pair (a1, a2), Pair (b1, b2) ->
    Option.bind (unify_match a1 b1 frame) (unify_match a2 b2)
  | Atom x, Atom y when String.equal x y -> Some frame
  | Num x, Num y when x = y -> Some frame
  | Str x, Str y when String.equal x y -> Some frame
  | Nil, Nil -> Some frame
  | _ -> None

and extend_if_possible v value frame =
  match binding_in_frame v frame, value with
  | Some bound, _ -> unify_match bound value frame
  | None, Var w ->
    (match binding_in_frame w frame with
     | Some bound -> unify_match (Var v) bound frame
     | None -> Some (extend v value frame))
  | None, _ -> if depends_on value v frame then None else Some (extend v value frame)
;;

let rec instantiate t frame unbound =
  match t with
  | Var v ->
    (match binding_in_frame v frame with
     | Some bound -> instantiate bound frame unbound
     | None -> unbound v)
  | Pair (a, b) -> Pair (instantiate a frame unbound, instantiate b frame unbound)
  | Atom _ | Num _ | Str _ | Nil -> t
;;

let rec map_terms f = function
  | Pattern t -> Pattern (f t)
  | And qs -> And (List.map (map_terms f) qs)
  | Or qs -> Or (List.map (map_terms f) qs)
  | Not q -> Not (map_terms f q)
  | Holds (p, ts) -> Holds (p, List.map f ts)
  | Always_true -> Always_true
  | Form (name, qs) -> Form (name, List.map (map_terms f) qs)
;;

let rename_variables_in (conclusion, body) id =
  let rec rename = function
    | Var v -> Var { v with id }
    | Pair (a, b) -> Pair (rename a, rename b)
    | (Atom _ | Num _ | Str _ | Nil) as t -> t
  in
  rename conclusion, map_terms rename body
;;

let instantiate_query q frame =
  map_terms (fun t -> instantiate t frame (fun v -> Var v)) q
;;

(* {1 4.4.4.6: streams} *)

let singleton_stream x = Streams.cons_stream x (fun () -> Streams.the_empty_stream)

let rec stream_append_delayed s1 delayed_s2 =
  match s1 with
  | Streams.Empty -> delayed_s2 ()
  | Streams.Cons (head, tail) ->
    Streams.cons_stream head (fun () ->
      stream_append_delayed (Lazy.force tail) delayed_s2)
;;

let rec interleave_delayed s1 delayed_s2 =
  match s1 with
  | Streams.Empty -> delayed_s2 ()
  | Streams.Cons (head, tail) ->
    Streams.cons_stream head (fun () ->
      interleave_delayed (delayed_s2 ()) (fun () -> Lazy.force tail))
;;

let rec flatten_stream = function
  | Streams.Empty -> Streams.the_empty_stream
  | Streams.Cons (head, tail) ->
    interleave_delayed head (fun () -> flatten_stream (Lazy.force tail))
;;

let stream_flatmap f s = flatten_stream (Streams.stream_map f s)

let rec list_to_stream = function
  | [] -> Streams.the_empty_stream
  | x :: rest -> Streams.cons_stream x (fun () -> list_to_stream rest)
;;

(* {1 4.4.4.5: the data base} *)

type session =
  { mutable assertions : term list
  ; mutable rules : (term * query) list
  ; mutable counter : int
  ; handlers : (string, handler) Hashtbl.t
  }

and handler = session -> query list -> frame Streams.stream -> frame Streams.stream

let new_session () =
  { assertions = []; rules = []; counter = 0; handlers = Hashtbl.create 8 }
;;

let put session name handler = Hashtbl.replace session.handlers name handler
let add_assertion session t = session.assertions <- session.assertions @ [ t ]

let add_rule session conclusion body =
  session.rules <- session.rules @ [ conclusion, body ]
;;

let fetch_assertions session _pattern = list_to_stream session.assertions
let fetch_rules session _pattern = list_to_stream session.rules

let new_rule_application_id session =
  session.counter <- session.counter + 1;
  session.counter
;;

(* {1 4.4.4.1 and 4.4.4.2: the evaluator} *)

let fail e = raise (Query_error e)

let predicate name args =
  let compare_terms a b =
    match a, b with
    | Num x, Num y -> Int.compare x y
    | Str x, Str y -> String.compare x y
    | _ -> fail (Eval_error.Type_error ("holds " ^ name ^ " compares numbers or strings"))
  in
  match name, args with
  | "<", [ a; b ] -> compare_terms a b < 0
  | ">", [ a; b ] -> compare_terms a b > 0
  | "<=", [ a; b ] -> compare_terms a b <= 0
  | ">=", [ a; b ] -> compare_terms a b >= 0
  | "=", [ a; b ] -> compare_terms a b = 0
  | "<>", [ a; b ] -> compare_terms a b <> 0
  | _ -> fail (Eval_error.Unknown_operation ("holds " ^ name))
;;

let rec qeval session q frames =
  match q with
  | Pattern p -> stream_flatmap (fun frame -> simple_query session p frame) frames
  | And conjuncts ->
    List.fold_left (fun frames c -> qeval session c frames) frames conjuncts
  | Or disjuncts -> disjoin session disjuncts frames
  | Not inner ->
    Streams.stream_filter
      (fun frame -> Streams.stream_null (qeval session inner (singleton_stream frame)))
      frames
  | Holds (name, args) ->
    Streams.stream_filter
      (fun frame ->
         predicate
           name
           (List.map
              (fun t ->
                 instantiate t frame (fun v ->
                   fail
                     (Eval_error.Unbound_variable ("holds on unbound " ^ render_variable v))))
              args))
      frames
  | Always_true -> frames
  | Form (name, operands) ->
    (match Hashtbl.find_opt session.handlers name with
     | Some handler -> handler session operands frames
     | None -> fail (Eval_error.Invalid_form ("unknown query form " ^ name)))

and disjoin session disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    interleave_delayed (qeval session first frames) (fun () ->
      disjoin session rest frames)

and simple_query session pattern frame =
  stream_append_delayed (find_assertions session pattern frame) (fun () ->
    apply_rules session pattern frame)

and find_assertions session pattern frame =
  stream_flatmap
    (fun datum ->
       match pattern_match pattern datum frame with
       | Some extended -> singleton_stream extended
       | None -> Streams.the_empty_stream)
    (fetch_assertions session pattern)

and apply_rules session pattern frame =
  stream_flatmap
    (fun rule -> apply_a_rule session rule pattern frame)
    (fetch_rules session pattern)

and apply_a_rule session rule pattern frame =
  let conclusion, body = rename_variables_in rule (new_rule_application_id session) in
  match unify_match pattern conclusion frame with
  | Some unified -> qeval session body (singleton_stream unified)
  | None -> Streams.the_empty_stream
;;

let answers session q =
  match
    Streams.stream_map (instantiate_query q) (qeval session q (singleton_stream []))
  with
  | s -> Ok s
  | exception Query_error e -> Error e
;;

(* An answer's unbound rule variables are numbered by first occurrence
   within the answer, so the printed form does not depend on how many
   rule applications the search happened to make before it. *)
let canonical q =
  let seen = ref [] in
  let rec term = function
    | Var v when v.id <> 0 ->
      (match List.find_opt (fun (w, _) -> same_variable v w) !seen with
       | Some (_, k) -> Var { v with id = k }
       | None ->
         let k = List.length !seen + 1 in
         seen := !seen @ [ v, k ];
         Var { v with id = k })
    | Pair (a, b) ->
      let a = term a in
      Pair (a, term b)
    | t -> t
  in
  map_terms term q
;;

let run ~emit session commands =
  let rec each s =
    match s with
    | Streams.Empty -> ()
    | Streams.Cons (answer, tail) ->
      emit (render_query (canonical answer) ^ "\n");
      each (Lazy.force tail)
  in
  let command = function
    | Assert t -> Ok (add_assertion session t)
    | Rule (conclusion, body) -> Ok (add_rule session conclusion body)
    | Query q ->
      emit ("? " ^ render_query q ^ "\n");
      let* s = answers session q in
      (match each s with
       | () -> Ok ()
       | exception Query_error e -> Error e)
  in
  List.fold_left (fun acc c -> Result.bind acc (fun () -> command c)) (Ok ()) commands
;;

(* {1 The fixture decoder} *)

let wanted what d = Error (Printf.sprintf "expected %s, found %s" what (Data.describe d))

let rec each f = function
  | [] -> Ok []
  | x :: rest ->
    let* y = f x in
    let* ys = each f rest in
    Ok (y :: ys)
;;

let rec decode_term = function
  | Data.Ctor ("Atom", [ Data.String a ]) -> Ok (Atom a)
  | Data.Ctor ("Num", [ Data.Int n ]) -> Ok (Num n)
  | Data.Ctor ("Str", [ Data.String s ]) -> Ok (Str s)
  | Data.Ctor ("Var", [ Data.String v ]) -> Ok (var v)
  | Data.Ctor ("List", [ Data.List items ]) -> Result.map list (each decode_term items)
  | Data.Ctor ("Dotted", [ Data.List items; tail ]) ->
    let* items = each decode_term items in
    let* tail = decode_term tail in
    Ok (dotted items tail)
  | d -> wanted "a term (Atom, Num, Str, Var, List, Dotted)" d
;;

let rec decode_query = function
  | Data.Ctor ("Pattern", [ t ]) -> Result.map (fun t -> Pattern t) (decode_term t)
  | Data.Ctor ("And", [ Data.List qs ]) ->
    Result.map (fun qs -> And qs) (each decode_query qs)
  | Data.Ctor ("Or", [ Data.List qs ]) ->
    Result.map (fun qs -> Or qs) (each decode_query qs)
  | Data.Ctor ("Not", [ q ]) -> Result.map (fun q -> Not q) (decode_query q)
  | Data.Ctor ("Holds", [ Data.String p; Data.List ts ]) ->
    Result.map (fun ts -> Holds (p, ts)) (each decode_term ts)
  | Data.Ctor ("Always_true", []) -> Ok Always_true
  | d -> wanted "a query (Pattern, And, Or, Not, Holds, Always_true)" d
;;

let decode_command = function
  | Data.Ctor ("Assert", [ t ]) -> Result.map (fun t -> Assert t) (decode_term t)
  | Data.Ctor ("Rule", [ conclusion; body ]) ->
    let* conclusion = decode_term conclusion in
    let* body = decode_query body in
    Ok (Rule (conclusion, body))
  | Data.Ctor ("Query", [ q ]) -> Result.map (fun q -> Query q) (decode_query q)
  | d -> wanted "a command (Assert, Rule, Query)" d
;;

let read_fixture ~filename text =
  let* d = Data.read ~filename text in
  match d with
  | Data.List commands -> each decode_command commands
  | d -> wanted "a list of commands" d
;;
