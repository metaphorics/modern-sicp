(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.3 *)

(** The nondeterministic evaluator of section 4.3 on the 4.1 substrate:
    the same typed [Ast], [Value], [Env], [Eval_error], and reader as
    4.1 and 4.2, with the section's search machinery built on OCaml 5
    effect handlers.

    An execution procedure is a continuation-passing step
    [env -> succeed -> unit]: it either calls its success continuation
    with a value -- and the whole future use of that value stays nested
    inside the call -- or it aborts the current branch by performing the
    [Fail] effect. There is no failure continuation to thread: a failure
    unwinds the host stack to the innermost handler installed for the
    branch, which is exactly the most recent choice point, so the search
    is chronological by construction.

    [choice] is the handler of the [Amb] effect: an [amb] form performs
    [Amb] carrying its analyzed alternatives, and the handler runs them
    one at a time under the success continuation in hand, intercepting a
    [Fail] from a tried alternative and moving to the next one. An
    exhausted alternative list performs [Fail] to the enclosing branch.
    Because the success path stays nested inside the choice handler's
    extent, a failure raised anywhere later in that branch lands here
    and the choice advances. Assignments intercept [Fail] the same way
    to undo themselves before propagating -- the edition's undo trail;
    [permanent-set!] installs no interceptor and survives.

    The driver keeps the search state across answers: a top-level
    success performs [Answer] at the sink, and the driver suspends the
    search by stashing the continuation of that Effect.perform -- a one-shot
    host continuation, resumed exactly once by [try_again], which is why
    one capture yields exactly one further answer. Resuming the stashed
    search answers the Effect.perform with [()], so [report] returns and the
    answer's production path unwinds into the choice handlers -- the
    book's driver loop, re-entering the failure branch to find the next
    value.

    Object-language errors are not dead ends (an unbound variable is a
    program bug, not a choice to reconsider): they travel as the typed
    [Raised] exception to the driver, aborting the search. *)

let ( >>= ) = Result.bind

module SE = Sec_4_1
module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Env = Sicp_common.Env
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value

type eval_t = SE.eval_t

(** [true_ v] holds for every value except the false object. *)
let true_ = SE.true_

(** [false_ v] holds exactly for the false object. *)
let false_ = SE.false_

let datum_to_value = SE.datum_to_value

