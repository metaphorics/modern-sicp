(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.47: compiled code calling interpreted procedures.  The
    compiler's [compound_calls] configuration adds the third branch to
    every procedure call: [compound-procedure?] hands the call to the
    evaluator's [compound-apply] through the [unev] register, whose
    value is dead at a call site (the machine's register set is the
    5.4 substrate's, so the book's [compapp] register has no name
    here; the entry point rides in [unev] instead).  The branch saves
    [continue] on the stack before jumping: the interpreted
    compound-apply reaches its body through ev-sequence, whose
    last-expression path restores [continue] from the stack, the
    interpreted calling convention, so the return address rides the
    stack exactly as it does when the evaluator itself dispatches.
    All four target/linkage combinations keep their shapes: the
    compound branch sets [continue] exactly as the compiled branch
    would. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind
let compound_calls = { C.default_config with compound_calls = true }

(** The book's test: compile [f], then define [g] interpreted at the
    driver, then call [f]. *)
let f_source =
  {|(define (f n)
  (g (+ n 1)))|}
;;

let driver_source =
  {|(define (g x) (* x 2))
(f 5)|}
;;

(** [ex_5_47 ()] runs the book's session: compile-and-go [f], then the
    interpreted definition of [g] and the call [(f 5)], which the
    compiled [f] routes through [compound-apply]; [g] doubles, so the
    answer is [g] of 6, that is 12.  The emitted compound branch is
    shown beside the run. *)
let ex_5_47 () =
  let state = C.new_state () in
  C.compile_block ~cfg:compound_calls state f_source
  >>= fun (_entry, block) ->
  let lines = String.split_on_char '\n' block in
  let has_prefix prefixes s =
    List.exists
      (fun p ->
         String.length s >= String.length p && String.sub s 0 (String.length p) = p)
      prefixes
  in
  let sample =
    List.filter
      (has_prefix
         [ "(assign unev (label compound-apply)"
         ; "(test (op compound-procedure?)"
         ; "(branch (label compound-branch"
         ; "(goto (reg unev))"
         ])
      lines
  in
  C.compile_and_go ~cfg:compound_calls ~state ~compiled:f_source ~source:driver_source ()
  >>= fun m ->
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () ->
  Ok
    [ "compound branch instructions: " ^ String.concat "; " sample
    ; "session: " ^ String.concat " " (C.transcript m)
    ]
;;
