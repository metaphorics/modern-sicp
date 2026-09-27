(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** The list-structure memory of section 5.3: the typed word vector of
    5.3.1 with its free pointer and allocation path, register machines
    whose words index the vector, and the stop-and-copy collector of
    5.3.2 -- the book's controller listing, run verbatim by a machine
    built from that text.

    The book's [vector-ref]/[vector-set!] are the semispace arrays'
    reads and writes; [cons] allocates at the free pointer; [car],
    [cdr], [set-car!], and [set-cdr!] go through the index part of a
    pair pointer. Save and restore are the 5.3.1 expansions -- a cons
    onto [the-stack] and a car/cdr off it -- so the stack is ordinary
    list structure and the collector reaches it through the stack
    register. Before a collection, the machine's registers are stored
    in a pre-allocated list in the working semispace's reserved strip,
    exactly the book's arrangement of the [root] register; after the
    controller's [gc-flip] the relocated root list is walked back to
    hand every register its forwarded word. *)

module S51 = Sec_5_1

let ( >>= ) = Result.bind

(** {1:words Typed words} *)

type word =
  | Pair of int
  | Num of int
  | Sym of string
  | Empty
  | Broken_heart
  | Bool of bool
  | Vec of int * bool
  | Lab of string

let word_to_string = function
  | Pair i -> "p" ^ string_of_int i
  | Num n -> "n" ^ string_of_int n
  | Sym s -> s
  | Empty -> "e0"
  | Broken_heart -> "broken-heart"
  | Bool b -> if b then "true" else "false"
  | Vec (k, cars) -> if cars then "cars-" ^ string_of_int k else "cdrs-" ^ string_of_int k
  | Lab l -> l
;;

let equal_word (a : word) (b : word) = a = b

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

(** {1:memory The word vector and its allocation path} *)

type memory =
  { size : int
    (** data cells per semispace; the top [root_capacity] cells of each
          semispace are the reserved strip the collector's pre-allocated
          root list occupies *)
  ; root_capacity : int
  ; spaces : (word array * word array) array
  ; mutable free : int
  ; mutable working : int
  ; mutable collections : int
  ; mutable trace : string list
  }

let make_memory ~size ~root_capacity ~free =
  let blank () = Array.make size Empty in
  { size
  ; root_capacity
  ; spaces = [| blank (), blank (); blank (), blank () |]
  ; free
  ; working = 0
  ; collections = 0
  ; trace = []
  }
;;

(** [data_limit mem] is the allocator's ceiling: the free pointer never
    enters the reserved strip. *)
let data_limit mem = mem.size - mem.root_capacity

let cars mem k = fst mem.spaces.(k)
let cdrs mem k = snd mem.spaces.(k)
let free_word mem = Pair mem.free
let collections mem = mem.collections
let working mem = mem.working
let allocation_trace mem = mem.trace
let record mem line = mem.trace <- mem.trace @ [ line ]

let index_of = function
  | Pair i when i >= 0 -> Ok i
  | Num n when n >= 0 -> Ok n
  | w -> Error (Op_failed ("not a memory index: " ^ word_to_string w))
;;

(** {2:cells The vector primitives} *)

(** The book's [vector-ref] and [vector-set!] over one semispace: a
    bounds-checked read or write of the cars or the cdrs array. *)
let read_cars mem k i =
  if i >= 0 && i < Array.length (cars mem k)
  then Ok (cars mem k).(i)
  else Error (Op_failed "memory index out of range")
;;

let read_cdrs mem k i =
  if i >= 0 && i < Array.length (cdrs mem k)
  then Ok (cdrs mem k).(i)
  else Error (Op_failed "memory index out of range")
;;

let store_cars mem k i v =
  if i >= 0 && i < Array.length (cars mem k)
  then (
    (cars mem k).(i) <- v;
    Ok ())
  else Error (Op_failed "memory index out of range")
;;

