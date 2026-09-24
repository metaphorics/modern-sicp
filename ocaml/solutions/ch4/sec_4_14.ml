(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.14 *)

(** Exercise 4.14: Louis installs the host [map] as a primitive. A
    primitive receives evaluated values and cannot apply an
    object-language procedure -- the primitive protocol has no [apply]
    -- so Louis's [map] fails on a compound mapping argument, while
    Eva's object-language [map] applies [f] through the evaluator and
    works. The evaluator's own dispatch resolves primitives against this
    module's table, the section's table plus the host [map] and [sort];
    the tailored addition 4.14a repeats the story for the host [sort].
    The dispatch follows the section's [Core]; only the primitive table
    and [apply] are its own. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** [entries_of v] is the value list of [v] as a host list. *)
let rec entries_of (v : Value.t) : (Value.t list, Eval_error.t) result =
  match Value.view v with
  | Value.Nil -> Ok []
  | Value.Pair (car, cdr) -> entries_of cdr >>= fun rest -> Ok (car :: rest)
  | _ -> Error (Eval_error.Type_error ("not a list: " ^ Value.to_string v))
;;

(** [list_value items] is the value list of the host list [items]. *)
let list_value items = List.fold_right Value.pair items Value.nil

(** The host-primitive layer. [table_entry] resolves a primitive's name
    through the section's table, then through the host entries, so a
    primitive-valued argument maps or sorts fine; [resolve] turns a
    non-primitive into the typed error the host's typing produces. *)
let rec table_entry name =
  match List.assoc_opt name SE.primitive_table with
  | Some f -> Some f
  | None -> host_entry name

and host_entry = function
  | "map" -> Some host_map
  | "sort" -> Some host_sort
  | _ -> None

and resolve op role proc =
  match Value.view proc with
  | Value.Primitive_procedure name ->
    (match table_entry name with
     | Some f -> Ok f
     | None ->
       Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
  | Value.Compound_procedure _ ->
    Error
      (Eval_error.Type_error
         (op ^ ": the " ^ role ^ " argument is not a primitive procedure"))
  | _ ->
    Error
      (Eval_error.Type_error
         (op ^ ": the " ^ role ^ " argument is not a procedure: " ^ Value.to_string proc))

and host_map (args : Value.t list) : (Value.t, Eval_error.t) result =
  match args with
  | [ proc; lst ] ->
    entries_of lst
    >>= fun items ->
    resolve "map" "mapping" proc
    >>= fun f ->
    let rec go = function
      | [] -> Ok Value.nil
      | x :: rest -> f [ x ] >>= fun y -> go rest >>= fun ys -> Ok (Value.pair y ys)
    in
    go items
  | args -> Error (Eval_error.Arity_mismatch { expected = 2; given = List.length args })

and insert cmp x sorted =
  match sorted with
  | [] -> Ok [ x ]
  | y :: rest ->
    cmp [ x; y ]
    >>= fun before ->
    if SE.true_ before
    then Ok (x :: y :: rest)
    else insert cmp x rest >>= fun rest -> Ok (y :: rest)

and sort_with cmp items =
  match items with
  | [] -> Ok []
  | x :: rest -> sort_with cmp rest >>= fun sorted -> insert cmp x sorted

and host_sort (args : Value.t list) : (Value.t, Eval_error.t) result =
  match args with
  | [ lst; cmp ] ->
    entries_of lst
    >>= fun items ->
    resolve "sort" "comparator" cmp
    >>= fun f -> sort_with f items >>= fun sorted -> Ok (list_value sorted)
  | args -> Error (Eval_error.Arity_mismatch { expected = 2; given = List.length args })
;;

(** [table] is this evaluator's primitive table: the section's table
    plus Louis's host [map] and [sort]. *)
let table = SE.primitive_table @ [ "map", host_map; "sort", host_sort ]

(** The evaluator: the section's standard dispatch with [apply_procedure]
    resolving against [table]. [Core] contributes the operand, sequence,
    if, assignment, and definition clauses, all recursing through
    [Ev.eval]; application is applied here so host primitives resolve. *)
module rec Ev : sig
  val eval : SE.eval_t
  val apply_procedure : Value.t -> Value.t list -> (Value.t, Eval_error.t) result
end = struct
  module C = SE.Core (Ev)

  let apply_procedure proc args =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure cv ->
      SE.extend_environment cv.parameters args cv.env
      >>= fun extended -> C.eval_sequence cv.body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> SE.lookup_variable_value name env
    | Ast.Quote datum -> Ok (SE.datum_to_value datum)
    | Ast.Definition d -> C.eval_definition d env
    | Ast.Set (name, e) -> C.eval_assignment name e env
    | Ast.If _ -> C.eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ -> SE.cond_to_if exp >>= fun lowered -> C.eval lowered env
    | Ast.Application (operator, operands) ->
      Ev.eval operator env
      >>= fun proc ->
      C.list_of_values operands env >>= fun args -> Ev.apply_procedure proc args
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type")
  ;;
end

(** [eval] is the evaluator with the host-flavored [map] and [sort]
      installed, the state Louis creates. *)
let eval : SE.eval_t = Ev.eval

(** [louis_environment ()] is a fresh global environment with the host
    [map] and [sort] installed. *)
let louis_environment () =
  let env = SE.the_global_environment () in
  Value.env_define env "map" (Value.primitive ~name:"map" host_map);
  Value.env_define env "sort" (Value.primitive ~name:"sort" host_sort);
  env
;;

(** [object_environment definitions] is a fresh global environment with
    [definitions] evaluated by this evaluator, the state Eva creates. *)
let object_environment definitions =
  let env = SE.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = SE.run_program env definitions in
  env
;;

let eva_map = "(define (map f l) (if (null? l) '() (cons (f (car l)) (map f (cdr l)))))"

let eva_insert =
  "(define (insert x sorted before?) (if (null? sorted) (list x) (if (before? x (car \
   sorted)) (cons x sorted) (cons (car sorted) (insert x (cdr sorted) before?)))))"
;;

let eva_insertion_sort =
  "(define (insertion-sort l before?) (if (null? l) '() (insert (car l) (insertion-sort \
   (cdr l) before?) before?)))"
;;

(** [run env text] reads one form of [text] and evaluates it with this
    module's evaluator. *)
let run env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> eval exp env
  | Error e -> Error (Eval_error.Invalid_form (Sicp_common.Reader.to_string e))
;;

(** [render r] is the printed outcome of one demonstration step. *)
let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_14 ()] runs the statement's story: Eva's object-language map
    squares a list, Louis's host map maps a primitive over a list, and
    Louis's host map fails on the compound squaring procedure. *)
let ex_4_14 () =
  let eva = object_environment eva_map in
  let louis = louis_environment () in
  let squaring = "(map (lambda (x) (* x x)) '(1 2 3))" in
  [ render (run eva squaring)
  ; render (run louis "(map car '((1 2) (3 4)))")
  ; render (run louis squaring)
  ]
;;

(** [ex_4_14a ()] is the tailored addition: the host sort sorts under a
    primitive comparator and fails on a compound one, while the
    object-language insertion sort sorts under the same compound
    comparator. *)
let ex_4_14a () =
  let eva = object_environment (eva_insert ^ " " ^ eva_insertion_sort) in
  let louis = louis_environment () in
  let descending = "(lambda (a b) (> a b))" in
  [ render (run louis "(sort '(3 1 2) <)")
  ; render (run louis ("(sort '(3 1 2) " ^ descending ^ ")"))
  ; render (run eva ("(insertion-sort '(3 1 2) " ^ descending ^ ")"))
  ]
;;
