(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** The explicit-control evaluator of section 5.4: the book's register
    machine that runs the metacircular evaluator's algorithm directly.

    The machine words are the edition's answer to the book's uniform
    list structure: a [V] word wraps a shared [Value.t] (numbers,
    symbols, pairs, strings, procedure values), and the evaluator's own
    registers carry words besides values -- [Exp]/[Seq] hold the typed
    expressions the shared [Reader] produced, [Args] an accumulated
    operand list, [Env] an environment, [Lab] a return address.  Two
    words exist for this section's exercises and never for the base
    controller: [Clause] is 5.24's cond clause, [Thunk] is 5.25's
    delayed operand.  The controller is the book's text in the book's
    notation, parsed by the 5.1 reader into the 5.1 instruction
    language and executed over the word registers; the operations table
    holds the syntax and environment procedures of 4.1 typed over
    words.  Nothing raises; every failure is a typed [error]. *)

module Ast = Sicp_common.Ast
module Value = Sicp_common.Value
module Env = Sicp_common.Env
module Reader = Sicp_common.Reader

let ( >>= ) = Result.bind

(** {1:machine-words Machine words} *)

(** One machine word: what a register or a stack entry holds. *)
type word =
  | V of Value.t (** an object-language value. *)
  | Exp of Ast.expr (** one expression, as [exp] holds it. *)
  | Seq of Ast.expr list
  (** a sequence of expressions, as [unev] holds operand lists and
          bodies. *)
  | Args of word list
  (** an operand list of already-evaluated words, as [argl] holds
          it; [adjoin-arg] appends at the end, the book's order. *)
  | Env of Value.env (** an environment, as [env] holds it. *)
  | Lab of string (** a return address, as [continue] holds it. *)
  | Clause of Ast.expr option * Ast.expr list
  (** one cond clause: [Some test] with its body, or [None] for the
          trailing else body -- 5.24's clause words. *)
  | Thunk of Ast.expr * Value.env
  (** an operand delayed with its environment, 5.25's lazy
          operand; the base evaluator never builds one. *)

let word_to_string = function
  | V v -> Value.to_string v
  | Exp _ -> "expression"
  | Seq _ -> "sequence"
  | Args _ -> "argument list"
  | Env _ -> "environment"
  | Lab l -> l
  | Clause _ -> "clause"
  | Thunk _ -> "thunk"
;;

(** Every failure of the substrate travels through [error]; the type is
    the 5.1 substrate's, re-exported. *)
type error = Sec_5_1.error =
  | Parse of string
  | Unknown_register of string
  | Unknown_operation of string
  | Unknown_label of string
  | Bad_instruction of string
  | Arity of string
  | Op_failed of string
  | Stack_underflow of string
  | Branch_without_test

let error_to_string = Sec_5_1.error_to_string

(** The message whose [Op_failed] ends the driver loop when the input
    queue runs dry: the edition's stop for the book's unbounded
    read-eval-print loop. *)
let input_exhausted = "the evaluator's input queue is empty"

(** {1:operations The operations table} *)

(** One operation of the evaluator machine: a [Value_op] computes a
    word for an [assign] or a [test]; an [Action_op] is an action under
    [perform]. *)
type op =
  | Value_op of (word list -> (word, error) result)
  | Action_op of (word list -> (unit, error) result)

(** {1:machine The machine} *)

type machine =
  { regs : (string, word ref) Hashtbl.t
  ; operations : (string, op) Hashtbl.t
  ; program : Sec_5_1.program
  ; pc : int ref
  ; the_stack : word list ref
  ; pushes : int ref
  ; depth : int ref
  ; max_depth : int ref
  ; output : string list ref
  ; input_queue : Ast.expr list ref
  }

(** The registers of the evaluator machine description: the book's
    seven plus [flag], which every [test] sets. *)
let evaluator_registers =
  [ "exp"; "env"; "val"; "continue"; "proc"; "argl"; "unev"; "flag" ]
;;

let lookup m r =
  match Hashtbl.find_opt m.regs r with
  | Some cell -> Ok !cell
  | None -> Error (Unknown_register r)
;;

let store m r w =
  match Hashtbl.find_opt m.regs r with
  | Some cell ->
    cell := w;
    Ok ()
  | None -> Error (Unknown_register r)
;;

let set_register m r w = store m r w
let get_register m r = lookup m r

let inputs m (sources : Sec_5_1.source list) =
  let one = function
    | Sec_5_1.Reg r -> lookup m r
    | Sec_5_1.Const v ->
      let value = function
        | Sec_5_1.Int n -> Value.int n
        | Sec_5_1.Float f -> Value.float f
        | Sec_5_1.Bool b -> Value.bool b
        | Sec_5_1.Symbol s -> Value.symbol s
        | Sec_5_1.Label l -> Value.symbol l
      in
      Ok (V (value v))
    | Sec_5_1.Label_source l -> Ok (Lab l)
  in
  let rec all = function
    | [] -> Ok []
    | s :: rest -> one s >>= fun w -> all rest >>= fun ws -> Ok (w :: ws)
  in
  all sources
;;

let apply_value m name args =
  match Hashtbl.find_opt m.operations name with
  | Some (Value_op f) -> f args
  | Some (Action_op _) ->
    Error
      (Bad_instruction ("the operation " ^ name ^ " is an action and produces no value"))
  | None -> Error (Unknown_operation name)
;;

let apply_action m name args =
  match Hashtbl.find_opt m.operations name with
  | Some (Action_op f) -> f args
  | Some (Value_op _) ->
    Error
      (Bad_instruction
         ("the operation " ^ name ^ " produces a value; assign it, do not perform it"))
  | None -> Error (Unknown_operation name)
;;

(** {1:stack The monitored stack of 5.2.4} *)

let push_stack m w =
  m.the_stack := w :: !(m.the_stack);
  incr m.pushes;
  incr m.depth;
  if !(m.depth) > !(m.max_depth) then m.max_depth := !(m.depth);
  Ok ()
;;

let pop_stack m r =
  match !(m.the_stack) with
  | [] -> Error (Stack_underflow r)
  | w :: rest ->
    m.the_stack := rest;
    decr m.depth;
    store m r w
;;

let initialize_stack m =
  m.the_stack := [];
  m.pushes := 0;
  m.depth := 0;
  m.max_depth := 0;
  Ok ()
;;

let print_stack_statistics m =
  Printf.sprintf "(total-pushes = %d maximum-depth = %d)" !(m.pushes) !(m.max_depth)
;;

(** {1:execution The execution cycle} *)

let jump_to m l =
  match List.assoc_opt l m.program.labels with
  | Some i ->
    m.pc := i;
    Ok ()
  | None -> Error (Unknown_label l)
;;

(** [is_true w] is the book's [true?]: every value counts as true
    except [false]. *)
let is_true = function
  | V v ->
    (match Value.view v with
     | Value.Bool false -> false
     | _ -> true)
  | _ -> false
;;

let step m (inst : Sec_5_1.instruction) =
  match inst with
  | Assign (r, src) ->
    (match src with
     | Sec_5_1.Reg r' when r' = "flag" ->
       Error (Bad_instruction "the flag register is not assignable")
     | _ -> inputs m [ src ])
    >>= (function
     | [ w ] -> store m r w
     | _ -> Error (Bad_instruction "an assign takes one source"))
  | Assign_op (r, name, srcs) -> inputs m srcs >>= apply_value m name >>= store m r
  | Test (name, srcs) ->
    inputs m srcs
    >>= apply_value m name
    >>= fun w -> store m "flag" (V (Value.bool (is_true w)))
  | Branch l -> lookup m "flag" >>= fun w -> if is_true w then jump_to m l else Ok ()
  | Goto_label l -> jump_to m l
  | Goto_reg r ->
    lookup m r
    >>= (function
     | Lab l -> jump_to m l
     | w -> Error (Op_failed ("the goto register " ^ r ^ " holds " ^ word_to_string w)))
  | Save r -> lookup m r >>= push_stack m
  | Restore r -> pop_stack m r
  | Perform (name, srcs) -> inputs m srcs >>= apply_action m name
;;

let start m =
  let size = Array.length m.program.code in
  let rec loop () =
    let pc = !(m.pc) in
    if pc >= size
    then Ok ()
    else (
      m.pc := pc + 1;
      match step m m.program.code.(pc) with
      | Ok () -> loop ()
      | Error e -> Error e)
  in
  loop ()
;;

(** {1:transcript The driver's transcript} *)

let say m line =
  m.output := !(m.output) @ [ line ];
  Ok ()
;;

let transcript m = !(m.output)

(** {1:object-data From expressions to values} *)

(** [datum_value d] is the quoted datum [d] as a value. *)
let rec datum_value (d : Ast.datum) =
  match d with
  | Ast.DInt n -> Value.int n
  | Ast.DFloat f -> Value.float f
  | Ast.DBool b -> Value.bool b
  | Ast.DString s -> Value.string s
  | Ast.DSymbol s -> Value.symbol s
  | Ast.DNil -> Value.nil
  | Ast.DPair (a, b) -> Value.pair (datum_value a) (datum_value b)
;;

let variable_name e =
  match Ast.view e with
  | Ast.Variable n -> Ok n
  | _ -> Error (Op_failed "a parameter is written as a variable")
;;

(** [word_value w] is the object value a word must carry when a
    primitive consumes it. *)
let word_value = function
  | V v -> Ok v
  | w -> Error (Op_failed ("expected a value, found " ^ word_to_string w))
;;

let rec result_all = function
  | [] -> Ok []
  | r :: rest ->
    (match r with
     | Ok v -> result_all rest >>= fun vs -> Ok (v :: vs)
     | Error e -> Error e)
;;

let word_values ws = result_all (List.map word_value ws)
let eval_error e = Op_failed (Sicp_common.Eval_error.to_string e)

(** [expr_word r] is the word of the expression result [r]: the smart
    constructors' failures are rendered into the substrate's error
    channel, the expression wrapped as an [Exp] word. *)
let expr_word r = Result.map (fun e -> Exp e) (Result.map_error eval_error r)

(** {1:primitives The object-language primitives} *)

let number_of name v =
  match Value.view v with
  | Value.Int n -> Ok n
  | Value.Float f -> Ok (int_of_float f)
  | _ ->
    Error
      (Sicp_common.Eval_error.Type_error
         (name ^ " needs a number, got " ^ Value.to_string v))
;;

let arity_error expected given = Sicp_common.Eval_error.Arity_mismatch { expected; given }

let prim1 f = function
  | [ a ] -> f a
  | vs -> Error (arity_error 1 (List.length vs))
;;

let prim2 f = function
  | [ a; b ] -> f a b
  | vs -> Error (arity_error 2 (List.length vs))
;;

let prim_list f = f

(** [arith name f] is the n-ary object-language arithmetic: [(f a b)]
    folded left over the numbers, [(− a)] the negation, and a division
    by zero the object-language's division failure. *)
let arith name f =
  let step acc v =
    if name = "/" && v = 0
    then Error Sicp_common.Eval_error.Division_by_zero
    else Ok (f acc v)
  in
  fun vs ->
    let rec go acc = function
      | [] -> Ok (Value.int acc)
      | v :: rest -> number_of name v >>= step acc >>= fun acc' -> go acc' rest
    in
    match vs with
    | [] -> Error (arity_error 2 0)
    | [ single ] ->
      if name = "-"
      then number_of name single >>= fun n -> Ok (Value.int (-n))
      else number_of name single >>= fun n -> Ok (Value.int n)
    | first :: rest -> number_of name first >>= fun n0 -> go n0 rest
;;

let comparison name test =
  prim2 (fun a b ->
    number_of name a >>= fun x -> number_of name b >>= fun y -> Ok (Value.bool (test x y)))
;;

(** The primitive procedures of the global environment, the book's
    eceval list trimmed to the subset the section's sessions use. *)
let object_primitives : (string * Value.primitive) list =
  [ "cons", prim2 (fun a b -> Ok (Value.pair a b))
  ; ( "car"
    , prim1 (fun v ->
        match Value.view v with
        | Value.Pair (a, _) -> Ok a
        | _ -> Error (Sicp_common.Eval_error.Type_error ("car of " ^ Value.to_string v)))
    )
  ; ( "cdr"
    , prim1 (fun v ->
        match Value.view v with
        | Value.Pair (_, d) -> Ok d
        | _ -> Error (Sicp_common.Eval_error.Type_error ("cdr of " ^ Value.to_string v)))
    )
  ; ( "null?"
    , prim1 (fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Nil -> true
              | _ -> false))) )
  ; ( "pair?"
    , prim1 (fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Pair _ -> true
              | _ -> false))) )
  ; ( "symbol?"
    , prim1 (fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Symbol _ -> true
              | _ -> false))) )
  ; ( "number?"
    , prim1 (fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Int _ | Value.Float _ -> true
              | _ -> false))) )
  ; ( "string?"
    , prim1 (fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.String _ -> true
              | _ -> false))) )
  ; ( "not"
    , prim1 (fun v ->
        Ok
          (Value.bool
             (match Value.view v with
              | Value.Bool false -> true
              | _ -> false))) )
  ; "eq?", prim2 (fun a b -> Ok (Value.bool (Value.physical_equal a b)))
  ; "equal?", prim2 (fun a b -> Ok (Value.bool (Value.structural_equal a b)))
  ; "list", prim_list (fun vs -> Ok (List.fold_right Value.pair vs Value.nil))
  ; "+", arith "+" ( + )
  ; "-", arith "-" ( - )
  ; "*", arith "*" ( * )
  ; "/", arith "/" ( / )
  ; "=", comparison "=" ( = )
  ; "<", comparison "<" ( < )
  ; ">", comparison ">" ( > )
  ; ( "remainder"
    , prim2 (fun a b ->
        number_of "remainder" a
        >>= fun x ->
        number_of "remainder" b
        >>= fun y ->
        if y = 0
        then Error Sicp_common.Eval_error.Division_by_zero
        else Ok (Value.int (x mod y))) )
  ]
