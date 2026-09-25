(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.49: a read-compile-execute-print loop.  The host drives
    the loop and calls [compile] and the assembly as operations
    between machine runs -- the book's own suggested arrangement: each
    form is compiled into a fresh block, the machine is assembled with
    it, the external entry runs it, and [print-result] prints the
    value. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** [loop forms] is the read-compile-execute-print session over [forms]:
    one line per form, the value the machine printed. *)
let loop forms =
  let state = C.new_state () in
  let out = ref [] in
  let rec go = function
    | [] -> Ok (List.rev !out)
    | form :: rest ->
      C.compile_block state form
      >>= fun (entry, block) ->
      C.make_compiled_evaluator
        ~controller:(C.eceval_controller ^ "\n" ^ block)
        ~source:""
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
      let lines = C.transcript m in
      out := !out @ [ lines ];
      go rest
  in
  go forms
;;

(** [ex_5_49 ()] runs the loop over a definition and two calls: the
    compiled definitions answer [ok] and the calls answer their
    values, all without an interpreter anywhere in the path. *)
let ex_5_49 () =
  loop
    [ "(define (square n) (* n n))"
    ; "(square 12)"
    ; "(define (twice n) (+ n n))"
    ; "(twice (square 21))"
    ]
  >>= fun sessions -> Ok (List.map (fun lines -> String.concat " " lines) sessions)
;;
