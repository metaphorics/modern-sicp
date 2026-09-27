(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** The register-machine substrate of section 5.1: the machine values,
    the instruction language of 5.1.5, and the simulator core (registers,
    labeled controller, save/restore stack, operations table) the rest of
    chapter 5 builds on.

    A machine description is data in the book's register-machine
    language: the shared [Reader] parses the controller text and
    [parse_program] turns the [Ast] into typed instructions, so 5.1's
    machines are programs in the book's notation, never host code.

    The machine and state types are deliberately open. Section 5.2
    elaborates the simulator around the same [instruction] array
    (counters, tracing, breakpoints); 5.3 replaces this flat register
    file by vector memory whose addresses name pairs, keeping [value]
    as the stack's element type; 5.4 shares the instruction type with
    the explicit-control evaluator, whose registers ([exp], [env],
    [val], [continue], [proc], [argl], [unev]) are ordinary entries of
    the same register table. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Reader = Sicp_common.Reader

(** {1:machine-values Machine values and failures} *)

(** One machine value: the contents a register or stack entry holds.
    [Label] is the first-class entry point of 5.1.3 (the [continue]
    register's contents); [Symbol] is the constant kind 5.1.5 announces
    beyond numbers. 5.3 adds pairs that live in vector memory. *)
type value =
  | Int of int
  | Float of float
  | Bool of bool
  | Symbol of string
  | Label of string

let float_to_string f =
  if Float.is_integer f && Float.abs f < 1e16
  then Printf.sprintf "%.0f" f
  else (
    let rec precision p =
      let s = Printf.sprintf "%.*g" p f in
      if p >= 17 || Float.equal (Float.of_string s) f then s else precision (p + 1)
    in
    precision 15)
;;

let value_to_string = function
  | Int n -> string_of_int n
  | Float f -> float_to_string f
  | Bool b -> if b then "true" else "false"
  | Symbol s -> s
  | Label l -> l
;;

let equal_value a b =
  match a, b with
  | Int x, Int y -> Int.equal x y
  | Float x, Float y -> Float.equal x y
  | Int x, Float y | Float y, Int x -> Float.equal (float_of_int x) y
  | Bool x, Bool y -> Bool.equal x y
  | Symbol x, Symbol y -> String.equal x y
  | Label x, Label y -> String.equal x y
  | _ -> false
;;

(** Every failure of the substrate travels through [error]; nothing
    raises. *)
type error =
  | Parse of string (** The controller text is not well-formed machine language. *)
  | Unknown_register of string
  | Unknown_operation of string
  | Unknown_label of string
  | Bad_instruction of string
  (** Well-read text that the grammar of 5.1.5 does not allow. *)
  | Arity of string (** An operation refused its arguments. *)
  | Op_failed of string
  (** An operation refused the computation itself: a division by
          zero, an exhausted input. *)
  | Stack_underflow of string (** A restore with an empty stack, naming the register. *)
  | Branch_without_test (** A branch with no preceding test. *)

let error_to_string = function
  | Parse s -> "machine parse error: " ^ s
  | Unknown_register r -> "unknown register " ^ r
  | Unknown_operation op -> "unknown operation " ^ op
  | Unknown_label l -> "unknown label " ^ l
  | Bad_instruction s -> "bad instruction: " ^ s
  | Arity s -> "operation arity: " ^ s
  | Op_failed s -> "operation failed: " ^ s
  | Stack_underflow r -> "stack underflow restoring " ^ r
  | Branch_without_test -> "branch without a preceding test"
;;

(** {1:instructions The instruction language of 5.1.5} *)

(** One source of a value: a register, a constant, or -- the 5.1.3
    extension -- a label read as a special constant. *)
type source =
  | Reg of string
  | Const of value
  | Label_source of string

(** One controller instruction. [Assign] is [assign] from a single
    source; [Assign_op] is [assign] from an operation applied to its
    inputs. *)
type instruction =
  | Assign of string * source
  | Assign_op of string * string * source list
  | Test of string * source list
  | Branch of string
  | Goto_label of string
  | Goto_reg of string
  | Perform of string * source list
  | Save of string
  | Restore of string

let source_to_string = function
  | Reg r -> "(reg " ^ r ^ ")"
  | Const v -> "(const " ^ value_to_string v ^ ")"
  | Label_source l -> "(label " ^ l ^ ")"
;;

let inputs_to_string inputs =
  match inputs with
  | [] -> ""
  | _ -> " " ^ String.concat " " (List.map source_to_string inputs)
;;

(** [instruction_to_string i] renders [i] back in the book's notation. *)
let instruction_to_string = function
  | Assign (r, src) -> "(assign " ^ r ^ " " ^ source_to_string src ^ ")"
  | Assign_op (r, op, inputs) ->
    "(assign " ^ r ^ " (op " ^ op ^ ")" ^ inputs_to_string inputs ^ ")"
  | Test (op, inputs) -> "(test (op " ^ op ^ ")" ^ inputs_to_string inputs ^ ")"
  | Branch l -> "(branch (label " ^ l ^ "))"
  | Goto_label l -> "(goto (label " ^ l ^ "))"
  | Goto_reg r -> "(goto (reg " ^ r ^ "))"
  | Perform (op, inputs) -> "(perform (op " ^ op ^ ")" ^ inputs_to_string inputs ^ ")"
  | Save r -> "(save " ^ r ^ ")"
  | Restore r -> "(restore " ^ r ^ ")"
;;

(** {1:parsing Reading machine descriptions} *)

(** A parsed controller: the instructions in order, and each label with
    the position of the instruction it names. *)
type program =
  { code : instruction array
  ; labels : (string * int) list
  }

let reg_name e =
  match Ast.view e with
  | Ast.Variable r -> Ok r
  | _ -> Error (Bad_instruction "a register or label name is written as a symbol")
;;

(** [tagged tag e] is the argument list when [e] is written
    [(tag arg ...)], and [None] otherwise. *)
let tagged tag e =
  match Ast.view e with
  | Ast.Application (op, args) ->
    (match Ast.view op with
     | Ast.Variable name when String.equal name tag -> Some args
     | _ -> None)
  | _ -> None
;;

let const_of_expr e =
  match Ast.view e with
  | Ast.Int n -> Ok (Int n)
  | Ast.Float f -> Ok (Float f)
  | Ast.Bool b -> Ok (Bool b)
  | Ast.Variable s -> Ok (Symbol s)
  | _ ->
    Error (Bad_instruction "the constant kinds of 5.1 are numbers, booleans, and symbols")
;;

let input_of_expr e =
  match Ast.view e with
  | Ast.Application (op, [ arg ]) ->
    (match Ast.view op with
     | Ast.Variable "reg" -> reg_name arg >>= fun r -> Ok (Reg r)
     | Ast.Variable "const" -> const_of_expr arg >>= fun v -> Ok (Const v)
     | _ -> Error (Bad_instruction "an operation input is written (reg r) or (const c)"))
  | _ -> Error (Bad_instruction "an operation input is written (reg r) or (const c)")
;;

let rec all f = function
  | [] -> Ok []
  | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
;;

let op_name e =
  match tagged "op" e with
  | Some [ name ] ->
    (match Ast.view name with
     | Ast.Variable name -> Ok name
     | _ -> Error (Bad_instruction "an operation is written (op name)"))
  | _ -> Error (Bad_instruction "an operation is written (op name)")
;;

let is_op_form e = Option.is_some (tagged "op" e)

let assign_single_source single =
  match Ast.view single with
  | Ast.Application (op, [ arg ]) ->
    (match Ast.view op with
     | Ast.Variable "reg" -> reg_name arg >>= fun r -> Ok (Reg r)
     | Ast.Variable "const" -> const_of_expr arg >>= fun v -> Ok (Const v)
     | Ast.Variable "label" -> reg_name arg >>= fun l -> Ok (Label_source l)
     | _ ->
       Error
         (Bad_instruction
            "the assign source is written (reg r), (const c), (label l), or (op name ...)"))
  | _ ->
    Error
      (Bad_instruction
         "the assign source is written (reg r), (const c), (label l), or (op name ...)")
;;

let assign_instruction args =
  match args with
  | [] -> Error (Bad_instruction "an assign names its target register")
  | target :: rhs ->
    reg_name target
    >>= fun reg ->
    (match rhs with
     | first :: rest when is_op_form first ->
       op_name first
       >>= fun name ->
       all input_of_expr rest >>= fun inputs -> Ok (Assign_op (reg, name, inputs))
     | [ single ] -> assign_single_source single >>= fun src -> Ok (Assign (reg, src))
     | _ ->
       Error
         (Bad_instruction "an assign needs one source or an operation with its inputs"))
;;

let label_ref e =
  match tagged "label" e with
  | Some [ l ] -> reg_name l
  | _ -> Error (Bad_instruction "a label reference is written (label name)")
;;

let goto_instruction args =
  match args with
  | [ form ] ->
    (match tagged "label" form with
     | Some [ l ] -> reg_name l >>= fun l -> Ok (Goto_label l)
     | Some _ -> Error (Bad_instruction "a goto names one label")
     | None ->
       (match tagged "reg" form with
        | Some [ r ] -> reg_name r >>= fun r -> Ok (Goto_reg r)
        | Some _ -> Error (Bad_instruction "a goto names one register")
        | None ->
          Error (Bad_instruction "a goto is written (goto (label l)) or (goto (reg r))")))
  | _ -> Error (Bad_instruction "a goto names one target")
;;

let one_register what args =
  match args with
  | [ r ] -> reg_name r
  | _ -> Error (Bad_instruction (what ^ " names one register"))
;;

let with_operation args build =
  match args with
  | [] -> Error (Bad_instruction "the instruction names its operation")
  | form :: rest ->
    op_name form
    >>= fun name -> all input_of_expr rest >>= fun inputs -> Ok (build name inputs)
;;

let instruction_of_expr e =
  match Ast.view e with
  | Ast.Application (op, args) ->
    (match Ast.view op with
     | Ast.Variable "assign" -> assign_instruction args
     | Ast.Variable "test" -> with_operation args (fun name inputs -> Test (name, inputs))
     | Ast.Variable "branch" ->
       (match args with
        | [ form ] -> label_ref form >>= fun l -> Ok (Branch l)
        | _ -> Error (Bad_instruction "a branch names one label"))
     | Ast.Variable "goto" -> goto_instruction args
     | Ast.Variable "perform" ->
       with_operation args (fun name inputs -> Perform (name, inputs))
     | Ast.Variable "save" -> one_register "a save" args >>= fun r -> Ok (Save r)
     | Ast.Variable "restore" -> one_register "a restore" args >>= fun r -> Ok (Restore r)
     | _ -> Error (Bad_instruction "the form is not one of the instructions of 5.1.5"))
  | _ -> Error (Bad_instruction "the form is not one of the instructions of 5.1.5")
;;

(** [parse_program text] reads one [(controller ...)] form from [text]
    with the shared reader and resolves its labels: each label names the
    position of the instruction that follows it, and a label at the end
    of the sequence names the stop address, one past the last
    instruction -- the exit point whose reach means the machine is
    done. No label may be used twice. *)
let parse_program text =
  let read_one () =
    match Reader.read text with
    | Ok exp -> Ok exp
    | Error e -> Error (Parse (Reader.to_string e))
  in
  let rec scan pending items acc =
    match items with
    | [] -> Ok (List.rev acc, List.rev pending)
    | item :: rest ->
      (match Ast.view item with
       | Ast.Variable lab -> scan (lab :: pending) rest acc
       | _ ->
         (match instruction_of_expr item with
          | Error e -> Error e
          | Ok inst -> scan [] rest ((pending, inst) :: acc)))
  in
  read_one ()
  >>= fun exp ->
  match tagged "controller" exp with
  | None ->
    Error (Parse "the controller is written (controller label-or-instruction ...)")
  | Some body ->
    scan [] body []
    >>= fun (entries, trailing) ->
    let code = Array.of_list (List.map snd entries) in
    let stop = Array.length code in
    let positioned entries =
      List.concat
        (List.mapi (fun i (labels, _) -> List.map (fun l -> l, i) labels) entries)
    in
    let labels = positioned entries @ List.map (fun l -> l, stop) trailing in
    let rec duplicate used = function
      | [] -> Ok ()
      | (l, _) :: rest ->
        if List.mem_assoc l used
        then Error (Bad_instruction ("the label " ^ l ^ " is used twice"))
        else duplicate ((l, ()) :: used) rest
    in
    duplicate [] labels >>= fun () -> Ok { code; labels }
;;

(** {1:simulator The simulator core} *)

(** The operations table of a machine. A [Value_op] computes a value for
    an [assign] or a [test]; an [Action_op] is the 5.1.1 notion of an
    action -- [print] -- pushed by [perform] and producing no value. *)
type op =
  | Value_op of (value list -> (value, error) result)
  | Action_op of (value list -> (unit, error) result)

type machine =
  { regs : (string, value ref) Hashtbl.t
  ; ops : (string, op) Hashtbl.t
  ; labels : (string, int) Hashtbl.t
  ; code : instruction array
  ; stack : value list ref
  ; flag : bool option ref
  ; pc : int ref
  }

let source_register = function
  | Reg r -> [ r ]
  | _ -> []
;;

let instruction_registers inst =
  match inst with
  | Assign (r, src) -> r :: source_register src
  | Assign_op (r, _, inputs) -> r :: List.concat_map source_register inputs
  | Test (_, inputs) -> List.concat_map source_register inputs
  | Branch _ | Goto_label _ -> []
  | Goto_reg r -> [ r ]
  | Perform (_, inputs) -> List.concat_map source_register inputs
  | Save r | Restore r -> [ r ]
;;

let instruction_operation inst =
  match inst with
  | Assign_op (_, op, _) | Test (op, _) | Perform (op, _) -> [ op ]
  | _ -> []
;;

let instruction_label inst =
  match inst with
  | Branch l | Goto_label l -> [ l ]
  | _ -> []
;;

let first_defect report defects =
  match defects with
  | x :: _ -> Error (report x)
  | [] -> Ok ()
;;

let check_registers regs (program : program) =
  Array.to_list program.code
  |> List.concat_map instruction_registers
  |> List.filter (fun r -> not (Hashtbl.mem regs r))
  |> first_defect (fun r -> Unknown_register r)
;;

let check_operations ops (program : program) =
  Array.to_list program.code
  |> List.concat_map instruction_operation
  |> List.filter (fun n -> not (Hashtbl.mem ops n))
  |> first_defect (fun n -> Unknown_operation n)
;;

let check_labels labels (program : program) =
  Array.to_list program.code
  |> List.concat_map instruction_label
  |> List.filter (fun l -> not (Hashtbl.mem labels l))
  |> first_defect (fun l -> Unknown_label l)
;;

let declare_registers registers =
  let regs = Hashtbl.create 16 in
  let rec go = function
    | [] -> Ok regs
    | r :: rest ->
      if Hashtbl.mem regs r
      then Error (Bad_instruction ("the register " ^ r ^ " is declared twice"))
      else (
        Hashtbl.replace regs r (ref (Int 0));
        go rest)
  in
  go registers
;;

(** [make_machine] assembles the controller text and checks it against
    the declared registers and the operations table before the machine
    can start: an unknown register, operation, or label is a defect of
    the description, caught before execution. *)
let make_machine ~registers ~operations ~controller =
  parse_program controller
  >>= fun program ->
  declare_registers registers
  >>= fun regs ->
  let ops = Hashtbl.create 16 in
  List.iter (fun (name, o) -> Hashtbl.replace ops name o) operations;
  check_registers regs program
  >>= fun () ->
  check_operations ops program
  >>= fun () ->
  let labels = Hashtbl.create 16 in
  List.iter (fun (l, i) -> Hashtbl.replace labels l i) program.labels;
  check_labels labels program
  >>= fun () ->
  Ok
    { regs
    ; ops
    ; labels
    ; code = program.code
    ; stack = ref []
    ; flag = ref None
    ; pc = ref 0
    }
;;

let lookup m r =
  match Hashtbl.find_opt m.regs r with
  | Some cell -> Ok !cell
  | None -> Error (Unknown_register r)
;;

let store m r v =
  match Hashtbl.find_opt m.regs r with
  | Some cell ->
    cell := v;
    Ok ()
  | None -> Error (Unknown_register r)
;;

(** [set_register m r v] loads an input register before [start]. *)
let set_register m r v = store m r v

(** [get_register m r] reads a result register after [start]. *)
let get_register m r = lookup m r

let eval_source m = function
  | Reg r -> lookup m r
  | Const v -> Ok v
  | Label_source l -> Ok (Label l)
;;

let apply_value m name args =
  match Hashtbl.find_opt m.ops name with
  | None -> Error (Unknown_operation name)
  | Some (Value_op f) -> f args
  | Some (Action_op _) ->
    Error
      (Bad_instruction ("the operation " ^ name ^ " is an action and produces no value"))
;;

let apply_action m name args =
  match Hashtbl.find_opt m.ops name with
  | None -> Error (Unknown_operation name)
  | Some (Action_op f) -> f args
  | Some (Value_op _) ->
    Error
      (Bad_instruction
         ("the operation " ^ name ^ " produces a value; assign it, do not perform it"))
;;

let jump_to m l =
  match Hashtbl.find_opt m.labels l with
  | Some i ->
    m.pc := i;
    Ok ()
  | None -> Error (Unknown_label l)
;;

let advance m =
  m.pc := !(m.pc) + 1;
  Ok ()
;;

let step m =
  match m.code.(!(m.pc)) with
  | Assign (r, src) -> eval_source m src >>= fun v -> store m r v >>= fun () -> advance m
  | Assign_op (r, op, inputs) ->
    all (eval_source m) inputs
    >>= fun args -> apply_value m op args >>= fun v -> store m r v >>= fun () -> advance m
  | Test (op, inputs) ->
    all (eval_source m) inputs
    >>= fun args ->
    apply_value m op args
    >>= fun v ->
    (match v with
     | Bool b ->
       m.flag := Some b;
       advance m
     | other ->
       Error
         (Bad_instruction
            ("the test " ^ op ^ " answered " ^ value_to_string other ^ ", not a boolean")))
  | Branch l ->
    (match !(m.flag) with
     | None -> Error Branch_without_test
     | Some b -> if b then jump_to m l else advance m)
  | Goto_label l -> jump_to m l
  | Goto_reg r ->
    lookup m r
    >>= fun v ->
    (match v with
     | Label l -> jump_to m l
     | other ->
       Error
         (Bad_instruction
            ("goto reads " ^ value_to_string other ^ " from " ^ r ^ ", not a label")))
  | Perform (op, inputs) ->
    all (eval_source m) inputs
    >>= fun args -> apply_action m op args >>= fun () -> advance m
  | Save r ->
    lookup m r
    >>= fun v ->
    m.stack := v :: !(m.stack);
    advance m
  | Restore r ->
    (match !(m.stack) with
     | [] -> Error (Stack_underflow r)
     | v :: rest ->
       m.stack := rest;
       store m r v >>= fun () -> advance m)
;;

(** [start m] runs the controller from its first instruction until the
    sequence ends -- the book's stop condition -- or until an
    instruction fails. A driver-loop machine whose reads run dry fails
    through its [read] operation, which is how this edition stops the
    unbounded loop of Figure 5.4. *)
let start m =
  m.pc := 0;
  m.stack := [];
  m.flag := None;
  let rec loop () = if !(m.pc) >= Array.length m.code then Ok () else step m >>= loop in
  loop ()
;;

(** {1:operations The section's operation tables} *)

let number = function
  | Int n -> Some (float_of_int n)
  | Float f -> Some f
  | _ -> None
;;

let int_or_float name f_int f_float =
  Value_op
    (function
      | [ Int a; Int b ] -> Ok (Int (f_int a b))
      | [ a; b ] ->
        (match number a, number b with
         | Some x, Some y -> Ok (Float (f_float x y))
         | _ -> Error (Arity (name ^ " needs two numbers")))
      | _ -> Error (Arity (name ^ " needs two arguments")))
;;

let comparison name test =
  Value_op
    (function
      | [ a; b ] ->
        (match number a, number b with
         | Some x, Some y -> Ok (Bool (test x y))
         | _ -> Error (Arity (name ^ " needs two numbers")))
      | _ -> Error (Arity (name ^ " needs two arguments")))
;;

(** [arith_operations] is the arithmetic table the 5.1 machines name in
    their [(op ...)]: the remainder device of the GCD machine, the
    comparisons of the controllers, and the arithmetic the elaborated
    machines and 5.3 expand into. *)
let arith_operations =
  [ ( "="
    , Value_op
        (function
          | [ a; b ] -> Ok (Bool (equal_value a b))
          | _ -> Error (Arity "= needs two arguments")) )
  ; "<", comparison "<" ( < )
  ; ">", comparison ">" ( > )
  ; "+", int_or_float "+" ( + ) ( +. )
  ; "-", int_or_float "-" ( - ) ( -. )
  ; "*", int_or_float "*" ( * ) ( *. )
  ; ( "/"
    , Value_op
        (function
          | [ a; b ] ->
            (match number a, number b with
             | Some _, Some 0.0 -> Error (Op_failed "division by zero")
             | Some x, Some y -> Ok (Float (x /. y))
             | _ -> Error (Arity "/ needs two numbers"))
          | _ -> Error (Arity "/ needs two arguments")) )
  ; ( "rem"
    , Value_op
        (function
          | [ Int a; Int b ] ->
            if Int.equal b 0
            then Error (Op_failed "remainder by zero")
            else Ok (Int (a mod b))
          | _ -> Error (Arity "rem needs two integers")) )
  ; ( "abs"
    , Value_op
        (function
          | [ Int n ] -> Ok (Int (abs n))
          | [ Float f ] -> Ok (Float (Float.abs f))
          | _ -> Error (Arity "abs needs one number")) )
  ; ( "square"
    , Value_op
        (function
          | [ Int n ] -> Ok (Int (n * n))
          | [ Float f ] -> Ok (Float (f *. f))
          | _ -> Error (Arity "square needs one number")) )
  ; ( "average"
    , Value_op
        (function
          | [ a; b ] ->
            (match number a, number b with
             | Some x, Some y -> Ok (Float ((x +. y) /. 2.0))
             | _ -> Error (Arity "average needs two numbers"))
          | _ -> Error (Arity "average needs two arguments")) )
  ]
;;

(** [read_print] is the operation pair of 5.1.1's Actions: [read]
    produces a value from the machine's input queue, [print] is the
    action of rendering a register's contents into [output], in order.
    The book leaves the implementation of reading and printing to the
    surrounding system; here [read] fails with a typed error when the
    queue is exhausted, which is what stops a driver-loop machine in a
    pinned transcript. *)
let read_print ~inputs ~output =
  [ ( "read"
    , Value_op
        (function
          | [] ->
            (match Queue.take_opt inputs with
             | Some v -> Ok v
             | None -> Error (Op_failed "read: the input is exhausted"))
          | _ -> Error (Arity "read takes no inputs")) )
  ; ( "print"
    , Action_op
        (function
          | [ v ] ->
            output := !output @ [ value_to_string v ];
            Ok ()
          | _ -> Error (Arity "print takes one input")) )
  ]
;;
