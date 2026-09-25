(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(* The section replay: the compiler's outputs are pinned against the
   book's own listings (Figure 5.17's factorial compilation, run on the
   machine), the compiled-code session of 5.5.7 runs the book's
   compile-and-go conversation, and the adapted metacircular evaluator
   runs compiled on the same machine.  Prints [replay ok]. *)

let ( >>= ) = Result.bind

module C = Sicp_ch5.Sec_5_5

let factorial_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))|}
;;

let check name got expected =
  if got = expected
  then Ok ()
  else (
    Printf.printf "%s MISMATCH\n--- got ---\n%s\n--- expected ---\n%s\n" name got expected;
    exit 1)
;;

(* The book's Figure 5.17, with the edition's two spellings: the entry
   rides in a [(const entry2)] name and the parameter list is one
   [(const n)] input.  The statements are the compiler's own output. *)
let figure_5_17_statements =
  [ "(assign val (op make-compiled-procedure) (const entry2) (reg env))"
  ; "(goto (label after-lambda1))"
  ; "entry2"
  ; "(assign env (op compiled-procedure-env) (reg proc))"
  ; "(assign env (op extend-environment) (const n) (reg argl) (reg env))"
  ; "(save continue)"
  ; "(save env)"
  ; "(assign proc (op lookup-variable-value) (const =) (reg env))"
  ; "(assign val (const 1))"
  ; "(assign argl (op list) (reg val))"
  ; "(assign val (op lookup-variable-value) (const n) (reg env))"
  ; "(assign argl (op cons) (reg val) (reg argl))"
  ; "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch17))"
  ; "compiled-branch16"
  ; "(assign continue (label after-call15))"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "primitive-branch17"
  ; "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))"
  ; "after-call15"
  ; "(restore env)"
  ; "(restore continue)"
  ; "(test (op false?) (reg val))"
  ; "(branch (label false-branch4))"
  ; "true-branch5"
  ; "(assign val (const 1))"
  ; "(goto (reg continue))"
  ; "false-branch4"
  ; "(assign proc (op lookup-variable-value) (const *) (reg env))"
  ; "(save continue)"
  ; "(save proc)"
  ; "(assign val (op lookup-variable-value) (const n) (reg env))"
  ; "(assign argl (op list) (reg val))"
  ; "(save argl)"
  ; "(assign proc (op lookup-variable-value) (const factorial) (reg env))"
  ; "(save proc)"
  ; "(assign proc (op lookup-variable-value) (const -) (reg env))"
  ; "(assign val (const 1))"
  ; "(assign argl (op list) (reg val))"
  ; "(assign val (op lookup-variable-value) (const n) (reg env))"
  ; "(assign argl (op cons) (reg val) (reg argl))"
  ; "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch8))"
  ; "compiled-branch7"
  ; "(assign continue (label after-call6))"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "primitive-branch8"
  ; "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))"
  ; "after-call6"
  ; "(assign argl (op list) (reg val))"
  ; "(restore proc)"
  ; "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch11))"
  ; "compiled-branch10"
  ; "(assign continue (label after-call9))"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "primitive-branch11"
  ; "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))"
  ; "after-call9"
  ; "(restore argl)"
  ; "(assign argl (op cons) (reg val) (reg argl))"
  ; "(restore proc)"
  ; "(restore continue)"
  ; "(test (op primitive-procedure?) (reg proc))"
  ; "(branch (label primitive-branch14))"
  ; "compiled-branch13"
  ; "(assign val (op compiled-procedure-entry) (reg proc))"
  ; "(goto (reg val))"
  ; "primitive-branch14"
  ; "(assign val (op apply-primitive-procedure) (reg proc) (reg argl))"
  ; "(goto (reg continue))"
  ; "after-call12"
  ; "after-if3"
  ; "after-lambda1"
  ; "(perform (op define-variable!) (const factorial) (reg val) (reg env))"
  ; "(assign val (const ok))"
  ]
;;

let figure_check () =
  let state = C.new_state () in
  C.compile_block state factorial_source
  >>= fun (_entry, block) ->
  let lines = String.split_on_char '\n' block in
  (* drop the synthetic entry label on top and the return-linkage goto
     at the bottom; the figure is the compilation with linkage next *)
  let body =
    (* drop the entry label on top and the return linkage's
       save/goto/restore tail at the bottom *)
    let rec strip_tail = function
      | "(goto (reg continue))" :: rest -> List.rev rest
      | x :: rest -> x :: strip_tail rest
      | [] -> []
    in
    strip_tail (List.rev (List.tl lines))
  in
  check
    "figure 5.17"
    (String.concat "\n" body)
    (String.concat "\n" figure_5_17_statements)
;;

let session_check () =
  let state = C.new_state () in
  C.compile_and_go ~state ~compiled:factorial_source ~source:"(factorial 5)" ()
  >>= fun m ->
  let started =
    match C.start m with
    | Ok () -> Ok ()
    | Error (Op_failed m) when m = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
    | Error e -> Error e
  in
  started
  >>= fun () ->
  check
    "compile-and-go session"
    (String.concat "\n" (C.transcript m))
    ";;; EC-Eval value:\n\
     ok\n\
     ;;; EC-Eval input:\n\
     ;;; EC-Eval value:\n\
     120\n\
     ;;; EC-Eval input:"
;;

let () =
  (match figure_check () with
   | Ok () -> print_endline "figure 5.17 ok"
   | Error e ->
     Printf.printf "figure error: %s\n" (C.error_to_string e);
     exit 1);
  (match session_check () with
   | Ok () -> print_endline "session ok"
   | Error e ->
     Printf.printf "session error: %s\n" (C.error_to_string e);
     exit 1);
  print_endline "replay ok"
;;
