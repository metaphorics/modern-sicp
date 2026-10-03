(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 5.5 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Prelude = Sicp_common.Prelude
module Value = Sicp_common.Value
module M = Sec_5_1
module W = Sec_5_4

type seq =
  { needs : string list
  ; modifies : string list
  ; statements : W.word M.instruction list
  }

type linkage =
  | Next
  | Return
  | Goto_label of string

type state = { counter : int ref }

let new_state () = { counter = ref 0 }

let make_label state name =
  incr state.counter;
  name ^ string_of_int !(state.counter)
;;

(* {1 Instruction sequences} *)

let make_instruction_sequence needs modifies statements = { needs; modifies; statements }
let empty_instruction_sequence = make_instruction_sequence [] [] []
let union a b = a @ List.filter (fun r -> not (List.mem r a)) b
let difference a b = List.filter (fun r -> not (List.mem r b)) a

let append_2_sequences a b =
  { needs = union a.needs (difference b.needs a.modifies)
  ; modifies = union a.modifies b.modifies
  ; statements = a.statements @ b.statements
  }
;;

let append_sequences seqs =
  List.fold_left append_2_sequences empty_instruction_sequence seqs
;;

let tack_on_instruction_sequence s body =
  { s with statements = s.statements @ body.statements }
;;

let parallel_instruction_sequences a b =
  { needs = union a.needs b.needs
  ; modifies = union a.modifies b.modifies
  ; statements = a.statements @ b.statements
  }
;;

let rec preserving regs first second =
  match regs with
  | [] -> append_2_sequences first second
  | r :: rest ->
    if List.mem r first.modifies && List.mem r second.needs
    then
      preserving
        rest
        { needs = union [ r ] first.needs
        ; modifies = difference first.modifies [ r ]
        ; statements = (M.Save r :: first.statements) @ [ M.Restore r ]
        }
        second
    else preserving rest first second
;;

let compiled_registers =
  [ "env"; "proc"; "val"; "argl"; "continue"; "unev"; "arg1"; "arg2"; "exp" ]
;;

let all_regs = compiled_registers
let label l = make_instruction_sequence [] [] [ M.Label l ]

(* {1 Linkage} *)

let compile_linkage = function
  | Next -> empty_instruction_sequence
  | Return -> make_instruction_sequence [ "continue" ] [] [ M.Goto_reg "continue" ]
  | Goto_label l -> make_instruction_sequence [] [] [ M.Goto l ]
;;

(* The open recursion of the code generators: [self] compiles every
   subexpression and [pres] makes every preservation. *)
type ctx =
  { self : state -> Ast.expr -> string -> linkage -> seq
  ; pres : string list -> seq -> seq -> seq
  }

let end_with_linkage ctx linkage s = ctx.pres [ "continue" ] s (compile_linkage linkage)
let exp e = M.Const (W.Exp e)

let assign_op needs target op sources =
  make_instruction_sequence needs [ target ] [ M.Assign_op (target, op, sources) ]
;;

(* {1 The code generators} *)

let rec compile_step ctx state e target linkage =
  match Ast.view e with
  | Ast.Scalar s ->
    end_with_linkage
      ctx
      linkage
      (make_instruction_sequence
         []
         [ target ]
         [ M.Assign (target, M.Const (W.V (Sicp_ch4.Sec_4_1.scalar_value s))) ])
  | Ast.Nil ->
    end_with_linkage
      ctx
      linkage
      (make_instruction_sequence
         []
         [ target ]
         [ M.Assign (target, M.Const (W.V Value.nil)) ])
  | Ast.Var _ ->
    end_with_linkage
      ctx
      linkage
      (assign_op [ "env" ] target "lookup-variable-value" [ exp e; M.Reg "env" ])
  | Ast.Fun (_, body) -> compile_lambda ctx state e body target linkage
  | Ast.If (c, t, f) -> compile_if ctx state c t f target linkage
  | Ast.And (a, b) ->
    compile_if ctx state a b (Ast.scalar (Ast.Bool false)) target linkage
  | Ast.Or (a, b) -> compile_if ctx state a (Ast.scalar (Ast.Bool true)) b target linkage
  | Ast.Sequence (a, b) ->
    ctx.pres
      [ "env"; "continue" ]
      (ctx.self state a target Next)
      (ctx.self state b target linkage)
  | Ast.Let (false, bindings, body) ->
    let names =
      List.map (fun (b : Ast.binding) -> Option.value b.name ~default:"_") bindings
    in
    let rhss = List.map (fun (b : Ast.binding) -> b.rhs) bindings in
    compile_application ctx state (Ast.fun_ names body) rhss target linkage
  | Ast.Let (true, bindings, body) ->
    append_2_sequences
      (compile_rec_group ctx state e bindings)
      (ctx.self state body target linkage)
  | Ast.Match (scrutinee, cases) -> compile_match ctx state scrutinee cases target linkage
  | Ast.Apply (fn, args) -> compile_application ctx state fn args target linkage
  | Ast.Arith (_, a, b)
  | Ast.Compare (_, a, b)
  | Ast.Concat (a, b)
  | Ast.Cons (a, b)
  | Ast.Assign (a, b) ->
    end_with_linkage
      ctx
      linkage
      (ctx.pres
         [ "env" ]
         (ctx.self state a "arg1" Next)
         (ctx.pres
            [ "arg1" ]
            (ctx.self state b "arg2" Next)
            (assign_op
               [ "arg1"; "arg2" ]
               target
               "apply-binary"
               [ exp e; M.Reg "arg1"; M.Reg "arg2" ])))
  | Ast.Not a | Ast.Neg a | Ast.Deref a | Ast.Make_ref a | Ast.Field (a, _) ->
    end_with_linkage
      ctx
      linkage
      (append_2_sequences
         (ctx.self state a "val" Next)
         (assign_op [ "val" ] target "apply-unary" [ exp e; M.Reg "val" ]))
  | Ast.Tuple parts -> compile_build ctx state e parts target linkage
  | Ast.Construct (_, fields) -> compile_build ctx state e fields target linkage
  | Ast.Record fields -> compile_build ctx state e (List.map snd fields) target linkage

and compile_build ctx state e operands target linkage =
  end_with_linkage
    ctx
    linkage
    (append_2_sequences
       (construct_arglist ctx state operands)
       (assign_op [ "argl" ] target "build" [ exp e; M.Reg "argl" ]))

and compile_if ctx state c t f target linkage =
  let t_branch = make_label state "true-branch" in
  let f_branch = make_label state "false-branch" in
  let after_if = make_label state "after-if" in
  let consequent_linkage = if linkage = Next then Goto_label after_if else linkage in
  let p_code = ctx.self state c "val" Next in
  let c_code = ctx.self state t target consequent_linkage in
  let a_code = ctx.self state f target linkage in
  ctx.pres
    [ "env"; "continue" ]
    p_code
    (append_sequences
       [ make_instruction_sequence
           [ "val" ]
           []
           [ M.Test ("false?", [ M.Reg "val" ]); M.Branch f_branch ]
       ; parallel_instruction_sequences
           (append_2_sequences (label t_branch) c_code)
           (append_2_sequences (label f_branch) a_code)
       ; label after_if
       ])

(* The 5.43 scan-out: the group's cells are allocated in front of [env],
   then each right-hand side runs in that environment and fills its
   cell in order. *)
and compile_rec_group ctx state group bindings =
  let allocate =
    make_instruction_sequence
      [ "env" ]
      [ "unev"; "env" ]
      [ M.Assign_op ("unev", "let-rec-group", [ exp group; M.Reg "env" ])
      ; M.Assign_op ("env", "group-environment", [ M.Reg "unev" ])
      ]
  in
  let fill (b : Ast.binding) =
    ctx.pres
      [ "unev" ]
      (ctx.self state b.rhs "val" Next)
      (make_instruction_sequence
         [ "unev"; "val" ]
         [ "unev" ]
         [ M.Perform ("fill-first-pending", [ M.Reg "unev"; M.Reg "val" ])
         ; M.Assign_op ("unev", "rest-pending", [ M.Reg "unev" ])
         ])
  in
  let fills =
    List.fold_left
      (fun acc b -> ctx.pres [ "env" ] acc (fill b))
      empty_instruction_sequence
      bindings
  in
  append_2_sequences allocate fills

and compile_match ctx state scrutinee cases target linkage =
  let after = make_label state "after-match" in
  let body_linkage = if linkage = Next then Goto_label after else linkage in
  let labelled = List.map (fun case -> make_label state "match-case", case) cases in
  let tries =
    make_instruction_sequence
      [ "val"; "env" ]
      [ "unev" ]
      (List.concat_map
         (fun (l, (pattern, _)) ->
            [ M.Assign_op
                ( "unev"
                , "try-pattern"
                , [ M.Const (W.Pat pattern); M.Reg "val"; M.Reg "env" ] )
            ; M.Test ("matched?", [ M.Reg "unev" ])
            ; M.Branch l
            ])
         labelled
       @ [ M.Perform ("signal-match-failure", [ M.Reg "val" ]) ])
  in
  let bodies =
    List.fold_left
      (fun acc (l, (_, body)) ->
         parallel_instruction_sequences
           acc
           (append_sequences
              [ label l
              ; make_instruction_sequence
                  [ "unev" ]
                  [ "env" ]
                  [ M.Assign ("env", M.Reg "unev") ]
              ; ctx.self state body target body_linkage
              ]))
      empty_instruction_sequence
      labelled
  in
  ctx.pres
    [ "env"; "continue" ]
    (ctx.self state scrutinee "val" Next)
    (append_sequences [ tries; bodies; label after ])

and compile_lambda ctx state e body target linkage =
  let entry = make_label state "entry" in
  let after = make_label state "after-lambda" in
  let lambda_linkage = if linkage = Next then Goto_label after else linkage in
  let make =
    make_instruction_sequence
      [ "env" ]
      [ target ]
      [ M.Assign_op
          (target, "make-compiled-procedure", [ M.Label_ref entry; exp e; M.Reg "env" ])
      ]
  in
  let body_code =
    append_sequences
      [ label entry
      ; make_instruction_sequence
          [ "proc"; "argl" ]
          [ "env" ]
          [ M.Assign_op ("env", "compiled-procedure-bind", [ M.Reg "proc"; M.Reg "argl" ])
          ]
      ; ctx.self state body "val" Return
      ]
  in
  append_2_sequences
    (tack_on_instruction_sequence (end_with_linkage ctx lambda_linkage make) body_code)
    (label after)

and construct_arglist ctx state operands =
  let start =
    make_instruction_sequence [] [ "argl" ] [ M.Assign ("argl", M.Const (W.Args [])) ]
  in
  List.fold_left
    (fun acc operand ->
       ctx.pres
         [ "env" ]
         acc
         (ctx.pres
            [ "argl" ]
            (ctx.self state operand "val" Next)
            (make_instruction_sequence
               [ "val"; "argl" ]
               [ "argl" ]
               [ M.Assign_op ("argl", "adjoin-arg", [ M.Reg "val"; M.Reg "argl" ]) ])))
    start
    operands

and compile_application ctx state fn args target linkage =
  let proc_code = ctx.self state fn "proc" Next in
  let operand_codes = construct_arglist ctx state args in
  ctx.pres
    [ "env"; "continue" ]
    proc_code
    (ctx.pres
       [ "proc"; "continue" ]
       operand_codes
       (compile_procedure_call ctx state target linkage))

(* A saturated primitive applies in line; every other call enters the
   shared [compiled-apply] routine with [continue] set by the linkage. *)
and compile_procedure_call ctx state target linkage =
  let primitive_branch = make_label state "primitive-branch" in
  let compiled_branch = make_label state "compiled-branch" in
  let after_call = make_label state "after-call" in
  let call_linkage = if linkage = Next then Goto_label after_call else linkage in
  let test =
    make_instruction_sequence
      [ "proc"; "argl" ]
      []
      [ M.Test ("primitive-exact?", [ M.Reg "proc"; M.Reg "argl" ])
      ; M.Branch primitive_branch
      ]
  in
  let compiled =
    append_2_sequences
      (label compiled_branch)
      (compile_proc_appl state target call_linkage)
  in
  let primitive =
    append_2_sequences
      (label primitive_branch)
      (end_with_linkage
         ctx
         call_linkage
         (assign_op
            [ "proc"; "argl" ]
            target
            "apply-primitive-procedure"
            [ M.Reg "proc"; M.Reg "argl" ]))
  in
  append_sequences
    [ test; parallel_instruction_sequences compiled primitive; label after_call ]

and compile_proc_appl state target linkage =
  let call = M.Goto "compiled-apply" in
  match target, linkage with
  | "val", Return ->
    make_instruction_sequence [ "proc"; "argl"; "continue" ] all_regs [ call ]
  | "val", Goto_label l ->
    make_instruction_sequence
      [ "proc"; "argl" ]
      all_regs
      [ M.Assign ("continue", M.Label_ref l); call ]
  | _, Goto_label l ->
    let proc_return = make_label state "proc-return" in
    make_instruction_sequence
      [ "proc"; "argl" ]
      all_regs
      [ M.Assign ("continue", M.Label_ref proc_return)
      ; call
      ; M.Label proc_return
      ; M.Assign (target, M.Reg "val")
      ; M.Goto l
      ]
  | _, Return -> invalid_arg "compile_proc_appl: a return linkage needs target val"
  | _, Next ->
    invalid_arg "compile_proc_appl: a call's linkage is resolved before this point"
;;

let compile_open ?(preserving = preserving) ~self state e target linkage =
  compile_step { self; pres = preserving } state e target linkage
;;

let rec compile state e target linkage =
  compile_step { self = compile; pres = preserving } state e target linkage
;;

let default_ctx = { self = compile; pres = preserving }

let compile_procedure_call state target linkage =
  compile_procedure_call default_ctx state target linkage
;;

(* {1 The runtime} *)

(* [compiled-apply]: [proc] applied to [argl], answering into [val] and
   returning through [continue].  A compiled procedure given exactly its
   parameters jumps to its entry; given fewer it answers the partial
   procedure; given more it is called on its share and the rest is
   applied to the result.  Primitives apply the same way through their
   arity. *)
let runtime =
  make_instruction_sequence
    []
    []
    [ M.Label "compiled-apply"
    ; M.Test ("primitive-procedure?", [ M.Reg "proc" ])
    ; M.Branch "ca-primitive"
    ; M.Test ("compiled-procedure?", [ M.Reg "proc" ])
    ; M.Branch "ca-compiled"
    ; M.Perform ("signal-not-applicable", [ M.Reg "proc" ])
    ; M.Label "ca-primitive"
    ; M.Assign_op ("val", "apply-primitive-procedure", [ M.Reg "proc"; M.Reg "argl" ])
    ; M.Assign_op ("argl", "primitive-excess-arguments", [ M.Reg "proc"; M.Reg "argl" ])
    ; M.Goto "ca-excess"
    ; M.Label "ca-compiled"
    ; M.Test ("compiled-partial?", [ M.Reg "proc"; M.Reg "argl" ])
    ; M.Branch "ca-partial"
    ; M.Assign_op ("unev", "compiled-excess-arguments", [ M.Reg "proc"; M.Reg "argl" ])
    ; M.Assign_op ("argl", "compiled-exact-arguments", [ M.Reg "proc"; M.Reg "argl" ])
    ; M.Test ("no-arguments?", [ M.Reg "unev" ])
    ; M.Branch "ca-tail"
    ; M.Save "continue"
    ; M.Save "unev"
    ; M.Assign ("continue", M.Label_ref "ca-after")
    ; M.Label "ca-tail"
    ; M.Assign_op ("val", "compiled-entry", [ M.Reg "proc" ])
    ; M.Goto_reg "val"
    ; M.Label "ca-after"
    ; M.Restore "argl"
    ; M.Restore "continue"
    ; M.Label "ca-excess"
    ; M.Test ("no-arguments?", [ M.Reg "argl" ])
    ; M.Branch "ca-return"
    ; M.Assign ("proc", M.Reg "val")
    ; M.Goto "compiled-apply"
    ; M.Label "ca-return"
    ; M.Goto_reg "continue"
    ; M.Label "ca-partial"
    ; M.Assign_op ("val", "partial-compiled", [ M.Reg "proc"; M.Reg "argl" ])
    ; M.Goto_reg "continue"
    ]
;;

let compile_program_with ~compile state items =
  let ctx = { self = compile; pres = preserving } in
  let item_code = function
    | Ast.Type_item _ -> empty_instruction_sequence
    | Ast.Value_item (true, bindings) ->
      compile_rec_group ctx state (Ast.let_ true bindings (Ast.scalar Ast.Unit)) bindings
    | Ast.Value_item (false, bindings) ->
      let descriptor = Ast.let_ false bindings (Ast.scalar Ast.Unit) in
      preserving
        [ "env" ]
        (construct_arglist ctx state (List.map (fun (b : Ast.binding) -> b.rhs) bindings))
        (make_instruction_sequence
           [ "argl"; "env" ]
           [ "env"; "val" ]
           [ M.Assign_op
               ("env", "let-environment", [ exp descriptor; M.Reg "argl"; M.Reg "env" ])
           ; M.Assign_op ("val", "last-argument", [ M.Reg "argl" ])
           ])
  in
  append_sequences
    (List.map item_code items
     @ [ make_instruction_sequence [] [] [ M.Goto "done" ]; runtime; label "done" ])
;;

let compile_program state items = compile_program_with ~compile state items

let statements_text s =
  String.concat "\n" (List.map (M.instruction_to_string W.word_to_string) s.statements)
;;

(* {1 Compiled-procedure operations} *)

let bad detail = Error (Eval_error.Bad_instruction detail)

let arity expected ws =
  Error (Eval_error.Arity_mismatch { expected; given = List.length ws })
;;

let compiled what = function
  | W.V v ->
    (match Value.view v with
     | Value.Compiled c -> Ok c
     | _ -> bad (what ^ " expects a compiled procedure"))
  | w -> bad (what ^ " expects a compiled procedure, not " ^ W.word_to_string w)
;;

let arguments what = function
  | W.Args vs -> Ok vs
  | w -> bad (what ^ " expects an argument list, not " ^ W.word_to_string w)
;;

let take n xs = List.filteri (fun i _ -> i < n) xs
let drop n xs = List.filteri (fun i _ -> i >= n) xs

let with_call what f = function
  | [ proc; argl ] ->
    let* c = compiled what proc in
    let* vs = arguments what argl in
    f c vs
  | ws -> arity 2 ws
;;

let compiler_operations =
  [ ( "make-compiled-procedure"
    , M.Value_op
        (function
          | [ W.Lab entry; W.Exp e; W.Env env ] ->
            (match Ast.view e with
             | Ast.Fun (parameters, _) ->
               Ok (W.V (Value.compiled ~entry ~parameters ~env))
             | _ -> bad "make-compiled-procedure expects a fun")
          | ws -> arity 3 ws) )
  ; ( "compiled-procedure-bind"
    , M.Value_op
        (with_call "compiled-procedure-bind" (fun c vs ->
           if List.length vs <> List.length c.entry_parameters
           then arity (List.length c.entry_parameters) vs
           else Ok (W.Env (Env.extend (List.combine c.entry_parameters vs) c.entry_env))))
    )
  ; ( "compiled-procedure?"
    , M.Test_op
        (function
          | [ W.V v ] ->
            Ok
              (match Value.view v with
               | Value.Compiled _ -> true
               | _ -> false)
          | [ _ ] -> Ok false
          | ws -> arity 1 ws) )
  ; ( "compiled-partial?"
    , M.Test_op
        (with_call "compiled-partial?" (fun c vs ->
           Ok (List.length vs < List.length c.entry_parameters))) )
  ; ( "compiled-exact-arguments"
    , M.Value_op
        (with_call "compiled-exact-arguments" (fun c vs ->
           Ok (W.Args (take (List.length c.entry_parameters) vs)))) )
  ; ( "compiled-excess-arguments"
    , M.Value_op
        (with_call "compiled-excess-arguments" (fun c vs ->
           Ok (W.Args (drop (List.length c.entry_parameters) vs)))) )
  ; ( "compiled-entry"
    , M.Value_op
        (function
          | [ proc ] ->
            let* c = compiled "compiled-entry" proc in
            Ok (W.Lab c.entry)
          | ws -> arity 1 ws) )
  ; ( "partial-compiled"
    , M.Value_op
        (with_call "partial-compiled" (fun c vs ->
           let n = List.length vs in
           let env =
             Env.extend (List.combine (take n c.entry_parameters) vs) c.entry_env
           in
           Ok
             (W.V
                (Value.compiled
                   ~entry:c.entry
                   ~parameters:(drop n c.entry_parameters)
                   ~env)))) )
  ; ( "primitive-exact?"
    , M.Test_op
        (function
          | [ W.V v; W.Args vs ] ->
            (match Value.view v with
             | Value.Primitive p -> Ok (List.length vs = p.prim_arity)
             | Value.Partial (p, gathered) ->
               Ok (List.length vs + List.length gathered = p.prim_arity)
             | _ -> Ok false)
          | [ _; _ ] -> Ok false
          | ws -> arity 2 ws) )
  ; ( "try-pattern"
    , M.Value_op
        (function
          | [ W.Pat pattern; W.V v; W.Env env ] ->
            (match Sicp_ch4.Sec_4_1.bind_pattern pattern v with
             | Some bindings -> Ok (W.Env (Env.extend bindings env))
             | None -> Ok W.Unassigned)
          | ws -> arity 3 ws) )
  ; ( "last-argument"
    , M.Value_op
        (function
          | [ W.Args vs ] ->
            (match List.rev vs with
             | v :: _ -> Ok (W.V v)
             | [] -> Ok (W.V Value.unit))
          | ws -> arity 1 ws) )
  ]
;;

let runtime_operations ~apply = compiler_operations @ W.operation_table ~apply

(* {1 Compile and go} *)

let load ?(operations = []) ~emit code =
  let global = Prelude.initial_env ~emit () in
  let knot =
    ref (fun () ->
      Error (Eval_error.Invalid_form "the compiled machine is not assembled"))
  in
  (* A primitive's guest callback runs on a fresh machine of the same
     code, entered at [compiled-apply] with [continue] at [done]. *)
  let apply proc vs =
    let* m = !knot () in
    let* () = M.set_register m "proc" (W.V proc) in
    let* () = M.set_register m "argl" (W.Args vs) in
    let* () = M.set_register m "continue" (W.Lab "done") in
    let* () = M.goto_label m "compiled-apply" in
    let* () = M.start m in
    let* w = M.get_register m "val" in
    match w with
    | W.V v -> Ok v
    | w -> bad ("a callback answered " ^ W.word_to_string w)
  in
  let assemble () =
    M.make
      ~words:W.words
      ~registers:compiled_registers
      ~operations:(operations @ runtime_operations ~apply)
      code.statements
  in
  knot := assemble;
  let* m = assemble () in
  let* () = M.set_register m "env" (W.Env global) in
  Ok m
;;

let execute ~emit program =
  let code = compile_program (new_state ()) (Check.items program) in
  let* m = load ~emit code in
  let* () = M.start m in
  let* w = M.get_register m "val" in
  let value =
    match w with
    | W.V v -> v
    | _ -> Value.unit
  in
  Ok (value, M.executed m)
;;

let run ~emit program = Result.map fst (execute ~emit program)
let run_stats ~emit program = execute ~emit program