(* {2 4.3.3: the search engine}

   The effects of the section's evaluation: failures unwind, choices
   deliver their alternatives to the handler, and top-level answers
   reach the driver's sink. *)

(** One execution procedure: the environment and the success
    continuation; a failure is performed, not called. *)
type exec = Value.env -> (Value.t -> unit) -> unit

(** One continuation-passing evaluation step, the shape every clause
    recurses through. *)
type eval_k = Ast.expr -> Value.env -> (Value.t -> unit) -> unit

(** [Fail] is a dead end: the branch aborts and unwinds to the innermost
    handler installed for it -- an assignment trail interceptor on the
    way, then the most recent choice point, or the driver when no choice
    is pending. *)
type _ Effect.t += Fail : unit Effect.t

(** [Amb alternatives] is one choice point: the [amb] form performs it
    carrying the execution procedures of its alternatives, and the
    [choice] handler runs them in order, advancing whenever the current
    one fails. *)
type _ Effect.t += Amb : exec list -> unit Effect.t

(** [Answer v] is the driver sink: a top-level success performs it, and
    the driver suspends the search on the continuation of that perform
    so [try_again] can demand the next answer. *)
type _ Effect.t += Answer : Value.t -> unit Effect.t

(** The typed object-language error across the continuation-passing
    steps; [drive] reports it as the [Eval_error] the rest of the
    substrate answers with. *)
exception Raised of Eval_error.t

let raise_error e = raise (Raised e)

(** The suspended search of the driver, if an answer is being displayed
    and more alternatives may exist: the continuation of the [Answer]
    perform, which yields the next answer when resumed under the
    driver's handler. *)
let pending : (unit, (Value.t, Eval_error.t) result) continuation option ref = ref None

(** [backtracks] counts deliveries of [Fail] to choice-point handlers:
    every time the search re-enters a made choice to try its next
    alternative (exercise 4.44a). *)
let backtracks = ref 0

(** [backtrack_count ()] is the number of backtracks since the last
    [reset_backtrack_count ()]. *)
let backtrack_count () = !backtracks

(** [reset_backtrack_count ()] zeroes the backtrack counter. *)
let reset_backtrack_count () = backtracks := 0

(** [choice alternatives] is the execution procedure of one choice
    point, the handler of the [Amb] effect. It runs the alternatives in
    order under the success continuation in hand; a [Fail] performed
    anywhere inside a tried alternative -- including the whole future
    use of its value, which stays nested in the call -- is intercepted
    here: the failed branch's continuation is dropped, one backtrack is
    counted (4.44a), and the next alternative runs. An exhausted list
    performs [Fail] to the enclosing branch. *)
let choice (alternatives : exec list) : exec =
  fun env succeed ->
  let rec try_next = function
    | [] -> Effect.perform Fail
    | first :: rest ->
      Effect.Deep.match_with
        (fun env -> first env succeed)
        env
        { retc = (fun () -> try_next rest)
        ; exnc = raise
        ; effc =
            (fun (type a) (eff : a Effect.t) ->
              match eff with
              | Fail ->
                Some
                  (fun (_dead : (a, _) continuation) ->
                    incr backtracks;
                    try_next rest)
              | _ -> None)
        }
  in
  try_next alternatives
;;

(** [report v] is the driver's success continuation: it performs
    [Answer v] at the sink. When the driver resumes a suspended search
    it answers the Effect.perform with [()], so [report] returns and the
    answer's production path unwinds into the choice handlers. *)
let report (v : Value.t) : unit = Effect.perform (Answer v)

(** {2 4.1.3: environment helpers} *)

let lookup_variable_value name env =
  match Env.find_binding env name with
  | Some v -> Ok v
  | None -> Error (Eval_error.Unbound_variable name)
;;

let extend_environment names values base_env = Env.extend names values base_env
let set_variable_value_ name value env = Env.set env name value

let define_variable_ name value env =
  Env.define env name value;
  Ok (Value.symbol "ok")
;;

(* The error side of the environment operations inside the
   continuation-passing steps: an error aborts the search. *)
let set_binding env name value =
  match Env.set env name value with
  | Ok () -> ()
  | Error e -> raise_error e
;;

let lookup_binding env name =
  match Env.find_binding env name with
  | Some v -> v
  | None -> raise_error (Eval_error.Unbound_variable name)
;;

(** {2 4.1.4: the primitive table of the section}

    The 4.1 table with the predicates and list operations the section's
    nondeterministic programs use: [abs], [min], and [max] over exact
    integers; [even?] and [odd?]; [member] with its [equal?] comparison,
    answering the suffix from the match; [append]; [>=] and [<=]; and
    [sqrt] with [integer?], where a perfect square answers an exact
    integer so Ben's generator of exercise 4.37 can test it. *)

let arity1 name f =
  ( name
  , fun args ->
      match args with
      | [ v ] -> f v
      | args ->
        Error (Eval_error.Arity_mismatch { expected = 1; given = List.length args }) )
;;

let arity2 name f =
  ( name
  , fun args ->
      match args with
      | [ a; b ] -> f a b
      | args ->
        Error (Eval_error.Arity_mismatch { expected = 2; given = List.length args }) )
;;

let rec to_list (v : Value.t) =
  match Value.view v with
  | Value.Nil -> Ok []
  | Value.Pair (car, cdr) -> to_list cdr >>= fun rest -> Ok (car :: rest)
  | _ -> Error (Eval_error.Type_error ("not a list: " ^ Value.to_string v))
;;

let list_values values = List.fold_right Value.pair values Value.nil

let int1 name f =
  arity1 name (fun v ->
    match Value.view v with
    | Value.Int n -> Ok (Value.int (f n))
    | _ -> Error (Eval_error.Type_error (name ^ ": the operand is not an integer")))
;;

let int_cmp name f =
  arity2 name (fun a b ->
    match Value.view a, Value.view b with
    | Value.Int a, Value.Int b -> Ok (Value.bool (f a b))
    | _ -> Error (Eval_error.Type_error (name ^ ": the operands are not both integers")))
;;

let bool1 name f =
  arity1 name (fun v ->
    match Value.view v with
    | Value.Int n -> Ok (Value.bool (f n))
    | _ -> Error (Eval_error.Type_error (name ^ ": the operand is not an integer")))
;;

let all_ints name args =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | v :: rest ->
      (match Value.view v with
       | Value.Int n -> go (n :: acc) rest
       | _ -> Error (Eval_error.Type_error (name ^ ": the operands are not all integers")))
  in
  go [] args
;;

let int_extreme name f =
  ( name
  , fun args ->
      all_ints name args
      >>= function
      | [] -> Error (Eval_error.Arity_mismatch { expected = 1; given = 0 })
      | first :: rest -> Ok (Value.int (List.fold_left f first rest)) )
;;

let member_prim =
  arity2 "member" (fun x l2 ->
    to_list l2
    >>= fun items ->
    let rec go = function
      | [] -> Ok (Value.bool false)
      | v :: suffix when Value.structural_equal x v -> Ok (list_values suffix)
      | _ :: rest -> go rest
    in
    go items)
;;

let append_prim =
  arity2 "append" (fun a b ->
    to_list a >>= fun front -> to_list b >>= fun back -> Ok (list_values (front @ back)))
;;

let sqrt_prim =
  arity1 "sqrt" (fun v ->
    match Value.view v with
    | Value.Int n when n >= 0 ->
      let root = int_of_float (sqrt (float_of_int n)) in
      let root = if root * root < n then root + 1 else root in
      if root * root = n
      then Ok (Value.int root)
      else Ok (Value.float (sqrt (float_of_int n)))
    | Value.Int _ -> Error (Eval_error.Type_error "sqrt: the operand is negative")
    | _ -> Error (Eval_error.Type_error "sqrt: the operand is not an integer"))
;;

let integer_prim =
  arity1 "integer?" (fun v ->
    match Value.view v with
    | Value.Int _ -> Ok (Value.bool true)
    | Value.Float f -> Ok (Value.bool (Float.is_integer f))
    | _ -> Ok (Value.bool false))
;;

let primitive_table : (string * Value.primitive) list =
  [ int_cmp ">=" ( >= )
  ; int_cmp "<=" ( <= )
  ; int1 "abs" abs
  ; int_extreme "min" min
  ; int_extreme "max" max
  ; bool1 "even?" (fun n -> n mod 2 = 0)
  ; bool1 "odd?" (fun n -> n mod 2 <> 0)
  ; member_prim
  ; append_prim
  ; sqrt_prim
  ; integer_prim
  ]
  @ SE.primitive_table
;;

(** [setup_environment ()] is a fresh global environment with the
    section's primitive table and the bindings of [true] and [false]. *)
let setup_environment () =
  let env = Env.empty () in
  Env.define env "true" (Value.bool true);
  Env.define env "false" (Value.bool false);
  List.iter
    (fun (name, f) -> Env.define env name (Value.primitive ~name f))
    primitive_table;
  env
;;

(** [the_global_environment ()] is a fresh global environment, the
    book's [the-global-environment] of this section. *)
let the_global_environment = setup_environment

(** [cond_to_if exp] is the derived-expression rewrite of one [cond]. *)
let cond_to_if = SE.cond_to_if

(** {2 4.1.1-derived: the clauses of the section's evaluator} *)

module Core (Eval : sig
    val eval_k : eval_k
  end) =
struct
  (** [list_of_values exps env k] evaluates the operands left to right
      (exercise 4.46: the parsing programs consume [*unparsed*] in
      order, so the order is load-bearing and fixed by construction). *)
  let rec list_of_values exps env k =
    match exps with
    | [] -> k []
    | exp :: rest ->
      Eval.eval_k exp env (fun v -> list_of_values rest env (fun vs -> k (v :: vs)))
  ;;

  (** [eval_sequence exps env k] evaluates a body or [begin] in order
      under the continuation [k]. *)
  let rec eval_sequence exps env succeed =
    match exps with
    | [] -> raise_error (Eval_error.Invalid_form "the body of the sequence is empty")
    | [ exp ] -> Eval.eval_k exp env succeed
    | exp :: rest -> Eval.eval_k exp env (fun _ -> eval_sequence rest env succeed)
  ;;

  (** [apply_procedure proc args k] is the book's [apply]. *)
  let apply_procedure proc args succeed =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name primitive_table with
       | Some f ->
         (match f args with
          | Ok v -> succeed v
          | Error e -> raise_error e)
       | None ->
         raise_error
           (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure cv ->
      (match Env.extend cv.parameters args cv.env with
       | Ok extended -> eval_sequence cv.body extended succeed
       | Error e -> raise_error e)
    | _ -> raise_error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  (** [eval_if exp env k] evaluates one [if] under the object
      language's truth. *)
  let eval_if exp env succeed =
    match Ast.view exp with
    | Ast.If (predicate, consequent, alternative) ->
      Eval.eval_k predicate env (fun tested ->
        if true_ tested
        then Eval.eval_k consequent env succeed
        else (
          match alternative with
          | Some branch -> Eval.eval_k branch env succeed
          | None -> succeed (Value.bool false)))
    | _ -> raise_error (Eval_error.Invalid_form "eval_if: not an if")
  ;;

  (** [eval_let bindings body env k] is [let] as a simultaneous
      binding: the values are evaluated in the outer environment, the
      body runs in the extended frame (the footnote's assumption that
      the evaluator supports [let], made good here). *)
  let eval_let bindings body env succeed =
    let names = List.map fst bindings in
    let rec collect acc = function
      | [] ->
        (match Env.extend names (List.rev acc) env with
         | Ok extended -> eval_sequence body extended succeed
         | Error e -> raise_error e)
      | (_, exp) :: rest -> Eval.eval_k exp env (fun v -> collect (v :: acc) rest)
    in
    collect [] bindings
  ;;

  (** [eval_assignment name exp env k] is the book's [analyze-assignment]
      with the undo trail: the old value is saved, the assignment is
      made, and the success that follows runs inside an interceptor --
      a failure later in the branch restores the old value before
      propagating. *)
  let eval_assignment name exp env succeed =
    Eval.eval_k exp env (fun value ->
      let old = lookup_binding env name in
      set_binding env name value;
      try succeed (Value.symbol "ok") with
      | effect Fail, _dead ->
        set_binding env name old;
        Effect.perform Fail)
  ;;

  (** [eval_permanent name exp env k] is exercise 4.51's
      [permanent-set!]: the assignment is made and never undone. *)
  let eval_permanent name exp env succeed =
    Eval.eval_k exp env (fun value ->
      set_binding env name value;
      succeed (Value.symbol "ok"))
  ;;

  (** [eval_if_fail e1 e2 env k] is exercise 4.52's [if-fail]: a
      failure of [e1] is caught once -- the dead branch is dropped and
      [e2] succeeds in its place; failures after the catch, including
      those of the branch that follows [e2]'s value, propagate. *)
  let eval_if_fail e1 e2 env succeed =
    let caught = ref false in
    try Eval.eval_k e1 env succeed with
    | effect Fail, _dead when not !caught ->
      caught := true;
      Eval.eval_k e2 env succeed
  ;;

  (** [eval_definition d env k] installs one definition and answers
      [ok]. Definitions are not undone on backtracking, as the
      footnote assumes scanned-out internal definitions. *)
  let eval_definition d env succeed =
    match Ast.view_definition d with
    | Ast.Define_variable (name, exp) ->
      Eval.eval_k exp env (fun value ->
        Env.define env name value;
        succeed (Value.symbol "ok"))
    | Ast.Define_function { name; parameters; body } ->
      Env.define env name (Value.compound ~name:(Some name) ~parameters ~body ~env);
      succeed (Value.symbol "ok")
  ;;

  (** The dispatch of the section: the 4.1 clauses in continuation
      style, plus the [amb], [permanent-set!], and [if-fail] special
      forms, recognized at application heads by their reserved names --
      the same tagged-list recognition the book uses, because the
      shared grammar gains no node kinds. [let] is a derived form. *)
  let rec eval_k exp env succeed =
    match Ast.view exp with
    | Ast.Int n -> succeed (Value.int n)
    | Ast.Float f -> succeed (Value.float f)
    | Ast.Bool b -> succeed (Value.bool b)
    | Ast.String s -> succeed (Value.string s)
    | Ast.Variable name ->
      (match lookup_variable_value name env with
       | Ok v -> succeed v
       | Error e -> raise_error e)
    | Ast.Quote datum -> succeed (datum_to_value datum)
    | Ast.Definition d -> eval_definition d env succeed
    | Ast.Set (name, exp) -> eval_assignment name exp env succeed
    | Ast.If _ -> eval_if exp env succeed
    | Ast.Lambda (parameters, body) ->
      succeed (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> eval_sequence body env succeed
    | Ast.Cond _ ->
      (match SE.cond_to_if exp with
       | Ok rewritten -> eval_k rewritten env succeed
       | Error e -> raise_error e)
    | Ast.Let (bindings, body) -> eval_let bindings body env succeed
    | Ast.Application (operator, operands) -> application operator operands env succeed
    | Ast.And _ | Ast.Or _ ->
      raise_error (Eval_error.Invalid_form "unknown expression type: EVAL")

  and application operator operands env succeed =
    match Ast.view operator with
    | Ast.Variable "amb" ->
      let alternatives =
        List.map (fun e env2 succ2 -> Eval.eval_k e env2 succ2) operands
      in
      choice alternatives env succeed
    | Ast.Variable "permanent-set!" ->
      (match operands with
       | [ name_exp; exp ] ->
         (match Ast.view name_exp with
          | Ast.Variable name -> eval_permanent name exp env succeed
          | _ ->
            raise_error
              (Eval_error.Invalid_form
                 "permanent-set!: expects (permanent-set! name expression)"))
       | _ ->
         raise_error
           (Eval_error.Invalid_form
              "permanent-set!: expects (permanent-set! name expression)"))
    | Ast.Variable "if-fail" ->
      (match operands with
       | [ e1; e2 ] -> eval_if_fail e1 e2 env succeed
       | _ -> raise_error (Eval_error.Invalid_form "if-fail: expects two expressions"))
    | _ ->
      Eval.eval_k operator env (fun proc ->
        list_of_values operands env (fun args -> apply_procedure proc args succeed))
  ;;
end

module rec Base : sig
  val eval_k : eval_k
end = struct
  module C = Core (Base)

  let eval_k = C.eval_k
end

(** {2 4.3.3: the driver} *)

let read_error e = Eval_error.Invalid_form (Reader.to_string e)
let no_more_values = Eval_error.User_error "there are no more values"

(** The driver's handler: the [Answer] suspend stashes the search on the
    continuation of the perform and reports the value; a [Fail] that
    reaches the driver means the choices are exhausted; the typed
    [Raised] error aborts the search. The handler is deep, and that is
    what carries the protocol: when [try_again] resumes the stashed
    search with [Effect.Deep.continue], the next [Answer] re-enters this
    very handler, which stashes the new continuation and answers the
    resumption with the next value -- one handler instance drives every
    answer of the problem. *)
let driver_handler : (unit, (Value.t, Eval_error.t) result) Effect.Deep.handler =
  { retc =
      (fun () ->
        pending := None;
        Error no_more_values)
  ; exnc =
      (fun e ->
        match e with
        | Raised error ->
          pending := None;
          Error error
        | other -> raise other)
  ; effc =
      (fun (type a) (eff : a Effect.t) ->
        match eff with
        | Answer v ->
          Some
            (fun (k : (a, (Value.t, Eval_error.t) result) continuation) ->
              pending := Some k;
              Ok v)
        | Fail ->
          Some
            (fun (_dead : (a, (Value.t, Eval_error.t) result) continuation) ->
              pending := None;
              Error no_more_values)
        | _ -> None)
  }
;;

(** [drive go] runs one search to its first answer or its exhaustion. *)
let drive (go : unit -> unit) : (Value.t, Eval_error.t) result =
  Effect.Deep.match_with (fun () -> go ()) () driver_handler
;;

(** [eval exp env] evaluates one expression as one problem: the value
    of its first non-failing execution, or the typed exhaustion error
    when every execution fails. *)
let eval exp env = drive (fun () -> Base.eval_k exp env report)

(** [run env text] reads one object-language form from [text] and
    evaluates it as a new problem; any pending search is discarded. *)
let run env text =
  pending := None;
  Reader.read text |> Result.map_error read_error >>= fun exp -> eval exp env
;;

(** [run_program env text] reads a program of forms and evaluates them
    in order, each as its own problem; the value of the last form is
    the program's value and its search state is what [try_again]
    continues. A form that suspends after its first answer has its
    remaining alternatives discarded when the next form starts. *)
let run_program env text =
  pending := None;
  Reader.read_program text
  |> Result.map_error read_error
  >>= fun exps ->
  let rec go = function
    | [] -> Ok (Value.symbol "ok")
    | [ exp ] -> eval exp env
    | exp :: rest -> eval exp env >>= fun _ -> go rest
  in
  go exps
;;

(** [try_again ()] is the driver's try-again request: it resumes the
    suspended search once, answering the next value, or reports
    exhaustion. With no search in progress it reports the book's
    ``there is no current problem''. *)
let try_again () =
  match !pending with
  | None -> Error (Eval_error.User_error "there is no current problem")
  | Some k ->
    pending := None;
    (* Resuming re-enters the driver's handler: its answer to the next
       [Answer] perform is exactly this call's result. *)
    Effect.Deep.continue k ()
;;
