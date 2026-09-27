(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.33: compiling the alternative factorial.  The
    compilation differs from the book's [factorial] in exactly one
    save/restore pair: the alternative's [*] needs [n] only after the
    recursive call returns, so [n] lives in its frame and needs no
    register; but the recursive call is an operand of [*], so [argl]
    is saved around it as before.  Both procedures answer the same
    values; neither is faster, the alternative's code is one pair of
    stack operations different, not faster or slower per call. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

let factorial_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))|}
;;

let factorial_alt_source =
  {|(define (factorial-alt n)
  (if (= n 1)
      1
      (* n (factorial-alt (- n 1)))))|}
;;

(** [statements_of state src] is the compilation's statements, one
    controller line each, without the linkage tail. *)
let statements_of state src =
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile C.default_config state [] exp "val" C.Next >>= fun seq -> Ok seq.stmts
;;

(** [saves_of stmts] is the save/restore instructions in order. *)
let has_prefix p s =
  String.length s >= String.length p && String.sub s 0 (String.length p) = p
;;

let saves_of stmts =
  List.filter (fun s -> has_prefix "(save" s || has_prefix "(restore" s) stmts
;;

(** [run source] compiles the source, runs it on the machine, and
    answers the transcript. *)
let run source =
  let state = C.new_state () in
  C.compile_and_go ~state ~compiled:source ~source:"" ()
  >>= fun m ->
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m) when m = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () -> Ok (C.transcript m)
;;

(** [ex_5_33 ()] compiles both definitions, reports the differing
    save/restore pairs, and runs both factorials at 5: the alternative
    preserves [env] around the recursive call where the book's version
    preserves nothing extra, because [n] is fetched after the call for
    the multiplication. *)
let ex_5_33 () =
  let state = C.new_state () in
  statements_of state factorial_source
  >>= fun stmts ->
  let state2 = C.new_state () in
  statements_of state2 factorial_alt_source
  >>= fun alt_stmts ->
  run (factorial_source ^ "\n(factorial 5)")
  >>= fun transcript ->
  run (factorial_alt_source ^ "\n(factorial-alt 5)")
  >>= fun alt_transcript ->
  Ok
    [ "factorial saves: " ^ String.concat "; " (saves_of stmts)
    ; "factorial-alt saves: " ^ String.concat "; " (saves_of alt_stmts)
    ; "factorial 5: " ^ String.concat " " transcript
    ; "factorial-alt 5: " ^ String.concat " " alt_transcript
    ]
;;

(** {1:five_33a Exercise 5.33a}

    The hand-optimized body of [factorial-alt].  The book's compiled
    listing pays full procedure-call machinery for every arithmetic
    step (a lookup for [proc], an argument list, a dispatch, an
    [apply-primitive-procedure], and the register saves the preserving
    mechanism wraps around it all); the optimized body runs the
    arithmetic on the machine's [arg1]/[arg2] words with one
    instruction per operation, drops the two [proc] saves the compiler
    emits around the argument-list build, and folds the constant 1 into
    [arg2].  The compiled calling convention is kept: [continue] and
    [env] still ride the stack around the recursive call (the callee
    clobbers both and the [Return] linkage owes the caller its return
    address), and [argl] is saved because the argument list of [*] is
    built around the call. *)

let alt_body_source = {|(if (= n 1) 1 (* n (factorial-alt (- n 1))))|}

let optimized_statements =
  [ "(assign env (op compiled-procedure-env) (reg proc))"
  ; "(assign env (op extend-environment) (const n) (reg argl) (reg env))"
  ; "(assign arg1 (op lookup-variable-value) (const n) (reg env))"
  ; "(assign arg2 (const 1))"
  ; "(assign val (op =) (reg arg1) (reg arg2))"
  ; "(test (op false?) (reg val))"
  ; "(branch (label false-branch1))"
  ; "(assign val (const 1))"
  ; "(goto (reg continue))"
  ; "false-branch1"
  ; "(save continue)"
  ; "(save env)"
  ; "(assign proc (op lookup-variable-value) (const factorial-alt) (reg env))"
  ; "(assign arg1 (op lookup-variable-value) (const n) (reg env))"
  ; "(assign arg2 (const 1))"
  ; "(assign val (op -) (reg arg1) (reg arg2))"
  ; "(assign argl (op list) (reg val))"
  ; "(save argl)"
  ; "(assign continue (label after-call2))"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "after-call2"
  ; "(restore argl)"
  ; "(restore env)"
  ; "(restore continue)"
  ; "(assign arg1 (op lookup-variable-value) (const n) (reg env))"
  ; "(assign arg2 (reg val))"
  ; "(assign val (op *) (reg arg1) (reg arg2))"
  ; "(goto (reg continue))"
  ]
