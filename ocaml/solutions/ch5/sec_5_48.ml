(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.48: [compile-and-run] as a primitive in the global
    environment.  The primitive compiles its (quoted) argument and
    answers [ok]; the compiled block lands in the machine's controller
    when the next assembly is built -- the edition's controller is
    parsed text, so the book's live patch of a running machine is
    reproduced as the next assembled machine with the same compile
    state and a continued session.  The observable session is the
    book's: [ok] from the primitive, [ok] from the compiled define,
    then [120]. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** [compile_quoted state blocks v] compiles the quoted expression the
    value [v] carries (the machine's own printed form of it, which the
    reader parses) into a controller block and records the block for
    the next assembly. *)
let compile_quoted state blocks v =
  match Sicp_common.Reader.read (Sicp_common.Value.display v) with
  | Error e -> Error (Sicp_common.Eval_error.Type_error (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile_program ~linkage:C.Return state [ exp ]
    |> Result.map_error (fun e -> Sicp_common.Eval_error.Type_error (C.error_to_string e))
    >>= fun seq ->
    let entry = "compiled-entry-run-" ^ string_of_int (List.length !blocks) in
    blocks := !blocks @ [ entry ^ "\n" ^ C.statements_text seq ];
    Ok (Sicp_common.Value.symbol "ok")
;;

(** [compile_and_run_primitive state blocks] is the primitive bound in
    the machine's global environment. *)
let compile_and_run_primitive state blocks =
  ( "compile-and-run"
  , function
    | [ v ] -> compile_quoted state blocks v
    | args ->
      Error
        (Sicp_common.Eval_error.Arity_mismatch { expected = 1; given = List.length args })
  )
;;

(** [arm m entry] points [val] at the compiled entry and arms the
    external entry, compile-and-go's wiring for an already compiled
    block. *)
let arm m entry =
  C.set_register m "val" (Sicp_ch5.Sec_5_4.Lab entry)
  >>= fun () ->
  C.set_flag m true;
  Ok ()
;;

(** [run m] drains the machine, the driver's queue-dry stop being the
    normal end. *)
let run m =
  match C.start m with
  | Ok () -> Ok ()
  | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
  | Error e -> Error e
;;

(** [ex_5_48 ()] runs the book's session: the compile-and-run define,
    then the call.  The first machine answers ok through the
    primitive and hands the block on; the second machine, assembled
    with the block and armed at its entry, answers 120. *)
let ex_5_48 () =
  let state = C.new_state () in
  let blocks = ref [] in
  let session_source =
    {|(compile-and-run
 '(define (factorial n)
    (if (= n 1)
        1
        (* (factorial (- n 1)) n))))|}
  in
  C.make_compiled_evaluator
    ~globals:[ compile_and_run_primitive state blocks ]
    ~source:session_source
    ~state
    ()
  >>= fun m1 ->
  run m1
  >>= fun () ->
  let first_transcript = C.transcript m1 in
  match !blocks with
  | [ block ] ->
    let entry =
      match String.index_opt block '\n' with
      | Some i -> String.sub block 0 i
      | None -> block
    in
    let controller = C.eceval_controller ^ "\n" ^ block in
    C.make_compiled_evaluator ~controller ~source:"(factorial 5)" ~state ()
    >>= fun m2 ->
    arm m2 entry
    >>= fun () ->
    run m2
    >>= fun () ->
    Ok [ "session: " ^ String.concat " " (first_transcript @ C.transcript m2) ]
  | _ -> Error (C.Op_failed "compile-and-run produced no block")
;;
