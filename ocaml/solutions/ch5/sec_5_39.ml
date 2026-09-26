(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.39: [lexical-address-lookup] and
    [lexical-address-set!].

    The lexical machine's run-time environments are the book's
    structure carried in object values: a frame is [(names . values)]
    and an environment chains such frames by [cons], the global frame
    at the end.  The edition's pairs are immutable, so every override
    of a frame cell or of a binding cell -- [define-variable!],
    [set-variable-value!], [lexical-address-set!] -- records the new
    content in a shadow table keyed by the cell, and every reader
    consults the shadow first: the same effect as mutation, visible to
    nothing else.  A lookup of [*unassigned*] is the exercise's error.
    The only names a compiled lexical access cannot address are the
    globals, and those ride the same chain through
    [lookup-variable-value]. *)

module C = Sicp_ch5.Sec_5_5
module Ast = Sicp_common.Ast
module Value = Sicp_common.Value

let ( >>= ) = Result.bind

(** The shadow store of a lexical machine: cells whose content was
    overridden, keyed physically by the cell. *)
type shadow = (Value.t, Value.t) Hashtbl.t

let value_is_symbol v name =
  match Value.view v with
  | Value.Symbol s -> String.equal s name
  | _ -> false
;;

(** [read_cell shadow cell] is the cell's content, the shadow first. *)
let read_cell shadow cell =
  match Hashtbl.find_opt shadow cell with
  | Some v -> v
  | None -> cell
;;

(** [values_list v] is the chain of pairs [v] as a list, head first. *)
let values_list v =
  let rec go acc v =
    match Value.view v with
    | Value.Pair (a, more) -> go (a :: acc) more
    | _ -> List.rev acc
  in
  go [] v
;;

(** [name_list v] is a chain of name symbols as a list of strings. *)
let name_list v =
  List.map
    (fun v ->
       match Value.view v with
       | Value.Symbol s -> s
       | _ -> "?")
    (values_list v)
;;

let names_value names =
  List.fold_right (fun n acc -> Value.pair (Value.symbol n) acc) names Value.nil
;;

let list_value values = List.fold_right Value.pair values Value.nil

(** [frame_at env frame] is the environment [frame] levels out. *)
let rec frame_at env frame =
  if frame < 0
  then Error (Sicp_ch5.Sec_5_4.Op_failed "no such frame")
  else if frame = 0
  then Ok env
  else (
    match Value.view env with
    | Value.Pair (_, rest) -> frame_at rest (frame - 1)
    | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "no such frame"))
;;

(** [binding_cell frame env] is the frame cell of the
    environment [frame] levels out -- the frame is the unit the shadow
    overrides. *)
let binding_cell frame env =
  frame_at env frame
  >>= fun fr ->
  match Value.view fr with
  | Value.Pair (cell, _) -> Ok cell
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "malformed frame")
;;

(** [with_value_at i v values] is [values] with position [i] replaced. *)
let with_value_at i v values =
  let rec go k = function
    | [] -> []
    | _ :: rest when k = i -> v :: rest
    | x :: rest -> x :: go (k + 1) rest
  in
  go 0 values
;;

(** [rebind shadow cell names values] records the frame cell's new
    content in the shadow. *)
let rebind shadow cell names values =
  Hashtbl.replace shadow cell (Value.pair (names_value names) (list_value values))
;;

(** [frame_parts shadow cell] is the frame cell's names and values,
    read through the shadow. *)
let frame_parts shadow cell =
  match Value.view (read_cell shadow cell) with
  | Value.Pair (names, values) -> Some (name_list names, values_list values)
  | _ -> None
;;

let index_of name names =
  let rec go i = function
    | [] -> None
    | n :: _ when String.equal n name -> Some i
    | _ :: rest -> go (i + 1) rest
  in
  go 0 names
;;

let nth_opt vs n = List.nth_opt vs n

(** [lexical_lookup shadow frame displacement env] answers the value at
    the address, the shadow first, [unassigned] as the exercise's
    error. *)
let lexical_lookup shadow frame displacement env =
  binding_cell frame env
  >>= fun cell ->
  match frame_parts shadow cell with
  | Some (_, values) ->
    (match List.nth_opt values displacement with
     | Some v when value_is_symbol v "*unassigned*" ->
       Error (Sicp_ch5.Sec_5_4.Op_failed "unassigned variable")
     | Some v -> Ok v
     | None -> Error (Sicp_ch5.Sec_5_4.Op_failed "no such displacement"))
  | None -> Error (Sicp_ch5.Sec_5_4.Op_failed "malformed frame")
;;

(** [lexical_set shadow frame displacement env value] rebinds the
    addressed binding through the frame cell's shadow. *)
let lexical_set shadow frame displacement env value =
  binding_cell frame env
  >>= fun cell ->
  match frame_parts shadow cell with
  | Some (names, values) when displacement < List.length values ->
    rebind shadow cell names (with_value_at displacement value values);
    Ok Value.(symbol "ok")
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "no such displacement")
;;

