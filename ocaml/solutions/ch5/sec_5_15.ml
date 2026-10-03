(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

let ( let* ) = Result.bind

module M = Sicp_ch5.Sec_5_1
module Eval_error = Sicp_common.Eval_error

type stop =
  | Completed
  | Breakpoint of string * int

module Monitor = struct
  type breakpoint =
    { label : string
    ; offset : int
    ; index : int
    }

  type machine =
    { sim : M.value M.machine
    ; mutable count : int
    ; mutable traced : string list
    ; mutable assignments : string list
    ; mutable breakpoints : breakpoint list
    }

  let make ~registers ~operations ~controller =
    let* sim = M.make_machine ~registers ~operations ~controller in
    Ok { sim; count = 0; traced = []; assignments = []; breakpoints = [] }
  ;;

  let get_register m = M.get_register m.sim
  let code m = (M.program m.sim).code
  let instruction_at m i = (code m).(i)

  let instruction_labels m i =
    List.filter_map
      (fun (l, index) -> if index = i then Some l else None)
      (M.program m.sim).labels
  ;;

  let note m r old v =
    if List.mem r m.traced
    then (
      let line = r ^ ": " ^ M.value_to_string old ^ " -> " ^ M.value_to_string v in
      m.assignments <- line :: m.assignments)
  ;;

  let set_register m r v =
    let* old = M.get_register m.sim r in
    let* () = M.set_register m.sim r v in
    note m r old v;
    Ok ()
  ;;

  let written_register = function
    | M.Assign (r, _) | M.Assign_op (r, _, _) | M.Restore r -> Some r
    | M.Label _ | M.Test _ | M.Branch _ | M.Goto _ | M.Goto_reg _ | M.Save _ | M.Perform _
      -> None
  ;;

  (* One counted step. A write to a traced register is reported with
     the contents it replaced, so a write of an equal word still
     reports. *)
  let step m =
    let target =
      Option.bind
        (written_register (instruction_at m (M.pc m.sim)))
        (fun r -> if List.mem r m.traced then Some r else None)
    in
    let* old =
      match target with
      | None -> Ok None
      | Some r -> Result.map Option.some (M.get_register m.sim r)
    in
    let* _ = M.step m.sim in
    m.count <- m.count + 1;
    match target, old with
    | Some r, Some before ->
      let* after = M.get_register m.sim r in
      note m r before after;
      Ok ()
    | _ -> Ok ()
  ;;

  let rec drive ?before ~skip m =
    let pc = M.pc m.sim in
    match List.find_opt (fun b -> (not skip) && b.index = pc) m.breakpoints with
    | Some b -> Ok (Breakpoint (b.label, b.offset))
    | None when pc >= Array.length (code m) -> Ok Completed
    | None ->
      Option.iter (fun f -> f pc) before;
      let* () = step m in
      drive ?before ~skip:false m
  ;;

  let start ?before m =
    M.restart m.sim;
    drive ?before ~skip:false m
  ;;

  let proceed ?before m = drive ?before ~skip:true m

  let take_instruction_count m =
    let n = m.count in
    m.count <- 0;
    n
  ;;

  let instruction_count m = m.count

  let trace_register m r on =
    let* _ = M.get_register m.sim r in
    m.traced <- List.filter (fun t -> not (String.equal t r)) m.traced;
    if on then m.traced <- r :: m.traced;
    Ok ()
  ;;

  let traced_assignments m = List.rev m.assignments

  let set_breakpoint m label offset =
    match List.assoc_opt label (M.program m.sim).labels with
    | None -> Error (Eval_error.Unknown_label label)
    | Some base ->
      let index = base + offset - 1 in
      if offset < 1 || index >= Array.length (code m)
      then
        Error
          (Eval_error.Bad_instruction
             (Printf.sprintf "the breakpoint at %s %d is past the code" label offset))
      else (
        m.breakpoints <- m.breakpoints @ [ { label; offset; index } ];
        Ok ())
  ;;

  let cancel_breakpoint m label offset =
    m.breakpoints
    <- List.filter
         (fun b -> not (String.equal b.label label && b.offset = offset))
         m.breakpoints
  ;;

  let cancel_all_breakpoints m = m.breakpoints <- []
end

let ex_5_15 () =
  let* m =
    Monitor.make
      ~registers:[ "n"; "val"; "continue" ]
      ~operations:M.arith_operations
      ~controller:Sec_5_5.fib_controller
  in
  let run_fib n =
    let* () = Monitor.set_register m "n" (M.Int n) in
    let* _ = Monitor.start m in
    let* answer = Monitor.get_register m "val" in
    Ok (answer, Monitor.take_instruction_count m)
  in
  let* answer3, count3 = run_fib 3 in
  let* answer6, count6 = run_fib 6 in
  let line n answer count =
    Printf.sprintf "fib %d = %s, instructions = %d" n (M.value_to_string answer) count
  in
  Ok
    [ line 3 answer3 count3
    ; line 6 answer6 count6
    ; "after the reset the count is " ^ string_of_int (Monitor.take_instruction_count m)
    ]
;;
