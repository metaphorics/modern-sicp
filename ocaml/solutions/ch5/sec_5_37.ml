(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.37: with [preserving] disabled, every register in every
    preserved set is saved and restored unconditionally.  Compiling
    the recursive factorial without the mechanism adds the blind saves
    of [continue] around every sequence link and of [env], [proc], and
    [argl] around every operand evaluation, whether or not the second
    sequence needs them.  The machine pays for each one: the measured
    pushes grow well past the base 144 at n = 5 while the answer stays
    120. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

let factorial_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))|}
;;

let no_preserving = { C.default_config with preserving_on = false }

(** [compile_count cfg src] is the compilation's statement count and
    its save/restore count. *)
let compile_count cfg src =
  let state = C.new_state () in
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile cfg state [] exp "val" C.Next
    >>= fun seq ->
    let saves =
      List.length
        (List.filter
           (fun s ->
              (String.length s >= 6 && String.sub s 0 6 = "(save ")
              || (String.length s >= 9 && String.sub s 0 9 = "(restore "))
           seq.stmts)
    in
    Ok (List.length seq.stmts, saves)
;;

(** [run_monitored cfg n] compiles [factorial] under [cfg], runs it at
    [n] on a monitored machine, and answers the pushes and depth. *)
let run_monitored cfg n =
  let state = C.new_state () in
  let monitored_driver =
    ";; branches if flag is set:\n(branch (label external-entry))\n"
    ^ {|read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))|}
  in
  let controller =
    String.concat
      "\n"
      (List.map
         (fun (nm, text) -> if nm = "driver" then monitored_driver else text)
         C.eceval_fragments)
  in
  C.compile_block ~cfg state factorial_source
  >>= fun (entry, block) ->
  C.make_compiled_evaluator
    ~controller:(controller ^ "\n" ^ block)
    ~source:(Printf.sprintf "(factorial %d)" n)
    ~state
    ()
  >>= fun m ->
  C.set_register m "val" (Sicp_ch5.Sec_5_4.Lab entry)
  >>= fun () ->
  C.set_flag m true;
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () ->
  let stats =
    List.filter
      (fun l -> String.length l > 14 && String.sub l 0 14 = "(total-pushes")
      (C.transcript m)
  in
  Ok stats
;;

(** [ex_5_37 ()] compares the two compilations: the statement and
    save/restore counts, and the measured monitored sessions at n = 5. *)
let ex_5_37 () =
  compile_count C.default_config factorial_source
  >>= fun (stmts_with, saves_with) ->
  compile_count no_preserving factorial_source
  >>= fun (stmts_without, saves_without) ->
  run_monitored C.default_config 5
  >>= fun stats_with ->
  run_monitored no_preserving 5
  >>= fun stats_without ->
  Ok
    [ Printf.sprintf
        "with preserving: %d statements, %d saves/restores"
        stmts_with
        saves_with
    ; Printf.sprintf
        "without: %d statements, %d saves/restores"
        stmts_without
        saves_without
    ; "monitored with: " ^ String.concat " " stats_with
    ; "monitored without: " ^ String.concat " " stats_without
    ]
;;