let store_cdrs mem k i v =
  if i >= 0 && i < Array.length (cdrs mem k)
  then (
    (cdrs mem k).(i) <- v;
    Ok ())
  else Error (Op_failed "memory index out of range")
;;

let cons mem a d =
  if mem.free >= mem.size
  then Error (Op_failed "the memory is exhausted")
  else (
    let i = mem.free in
    let here = word_to_string (Pair i) in
    store_cars mem mem.working i a
    >>= fun () ->
    store_cdrs mem mem.working i d
    >>= fun () ->
    mem.free <- i + 1;
    record
      mem
      ("cons -> "
       ^ here
       ^ " = ("
       ^ word_to_string a
       ^ " "
       ^ word_to_string d
       ^ "); free "
       ^ here
       ^ " -> "
       ^ word_to_string (Pair mem.free));
    Ok (Pair i))
;;

let car mem = function
  | Pair i -> read_cars mem mem.working i
  | w -> Error (Op_failed ("car: not a pair: " ^ word_to_string w))
;;

let cdr mem = function
  | Pair i -> read_cdrs mem mem.working i
  | w -> Error (Op_failed ("cdr: not a pair: " ^ word_to_string w))
;;

let set_car mem w v =
  match w with
  | Pair i -> store_cars mem mem.working i v
  | _ -> Error (Op_failed ("set-car!: not a pair: " ^ word_to_string w))
;;

let set_cdr mem w v =
  match w with
  | Pair i -> store_cdrs mem mem.working i v
  | _ -> Error (Op_failed ("set-cdr!: not a pair: " ^ word_to_string w))
;;

let is_pair = function
  | Pair _ -> true
  | _ -> false
;;

let is_null = function
  | Empty -> true
  | _ -> false
;;

let is_symbol = function
  | Sym _ -> true
  | _ -> false
;;

let is_number = function
  | Num _ -> true
  | _ -> false
;;

let pointer_to_pair = is_pair

let is_broken_heart = function
  | Broken_heart -> true
  | _ -> false
;;

(** {1:render The drawings} *)

let dump mem =
  let cs, ds = mem.spaces.(mem.working) in
  let n = Array.length cs in
  let width i =
    max
      (String.length (string_of_int i))
      (max
         (String.length (word_to_string cs.(i)))
         (String.length (word_to_string ds.(i))))
    + 2
  in
  let widths = Array.init n width in
  let pad w s = s ^ String.make (max 1 (w - String.length s)) ' ' in
  let row name cell =
    String.trim
      (name
       ^ String.concat "" (Array.to_list (Array.mapi (fun i w -> pad w (cell i)) widths))
      )
  in
  String.concat
    "\n"
    [ row "index    " (fun i -> string_of_int i)
    ; row "the-cars " (fun i -> word_to_string cs.(i))
    ; row "the-cdrs " (fun i -> word_to_string ds.(i))
    ]
;;

let rec write_result mem w =
  match w with
  | Pair _ ->
    (match cdr mem w with
     | Ok (Pair _) | Ok Empty -> write_items mem w >>= fun items -> Ok ("(" ^ items ^ ")")
     | Ok d ->
       car mem w
       >>= fun a ->
       write_result mem d
       >>= fun d -> write_result mem a >>= fun a -> Ok ("(" ^ a ^ " . " ^ d ^ ")")
     | Error _ -> Ok (word_to_string w))
  | Num n -> Ok (string_of_int n)
  | Sym s -> Ok s
  | Empty -> Ok "()"
  | Broken_heart -> Ok "broken-heart"
  | Bool b -> Ok (string_of_bool b)
  | Vec _ | Lab _ -> Ok (word_to_string w)

and write_items mem w =
  car mem w
  >>= fun a ->
  write_result mem a
  >>= fun a ->
  cdr mem w
  >>= fun rest ->
  match rest with
  | Empty -> Ok a
  | Pair _ -> write_items mem rest >>= fun r -> Ok (a ^ " " ^ r)
  | d -> write_result mem d >>= fun d -> Ok (a ^ " . " ^ d)
