(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.15 and the simulator core the monitoring exercises
    share: a machine that counts the instructions it executes, with
    the label information, the register tracing, and the breakpoint
    hooks the later exercises turn on. The hooks are inert here.

    The core mirrors the section's simulator -- the same typed
    instructions, the same assembly-time checks -- with the monitoring
    state the book's exercises add one piece at a time. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_2

(** How a run ended: the sequence ran out, or a breakpoint stopped the
    machine just before the [n]th instruction after [label]. *)
type stop =
  | Completed
  | Breakpoint of string * int

module Sim = struct
  type register =
    { name : string
    ; mutable contents : Machine.value
    ; mutable trace : bool
    }

  type machine =
    { regs : (string, register) Hashtbl.t
    ; ops : (string, Machine.op) Hashtbl.t
    ; labels : (string, int) Hashtbl.t
    ; code : Machine.instruction array
    ; labels_before : string list array
      (* 5.17's retained label information, attached at assembly *)
    ; mutable pc : int
    ; mutable flag : Machine.value
    ; stack : Machine.value list ref
    ; mutable count : int
    ; mutable breakpoints : (string * int * int) list
      (* label, 1-based offset, resolved index *)
    ; assignments : string list ref (* 5.18's transcript of traced register writes *)
    }

  let rec all f = function
    | [] -> Ok []
    | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
  ;;

  let source_value m = function
    | Machine.Reg r ->
      (match Hashtbl.find_opt m.regs r with
       | Some reg -> Ok reg.contents
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

  (** [store m reg v] is the one write path: 5.18's tracing hangs
      here. *)
  let store m reg v =
    if reg.trace
    then
      m.assignments
      := !(m.assignments)
         @ [ reg.name
             ^ ": "
             ^ Machine.value_to_string reg.contents
             ^ " -> "
             ^ Machine.value_to_string v
           ]
    else ();
    reg.contents <- v
  ;;

  let jump_to m l =
    match Hashtbl.find_opt m.labels l with
    | Some i ->
      m.pc <- i;
      Ok ()
    | None -> Error (Machine.Unknown_label l)
  ;;

  let advance m = m.pc <- m.pc + 1

  (** [step m] executes one instruction and counts it. *)
  let step m =
    m.count <- m.count + 1;
    match m.code.(m.pc) with
    | Machine.Assign (r, src) ->
      source_value m src
      >>= fun v ->
      store m (Hashtbl.find m.regs r) v;
      advance m;
      Ok ()
    | Machine.Assign_op (r, op, inputs) ->
      all (source_value m) inputs
      >>= fun args ->
      apply m op args
      >>= fun v ->
      store m (Hashtbl.find m.regs r) v;
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
      (match (Hashtbl.find m.regs r).contents with
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
      m.stack := (Hashtbl.find m.regs r).contents :: !(m.stack);
      advance m;
      Ok ()
    | Machine.Restore r ->
      (match !(m.stack) with
       | [] -> Error (Machine.Stack_underflow r)
       | v :: rest ->
         m.stack := rest;
         store m (Hashtbl.find m.regs r) v;
         advance m;
         Ok ())
  ;;

  (** [drive ?before ~skip m] executes from the pc until the sequence
      ends or a breakpoint stops it. A breakpoint catches the machine
      as it ARRIVES at the stopped instruction; [skip] lets [proceed]
      execute the instruction it rests on before the checks resume.
      [before] runs just before each executed instruction, 5.16's and
      5.17's hook, and never disturbs the count. *)
  let rec drive ?before ~skip m =
    let stopped =
      if skip
      then None
      else List.find_opt (fun (_, _, index) -> index = m.pc) m.breakpoints
    in
    match stopped with
    | Some (label, offset, _) -> Ok (Breakpoint (label, offset))
    | None ->
      if m.pc >= Array.length m.code
      then Ok Completed
      else (
        (match before with
         | Some f -> f m.pc
         | None -> ());
        step m >>= fun () -> drive ?before ~skip:false m)
  ;;

  let start ?before m =
    m.pc <- 0;
    drive ?before ~skip:false m
  ;;

  let proceed ?before m = drive ?before ~skip:true m

  let make ~registers ~operations ~controller =
    Machine.parse_program controller
    >>= fun (program : Machine.program) ->
    let registers = registers @ [ "flag" ] in
    let m =
      { regs = Hashtbl.create 16
      ; ops = Hashtbl.create 16
      ; labels = Hashtbl.create 16
      ; code = program.code
      ; labels_before = Array.make (Array.length program.code + 1) []
      ; pc = 0
      ; flag = Machine.Symbol "*unassigned*"
      ; stack = ref []
      ; count = 0
      ; breakpoints = []
      ; assignments = ref []
      }
    in
    let rec declare = function
      | [] -> Ok ()
      | r :: rest ->
        if Hashtbl.mem m.regs r
        then Error (Machine.Bad_instruction ("the register " ^ r ^ " is declared twice"))
        else (
          Hashtbl.replace
            m.regs
            r
            { name = r; contents = Machine.Symbol "*unassigned*"; trace = false };
          declare rest)
    in
    declare registers
    >>= fun () ->
    (* the register check the section's assembler performs *)
    let missing =
      Array.to_list program.code
      |> List.concat_map Machine.instruction_registers
      |> List.filter (fun r -> not (Hashtbl.mem m.regs r))
    in
    match missing with
    | r :: _ -> Error (Machine.Unknown_register r)
    | [] ->
      List.iter (fun (name, o) -> Hashtbl.replace m.ops name o) operations;
      List.iter (fun (l, i) -> Hashtbl.replace m.labels l i) program.labels;
      List.iter
        (fun (l, i) ->
           if i <= Array.length m.labels_before
           then m.labels_before.(i) <- l :: m.labels_before.(i))
        program.labels;
      Ok m
  ;;

  let set_register m r v =
    match Hashtbl.find_opt m.regs r with
    | Some reg ->
      store m reg v;
      Ok ()
    | None -> Error (Machine.Unknown_register r)
  ;;

  let get_register m r =
    match Hashtbl.find_opt m.regs r with
    | Some reg -> Ok reg.contents
    | None -> Error (Machine.Unknown_register r)
  ;;

  (** [take_instruction_count m] is 5.15's interface: the count since
      the last reset, and the reset. *)
  let take_instruction_count m =
    let n = m.count in
    m.count <- 0;
    n
  ;;

  let instruction_count m = m.count
  let instruction_labels m i = (Array.get m.labels_before) i
  let instruction_text m i = (Array.get m.code) i
  let instruction_total m = Array.length m.code

  (** [trace_register m r on] turns one register's tracing on or off. *)
  let trace_register m r on =
    match Hashtbl.find_opt m.regs r with
    | Some reg ->
      reg.trace <- on;
      Ok ()
    | None -> Error (Machine.Unknown_register r)
  ;;

  let traced_assignments m = !(m.assignments)

  (** [set_breakpoint m label n] stops the machine just before the
      [n]th instruction after [label], counted from one -- the book's
      [(set-breakpoint gcd-machine 'test-b 4)] sits before the
      assignment to [a], three instructions past the label. *)
  let set_breakpoint m label n =
    match Hashtbl.find_opt m.labels label with
    | None -> Error (Machine.Unknown_label label)
    | Some base ->
      let index = base + n - 1 in
      if index >= Array.length m.code || index < 0
      then
        Error
          (Machine.Bad_instruction
             ("the breakpoint at " ^ label ^ " " ^ string_of_int n ^ " is past the end"))
      else (
        m.breakpoints <- m.breakpoints @ [ label, n, index ];
        Ok ())
  ;;

  let cancel_breakpoint m label n =
    m.breakpoints
    <- List.filter (fun (l, o, _) -> not (String.equal l label && o = n)) m.breakpoints;
    Ok ()
  ;;

  let cancel_all_breakpoints m =
    m.breakpoints <- [];
    Ok ()
  ;;
end

(** The Fibonacci machine of Figure 5.12, the monitoring exercises'
    running example. *)
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

(** [ex_5_15 ()] counts the instructions of the Fibonacci machine on
    [n = 3] and [n = 6] and shows the count message's reset: the same
    machine rerun reports from zero again. *)
let ex_5_15 () =
  Sim.make
    ~registers:[ "n"; "val"; "continue" ]
    ~operations:Machine.arith_operations
    ~controller:fib_controller
  >>= fun m ->
  let run_fib n = Sim.set_register m "n" (Machine.Int n) >>= fun () -> Sim.start m in
  run_fib 3
  >>= fun _ ->
  Sim.get_register m "val"
  >>= fun answer3 ->
  let count3 = Sim.take_instruction_count m in
  run_fib 6
  >>= fun _ ->
  Sim.get_register m "val"
  >>= fun answer6 ->
  let count6 = Sim.take_instruction_count m in
  let count_after_reset = Sim.take_instruction_count m in
  Ok
    [ "fib 3 = "
      ^ Machine.value_to_string answer3
      ^ ", instructions = "
      ^ string_of_int count3
    ; "fib 6 = "
      ^ Machine.value_to_string answer6
      ^ ", instructions = "
      ^ string_of_int count6
    ; "after the reset the count is " ^ string_of_int count_after_reset
    ]
;;
