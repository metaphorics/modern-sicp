(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.5: the hand simulations of the factorial machine of
    Figure 5.11 and the Fibonacci machine of Figure 5.12. [Handsim] is
    a direct transcription of the controller semantics -- its own pc,
    flag, register file, and stack -- that shares only the substrate's
    operation table with the simulator, so the traces are the hand
    model's answers, not the simulator's. *)

let ( >>= ) = Result.bind

module Machine = Sicp_ch5.Sec_5_1

let rec all f = function
  | [] -> Ok []
  | x :: xs -> f x >>= fun y -> all f xs >>= fun ys -> Ok (y :: ys)
;;

(** The recursive factorial machine of Figure 5.11. *)
let factorial_recursive_controller =
  {|(controller
   (assign continue (label fact-done))
 fact-loop
   (test (op =) (reg n) (const 1))
   (branch (label base-case))
   (save continue)
   (save n)
   (assign n (op -) (reg n) (const 1))
   (assign continue (label after-fact))
   (goto (label fact-loop))
 after-fact
   (restore n)
   (restore continue)
   (assign val (op *) (reg n) (reg val))
   (goto (reg continue))
 base-case
   (assign val (const 1))
   (goto (reg continue))
 fact-done)|}
;;

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

(** The hand-simulation transcription: labels resolve through the
    substrate's parsed program; the arithmetic primitives come from
    [arith_operations]; the stepping is the controller's semantics read
    straight off the text. *)
module Handsim : sig
  (** One state of the transcription. *)
  type state =
    { pc : int
    ; flag : bool
    ; regs : (string * Machine.value) list
    ; stack : Machine.value list
    ; steps : int
    ; saves : int
    }

  (** One significant point of 5.5: a save, a restore, a taken branch,
      or a return through [continue]. *)
  type event =
    | Saved of string * Machine.value list
    | Restored of string * Machine.value * Machine.value list
    | Branch_taken of string
    | Returned_to of string

  val initial : (string * Machine.value) list -> state

  (** [run program state events] steps until the sequence ends. *)
  val run
    :  Machine.program
    -> state
    -> event list
    -> (event list * state, Machine.error) result

  (** [render e] is the trace line of [e]; the stack is written with
      its top on the left. *)
  val render : event -> string
end = struct
  type state =
    { pc : int
    ; flag : bool
    ; regs : (string * Machine.value) list
    ; stack : Machine.value list
    ; steps : int
    ; saves : int
    }

  type event =
    | Saved of string * Machine.value list
    | Restored of string * Machine.value * Machine.value list
    | Branch_taken of string
    | Returned_to of string

  let initial regs = { pc = 0; flag = false; regs; stack = []; steps = 0; saves = 0 }

  let find regs r =
    match List.assoc_opt r regs with
    | Some v -> Ok v
    | None -> Error (Machine.Unknown_register r)
  ;;

  let bind regs r v = (r, v) :: List.remove_assoc r regs

  let eval regs = function
    | Machine.Reg r -> find regs r
    | Machine.Const v -> Ok v
    | Machine.Label_source l -> Ok (Machine.Label l)
  ;;

  let operation opname args =
    match List.assoc_opt opname Machine.arith_operations with
    | Some (Machine.Value_op f) -> f args
    | _ -> Error (Machine.Unknown_operation opname)
  ;;

  let label_pos labels l =
    match List.assoc_opt l labels with
    | Some i -> Ok i
    | None -> Error (Machine.Unknown_label l)
  ;;

  let rec run (program : Machine.program) st events =
    if st.pc >= Array.length program.code
    then Ok (List.rev events, st)
    else (
      let bump st = { st with pc = st.pc + 1; steps = st.steps + 1 } in
      let continue_ st = run program st events in
      match program.code.(st.pc) with
      | Machine.Assign (r, src) ->
        eval st.regs src >>= fun v -> continue_ { (bump st) with regs = bind st.regs r v }
      | Machine.Assign_op (r, opname, inputs) ->
        all (eval st.regs) inputs
        >>= fun vals ->
        operation opname vals
        >>= fun v -> continue_ { (bump st) with regs = bind st.regs r v }
      | Machine.Test (opname, inputs) ->
        all (eval st.regs) inputs
        >>= fun vals ->
        operation opname vals
        >>= fun v ->
        (match v with
         | Machine.Bool b -> continue_ { (bump st) with flag = b }
         | _ ->
           Error
             (Machine.Bad_instruction
                ("hand simulation: the test " ^ opname ^ " did not answer a boolean")))
      | Machine.Branch l ->
        if st.flag
        then
          label_pos program.labels l
          >>= fun i -> run program { (bump st) with pc = i } (Branch_taken l :: events)
        else continue_ (bump st)
      | Machine.Goto_label l ->
        label_pos program.labels l >>= fun i -> continue_ { (bump st) with pc = i }
      | Machine.Goto_reg r ->
        find st.regs r
        >>= fun v ->
        (match v with
         | Machine.Label l ->
           label_pos program.labels l
           >>= fun i -> run program { (bump st) with pc = i } (Returned_to l :: events)
         | _ ->
           Error
             (Machine.Bad_instruction
                "hand simulation: the goto register does not hold a label"))
      | Machine.Perform (opname, inputs) ->
        all (eval st.regs) inputs
        >>= fun vals -> operation opname vals >>= fun _ -> continue_ (bump st)
      | Machine.Save r ->
        find st.regs r
        >>= fun v ->
        run
          program
          { (bump st) with stack = v :: st.stack; saves = st.saves + 1 }
          (Saved (r, v :: st.stack) :: events)
      | Machine.Restore r ->
        (match st.stack with
         | v :: rest ->
           run
             program
             { (bump st) with regs = bind st.regs r v; stack = rest }
             (Restored (r, v, rest) :: events)
         | [] -> Error (Machine.Stack_underflow r)))
  ;;

  let stack_to_string stack =
    "(" ^ String.concat " " (List.map Machine.value_to_string stack) ^ ")"
  ;;

  let render = function
    | Saved (r, stack) -> Printf.sprintf "(save %s) stack=%s" r (stack_to_string stack)
    | Restored (r, v, stack) ->
      Printf.sprintf
        "(restore %s) %s=%s stack=%s"
        r
        r
        (Machine.value_to_string v)
        (stack_to_string stack)
    | Branch_taken l -> "branch taken to " ^ l
    | Returned_to l -> "return to " ^ l
  ;;
end

let answer_line (st : Handsim.state) r =
  match List.assoc_opt r st.regs with
  | Some v -> "answer " ^ Machine.value_to_string v
  | None -> "Error: " ^ Machine.error_to_string (Machine.Unknown_register r)
;;

(** [hand_trace controller initial answer] is the full trace of one hand
    simulation: one line per significant point, then the answer. *)
let hand_trace controller initial answer =
  Machine.parse_program controller
  >>= fun program ->
  Handsim.run program (Handsim.initial initial) []
  >>= fun (events, st) -> Ok (List.map Handsim.render events @ [ answer_line st answer ])
;;

(** [ex_5_05 ()] hand-simulates the factorial machine on [n = 3] and the
    Fibonacci machine on [n = 3]. *)
let ex_5_05 () =
  hand_trace
    factorial_recursive_controller
    [ "n", Machine.Int 3; "val", Machine.Int 0 ]
    "val"
  >>= fun fact_trace ->
  hand_trace fib_controller [ "n", Machine.Int 3; "val", Machine.Int 0 ] "val"
  >>= fun fib_trace -> Ok (fact_trace @ fib_trace)
;;
