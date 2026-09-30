(* SPDX-License-Identifier: GPL-3.0-only *)

let ( let* ) = Result.bind

module Eval_error = Sicp_common.Eval_error
module Data = Sicp_common.Constructor_data
module M = Sec_5_1

type error = Sec_5_1.error

type value = Sec_5_1.value =
  | Int of int
  | Float of float
  | Bool of bool
  | Str of string
  | Addr of string
  | Unassigned

type machine =
  { simulator : value M.machine
  ; lines : string list ref
  }

let simulator m = m.simulator

let print_stack_statistics m =
  let pushes, depth = M.stack_statistics m.simulator in
  Printf.sprintf "total-pushes = %d maximum-depth = %d" pushes depth
;;

let make_machine ~registers ~operations ~controller =
  let lines = ref [] in
  let write line = lines := line :: !lines in
  let knot = ref None in
  let with_machine f =
    match !knot with
    | Some m -> Ok (f m)
    | None -> Error (Eval_error.Invalid_form "the machine is not assembled")
  in
  let section =
    [ ( "initialize-stack"
      , M.Action_op (fun _ -> with_machine (fun m -> M.initialize_stack m.simulator)) )
    ; ( "print-stack-statistics"
      , M.Action_op (fun _ -> with_machine (fun m -> write (print_stack_statistics m))) )
    ; ( "print"
      , M.Action_op
          (function
            | [ v ] ->
              write (M.value_to_string v);
              Ok ()
            | args ->
              Error (Eval_error.Arity_mismatch { expected = 1; given = List.length args }))
      )
    ]
  in
  let* simulator =
    M.make_machine ~registers ~operations:(operations @ section) ~controller
  in
  let m = { simulator; lines } in
  knot := Some m;
  Ok m
;;

let set_register m = M.set_register m.simulator
let get_register m = M.get_register m.simulator
let start m = M.start m.simulator
let transcript m = List.rev !(m.lines)

type op_type =
  | Int_type
  | Float_type
  | Bool_type
  | Unit_type

type fixture =
  { registers : string list
  ; operations : (string * op_type list * op_type) list
  ; inputs : (string * value) list
  ; controller : value Sec_5_1.instruction list
  }

(* {1 The fixture decoder} *)

let wanted what d = Error (Printf.sprintf "expected %s, found %s" what (Data.describe d))

let decode_list f = function
  | Data.List items ->
    List.fold_right
      (fun item acc ->
         let* acc = acc in
         let* v = f item in
         Ok (v :: acc))
      items
      (Ok [])
  | d -> wanted "a list" d
;;

let decode_string = function
  | Data.String s -> Ok s
  | d -> wanted "a string" d
;;

let decode_value = function
  | Data.Ctor ("Int", [ Data.Int n ]) -> Ok (Int n)
  | Data.Ctor ("Float", [ Data.Float f ]) -> Ok (Float f)
  | Data.Ctor ("Bool", [ Data.Ctor ("true", []) ]) -> Ok (Bool true)
  | Data.Ctor ("Bool", [ Data.Ctor ("false", []) ]) -> Ok (Bool false)
  | Data.Ctor ("Str", [ Data.String s ]) -> Ok (Str s)
  | d -> wanted "a machine word (Int, Float, Bool, Str)" d
;;

let decode_type = function
  | Data.Ctor ("Int_type", []) -> Ok Int_type
  | Data.Ctor ("Float_type", []) -> Ok Float_type
  | Data.Ctor ("Bool_type", []) -> Ok Bool_type
  | Data.Ctor ("Unit_type", []) -> Ok Unit_type
  | d -> wanted "an operation type" d
;;

let decode_source = function
  | Data.Ctor ("Const", [ v ]) -> Result.map (fun v -> M.Const v) (decode_value v)
  | Data.Ctor ("Reg", [ Data.String r ]) -> Ok (M.Reg r)
  | Data.Ctor ("Label_ref", [ Data.String l ]) -> Ok (M.Label_ref l)
  | d -> wanted "an operand (Const, Reg, Label_ref)" d
;;