;;

let write mem w =
  match write_result mem w with
  | Ok s -> s
  | Error e -> "unreadable: " ^ error_to_string e
;;

(** {1:machine The word machines} *)

type op =
  | Value_op of (word list -> (word, error) result)
  | Action_op of (word list -> (unit, error) result)

type inst =
  { text : S51.instruction
  ; mutable exec : unit -> (unit, error) result
  }

type machine =
  { mutable names : string list (** declaration order; the collector's root order *)
  ; regs : (string, word ref) Hashtbl.t
  ; mem : memory
  ; mutable ops : (string * op) list
  ; const : S51.value -> (word, error) result
  ; mutable insts : inst array
  ; mutable labels : (string * int) list
  ; pc : int ref
  ; flag : word ref
  ; pushes : int ref
  ; depth : int ref
  ; max_depth : int ref
  ; output : string list ref
  ; mutable collect : (unit -> (unit, error) result) option
  }

(** [word_of_value] is the plain reading of a constant: numbers, words
    for symbols, and the [broken-heart] constant naming the moved tag. *)
let word_of_value = function
  | S51.Int n -> Ok (Num n)
  | S51.Bool b -> Ok (Bool b)
  | S51.Symbol "broken-heart" -> Ok Broken_heart
  | S51.Symbol s -> Ok (Sym s)
  | S51.Label l -> Ok (Lab l)
  | S51.Float _ ->
    Error (Bad_instruction "the word constants of 5.3 are numbers, symbols, and booleans")
;;

let register_of m name =
  match Hashtbl.find_opt m.regs name with
  | Some r -> Ok r
  | None -> Error (Unknown_register name)
;;

let set_register m name w =
  register_of m name
  >>= fun r ->
  r := w;
  Ok ()
;;

let get_register m name = register_of m name >>= fun r -> Ok !r

let lookup_label m name =
  match List.assoc_opt name m.labels with
  | Some i -> Ok i
  | None -> Error (Unknown_label name)
;;

let lookup_prim name ops =
  match List.assoc_opt name ops with
  | Some o -> Ok o
  | None -> Error (Unknown_operation name)
;;

let append_line m line = m.output := !(m.output) @ [ line ]

let statistics m =
  "total-pushes = "
  ^ string_of_int !(m.pushes)
  ^ " maximum-depth = "
  ^ string_of_int !(m.max_depth)
;;

(** {2:defaults The operations the memory provides} *)

(** [machine_cons m a d] is the allocation path: it allocates while the
    free pointer stays under the strip ceiling and answers the typed
    exhausted-memory failure once the free pointer reaches it. The
    retry -- and the collection it needs -- belongs to the instruction
    that asked for the cell, which re-executes and re-reads its
    register operands after the collection, exactly as the book's
    expansion of [cons] re-reads its registers. *)
let machine_cons m a d =
  if m.mem.free < data_limit m.mem
  then cons m.mem a d
  else Error (Op_failed "the memory is exhausted")
;;

let push_stack m v =
  register_of m "the-stack"
  >>= fun stk ->
  machine_cons m v !stk
  >>= fun w ->
  stk := w;
  m.pushes := !(m.pushes) + 1;
  m.depth := !(m.depth) + 1;
  if !(m.depth) > !(m.max_depth) then m.max_depth := !(m.depth);
  Ok ()
;;

let pop_stack m name =
  register_of m "the-stack"
  >>= fun stk ->
  match !stk with
  | Pair _ ->
    car m.mem !stk
    >>= fun top ->
    cdr m.mem !stk
    >>= fun rest ->
    stk := rest;
    m.depth := !(m.depth) - 1;
    Ok top
  | _ -> Error (Stack_underflow name)
;;

(** [default_operations m] installs the 5.3.1 list-structure operations
    over the machine's memory: the selectors and mutators go through
    the index part of a pair pointer, [cons] through the free pointer
    with the collector armed, and the predicates check only the type
    field. *)
let default_operations m =
  let mem = m.mem in
  let one name f =
    Value_op
      (function
        | [ w ] -> f w
        | _ -> Error (Arity (name ^ " expects one word")))
  in
  let two name f =
    Value_op
      (function
        | [ a; b ] -> f a b
        | _ -> Error (Arity (name ^ " expects two words")))
  in
  [ ( "initialize-stack"
    , Action_op
        (fun _ ->
          register_of m "the-stack"
          >>= fun stk ->
          stk := Empty;
          m.depth := 0;
          Ok ()) )
  ; ( "print-stack-statistics"
    , Action_op
        (fun _ ->
          append_line m (statistics m);
          Ok ()) )
  ; "cons", two "cons" (fun a d -> machine_cons m a d)
  ; "car", one "car" (car mem)
  ; "cdr", one "cdr" (cdr mem)
  ; ( "set-car!"
    , Action_op
        (function
          | [ p; v ] -> set_car mem p v
          | _ -> Error (Arity "set-car! expects a pair and a value")) )
  ; ( "set-cdr!"
    , Action_op
        (function
          | [ p; v ] -> set_cdr mem p v
          | _ -> Error (Arity "set-cdr! expects a pair and a value")) )
  ; "eq?", two "eq?" (fun a b -> Ok (Bool (equal_word a b)))
  ; ( "="
    , two "=" (fun a b ->
        match a, b with
        | Num x, Num y -> Ok (Bool (x = y))
        | _ -> Error (Arity "= expects two numbers")) )
  ; ( "+"
    , two "+" (fun a b ->
        match a, b with
        | Num x, Num y -> Ok (Num (x + y))
        | _ -> Error (Arity "+ expects two numbers")) )
  ; ( "-"
    , two "-" (fun a b ->
        match a, b with
        | Num x, Num y -> Ok (Num (x - y))
        | _ -> Error (Arity "- expects two numbers")) )
  ; ( "not"
    , one "not" (function
        | Bool b -> Ok (Bool (not b))
        | _ -> Error (Arity "not expects a boolean")) )
  ; "null?", one "null?" (fun w -> Ok (Bool (is_null w)))
  ; "pair?", one "pair?" (fun w -> Ok (Bool (is_pair w)))
  ; "symbol?", one "symbol?" (fun w -> Ok (Bool (is_symbol w)))
  ; "number?", one "number?" (fun w -> Ok (Bool (is_number w)))
  ]
;;

(** {2:execution Execution procedures} *)

let advance m =
  incr m.pc;
  Ok ()
;;

let rec run_all = function
  | [] -> Ok []
  | p :: ps -> p () >>= fun w -> run_all ps >>= fun ws -> Ok (w :: ws)
;;

let operand_proc m exp =
  match exp with
  | S51.Reg r -> register_of m r >>= fun r -> Ok (fun () -> Ok !r)
  | S51.Const v -> m.const v >>= fun w -> Ok (fun () -> Ok w)
  | S51.Label_source l -> lookup_label m l >>= fun _ -> Ok (fun () -> Ok (Lab l))
;;

let rec build_procs m = function
  | [] -> Ok []
  | e :: es -> operand_proc m e >>= fun p -> build_procs m es >>= fun ps -> Ok (p :: ps)
;;

let operation_proc m name inputs =
  lookup_prim name m.ops
  >>= fun o -> build_procs m inputs >>= fun argprocs -> Ok (o, argprocs)
;;

let apply_value name o args =
  match o with
  | Value_op f -> f args
  | Action_op _ ->
    Error
      (Bad_instruction ("the operation " ^ name ^ " is an action and produces no value"))
;;

let apply_action name o args =
  match o with
  | Action_op f -> f args
  | Value_op _ ->
    Error
      (Bad_instruction
         ("the operation " ^ name ^ " produces a value; assign it, do not perform it"))
;;

let make_assign m target rhs =
  register_of m target
  >>= fun reg ->
  match rhs with
  | S51.Reg r ->
    register_of m r
    >>= fun src ->
    Ok
      (fun () ->
        reg := !src;
        advance m)
  | S51.Const v ->
    m.const v
    >>= fun w ->
    Ok
      (fun () ->
        reg := w;
        advance m)
  | S51.Label_source l ->
    lookup_label m l
    >>= fun _ ->
    Ok
      (fun () ->
        reg := Lab l;
        advance m)
;;

let make_assign_op m target name inputs =
  register_of m target
  >>= fun reg ->
  operation_proc m name inputs
  >>= fun (o, argprocs) ->
  Ok
    (fun () ->
      run_all argprocs
      >>= fun args ->
      apply_value name o args
      >>= fun w ->
      reg := w;
      advance m)
;;

let make_test m name inputs =
  operation_proc m name inputs
  >>= fun (o, argprocs) ->
  Ok
    (fun () ->
      run_all argprocs
      >>= fun args ->
      apply_value name o args
      >>= fun w ->
      match w with
      | Bool b ->
        m.flag := Bool b;
        advance m
      | _ -> Error (Bad_instruction "a test answers true or false"))
;;

let make_branch m label =
  lookup_label m label
  >>= fun idx ->
  Ok
    (fun () ->
      match !(m.flag) with
      | Bool true ->
        m.pc := idx;
        Ok ()
      | Bool false -> advance m
      | _ -> Error Branch_without_test)
;;

let make_goto_label m label =
  lookup_label m label
  >>= fun idx ->
  Ok
    (fun () ->
      m.pc := idx;
      Ok ())
;;

let make_goto_reg m reg =
  register_of m reg
  >>= fun r ->
  Ok
    (fun () ->
      match !r with
      | Lab l ->
        lookup_label m l
        >>= fun idx ->
        m.pc := idx;
        Ok ()
      | w ->
        Error (Bad_instruction ("a goto register holds a label, not " ^ word_to_string w)))
;;

(** [make_save m r] is the 5.3.1 expansion [(assign the-stack (op cons)
    (reg r) (reg the-stack))] with the monitored counters advanced. *)
let make_save m r =
  register_of m r >>= fun reg -> Ok (fun () -> push_stack m !reg >>= fun () -> advance m)
;;

(** [make_restore m r] is [(assign r (op car) (reg the-stack))] and
    [(assign the-stack (op cdr) (reg the-stack))]. *)
let make_restore m r =
  register_of m r
  >>= fun reg ->
  Ok
    (fun () ->
      pop_stack m r
      >>= fun top ->
      reg := top;
      advance m)
;;

let make_perform m name inputs =
  operation_proc m name inputs
  >>= fun (o, argprocs) ->
  Ok
    (fun () ->
      run_all argprocs >>= fun args -> apply_action name o args >>= fun () -> advance m)
;;

let make_execution_procedure m text =
  match text with
  | S51.Assign (target, rhs) -> make_assign m target rhs
  | S51.Assign_op (target, name, inputs) -> make_assign_op m target name inputs
  | S51.Test (name, inputs) -> make_test m name inputs
  | S51.Branch label -> make_branch m label
  | S51.Goto_label label -> make_goto_label m label
  | S51.Goto_reg reg -> make_goto_reg m reg
  | S51.Save r -> make_save m r
  | S51.Restore r -> make_restore m r
  | S51.Perform (name, inputs) -> make_perform m name inputs
;;

let update_insts m =
  let count = Array.length m.insts in
  let rec fill i =
    if i < count
    then (
      make_execution_procedure m m.insts.(i).text
      >>= fun exec ->
      m.insts.(i).exec <- exec;
      fill (i + 1))
    else Ok ()
  in
  fill 0
;;

(** {2:assembly The assembler} *)

let install_program m (program : S51.program) =
  m.insts <- Array.map (fun text -> { text; exec = (fun () -> Ok ()) }) program.code;
  m.labels <- program.labels;
  update_insts m
;;

let make_new_machine mem const =
  { names = []
  ; regs = Hashtbl.create 16
  ; mem
  ; const
  ; ops = []
  ; insts = [||]
  ; labels = []
  ; pc = ref 0
  ; flag = ref (Sym "*unassigned*")
  ; pushes = ref 0
  ; depth = ref 0
  ; max_depth = ref 0
  ; output = ref []
  ; collect = None
  }
;;

let allocate_register m name =
  if Hashtbl.mem m.regs name
  then Error (Bad_instruction ("the register " ^ name ^ " is allocated twice"))
  else (
    Hashtbl.replace m.regs name (ref (Sym "*unassigned*"));
    Ok ())
;;

let rec allocate_registers m = function
  | [] -> Ok ()
  | name :: rest -> allocate_register m name >>= fun () -> allocate_registers m rest
;;

let make_machine_gen const ~registers ~operations ~controller ~memory =
  let m = make_new_machine memory const in
  allocate_registers m registers
  >>= fun () ->
  allocate_register m "the-stack"
  >>= fun () ->
  m.names <- registers @ [ "the-stack" ];
  m.ops <- operations @ default_operations m;
  S51.parse_program controller >>= install_program m >>= fun () -> Ok m
;;

let make_machine ~registers ~operations ~controller ~memory =
  make_machine_gen word_of_value ~registers ~operations ~controller ~memory
;;

(** [start m] runs the machine from the first instruction until the
    sequence ends or an instruction fails. One failure of the
    allocation path -- the typed exhausted-memory error -- is retried:
    the armed collector runs once and the same instruction executes
    again, re-reading its register operands from the relocated heap;
    a second exhaustion fails for good. *)
let start m =
  m.pc := 0;
  let retried = ref false in
  let rec execute () =
    if !(m.pc) < Array.length m.insts
    then (
      match m.insts.(!(m.pc)).exec () with
      | Error (Op_failed message) when message = "the memory is exhausted" && not !retried
        ->
        (match m.collect with
         | Some collect ->
           retried := true;
           collect () >>= fun () -> execute ()
         | None -> Error (Op_failed message))
      | Ok () ->
        retried := false;
        execute ()
      | Error e ->
        retried := false;
        Error e)
    else Ok ()
  in
  execute ()
;;

let print_stack_statistics m = statistics m
let transcript m = !(m.output)

(** {1:collector The stop-and-copy collector} *)

(** The collector's controller, the book's 5.3.2 listings verbatim:
    [begin-garbage-collection], the [gc-loop] with [update-car] and
    [update-cdr], the [relocate-old-result-in-new] subroutine, and the
    [gc-flip] that swaps the semispaces by swapping the vector
    registers. *)
let gc_controller =
  {|(controller
 begin-garbage-collection
   (assign free (const 0))
   (assign scan (const 0))
   (assign old (reg root))
   (assign relocate-continue (label reassign-root))
   (goto (label relocate-old-result-in-new))
 reassign-root
   (assign root (reg new))
   (goto (label gc-loop))
 gc-loop
   (test (op =) (reg scan) (reg free))
   (branch (label gc-flip))
   (assign old (op vector-ref) (reg new-cars) (reg scan))
   (assign relocate-continue (label update-car))
   (goto (label relocate-old-result-in-new))
 update-car
   (perform (op vector-set!) (reg new-cars) (reg scan) (reg new))
   (assign old (op vector-ref) (reg new-cdrs) (reg scan))
   (assign relocate-continue (label update-cdr))
   (goto (label relocate-old-result-in-new))
 update-cdr
   (perform (op vector-set!) (reg new-cdrs) (reg scan) (reg new))
   (assign scan (op +) (reg scan) (const 1))
   (goto (label gc-loop))
 relocate-old-result-in-new
   (test (op pointer-to-pair?) (reg old))
   (branch (label pair))
   (assign new (reg old))
   (goto (reg relocate-continue))
 pair
   (assign oldcr (op vector-ref) (reg the-cars) (reg old))
   (test (op broken-heart?) (reg oldcr))
   (branch (label already-moved))
   (assign new (reg free))
   (assign free (op +) (reg free) (const 1))
   (perform (op vector-set!) (reg new-cars) (reg new) (reg oldcr))
   (assign oldcr (op vector-ref) (reg the-cdrs) (reg old))
   (perform (op vector-set!) (reg new-cdrs) (reg new) (reg oldcr))
   (perform (op vector-set!) (reg the-cars) (reg old) (const broken-heart))
   (perform (op vector-set!) (reg the-cdrs) (reg old) (reg new))
   (goto (reg relocate-continue))
 already-moved
   (assign new (op vector-ref) (reg the-cdrs) (reg old))
   (goto (reg relocate-continue))
 gc-flip
   (assign temp (reg the-cdrs))
   (assign the-cdrs (reg new-cdrs))
   (assign new-cdrs (reg temp))
   (assign temp (reg the-cars))
   (assign the-cars (reg new-cars))
   (assign new-cars (reg temp)))|}
;;

(** The collector machine's own operations: the two vector primitives,
    the scan/free comparison, the pointer increment, and the two
    low-level predicates the book's footnote names. *)
let collector_operations mem =
  let vector_ref = function
    | [ Vec (k, true); i ] -> index_of i >>= fun j -> read_cars mem k j
    | [ Vec (k, false); i ] -> index_of i >>= fun j -> read_cdrs mem k j
    | _ -> Error (Arity "vector-ref expects a vector and an index")
  in
  let vector_set = function
    | [ Vec (k, true); i; v ] -> index_of i >>= fun j -> store_cars mem k j v
    | [ Vec (k, false); i; v ] -> index_of i >>= fun j -> store_cdrs mem k j v
    | _ -> Error (Arity "vector-set! expects a vector, an index, and a value")
  in
  [ "vector-ref", Value_op vector_ref
  ; "vector-set!", Action_op vector_set
  ; ( "="
    , Value_op
        (function
          | [ Num a; Num b ] -> Ok (Bool (a = b))
          | [ Pair a; Pair b ] -> Ok (Bool (a = b))
          | _ -> Error (Arity "= expects two indices")) )
  ; ( "+"
    , Value_op
        (function
          | [ Num a; Num b ] -> Ok (Num (a + b))
          | [ Pair a; Num b ] -> Ok (Pair (a + b))
          | _ -> Error (Arity "+ expects an index and a number")) )
  ; ( "pointer-to-pair?"
    , Value_op
        (function
          | [ w ] -> Ok (Bool (pointer_to_pair w))
          | _ -> Error (Arity "pointer-to-pair? expects one word")) )
  ; ( "broken-heart?"
    , Value_op
        (function
          | [ w ] -> Ok (Bool (is_broken_heart w))
          | _ -> Error (Arity "broken-heart? expects one word")) )
  ]
;;

(** [collect app gc] is the book's arrangement around the controller:
    the machine's registers go into a pre-allocated list in the working
    semispace's reserved strip, the collector relocates from the list's
    head, and after the flip the relocated list is walked to hand every
    application register -- the stack register among them -- its
    forwarded word. *)
let collect app gc =
  let mem = app.mem in
  let roots = List.map (fun name -> name, !(Hashtbl.find app.regs name)) app.names in
  let n = List.length roots in
  if n > mem.root_capacity
  then Error (Op_failed "the root buffer is smaller than the register set")
  else if mem.free + n > mem.size
  then
    Error (Op_failed "the relocated data and the root list do not fit the new semispace")
  else if n = 0
  then (
    mem.free <- 0;
    mem.working <- 1 - mem.working;
    mem.collections <- mem.collections + 1;
    Ok ())
  else (
    (* The pre-allocated root list lives in the working semispace's
       reserved strip; its head is the collector's root. *)
    let spine i = mem.size - mem.root_capacity + i in
    List.iteri
      (fun i (_, w) ->
         (cars mem mem.working).(spine i) <- w;
         (cdrs mem mem.working).(spine i)
         <- (if i + 1 < n then Pair (spine (i + 1)) else Empty))
      roots;
    let other = 1 - mem.working in
    set_register gc "root" (Pair (spine 0))
    >>= fun () ->
    set_register gc "the-cars" (Vec (mem.working, true))
    >>= fun () ->
    set_register gc "the-cdrs" (Vec (mem.working, false))
    >>= fun () ->
    set_register gc "new-cars" (Vec (other, true))
    >>= fun () ->
    set_register gc "new-cdrs" (Vec (other, false))
    >>= fun () ->
    start gc
    >>= fun () ->
    (match get_register gc "free" with
     | Ok (Pair f) | Ok (Num f) -> Ok f
     | Ok w -> Error (Op_failed ("the collector left free = " ^ word_to_string w))
     | Error e -> Error e)
    >>= fun f ->
    (match get_register gc "the-cars" with
     | Ok (Vec (k, _)) -> Ok k
     | Ok w -> Error (Op_failed ("the collector left the-cars = " ^ word_to_string w))
     | Error e -> Error e)
    >>= fun k ->
    mem.free <- f;
    mem.working <- k;
    mem.collections <- mem.collections + 1;
    (match get_register gc "root" with
     | Ok (Pair j) -> Ok j
     | Ok w -> Error (Op_failed ("the collector left root = " ^ word_to_string w))
     | Error e -> Error e)
    >>= fun head ->
    let rec walk idx = function
      | [] -> Ok ()
      | (name, _) :: rest ->
        car mem (Pair idx)
        >>= fun w ->
        set_register app name w
        >>= fun () ->
        cdr mem (Pair idx)
        >>= fun next ->
        (match next with
         | Pair j -> walk j rest
         | Empty when rest = [] -> Ok ()
         | Empty -> Error (Op_failed "the root spine ended before the registers did")
         | w -> Error (Op_failed ("the root spine broke at " ^ word_to_string w)))
    in
    walk head roots)
;;

(** The collector's free and scan are pair pointers -- 5.3.1 gives free
    "a pair pointer containing the next available index" -- so the
    controller's zero constants name [p0], and the pointer kind then
    rides through [new] and the forwarding addresses. *)
let collector_const = function
  | S51.Int 0 -> Ok (Pair 0)
  | v -> word_of_value v
;;

(** [collect_garbage m] runs the collector now, whatever the free
    pointer -- the demonstration entry the prose uses to show the
    reconfiguration of Figure 5.15; the allocation path calls the same
    driver on exhaustion. *)
let collect_garbage m =
  match m.collect with
  | Some collect -> collect ()
  | None -> Error (Op_failed "no collector is attached to the machine")
;;

let attach_collector app =
  make_machine_gen
    collector_const
    ~registers:
      [ "root"
      ; "free"
      ; "scan"
      ; "old"
      ; "new"
      ; "oldcr"
      ; "relocate-continue"
      ; "the-cars"
      ; "the-cdrs"
      ; "new-cars"
      ; "new-cdrs"
      ; "temp"
      ]
    ~operations:(collector_operations app.mem)
    ~controller:gc_controller
    ~memory:app.mem
  >>= fun gc ->
  app.collect <- Some (fun () -> collect app gc);
  Ok ()
;;