;;

(** [apply_object_primitive name args] applies the object-language
    primitive [name] to the values [args]: the evaluator's
    [apply-primitive-procedure] calls it with [proc]'s name, and the
    error-signaling exercise wraps its failures in condition-code
    words.  An unknown name is the typed unknown-operation failure. *)
let apply_object_primitive name args =
  match List.assoc_opt name object_primitives with
  | Some f ->
    (match f args with
     | Ok v -> Ok v
     | Error e -> Error (eval_error e))
  | None -> Error (Unknown_operation name)
;;

(** {1:syntax The syntax and environment operations of 4.1, typed over words} *)

let shape_of name w =
  match w with
  | Exp e -> Ok e
  | _ -> Error (Op_failed (name ^ " needs an expression"))
;;

(** [variable_of name w] is the variable's own name when the word [w]
    is the expression of a variable reference. *)
let variable_of name w =
  match shape_of name w with
  | Error e -> Error e
  | Ok e ->
    (match Ast.view e with
     | Ast.Variable n -> Ok n
     | _ -> Error (Op_failed (name ^ " needs a variable")))
;;

let is_kind kind w =
  match w with
  | Exp e ->
    let matches =
      match Ast.view e, kind with
      | (Ast.Int _ | Ast.Float _ | Ast.Bool _ | Ast.String _), `self_evaluating -> true
      | Ast.Variable _, `variable -> true
      | Ast.Quote _, `quoted -> true
      | Ast.Set _, `assignment -> true
      | Ast.Definition _, `definition -> true
      | Ast.If _, `if_form -> true
      | Ast.Lambda _, `lambda_form -> true
      | Ast.Sequence _, `begin_form -> true
      | Ast.Application _, `application -> true
      | Ast.Cond _, `cond_form -> true
      | Ast.And _, `and_form -> true
      | Ast.Or _, `or_form -> true
      | Ast.Let _, `let_form -> true
      | _ -> false
    in
    Ok (V (Value.bool matches))
  | _ -> Error (Op_failed "the register holds no expression")
;;

let test_op name kind =
  ( name
  , Value_op
      (function
        | [ w ] -> is_kind kind w
        | _ -> Error (Arity (name ^ " needs one argument"))) )
;;

let one_word name f =
  ( name
  , Value_op
      (function
        | [ w ] -> f w
        | _ -> Error (Arity (name ^ " needs one argument"))) )
;;

let two_words name f =
  ( name
  , Value_op
      (function
        | [ a; b ] -> f a b
        | _ -> Error (Arity (name ^ " needs two arguments"))) )
;;

let three_words name f =
  ( name
  , Value_op
      (function
        | [ a; b; c ] -> f a b c
        | _ -> Error (Arity (name ^ " needs three arguments"))) )
;;

let selector name f = one_word name (fun w -> shape_of name w >>= f)
let exp_word e = Ok (Exp e)
let seq_word xs = Ok (Seq xs)
let env_word e = Ok (Env e)

let quoted_datum e =
  match Ast.view e with
  | Ast.Quote d -> Ok d
  | _ -> Error (Op_failed "not a quotation")
;;

let if_part part e =
  match Ast.view e with
  | Ast.If (c, t, a) ->
    (match part with
     | `predicate -> Ok c
     | `consequent -> Ok t
     | `alternative ->
       (match a with
        | Some a -> Ok a
        | None -> Ok (Ast.bool false)))
  | _ -> Error (Op_failed "not an if")
