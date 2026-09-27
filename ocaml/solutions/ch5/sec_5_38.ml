(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.38: open-coded primitives.  The compiler dispatches
    [(= n 1)], [(- n 1)], [(+ a b)], [( * )] and [(<)] calls to code
    that spreads the operands into the machine's [arg1] and [arg2]
    registers and applies the machine's own arithmetic, with the
    remaining argument register preserved around each operand
    evaluation (an operand may itself be an open-coded call).  The
    n-ary [+] and [*] fold through one register.  The whole factorial
    then compiles without a single general procedure call to
    arithmetic, and the answer to (c) is the measured instruction
    counts: the open-coded compilation is roughly half the size. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

let factorial_source =
  {|(define (factorial n)
  (if (= n 1)
      1
      (* (factorial (- n 1)) n)))|}
;;

let open_code = { C.default_config with open_code = true }

(** [compile_count cfg src] is the statement count of the compilation. *)
let compile_count cfg src =
  let state = C.new_state () in
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile cfg state [] exp "val" C.Next >>= fun seq -> Ok (List.length seq.stmts)
;;

(** [run cfg compiled source] runs compiled code on a machine with the
    open-coding registers in place. *)
let run cfg compiled source =
  let state = C.new_state () in
  C.compile_and_go ~cfg ~state ~compiled ~source ()
  >>= fun m ->
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () -> Ok (C.transcript m)
;;

(** [ex_5_38 ()] compiles the factorial both ways, counts the
    statements, and runs the open-coded version: (c)'s comparison as
    measured counts, plus (d)'s n-ary checks. *)
let ex_5_38 () =
  compile_count C.default_config factorial_source
  >>= fun plain_count ->
  compile_count open_code factorial_source
  >>= fun open_count ->
  run open_code factorial_source "(factorial 5)"
  >>= fun factorial_transcript ->
  run open_code "(define (try a) (+ 1 2 3 4))" "(try 0)"
  >>= fun nary_transcript ->
  run open_code "(define (try a b) (< a b))" "(try 1 2)"
  >>= fun cmp_transcript ->
  Ok
    [ Printf.sprintf "plain compilation: %d statements" plain_count
    ; Printf.sprintf "open-coded compilation: %d statements" open_count
    ; "factorial 5: " ^ String.concat " " factorial_transcript
    ; "(+ 1 2 3 4): " ^ String.concat " " nary_transcript
    ; "(< 1 2): " ^ String.concat " " cmp_transcript
    ]
;;
