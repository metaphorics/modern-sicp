(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.44: open coding must respect shadowing.  The compiler
    dispatches to the open-coded generators only when the operator
    name is NOT bound in the compile-time environment, so a lambda
    whose parameters are named [+] and [*] compiles its body as
    ordinary procedure applications.  A rebinding through [define] or
    [set!] at top level would not be seen -- the book's own caveat. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind
let shadowed_source = "(lambda (+ * a b x y) (+ (* a x) (* b y)))"
let open_code = { C.default_config with open_code = true }

(** [open_code_ops src] is the number of instructions the compilation
    applies through the machine's open-coded arithmetic, [(op +)] or
    [( * )] over [arg1] and [arg2]. *)
let open_code_ops src =
  let state = C.new_state () in
  match Sicp_common.Reader.read src with
  | Error e -> Error (C.Parse (Sicp_common.Reader.to_string e))
  | Ok exp ->
    C.compile open_code state [] exp "val" C.Next
    >>= fun seq ->
    Ok
      (List.length
         (List.filter
            (fun s ->
               String.length s > 30
               &&
               try
                 let sub = String.sub s 14 24 in
                 String.sub sub 0 24 = "(op +) (reg arg1) (reg "
                 || String.sub sub 0 24 = "(op *) (reg arg1) (reg "
               with
               | _ -> false)
            seq.stmts))
;;

(** [ex_5_44 ()] compiles the linear-combination lambda: with the
    names shadowed by the parameters, zero open-coded operations
    appear; compiling the same body unshadowed open-codes all four. *)
let ex_5_44 () =
  open_code_ops shadowed_source
  >>= fun shadowed_count ->
  open_code_ops "(lambda (a b x y) (+ (* a x) (* b y)))"
  >>= fun free_count ->
  Ok
    [ Printf.sprintf "shadowed parameters: %d open-coded operations" shadowed_count
    ; Printf.sprintf "free names: %d open-coded operations" free_count
    ]
;;
