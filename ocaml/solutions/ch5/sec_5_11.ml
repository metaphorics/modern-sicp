(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.11: the three disciplines a [restore] can follow, one
    simulator parameterized over them all. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2

(** Which [restore] semantics the machine follows:

    - [Untagged], the book simulator's: [restore] puts into the
      register the last value saved, whatever register it came from.
    - [Tagged]: [save] files the register's name with the value, and
      [restore] refuses a value saved from another register.
    - [Per_register]: each register owns a stack, so [restore] always
      recovers that register's own last save. *)
type discipline =
  | Untagged
  | Tagged
  | Per_register

module Sim = struct
  type machine =
    { discipline : discipline
    ; regs : (string, Machine.value ref) Hashtbl.t
    ; ops : (string, Machine.op) Hashtbl.t
    ; labels : (string, int) Hashtbl.t
    ; code : Machine.instruction array
    ; mutable pc : int
    ; mutable flag : Machine.value
    ; stack : Machine.value list ref (* the Untagged entries *)
    ; tagged_stack : (string * Machine.value) list ref
      (* the Tagged entries, the register's name filed with each value *)
    ; reg_stacks : (string, Machine.value list) Hashtbl.t
      (* the Per_register machine's per-register stacks *)
    }

  let rec all f = function
    | [] -> Ok []
    | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
  ;;

  let source_value m = function
    | Machine.Reg r ->
      (match Hashtbl.find_opt m.regs r with
       | Some cell -> Ok !cell
       | None -> Error (Machine.Unknown_register r))
    | Machine.Const v -> Ok v
    | Machine.Label_source l -> Ok (Machine.Label l)
  ;;

  let apply m name args =
    match Hashtbl.find_opt m.ops name with
    | None -> Error (Machine.Unknown_operation name)
    | Some (Machine.Value_op f) -> f args
    | Some (Machine.Action_op _) ->
      Error
        (Machine.Bad_instruction
           ("the operation " ^ name ^ " is an action and produces no value"))
  ;;

  let apply_action m name args =
    match Hashtbl.find_opt m.ops name with
    | None -> Error (Machine.Unknown_operation name)
    | Some (Machine.Action_op f) -> f args
    | Some (Machine.Value_op _) ->
      Error
        (Machine.Bad_instruction
           ("the operation " ^ name ^ " produces a value; assign it, do not perform it"))
  ;;

  (** [push m reg v] saves [v] under the machine's discipline. *)
  let push m reg v =
    match m.discipline with
    | Untagged -> m.stack := v :: !(m.stack)
    | Tagged -> m.tagged_stack := (reg, v) :: !(m.tagged_stack)
    | Per_register ->
      let held = Option.value (Hashtbl.find_opt m.reg_stacks reg) ~default:[] in
      Hashtbl.replace m.reg_stacks reg (v :: held)
  ;;

  (** [restore m reg] recovers the value the discipline hands back; a
    discipline that refuses names the refusal in its typed error. *)
  let restore m reg =
    match m.discipline with
    | Untagged ->
      (match !(m.stack) with
       | [] -> Error (Machine.Stack_underflow reg)
       | v :: rest ->
         m.stack := rest;
         Ok v)
    | Tagged ->
      (match !(m.tagged_stack) with
       | [] -> Error (Machine.Stack_underflow reg)
       | (saved_reg, v) :: rest ->
         if String.equal saved_reg reg
         then (
           m.tagged_stack := rest;
           Ok v)
         else
           Error
             (Machine.Bad_instruction
                ("restore " ^ reg ^ " but the stack holds " ^ saved_reg)))
    | Per_register ->
      (match Hashtbl.find_opt m.reg_stacks reg with
       | Some (v :: rest) ->
         Hashtbl.replace m.reg_stacks reg rest;
         Ok v
       | _ -> Error (Machine.Stack_underflow reg))
  ;;

  let jump_to m l =
    match Hashtbl.find_opt m.labels l with
    | Some i ->
      m.pc <- i;
      Ok ()
    | None -> Error (Machine.Unknown_label l)
  ;;

  let advance m = m.pc <- m.pc + 1

  (** [step m] executes the instruction at the pc under the machine's
    discipline; only the save and restore clauses differ between the
    three machines. *)
  let step m =
    match m.code.(m.pc) with
    | Machine.Assign (r, src) ->
      source_value m src
      >>= fun v ->
      Hashtbl.find m.regs r := v;
      advance m;
      Ok ()
    | Machine.Assign_op (r, op, inputs) ->
      all (source_value m) inputs
      >>= fun args ->
      apply m op args
      >>= fun v ->
      Hashtbl.find m.regs r := v;
      advance m;
      Ok ()
    | Machine.Test (op, inputs) ->
      all (source_value m) inputs
      >>= fun args ->
      apply m op args
      >>= fun v ->
      (match v with
       | Machine.Bool _ as b ->
         m.flag <- b;
         advance m;
         Ok ()
       | other ->
         Error
           (Machine.Bad_instruction
              ("the test "
               ^ op
               ^ " answered "
               ^ Machine.value_to_string other
               ^ ", not a boolean")))
    | Machine.Branch l ->
      (match m.flag with
       | Machine.Bool b ->
         if b
         then jump_to m l
         else (
           advance m;
           Ok ())
       | _ -> Error Machine.Branch_without_test)
    | Machine.Goto_label l -> jump_to m l
    | Machine.Goto_reg r ->
      (match !(Hashtbl.find m.regs r) with
       | Machine.Label l -> jump_to m l
       | other ->
         Error
           (Machine.Bad_instruction
              ("goto reads "
               ^ Machine.value_to_string other
               ^ " from "
               ^ r
               ^ ", not a label")))
    | Machine.Perform (op, inputs) ->
      all (source_value m) inputs
      >>= fun args ->
      apply_action m op args
      >>= fun () ->
      advance m;
      Ok ()
    | Machine.Save r ->
      push m r !(Hashtbl.find m.regs r);
      advance m;
      Ok ()
    | Machine.Restore r ->
      restore m r
      >>= fun v ->
      Hashtbl.find m.regs r := v;
      advance m;
      Ok ()
  ;;

  let rec run m =
    if m.pc >= Array.length m.code then Ok () else step m >>= fun () -> run m
  ;;

  (** [make ~discipline ~registers ~operations ~controller] builds a
    machine of the given discipline. *)
  let make ~discipline ~registers ~operations ~controller =
    Machine.parse_program controller
    >>= fun (program : Machine.program) ->
    let m =
      { discipline
      ; regs = Hashtbl.create 16
      ; ops = Hashtbl.create 16
      ; labels = Hashtbl.create 16
      ; code = program.code
      ; pc = 0
      ; flag = Machine.Symbol "*unassigned*"
      ; stack = ref []
      ; tagged_stack = ref []
      ; reg_stacks = Hashtbl.create 16
      }
    in
    List.iter (fun r -> Hashtbl.replace m.regs r (ref (Machine.Int 0))) registers;
    List.iter (fun (name, o) -> Hashtbl.replace m.ops name o) operations;
    List.iter (fun (l, i) -> Hashtbl.replace m.labels l i) program.labels;
    Ok m
  ;;

  let set_register m r v =
    Hashtbl.find m.regs r := v;
    Ok ()
  ;;

  let get_register m r = Ok !(Hashtbl.find m.regs r)

  let initialize_stack m =
    m.stack := [];
    Hashtbl.reset m.reg_stacks;
    Ok ()
  ;;
end

(** The Fibonacci machine of Figure 5.12. *)
let fib_controller =
  {|(controller
   (assign continue (label fib-done))
 fib-loop
   (test (op <) (reg n) (const 2))
   (branch (label immediate-answer))
   (save continue)
   (assign continue (label afterfib-n-1))
   (save n)
   (assign n (op -) (reg n) (const 1))
   (goto (label fib-loop))
 afterfib-n-1
   (restore n)
   (restore continue)
   (assign n (op -) (reg n) (const 2))
   (save continue)
   (assign continue (label afterfib-n-2))
   (save val)
   (goto (label fib-loop))
 afterfib-n-2
   (assign n (reg val))
   (restore val)
   (restore continue)
   (assign val (op +) (reg val) (reg n))
   (goto (reg continue))
 immediate-answer
   (assign val (reg n))
   (goto (reg continue))
 fib-done)|}
;;

(** The Figure 5.12 machine with one instruction eliminated: the
    exchange at afterfib-n-2 -- [n] from [val], [val] from the stack --
    becomes the single untagged [restore n], which takes the saved
    Fibonacci value from wherever it was filed. One fewer instruction
    per level, same answers. *)
let fib_one_fewer_controller =
  {|(controller
   (assign continue (label fib-done))
 fib-loop
   (test (op <) (reg n) (const 2))
   (branch (label immediate-answer))
   (save continue)
   (assign continue (label afterfib-n-1))
   (save n)
   (assign n (op -) (reg n) (const 1))
   (goto (label fib-loop))
 afterfib-n-1
   (restore n)
   (restore continue)
   (assign n (op -) (reg n) (const 2))
   (save continue)
   (assign continue (label afterfib-n-2))
   (save val)
   (goto (label fib-loop))
 afterfib-n-2
   (restore n)
   (restore continue)
   (assign val (op +) (reg val) (reg n))
   (goto (reg continue))
 immediate-answer
   (assign val (reg n))
   (goto (reg continue))
 fib-done)|}
;;

(** [run_fib controller n] computes Fib(n) on the Untagged machine. *)
let run_fib controller n =
  Sim.make
    ~discipline:Untagged
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller
  >>= fun m ->
  Sim.set_register m "n" (Machine.Int n)
  >>= fun () -> Sim.run m >>= fun () -> Sim.get_register m "val"
;;

(** [run_sequence discipline] runs the book's save/save/restore
    sequence -- x holds 8, y holds 7 -- and reports y. *)
let run_sequence discipline =
  Sim.make
    ~discipline
    ~registers:[ "x"; "y" ]
    ~operations:[]
    ~controller:{|(controller (save y) (save x) (restore y))|}
  >>= fun m ->
  Sim.set_register m "y" (Machine.Int 7)
  >>= fun () ->
  Sim.set_register m "x" (Machine.Int 8)
  >>= fun () -> Sim.run m >>= fun () -> Sim.get_register m "y"
;;

(** [ex_5_11 ()] demonstrates all three disciplines: the untagged
    machine and its one-instruction-smaller Fibonacci machine, the
    tagged machine's refusal, and the per-register machine's own
    stacks. *)
let ex_5_11 () =
  run_sequence Untagged
  >>= fun untagged_y ->
  let rec fib_pairs = function
    | [] -> Ok []
    | n :: ns ->
      run_fib fib_controller n
      >>= fun v ->
      run_fib fib_one_fewer_controller n
      >>= fun w -> fib_pairs ns >>= fun rest -> Ok ((v, w) :: rest)
  in
  fib_pairs [ 0; 1; 2; 3; 4; 5; 6; 7; 8; 9 ]
  >>= fun pairs ->
  let all_match = List.for_all (fun (v, w) -> Machine.equal_value v w) pairs in
  let tagged_outcome =
    run_sequence Tagged
    |> function
    | Ok _ -> "the sequence ran (tagging refused nothing)"
    | Error e -> "Error: " ^ Machine.error_to_string e
  in
  Sim.make
    ~discipline:Tagged
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller:fib_controller
  >>= fun tagged_m ->
  Sim.set_register tagged_m "n" (Machine.Int 6)
  >>= fun () ->
  Sim.run tagged_m
  >>= fun () ->
  Sim.get_register tagged_m "val"
  >>= fun tagged_fib6 ->
  run_sequence Per_register
  >>= fun per_register_y ->
  let underflow_outcome =
    Sim.make
      ~discipline:Per_register
      ~registers:[ "x"; "y" ]
      ~operations:[]
      ~controller:{|(controller (restore x))|}
    >>= Sim.run
    |> function
    | Ok () -> "the restore ran on an empty stack"
    | Error e -> "Error: " ^ Machine.error_to_string e
  in
  Ok
    [ "(a) untagged: (save y) (save x) (restore y) leaves y = "
      ^ Machine.value_to_string untagged_y
    ; ("(a) fib with afterfib-n-2's exchange replaced by one (restore n): answers n=0..9 "
       ^ if all_match then "match the original" else "DIVERGE")
    ; "(b) tagged: (save y) (save x) (restore y) reports -- " ^ tagged_outcome
    ; "(b) tagged fib 6 = " ^ Machine.value_to_string tagged_fib6
    ; "(c) per-register: (save y) (save x) (restore y) leaves y = "
      ^ Machine.value_to_string per_register_y
    ; "(c) restore from an empty per-register stack reports -- " ^ underflow_outcome
    ]
;;