(** [chain_lookup shadow name env] walks the frames newest first, the
    book's global fallback for the names the compile-time environment
    cannot address. *)
let rec chain_lookup shadow name env =
  match Value.view env with
  | Value.Pair (cell, rest) ->
    (match frame_parts shadow cell with
     | Some (names, values) ->
       (match Option.bind (index_of name names) (nth_opt values) with
        | Some v -> Ok v
        | None -> chain_lookup shadow name rest)
     | None -> Error (Sicp_ch5.Sec_5_4.Op_failed "malformed frame"))
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed ("unbound variable: " ^ name))
;;

(** [chain_set shadow name value env] rebinds the nearest binding
    through its frame cell's shadow. *)
let rec chain_set shadow name value env =
  match Value.view env with
  | Value.Pair (cell, rest) ->
    (match frame_parts shadow cell with
     | Some (names, values) ->
       (match index_of name names with
        | Some i ->
          rebind shadow cell names (with_value_at i value values);
          Ok Value.(symbol "ok")
        | None -> chain_set shadow name value rest)
     | None -> Error (Sicp_ch5.Sec_5_4.Op_failed "malformed frame"))
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed ("unbound variable: " ^ name))
;;

(** [chain_define shadow name value env] binds [name] in the newest
    frame, replacing or appending, through the frame cell's shadow. *)
let chain_define shadow name value env =
  match Value.view env with
  | Value.Pair (cell, _) ->
    (match frame_parts shadow cell with
     | Some (names, values) ->
       (match index_of name names with
        | Some i -> rebind shadow cell names (with_value_at i value values)
        | None -> rebind shadow cell (names @ [ name ]) (values @ [ value ]));
       Ok ()
     | None -> Error (Sicp_ch5.Sec_5_4.Op_failed "malformed frame"))
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "define-variable! needs an environment")
;;

(** [global_frame shadow prims] primes the global frame cell with
    [true], [false], and the object-language primitives, and answers
    the cell. *)
let global_frame shadow prims =
  let cell = Value.pair Value.nil Value.nil in
  let names = [ "true"; "false" ] @ List.map fst prims in
  let values =
    Value.(bool true)
    :: Value.(bool false)
    :: List.map (fun (name, f) -> Value.primitive ~name f) prims
  in
  Hashtbl.replace shadow cell (Value.pair (names_value names) (list_value values));
  cell
;;

(** [global_chain cell] is the top-level environment: the chain of the
    one global frame. *)
let global_chain cell = Value.pair cell Value.nil

(** [name_word w] is the variable name a [V] constant or an [Exp]
    register word carries. *)
let name_word = function
  | Sicp_ch5.Sec_5_4.V v ->
    (match Value.view v with
     | Value.Symbol s -> Ok s
     | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "the constant is not a variable name"))
  | Sicp_ch5.Sec_5_4.Exp e ->
    (match Ast.view e with
     | Ast.Variable n -> Ok n
     | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "the register holds no variable"))
  | w ->
    Error
      (Sicp_ch5.Sec_5_4.Op_failed ("no variable: " ^ Sicp_ch5.Sec_5_4.word_to_string w))
;;

let int_const_of w what =
  match Value.view w with
  | Value.Int n -> Ok n
  | Value.Symbol s ->
    (try Ok (int_of_string s) with
     | Failure _ -> Error (Sicp_ch5.Sec_5_4.Op_failed what))
  | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed what)
;;

(** [extend_lexical_op] is the lexical machine's
    [extend-environment]: compiled code passes the parameter names as
    leading [const] inputs, the interpreted path of compound-apply the
    evaluator's formals word first; both spellings end in the operand
    list and the parent environment, whose chain value becomes the
    tail of the new environment. *)
let extend_lexical_op =
  ( "extend-environment"
  , Sicp_ch5.Sec_5_4.Value_op
      (fun words ->
        let param_name e =
          match Ast.view e with
          | Ast.Variable n -> Ok n
          | _ -> Error (Sicp_ch5.Sec_5_4.Op_failed "a formal is not a variable")
        in
        let rec names_of = function
          | [] -> Ok []
          | e :: rest -> param_name e >>= fun n -> names_of rest >>= fun ns -> Ok (n :: ns)
        in
        let rec split names = function
          | [ Sicp_ch5.Sec_5_4.Args args; Sicp_ch5.Sec_5_4.V base ] ->
            Sicp_ch5.Sec_5_4.word_values args
            >>= fun values ->
            let frame = Value.pair (names_value (List.rev names)) (list_value values) in
            Ok (Sicp_ch5.Sec_5_4.V (Value.pair frame base))
          | [ Sicp_ch5.Sec_5_4.Seq formals
            ; Sicp_ch5.Sec_5_4.Args args
            ; Sicp_ch5.Sec_5_4.V base
            ]
            when names = [] ->
            names_of formals
            >>= fun names ->
            Sicp_ch5.Sec_5_4.word_values args
            >>= fun values ->
            let frame = Value.pair (names_value names) (list_value values) in
            Ok (Sicp_ch5.Sec_5_4.V (Value.pair frame base))
          | Sicp_ch5.Sec_5_4.V v :: rest ->
            (match Value.view v with
             | Value.Symbol s -> split (s :: names) rest
             | _ ->
               Error (Sicp_ch5.Sec_5_4.Op_failed "a parameter name constant is a symbol"))
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Op_failed
                 "extend-environment needs names, an operand list, and an environment")
        in
        split [] words) )
