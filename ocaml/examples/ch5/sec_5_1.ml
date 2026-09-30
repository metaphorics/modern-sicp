(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 5.1 *)

let ( let* ) = Result.bind

module Eval_error = Sicp_common.Eval_error

type error = Eval_error.t

type 'w source =
  | Const of 'w
  | Reg of string
  | Label_ref of string

type 'w instruction =
  | Label of string
  | Assign of string * 'w source
  | Assign_op of string * string * 'w source list
  | Test of string * 'w source list
  | Branch of string
  | Goto of string
  | Goto_reg of string
  | Save of string
  | Restore of string
  | Perform of string * 'w source list

type 'w op =
  | Value_op of ('w list -> ('w, error) result)
  | Test_op of ('w list -> (bool, error) result)
  | Action_op of ('w list -> (unit, error) result)

type 'w words =
  { label : string -> 'w
  ; to_label : 'w -> string option
  ; show : 'w -> string
  ; unassigned : 'w
  }

type 'w program =
  { code : 'w instruction array
  ; labels : (string * int) list
  }

let bad detail = Error (Eval_error.Bad_instruction detail)

let assemble controller =
  let rec go index code labels = function
    | [] -> Ok { code = Array.of_list (List.rev code); labels = List.rev labels }
    | Label name :: rest ->
      if List.mem_assoc name labels
      then bad ("label " ^ name ^ " is defined twice")
      else go index code ((name, index) :: labels) rest
    | instruction :: rest -> go (index + 1) (instruction :: code) labels rest
  in
  go 0 [] [] controller
;;

let source_registers = function
  | Reg r -> [ r ]
  | Const _ | Label_ref _ -> []
;;

let instruction_registers = function
  | Label _ | Branch _ | Goto _ -> []
  | Assign (target, source) -> target :: source_registers source
  | Assign_op (target, _, sources) -> target :: List.concat_map source_registers sources
  | Test (_, sources) | Perform (_, sources) -> List.concat_map source_registers sources
  | Goto_reg r | Save r | Restore r -> [ r ]
;;

let source_to_string show = function
  | Const w -> "Const (" ^ show w ^ ")"
  | Reg r -> Printf.sprintf "Reg %S" r
  | Label_ref l -> Printf.sprintf "Label_ref %S" l
;;

let instruction_to_string show instruction =
  let sources ss = "[" ^ String.concat "; " (List.map (source_to_string show) ss) ^ "]" in
  match instruction with
  | Label l -> Printf.sprintf "Label %S" l
  | Assign (t, s) -> Printf.sprintf "Assign (%S, %s)" t (source_to_string show s)
  | Assign_op (t, op, ss) -> Printf.sprintf "Assign_op (%S, %S, %s)" t op (sources ss)
  | Test (op, ss) -> Printf.sprintf "Test (%S, %s)" op (sources ss)
  | Branch l -> Printf.sprintf "Branch %S" l
  | Goto l -> Printf.sprintf "Goto %S" l
  | Goto_reg r -> Printf.sprintf "Goto_reg %S" r
  | Save r -> Printf.sprintf "Save %S" r
  | Restore r -> Printf.sprintf "Restore %S" r
  | Perform (op, ss) -> Printf.sprintf "Perform (%S, %s)" op (sources ss)
;;

(* An operand resolved at assembly time: registers become indices. *)
type 'w operand =
  | Word of 'w
  | Slot of int

(* The execution form of one instruction, every name resolved. *)
type 'w executable =
  | X_assign of int * 'w operand
  | X_compute of int * ('w list -> ('w, error) result) * 'w operand list
  | X_test of ('w list -> (bool, error) result) * 'w operand list
  | X_branch of int
  | X_goto of int
  | X_goto_reg of int
  | X_save of int
  | X_restore of int * string
  | X_perform of ('w list -> (unit, error) result) * 'w operand list

type 'w machine =
  { words : 'w words
  ; names : string array
  ; contents : 'w array
  ; source : 'w program
  ; executable : 'w executable array
  ; label_table : (string, int) Hashtbl.t
  ; pc_ref : int ref
  ; flag : bool option ref
  ; stack : 'w list ref
  ; depth : int ref
  ; pushes : int ref
  ; max_depth : int ref
  ; count : int ref
  }

let make ~words ~registers ~operations controller =
  let* program = assemble controller in
  let names = Array.of_list registers in
  let slot name =
    let rec find i =
      if i >= Array.length names
      then Error (Eval_error.Unknown_register name)
      else if String.equal names.(i) name
      then Ok i
      else find (i + 1)
    in
    find 0
  in
  let* () =
    let rec distinct = function
      | [] -> Ok ()
      | r :: rest ->
        if List.mem r rest
        then bad ("register " ^ r ^ " is declared twice")
        else distinct rest
    in
    distinct registers
  in
  let label_table = Hashtbl.create 16 in
  List.iter (fun (name, index) -> Hashtbl.replace label_table name index) program.labels;
  let address name =
    match Hashtbl.find_opt label_table name with
    | Some index -> Ok index
    | None -> Error (Eval_error.Unknown_label name)
  in
  let operand = function
    | Const w -> Ok (Word w)
    | Reg r -> Result.map (fun i -> Slot i) (slot r)
    | Label_ref l -> Result.map (fun _ -> Word (words.label l)) (address l)
  in
  let operands sources =
    List.fold_right
      (fun s acc ->
         let* acc = acc in
         let* o = operand s in
         Ok (o :: acc))
      sources
      (Ok [])
  in
  let operation name =
    match List.assoc_opt name operations with
    | Some op -> Ok op
    | None -> Error (Eval_error.Unknown_operation name)
  in
  let wrong_kind name wanted = bad ("operation " ^ name ^ " is not " ^ wanted) in
  let translate = function
    | Label _ -> bad "a label is not executable"
    | Assign (target, source) ->
      let* t = slot target in
      let* o = operand source in
      Ok (X_assign (t, o))
    | Assign_op (target, name, sources) ->
      let* t = slot target in
      let* os = operands sources in
      (match operation name with
       | Ok (Value_op f) -> Ok (X_compute (t, f, os))
       | Ok _ -> wrong_kind name "a value operation"
       | Error e -> Error e)
    | Test (name, sources) ->
      let* os = operands sources in
      (match operation name with
       | Ok (Test_op f) -> Ok (X_test (f, os))
       | Ok _ -> wrong_kind name "a test operation"
       | Error e -> Error e)
    | Branch l -> Result.map (fun i -> X_branch i) (address l)
    | Goto l -> Result.map (fun i -> X_goto i) (address l)
    | Goto_reg r -> Result.map (fun i -> X_goto_reg i) (slot r)
    | Save r -> Result.map (fun i -> X_save i) (slot r)
    | Restore r -> Result.map (fun i -> X_restore (i, r)) (slot r)
    | Perform (name, sources) ->
      let* os = operands sources in
      (match operation name with
       | Ok (Action_op f) -> Ok (X_perform (f, os))
       | Ok _ -> wrong_kind name "an action"
       | Error e -> Error e)
  in
  let* executable =
    Array.fold_right
      (fun i acc ->
         let* acc = acc in
         let* x = translate i in
         Ok (x :: acc))
      program.code
      (Ok [])
  in
  Ok
    { words
    ; names
    ; contents = Array.make (Array.length names) words.unassigned
    ; source = program
    ; executable = Array.of_list executable
    ; label_table
    ; pc_ref = ref 0
    ; flag = ref None
    ; stack = ref []
    ; depth = ref 0
    ; pushes = ref 0
    ; max_depth = ref 0
    ; count = ref 0
    }
;;

let find_slot m name =
  let rec find i =
    if i >= Array.length m.names
    then Error (Eval_error.Unknown_register name)
    else if String.equal m.names.(i) name
    then Ok i
    else find (i + 1)
  in
  find 0
;;

let set_register m name w =
  let* i = find_slot m name in
  m.contents.(i) <- w;
  Ok ()
;;

let get_register m name =
  let* i = find_slot m name in
  Ok m.contents.(i)
;;

let registers m = Array.to_list m.names
let program m = m.source
let pc m = !(m.pc_ref)

let value m = function
  | Word w -> w
  | Slot i -> m.contents.(i)
;;

let values m operands = List.map (value m) operands

let step m =
  let pc = !(m.pc_ref) in
  if pc >= Array.length m.executable
  then Ok false
  else (
    incr m.count;
    let next () =
      m.pc_ref := pc + 1;
      Ok true
    in
    match m.executable.(pc) with
    | X_assign (t, o) ->
      m.contents.(t) <- value m o;
      next ()
    | X_compute (t, f, os) ->
      let* w = f (values m os) in
      m.contents.(t) <- w;
      next ()
    | X_test (f, os) ->
      let* b = f (values m os) in
      m.flag := Some b;
      next ()
    | X_branch target ->
      (match !(m.flag) with
       | None -> Error Eval_error.Branch_without_test
       | Some true ->
         m.pc_ref := target;
         Ok true
       | Some false -> next ())
    | X_goto target ->
      m.pc_ref := target;
      Ok true
    | X_goto_reg r ->
      let w = m.contents.(r) in
      (match m.words.to_label w with
       | None -> bad ("goto through " ^ m.names.(r) ^ ", which holds " ^ m.words.show w)
       | Some l ->
         (match Hashtbl.find_opt m.label_table l with
          | Some target ->
            m.pc_ref := target;
            Ok true
          | None -> Error (Eval_error.Unknown_label l)))
    | X_save r ->
      m.stack := m.contents.(r) :: !(m.stack);
      incr m.pushes;
      incr m.depth;
      if !(m.depth) > !(m.max_depth) then m.max_depth := !(m.depth);
      next ()
    | X_restore (r, name) ->
      (match !(m.stack) with
       | [] -> bad ("restore " ^ name ^ " from an empty stack")
       | w :: rest ->
         m.stack := rest;
         decr m.depth;
         m.contents.(r) <- w;
         next ())
    | X_perform (f, os) ->
      let* () = f (values m os) in
      next ())
;;

let rec start m =
  let* running = step m in
  if running then start m else Ok ()
;;

let goto_label m l =
  match Hashtbl.find_opt m.label_table l with
  | Some target ->
    m.pc_ref := target;
    Ok ()
  | None -> Error (Eval_error.Unknown_label l)
;;

let initialize_stack m =
  m.stack := [];
  m.depth := 0;
  m.pushes := 0;
  m.max_depth := 0
;;

let restart m =
  m.pc_ref := 0;
  m.flag := None;
  m.count := 0;
  initialize_stack m
;;

let stack_statistics m = !(m.pushes), !(m.max_depth)
let stack_depth m = !(m.depth)
let executed m = !(m.count)

type value =
  | Int of int
  | Float of float
  | Bool of bool
  | Str of string
  | Addr of string
  | Unassigned

let value_to_string = function
  | Int n -> string_of_int n
  | Float f -> string_of_float f
  | Bool b -> string_of_bool b
  | Str s -> Printf.sprintf "%S" s
  | Addr l -> l
  | Unassigned -> "unassigned"
;;

let value_words =
  { label = (fun l -> Addr l)
  ; to_label =
      (function
        | Addr l -> Some l
        | _ -> None)
  ; show = value_to_string
  ; unassigned = Unassigned
  }
;;

let type_error name args =
  Error
    (Eval_error.Type_error
       (name ^ " cannot take " ^ String.concat ", " (List.map value_to_string args)))
;;

let int2 name f =
  ( name
  , Value_op
      (function
        | [ Int a; Int b ] -> f a b
        | args -> type_error name args) )
;;

let float2 name f =
  ( name
  , Value_op
      (function
        | [ Float a; Float b ] -> Ok (Float (f a b))
        | args -> type_error name args) )
;;

(* Each side compares in its own type: the float operators are the IEEE
   ones, not [compare], which is a total order -- [compare nan nan = 0]
   would make [=] true and [compare nan 1. < 0] would make [<] true,
   while native OCaml answers false for both. *)
let test2 name int_op float_op =
  ( name
  , Test_op
      (function
        | [ Int a; Int b ] -> Ok (int_op a b)
        | [ Float a; Float b ] -> Ok (float_op a b)
        | args -> type_error name args) )
;;

let arith_operations =
  [ int2 "+" (fun a b -> Ok (Int (a + b)))
  ; int2 "-" (fun a b -> Ok (Int (a - b)))
  ; int2 "*" (fun a b -> Ok (Int (a * b)))
  ; int2 "/" (fun a b ->
      if b = 0 then Error Eval_error.Division_by_zero else Ok (Int (a / b)))
  ; int2 "rem" (fun a b ->
      if b = 0 then Error Eval_error.Division_by_zero else Ok (Int (a mod b)))
  ; float2 "+." ( +. )
  ; float2 "-." ( -. )
  ; float2 "*." ( *. )
  ; float2 "/." ( /. )
  ; test2 "=" ( = ) ( = )
  ; test2 "<" ( < ) ( < )
  ; test2 ">" ( > ) ( > )
  ; test2 "<=" ( <= ) ( <= )
  ; test2 ">=" ( >= ) ( >= )
  ; ( "abs"
    , Value_op
        (function
          | [ Int a ] -> Ok (Int (abs a))
          | [ Float a ] -> Ok (Float (Float.abs a))
          | args -> type_error "abs" args) )
  ; ( "sqrt"
    , Value_op
        (function
          | [ Float a ] -> Ok (Float (sqrt a))
          | args -> type_error "sqrt" args) )
  ; ( "float_of_int"
    , Value_op
        (function
          | [ Int a ] -> Ok (Float (float_of_int a))
          | args -> type_error "float_of_int" args) )
  ]
;;

let print_operation emit =
  ( "print"
  , Action_op
      (function
        | [ v ] ->
          emit (value_to_string v ^ "\n");
          Ok ()
        | args ->
          Error (Eval_error.Arity_mismatch { expected = 1; given = List.length args })) )
;;

let make_machine ~registers ~operations ~controller =
  make ~words:value_words ~registers ~operations controller
;;

let run ~registers ~operations ~inputs ~controller result =
  let* m = make_machine ~registers ~operations ~controller in
  let* () =
    List.fold_left
      (fun acc (name, v) ->
         let* () = acc in
         set_register m name v)
      (Ok ())
      inputs
  in
  let* () = start m in
  get_register m result
;;
