(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.36: the compiler's operand evaluation order is right to
    left: [construct-arglist] reverses the operand codes, so the last
    operand's value initializes [argl] and each earlier operand conses
    onto it.  The order is determined in [construct-arglist] (the one
    [reverse]); the [left_to_right] configuration turns it off.

    The measurement answers the efficiency question: both orders
    evaluate each operand once, build the list with one [list] and one
    [cons] per additional operand, and preserve the same registers, so
    the instruction counts are equal -- the code size is unaffected. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** [record_primitive] is the [record] procedure bound in the machine's
    global environment: it logs the value it sees, in order, and
    returns it; the observable the order shows through. *)
let log = ref []

let record_primitive : Sicp_common.Value.primitive = function
  | [ v ] ->
    log := !log @ [ Sicp_common.Value.to_string v ];
    Ok v
  | _ -> Error (Sicp_common.Eval_error.Arity_mismatch { expected = 1; given = 0 })
;;

(** [compiled_statements cfg src] compiles [src] under [cfg]. *)
let compiled_statements cfg src =
  let state = C.new_state () in
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp -> C.compile cfg state [] exp "val" C.Next >>= fun seq -> Ok seq.stmts
;;

(** [run cfg src] compiles [src] under [cfg] and runs it on a machine
    whose global environment binds [record], so the recorded order is
    the evaluation order of the operands. *)
let run cfg src =
  let state = C.new_state () in
  log := [];
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile cfg state [] exp "val" C.Next
    >>= fun seq ->
    let controller =
      C.eceval_controller
      ^ "\ncompiled-entry-x\n"
      ^ C.statements_text seq
      ^ "\n(goto (reg continue))"
    in
    C.make_compiled_evaluator
      ~controller
      ~globals:[ "record", record_primitive ]
      ~source:""
      ~state
      ()
    >>= fun m ->
    C.set_register m "val" (Sicp_ch5.Sec_5_4.Lab "compiled-entry-x")
    >>= fun () ->
    C.set_flag m true;
    (match C.start m with
     | Ok () -> Ok ()
     | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
     | Error e -> Error e)
    >>= fun () -> Ok (!log, List.length seq.stmts)
;;

(** [ex_5_36 ()] runs [(list (record 1) (record 2))] under both
    configurations: the default records [2 1] (right to left), the
    reordered configuration records [1 2], and the instruction counts
    are equal. *)
let ex_5_36 () =
  let src = "(list (record 1) (record 2))" in
  run C.default_config src
  >>= fun (log_right, stmts_right) ->
  run { C.default_config with left_to_right = true } src
  >>= fun (log_left, stmts_left) ->
  Ok
    [ "default order: " ^ String.concat " " log_right
    ; "left-to-right order: " ^ String.concat " " log_left
    ; Printf.sprintf
        "instruction counts: %d = %d: %b"
        stmts_right
        stmts_left
        (stmts_right = stmts_left)
    ]
;;