;;

let begin_actions e =
  match Ast.view e with
  | Ast.Sequence body -> seq_word body
  | _ -> Error (Op_failed "not a begin")
;;

let lambda_part part e =
  match Ast.view e with
  | Ast.Lambda (ps, body) ->
    (match part with
     | `parameters -> seq_word (List.map Ast.variable ps)
     | `body -> seq_word body)
  | _ -> Error (Op_failed "not a lambda")
;;

let application_part part e =
  match Ast.view e with
  | Ast.Application (op, ops) ->
    (match part with
     | `operator -> exp_word op
     | `operands -> seq_word ops)
  | _ -> Error (Op_failed "not an application")
;;

let assignment_part part e =
  match Ast.view e with
  | Ast.Set (n, v) ->
    (match part with
     | `variable -> exp_word (Ast.variable n)
     | `value -> exp_word v)
  | _ -> Error (Op_failed "not an assignment")
;;

let definition_part part e =
  match Ast.view e with
  | Ast.Definition d ->
    (match Ast.view_definition d, part with
     | Ast.Define_variable (n, _), `variable -> exp_word (Ast.variable n)
     | Ast.Define_variable (_, v), `value -> exp_word v
     | Ast.Define_function { name; _ }, `variable -> exp_word (Ast.variable name)
     | Ast.Define_function { parameters; body; _ }, `value ->
       (match Ast.lambda parameters body with
        | Ok l -> exp_word l
        | Error e -> Error (eval_error e)))
  | _ -> Error (Op_failed "not a definition")
