(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.39: [lexical-address-lookup] and
    [lexical-address-set!].

    The lexical machine's run-time environments are the book's
    structure carried in object values: a frame is [(names . values)]
    and an environment chains such frames by [cons].  The edition's
    pairs are immutable, so [lexical-address-set!] records the new
    value in a shadow table keyed by the cons cell holding the
    binding -- the same effect as mutation, visible to nothing else.
    A lookup of [*unassigned*] is the exercise's error. *)

module C = Sicp_ch5.Sec_5_5

let ( >>= ) = Result.bind

(** The shadow store of a lexical machine: cells whose bindings were
    set, keyed physically by the cell. *)
type shadow = (Sicp_common.Value.t, Sicp_common.Value.t) Hashtbl.t

let value_is_symbol v name =
  match Sicp_common.Value.view v with
  | Sicp_common.Value.Symbol s -> String.equal s name
  | _ -> false
;;

(** [frame_cell values displacement] is the cons cell holding the
    binding at [displacement]. *)
let frame_cell values displacement =
  let rec go n v =
    match n, Sicp_common.Value.view v with
    | 0, Sicp_common.Value.Pair (cell, _) -> Ok cell
    | 0, _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "no such displacement")
    | _, Sicp_common.Value.Pair (_, more) -> go (n - 1) more
    | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "frame values exhausted")
  in
  go displacement values
;;

(** [frame_at env frame] is the environment [frame] levels out. *)
let rec frame_at env frame =
  if frame < 0
  then Error (Sicp_ch5.Sec_5_4.Op_failed "no such frame")
  else if frame = 0
  then Ok env
  else (
    match Sicp_common.Value.view env with
    | Sicp_common.Value.Pair (_, rest) -> frame_at rest (frame - 1)
    | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "no such frame"))
;;

(** [binding_cell shadow frame displacement env] is the cell of the
    binding at the lexical address. *)
let binding_cell frame displacement env =
  frame_at env frame
  >>= fun fr ->
  match Sicp_common.Value.view fr with
  | Sicp_common.Value.Pair (frame_cell_v, _) ->
    (match Sicp_common.Value.view frame_cell_v with
     | Sicp_common.Value.Pair (_, values) -> frame_cell values displacement
     | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "malformed frame"))
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "malformed frame")
;;

(** [lexical_lookup shadow frame displacement env] answers the value at
    the address, the shadow first, [unassigned] as the exercise's
    error. *)
let lexical_lookup shadow frame displacement env =
  binding_cell frame displacement env
  >>= fun cell ->
  match Hashtbl.find_opt shadow cell with
  | Some v -> Ok v
  | None ->
    (match Sicp_common.Value.view cell with
     | Sicp_common.Value.Pair (v, _) when value_is_symbol v "*unassigned*" ->
       Error (Sicp_ch5.Sec_5_4.Op_failed "unassigned variable")
     | Sicp_common.Value.Pair (v, _) -> Ok v
     | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "binding is not a pair"))
;;

(** [lexical_set shadow frame displacement env value] shadows the
    binding cell's content. *)
let lexical_set shadow frame displacement env value =
  binding_cell frame displacement env
  >>= fun cell ->
  Hashtbl.replace shadow cell value;
  Ok Sicp_common.Value.(symbol "ok")
;;

(** [driver_plain] is the plain driver fragment for the lexical
    machine. *)
let driver_plain =
  ";; branches if flag is set:\n(branch (label external-entry))\n"
  ^ {|read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op announce-output))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))|}
;;

let names_value names =
  List.fold_right
    (fun n acc -> Sicp_common.Value.pair (Sicp_common.Value.symbol n) acc)
    names
    Sicp_common.Value.nil
;;

let list_value values =
  List.fold_right Sicp_common.Value.pair values Sicp_common.Value.nil
;;

(** [extend_lexical_op] is the lexical machine's
    [extend-environment]: the parameter names arrive as [const] inputs
    before the operand list and the parent environment. *)
let extend_lexical_op =
  ( "extend-environment"
  , Sicp_ch5.Sec_5_4.Value_op
      (fun words ->
        let rec split names = function
          | [ Sicp_ch5.Sec_5_4.Args args; Sicp_ch5.Sec_5_4.V base ] ->
            (match Sicp_common.Value.view base with
             | Sicp_common.Value.Symbol _ ->
               Sicp_ch5.Sec_5_4.word_values args
               >>= fun values ->
               let frame =
                 Sicp_common.Value.pair (names_value (List.rev names)) (list_value values)
               in
               Ok (Sicp_ch5.Sec_5_4.V (Sicp_common.Value.pair frame base))
             | _ ->
               Error
                 (Sicp_ch5.Sec_5_4.Op_failed
                    "extend-environment needs the parent environment"))
          | Sicp_ch5.Sec_5_4.V v :: rest ->
            (match Sicp_common.Value.view v with
             | Sicp_common.Value.Symbol s -> split (s :: names) rest
             | _ ->
               Error (Sicp_ch5.Sec_5_4.Op_failed "a parameter name constant is a symbol"))
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Op_failed
                 "extend-environment needs names, an operand list, and an environment")
        in
        split [] words) )