;;

(** [block_of name stmts] is one enterable controller block: the entry
    label and the statements.  The compiler's [Return]-linkage output
    and the hand-optimized listing both already end in the return
    [goto]; nothing is appended. *)
let block_of name stmts = String.concat "\n" (name :: stmts)

(** The naive compilation's statements: the body of the alternative
    factorial exactly as 5.33's compiler emits it, entered with the
    compiled calling convention ([proc] is the compiled procedure,
    [argl] the argument list, [continue] the return address). *)
let naive_statements =
  let state = C.new_state () in
  (match Sicp_common.Reader.read alt_body_source with
   | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
   | Ok body ->
     C.compile C.default_config state [ [ "n" ] ] body "val" C.Return
     >>= fun seq ->
     Ok
       ([ "(assign env (op compiled-procedure-env) (reg proc))"
        ; "(assign env (op extend-environment) (const n) (reg argl) (reg env))"
        ]
        @ seq.stmts))
  |> function
  | Ok stmts -> stmts
  | Error e -> failwith (C.error_to_string e)
;;

let naive_block = block_of "entry1" naive_statements
let optimized_block = block_of "entry1" optimized_statements

(** [measure block n] runs one block as [factorial-alt] on a fresh
    machine and answers the transcript plus the executed-instruction
    count.  The harness arms the compiled calling convention the way
    [compile_and_go] cannot for a bare body: the entry block builds the
    compiled-procedure object for [entry1] (so the body's recursive
    lookups of [factorial-alt] re-enter the same block as compiled
    code), defines it in the global environment, and lists the
    argument.  The fixed harness instructions are identical for both
    blocks, so their step-count difference is the code's alone. *)
let measure block n =
  let state = C.new_state () in
  let controller =
    C.eceval_controller
    ^ "\nmeasure-entry\n"
    ^ "  (assign proc (op make-compiled-procedure) (const entry1) (reg env))\n"
    ^ "  (perform (op define-variable!) (const factorial-alt) (reg proc) (reg env))\n"
    ^ Printf.sprintf "  (assign argl (op list) (const %d))\n" n
    ^ "  (assign val (op compiled-procedure-entry) (reg proc))\n"
    ^ "  (goto (reg val))\n"
    ^ block
  in
  C.make_compiled_evaluator ~controller ~source:"" ~state ()
  >>= fun m ->
  C.set_register m "val" (Sicp_ch5.Sec_5_4.Lab "measure-entry")
  >>= fun () ->
  C.set_flag m true;
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () -> Ok (C.transcript m, C.step_count m)
;;

(** [ex_5_33a ()] measures both versions at n = 5: the machine's own
    step counts, same answer 120. *)
let ex_5_33a () =
  measure naive_block 5
  >>= fun (naive_transcript, naive_steps) ->
  measure optimized_block 5
  >>= fun (opt_transcript, opt_steps) ->
  Ok
    [ "naive steps: " ^ string_of_int naive_steps
    ; "optimized steps: " ^ string_of_int opt_steps
    ; "naive transcript: " ^ String.concat " " naive_transcript
    ; "optimized transcript: " ^ String.concat " " opt_transcript
    ; Printf.sprintf
        "instruction win: %d of %d (%.1f%%)"
        (naive_steps - opt_steps)
        naive_steps
        (100.0 *. float_of_int (naive_steps - opt_steps) /. float_of_int naive_steps)
    ]
;;