;;

(** The names of the [Seq] of variable expressions a parameter list
    is. *)
let parameters_of = function
  | Seq xs -> result_all (List.map variable_name xs)
  | _ -> Error (Op_failed "no parameter sequence")
;;

let compound_of name w =
  match w with
  | V v ->
    (match Value.view v with
     | Value.Compound_procedure cv -> Ok cv
     | _ -> Error (Op_failed (name ^ " needs a compound procedure")))
  | _ -> Error (Op_failed (name ^ " needs a compound procedure"))
;;

let lookup_binding env name =
  match Env.find_binding env name with
  | Some v -> Ok (V v)
  | None -> Error (Op_failed ("unbound variable: " ^ name))
;;

let extend_environment params args base =
  match parameters_of params, args, base with
  | Error e, _, _ -> Error e
  | Ok names, Args ws, env ->
    word_values ws
    >>= fun vs ->
    (match Env.extend names vs env with
     | Ok e -> Ok (Env e)
     | Error e -> Error (eval_error e))
  | _, _, _ -> Error (Op_failed "the arguments are not an operand list")
;;

(** {1:controller-fragments The controller text, in the book's fragments} *)

(** The controller fragments of the base evaluator, in printed order:
    each pair is the fragment's name and its controller text.  The base
    controller is their concatenation; an exercise replaces a fragment
    or appends its own and hands the composed text to
    [make_evaluator]. *)
let controller_fragments =
  [ ( "driver"
    , {|read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op announce-output))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))|}
    )
  ; ( "eval-dispatch"
    , {|eval-dispatch
  (test (op self-evaluating?) (reg exp))
  (branch (label ev-self-eval))
  (test (op variable?) (reg exp))
  (branch (label ev-variable))
  (test (op quoted?) (reg exp))
  (branch (label ev-quoted))
  (test (op assignment?) (reg exp))
  (branch (label ev-assignment))
  (test (op definition?) (reg exp))
  (branch (label ev-definition))
  (test (op if?) (reg exp))
  (branch (label ev-if))
  (test (op lambda?) (reg exp))
  (branch (label ev-lambda))
  (test (op begin?) (reg exp))
  (branch (label ev-begin))
  (test (op application?) (reg exp))
  (branch (label ev-application))
  (goto (label unknown-expression-type))|}
    )
  ; ( "ev-self-eval"
    , {|ev-self-eval
  (assign val (op self-evaluating-value) (reg exp))
  (goto (reg continue))|}
    )
  ; ( "ev-variable"
    , {|ev-variable
  (assign val
          (op lookup-variable-value)
          (reg exp)
          (reg env))
  (goto (reg continue))|}
    )
  ; ( "ev-quoted"
    , {|ev-quoted
  (assign val
          (op text-of-quotation)
          (reg exp))
  (goto (reg continue))|}
    )
  ; ( "ev-lambda"
    , {|ev-lambda
  (assign unev
          (op lambda-parameters)
          (reg exp))
  (assign exp
          (op lambda-body)
          (reg exp))
  (assign val
          (op make-procedure)
          (reg unev)
          (reg exp)
          (reg env))
  (goto (reg continue))|}
    )
  ; ( "ev-application"
    , {|ev-application
  (save continue)
  (save env)
  (assign unev (op operands) (reg exp))
  (save unev)
  (assign exp (op operator) (reg exp))
  (assign
   continue (label ev-appl-did-operator))
  (goto (label eval-dispatch))|}
    )
  ; ( "ev-appl-did-operator"
    , {|ev-appl-did-operator
  (restore unev)
  (restore env)
  (assign argl (op empty-arglist))
  (assign proc (reg val))
  (test (op no-operands?) (reg unev))
  (branch (label apply-dispatch))
  (save proc)|}
    )
  ; ( "argument-loop"
    , {|ev-appl-operand-loop
  (save argl)
  (assign exp
          (op first-operand)
          (reg unev))
  (test (op last-operand?) (reg unev))
  (branch (label ev-appl-last-arg))
  (save env)
  (save unev)
  (assign continue
          (label ev-appl-accumulate-arg))
  (goto (label eval-dispatch))
ev-appl-accumulate-arg
  (restore unev)
  (restore env)
  (restore argl)
  (assign argl
          (op adjoin-arg)
          (reg val)
          (reg argl))
  (assign unev
          (op rest-operands)
          (reg unev))
  (goto (label ev-appl-operand-loop))
ev-appl-last-arg
  (assign continue
          (label ev-appl-accum-last-arg))
  (goto (label eval-dispatch))
ev-appl-accum-last-arg
  (restore argl)
  (assign argl
          (op adjoin-arg)
          (reg val)
          (reg argl))
  (restore proc)
  (goto (label apply-dispatch))|}
    )
  ; ( "apply-dispatch"
    , {|apply-dispatch
  (test (op primitive-procedure?) (reg proc))
  (branch (label primitive-apply))
  (test (op compound-procedure?) (reg proc))
  (branch (label compound-apply))
  (goto (label unknown-procedure-type))|}
    )
  ; ( "primitive-apply"
    , {|primitive-apply
  (assign val (op apply-primitive-procedure)
              (reg proc)
              (reg argl))
  (restore continue)
  (goto (reg continue))|}
    )
  ; ( "compound-apply"
    , {|compound-apply
  (assign unev
          (op procedure-parameters)
          (reg proc))
  (assign env
          (op procedure-environment)
          (reg proc))
  (assign env
          (op extend-environment)
          (reg unev)
          (reg argl)
          (reg env))
  (assign unev
          (op procedure-body)
          (reg proc))
  (goto (label ev-sequence))|}
    )
  ; ( "begin"
    , {|ev-begin
  (assign unev
          (op begin-actions)
          (reg exp))
  (save continue)
  (goto (label ev-sequence))|}
    )
  ; ( "ev-sequence"
    , {|ev-sequence
  (assign exp (op first-exp) (reg unev))
  (test (op last-exp?) (reg unev))
  (branch (label ev-sequence-last-exp))
  (save unev)
  (save env)
  (assign continue
          (label ev-sequence-continue))
  (goto (label eval-dispatch))
ev-sequence-continue
  (restore env)
  (restore unev)
  (assign unev
          (op rest-exps)
          (reg unev))
  (goto (label ev-sequence))
ev-sequence-last-exp
  (restore continue)
  (goto (label eval-dispatch))|}
    )
  ; ( "if"
    , {|ev-if
  (save continue)
  (save env)
  (save exp)
  (assign continue (label ev-if-decide))
  (assign exp (op if-predicate) (reg exp))
  (goto (label eval-dispatch))
ev-if-decide
  (restore exp)
  (restore env)
  (restore continue)
  (test (op true?) (reg val))
  (branch (label ev-if-consequent))
ev-if-alternative
  (assign exp (op if-alternative) (reg exp))
  (goto (label eval-dispatch))
ev-if-consequent
  (assign exp (op if-consequent) (reg exp))
  (goto (label eval-dispatch))|}
    )
  ; ( "assignment"
    , {|ev-assignment
  (assign unev (op assignment-variable) (reg exp))
  (save unev)
  (assign exp (op assignment-value) (reg exp))
  (save env)
  (save continue)
  (assign continue (label ev-assignment-1))
  (goto (label eval-dispatch))
ev-assignment-1
  (restore continue)
  (restore env)
  (restore unev)
  (perform
   (op set-variable-value!) (reg unev) (reg val) (reg env))
  (assign val (const ok))
  (goto (reg continue))|}
    )
  ; ( "definition"
    , {|ev-definition
  (assign unev (op definition-variable) (reg exp))
  (save unev)
  (assign exp (op definition-value) (reg exp))
  (save env)
  (save continue)
  (assign continue (label ev-definition-1))
  (goto (label eval-dispatch))
ev-definition-1
  (restore continue)
  (restore env)
  (restore unev)
  (perform
   (op define-variable!) (reg unev) (reg val) (reg env))
  (assign val (const ok))
  (goto (reg continue))|}
    )
  ; ( "errors"
    , {|unknown-expression-type
  (assign val (const unknown-expression-type-error))
  (goto (label signal-error))
unknown-procedure-type
  (restore continue)
  (assign val (const unknown-procedure-type-error))
  (goto (label signal-error))
signal-error
  (perform (op signal-error) (reg val))|}
    )
  ]
