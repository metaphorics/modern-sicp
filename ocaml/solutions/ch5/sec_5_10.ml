(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.10: a new surface syntax for the machine language,
    isolated in its own reader. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2
module Ast = Sicp_common.Ast

(** The new syntax keeps S-expressions but strips the keywords the old
    syntax repeats: an operation call writes its operation in head
    position [(rem (reg a) (reg b))], a test writes [(test = (reg b)
    (const 0))], and branch and goto targets are bare names. *)

let rec all f = function
  | [] -> Ok []
  | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
;;

let symbol e =
  match Ast.view e with
  | Ast.Variable s -> Ok s
  | _ -> Error (Machine.Bad_instruction "a name is written as a symbol")
;;

(** [constant e] reads the old [const] argument kinds. *)
let constant e =
  match Ast.view e with
  | Ast.Int n -> Ok (Machine.Int n)
  | Ast.Float f -> Ok (Machine.Float f)
  | Ast.Bool b -> Ok (Machine.Bool b)
  | Ast.Variable s -> Ok (Machine.Symbol s)
  | _ -> Error (Machine.Bad_instruction "a constant is a number, a boolean, or a symbol")
;;

(** [operand e] reads the new operand forms: [(reg r)] or [(const c)].
    A bare name is a register, the new syntax's most common shorthand:
    [(= n 1)] reads as [(= (reg n) (const 1))]. *)
let operand e =
  match Ast.view e with
  | Ast.Application (op, [ arg ]) ->
    (match Ast.view op with
     | Ast.Variable "reg" -> symbol arg >>= fun r -> Ok (Machine.Reg r)
     | Ast.Variable "const" -> constant arg >>= fun v -> Ok (Machine.Const v)
     | _ ->
       Error (Machine.Bad_instruction "an operand is (reg r), (const c), or a bare name"))
  | Ast.Variable r -> Ok (Machine.Reg r)
  | _ ->
    Error (Machine.Bad_instruction "an operand is (reg r), (const c), or a bare name")
;;

let keywords =
  [ "assign"; "test"; "branch"; "goto"; "save"; "restore"; "perform"; "reg"; "const" ]
;;

(** [operation e] reads the new operation call: the head is the
    operation name unless it is a keyword. *)
let operation e =
  match Ast.view e with
  | Ast.Application (op, args) ->
    (match Ast.view op with
     | Ast.Variable name when not (List.mem name keywords) ->
       all operand args >>= fun inputs -> Ok (name, inputs)
     | _ -> Error (Machine.Bad_instruction "an operation call is (name operand ...)"))
  | _ -> Error (Machine.Bad_instruction "an operation call is (name operand ...)")
;;

(** [instruction e] reads one instruction of the new syntax. *)
let instruction e =
  match Ast.view e with
  | Ast.Application (op, args) ->
    (match Ast.view op with
     | Ast.Variable "assign" ->
       (match args with
        | [ target; value ] ->
          symbol target
          >>= fun r ->
          (match Ast.view value with
           | Ast.Application (vop, [ vlabel ]) ->
             (match Ast.view vop with
              | Ast.Variable "label" ->
                symbol vlabel >>= fun l -> Ok (Machine.Assign (r, Machine.Label_source l))
              | Ast.Variable "const" ->
                constant vlabel >>= fun v -> Ok (Machine.Assign (r, Machine.Const v))
              | Ast.Variable "reg" ->
                symbol vlabel >>= fun s -> Ok (Machine.Assign (r, Machine.Reg s))
              | _ ->
                operation value
                >>= fun (name, inputs) -> Ok (Machine.Assign_op (r, name, inputs)))
           | Ast.Application _ ->
             operation value
             >>= fun (name, inputs) -> Ok (Machine.Assign_op (r, name, inputs))
           | Ast.Variable s -> Ok (Machine.Assign (r, Machine.Reg s))
           | _ ->
             Error (Machine.Bad_instruction "an assign names its target and one source"))
        | _ -> Error (Machine.Bad_instruction "an assign names its target and one source"))
     | Ast.Variable "test" ->
       (match args with
        | name :: operands ->
          symbol name
          >>= fun n ->
          all operand operands >>= fun inputs -> Ok (Machine.Test (n, inputs))
        | [] -> Error (Machine.Bad_instruction "a test names one operation"))
     | Ast.Variable "branch" ->
       (match args with
        | [ target ] -> symbol target >>= fun l -> Ok (Machine.Branch l)
        | _ -> Error (Machine.Bad_instruction "a branch names one label"))
     | Ast.Variable "goto" ->
       (match args with
        | [ target ] ->
          (match Ast.view target with
           | Ast.Variable l -> Ok (Machine.Goto_label l)
           | Ast.Application (rop, [ rarg ]) ->
             (match Ast.view rop with
              | Ast.Variable "reg" -> symbol rarg >>= fun r -> Ok (Machine.Goto_reg r)
              | _ -> Error (Machine.Bad_instruction "a goto names one label or register"))
           | _ -> Error (Machine.Bad_instruction "a goto names one label or register"))
        | _ -> Error (Machine.Bad_instruction "a goto names one target"))
     | Ast.Variable "save" ->
       (match args with
        | [ r ] -> symbol r >>= fun name -> Ok (Machine.Save name)
        | _ -> Error (Machine.Bad_instruction "a save names one register"))
     | Ast.Variable "restore" ->
       (match args with
        | [ r ] -> symbol r >>= fun name -> Ok (Machine.Restore name)
        | _ -> Error (Machine.Bad_instruction "a restore names one register"))
     | Ast.Variable "perform" ->
       (match args with
        | call :: [] ->
          operation call >>= fun (name, inputs) -> Ok (Machine.Perform (name, inputs))
        | _ -> Error (Machine.Bad_instruction "a perform names one operation call"))
     | _ ->
       Error (Machine.Bad_instruction "the form is not an instruction of the new syntax"))
  | _ ->
    Error (Machine.Bad_instruction "the form is not an instruction of the new syntax")
;;

(** [read_syntax text] is the new-syntax reader: it parses the text and
    resolves labels exactly as the old one does, bare names included,
    producing the same typed program the simulator consumes. *)
let read_syntax text =
  Sicp_common.Reader.read text
  |> Result.map_error (fun e -> Machine.Parse (Sicp_common.Reader.to_string e))
  >>= fun exp ->
  (match Ast.view exp with
   | Ast.Application (op, body) ->
     (match Ast.view op with
      | Ast.Variable "controller" -> Ok body
      | _ -> Error (Machine.Parse "the controller is written (controller ...)"))
   | _ -> Error (Machine.Parse "the controller is written (controller ...)"))
  >>= fun body ->
  let rec scan pending items entries =
    match items with
    | [] -> Ok (List.rev entries, List.rev pending)
    | item :: rest ->
      (match Ast.view item with
       | Ast.Variable lab -> scan (lab :: pending) rest entries
       | _ ->
         (match instruction item with
          | Error e -> Error e
          | Ok inst -> scan [] rest ((pending, inst) :: entries)))
  in
  scan [] body []
  >>= fun (entries, trailing) ->
  let code = Array.of_list (List.map snd entries) in
  let positioned =
    List.concat (List.mapi (fun i (labels, _) -> List.map (fun l -> l, i) labels) entries)
  in
  let labels = positioned @ List.map (fun l -> l, Array.length code) trailing in
  let rec duplicate used = function
    | [] -> Ok ()
    | (l, _) :: rest ->
      if List.mem_assoc l used
      then Error (Machine.Bad_instruction ("the label " ^ l ^ " is used twice"))
      else duplicate ((l, ()) :: used) rest
  in
  duplicate [] labels >>= fun () -> Ok { Machine.code; Machine.labels }
;;

(** [ex_5_10 ()] runs the GCD machine written in the old syntax and the
    same machine written in the new syntax, and shows one compiled
    instruction of each: the new syntax compiles to the same typed
    instruction the old one always produced. *)
let ex_5_10 () =
  let old_controller =
    {|(controller
 test-b
   (test (op =) (reg b) (const 0))
   (branch (label gcd-done))
   (assign t (op rem) (reg a) (reg b))
   (assign a (reg b))
   (assign b (reg t))
   (goto (label test-b))
 gcd-done)|}
  in
  let new_controller =
    {|(controller
 test-b
   (test = b (const 0))
   (branch gcd-done)
   (assign t (rem a b))
   (assign a b)
   (assign b t)
   (goto test-b)
 gcd-done)|}
  in
  let load m =
    Machine.set_register m "a" (Machine.Int 12)
    >>= fun () ->
    Machine.set_register m "b" (Machine.Int 8)
    >>= fun () -> Machine.start m >>= fun () -> Machine.get_register m "a"
  in
  let run_old controller =
    Machine.make_machine
      ~registers:[ "a"; "b"; "t" ]
      ~operations:Machine.arith_operations
      ~controller
    >>= load
  in
  let run_new controller =
    read_syntax controller
    >>= fun program ->
    Machine.make_machine_from_program
      ~registers:[ "a"; "b"; "t" ]
      ~operations:Machine.arith_operations
      program
    >>= load
  in
  run_old old_controller
  >>= fun old_answer ->
  run_new new_controller
  >>= fun new_answer ->
  Machine.parse_program old_controller
  >>= fun old_program ->
  read_syntax new_controller
  >>= fun new_program ->
  Ok
    (List.map Machine.value_to_string [ old_answer; new_answer ]
     @ [ Machine.instruction_to_string old_program.code.(2)
       ; Machine.instruction_to_string new_program.code.(2)
       ])
;;
