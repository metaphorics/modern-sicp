(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** The register-machine simulator of section 5.2: the assembler that
    turns controller text into instruction objects with resolved
    labels, the execution-procedure dispatch, and the monitored stack
    of 5.2.4.

    The instruction ADT and the typed failures are the 5.1 substrate's,
    re-exported here: a machine description is still data in the book's
    notation, and nothing raises. The machine itself is an abstract
    mutable record -- the edition's reading of the book's
    message-passing model -- whose [pc] is an index into the
    instruction array (labels assemble to indices) and whose [flag] is
    an ordinary register that starts unassigned.

    The machine type stays open, as in 5.1: the monitoring exercises of
    5.2.4 wrap the same execution loop; 5.3 replaces the register file
    and the stack by vector memory behind this surface; 5.4 shares the
    instruction type with the explicit-control evaluator. *)

let ( >>= ) = Result.bind

(** {1:re-exports The 5.1 substrate's values, failures, and reader} *)

type value = Sec_5_1.value =
  | Int of int
  | Float of float
  | Bool of bool
  | Symbol of string
  | Label of string

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

type source = Sec_5_1.source =
  | Reg of string
  | Const of value
  | Label_source of string

type instruction = Sec_5_1.instruction =
  | Assign of string * source
  | Assign_op of string * string * source list
  | Test of string * source list
  | Branch of string
  | Goto_label of string
  | Goto_reg of string
  | Perform of string * source list
  | Save of string
  | Restore of string

type op = Sec_5_1.op =
  | Value_op of (value list -> (value, error) result)
  | Action_op of (value list -> (unit, error) result)

let value_to_string = Sec_5_1.value_to_string
let equal_value = Sec_5_1.equal_value
let error_to_string = Sec_5_1.error_to_string
let instruction_to_string = Sec_5_1.instruction_to_string
let source_to_string = Sec_5_1.source_to_string
let parse_program = Sec_5_1.parse_program
let arith_operations = Sec_5_1.arith_operations

(** A parsed controller, the reader's output and the assembler's input. *)
type program = Sec_5_1.program =
  { code : instruction array
  ; labels : (string * int) list
  }

(** [instruction_registers inst] names every register [inst] reads or
    writes, in order; the machine's own [flag] is never named. *)
let source_register = function
  | Reg r -> [ r ]
  | _ -> []
;;

let instruction_registers = function
  | Assign (r, src) -> r :: source_register src
  | Assign_op (r, _, inputs) -> r :: List.concat_map source_register inputs
  | Test (_, inputs) -> List.concat_map source_register inputs
  | Branch _ | Goto_label _ -> []
  | Goto_reg r -> [ r ]
  | Perform (_, inputs) -> List.concat_map source_register inputs
  | Save r | Restore r -> [ r ]
;;

(** {1:machine-model The machine model} *)

(** One simulated register: its name and its contents. The book's
    [make-register] is a message-passing object; the edition's is a
    record with a mutable field. A register starts unassigned -- the
    book's [*unassigned*] sentinel is the [Symbol] value below. *)
type register =
  { name : string
  ; mutable contents : value
  }

let make_register name = { name; contents = Symbol "*unassigned*" }

(** The monitored stack of 5.2.4: a push count and a maximum depth
    beside the entries, with [initialize] clearing all of them and
    [statistics] rendering the counters the exercises measure. *)
type stack =
  { push : value -> unit
  ; pop : string -> (value, error) result
    (* the argument names the register whose restore asked, for the
           typed underflow report *)
  ; initialize : unit -> unit
  ; statistics : unit -> string
  }

(** [make_stack ()] is the empty stack. *)
let make_stack () =
  let entries = ref [] in
  let number_pushes = ref 0 in
  let current_depth = ref 0 in
  let maximum_depth = ref 0 in
  let push v =
    entries := v :: !entries;
    incr number_pushes;
    incr current_depth;
    if !current_depth > !maximum_depth then maximum_depth := !current_depth
  in
  let pop name =
    match !entries with
    | [] -> Error (Stack_underflow name)
    | top :: rest ->
      decr current_depth;
      entries := rest;
      Ok top
  in
  let initialize () =
    entries := [];
    number_pushes := 0;
    current_depth := 0;
    maximum_depth := 0
  in
  let statistics () =
    Printf.sprintf "total-pushes = %d maximum-depth = %d" !number_pushes !maximum_depth
  in
  { push; pop; initialize; statistics }
;;

(** One instruction object of 5.2.2: the typed instruction -- the
    book's [instruction-text], retained for the tracing exercises --
    and the execution procedure built for it at assembly time. *)
type inst =
  { text : instruction
  ; mutable exec : unit -> (unit, error) result
  }

(** [make_inst text] is the placeholder the assembler fills in: the
    execution procedure is not yet available when the label scan runs. *)
let make_inst text = { text; exec = (fun () -> Ok ()) }

(** One machine: the register table (which always contains [flag]),
    the operations list (which always begins with the stack
    operations), the assembled instruction array with each label
    resolved to an index, the monitored stack, the [pc], and the
    transcript the [print-stack-statistics] action appends to. *)
type machine =
  { regs : (string, register) Hashtbl.t
  ; ops : (string * op) list ref
  ; mutable insts : inst array
  ; labels : (string, int) Hashtbl.t
  ; stack : stack
  ; pc : int ref
  ; output : string list ref
  }

(** [make_new_machine ()] is the basic machine of Figure 5.13: a stack,
    an empty instruction sequence, the stack operations, and a register
    table holding [flag]. The book's [pc] register holds the remaining
    instruction list; the edition's is the index of the next
    instruction, so [start] seeds it with [0] and the sequence ends
    when the index reaches the array length. *)
let make_new_machine () =
  let m =
    { regs = Hashtbl.create 16
    ; ops = ref []
    ; insts = [||]
    ; labels = Hashtbl.create 16
    ; stack = make_stack ()
    ; pc = ref 0
    ; output = ref []
    }
  in
  Hashtbl.replace m.regs "flag" (make_register "flag");
  let transcript line = m.output := !(m.output) @ [ line ] in
  m.ops
  := [ ( "initialize-stack"
       , Action_op
           (fun _ ->
             m.stack.initialize ();
             Ok ()) )
     ; ( "print-stack-statistics"
       , Action_op
           (fun _ ->
             transcript (m.stack.statistics ());
             Ok ()) )
     ];
  m
;;

(** [allocate_register m name] adds a register to the table; a name
    used twice is a defect of the description. *)
let allocate_register m name =
  if Hashtbl.mem m.regs name
  then Error (Bad_instruction ("the register " ^ name ^ " is declared twice"))
  else (
    Hashtbl.replace m.regs name (make_register name);
    Ok ())
;;

let rec seq f = function
  | [] -> Ok ()
  | x :: xs -> f x >>= fun () -> seq f xs
;;

let register_of m name =
  match Hashtbl.find_opt m.regs name with
  | Some r -> Ok r
  | None -> Error (Unknown_register name)
;;

(** [set_register m r v] is [set-register-contents!]: it stores a value
    in the named register. *)
let set_register m name v =
  register_of m name
  >>= fun r ->
  r.contents <- v;
  Ok ()
;;

(** [get_register m r] is [get-register-contents]. *)
let get_register m name = register_of m name >>= fun r -> Ok r.contents

let check_registers m (program : program) =
  let defects =
    Array.to_list program.code
    |> List.concat_map instruction_registers
    |> List.filter (fun r -> not (Hashtbl.mem m.regs r))
  in
  match defects with
  | r :: _ -> Error (Unknown_register r)
  | [] -> Ok ()
;;

(** {1:assembler The assembler} *)

(** [make_label_entry l i] pairs a label with the index it names. *)
let make_label_entry label index = label, index

(** [lookup_label m l] resolves a label to an instruction index. *)
let lookup_label m name =
  match Hashtbl.find_opt m.labels name with
  | Some i -> Ok i
  | None -> Error (Unknown_label name)
;;

(** {1:execution Execution procedures for instructions} *)

(** [lookup_prim name ops] finds the operation table's entry at assembly
    time; an operation the table does not name fails the assembly. *)
let lookup_prim name ops =
  match List.assoc_opt name ops with
  | Some o -> Ok o
  | None -> Error (Unknown_operation name)
;;

(** [make_primitive_exp exp m] builds the execution procedure for one
    [reg], [const], or [label] expression: the register is resolved to
    its record and the label to its address now, once, so the procedure
    only reads at simulation time. *)
let make_primitive_exp exp m =
  match exp with
  | Const c -> Ok (fun () -> Ok c)
  | Reg r -> register_of m r >>= fun reg -> Ok (fun () -> Ok reg.contents)
  | Label_source l -> lookup_label m l >>= fun _ -> Ok (fun () -> Ok (Label l))
;;

(** [make_operation_exp name inputs m] builds the procedure that
    produces an operation's argument values: one operand procedure per
    operand, assembled now -- the same analysis the metacircular
    evaluator's [analyze-application] performs. At simulation time the
    operand procedures run and the table's operation consumes the
    values. *)
let make_operation_exp name inputs m =
  lookup_prim name !(m.ops)
  >>= fun o ->
  let rec build = function
    | [] -> Ok []
    | e :: rest ->
      make_primitive_exp e m >>= fun p -> build rest >>= fun ps -> Ok (p :: ps)
  in
  build inputs
  >>= fun argprocs ->
  let rec collect = function
    | [] -> Ok []
    | p :: rest -> p () >>= fun v -> collect rest >>= fun vs -> Ok (v :: vs)
  in
  Ok (fun () -> collect argprocs >>= fun args -> Ok (o, args))
;;

(** An operation expression used where a value is wanted ([assign],
    [test]) refuses an action; one under [perform] refuses a value.
    The mismatches are the typed failures the substrate pins. *)
let apply_value_op name = function
  | o, args ->
    (match o with
     | Value_op f -> f args
     | Action_op _ ->
       Error
         (Bad_instruction ("the operation " ^ name ^ " is an action and produces no value")))
;;

let apply_action_op name = function
  | o, args ->
    (match o with
     | Action_op f -> f args
     | Value_op _ ->
       Error
         (Bad_instruction
            ("the operation " ^ name ^ " produces a value; assign it, do not perform it")))
;;

(** [advance_pc m] steps past the instruction just executed; it is the
    normal termination for every instruction except [branch] and
    [goto]. *)
let advance_pc m =
  incr m.pc;
  Ok ()
;;

(** [make_assign inst m] resolves the target register and the value
    expression at assembly time. *)
let make_assign inst m =
  let finish target value_proc =
    register_of m target
    >>= fun reg ->
    Ok
      (fun () ->
        value_proc ()
        >>= fun v ->
        reg.contents <- v;
        advance_pc m)
  in
  match inst with
  | Assign (target, src) -> make_primitive_exp src m >>= fun vp -> finish target vp
  | Assign_op (target, name, inputs) ->
    make_operation_exp name inputs m
    >>= fun vp -> finish target (fun () -> vp () >>= apply_value_op name)
  | _ -> Error (Bad_instruction "not an assign")
;;

(** [make_test inst m] requires the operation form -- the typed ADT
    admits no other -- and the flag register is resolved at assembly
    time; a test that answers a non-boolean fails at simulation time. *)
let make_test inst m =
  match inst with
  | Test (name, inputs) ->
    make_operation_exp name inputs m
    >>= fun cond ->
    register_of m "flag"
    >>= fun flag ->
    Ok
      (fun () ->
        cond ()
        >>= apply_value_op name
        >>= fun v ->
        match v with
        | Bool b ->
          flag.contents <- Bool b;
          advance_pc m
        | other ->
          Error
            (Bad_instruction
               ("the test "
                ^ name
                ^ " answered "
                ^ value_to_string other
                ^ ", not a boolean")))
  | _ -> Error (Bad_instruction "not a test")
;;

(** [make_branch inst m] requires a label destination and resolves it
    to an index now; the flag is read when the branch runs, and a
    branch reached with no preceding test is the typed failure. *)
let make_branch inst m =
  match inst with
  | Branch label_name ->
    lookup_label m label_name
    >>= fun target ->
    register_of m "flag"
    >>= fun flag ->
    Ok
      (fun () ->
        match flag.contents with
        | Bool b ->
          if b
          then (
            m.pc := target;
            Ok ())
          else advance_pc m
        | _ -> Error Branch_without_test)
  | _ -> Error (Bad_instruction "not a branch")
;;

(** [make_goto inst m] accepts either destination: a label resolved
    now, or a register whose [Label] contents name the target when the
    instruction runs. *)
let make_goto inst m =
  match inst with
  | Goto_label label_name ->
    lookup_label m label_name
    >>= fun target ->
    Ok
      (fun () ->
        m.pc := target;
        Ok ())
  | Goto_reg reg_name ->
    register_of m reg_name
    >>= fun reg ->
    Ok
      (fun () ->
        match reg.contents with
        | Label l ->
          lookup_label m l
          >>= fun target ->
          m.pc := target;
          Ok ()
        | other ->
          Error
            (Bad_instruction
               ("goto reads "
                ^ value_to_string other
                ^ " from "
                ^ reg_name
                ^ ", not a label")))
  | _ -> Error (Bad_instruction "not a goto")
;;

(** The stack instructions use the machine's monitored stack with the
    designated register and advance the [pc]. *)
let make_save inst m =
  match inst with
  | Save reg_name ->
    register_of m reg_name
    >>= fun reg ->
    Ok
      (fun () ->
        m.stack.push reg.contents;
        advance_pc m)
  | _ -> Error (Bad_instruction "not a save")
;;

let make_restore inst m =
  match inst with
  | Restore reg_name ->
    register_of m reg_name
    >>= fun reg ->
    Ok
      (fun () ->
        m.stack.pop reg.name
        >>= fun v ->
        reg.contents <- v;
        advance_pc m)
  | _ -> Error (Bad_instruction "not a restore")
;;

(** [make_perform inst m] builds the action's procedure; at simulation
    time the action runs and the [pc] advances. *)
let make_perform inst m =
  match inst with
  | Perform (name, inputs) ->
    make_operation_exp name inputs m
    >>= fun action ->
    Ok (fun () -> action () >>= apply_action_op name >>= fun () -> advance_pc m)
  | _ -> Error (Bad_instruction "not a perform")
;;

(** [make_execution_procedure inst m] dispatches on the instruction's
    constructor -- the typed edition's reading of the book's [cond]
    over instruction types, exhaustive by construction. *)
let make_execution_procedure inst m =
  match inst with
  | Assign _ | Assign_op _ -> make_assign inst m
  | Test _ -> make_test inst m
  | Branch _ -> make_branch inst m
  | Goto_label _ | Goto_reg _ -> make_goto inst m
  | Save _ -> make_save inst m
  | Restore _ -> make_restore inst m
  | Perform _ -> make_perform inst m
;;

(** [update_insts texts m] modifies the instruction list, which
    initially contains only the text of the instructions, to include
    the corresponding execution procedures. *)
let update_insts texts m =
  let insts = Array.map make_inst texts in
  let rec fill i =
    if i = Array.length insts
    then Ok insts
    else
      make_execution_procedure insts.(i).text m
      >>= fun exec ->
      insts.(i).exec <- exec;
      fill (i + 1)
  in
  fill 0
;;

(** [assemble controller m] transforms the controller text into the
    machine's instruction sequence. The label scan is the shared
    reader's [parse_program]: each label names the index of the
    instruction that follows it, a label at the end names the stop
    index one past the last instruction, and a label used twice is
    rejected there. [update_insts] then fills each instruction
    object's execution procedure, resolving branch and goto targets to
    indices as it goes -- an unknown register, operation, or label
    fails the assembly before the machine can start. *)
let install_program (m : machine) (program : program) =
  check_registers m program
  >>= fun () ->
  let entries =
    List.map (fun (label, index) -> make_label_entry label index) program.labels
  in
  List.iter (fun (label, index) -> Hashtbl.replace m.labels label index) entries;
  update_insts program.code m
  >>= fun insts ->
  m.insts <- insts;
  Ok ()
;;

let assemble controller m = parse_program controller >>= install_program m

(** [make_machine_from_program ~registers ~operations program] is the
    assembler's full path from a parsed controller -- the entry the
    exercises that build programs by other syntaxes reuse. *)
let make_machine_from_program ~registers ~operations program =
  let m = make_new_machine () in
  seq (allocate_register m) registers
  >>= fun () ->
  m.ops := !(m.ops) @ operations;
  install_program m program >>= fun () -> Ok m
;;

(** [make_machine ~registers ~operations ~controller] is the book's
    constructor: allocate the registers, install the operations, and
    assemble the controller into the machine. *)
let make_machine ~registers ~operations ~controller =
  let m = make_new_machine () in
  seq (allocate_register m) registers
  >>= fun () ->
  m.ops := !(m.ops) @ operations;
  assemble controller m >>= fun () -> Ok m
;;

(** {1:driver The driver loop} *)

(** [start m] runs the machine from the beginning of the controller
    sequence: the [pc] seeds at [0] and each instruction's execution
    procedure runs in turn until the sequence ends -- the book's stop
    condition -- or an instruction fails. *)
let start m =
  m.pc := 0;
  let rec execute () =
    if !(m.pc) >= Array.length m.insts
    then Ok ()
    else m.insts.(!(m.pc)).exec () >>= execute
  in
  execute ()
;;

(** [print_stack_statistics m] renders the monitored stack's counters. *)
let print_stack_statistics m = m.stack.statistics ()

(** [transcript m] is what the [print-stack-statistics] action has
    printed, in order. *)
let transcript m = !(m.output)