;;

let base_controller = List.map snd controller_fragments |> String.concat "\n"

(** {1:base-operations The base operations table} *)

let base_operations =
  [ test_op "self-evaluating?" `self_evaluating
  ; test_op "variable?" `variable
  ; test_op "quoted?" `quoted
  ; test_op "assignment?" `assignment
  ; test_op "definition?" `definition
  ; test_op "if?" `if_form
  ; test_op "lambda?" `lambda_form
  ; test_op "begin?" `begin_form
  ; test_op "application?" `application
  ; one_word "self-evaluating-value" (function
      | Exp e ->
        (match Ast.view e with
         | Ast.Int n -> Ok (V (Value.int n))
         | Ast.Float f -> Ok (V (Value.float f))
         | Ast.Bool b -> Ok (V (Value.bool b))
         | Ast.String s -> Ok (V (Value.string s))
         | _ -> Error (Op_failed "not a self-evaluating expression"))
      | w -> Error (Op_failed ("expected an expression, found " ^ word_to_string w)))
  ; selector "text-of-quotation" (fun e ->
      Result.map (fun d -> V (datum_value d)) (quoted_datum e))
  ; selector "if-predicate" (fun e -> if_part `predicate e >>= exp_word)
  ; selector "if-consequent" (fun e -> if_part `consequent e >>= exp_word)
  ; selector "if-alternative" (fun e -> if_part `alternative e >>= exp_word)
  ; selector "begin-actions" begin_actions
  ; selector "lambda-parameters" (fun e -> lambda_part `parameters e)
  ; selector "lambda-body" (fun e -> lambda_part `body e)
  ; selector "operator" (fun e -> application_part `operator e)
  ; selector "operands" (fun e -> application_part `operands e)
  ; selector "assignment-variable" (fun e -> assignment_part `variable e)
  ; selector "assignment-value" (fun e -> assignment_part `value e)
  ; selector "definition-variable" (fun e -> definition_part `variable e)
  ; selector "definition-value" (fun e -> definition_part `value e)
  ; one_word "first-exp" (function
      | Seq (e :: _) -> exp_word e
      | Seq [] -> Error (Op_failed "first-exp of an empty sequence")
      | w -> Error (Op_failed ("no sequence: " ^ word_to_string w)))
  ; one_word "rest-exps" (function
      | Seq (_ :: rest) -> seq_word rest
      | Seq [] -> Error (Op_failed "rest-exps of an empty sequence")
      | w -> Error (Op_failed ("no sequence: " ^ word_to_string w)))
  ; one_word "last-exp?" (function
      | Seq [ _ ] -> Ok (V (Value.bool true))
      | Seq _ -> Ok (V (Value.bool false))
      | w -> Error (Op_failed ("no sequence: " ^ word_to_string w)))
  ; one_word "no-more-exps?" (function
      | Seq [] -> Ok (V (Value.bool true))
      | Seq _ -> Ok (V (Value.bool false))
      | w -> Error (Op_failed ("no sequence: " ^ word_to_string w)))
  ; one_word "no-operands?" (function
      | Seq [] -> Ok (V (Value.bool true))
      | Seq _ -> Ok (V (Value.bool false))
      | w -> Error (Op_failed ("no operand list: " ^ word_to_string w)))
  ; one_word "first-operand" (function
      | Seq (e :: _) -> exp_word e
      | Seq [] -> Error (Op_failed "first-operand of an empty list")
      | w -> Error (Op_failed ("no operand list: " ^ word_to_string w)))
  ; one_word "rest-operands" (function
      | Seq (_ :: rest) -> seq_word rest
      | Seq [] -> Error (Op_failed "rest-operands of an empty list")
      | w -> Error (Op_failed ("no operand list: " ^ word_to_string w)))
  ; one_word "last-operand?" (function
      | Seq [ _ ] -> Ok (V (Value.bool true))
      | Seq _ -> Ok (V (Value.bool false))
      | w -> Error (Op_failed ("no operand list: " ^ word_to_string w)))
  ; "empty-arglist", Value_op (fun _ -> Ok (Args []))
  ; two_words "adjoin-arg" (fun w argl ->
      match argl with
      | Args ws -> Ok (Args (ws @ [ w ]))
      | _ -> Error (Op_failed "adjoin-arg needs an operand list"))
  ; one_word "no-args?" (function
      | Args [] -> Ok (V (Value.bool true))
      | Args _ -> Ok (V (Value.bool false))
      | w -> Error (Op_failed ("no operand list: " ^ word_to_string w)))
  ; one_word "first-arg" (function
      | Args (w :: _) -> Ok w
      | Args [] -> Error (Op_failed "first-arg of an empty list")
      | w -> Error (Op_failed ("no operand list: " ^ word_to_string w)))
  ; one_word "rest-args" (function
      | Args (_ :: rest) -> Ok (Args rest)
      | Args [] -> Error (Op_failed "rest-args of an empty list")
      | w -> Error (Op_failed ("no operand list: " ^ word_to_string w)))
  ; one_word "primitive-procedure?" (function
      | V v ->
        Ok
          (V
             (Value.bool
                (match Value.view v with
                 | Value.Primitive_procedure _ -> true
                 | _ -> false)))
      | w -> Error (Op_failed ("no procedure: " ^ word_to_string w)))
  ; one_word "compound-procedure?" (function
      | V v ->
        Ok
          (V
             (Value.bool
                (match Value.view v with
                 | Value.Compound_procedure _ -> true
                 | _ -> false)))
      | w -> Error (Op_failed ("no procedure: " ^ word_to_string w)))
  ; two_words "apply-primitive-procedure" (fun proc argl ->
      match proc, argl with
      | V v, Args ws ->
        (match Value.view v with
         | Value.Primitive_procedure name ->
           word_values ws
           >>= fun vs -> Result.map (fun r -> V r) (apply_object_primitive name vs)
         | _ -> Error (Op_failed "apply-primitive-procedure needs a primitive procedure"))
      | _, _ ->
        Error
          (Op_failed "apply-primitive-procedure needs a procedure and an operand list"))
  ; one_word "procedure-parameters" (fun w ->
      compound_of "procedure-parameters" w
      >>= fun cv -> seq_word (List.map Ast.variable cv.parameters))
  ; one_word "procedure-body" (fun w ->
      compound_of "procedure-body" w >>= fun cv -> seq_word cv.body)
  ; one_word "procedure-environment" (fun w ->
      compound_of "procedure-environment" w >>= fun cv -> env_word cv.env)
  ; three_words "extend-environment" (fun params args base ->
      match base with
      | Env env -> extend_environment params args env
      | _ -> Error (Op_failed "extend-environment needs an environment"))
  ; two_words "lookup-variable-value" (fun w env ->
      match variable_of "lookup-variable-value" w, env with
      | Ok n, Env env -> lookup_binding env n
      | Error e, _ -> Error e
      | _, _ -> Error (Op_failed "lookup-variable-value needs an environment"))
  ; ( "set-variable-value!"
    , Action_op
        (function
          | [ w; V v; Env env ] ->
            (match variable_of "set-variable-value!" w with
             | Error e -> Error e
             | Ok n ->
               (match Env.set env n v with
                | Ok () -> Ok ()
                | Error e -> Error (eval_error e)))
          | _ ->
            Error
              (Arity "set-variable-value! needs a variable, a value, and an environment"))
    )
  ; ( "define-variable!"
    , Action_op
        (function
          | [ w; V v; Env env ] ->
            (match variable_of "define-variable!" w with
             | Error e -> Error e
             | Ok n ->
               Env.define env n v;
               Ok ())
          | _ ->
            Error (Arity "define-variable! needs a variable, a value, and an environment"))
    )
  ; one_word "true?" (function
      | V v ->
        Ok
          (V
             (Value.bool
                (match Value.view v with
                 | Value.Bool false -> false
                 | _ -> true)))
      | w -> Error (Op_failed ("true? needs a value, found " ^ word_to_string w)))
  ; three_words "make-procedure" (fun params body env ->
      match params, body, env with
      | Seq _, Seq body, Env env ->
        parameters_of params
        >>= fun names -> Ok (V (Value.compound ~name:None ~parameters:names ~body ~env))
      | _, _, _ ->
        Error (Op_failed "make-procedure needs parameters, a body, and an environment"))
  ]
