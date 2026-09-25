(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** The compiler of section 5.5: the book's [compile] and its code
    generators, emitting instruction sequences in the book's register-
    machine notation, and the 5.5.7 evaluator machine that runs
    compiled code beside interpreted code.

    An instruction sequence is the book's three parts: the registers
    the code needs, the registers it modifies, and the statements --
    controller lines in the book's notation, parsed by the 5.1 reader
    like any controller text.  [preserving] reads the recorded register
    use, never the code.

    Two spellings differ from the book's listings, both forced by the
    machine language the 5.1 reader defines (exercise 5.9 forbids
    labels as operation inputs; the 5.1 constant kinds have no lists
    and no strings): the book's [(label l)] operation inputs are
    spelled [(const l)]; list constants ride in a compile-time table
    behind one [(const compile-time-constant-N)] name, so the book's parameter list
    [(const (x))] becomes one [(const x)] input per parameter and the
    quotation [(const 'x)] becomes [(const compile-time-constant-N)] naming the datum. *)

module Ast = Sicp_common.Ast
module Value = Sicp_common.Value
module Env = Sicp_common.Env
module Reader = Sicp_common.Reader

let ( >>= ) = Result.bind

(** Every failure travels through [error]; the type is the 5.1
    substrate's, re-exported. *)
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
let eval_fail e = Op_failed (Sicp_common.Eval_error.to_string e)

(** {1:sequences Instruction sequences} *)

(** One instruction sequence: the book's three parts. *)
type seq =
  { needs : string list
  ; modifies : string list
  ; stmts : string list
  }

let make_instruction_sequence needs modifies stmts = { needs; modifies; stmts }
let empty_instruction_sequence = { needs = []; modifies = []; stmts = [] }

let list_union s1 s2 =
  let rec go acc = function
    | [] -> List.rev acc @ s2
    | x :: rest ->
      if List.mem x s2 || List.mem x acc then go acc rest else go (x :: acc) rest
  in
  go [] s1
;;

let rec list_difference s1 s2 =
  match s1 with
  | [] -> []
  | x :: rest ->
    if List.mem x s2 then list_difference rest s2 else x :: list_difference rest s2
;;

let append_2_sequences seq1 seq2 =
  { needs = list_union seq1.needs (list_difference seq2.needs seq1.modifies)
  ; modifies = list_union seq1.modifies seq2.modifies
  ; stmts = seq1.stmts @ seq2.stmts
  }
;;

let rec append_sequences = function
  | [] -> empty_instruction_sequence
  | [ seq ] -> seq
  | seq :: rest -> append_2_sequences seq (append_sequences rest)
;;

let tack_on_instruction_sequence seq body_seq =
  { seq with stmts = seq.stmts @ body_seq.stmts }
;;

(** The two branches after a test are never executed sequentially, so
    the combined sequence modifies what either branch modifies. *)
let parallel_instruction_sequences seq1 seq2 =
  { needs = list_union seq1.needs seq2.needs
  ; modifies = list_union seq1.modifies seq2.modifies
  ; stmts = seq1.stmts @ seq2.stmts
  }
;;

(** {1:config The compiler configuration} *)

(** The knobs the section's exercises turn: lexical addressing
    (5.40 to 5.42), scanning out internal definitions (5.43),
    open-coded primitives (5.38 and 5.44), the operand evaluation
    order (5.36), and the preserving mechanism itself (5.37). *)
type config =
  { lexical : bool
  ; scan_out : bool
  ; open_code : bool
  ; left_to_right : bool
  ; preserving_on : bool
  ; compound_calls : bool (** 5.47: compiled code may call interpreted procedures. *)
  ; trace : (string list list -> string -> unit) option
    (** 5.40: the compile-time environment dump -- every variable
          reference reports the environment it was compiled against. *)
  }

let default_config =
  { lexical = false
  ; scan_out = false
  ; open_code = false
  ; left_to_right = false
  ; preserving_on = true
  ; compound_calls = false
  ; trace = None
  }
;;

let open_coded_primitives = [ "+"; "-"; "*"; "<"; "=" ]
let stmt = Printf.sprintf

let result_map_all rs =
  let rec go acc = function
    | [] -> Ok (List.rev acc)
    | r :: rest ->
      (match r with
       | Ok x -> go (x :: acc) rest
       | Error e -> Error e)
  in
  go [] rs
;;

(** [preserving cfg regs seq1 seq2] appends with a [save]/[restore]
    around the first sequence of every register the first modifies and
    the second needs; the last register of the set is saved first, the
    book's own order.  With [preserving_on] off (the 5.37 comparison)
    every register in the set is saved unconditionally. *)
let preserving cfg regs seq1 seq2 =
  let rec go seq1 = function
    | [] -> Ok (append_2_sequences seq1 seq2)
    | first_reg :: rest ->
      let needed = List.mem first_reg seq2.needs && List.mem first_reg seq1.modifies in
      let saves = if cfg.preserving_on then needed else true in
      if not saves
      then go seq1 rest
      else (
        let wrapped =
          { needs = first_reg :: seq1.needs
          ; modifies = list_difference seq1.modifies [ first_reg ]
          ; stmts =
              (("(save " ^ first_reg ^ ")") :: seq1.stmts)
              @ [ "(restore " ^ first_reg ^ ")" ]
          }
        in
        go wrapped rest)
  in
  go seq1 regs
;;

(** {1:labels Labels and compile-time constants} *)

(** The compiler's state: the book's label counter and the compile-time
    constant table the emitted [(const compile-time-constant-N)] names point into. *)
type state =
  { counter : int ref
  ; consts : (string, Value.t) Hashtbl.t
  }

let new_state () = { counter = ref 0; consts = Hashtbl.create 16 }

(** [new_state_seeded n] starts the label counter at [n]: 5.35's
    reproduction of Figure 5.18 seeds 14, the labels the book's
    session had already generated. *)
let new_state_seeded n = { counter = ref n; consts = Hashtbl.create 16 }

let make_label state name =
  incr state.counter;
  name ^ string_of_int !(state.counter)
;;

let register_const state v =
  let name = "compile-time-constant-" ^ string_of_int (Hashtbl.length state.consts + 1) in
  Hashtbl.replace state.consts name v;
  name
;;

(** {1:linkage Targets and linkages} *)

type linkage =
  | Next
  | Return
  | Lab of string

let compile_linkage = function
  | Return -> Ok (make_instruction_sequence [ "continue" ] [] [ "(goto (reg continue))" ])
  | Next -> Ok empty_instruction_sequence
  | Lab l -> Ok (make_instruction_sequence [] [] [ "(goto (label " ^ l ^ "))" ])
;;

let end_with_linkage cfg linkage seq =
  compile_linkage linkage >>= fun tail -> preserving cfg [ "continue" ] seq tail
;;

(** {1:syntax Derived expressions over the typed AST} *)

(** [datum_value d] is the quoted datum [d] as an object value. *)
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

(** [cond_to_if exp] is the 4.1.2 transformation; a clause with no
    actions answers its test. *)
let cond_to_if exp =
  let open Ast in
  match view exp with
  | Cond (clauses, else_body) ->
    let base =
      match else_body with
      | Some body -> Result.value ~default:(bool false) (sequence body)
      | None -> bool false
    in
    let rec build = function
      | [] -> base
      | (test, actions) :: rest ->
        let alternative = build rest in
        let consequent =
          match actions with
          | [] -> test
          | acts -> Result.value ~default:test (sequence acts)
        in
        if_ test consequent (Some alternative)
    in
    Ok (build clauses)
  | _ -> Error (Op_failed "not a cond" : error)
;;

(** [let_to_combination exp] is [(let ((n e) ...) body)] as the call of
    a lambda on the inits. *)
let let_to_combination exp =
  match Ast.view exp with
  | Ast.Let (bindings, body) ->
    (match Ast.lambda (List.map fst bindings) body with
     | Ok fn -> Ok (Ast.application fn (List.map snd bindings))
     | Error e -> Error (eval_fail e))
  | _ -> Error (Op_failed "not a let")
;;

(** The compile-time environment: the frames of parameter names, newest
    first. *)
type cenv = string list list

let extend_cenv params (frames : cenv) : cenv = params :: frames

let find_variable var (frames : cenv) =
  let rec scan frame_number = function
    | [] -> None
    | names :: rest ->
      let rec displacement i = function
        | [] -> None
        | n :: _ when String.equal n var -> Some i
        | _ :: rest_names -> displacement (i + 1) rest_names
      in
      (match displacement 0 names with
       | Some d -> Some (frame_number, d)
       | None -> scan (frame_number + 1) rest)
  in
  scan 0 frames
;;

(** {1:compiler The code generators} *)

let rec compile cfg state cenv exp target linkage =
  match Ast.view exp with
  | Ast.Int _ | Ast.Float _ | Ast.Bool _ | Ast.String _ ->
    compile_self_evaluating cfg state exp target linkage
  | Ast.Quote _ -> compile_quoted cfg state exp target linkage
  | Ast.Variable _ -> compile_variable cfg cenv exp target linkage
  | Ast.Set _ -> compile_assignment cfg state cenv exp target linkage
  | Ast.Definition _ -> compile_definition cfg state cenv exp target linkage
  | Ast.If _ -> compile_if cfg state cenv exp target linkage
  | Ast.Lambda _ -> compile_lambda cfg state cenv exp target linkage
  | Ast.Sequence body -> compile_sequence cfg state cenv body target linkage
  | Ast.Cond _ ->
    (match cond_to_if exp with
     | Ok e -> compile cfg state cenv e target linkage
     | Error e -> Error e)
  | Ast.Let _ ->
    (match let_to_combination exp with
     | Ok e -> compile cfg state cenv e target linkage
     | Error e -> Error e)
  | Ast.Application (operator, operands) ->
    if is_open_coded cfg cenv operator
    then compile_open_code cfg state cenv exp target linkage
    else compile_application cfg state cenv operator operands target linkage
  | Ast.And _ | Ast.Or _ -> Error (Op_failed "Unknown expression type: COMPILE")

and is_open_coded cfg cenv operator =
  match Ast.view operator with
  | Ast.Variable name ->
    cfg.open_code
    && List.mem name open_coded_primitives
    && not (Option.is_some (find_variable name cenv))
  | _ -> false

and compile_self_evaluating cfg state exp target linkage =
  let spelling =
    match Ast.view exp with
    | Ast.Int n -> "const " ^ string_of_int n
    | Ast.Float f -> "const " ^ Value.to_string (Value.float f)
    | Ast.Bool b -> "const " ^ if b then "#t" else "#f"
    | Ast.String s -> "const " ^ register_const state (Value.string s)
    | _ -> assert false
  in
  end_with_linkage
    cfg
    linkage
    (make_instruction_sequence
       [ "env" ]
       [ target ]
       [ stmt "(assign %s (%s))" target spelling ])

and compile_quoted cfg state exp target linkage =
  match Ast.view exp with
  | Ast.Quote d ->
    let name = register_const state (datum_value d) in
    end_with_linkage
      cfg
      linkage
      (make_instruction_sequence
         [ "env" ]
         [ target ]
         [ stmt "(assign %s (op text-of-quotation) (const %s))" target name ])
  | _ -> Error (Op_failed "not a quotation")

and compile_variable cfg cenv exp target linkage =
  match Ast.view exp with
  | Ast.Variable name ->
    (match cfg.trace with
     | Some trace -> trace cenv name
     | None -> ());
    let access =
      if cfg.lexical
      then (
        match find_variable name cenv with
        | Some (frame, displacement) ->
          [ stmt
              "(assign %s (op lexical-address-lookup) (const %d) (const %d) (reg env))"
              target
              frame
              displacement
          ]
        | None ->
          [ stmt "(assign %s (op lookup-variable-value) (const %s) (reg env))" target name
          ])
      else
        [ stmt "(assign %s (op lookup-variable-value) (const %s) (reg env))" target name ]
    in
    end_with_linkage cfg linkage (make_instruction_sequence [ "env" ] [ target ] access)
  | _ -> Error (Op_failed "compile-variable needs a variable")

and compile_assignment cfg state cenv exp target linkage =
  match Ast.view exp with
  | Ast.Set (name, value) ->
    compile cfg state cenv value "val" Next
    >>= fun value_code ->
    preserving
      cfg
      [ "env" ]
      value_code
      (make_instruction_sequence
         [ "env"; "val" ]
         [ target ]
         [ stmt "(perform (op set-variable-value!) (const %s) (reg val) (reg env))" name
         ; stmt "(assign %s (const ok))" target
         ])
    >>= fun seq -> end_with_linkage cfg linkage seq
  | _ -> Error (Op_failed "compile-assignment needs an assignment")

and definition_parts exp =
  match Ast.view exp with
  | Ast.Definition d ->
    (match Ast.view_definition d with
     | Ast.Define_variable (name, value) -> Ok (name, value)
     | Ast.Define_function { name; parameters; body } ->
       (match Ast.lambda parameters body with
        | Ok lam -> Ok (name, lam)
        | Error e -> Error (eval_fail e)))
  | _ -> Error (Op_failed "compile-definition needs a definition")

and compile_definition cfg state cenv exp target linkage =
  definition_parts exp
  >>= fun (name, value) ->
  compile cfg state cenv value "val" Next
  >>= fun value_code ->
  preserving
    cfg
    [ "env" ]
    value_code
    (make_instruction_sequence
       [ "env" ]
       [ target ]
       [ stmt "(perform (op define-variable!) (const %s) (reg val) (reg env))" name
       ; stmt "(assign %s (const ok))" target
       ])
  >>= fun seq -> end_with_linkage cfg linkage seq

and compile_if cfg state cenv exp target linkage =
  match Ast.view exp with
  | Ast.If (predicate, consequent, alternative) ->
    (* The label allocations and the compilation order (alternative,
       consequent, predicate) are the orders the book's own figures
       show. *)
    let after_if = make_label state "after-if" in
    let f_branch = make_label state "false-branch" in
    let t_branch = make_label state "true-branch" in
    let consequent_linkage =
      match linkage with
      | Next -> Lab after_if
      | l -> l
    in
    let alternative_exp = Option.value alternative ~default:(Ast.bool false) in
    compile cfg state cenv alternative_exp target linkage
    >>= fun a_code ->
    compile cfg state cenv consequent target consequent_linkage
    >>= fun c_code ->
    compile cfg state cenv predicate "val" Next
    >>= fun p_code ->
    let test_code =
      make_instruction_sequence
        [ "val" ]
        []
        [ "(test (op false?) (reg val))"; stmt "(branch (label %s))" f_branch ]
    in
    let branches =
      parallel_instruction_sequences
        (append_2_sequences (make_instruction_sequence [] [] [ t_branch ]) c_code)
        (append_2_sequences (make_instruction_sequence [] [] [ f_branch ]) a_code)
    in
    let branch_tail = { branches with stmts = branches.stmts @ [ after_if ] } in
    let with_test = append_2_sequences test_code branch_tail in
    preserving cfg [ "env"; "continue" ] p_code with_test
  | _ -> Error (Op_failed "compile-if needs an if")

and scan_out_defines body =
  (* 5.43: the internal definitions become a let of *unassigned*
     bindings whose values are set after it, the book's scan-out. *)
  let is_define e =
    match Ast.view e with
    | Ast.Definition _ -> true
    | _ -> false
  in
  let defines, rest = List.partition is_define body in
  match defines with
  | [] -> body
  | first_define :: _ ->
    ignore first_define;
    let open Ast in
    let part d =
      match Ast.view d with
      | Ast.Definition def ->
        (match Ast.view_definition def with
         | Ast.Define_variable (n, v) -> n, Some v
         | Ast.Define_function { name; parameters; body = b } ->
           (match lambda parameters b with
            | Ok lam -> name, Some lam
            | Error _ -> name, None))
      | _ -> "", None
    in
    let parts = List.map part defines in
    let names = List.map fst parts in
    let unassigned = variable "*unassigned*" in
    let bindings = List.map (fun n -> n, unassigned) names in
    let sets = List.filter_map (fun (n, v) -> Option.map (set n) v) parts in
    (match let_ bindings (sets @ rest) with
     | Ok derived -> [ derived ]
     | Error _ -> sets @ rest)

and compile_lambda_body cfg state cenv exp proc_entry =
  match Ast.view exp with
  | Ast.Lambda (params, body) ->
    let body = if cfg.scan_out then scan_out_defines body else body in
    let name_consts =
      String.concat " " (List.map (fun p -> "(const " ^ p ^ ")") params)
    in
    let extend_stmt =
      if params = []
      then "(assign env (op extend-environment) (reg argl) (reg env))"
      else stmt "(assign env (op extend-environment) %s (reg argl) (reg env))" name_consts
    in
    compile_sequence cfg state (extend_cenv params cenv) body "val" Return
    >>= fun body_code ->
    Ok
      (append_2_sequences
         (make_instruction_sequence
            [ "env"; "proc"; "argl" ]
            [ "env" ]
            [ proc_entry
            ; "(assign env (op compiled-procedure-env) (reg proc))"
            ; extend_stmt
            ])
         body_code)
  | _ -> Error (Op_failed "compile-lambda-body needs a lambda")

and compile_lambda cfg state cenv exp target linkage =
  match Ast.view exp with
  | Ast.Lambda _ ->
    let after_lambda = make_label state "after-lambda" in
    let proc_entry = make_label state "entry" in
    let lambda_linkage =
      match linkage with
      | Next -> Lab after_lambda
      | l -> l
    in
    let construct =
      make_instruction_sequence
        [ "env" ]
        [ target ]
        [ stmt
            "(assign %s (op make-compiled-procedure) (const %s) (reg env))"
            target
            proc_entry
        ]
    in
    compile_lambda_body cfg state cenv exp proc_entry
    >>= fun body ->
    end_with_linkage cfg lambda_linkage construct
    >>= fun seq ->
    Ok
      (append_2_sequences
         (tack_on_instruction_sequence seq body)
         (make_instruction_sequence [] [] [ after_lambda ]))
  | _ -> Error (Op_failed "compile-lambda needs a lambda")

and compile_sequence cfg state cenv body target linkage =
  match body with
  | [] -> Error (Op_failed "compile-sequence of an empty body")
  | [ last ] -> compile cfg state cenv last target linkage
  | first :: rest ->
    compile cfg state cenv first target Next
    >>= fun first_code ->
    compile_sequence cfg state cenv rest target linkage
    >>= fun rest_code -> preserving cfg [ "env"; "continue" ] first_code rest_code

and construct_arglist cfg operand_codes =
  let ordered = if cfg.left_to_right then operand_codes else List.rev operand_codes in
  match ordered with
  | [] ->
    Ok (make_instruction_sequence [] [ "argl" ] [ "(assign argl (op empty-arglist))" ])
  | last_code :: rest_codes ->
    let code_to_get_last_arg =
      append_2_sequences
        last_code
        (make_instruction_sequence
           [ "val" ]
           [ "argl" ]
           [ "(assign argl (op list) (reg val))" ])
    in
    let cons_step =
      make_instruction_sequence
        [ "val"; "argl" ]
        [ "argl" ]
        [ "(assign argl (op cons) (reg val) (reg argl))" ]
    in
    let rec code_to_get_rest_args = function
      | [] -> Ok empty_instruction_sequence
      | next :: rest ->
        preserving cfg [ "argl" ] next cons_step
        >>= fun code_for_next_arg ->
        if rest = []
        then Ok code_for_next_arg
        else
          code_to_get_rest_args rest
          >>= fun rest_code -> preserving cfg [ "env" ] code_for_next_arg rest_code
    in
    if rest_codes = []
    then Ok code_to_get_last_arg
    else
      code_to_get_rest_args rest_codes
      >>= fun rest_code -> preserving cfg [ "env" ] code_to_get_last_arg rest_code

and compile_application cfg state cenv operator operands target linkage =
  compile cfg state cenv operator "proc" Next
  >>= fun proc_code ->
  result_map_all (List.map (fun o -> compile cfg state cenv o "val" Next) operands)
  >>= fun codes ->
  construct_arglist cfg codes
  >>= fun arglist_code ->
  compile_procedure_call cfg state target linkage
  >>= fun call_code ->
  preserving cfg [ "proc"; "continue" ] arglist_code call_code
  >>= fun call_and_args -> preserving cfg [ "env"; "continue" ] proc_code call_and_args

and compile_procedure_call cfg state target linkage =
  let after_call = make_label state "after-call" in
  let compiled_branch = make_label state "compiled-branch" in
  let primitive_branch = make_label state "primitive-branch" in
  let compound_branch =
    if cfg.compound_calls then Some (make_label state "compound-branch") else None
  in
  let compiled_linkage =
    match linkage with
    | Next -> Lab after_call
    | l -> l
  in
  compile_proc_appl state target compiled_linkage
  >>= fun appl_code ->
  let primitive_tail =
    match compound_branch, linkage with
    | Some _, Next -> [ stmt "(goto (label %s))" after_call ]
    | _ -> []
  in
  end_with_linkage
    cfg
    linkage
    (make_instruction_sequence
       [ "proc"; "argl" ]
       [ target ]
       (stmt "(assign %s (op apply-primitive-procedure) (reg proc) (reg argl))" target
        :: primitive_tail))
  >>= fun primitive_code ->
  (* 5.47: a third branch hands interpreted procedures to the
     evaluator's compound-apply through the [unev] register, which is
     dead at the call site (the substrate's register set is the 5.4
     machine's, so the book's [compapp] register has no name here). *)
  let compound_test, compound_label =
    match compound_branch with
    | Some cb ->
      let cont_setup =
        match linkage with
        | Return -> []
        | Next -> [ stmt "(assign continue (label %s))" after_call ]
        | Lab l -> [ stmt "(assign continue (label %s))" l ]
      in
      ( [ make_instruction_sequence
            [ "proc" ]
            []
            [ "(test (op compound-procedure?) (reg proc))"
            ; stmt "(branch (label %s))" cb
            ]
        ]
      , [ make_instruction_sequence [] [] [ cb ]
        ; make_instruction_sequence
            [ "proc" ]
            [ "unev"; "continue" ]
            (cont_setup
             @ [ "(save continue)"
                 (* the interpreted compound-apply reaches its body
                      through ev-sequence, whose last-expression path
                      restores [continue] from the stack: the return
                      address rides the stack, the interpreted
                      calling convention, not the compiled register *)
               ; "(assign unev (label compound-apply))"
               ; "(goto (reg unev))"
               ])
        ] )
    | None -> [], []
  in
  Ok
    (append_sequences
       ([ make_instruction_sequence
            [ "proc" ]
            []
            [ "(test (op primitive-procedure?) (reg proc))"
            ; stmt "(branch (label %s))" primitive_branch
            ]
        ]
        @ compound_test
        @ [ parallel_instruction_sequences
              (append_2_sequences
                 (make_instruction_sequence [] [] [ compiled_branch ])
                 appl_code)
              (append_2_sequences
                 (make_instruction_sequence [] [] [ primitive_branch ])
                 primitive_code)
          ]
        @ compound_label
        @ [ make_instruction_sequence [] [] [ after_call ] ]))

and compile_proc_appl state target linkage =
  let all_regs = [ "env"; "proc"; "val"; "argl"; "continue" ] in
  match target, linkage with
  | "val", Return ->
    Ok
      (make_instruction_sequence
         [ "proc"; "continue" ]
         all_regs
         [ "(assign val (op compiled-procedure-entry) (reg proc))"; "(goto (reg val))" ])
  | "val", Lab l ->
    Ok
      (make_instruction_sequence
         [ "proc" ]
         all_regs
         [ stmt "(assign continue (label %s))" l
         ; "(assign val (op compiled-procedure-entry) (reg proc))"
         ; "(goto (reg val))"
         ])
  | _, Return -> Error (Op_failed "return linkage, target not val: COMPILE")
  | _, _ ->
    let proc_return = make_label state "proc-return" in
    let goto_label =
      match linkage with
      | Lab l -> l
      | _ -> proc_return
    in
    Ok
      (make_instruction_sequence
         [ "proc" ]
         all_regs
         [ stmt "(assign continue (label %s))" proc_return
         ; "(assign val (op compiled-procedure-entry) (reg proc))"
         ; "(goto (reg val))"
         ; proc_return
         ; stmt "(assign %s (reg val))" target
         ; stmt "(goto (label %s))" goto_label
         ])

and spread_arguments cfg state cenv operands targets =
  (* 5.38(a): operands evaluated into successive argument registers,
     with the registers still to come preserved around each evaluation,
     because an operand may itself be an open-coded call; the
     environment is preserved with them, because a nested call operand
     rebinds it and a later operand (a variable reference) reads the
     caller's frame. *)
  match operands, targets with
  | [], _ -> Ok empty_instruction_sequence
  | operand :: rest, target :: rest_targets ->
    compile cfg state cenv operand target Next
    >>= fun code ->
    if rest = []
    then Ok code
    else
      spread_arguments cfg state cenv rest rest_targets
      >>= fun rest_code -> preserving cfg (rest_targets @ [ "env" ]) code rest_code
  | _ -> Error (Op_failed "spread-arguments ran out of argument registers")

and compile_open_code cfg state cenv exp target linkage =
  match Ast.view exp with
  | Ast.Application (operator, operands) ->
    (match Ast.view operator with
     | Ast.Variable name when List.mem name open_coded_primitives ->
       if List.length operands > 2 && (name = "+" || name = "*")
       then compile_open_code_nary cfg state cenv name operands target linkage
       else if List.length operands <> 2
       then Error (Op_failed ("open coding needs two operands for " ^ name))
       else
         spread_arguments cfg state cenv operands [ "arg1"; "arg2" ]
         >>= fun spread ->
         end_with_linkage
           cfg
           linkage
           (append_2_sequences
              spread
              (make_instruction_sequence
                 [ "arg1"; "arg2" ]
                 [ target ]
                 [ stmt "(assign %s (op %s) (reg arg1) (reg arg2))" target name ]))
     | _ -> Error (Op_failed "not an open-coded primitive"))
  | _ -> Error (Op_failed "compile needs an application")

and compile_open_code_nary cfg state cenv name operands _target linkage =
  (* 5.38(d): more than two operands fold through one register: each
     operand is evaluated into [arg1] and folded into [val].  The
     environment is preserved around an evaluation whose tail reads it
     (a later operand may be a call that rebinds [env]); [arg1] itself
     is never preserved around its own evaluation, it is the
     evaluation's output. *)
  let open_step =
    make_instruction_sequence
      [ "arg1"; "val" ]
      [ "val" ]
      [ stmt "(assign val (op %s) (reg arg1) (reg val))" name ]
  in
  let rec go = function
    | [] -> Ok empty_instruction_sequence
    | operand :: rest ->
      compile cfg state cenv operand "arg1" Next
      >>= fun code ->
      go rest
      >>= fun rest_code ->
      preserving cfg [ "env" ] code (append_2_sequences open_step rest_code)
  in
  match operands with
  | first :: second :: rest ->
    compile cfg state cenv first "arg1" Next
    >>= fun c1 ->
    compile cfg state cenv second "arg2" Next
    >>= fun c2 ->
    (* the second operand's evaluation may clobber [arg1] internally
       (an open-coded operand), so the first operand's result is
       shielded across it *)
    let c2_shielded =
      if List.mem "arg1" c2.modifies
      then
        { c2 with
          stmts = [ "(save arg1)" ] @ c2.stmts @ [ "(restore arg1)" ]
        ; needs = "arg1" :: c2.needs
        }
      else c2
    in
    let first_two =
      append_2_sequences
        c1
        (append_2_sequences
           c2_shielded
           (make_instruction_sequence
              [ "arg1"; "arg2" ]
              [ "val" ]
              [ stmt "(assign val (op %s) (reg arg1) (reg arg2))" name ]))
    in
    go rest
    >>= fun rest_code ->
    end_with_linkage cfg linkage (append_2_sequences first_two rest_code)
  | _ -> Error (Op_failed "open coding needs operands")
;;

let compile_program ?(cfg = default_config) ?(linkage = Next) state forms =
  let rec go = function
    | [] -> Ok empty_instruction_sequence
    | [ single ] -> compile cfg state [] single "val" linkage
    | first :: rest ->
      compile cfg state [] first "val" Next
      >>= fun first_code ->
      go rest
      >>= fun rest_code -> preserving cfg [ "env"; "continue" ] first_code rest_code
  in
  go forms
;;

(** [registered_constants state] is the compile-time constants in
    registration order: the C backend's data table. *)
let registered_constants state =
  let n = Hashtbl.length state.consts in
  List.init n (fun i ->
    let name = "compile-time-constant-" ^ string_of_int (i + 1) in
    name, Hashtbl.find state.consts name)
;;

(** [statements_text seq] is the sequence's statements, one controller
    line each. *)
let statements_text seq = String.concat "\n" seq.stmts

(** {1:machine The evaluator machine of 5.5.7} *)

(** One machine operation: the 5.4 op type, re-exported. *)
type op = Sec_5_4.op =
  | Value_op of (Sec_5_4.word list -> (Sec_5_4.word, error) result)
  | Action_op of (Sec_5_4.word list -> (unit, error) result)

(** One compiled procedure object: the book's [(compiled-procedure
    entry env)] list, carried as a value tagged [compiled-procedure]
    whose second element is the procedure's table index; the entry name
    and the environment word live in the machine's table, because an
    environment word cannot sit inside an object value. *)
type compiled_table = (int, string * Sec_5_4.word) Hashtbl.t

let is_compiled_object v =
  match Value.view v with
  | Value.Pair (tag, second) ->
    (match Value.view tag, Value.view second with
     | Value.Symbol s, Value.Int _ -> String.equal s "compiled-procedure"
     | _ -> false)
  | _ -> false
;;

let compiled_parts table v =
  match Value.view v with
  | Value.Pair (_, second) ->
    (match Value.view second with
     | Value.Int id ->
       (match Hashtbl.find_opt table id with
        | Some (entry, env) -> Ok (entry, env)
        | None -> Error (Op_failed "the compiled procedure's entry is gone"))
     | _ -> Error (Op_failed "not a compiled procedure"))
  | _ -> Error (Op_failed "not a compiled procedure")
;;

let const_name_word = function
  | Sec_5_4.V v ->
    (match Value.view v with
     | Value.Symbol s -> Ok s
     | _ -> Error (Op_failed "the compiled procedure entry is spelled (const name)"))
  | w -> Error (Op_failed ("expected a name constant, found " ^ Sec_5_4.word_to_string w))
;;

(** The compiled machine's operations over the 5.4 word type: the
    footnote-323 compiled-procedure family, the argument-list builders
    compiled code uses, the book's [false?], the constant-name inputs
    the compiler bakes, and the word arithmetic the open-coded calls of
    5.38 run on (the machine carries [arg1] and [arg2] for them). *)
let compiled_operations state table =
  let constant_of = function
    | [ Sec_5_4.V v ] ->
      (match Value.view v with
       | Value.Symbol name ->
         (match Hashtbl.find_opt state.consts name with
          | Some v -> Ok (Sec_5_4.V v)
          | None -> Error (Op_failed ("no compile-time constant " ^ name)))
       | _ ->
         Error (Op_failed "the constant input is spelled (const compile-time-constant-N)"))
    | ws -> Error (Arity (string_of_int (List.length ws) ^ " inputs for a constant"))
  in
  let word_arith name =
    ( name
    , Sec_5_4.Value_op
        (function
          | [ Sec_5_4.V a; Sec_5_4.V b ] ->
            Sec_5_4.apply_object_primitive name [ a; b ]
            |> Result.map (fun v -> Sec_5_4.V v)
          | _ -> Error (Arity (name ^ " needs two values"))) )
  in
  [ ( "list"
    , Sec_5_4.Value_op
        (function
          | [ w ] -> Ok (Sec_5_4.Args [ w ])
          | _ -> Error (Arity "list needs one argument")) )
  ; ( "cons"
    , Sec_5_4.Value_op
        (function
          | [ w; Sec_5_4.Args ws ] -> Ok (Sec_5_4.Args (w :: ws))
          | _ -> Error (Arity "cons needs a word and an operand list")) )
  ; ( "false?"
    , Sec_5_4.Value_op
        (function
          | [ Sec_5_4.V v ] ->
            Ok
              (Sec_5_4.V
                 (Value.bool
                    (match Value.view v with
                     | Value.Bool false -> true
                     | _ -> false)))
          | _ -> Error (Arity "false? needs a value")) )
  ; "compile-time-constant", Sec_5_4.Value_op constant_of
  ; ( "text-of-quotation"
    , Sec_5_4.Value_op
        (function
          | [ Sec_5_4.Exp e ] ->
            (match Ast.view e with
             | Ast.Quote d -> Ok (Sec_5_4.V (datum_value d))
             | _ -> Error (Op_failed "not a quotation"))
          | [ (Sec_5_4.V _ as w) ] -> constant_of [ w ]
          | _ -> Error (Arity "text-of-quotation needs one input")) )
  ; ( "make-compiled-procedure"
    , Sec_5_4.Value_op
        (function
          | [ V v; Sec_5_4.Env env ] ->
            const_name_word (V v)
            >>= fun entry ->
            let id = Hashtbl.length table in
            Hashtbl.replace table id (entry, Sec_5_4.Env env);
            Ok (Sec_5_4.V (Value.pair (Value.symbol "compiled-procedure") (Value.int id)))
          | _ ->
            Error (Arity "make-compiled-procedure needs an entry name and an environment"))
    )
  ; ( "compiled-procedure?"
    , Sec_5_4.Value_op
        (function
          | [ V v ] -> Ok (Sec_5_4.V (Value.bool (is_compiled_object v)))
          | _ -> Error (Arity "compiled-procedure? needs a procedure")) )
  ; ( "compiled-procedure-entry"
    , Sec_5_4.Value_op
        (function
          | [ V v ] ->
            compiled_parts table v |> Result.map (fun (entry, _) -> Sec_5_4.Lab entry)
          | _ -> Error (Arity "compiled-procedure-entry needs a compiled procedure")) )
  ; ( "compiled-procedure-env"
    , Sec_5_4.Value_op
        (function
          | [ V v ] -> compiled_parts table v |> Result.map (fun (_, env) -> env)
          | _ -> Error (Arity "compiled-procedure-env needs a compiled procedure")) )
  ; word_arith "+"
  ; word_arith "-"
  ; word_arith "*"
  ; word_arith "<"
  ; word_arith "="
  ]
;;

(** The compiled machine's [extend-environment]: the interpreted path
    of compound-apply passes the evaluator's parameters word first,
    compiled code passes the parameter names as leading [(const name)]
    inputs; both spellings end in the operand list and the environment,
    so the operation splits its inputs from the back. *)
let extend_environment_compiled =
  Sec_5_4.Value_op
    (fun words ->
      let extend resolved args env =
        Sec_5_4.word_values args
        >>= fun values ->
        match Env.extend resolved values env with
        | Ok e -> Ok (Sec_5_4.Env e)
        | Error e -> Error (eval_fail e)
      in
      let rec split names = function
        | [ Sec_5_4.Args args; Sec_5_4.Env env ] ->
          Sec_5_4.parameters_of (Sec_5_4.Seq (List.map Ast.variable (List.rev names)))
          >>= fun resolved -> extend resolved args env
        | Sec_5_4.V v :: rest ->
          (match Value.view v with
           | Value.Symbol s -> split (s :: names) rest
           | _ -> Error (Op_failed "a parameter name constant is a symbol"))
        | [ Sec_5_4.Seq params; Sec_5_4.Args args; Sec_5_4.Env env ] ->
          (* the interpreted path of compound-apply: the evaluator's
               own parameters word, not baked name constants *)
          Sec_5_4.parameters_of (Sec_5_4.Seq params)
          >>= fun resolved -> extend resolved args env
        | _ ->
          Error
            (Op_failed
               "extend-environment needs names, an operand list, and an environment")
      in
      split [] words)
;;

(** {1:runtime The runtime primitives} *)

(** [build_runtime_primitives ()] is the object-language primitive
    table of a compiled machine: the 5.4 set, rewritten here so the
    machine can apply it by name (an abstract procedure value exposes
    only its name), plus the names the adapted metacircular evaluator
    of 4.1 binds.  The environment procedures are provided by the
    machine: the edition's environments are machine environments the
    object world reaches through id symbols, so the book's
    list-structure frames, which need mutable pairs, are not
    reproduced.  One call per machine; the environment id table is that
    machine's own state. *)
let build_runtime_primitives () =
  let table : (string * Value.primitive) list ref = ref [] in
  let apply_by_name name args =
    match List.assoc_opt name !table with
    | Some f -> f args
    | None -> Error (Sicp_common.Eval_error.Type_error ("no primitive " ^ name))
  in
  let arity expected given = Sicp_common.Eval_error.Arity_mismatch { expected; given } in
  let prim1 f = function
    | [ a ] -> f a
    | args -> Error (arity 1 (List.length args))
  in
  let prim2 f = function
    | [ a; b ] -> f a b
    | args -> Error (arity 2 (List.length args))
  in
  let type_error what v =
    Sicp_common.Eval_error.Type_error (what ^ ": " ^ Value.to_string v)
  in
  let number_of what v =
    match Value.view v with
    | Value.Int n -> Ok n
    | Value.Float f -> Ok (int_of_float f)
    | _ -> Error (type_error (what ^ " needs a number") v)
  in
  let arith name f = function
    | first :: second :: _ ->
      number_of name first
      >>= fun a -> number_of name second >>= fun b -> Ok (Value.int (f a b))
    | args -> Error (arity 2 (List.length args))
  in
  let compare_with test =
    prim2 (fun a b ->
      number_of "comparison" a
      >>= fun x -> number_of "comparison" b >>= fun y -> Ok (Value.bool (test x y)))
  in
  let list_values what v =
    let rec go acc = function
      | v ->
        (match Value.view v with
         | Value.Nil -> Ok (List.rev acc)
         | Value.Pair (a, d) -> go (a :: acc) d
         | _ -> Error (type_error (what ^ " needs a list") v))
    in
    go [] v
  in
  let symbols_of what v =
    list_values what v
    >>= fun vs ->
    result_map_all
      (List.map
         (fun v ->
            match Value.view v with
            | Value.Symbol s -> Ok s
            | _ -> Error (type_error (what ^ " needs symbols") v))
         vs)
  in
  let car_v v =
    match Value.view v with
    | Value.Pair (a, _) -> Ok a
    | _ -> Error (type_error "car needs a pair" v)
  in
  let cdr_v v =
    match Value.view v with
    | Value.Pair (_, d) -> Ok d
    | _ -> Error (type_error "cdr needs a pair" v)
  in
  let cxr path = prim1 (fun v -> List.fold_left (fun acc f -> acc >>= f) (Ok v) path) in
  let cadr = cxr [ cdr_v; car_v ] in
  let caddr = cxr [ cdr_v; cdr_v; car_v ] in
  let cadddr = cxr [ cdr_v; cdr_v; cdr_v; car_v ] in
  let caadr = cxr [ cdr_v; car_v; car_v ] in
  let cdadr = cxr [ cdr_v; car_v; cdr_v ] in
  let cddr = cxr [ cdr_v; cdr_v ] in
  let cdddr = cxr [ cdr_v; cdr_v; cdr_v ] in
  let envs : (string, Value.env) Hashtbl.t = Hashtbl.create 8 in
  let () = Hashtbl.replace envs "the-empty" (Env.empty ()) in
  let env_of name =
    match Hashtbl.find_opt envs name with
    | Some e -> Ok e
    | None -> Error (Sicp_common.Eval_error.Type_error ("no environment " ^ name))
  in
  let fresh_id () = string_of_int (Hashtbl.length envs + 1) in
  let data : (string * Value.primitive) list =
    [ "cons", prim2 (fun a b -> Ok (Value.pair a b))
    ; ( "car"
      , prim1 (fun v ->
          match Value.view v with
          | Value.Pair (a, _) -> Ok a
          | _ -> Error (type_error "car needs a pair" v)) )
    ; ( "cdr"
      , prim1 (fun v ->
          match Value.view v with
          | Value.Pair (_, d) -> Ok d
          | _ -> Error (type_error "cdr needs a pair" v)) )
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
    ; ("list", fun args -> Ok (List.fold_right Value.pair args Value.nil))
    ; "cadr", cadr
    ; "caddr", caddr
    ; "cadddr", cadddr
    ; "caadr", caadr
    ; "cdadr", cdadr
    ; "cddr", cddr
    ; "cdddr", cdddr
    ; "+", arith "+" ( + )
    ; "-", arith "-" ( - )
    ; "*", arith "*" ( * )
    ; ( "/"
      , prim2 (fun a b ->
          number_of "/" a
          >>= fun x ->
          number_of "/" b
          >>= fun y ->
          if y = 0
          then Error Sicp_common.Eval_error.Division_by_zero
          else Ok (Value.int (x / y))) )
    ; "=", compare_with ( = )
    ; "<", compare_with ( < )
    ; ">", compare_with ( > )
    ; "<=", compare_with ( <= )
    ; ">=", compare_with ( >= )
    ; ( "remainder"
      , prim2 (fun a b ->
          number_of "remainder" a
          >>= fun x ->
          number_of "remainder" b
          >>= fun y ->
          if y = 0
          then Error Sicp_common.Eval_error.Division_by_zero
          else Ok (Value.int (x mod y))) )
    ; ( "quotient"
      , prim2 (fun a b ->
          number_of "quotient" a
          >>= fun x ->
          number_of "quotient" b
          >>= fun y ->
          if y = 0
          then Error Sicp_common.Eval_error.Division_by_zero
          else Ok (Value.int (x / y))) )
    ; "abs", prim1 (fun v -> number_of "abs" v >>= fun n -> Ok (Value.int (abs n)))
    ; ( "error"
      , fun args ->
          Error
            (Sicp_common.Eval_error.Type_error
               ("error: " ^ String.concat " " (List.map Value.to_string args))) )
    ; "display", prim1 (fun v -> Ok v)
    ; ( "newline"
      , function
        | [] -> Ok (Value.symbol "newline")
        | args -> Error (arity 0 (List.length args)) )
    ]
  in
  let environment_support : (string * Value.primitive) list =
    [ ( "extend-environment"
      , fun args ->
          match args with
          | [ vars; vals; v ] ->
            (match Value.view v with
             | Value.Symbol base ->
               env_of base
               >>= fun base_env ->
               symbols_of "extend-environment" vars
               >>= fun names ->
               list_values "extend-environment" vals
               >>= fun values ->
               (match Env.extend names values base_env with
                | Ok extended ->
                  let id = fresh_id () in
                  Hashtbl.replace envs id extended;
                  Ok (Value.symbol id)
                | Error e -> Error e)
             | _ -> Error (type_error "extend-environment needs an environment id" v))
          | args -> Error (arity 3 (List.length args)) )
    ; ( "lookup-variable-value"
      , fun args ->
          match args with
          | [ v; e ] ->
            (match Value.view v, Value.view e with
             | Value.Symbol name, Value.Symbol id ->
               env_of id
               >>= fun env ->
               (match Env.find_binding env name with
                | Some value -> Ok value
                | None ->
                  Error (Sicp_common.Eval_error.Type_error ("unbound variable: " ^ name)))
             | _ -> Error (type_error "lookup-variable-value needs a name and an id" v))
          | args -> Error (arity 2 (List.length args)) )
    ; ( "set-variable-value!"
      , fun args ->
          match args with
          | [ v; value; e ] ->
            (match Value.view v, Value.view e with
             | Value.Symbol name, Value.Symbol id ->
               env_of id
               >>= fun env -> Env.set env name value >>= fun () -> Ok (Value.symbol "ok")
             | _ -> Error (type_error "set-variable-value! needs a name and an id" v))
          | args -> Error (arity 3 (List.length args)) )
    ; ( "define-variable!"
      , fun args ->
          match args with
          | [ v; value; e ] ->
            (match Value.view v, Value.view e with
             | Value.Symbol name, Value.Symbol id ->
               env_of id
               >>= fun env -> Ok (Env.define env name value |> fun () -> Value.symbol "ok")
             | _ -> Error (type_error "define-variable! needs a name and an id" v))
          | args -> Error (arity 3 (List.length args)) )
    ; ( "apply-in-underlying-scheme"
      , fun args ->
          match args with
          | [ impl; values ] ->
            (match Value.view impl with
             | Value.Primitive_procedure name ->
               list_values "apply" values >>= fun xs -> apply_by_name name xs
             | _ -> Error (type_error "apply-in-underlying-scheme needs a primitive" impl))
          | args -> Error (arity 2 (List.length args)) )
    ]
  in
  let all = data @ environment_support in
  table := all;
  all
;;

(** {1:builder Building the compiled evaluator} *)

(** The machine: the 5.4 word registers plus the open-coding argument
    registers, the 5.2.4 monitored stack, the transcript, the input
    queue, and the executed-instruction counter 5.33a measures. *)
type machine =
  { regs : (string, Sec_5_4.word ref) Hashtbl.t
  ; operations : (string, Sec_5_4.op) Hashtbl.t
  ; program : Sec_5_1.program
  ; pc : int ref
  ; the_stack : Sec_5_4.word list ref
  ; pushes : int ref
  ; depth : int ref
  ; max_depth : int ref
  ; output : string list ref
  ; input_queue : Ast.expr list ref
  ; steps : int ref
  ; rt_primitives : (string * Value.primitive) list
  ; label_table : (string, int) Hashtbl.t
    (** The labels as a table: the compiled evaluator's controllers
          carry thousands of labels, and a list walk per jump is the
          machine's whole cost. *)
  }

(** The registers of the 5.5.7 machine description: the evaluator's
    eight plus [arg1] and [arg2] of 5.38. *)
let machine_registers = Sec_5_4.evaluator_registers @ [ "arg1"; "arg2" ]

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
      Ok (Sec_5_4.V (value v))
    | Sec_5_1.Label_source l -> Ok (Sec_5_4.Lab l)
  in
  result_map_all (List.map one sources)
;;

let apply_value m name args =
  match Hashtbl.find_opt m.operations name with
  | Some (Sec_5_4.Value_op f) -> f args
  | Some (Sec_5_4.Action_op _) ->
    Error
      (Bad_instruction ("the operation " ^ name ^ " is an action and produces no value"))
  | None -> Error (Unknown_operation name)
;;

let apply_action m name args =
  match Hashtbl.find_opt m.operations name with
  | Some (Sec_5_4.Action_op f) -> f args
  | Some (Sec_5_4.Value_op _) ->
    Error
      (Bad_instruction
         ("the operation " ^ name ^ " produces a value; assign it, do not perform it"))
  | None -> Error (Unknown_operation name)
;;

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

let jump_to m l =
  match Hashtbl.find_opt m.label_table l with
  | Some i ->
    m.pc := i;
    Ok ()
  | None -> Error (Unknown_label l)
;;

let is_true = function
  | Sec_5_4.V v ->
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
     | _ ->
       inputs m [ src ]
       >>= (function
        | [ w ] -> store m r w
        | _ -> Error (Bad_instruction "an assign takes one source")))
  | Assign_op (r, name, srcs) -> inputs m srcs >>= apply_value m name >>= store m r
  | Test (name, srcs) ->
    inputs m srcs
    >>= apply_value m name
    >>= fun w -> store m "flag" (Sec_5_4.V (Value.bool (is_true w)))
  | Branch l -> lookup m "flag" >>= fun w -> if is_true w then jump_to m l else Ok ()
  | Goto_label l -> jump_to m l
  | Goto_reg r ->
    lookup m r
    >>= (function
     | Sec_5_4.Lab l -> jump_to m l
     | w ->
       Error (Op_failed ("the goto register " ^ r ^ " holds " ^ Sec_5_4.word_to_string w)))
  | Save r -> lookup m r >>= push_stack m
  | Restore r -> pop_stack m r
  | Perform (name, srcs) -> inputs m srcs >>= apply_action m name
;;

(** [start m] runs the controller from the first instruction until a
    failure; every executed instruction counts in [steps]. *)
let start m =
  let size = Array.length m.program.code in
  let rec loop () =
    let pc = !(m.pc) in
    if pc >= size
    then Ok ()
    else (
      m.pc := pc + 1;
      incr m.steps;
      match step m m.program.code.(pc) with
      | Ok () -> loop ()
      | Error e -> Error e)
  in
  loop ()
;;

(** [start_upto m limit] runs at most [limit] instructions, then
    fails with [Op_failed "step limit exceeded"]; the debugging
    harness of a loop the controller cannot leave. *)
let start_upto m limit =
  let size = Array.length m.program.code in
  let rec loop budget =
    let pc = !(m.pc) in
    if pc >= size
    then Ok ()
    else if budget = 0
    then Error (Op_failed "step limit exceeded")
    else (
      m.pc := pc + 1;
      incr m.steps;
      match step m m.program.code.(pc) with
      | Ok () -> loop (budget - 1)
      | Error e -> Error e)
  in
  loop limit
;;

(** [instruction_text m i] is the [i]th instruction as the machine
    reads it. *)
let instruction_text m i = Sec_5_1.instruction_to_string m.program.code.(i)

(** [current_pc m] is the program counter. *)
let current_pc m = !(m.pc)

let say m line =
  m.output := !(m.output) @ [ line ];
  Ok ()
;;

let transcript m = !(m.output)

(** [step_count m] is the number of instructions the machine executed. *)
let step_count m = !(m.steps)

(** {1:controller The 5.5.7 controller} *)

(** The apply-dispatch of 5.5.7: the compiled-procedure test before the
    unknown-type stop, and the compiled entry that restores [continue]
    and jumps to the compiled code. *)
let apply_dispatch_compiled =
  {|apply-dispatch
  (test (op primitive-procedure?) (reg proc))
  (branch (label primitive-apply))
  (test (op compound-procedure?) (reg proc))
  (branch (label compound-apply))
  (test (op compiled-procedure?) (reg proc))
  (branch (label compiled-apply))
  (goto (label unknown-procedure-type))
compiled-apply
  (restore continue)
  (assign val (op compiled-procedure-entry) (reg proc))
  (goto (reg val))|}
;;

(** The external entry: reached when the machine starts with [flag]
    set, it points [continue] at [print-result] and jumps to the
    compiled code in [val]. *)
let external_entry_block =
  {|external-entry
  (perform (op initialize-stack))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (reg val))|}
;;

let driver_with_external_entry =
  ";; branches if flag is set:\n(branch (label external-entry))\n"
  ^ List.assoc "driver" Sec_5_4.controller_fragments
;;

(** [eceval_fragments] is the 5.5.7 evaluator's controller in named
    fragments, in printed order: the 5.4 fragments, the flag-guarded
    driver, the compiled apply-dispatch, and the external entry.  A
    variant machine replaces a fragment (the monitored driver of 5.45)
    and concatenates. *)
let eceval_fragments =
  let frag name = List.assoc name Sec_5_4.controller_fragments in
  [ "driver", driver_with_external_entry
  ; "eval-dispatch", frag "eval-dispatch"
  ; "ev-self-eval", frag "ev-self-eval"
  ; "ev-variable", frag "ev-variable"
  ; "ev-quoted", frag "ev-quoted"
  ; "ev-lambda", frag "ev-lambda"
  ; "ev-application", frag "ev-application"
  ; "ev-appl-did-operator", frag "ev-appl-did-operator"
  ; "argument-loop", frag "argument-loop"
  ; "apply-dispatch", apply_dispatch_compiled
  ; "primitive-apply", frag "primitive-apply"
  ; "compound-apply", frag "compound-apply"
  ; "begin", frag "begin"
  ; "ev-sequence", frag "ev-sequence"
  ; "if", frag "if"
  ; "assignment", frag "assignment"
  ; "definition", frag "definition"
  ; "errors", frag "errors"
  ; "external-entry", external_entry_block
  ]
;;

(** [eceval_controller] is the fragments concatenated. *)
let eceval_controller = String.concat "\n" (List.map snd eceval_fragments)

(** [make_compiled_evaluator ~source ~state ()] builds the 5.5.7
    machine: the controller is the [eceval_controller] unless a variant
    is given, the operations are the 5.4 base table, the compiled
    operations, the compiled [extend-environment], the machine's own
    driver operations, and any extras last (overriding on a name
    collision); the global environment binds [true], [false], and the
    runtime primitives; the source is read into the driver's input
    queue.  The [flag] register starts false, so the plain driver
    path runs. *)
let make_compiled_evaluator
      ?(controller = eceval_controller)
      ?(operations = [])
      ?(globals = [])
      ~source
      ~state
      ()
  : (machine, error) result
  =
  Sec_5_1.parse_program ("(controller\n" ^ controller ^ "\n)")
  >>= fun program ->
  let rt = build_runtime_primitives () @ List.map (fun (name, f) -> name, f) globals in
  let global_env = Env.empty () in
  Env.define global_env "true" (Value.bool true);
  Env.define global_env "false" (Value.bool false);
  List.iter (fun (name, f) -> Env.define global_env name (Value.primitive ~name f)) rt;
  let table : compiled_table = Hashtbl.create 16 in
  let label_table = Hashtbl.create (List.length program.labels * 2) in
  List.iter (fun (l, i) -> Hashtbl.replace label_table l i) program.labels;
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
    ; steps = ref 0
    ; rt_primitives = rt
    ; label_table
    }
  in
  List.iter
    (fun r -> Hashtbl.replace m.regs r (ref (Sec_5_4.V (Value.symbol "*unassigned*"))))
    machine_registers;
  let install (n, o) = Hashtbl.replace m.operations n o in
  List.iter install Sec_5_4.base_operations;
  List.iter install (compiled_operations state table);
  install ("extend-environment", extend_environment_compiled);
  (* The environment procedures accept either an [Exp] word (the
     interpreted path, from [reg exp]) or a [V] name constant (the
     compiled path, from [(const name)]). *)
  let name_of = function
    | Sec_5_4.Exp e ->
      (match Ast.view e with
       | Ast.Variable n -> Ok n
       | _ -> Error (Op_failed "the register holds no variable"))
    | Sec_5_4.V v ->
      (match Value.view v with
       | Value.Symbol n -> Ok n
       | _ -> Error (Op_failed "the constant is not a variable name"))
    | w -> Error (Op_failed ("no variable: " ^ Sec_5_4.word_to_string w))
  in
  install
    ( "lookup-variable-value"
    , Sec_5_4.Value_op
        (function
          | [ w; Sec_5_4.Env env ] ->
            name_of w
            >>= fun name ->
            (match Env.find_binding env name with
             | Some v -> Ok (Sec_5_4.V v)
             | None -> Error (Op_failed ("unbound variable: " ^ name)))
          | _ -> Error (Arity "lookup-variable-value needs a variable and an environment"))
    );
  install
    ( "set-variable-value!"
    , Sec_5_4.Action_op
        (function
          | [ w; Sec_5_4.V v; Sec_5_4.Env env ] ->
            name_of w
            >>= fun name -> Env.set env name v |> Result.map_error Sec_5_4.eval_error
          | _ ->
            Error
              (Arity "set-variable-value! needs a variable, a value, and an environment"))
    );
  install
    ( "define-variable!"
    , Sec_5_4.Action_op
        (function
          | [ w; Sec_5_4.V v; Sec_5_4.Env env ] ->
            name_of w
            >>= fun name ->
            Env.define env name v;
            Ok ()
          | _ ->
            Error (Arity "define-variable! needs a variable, a value, and an environment"))
    );
  install
    ( "apply-primitive-procedure"
    , Sec_5_4.Value_op
        (function
          | [ Sec_5_4.V v; Sec_5_4.Args ws ] ->
            (match Value.view v with
             | Value.Primitive_procedure name ->
               Sec_5_4.word_values ws
               >>= fun values ->
               (match List.assoc_opt name m.rt_primitives with
                | Some f ->
                  f values
                  |> Result.map (fun r -> Sec_5_4.V r)
                  |> Result.map_error Sec_5_4.eval_error
                | None -> Error (Op_failed ("no primitive " ^ name)))
             | _ ->
               Error (Op_failed "apply-primitive-procedure needs a primitive procedure"))
          | _ ->
            Error
              (Op_failed "apply-primitive-procedure needs a procedure and an operand list"))
    );
  install
    ("get-global-environment", Sec_5_4.Value_op (fun _ -> Ok (Sec_5_4.Env global_env)));
  install ("initialize-stack", Sec_5_4.Action_op (fun _ -> initialize_stack m));
  install
    ( "print-stack-statistics"
    , Sec_5_4.Action_op (fun _ -> say m (print_stack_statistics m)) );
  install ("prompt-for-input", Sec_5_4.Action_op (fun _ -> say m ";;; EC-Eval input:"));
  install ("announce-output", Sec_5_4.Action_op (fun _ -> say m ";;; EC-Eval value:"));
  install
    ( "read"
    , Sec_5_4.Value_op
        (function
          | [] ->
            (match !(m.input_queue) with
             | [] -> Error (Op_failed Sec_5_4.input_exhausted)
             | form :: rest ->
               m.input_queue := rest;
               Ok (Sec_5_4.Exp form))
          | _ -> Error (Arity "read takes no arguments")) );
  install
    ( "user-print"
    , Sec_5_4.Action_op
        (function
          | [ w ] -> say m (Sec_5_4.word_to_string w)
          | _ -> Error (Arity "user-print needs one argument")) );
  install
    ( "signal-error"
    , Sec_5_4.Action_op
        (function
          | [ w ] -> Error (Op_failed ("signal-error: " ^ Sec_5_4.word_to_string w))
          | _ -> Error (Arity "signal-error needs one argument")) );
  List.iter install operations;
  match Reader.read_program source with
  | Ok forms ->
    m.input_queue := forms;
    Ok m
  | Error e -> Error (Parse (Reader.to_string e))
;;

(** [set_flag m b] arms the external entry: the machine's controller
    branches to [external-entry] exactly when [flag] is true. *)
let set_flag m b = ignore (set_register m "flag" (Sec_5_4.V (Value.bool b)))

(** [compile_block state forms] is the controller block of the compiled
    forms under a fresh entry label, plus that label. *)
let compile_block ?(cfg = default_config) state forms =
  (match Reader.read_program forms with
   | Ok exps -> Ok exps
   | Error e -> Error (Parse (Reader.to_string e)))
  >>= fun exps ->
  compile_program ~cfg ~linkage:Return state exps
  >>= fun seq ->
  let entry = "compiled-entry-" ^ string_of_int (Hashtbl.length state.consts) in
  (* the book compiles the whole expression with target [val] and
     linkage [return]: the last form's linkage preserves the caller's
     [continue] and returns to it *)
  Ok (entry, entry ^ "\n" ^ statements_text seq)
;;

(** [compile_and_go ~state ~compiled ~source ()] is the book's
    compile-and-go: the compiled expression is appended to the
    machine's controller, [val] is set to its entry, the flag arms the
    external entry, and the machine runs the compiled code, prints the
    value, and enters the driver loop, whose inputs are [source]. *)
let compile_and_go ?(cfg = default_config) ~state ~compiled ~source ()
  : (machine, error) result
  =
  compile_block ~cfg state compiled
  >>= fun (entry, block) ->
  make_compiled_evaluator ~controller:(eceval_controller ^ "\n" ^ block) ~source ~state ()
  >>= fun m ->
  set_register m "val" (Sec_5_4.Lab entry)
  >>= fun () ->
  set_flag m true;
  Ok m
;;
