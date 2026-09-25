(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.43: internal definitions are scanned out before a
    procedure body compiles: the defines become a [let] of
    [*unassigned*] bindings whose values are [set!] after it, the
    transformation 4.1.6 argued for and 4.16 implemented.  The
    compiler's [scan_out] configuration performs it.  Scanning out is
    what makes lexical addressing sound: a [define] executed deep in a
    body would grow the frame the compile-time environment predicted. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

let source =
  {|(define (f)
  (define a 1)
  (define b 2)
  (+ a b))
(f)|}
;;

let scan_out = { C.default_config with scan_out = true }

(** [body_shapes cfg] compiles [f]'s lambda body under [cfg] and
    answers whether the compilation binds [*unassigned*] (the scanned
    shape) or performs [define-variable!] (the plain shape). *)
let body_shape cfg =
  let state = C.new_state () in
  match Sicp_common.Reader.read "(lambda () (define a 1) (define b 2) (+ a b))" with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile cfg state [] exp "val" C.Next
    >>= fun seq ->
    let has prefix =
      List.exists
        (fun s ->
           String.length s >= String.length prefix
           && String.sub s 0 (String.length prefix) = prefix)
        seq.stmts
    in
    Ok (has "(const *unassigned*)", has "(op define-variable!)")
;;

(** [ex_5_43 ()] shows both shapes and runs the scanned program on the
    plain compiled machine: the internal defines answer [3] without
    ever executing a [define]. *)
let ex_5_43 () =
  body_shape C.default_config
  >>= fun (unassigned_plain, define_plain) ->
  body_shape scan_out
  >>= fun (unassigned_scanned, define_scanned) ->
  let state = C.new_state () in
  C.compile_and_go ~cfg:scan_out ~state ~compiled:source ~source:"" ()
  >>= fun _m ->
  let state2 = C.new_state () in
  C.compile_and_go ~cfg:scan_out ~state:state2 ~compiled:source ~source:"(f)" ()
  >>= fun m ->
  (match C.start m with
   | Ok () -> Ok ()
   | Error (C.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted -> Ok ()
   | Error e -> Error e)
  >>= fun () ->
  Ok
    [ Printf.sprintf
        "plain body: *unassigned* = %b, define-variable! = %b"
        unassigned_plain
        define_plain
    ; Printf.sprintf
        "scanned body: *unassigned* = %b, define-variable! = %b"
        unassigned_scanned
        define_scanned
    ; "scanned run: " ^ String.concat " " (C.transcript m)
    ]
;;