;;

(** {1:rendering Rendering a word for the transcript} *)

let render_word = function
  | V v -> Value.to_string v
  | Lab l -> l
  | w -> word_to_string w
;;

(** {1:building Building the evaluator machine} *)

(** The operations the controller names that the tables do not
    install. *)
let unknown_operations (program : Sec_5_1.program) installed =
  let named =
    List.fold_left
      (fun acc (inst : Sec_5_1.instruction) ->
         match inst with
         | Assign_op (_, name, _) | Test (name, _) | Perform (name, _) -> name :: acc
         | _ -> acc)
      []
      (Array.to_list program.code)
  in
  let installed_names = List.fold_left (fun acc (n, _) -> n :: acc) [] installed in
  let is_installed n = List.mem n installed_names in
  List.filter (fun n -> not (is_installed n)) named
;;

(** [make_evaluator ~controller ~operations ~source ()] builds the
    section's evaluator machine: the controller text (the book's, or a
    composed exercise variant) is assembled by the 5.1 reader, the
    operations table is the machine's own operations, then
    [base_operations], then the extra [operations] last so they
    override on a name collision, the object program [source] is read
    into the input queue, and the global environment is the book's
    setup: [true], [false], and the object-language primitives.  The
    driver loop ends when the queue runs dry, through the typed
    [Op_failed] whose message is [input_exhausted]; every other
    failure stops the machine too. *)