;;

(** [lexical_operations shadow global] is the lexical machine's table:
    the structure builder, the addressing operations, the chain-walking
    global fallbacks, and the global environment.  [global] is the
    top-level chain [global_chain] built. *)
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
    , Sicp_ch5.Sec_5_4.Action_op
        (function
          | [ Sicp_ch5.Sec_5_4.V frame
            ; Sicp_ch5.Sec_5_4.V displacement
            ; Sicp_ch5.Sec_5_4.V value
            ; Sicp_ch5.Sec_5_4.V env
            ] ->
            int_const_of frame "the lexical address is two constants"
            >>= fun f ->
            int_const_of displacement "the lexical address is two constants"
            >>= fun d -> lexical_set shadow f d env value >>= fun _ -> Ok ()
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Arity
                 "lexical-address-set! needs an address, an environment, and a value")) )
  in
  let lookup_global =
    ( "lookup-variable-value"
    , Sicp_ch5.Sec_5_4.Value_op
        (function
          | [ w; Sicp_ch5.Sec_5_4.V env ] ->
            name_word w
            >>= fun name ->
            chain_lookup shadow name env >>= fun v -> Ok (Sicp_ch5.Sec_5_4.V v)
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Arity
                 "lookup-variable-value needs a variable and an environment")) )
  in
  let set_global =
    ( "set-variable-value!"
    , Sicp_ch5.Sec_5_4.Action_op
        (function
          | [ w; Sicp_ch5.Sec_5_4.V value; Sicp_ch5.Sec_5_4.V env ] ->
            name_word w >>= fun name -> chain_set shadow name value env >>= fun _ -> Ok ()
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Arity
                 "set-variable-value! needs a variable, a value, and an environment")) )
  in
  let define =
    ( "define-variable!"
    , Sicp_ch5.Sec_5_4.Action_op
        (function
          | [ w; Sicp_ch5.Sec_5_4.V value; Sicp_ch5.Sec_5_4.V env ] ->
            name_word w >>= fun name -> chain_define shadow name value env
          | _ ->
            Error
              (Sicp_ch5.Sec_5_4.Arity
                 "define-variable! needs a variable, a value, and an environment")) )
  in
  let global_environment =
    ( "get-global-environment"
    , Sicp_ch5.Sec_5_4.Value_op (fun _ -> Ok (Sicp_ch5.Sec_5_4.V global)) )
  in
  [ extend_lexical_op
  ; lookup_lexical
  ; set_lexical
  ; lookup_global
  ; set_global
  ; define
  ; global_environment
  ]
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

(** [values_of transcript] is the printed values: every line that
    follows a value announcement. *)
let rec values_of transcript =
  match transcript with
  | announce :: value :: rest when String.equal announce ";;; EC-Eval value:" ->
    value :: values_of rest
  | _ :: rest -> values_of rest
  | [] -> []
;;

(** [run_lexical source] compiles [source] with lexical addressing and
    runs it on the lexical machine. *)
let run_lexical source =
  let shadow : shadow = Hashtbl.create 16 in
  let global =
    global_frame shadow (Sicp_ch5.Sec_5_5.build_runtime_primitives ()) |> global_chain
  in
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
    compiled lexical machine: a scanned-out [set!] rebinding through
    the global fallback, a compiled closure reading its captured frame
    through [lexical-address-lookup], and a compiled [set!] on that
    frame through [lexical-address-set!].  The program answers one
    value, its last form's: the counter must have accumulated 1 then 2
    across the two calls and the cell 107, so [(+ (c) (c) (cell))]
    answers 110. *)
let ex_5_39 () =
  let source =
    {|(define (make-counter)
  (define count 0)
  (lambda ()
    (set! count (+ count 1))
    count))
(define c (make-counter))
(define (make-cell n)
  (lambda ()
    (set! n (+ n 100))
    n))
(define cell (make-cell 7))
(+ (c) (c) (cell))|}
  in
  run_lexical source
  >>= fun transcript ->
  Ok [ "lexical machine session: " ^ String.concat " " (values_of transcript) ]
;;
