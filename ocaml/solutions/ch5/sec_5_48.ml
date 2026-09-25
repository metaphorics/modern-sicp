(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.48: [compile-and-run] as an evaluator primitive.  The
    primitive compiles its (quoted) argument at run time and answers
    [ok]; the compiled block lands in the machine's controller when
    the next assembly is built -- the edition's controller is parsed
    text, so the book's live patch of a running machine is reproduced
    as the next assembled machine with the same operations and a
    continued session.  The observable session is the book's. *)

module C = Sicp_ch5.Sec_5_5
module Ast = Sicp_common.Ast

let ( >>= ) = Result.bind

(** [datum_of_value v] is the quoted datum a [V] word carries. *)
let rec datum_of_value v =
  match Sicp_common.Value.view v with
  | Sicp_common.Value.Int n -> Ast.DInt n
  | Sicp_common.Value.Bool b -> Ast.DBool b
  | Sicp_common.Value.Symbol s -> Ast.DSymbol s
  | Sicp_common.Value.String s -> Ast.DString s
  | Sicp_common.Value.Nil -> Ast.DNil
  | Sicp_common.Value.Pair (a, d) -> Ast.DPair (datum_of_value a, datum_of_value d)
  | _ -> Ast.DSymbol "?"
;;

(** [compile_and_run_op state blocks] is the primitive: it reads the
    quoted expression out of the argument value, compiles it into a
    controller block, and records the block for the next assembly. *)
let compile_and_run_op state blocks =
  ( "compile-and-run"
  , C.Value_op
      (function
        | [ Sicp_ch5.Sec_5_4.V v ] ->
          let datum = Ast.DPair (Ast.DSymbol "begin", datum_of_value v) in
          let exp = Ast.quote datum in
          C.compile_program state [ exp ]
          >>= fun seq ->
          let entry = "compiled-entry-run-" ^ string_of_int (List.length !blocks) in
          blocks := !blocks @ [ entry ^ "\n" ^ C.statements_text seq ];
          Ok (Sicp_ch5.Sec_5_4.V (Sicp_common.Value.symbol "ok"))
        | _ -> Error (C.Arity "compile-and-run needs one quoted expression")) )
;;

(** [ex_5_48 ()] runs the book's session: the compile-and-run define,
    then the call.  The first machine answers ok and hands the block
    on; the second machine, assembled with the block, answers 120. *)
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
    ~operations:[ compile_and_run_op state blocks ]
    ~source:session_source
    ~state
    ()
  >>= fun m1 ->
  (match C.start m1 with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () ->
  let first_transcript = C.transcript m1 in
  match !blocks with
  | [ block ] ->
    let controller = C.eceval_controller ^ "\n" ^ block in
    C.make_compiled_evaluator ~controller ~source:"(factorial 5)" ~state ()
    >>= fun m2 ->
    (match C.start m2 with
     | Ok () -> Ok ()
     | Error (C.Op_failed m3) when m3 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
     | Error e -> Error e)
    >>= fun () -> Ok (first_transcript @ C.transcript m2)
  | _ -> Error (C.Op_failed "compile-and-run produced no block")
;;