let make_evaluator ?(controller = base_controller) ?(operations = []) ~source ()
  : (machine, error) result
  =
  Sec_5_1.parse_program ("(controller\n" ^ controller ^ "\n)")
  >>= fun program ->
  let global_env = Env.empty () in
  Env.define global_env "true" (Value.bool true);
  Env.define global_env "false" (Value.bool false);
  List.iter
    (fun (name, f) -> Env.define global_env name (Value.primitive ~name f))
    object_primitives;
  let m =
    { regs = Hashtbl.create 16
    ; operations = Hashtbl.create 64
    ; program
    ; pc = ref 0
    ; the_stack = ref []
    ; pushes = ref 0
    ; depth = ref 0
    ; max_depth = ref 0
    ; output = ref []
    ; input_queue = ref []
    }
  in
  List.iter
    (fun r -> Hashtbl.replace m.regs r (ref (V (Value.symbol "*unassigned*"))))
    evaluator_registers;
  let install (n, o) = Hashtbl.replace m.operations n o in
  List.iter install base_operations;
  (* The machine's own operations, closures over this machine's
     transcript, counters, and input queue. *)
  install ("get-global-environment", Value_op (fun _ -> Ok (Env global_env)));
  install ("initialize-stack", Action_op (fun _ -> initialize_stack m));
  install ("print-stack-statistics", Action_op (fun _ -> say m (print_stack_statistics m)));
  install ("prompt-for-input", Action_op (fun _ -> say m ";;; EC-Eval input:"));
  install ("announce-output", Action_op (fun _ -> say m ";;; EC-Eval value:"));
  install
    ( "read"
    , Value_op
        (function
          | [] ->
            (match !(m.input_queue) with
             | [] -> Error (Op_failed input_exhausted)
             | form :: rest ->
               m.input_queue := rest;
               Ok (Exp form))
          | _ -> Error (Arity "read takes no arguments")) );
  install
    ( "user-print"
    , Action_op
        (function
          | [ w ] -> say m (render_word w)
          | _ -> Error (Arity "user-print needs one argument")) );
  install
    ( "signal-error"
    , Action_op
        (function
          | [ w ] -> Error (Op_failed ("signal-error: " ^ render_word w))
          | _ -> Error (Arity "signal-error needs one argument")) );
  List.iter install operations;
  let all_installed =
    List.map (fun (n, o) -> n, o) (Hashtbl.to_seq m.operations |> List.of_seq)
  in
  (match unknown_operations program all_installed with
   | [] -> Ok ()
   | n :: _ -> Error (Unknown_operation n))
  >>= fun () ->
  match Reader.read_program source with
  | Ok forms ->
    m.input_queue := forms;
    Ok m
  | Error e -> Error (Parse (Reader.to_string e))
;;