;;

let int_const_of w what =
  match Sicp_common.Value.view w with
  | Sicp_common.Value.Symbol s ->
    (try Ok (int_of_string s) with
     | Failure _ -> Error (Sicp_ch5.Sec_5_4.Op_failed what))
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed what)
;;

(** [lexical_operations shadow global] is the lexical machine's table:
    the structure builder, the addressing operations, and the global
    fallback [lookup-variable-value] -- the only names the
    compile-time environment cannot address are the globals. *)
let lexical_operations shadow global =
  let lookup_lexical =
    ( "lexical-address-lookup"
    , Sicp_ch5.Sec_5_4.Value_op
        (function
          | [ Sicp_ch5.Sec_5_4.V frame
            ; Sicp_ch5.Sec_5_4.V displacement
            ; Sicp_ch5.Sec_5_4.V env
            ] ->
            int_const_of frame "the lexical address is two constants"
            >>= fun f ->
            int_const_of displacement "the lexical address is two constants"
            >>= fun d ->
            lexical_lookup shadow f d env >>= fun v -> Ok (Sicp_ch5.Sec_5_4.V v)
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Arity
                 "lexical-address-lookup needs an address and an environment")) )
  in
  let set_lexical =
    ( "lexical-address-set!"
    , Sicp_ch5.Sec_5_4.Value_op
        (function
          | [ Sicp_ch5.Sec_5_4.V frame
            ; Sicp_ch5.Sec_5_4.V displacement
            ; Sicp_ch5.Sec_5_4.V env
            ; Sicp_ch5.Sec_5_4.V value
            ] ->
            int_const_of frame "the lexical address is two constants"
            >>= fun f ->
            int_const_of displacement "the lexical address is two constants"
            >>= fun d ->
            lexical_set shadow f d env value >>= fun v -> Ok (Sicp_ch5.Sec_5_4.V v)
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Arity
                 "lexical-address-set! needs an address, an environment, and a value")) )
  in
  let lookup_global =
    ( "lookup-variable-value"
    , Sicp_ch5.Sec_5_4.Value_op
        (function
          | [ Sicp_ch5.Sec_5_4.V v; _ ] ->
            (match Sicp_common.Value.view v with
             | Sicp_common.Value.Symbol name ->
               (match Sicp_common.Env.find_binding global name with
                | Some v -> Ok (Sicp_ch5.Sec_5_4.V v)
                | None -> Error (Sicp_ch5.Sec_5_4.Op_failed ("unbound variable: " ^ name)))
             | _ ->
               Error (Sicp_ch5.Sec_5_4.Op_failed "lookup-variable-value needs a name"))
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Arity
                 "lookup-variable-value needs a variable and an environment")) )
  in
  [ extend_lexical_op; lookup_lexical; set_lexical; lookup_global ]
;;

(** [run_lexical source] compiles [source] with lexical addressing and
    runs it on the lexical machine. *)
let run_lexical source =
  let shadow : shadow = Hashtbl.create 16 in
  let global = Sicp_common.Env.empty () in
  Sicp_common.Env.define global "true" (Sicp_common.Value.bool true);
  Sicp_common.Env.define global "false" (Sicp_common.Value.bool false);
  List.iter
    (fun (name, f) ->
       Sicp_common.Env.define global name (Sicp_common.Value.primitive ~name f))
    (Sicp_ch5.Sec_5_5.build_runtime_primitives ());
  let state = C.new_state () in
  C.compile_block ~cfg:{ C.default_config with lexical = true } state source
  >>= fun (entry, block) ->
  let controller =
    String.concat
      "\n"
      (List.map
         (fun (nm, text) -> if nm = "driver" then driver_plain else text)
         C.eceval_fragments)
  in
  C.make_compiled_evaluator
    ~controller:(controller ^ "\n" ^ block)
    ~operations:(lexical_operations shadow global)
    ~source:""
    ~state
    ()
  >>= fun m ->
  C.set_register m "val" (Sicp_ch5.Sec_5_4.Lab entry)
  >>= fun () ->
  C.set_flag m true;
  (match C.start m with
   | Ok () -> Ok ()
   | Error (Sicp_ch5.Sec_5_4.Op_failed m2) when m2 = Sicp_ch5.Sec_5_4.input_exhausted ->
     Ok ()
   | Error e -> Error e)
  >>= fun () -> Ok (C.transcript m)
;;

(** [ex_5_39 ()] exercises the addressing operations through the
    compiled lexical machine: a compiled closure reading its captured
    frame, a scanned-out [set!] rebinding through
    [lexical-address-set!], and an untouched [*unassigned*] binding
    as the error. *)
let ex_5_39 () =
  let with_set =
    {|(define (make-counter)
  (define count 0)
  (lambda ()
    (set! count (+ count 1))
    count))
(define c (make-counter))
(c)
(c)|}
  in
  run_lexical with_set
  >>= fun counted -> Ok [ "lexical machine session: " ^ String.concat " " counted ]
;;
