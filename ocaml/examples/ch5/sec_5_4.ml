(* SPDX-License-Identifier: GPL-3.0-only *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude
module Value = Sicp_common.Value
module M = Sec_5_1
module Sec_4_1 = Sicp_ch4.Sec_4_1

type word =
  | V of Value.t
  | Exp of Ast.expr
  | Exps of Ast.expr list
  | Args of Value.t list
  | Env of Env.t
  | Lab of string
  | Cases of (Ast.pattern * Ast.expr) list
  | Pat of Ast.pattern
  | Pending of Env.t * (Ast.binding * Env.cell option) list
  | Unassigned

let word_to_string = function
  | V v -> Value.to_string v
  | Exp _ -> "<expression>"
  | Exps es -> Printf.sprintf "<%d expressions>" (List.length es)
  | Args vs -> "[" ^ String.concat "; " (List.map Value.to_string vs) ^ "]"
  | Env _ -> "<environment>"
  | Lab l -> l
  | Cases cs -> Printf.sprintf "<%d cases>" (List.length cs)
  | Pat _ -> "<pattern>"
  | Pending (_, ps) -> Printf.sprintf "<%d recursive bindings>" (List.length ps)
  | Unassigned -> "unassigned"
;;

let evaluator_registers = [ "exp"; "env"; "val"; "continue"; "proc"; "argl"; "unev" ]

let words =
  { M.label = (fun l -> Lab l)
  ; to_label =
      (function
        | Lab l -> Some l
        | _ -> None)
  ; show = word_to_string
  ; unassigned = Unassigned
  }
;;

(* {1 The controller} *)

let r name = M.Reg name
let l name = M.Label_ref name
let assign target source = M.Assign (target, source)
let compute target op sources = M.Assign_op (target, op, sources)
let test op sources = M.Test (op, sources)
let branch_if op sources target = [ test op sources; M.Branch target ]
let return = M.Goto_reg "continue"
let dispatch = M.Goto "eval-dispatch"
let driver = [ assign "continue" (l "done"); dispatch ]

let eval_dispatch =
  List.concat
    [ [ M.Label "eval-dispatch" ]
    ; branch_if "self-evaluating?" [ r "exp" ] "ev-self-eval"
    ; branch_if "variable?" [ r "exp" ] "ev-variable"
    ; branch_if "fun?" [ r "exp" ] "ev-lambda"
    ; branch_if "if?" [ r "exp" ] "ev-if"
    ; branch_if "match?" [ r "exp" ] "ev-match"
    ; branch_if "let-rec?" [ r "exp" ] "ev-let-rec"
    ; branch_if "let?" [ r "exp" ] "ev-let"
    ; branch_if "sequence?" [ r "exp" ] "ev-sequence"
    ; branch_if "and?" [ r "exp" ] "ev-and"
    ; branch_if "or?" [ r "exp" ] "ev-or"
    ; branch_if "unary?" [ r "exp" ] "ev-unary"
    ; branch_if "binary?" [ r "exp" ] "ev-binary"
    ; branch_if "construction?" [ r "exp" ] "ev-collect"
    ; branch_if "application?" [ r "exp" ] "ev-application"
    ; [ M.Perform ("unknown-expression-type", [ r "exp" ]) ]
    ]
;;

let ev_simple =
  [ M.Label "ev-self-eval"
  ; compute "val" "literal-value" [ r "exp" ]
  ; return
  ; M.Label "ev-variable"
  ; compute "val" "lookup-variable-value" [ r "exp"; r "env" ]
  ; return
  ; M.Label "ev-lambda"
  ; compute "val" "make-procedure" [ r "exp"; r "env" ]
  ; return
  ]
;;

let ev_if =
  [ M.Label "ev-if"
  ; M.Save "exp"
  ; M.Save "env"
  ; M.Save "continue"
  ; assign "continue" (l "ev-if-decide")
  ; compute "exp" "if-predicate" [ r "exp" ]
  ; dispatch
  ; M.Label "ev-if-decide"
  ; M.Restore "continue"
  ; M.Restore "env"
  ; M.Restore "exp"
  ; test "true?" [ r "val" ]
  ; M.Branch "ev-if-consequent"
  ; compute "exp" "if-alternative" [ r "exp" ]
  ; dispatch
  ; M.Label "ev-if-consequent"
  ; compute "exp" "if-consequent" [ r "exp" ]
  ; dispatch
  ]
;;

(* A match runs its scrutinee once, then tries the cases in order; the
   first matching case's body runs in tail position. *)
let ev_match =
  [ M.Label "ev-match"
  ; M.Save "exp"
  ; M.Save "env"
  ; M.Save "continue"
  ; assign "continue" (l "ev-match-cases")
  ; compute "exp" "match-scrutinee" [ r "exp" ]
  ; dispatch
  ; M.Label "ev-match-cases"
  ; M.Restore "continue"
  ; M.Restore "env"
  ; M.Restore "exp"
  ; compute "unev" "match-cases" [ r "exp" ]
  ; M.Label "ev-match-loop"
  ; test "no-cases?" [ r "unev" ]
  ; M.Branch "match-failure"
  ; compute "argl" "try-first-case" [ r "unev"; r "val"; r "env" ]
  ; test "matched?" [ r "argl" ]
  ; M.Branch "ev-match-body"
  ; compute "unev" "rest-cases" [ r "unev" ]
  ; M.Goto "ev-match-loop"
  ; M.Label "ev-match-body"
  ; assign "env" (r "argl")
  ; compute "exp" "first-case-body" [ r "unev" ]
  ; dispatch
  ; M.Label "match-failure"
  ; M.Perform ("signal-match-failure", [ r "val" ])
  ]
;;

(* A parallel [let] evaluates its right-hand sides with the collection
   loop, then runs its body in tail position; a recursive group binds
   fresh cells first and fills them in order. *)
let ev_let =
  [ M.Label "ev-let-rec"
  ; compute "unev" "let-rec-group" [ r "exp"; r "env" ]
  ; compute "env" "group-environment" [ r "unev" ]
  ; M.Label "ev-let-rec-loop"
  ; test "no-pending?" [ r "unev" ]
  ; M.Branch "ev-let-body"
  ; M.Save "continue"
  ; M.Save "exp"
  ; M.Save "env"
  ; M.Save "unev"
  ; assign "continue" (l "ev-let-rec-fill")
  ; compute "exp" "first-pending-rhs" [ r "unev" ]
  ; dispatch
  ; M.Label "ev-let-rec-fill"
  ; M.Restore "unev"
  ; M.Restore "env"
  ; M.Restore "exp"
  ; M.Restore "continue"
  ; M.Perform ("fill-first-pending", [ r "unev"; r "val" ])
  ; compute "unev" "rest-pending" [ r "unev" ]
  ; M.Goto "ev-let-rec-loop"
  ; M.Label "ev-let"
  ; M.Goto "ev-collect"
  ; M.Label "ev-let-bind"
  ; compute "env" "let-environment" [ r "exp"; r "argl"; r "env" ]
  ; M.Label "ev-let-body"
  ; compute "exp" "let-body" [ r "exp" ]
  ; dispatch
  ]
;;

let ev_sequence =
  [ M.Label "ev-sequence"
  ; M.Save "exp"
  ; M.Save "env"
  ; M.Save "continue"
  ; assign "continue" (l "ev-sequence-rest")
  ; compute "exp" "first-expression" [ r "exp" ]
  ; dispatch
  ; M.Label "ev-sequence-rest"
  ; M.Restore "continue"
  ; M.Restore "env"
  ; M.Restore "exp"
  ; compute "exp" "second-expression" [ r "exp" ]
  ; dispatch
  ]
;;

(* [&&] and [||] evaluate their right operand only when the left one
   does not decide, and then in tail position. *)
let ev_logic =
  let block name decided =
    [ M.Label ("ev-" ^ name)
    ; M.Save "exp"
    ; M.Save "env"
    ; M.Save "continue"
    ; assign "continue" (l ("ev-" ^ name ^ "-decide"))
    ; compute "exp" "left-operand" [ r "exp" ]
    ; dispatch
    ; M.Label ("ev-" ^ name ^ "-decide")
    ; M.Restore "continue"
    ; M.Restore "env"
    ; M.Restore "exp"
    ; test decided [ r "val" ]
    ; M.Branch "ev-return-val"
    ; compute "exp" "right-operand" [ r "exp" ]
    ; dispatch
    ]
  in
  List.concat
    [ block "and" "false?"; block "or" "true?"; [ M.Label "ev-return-val"; return ] ]
;;

let ev_unary =
  [ M.Label "ev-unary"
  ; M.Save "exp"
  ; M.Save "continue"
  ; assign "continue" (l "ev-unary-apply")
  ; compute "exp" "unary-operand" [ r "exp" ]
  ; dispatch
  ; M.Label "ev-unary-apply"
  ; M.Restore "continue"
  ; M.Restore "exp"
  ; compute "val" "apply-unary" [ r "exp"; r "val" ]
  ; return
  ]
;;

(* Binary operators evaluate the left operand, save it, evaluate the
   right one, and restore the left one into [argl]. *)
let ev_binary =
  [ M.Label "ev-binary"
  ; M.Save "continue"
  ; M.Save "exp"
  ; M.Save "env"
  ; assign "continue" (l "ev-binary-right")
  ; compute "exp" "left-operand" [ r "exp" ]
  ; dispatch
  ; M.Label "ev-binary-right"
  ; M.Restore "env"
  ; M.Restore "exp"
  ; M.Save "exp"
  ; M.Save "val"
  ; assign "continue" (l "ev-binary-apply")
  ; compute "exp" "right-operand" [ r "exp" ]
  ; dispatch
  ; M.Label "ev-binary-apply"
  ; M.Restore "argl"
  ; M.Restore "exp"
  ; M.Restore "continue"
  ; compute "val" "apply-binary" [ r "exp"; r "argl"; r "val" ]
  ; return
  ]
;;

(* Tuples, constructors, records, and parallel [let] right-hand sides
   evaluate a list of operands left to right into [argl]; the node in
   [exp] then decides what the list builds. *)
let ev_collect =
  [ M.Label "ev-collect"
  ; M.Save "continue"
  ; M.Save "exp"
  ; assign "argl" (M.Const (Args []))
  ; compute "unev" "collected-operands" [ r "exp" ]
  ; M.Label "ev-collect-loop"
  ; test "no-operands?" [ r "unev" ]
  ; M.Branch "ev-collect-done"
  ; M.Save "argl"
  ; M.Save "env"
  ; M.Save "unev"
  ; assign "continue" (l "ev-collect-accumulate")
  ; compute "exp" "first-operand" [ r "unev" ]
  ; dispatch
  ; M.Label "ev-collect-accumulate"
  ; M.Restore "unev"
  ; M.Restore "env"
  ; M.Restore "argl"
  ; compute "argl" "adjoin-arg" [ r "val"; r "argl" ]
  ; compute "unev" "rest-operands" [ r "unev" ]
  ; M.Goto "ev-collect-loop"
  ; M.Label "ev-collect-done"
  ; M.Restore "exp"
  ; M.Restore "continue"
  ; test "let?" [ r "exp" ]
  ; M.Branch "ev-let-bind"
  ; compute "val" "build" [ r "exp"; r "argl" ]
  ; return
  ]
;;

let ev_application =
  [ M.Label "ev-application"
  ; M.Save "continue"
  ; M.Save "env"
  ; compute "unev" "operands" [ r "exp" ]
  ; M.Save "unev"
  ; compute "exp" "operator" [ r "exp" ]
  ; assign "continue" (l "ev-appl-did-operator")
  ; dispatch
  ; M.Label "ev-appl-did-operator"
  ; M.Restore "unev"
  ; M.Restore "env"
  ; assign "argl" (M.Const (Args []))
  ; assign "proc" (r "val")
  ; test "no-operands?" [ r "unev" ]
  ; M.Branch "apply-dispatch"
  ; M.Save "proc"
  ; M.Label "ev-appl-operand-loop"
  ; M.Save "argl"
  ; compute "exp" "first-operand" [ r "unev" ]
  ; test "last-operand?" [ r "unev" ]
  ; M.Branch "ev-appl-last-arg"
  ; M.Save "env"
  ; M.Save "unev"
  ; assign "continue" (l "ev-appl-accumulate-arg")
  ; dispatch
  ; M.Label "ev-appl-accumulate-arg"
  ; M.Restore "unev"
  ; M.Restore "env"
  ; M.Restore "argl"
  ; compute "argl" "adjoin-arg" [ r "val"; r "argl" ]
  ; compute "unev" "rest-operands" [ r "unev" ]
  ; M.Goto "ev-appl-operand-loop"
  ; M.Label "ev-appl-last-arg"
  ; assign "continue" (l "ev-appl-accum-last-arg")
  ; dispatch
  ; M.Label "ev-appl-accum-last-arg"
  ; M.Restore "argl"
  ; compute "argl" "adjoin-arg" [ r "val"; r "argl" ]
  ; M.Restore "proc"
  ; M.Goto "apply-dispatch"
  ]
;;

(* A closure consumes as many operands as it has parameters; a closure
   given fewer answers the partial closure; operands left over after a
   body returns are applied to its value.  As in the book, [continue]
   is on the stack at [apply-dispatch], saved by [ev-application]; a
   saturated call with no operands left over restores it and runs the
   body with nothing saved: the tail call of 5.4.2. *)
let apply_dispatch =
  [ M.Label "apply-entry"
  ; M.Save "continue"
  ; M.Label "apply-dispatch"
  ; test "primitive-procedure?" [ r "proc" ]
  ; M.Branch "primitive-apply"
  ; test "compound-procedure?" [ r "proc" ]
  ; M.Branch "compound-apply"
  ; M.Perform ("signal-not-applicable", [ r "proc" ])
  ; M.Label "primitive-apply"
  ; compute "val" "apply-primitive-procedure" [ r "proc"; r "argl" ]
  ; compute "argl" "primitive-excess-arguments" [ r "proc"; r "argl" ]
  ; M.Restore "continue"
  ; M.Goto "apply-excess"
  ; M.Label "compound-apply"
  ; test "partial-application?" [ r "proc"; r "argl" ]
  ; M.Branch "compound-partial"
  ; compute "env" "bind-parameters" [ r "proc"; r "argl" ]
  ; compute "exp" "procedure-body" [ r "proc" ]
  ; compute "argl" "compound-excess-arguments" [ r "proc"; r "argl" ]
  ; test "no-arguments?" [ r "argl" ]
  ; M.Branch "compound-tail"
  ; M.Save "argl"
  ; assign "continue" (l "compound-excess")
  ; dispatch
  ; M.Label "compound-excess"
  ; M.Restore "argl"
  ; M.Restore "continue"
  ; M.Goto "apply-excess"
  ; M.Label "compound-tail"
  ; M.Restore "continue"
  ; dispatch
  ; M.Label "compound-partial"
  ; compute "val" "partial-procedure" [ r "proc"; r "argl" ]
  ; M.Restore "continue"
  ; return
  ; M.Label "apply-excess"
  ; test "no-arguments?" [ r "argl" ]
  ; M.Branch "ev-return-val"
  ; assign "proc" (r "val")
  ; M.Save "continue"
  ; M.Goto "apply-dispatch"
  ]
;;

let controller_fragments =
  [ "driver", driver
  ; "eval-dispatch", eval_dispatch
  ; "ev-simple", ev_simple
  ; "ev-if", ev_if
  ; "ev-match", ev_match
  ; "ev-let", ev_let
  ; "ev-sequence", ev_sequence
  ; "ev-logic", ev_logic
  ; "ev-unary", ev_unary
  ; "ev-binary", ev_binary
  ; "ev-collect", ev_collect
  ; "ev-application", ev_application
  ; "apply-dispatch", apply_dispatch
  ; "done", [ M.Label "done" ]
  ]
;;

let base_controller = List.concat_map snd controller_fragments

(* {1 The machine operations}

   Operations inspect syntax, build values, and bind environments; they
   never evaluate a subexpression.  The one exception is a primitive
   that calls a guest procedure (the [List] members): it re-enters the
   controller at [apply-dispatch] on a fresh machine. *)

let bad detail = Error (Eval_error.Bad_instruction detail)
let type_error detail = Error (Eval_error.Type_error detail)

let arity expected ws =
  Error (Eval_error.Arity_mismatch { expected; given = List.length ws })
;;

let expr what = function
  | Exp e -> Ok e
  | w -> bad (what ^ " expects an expression, not " ^ word_to_string w)
;;

let value what = function
  | V v -> Ok v
  | w -> bad (what ^ " expects a value, not " ^ word_to_string w)
;;

let environment what = function
  | Env env -> Ok env
  | w -> bad (what ^ " expects an environment, not " ^ word_to_string w)
;;

let arguments what = function
  | Args vs -> Ok vs
  | w -> bad (what ^ " expects an argument list, not " ^ word_to_string w)
;;

let shape what = bad (what ^ ": the expression has another shape")
let value_op name f = name, M.Value_op f
let test_op name f = name, M.Test_op f

let unary_op name f =
  value_op name (function
    | [ w ] -> f w
    | ws -> arity 1 ws)
;;

let syntax_op name f = unary_op name (fun w -> Result.bind (expr name w) f)

let syntax_test name p =
  test_op name (function
    | [ w ] -> Result.map (fun e -> p (Ast.view e)) (expr name w)
    | ws -> arity 1 ws)
;;

let is_true what v =
  match Value.view v with
  | Value.Bool b -> Ok b
  | _ -> type_error (what ^ ": a condition is not a bool")
;;

let take n xs = List.filteri (fun i _ -> i < n) xs
let drop n xs = List.filteri (fun i _ -> i >= n) xs

let closure what w =
  let* v = value what w in
  match Value.view v with
  | Value.Closure c -> Ok c
  | _ -> bad (what ^ " expects a closure")
;;

let primitive what w =
  let* v = value what w in
  match Value.view v with
  | Value.Primitive p -> Ok (p, [])
  | Value.Partial (p, gathered) -> Ok (p, gathered)
  | _ -> bad (what ^ " expects a primitive")
;;

let collected what e =
  match Ast.view e with
  | Ast.Tuple parts -> Ok parts
  | Ast.Construct (_, fields) -> Ok fields
  | Ast.Record fields -> Ok (List.map snd fields)
  | Ast.Let (false, bindings, _) ->
    Ok (List.map (fun (b : Ast.binding) -> b.rhs) bindings)
  | _ -> shape what
;;

let named_values bindings values =
  List.filter_map
    (fun ((b : Ast.binding), v) -> Option.map (fun name -> name, v) b.name)
    (List.combine bindings values)
;;

let unary e v =
  match Ast.view e with
  | Ast.Not _ ->
    let* b = is_true "not" v in
    Ok (Value.bool (not b))
  | Ast.Neg _ -> Sec_4_1.negate v
  | Ast.Deref _ ->
    (match Value.view v with
     | Value.Ref cell -> Ok !cell
     | _ -> type_error "! target is not a reference")
  | Ast.Make_ref _ -> Ok (Value.ref_value v)
  | Ast.Field (_, name) ->
    (match Value.view v with
     | Value.Record fields ->
       (match List.assoc_opt name fields with
        | Some field -> Ok field
        | None -> type_error ("record field " ^ name ^ " is absent"))
     | _ -> type_error "field access target is not a record")
  | _ -> shape "apply-unary"
;;

let binary e left right =
  match Ast.view e with
  | Ast.Arith (op, _, _) -> Sec_4_1.arithmetic op left right
  | Ast.Compare (op, _, _) -> Sec_4_1.comparison op left right
  | Ast.Concat _ ->
    (match Value.view left, Value.view right with
     | Value.String a, Value.String b -> Ok (Value.string (a ^ b))
     | _ -> type_error "^ operands are not strings")
  | Ast.Cons _ -> Ok (Value.cons left right)
  | Ast.Assign _ ->
    (match Value.view left with
     | Value.Ref cell ->
       cell := right;
       Ok Value.unit
     | _ -> type_error ":= target is not a reference")
  | _ -> shape "apply-binary"
;;

let operation_table ~apply =
  [ syntax_test "self-evaluating?" (function
      | Ast.Scalar _ | Ast.Nil -> true
      | _ -> false)
  ; syntax_test "variable?" (function
      | Ast.Var _ -> true
      | _ -> false)
  ; syntax_test "fun?" (function
      | Ast.Fun _ -> true
      | _ -> false)
  ; syntax_test "if?" (function
      | Ast.If _ -> true
      | _ -> false)
  ; syntax_test "match?" (function
      | Ast.Match _ -> true
      | _ -> false)
  ; syntax_test "let-rec?" (function
      | Ast.Let (true, _, _) -> true
      | _ -> false)
  ; syntax_test "let?" (function
      | Ast.Let (false, _, _) -> true
      | _ -> false)
  ; syntax_test "sequence?" (function
      | Ast.Sequence _ -> true
      | _ -> false)
  ; syntax_test "and?" (function
      | Ast.And _ -> true
      | _ -> false)
  ; syntax_test "or?" (function
      | Ast.Or _ -> true
      | _ -> false)
  ; syntax_test "unary?" (function
      | Ast.Not _ | Ast.Neg _ | Ast.Deref _ | Ast.Make_ref _ | Ast.Field _ -> true
      | _ -> false)
  ; syntax_test "binary?" (function
      | Ast.Arith _ | Ast.Compare _ | Ast.Concat _ | Ast.Cons _ | Ast.Assign _ -> true
      | _ -> false)
  ; syntax_test "construction?" (function
      | Ast.Tuple _ | Ast.Construct _ | Ast.Record _ -> true
      | _ -> false)
  ; syntax_test "application?" (function
      | Ast.Apply _ -> true
      | _ -> false)
  ; ( "unknown-expression-type"
    , M.Action_op (fun _ -> Error (Eval_error.Invalid_form "unknown expression type")) )
  ; syntax_op "literal-value" (fun e ->
      match Ast.view e with
      | Ast.Scalar s -> Ok (V (Sec_4_1.scalar_value s))
      | Ast.Nil -> Ok (V Value.nil)
      | _ -> shape "literal-value")
  ; value_op "lookup-variable-value" (function
      | [ e; env ] ->
        let* e = expr "lookup-variable-value" e in
        let* env = environment "lookup-variable-value" env in
        (match Ast.view e with
         | Ast.Var name ->
           (match Env.find env name with
            | Some v -> Ok (V v)
            | None -> Error (Eval_error.Unbound_variable name))
         | _ -> shape "lookup-variable-value")
      | ws -> arity 2 ws)
  ; value_op "make-procedure" (function
      | [ e; env ] ->
        let* e = expr "make-procedure" e in
        let* env = environment "make-procedure" env in
        (match Ast.view e with
         | Ast.Fun (parameters, body) ->
           Ok (V (Value.closure ~name:None ~parameters ~body ~env))
         | _ -> shape "make-procedure")
      | ws -> arity 2 ws)
  ; syntax_op "if-predicate" (fun e ->
      match Ast.view e with
      | Ast.If (c, _, _) -> Ok (Exp c)
      | _ -> shape "if-predicate")
  ; syntax_op "if-consequent" (fun e ->
      match Ast.view e with
      | Ast.If (_, c, _) -> Ok (Exp c)
      | _ -> shape "if-consequent")
  ; syntax_op "if-alternative" (fun e ->
      match Ast.view e with
      | Ast.If (_, _, a) -> Ok (Exp a)
      | _ -> shape "if-alternative")
  ; test_op "true?" (function
      | [ w ] -> Result.bind (value "true?" w) (is_true "true?")
      | ws -> arity 1 ws)
  ; test_op "false?" (function
      | [ w ] -> Result.map not (Result.bind (value "false?" w) (is_true "false?"))
      | ws -> arity 1 ws)
  ; syntax_op "match-scrutinee" (fun e ->
      match Ast.view e with
      | Ast.Match (s, _) -> Ok (Exp s)
      | _ -> shape "match-scrutinee")
  ; syntax_op "match-cases" (fun e ->
      match Ast.view e with
      | Ast.Match (_, cases) -> Ok (Cases cases)
      | _ -> shape "match-cases")
  ; test_op "no-cases?" (function
      | [ Cases cases ] -> Ok (cases = [])
      | ws -> arity 1 ws)
  ; value_op "try-first-case" (function
      | [ Cases ((pattern, _) :: _); v; env ] ->
        let* v = value "try-first-case" v in
        let* env = environment "try-first-case" env in
        (match Sec_4_1.bind_pattern pattern v with
         | Some bindings -> Ok (Env (Env.extend bindings env))
         | None -> Ok Unassigned)
      | ws -> arity 3 ws)
  ; test_op "matched?" (function
      | [ Env _ ] -> Ok true
      | [ _ ] -> Ok false
      | ws -> arity 1 ws)
  ; unary_op "rest-cases" (function
      | Cases (_ :: rest) -> Ok (Cases rest)
      | w -> bad ("rest-cases of " ^ word_to_string w))
  ; unary_op "first-case-body" (function
      | Cases ((_, body) :: _) -> Ok (Exp body)
      | w -> bad ("first-case-body of " ^ word_to_string w))
  ; "signal-match-failure", M.Action_op (fun _ -> type_error "no case matched")
  ; value_op "let-rec-group" (function
      | [ e; env ] ->
        let* e = expr "let-rec-group" e in
        let* env = environment "let-rec-group" env in
        (match Ast.view e with
         | Ast.Let (true, bindings, _) ->
           let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
           let env, cells = Env.extend_recursive names env in
           let rec pair cells = function
             | [] -> []
             | ({ Ast.name = Some _; _ } as b) :: rest ->
               (match cells with
                | cell :: cells -> (b, Some cell) :: pair cells rest
                | [] -> (b, None) :: pair [] rest)
             | b :: rest -> (b, None) :: pair cells rest
           in
           Ok (Pending (env, pair cells bindings))
         | _ -> shape "let-rec-group")
      | ws -> arity 2 ws)
  ; unary_op "group-environment" (function
      | Pending (env, _) -> Ok (Env env)
      | w -> bad ("group-environment of " ^ word_to_string w))
  ; test_op "no-pending?" (function
      | [ Pending (_, pending) ] -> Ok (pending = [])
      | ws -> arity 1 ws)
  ; unary_op "first-pending-rhs" (function
      | Pending (_, (b, _) :: _) -> Ok (Exp b.Ast.rhs)
      | w -> bad ("first-pending-rhs of " ^ word_to_string w))
  ; ( "fill-first-pending"
    , M.Action_op
        (function
          | [ Pending (_, (_, cell) :: _); v ] ->
            let* v = value "fill-first-pending" v in
            Option.iter (fun cell -> Env.fill cell v) cell;
            Ok ()
          | ws -> arity 2 ws) )
  ; unary_op "rest-pending" (function
      | Pending (env, _ :: rest) -> Ok (Pending (env, rest))
      | w -> bad ("rest-pending of " ^ word_to_string w))
  ; value_op "let-environment" (function
      | [ e; vs; env ] ->
        let* e = expr "let-environment" e in
        let* vs = arguments "let-environment" vs in
        let* env = environment "let-environment" env in
        (match Ast.view e with
         | Ast.Let (false, bindings, _) ->
           Ok (Env (Env.extend (named_values bindings vs) env))
         | _ -> shape "let-environment")
      | ws -> arity 3 ws)
  ; syntax_op "let-body" (fun e ->
      match Ast.view e with
      | Ast.Let (_, _, body) -> Ok (Exp body)
      | _ -> shape "let-body")
  ; syntax_op "first-expression" (fun e ->
      match Ast.view e with
      | Ast.Sequence (a, _) -> Ok (Exp a)
      | _ -> shape "first-expression")
  ; syntax_op "second-expression" (fun e ->
      match Ast.view e with
      | Ast.Sequence (_, b) -> Ok (Exp b)
      | _ -> shape "second-expression")
  ; syntax_op "left-operand" (fun e ->
      match Ast.view e with
      | Ast.And (a, _)
      | Ast.Or (a, _)
      | Ast.Arith (_, a, _)
      | Ast.Compare (_, a, _)
      | Ast.Concat (a, _)
      | Ast.Cons (a, _)
      | Ast.Assign (a, _) -> Ok (Exp a)
      | _ -> shape "left-operand")
  ; syntax_op "right-operand" (fun e ->
      match Ast.view e with
      | Ast.And (_, b)
      | Ast.Or (_, b)
      | Ast.Arith (_, _, b)
      | Ast.Compare (_, _, b)
      | Ast.Concat (_, b)
      | Ast.Cons (_, b)
      | Ast.Assign (_, b) -> Ok (Exp b)
      | _ -> shape "right-operand")
  ; syntax_op "unary-operand" (fun e ->
      match Ast.view e with
      | Ast.Not a | Ast.Neg a | Ast.Deref a | Ast.Make_ref a | Ast.Field (a, _) ->
        Ok (Exp a)
      | _ -> shape "unary-operand")
  ; value_op "apply-unary" (function
      | [ e; v ] ->
        let* e = expr "apply-unary" e in
        let* v = value "apply-unary" v in
        Result.map (fun v -> V v) (unary e v)
      | ws -> arity 2 ws)
  ; value_op "apply-binary" (function
      | [ e; left; right ] ->
        let* e = expr "apply-binary" e in
        let* left =
          match left with
          | V v -> Ok v
          | w -> bad ("apply-binary left operand " ^ word_to_string w)
        in
        let* right = value "apply-binary" right in
        Result.map (fun v -> V v) (binary e left right)
      | ws -> arity 3 ws)
  ; syntax_op "collected-operands" (fun e ->
      Result.map (fun es -> Exps es) (collected "collected-operands" e))
  ; value_op "build" (function
      | [ e; vs ] ->
        let* e = expr "build" e in
        let* vs = arguments "build" vs in
        (match Ast.view e with
         | Ast.Tuple _ -> Ok (V (Value.tuple vs))
         | Ast.Construct (name, _) -> Ok (V (Value.construct name vs))
         | Ast.Record fields ->
           Ok (V (Value.record (List.combine (List.map fst fields) vs)))
         | _ -> shape "build")
      | ws -> arity 2 ws)
  ; syntax_op "operator" (fun e ->
      match Ast.view e with
      | Ast.Apply (f, _) -> Ok (Exp f)
      | _ -> shape "operator")
  ; syntax_op "operands" (fun e ->
      match Ast.view e with
      | Ast.Apply (_, args) -> Ok (Exps args)
      | _ -> shape "operands")
  ; test_op "no-operands?" (function
      | [ Exps es ] -> Ok (es = [])
      | ws -> arity 1 ws)
  ; test_op "last-operand?" (function
      | [ Exps es ] -> Ok (List.length es = 1)
      | ws -> arity 1 ws)
  ; unary_op "first-operand" (function
      | Exps (e :: _) -> Ok (Exp e)
      | w -> bad ("first-operand of " ^ word_to_string w))
  ; unary_op "rest-operands" (function
      | Exps (_ :: rest) -> Ok (Exps rest)
      | w -> bad ("rest-operands of " ^ word_to_string w))
  ; value_op "adjoin-arg" (function
      | [ v; vs ] ->
        let* v = value "adjoin-arg" v in
        let* vs = arguments "adjoin-arg" vs in
        Ok (Args (vs @ [ v ]))
      | ws -> arity 2 ws)
  ; test_op "no-arguments?" (function
      | [ Args vs ] -> Ok (vs = [])
      | ws -> arity 1 ws)
  ; test_op "primitive-procedure?" (function
      | [ V v ] ->
        Ok
          (match Value.view v with
           | Value.Primitive _ | Value.Partial _ -> true
           | _ -> false)
      | [ _ ] -> Ok false
      | ws -> arity 1 ws)
  ; test_op "compound-procedure?" (function
      | [ V v ] ->
        Ok
          (match Value.view v with
           | Value.Closure _ -> true
           | _ -> false)
      | [ _ ] -> Ok false
      | ws -> arity 1 ws)
  ; ( "signal-not-applicable"
    , M.Action_op
        (function
          | [ V v ] -> Error (Eval_error.Not_applicable (Value.to_string v))
          | ws -> arity 1 ws) )
  ; value_op "apply-primitive-procedure" (function
      | [ proc; vs ] ->
        let* p, gathered = primitive "apply-primitive-procedure" proc in
        let* vs = arguments "apply-primitive-procedure" vs in
        let missing = p.prim_arity - List.length gathered in
        if List.length vs < missing
        then Ok (V (Value.partial p (gathered @ vs)))
        else Result.map (fun v -> V v) (p.prim_apply apply (gathered @ take missing vs))
      | ws -> arity 2 ws)
  ; value_op "primitive-excess-arguments" (function
      | [ proc; vs ] ->
        let* p, gathered = primitive "primitive-excess-arguments" proc in
        let* vs = arguments "primitive-excess-arguments" vs in
        Ok (Args (drop (p.prim_arity - List.length gathered) vs))
      | ws -> arity 2 ws)
  ; test_op "partial-application?" (function
      | [ proc; vs ] ->
        let* c = closure "partial-application?" proc in
        let* vs = arguments "partial-application?" vs in
        Ok (List.length vs < List.length c.parameters)
      | ws -> arity 2 ws)
  ; value_op "bind-parameters" (function
      | [ proc; vs ] ->
        let* c = closure "bind-parameters" proc in
        let* vs = arguments "bind-parameters" vs in
        let n = List.length c.parameters in
        Ok (Env (Env.extend (List.combine c.parameters (take n vs)) c.env))
      | ws -> arity 2 ws)
  ; unary_op "procedure-body" (fun w ->
      let* c = closure "procedure-body" w in
      Ok (Exp c.body))
  ; value_op "compound-excess-arguments" (function
      | [ proc; vs ] ->
        let* c = closure "compound-excess-arguments" proc in
        let* vs = arguments "compound-excess-arguments" vs in
        Ok (Args (drop (List.length c.parameters) vs))
      | ws -> arity 2 ws)
  ; value_op "partial-procedure" (function
      | [ proc; vs ] ->
        let* c = closure "partial-procedure" proc in
        let* vs = arguments "partial-procedure" vs in
        let n = List.length vs in
        let env = Env.extend (List.combine (take n c.parameters) vs) c.env in
        Ok
          (V
             (Value.closure
                ~name:c.name
                ~parameters:(drop n c.parameters)
                ~body:c.body
                ~env))
      | ws -> arity 2 ws)
  ]
;;

let base_operation_names =
  List.map fst (operation_table ~apply:(fun _ _ -> Ok Value.unit))
;;

(* {1 The driver} *)

type evaluator =
  { machine : word M.machine
  ; global : Env.t
  }

let machine ev = ev.machine

let collect m =
  let* () = M.start m in
  let* w = M.get_register m "val" in
  value "the driver" w
;;

let enter m label registers =
  M.restart m;
  let* () =
    List.fold_left
      (fun acc (name, w) ->
         let* () = acc in
         M.set_register m name w)
      (Ok ())
      (("continue", Lab "done") :: registers)
  in
  let* () = M.goto_label m label in
  collect m
;;

let make_evaluator ?(operations = []) ?(registers = []) ~controller ~emit () =
  let sub =
    ref (fun () -> Error (Eval_error.Invalid_form "evaluator is not assembled"))
  in
  (* A primitive's guest callback runs on a fresh machine of the same
     controller, entered at [apply-dispatch]. *)
  let apply proc vs =
    let* m = !sub () in
    enter m "apply-entry" [ "proc", V proc; "argl", Args vs ]
  in
  let table = operations @ operation_table ~apply in
  let assemble () =
    M.make
      ~words
      ~registers:(evaluator_registers @ registers)
      ~operations:table
      controller
  in
  sub := assemble;
  let* machine = assemble () in
  Ok { machine; global = Prelude.initial_env ~emit () }
;;

let eval ev env e = enter ev.machine "eval-dispatch" [ "exp", Exp e; "env", Env env ]

let run_program ev program =
  let rec go env outcome = function
    | [] -> Ok outcome
    | Ast.Type_item _ :: rest -> go env outcome rest
    | Ast.Value_item (true, bindings) :: rest ->
      let names = List.filter_map (fun (b : Ast.binding) -> b.name) bindings in
      let env, cells = Env.extend_recursive names env in
      let rec fill cells = function
        | [] -> Ok ()
        | (b : Ast.binding) :: bindings ->
          let* v = eval ev env b.rhs in
          (match b.name, cells with
           | Some _, cell :: cells ->
             Env.fill cell v;
             fill cells bindings
           | _, cells -> fill cells bindings)
      in
      let* () = fill cells bindings in
      go env outcome rest
    | Ast.Value_item (false, bindings) :: rest ->
      let rec values acc = function
        | [] -> Ok (List.rev acc)
        | (b : Ast.binding) :: bindings ->
          let* v = eval ev env b.rhs in
          values (v :: acc) bindings
      in
      let* vs = values [] bindings in
      let outcome =
        match List.rev vs with
        | v :: _ -> v
        | [] -> outcome
      in
      go (Env.extend (named_values bindings vs) env) outcome rest
  in
  go ev.global Value.unit (Check.items program)
;;

let run ~emit program =
  let* ev = make_evaluator ~controller:base_controller ~emit () in
  run_program ev program
;;

let stack_statistics_after program =
  let* ev = make_evaluator ~controller:base_controller ~emit:ignore () in
  let* _ = run_program ev program in
  Ok (M.stack_statistics ev.machine)
;;