let decode_instruction = function
  | Data.Ctor ("Label", [ Data.String l ]) -> Ok (M.Label l)
  | Data.Ctor ("Assign", [ Data.String t; s ]) ->
    Result.map (fun s -> M.Assign (t, s)) (decode_source s)
  | Data.Ctor ("Assign_op", [ Data.String t; Data.String op; ss ]) ->
    Result.map (fun ss -> M.Assign_op (t, op, ss)) (decode_list decode_source ss)
  | Data.Ctor ("Test", [ Data.String op; ss ]) ->
    Result.map (fun ss -> M.Test (op, ss)) (decode_list decode_source ss)
  | Data.Ctor ("Branch", [ Data.String l ]) -> Ok (M.Branch l)
  | Data.Ctor ("Goto", [ Data.String l ]) -> Ok (M.Goto l)
  | Data.Ctor ("Goto_reg", [ Data.String r ]) -> Ok (M.Goto_reg r)
  | Data.Ctor ("Save", [ Data.String r ]) -> Ok (M.Save r)
  | Data.Ctor ("Restore", [ Data.String r ]) -> Ok (M.Restore r)
  | Data.Ctor ("Perform", [ Data.String op; ss ]) ->
    Result.map (fun ss -> M.Perform (op, ss)) (decode_list decode_source ss)
  | d -> wanted "an instruction" d
;;

let decode_operation = function
  | Data.Tuple [ Data.String name; args; result ] ->
    let* args = decode_list decode_type args in
    let* result = decode_type result in
    Ok (name, args, result)
  | d -> wanted "an operation declaration (name, operand types, result type)" d
;;

let decode_input = function
  | Data.Tuple [ Data.String name; v ] -> Result.map (fun v -> name, v) (decode_value v)
  | d -> wanted "an input (register, word)" d
;;

let field fields name =
  match List.assoc_opt name fields with
  | Some d -> Ok d
  | None -> Error ("the machine fixture has no field " ^ name)
;;

let read_fixture ~filename text =
  let* d = Data.read ~filename text in
  match d with
  | Data.Ctor ("Machine", [ Data.Record fields ]) ->
    let known = [ "registers"; "operations"; "inputs"; "controller" ] in
    (match List.find_opt (fun (name, _) -> not (List.mem name known)) fields with
     | Some (name, _) -> Error ("the machine fixture has an unknown field " ^ name)
     | None ->
       let* registers =
         Result.bind (field fields "registers") (decode_list decode_string)
       in
       let* operations =
         Result.bind (field fields "operations") (decode_list decode_operation)
       in
       let* inputs = Result.bind (field fields "inputs") (decode_list decode_input) in
       let* controller =
         Result.bind (field fields "controller") (decode_list decode_instruction)
       in
       Ok { registers; operations; inputs; controller })
  | d -> wanted "Machine { registers; operations; inputs; controller }" d
;;

(* {1 Running a fixture} *)

let has_type ty v =
  match ty, v with
  | Int_type, Int _ | Float_type, Float _ | Bool_type, Bool _ -> true
  | _ -> false
;;

let check_operands name types args =
  if List.length types <> List.length args
  then
    Error
      (Eval_error.Arity_mismatch
         { expected = List.length types; given = List.length args })
  else if List.for_all2 has_type types args
  then Ok ()
  else
    Error
      (Eval_error.Type_error
         (name
          ^ " was declared over other operand types than "
          ^ String.concat ", " (List.map M.value_to_string args)))
;;

let declared_operation emit (name, types, result) =
  let guard f args =
    let* () = check_operands name types args in
    f args
  in
  match name, result with
  | "print", Unit_type ->
    Ok
      ( name
      , M.Action_op
          (guard (function
             | [ v ] ->
               emit (M.value_to_string v ^ "\n");
               Ok ()
             | args ->
               Error
                 (Eval_error.Arity_mismatch { expected = 1; given = List.length args })))
      )
  | _ ->
    (match List.assoc_opt name M.arith_operations, result with
     | Some (M.Test_op f), Bool_type -> Ok (name, M.Test_op (guard f))
     | Some (M.Value_op f), (Int_type | Float_type) ->
       Ok
         ( name
         , M.Value_op
             (guard (fun args ->
                let* v = f args in
                if has_type result v
                then Ok v
                else
                  Error
                    (Eval_error.Type_error (name ^ " answered another type than declared"))))
         )
     | Some _, _ ->
       Error
         (Eval_error.Bad_instruction (name ^ " is declared with the wrong result type"))
     | None, _ -> Error (Eval_error.Unknown_operation name))
;;

let run_fixture ~emit fixture =
  let* operations =
    List.fold_right
      (fun declaration acc ->
         let* acc = acc in
         let* op = declared_operation emit declaration in
         Ok (op :: acc))
      fixture.operations
      (Ok [])
  in
  let* m =
    M.make_machine ~registers:fixture.registers ~operations ~controller:fixture.controller
  in
  let* () =
    List.fold_left
      (fun acc (name, v) ->
         let* () = acc in
         M.set_register m name v)
      (Ok ())
      fixture.inputs
  in
  let* () = M.start m in
  List.fold_left
    (fun acc name ->
       let* () = acc in
       let* v = M.get_register m name in
       emit (name ^ ": " ^ M.value_to_string v ^ "\n");
       Ok ())
    (Ok ())
    fixture.registers
;;
