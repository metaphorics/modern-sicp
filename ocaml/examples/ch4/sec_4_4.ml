(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.4 *)

(** The query system of section 4.4 on the chapter's substrate: the same
    typed [Value] domain the 4.1 evaluator computes over -- assertions,
    rules, patterns, and frames are values, pairs and symbols included --
    and the chapter 3 memoized stream as the one lazy-stream choice
    ([Sicp_ch3.Sec_3_5.Streams]: [Cons] of an element and a [Lazy.t] tail;
    every [delay]/[force] of the book is that cell). The four layers of the
    book keep their shape: the driver loop ([run]), the evaluator
    ([qeval] and its data-directed dispatch), the matcher and unifier
    ([pattern_match], [unify_match]), and the frame/database machinery.

    {2 Frames}

    A frame is an immutable association list of bindings, the book's
    representation: [extend] conses one [(variable, value)] pair in front,
    [binding_in_frame] walks the list under [structural_equal]. There is no
    marked inhabitant: a frame never stores a sentinel for ``no binding'',
    and an absent variable is absence, not a value -- the matcher's failure
    is the typed [option] ([None]), not a [failed] symbol living in the data.
    Frame variables are the internal [(? name)] lists the book's
    [query-syntax-process] produces, so a variable is a value distinct from
    every data symbol, and a scanned-out rule variable renamed to
    [(? id name)] cannot collide with any explicitly written [(? name)]
    (4.4.4.7).

    {2 The data base}

    [THE-ASSERTIONS] and [THE-RULES] are chronological: the book's
    [add-assertion!] conses the newest entry in front, yet every sample
    interaction the book pins -- the [job] queries of 4.4.1, the [or] pairs,
    [lives-near] -- lists answers in the order the assertions were entered,
    so the edition stores assertions and rules in insertion order (lists
    appended at the end) behind the same stream interface, indexed by the
    leading symbol exactly as 4.4.4.5 describes. The index streams and the
    dispatch table ([put]/[get], the 2.4.2 table) are the book's.

    {2 The query syntax reader}

    The driver reads query text with this module's own datum scanner, not
    the shared expression reader: the query language is a data language --
    the empty list [()] and dotted-tail patterns such as [(computer .
    ?type)] are ordinary patterns, and tokens such as [9am] (4.59's meeting
    times) are symbols -- while the shared Scheme reader rejects [()], rejects
    dotted call forms, and would classify [9am] as a malformed number. The
    lexical conventions are the shared reader's: parentheses, [;] comments,
    strings with the same two escapes, [#t]/[#f]; a token reads as an
    integer, then a float, then a symbol. [query-syntax-process] then
    expands [?x] to the internal [(? x)] exactly as the book does.

    {2 Errors}

    Object-level failures of the query machinery (a malformed query, an
    unbound pattern variable under [lisp-value], an unknown predicate)
    travel as the typed [Raised] exception inside stream production; the
    driver catches it and answers the [Eval_error] the substrate shares.
    As with the book's driver, an error aborts the answer being produced. *)

let ( >>= ) = Result.bind

module Streams = Sicp_ch3.Sec_3_5.Streams
module SE = Sec_4_1
module Eval_error = Sicp_common.Eval_error
module Env = Sicp_common.Env
module Value = Sicp_common.Value

(** The typed error across lazy stream production; [run] reports it as the
    [Eval_error] the rest of the substrate answers with. *)
exception Raised of Eval_error.t

let raise_error e = raise (Raised e)

(** One frame: the book's list of bindings, newest first. *)
type frame = (Value.t * Value.t) list

(** The book's empty frame, assigning no variables. *)
let the_empty_frame : frame = []

(** [binding_in_frame variable frame] is the [(variable, value)] pair of
    [variable], or [None]; [structural_equal] over the internal variable
    values is the book's [equal?] [assoc]. *)
let binding_in_frame variable frame =
  List.find_opt (fun (v, _) -> Value.structural_equal v variable) frame
;;

(** [frame_bindings frame] is the association list itself, newest first. *)
let frame_bindings frame = frame

(** [extend variable value frame] is the frame with one more binding in
    front. *)
let extend variable value frame = (variable, value) :: frame

(** {2 4.4.4.7: the query syntax procedures} *)

(** [is_var exp] holds for the internal pattern variables: lists whose car
    is the symbol [?]. *)
let is_var exp =
  match Value.view exp with
  | Value.Pair (car, _) -> Value.structural_equal car (Value.symbol "?")
  | _ -> false
;;

(** [constant_symbol?]: a plain symbol, the indexable head of a pattern. *)
let is_constant_symbol exp =
  match Value.view exp with
  | Value.Symbol _ -> true
  | _ -> false
;;

exception Query_read_error of string

let query_read_error fmt = Format.ksprintf (fun s -> raise (Query_read_error s)) fmt

(* The datum scanner described in the module doc. The delimiters are the
   shared reader's; the input is a one-character pushback stream. *)
let is_delimiter = function
  | ' ' | '\t' | '\r' | '\n' | '(' | ')' | '"' | ';' | '\'' -> true
  | _ -> false
;;

type chars =
  { source : string
  ; mutable pos : int
  ; mutable pushed : char option
  }

let next chars =
  match chars.pushed with
  | Some c ->
    chars.pushed <- None;
    Some c
  | None ->
    if chars.pos >= String.length chars.source
    then None
    else (
      let c = chars.source.[chars.pos] in
      chars.pos <- chars.pos + 1;
      Some c)
;;

let push chars c = chars.pushed <- Some c

let rec skip_atmosphere chars =
  match next chars with
  | None -> ()
  | Some ';' ->
    let rec line () =
      match next chars with
      | None | Some '\n' -> skip_atmosphere chars
      | Some _ -> line ()
    in
    line ()
  | Some c when c = ' ' || c = '\t' || c = '\r' || c = '\n' -> skip_atmosphere chars
  | Some c -> push chars c
;;

let is_integer_token token =
  let body =
    match token.[0] with
    | '+' | '-' -> String.sub token 1 (String.length token - 1)
    | _ -> token
  in
  body <> "" && String.for_all (fun c -> c >= '0' && c <= '9') body
;;

let digits body = body <> "" && String.for_all (fun c -> c >= '0' && c <= '9') body

let is_float_token token =
  match String.index_opt token '.' with
  | None -> false
  | Some k ->
    let pre = String.sub token 0 k in
    let post = String.sub token (k + 1) (String.length token - k - 1) in
    (match String.index_opt post 'e', String.index_opt post 'E' with
     | Some e, _ | None, Some e ->
       let mantissa = String.sub post 0 e in
       let exponent = String.sub post (e + 1) (String.length post - e - 1) in
       digits pre && digits mantissa && exponent <> "" && digits exponent
     | None, None -> digits pre && digits post)
;;

let read_token chars =
  let buffer = Buffer.create 16 in
  let rec go () =
    match next chars with
    | Some c when not (is_delimiter c) ->
      Buffer.add_char buffer c;
      go ()
    | Some c ->
      push chars c;
      ()
    | None -> ()
  in
  go ();
  Buffer.contents buffer
;;

(** [read_value chars] reads one datum: an atom or a list with the dotted
    tail, the shared reader's conventions with the data-language atom rule
    (an integer, then a float, then [#t]/[#f], then a symbol). *)
let rec read_value chars =
  skip_atmosphere chars;
  match next chars with
  | None -> query_read_error "unexpected end of query input"
  | Some ')' -> query_read_error "unexpected )"
  | Some '\'' -> query_read_error "the query language has no quote"
  | Some '"' ->
    let buffer = Buffer.create 16 in
    let rec go () =
      match next chars with
      | None -> query_read_error "unterminated string"
      | Some '"' -> ()
      | Some '\\' ->
        (match next chars with
         | Some '"' ->
           Buffer.add_char buffer '"';
           go ()
         | Some '\\' ->
           Buffer.add_char buffer '\\';
           go ()
         | _ -> query_read_error "unknown string escape")
      | Some c ->
        Buffer.add_char buffer c;
        go ()
    in
    go ();
    Value.string (Buffer.contents buffer)
  | Some '(' -> read_list chars
  | Some first ->
    push chars first;
    let token = read_token chars in
    if token = ""
    then query_read_error "a query datum cannot start that character"
    else if is_integer_token token
    then (
      match int_of_string_opt token with
      | Some n -> Value.int n
      | None -> query_read_error "malformed integer %s" token)
    else if is_float_token token
    then (
      match float_of_string_opt token with
      | Some f -> Value.float f
      | None -> query_read_error "malformed number %s" token)
    else if token = "#t"
    then Value.bool true
    else if token = "#f"
    then Value.bool false
    else if token = "."
    then query_read_error "`.` outside a list"
    else Value.symbol token

and read_list chars =
  let rec loop elements =
    skip_atmosphere chars;
    match next chars with
    | None -> query_read_error "unexpected end of query input"
    | Some ')' ->
      (* [elements] is newest first; the fold consumes it tail first. *)
      List.fold_left (fun tail d -> Value.pair d tail) Value.nil elements
    | Some '.' ->
      (match next chars with
       | Some c when is_delimiter c ->
         push chars c;
         (match elements with
          | [] -> query_read_error "`.` with no preceding element"
          | _ ->
            let tail = read_value chars in
            skip_atmosphere chars;
            (match next chars with
             | Some ')' -> List.fold_left (fun tail d -> Value.pair d tail) tail elements
             | _ -> query_read_error "garbage after the dot tail"))
       | _ -> query_read_error "a dot must stand alone")
    | Some c ->
      push chars c;
      loop (read_value chars :: elements)
  in
  loop []
;;

(** [read_query text] reads the one datum of [text] as a value, ignoring
    everything after it, or the scanner's message. *)
let read_query text =
  let chars = { source = text; pos = 0; pushed = None } in
  try
    skip_atmosphere chars;
    if chars.pos >= String.length text && chars.pushed = None
    then Error "the query input is empty"
    else Ok (read_value chars)
  with
  | Query_read_error message -> Error message
;;

(** [query_syntax_process exp] expands every [?x] symbol to the internal
    [(? x)], the book's [map-over-symbols] walk. *)
let rec query_syntax_process exp =
  match Value.view exp with
  | Value.Pair (car, cdr) ->
    Value.pair (query_syntax_process car) (query_syntax_process cdr)
  | Value.Symbol name when String.length name > 0 && name.[0] = '?' ->
    Value.pair
      (Value.symbol "?")
      (Value.pair (Value.symbol (String.sub name 1 (String.length name - 1))) Value.nil)
  | _ -> exp
;;

(** [contract_question_mark variable] is the external spelling of an
    internal variable: [(? x)] reads back as [?x], [(? 7 x)] as [?x-7]. *)
let contract_question_mark variable =
  match Value.view variable with
  | Value.Pair (_, second_cell) ->
    (match Value.view second_cell with
     | Value.Pair (second, rest) ->
       (match Value.view second, Value.view rest with
        | Value.Int n, Value.Pair (name, tail) when Value.structural_equal tail Value.nil
          ->
          (* the renamed form [(? id name)] reads as [?name-id] *)
          Value.symbol ("?" ^ Value.to_string name ^ "-" ^ string_of_int n)
        | _ -> Value.symbol ("?" ^ Value.to_string second))
     | _ -> variable)
  | _ -> variable
;;

(** [make_new_variable var id] is the renamed variable [(? id name)], the
    book's [make-new-variable]. *)
let make_new_variable variable id =
  match Value.view variable with
  | Value.Pair (q, name) -> Value.pair q (Value.pair (Value.int id) name)
  | _ -> variable
;;

(** {2 4.4.4.8: instantiation} *)

(** [instantiate exp frame unbound_var_handler] copies [exp], replacing
    every variable by its frame value -- itself instantiated, since a
    unification may bind a variable to a pattern -- and hands an unbound
    variable to the handler. *)
let rec instantiate exp frame unbound_var_handler =
  if is_var exp
  then (
    match binding_in_frame exp frame with
    | Some (_, value) -> instantiate value frame unbound_var_handler
    | None -> unbound_var_handler exp frame)
  else (
    match Value.view exp with
    | Value.Pair (car, cdr) ->
      Value.pair
        (instantiate car frame unbound_var_handler)
        (instantiate cdr frame unbound_var_handler)
    | _ -> exp)
;;

(** {2 4.4.4.6: the stream operations of the query system} *)

(** [singleton_stream x] is the one-element stream. *)
let singleton_stream x = Streams.cons_stream x (fun () -> Streams.the_empty_stream)

(** [stream_append s1 s2] is the ordinary append of 3.5.3, used where both
    streams are finite candidates (the rule index). *)
let rec stream_append s1 s2 =
  if Streams.stream_null s1
  then s2
  else
    Streams.cons_stream (Streams.stream_car s1) (fun () ->
      stream_append (Streams.stream_cdr s1) s2)
;;

(** [stream_append_delayed s1 delayed_s2] appends, forcing the second
    stream only when the first runs out (the explicit delay of the book's
    4.4.4.6, which postpones looping, 4.71). *)
let rec stream_append_delayed s1 delayed_s2 =
  if Streams.stream_null s1
  then delayed_s2 ()
  else
    Streams.cons_stream (Streams.stream_car s1) (fun () ->
      stream_append_delayed (Streams.stream_cdr s1) delayed_s2)
;;

(** [interleave_delayed s1 delayed_s2] alternates the two streams, the
    second forced only when first needed (4.72). *)
let rec interleave_delayed s1 delayed_s2 =
  if Streams.stream_null s1
  then delayed_s2 ()
  else
    Streams.cons_stream (Streams.stream_car s1) (fun () ->
      interleave_delayed (delayed_s2 ()) (fun () -> Streams.stream_cdr s1))
;;

(** [flatten_stream stream] interleaves a stream of streams; its explicit
    delay keeps the rest of the input from being demanded before the head
    stream is consumed (4.73). *)
let rec flatten_stream stream =
  if Streams.stream_null stream
  then Streams.the_empty_stream
  else
    interleave_delayed (Streams.stream_car stream) (fun () ->
      flatten_stream (Streams.stream_cdr stream))
;;

(** [stream_flatmap proc s] maps [proc] over [s] and interleaves the
    resulting streams. *)
let stream_flatmap proc s = flatten_stream (Streams.stream_map proc s)

(** {2 4.4.4.5: the data base}

    Insertion order under the stream interface; the index is the book's
    two-dimensional [put]/[get] table keyed by the leading symbol and the
    kind. *)

let list_to_stream items =
  List.fold_right
    (fun v acc -> Streams.cons_stream v (fun () -> acc))
    items
    Streams.the_empty_stream
;;

let stream_to_list s =
  let rec go acc s =
    if Streams.stream_null s
    then List.rev acc
    else go (Streams.stream_car s :: acc) (Streams.stream_cdr s)
  in
  go [] s
;;

let the_assertions : Value.t list ref = ref []
let the_rules : Value.t list ref = ref []
let the_index : (string * string, Value.t list ref) Hashtbl.t = Hashtbl.create 16
let rule_counter = ref 0

(** [new_rule_application_id] is the fresh renaming id of one rule
    application. *)
let new_rule_application_id () =
  incr rule_counter;
  !rule_counter
;;

let index_key_of pattern =
  match Value.view pattern with
  | Value.Pair (car, _) when is_var car -> "?"
  | Value.Pair (car, _) -> Value.to_string car
  | _ -> query_read_error "a pattern must be a list"
;;

let indexable pattern =
  match Value.view pattern with
  | Value.Pair (car, _) -> is_var car || is_constant_symbol car
  | _ -> false
;;

let use_index pattern =
  match Value.view pattern with
  | Value.Pair (car, _) -> is_constant_symbol car
  | _ -> false
;;

let get_stream key1 key2 =
  match Hashtbl.find_opt the_index (key1, key2) with
  | Some cell -> list_to_stream !cell
  | None -> Streams.the_empty_stream
;;

let put_index key1 key2 assertion =
  match Hashtbl.find_opt the_index (key1, key2) with
  | Some cell -> cell := !cell @ [ assertion ]
  | None -> Hashtbl.add the_index (key1, key2) (ref [ assertion ])
;;

let get_all_assertions () = list_to_stream !the_assertions
let get_indexed_assertions pattern = get_stream (index_key_of pattern) "assertion-stream"

(** [fetch_assertions pattern frame] is the candidate assertions for the
    pattern: the index when the pattern starts with a constant symbol, all
    assertions otherwise. *)
let fetch_assertions pattern frame =
  ignore frame;
  if use_index pattern then get_indexed_assertions pattern else get_all_assertions ()
;;

let get_all_rules () = list_to_stream !the_rules

let get_indexed_rules pattern =
  stream_append
    (get_stream (index_key_of pattern) "rule-stream")
    (get_stream "?" "rule-stream")
;;

(** [fetch_rules pattern frame] is the candidate rules: those indexed under
    the pattern's key and under [?], or all rules. *)
let fetch_rules pattern frame =
  ignore frame;
  if use_index pattern then get_indexed_rules pattern else get_all_rules ()
;;

let store_assertion_in_index assertion =
  if indexable assertion
  then put_index (index_key_of assertion) "assertion-stream" assertion
;;

let store_rule_in_index rule =
  match Value.view rule with
  | Value.Pair (_, cell) ->
    (match Value.view cell with
     | Value.Pair (conclusion, _) ->
       if indexable conclusion then put_index (index_key_of conclusion) "rule-stream" rule
     | _ -> raise_error (Eval_error.Invalid_form "a rule must be a list"))
  | _ -> raise_error (Eval_error.Invalid_form "a rule must be a list")
;;

(** [add_assertion! assertion] stores it in the index and appends it to the
    data base in insertion order, the old collection bound before the new
    one is installed -- the discipline the book's [let] enforces (4.70). *)
let add_assertion assertion =
  store_assertion_in_index assertion;
  let old_assertions = !the_assertions in
  the_assertions := old_assertions @ [ assertion ];
  Value.symbol "ok"
;;

(** [add_rule! rule] is the rule side of the same discipline. *)
let add_rule rule =
  store_rule_in_index rule;
  let old_rules = !the_rules in
  the_rules := old_rules @ [ rule ];
  Value.symbol "ok"
;;

(** [is_rule statement] holds for [(rule ...)] forms. *)
let is_rule statement =
  match Value.view statement with
  | Value.Pair (car, _) -> Value.structural_equal car (Value.symbol "rule")
  | _ -> false
;;

(** [add_rule_or_assertion! statement] files one [assert!] body. *)
let add_rule_or_assertion statement =
  if is_rule statement then add_rule statement else add_assertion statement
;;

(** {2 4.4.4.7: the selectors of the special forms and of rules} *)

(** The typed dispatch table of [qeval], the book's [(put key 'qeval proc)]
    over the 2.4.2 table: one handler per special form, registered once
    under its object-language name. A handler receives the session
    environment (filters such as [lisp_value] evaluate host predicates in
    it), the contents of the tagged query, and the frame stream. *)
type qproc = Value.env -> Value.t list -> frame Streams.stream -> frame Streams.stream

let qprocs : (string * string, qproc) Hashtbl.t = Hashtbl.create 8
let put key1 key2 proc = Hashtbl.replace qprocs (key1, key2) proc
let get key1 key2 = Hashtbl.find_opt qprocs (key1, key2)

(** [value_list v] is the elements of a proper list value, or an error. *)
let value_list (v : Value.t) =
  let rec go acc v =
    match Value.view v with
    | Value.Nil -> Ok (List.rev acc)
    | Value.Pair (car, cdr) -> go (car :: acc) cdr
    | _ -> Error (Eval_error.Invalid_form ("not a list: " ^ Value.to_string v))
  in
  go [] v
;;

(** [type_of exp] is the dispatch tag: the car symbol of a tagged list. *)
let type_of exp =
  match Value.view exp with
  | Value.Pair (car, _) -> Value.to_string car
  | _ ->
    raise_error
      (Eval_error.Invalid_form ("unknown expression type: " ^ Value.to_string exp))
;;

(** The syntax of conjunctions, disjunctions, negations, calls, and rules:
    the book's small selectors, named for their object-language roles. *)
let first_operand items = List.hd items

(** [rule_conclusion rule] is the conclusion pattern of [(rule conclusion
    body?)]. *)
let rule_conclusion rule =
  match Value.view rule with
  | Value.Pair (_, cell) ->
    (match Value.view cell with
     | Value.Pair (conclusion, _) -> conclusion
     | _ -> raise_error (Eval_error.Invalid_form "a rule needs a conclusion"))
  | _ -> raise_error (Eval_error.Invalid_form "a rule needs a conclusion")
;;

(** [rule_body rule] is the body query, or the always-true placeholder for
    a rule without one. *)
let rule_body rule =
  match Value.view rule with
  | Value.Pair (_, cell) ->
    (match Value.view cell with
     | Value.Pair (_, tail) when Value.structural_equal tail Value.nil ->
       Value.pair (Value.symbol "always-true") Value.nil
     | Value.Pair (_, body_cell) ->
       (match Value.view body_cell with
        | Value.Pair (body, _) -> body
        | _ -> raise_error (Eval_error.Invalid_form "a rule needs a conclusion"))
     | _ -> raise_error (Eval_error.Invalid_form "a rule needs a conclusion"))
  | _ -> raise_error (Eval_error.Invalid_form "a rule needs a conclusion")
;;

(** [rename_variables_in rule] copies the rule with every variable renamed
    to [(? id name)] under one fresh application id, so two applications of
    one rule cannot confuse their variables. *)
let rename_variables_in rule =
  let id = new_rule_application_id () in
  let rec tree_walk exp =
    if is_var exp
    then make_new_variable exp id
    else (
      match Value.view exp with
      | Value.Pair (car, cdr) -> Value.pair (tree_walk car) (tree_walk cdr)
      | _ -> exp)
  in
  tree_walk rule
;;

(** {2 4.4.4.3: finding assertions by pattern matching} *)

(** [pattern_match pat dat frame] is the frame extended by the match of the
    datum against the pattern, consistent with the bindings already in the
    frame, or [None]. *)
let rec pattern_match pat dat frame =
  if Value.structural_equal pat dat
  then Some frame
  else if is_var pat
  then extend_if_consistent pat dat frame
  else (
    match Value.view pat, Value.view dat with
    | Value.Pair (pcar, pcdr), Value.Pair (dcar, dcdr) ->
      (match pattern_match pcar dcar frame with
       | None -> None
       | Some frame -> pattern_match pcdr dcdr frame)
    | _ -> None)

(** [extend_if_consistent var dat frame] binds [var] to the datum, or, when
    the frame already binds it, matches the datum against the stored value
    in the same frame -- never overwriting a stored binding. *)
and extend_if_consistent var dat frame =
  match binding_in_frame var frame with
  | Some (_, value) -> pattern_match value dat frame
  | None -> Some (extend var dat frame)
;;

(** {2 4.4.4.4: rules and unification} *)

(** [depends_on exp var frame] holds when the expression proposed as a
    value mentions the variable, directly or through the frame's bindings:
    binding it would ask for a fixed point the unifier cannot find. *)
let rec depends_on exp var frame =
  if is_var exp
  then
    if Value.structural_equal var exp
    then true
    else (
      match binding_in_frame exp frame with
      | Some (_, value) -> depends_on value var frame
      | None -> false)
  else (
    match Value.view exp with
    | Value.Pair (car, cdr) -> depends_on car var frame || depends_on cdr var frame
    | _ -> false)
;;

(** [unify_match p1 p2 frame] is the symmetric matcher: variables may occur
    on both sides. *)
let rec unify_match p1 p2 frame =
  if Value.structural_equal p1 p2
  then Some frame
  else if is_var p1
  then extend_if_possible p1 p2 frame
  else if is_var p2
  then extend_if_possible p2 p1 frame
  else (
    match Value.view p1, Value.view p2 with
    | Value.Pair (a1, d1), Value.Pair (a2, d2) ->
      (match unify_match a1 a2 frame with
       | None -> None
       | Some frame -> unify_match d1 d2 frame)
    | _ -> None)

(** [extend_if_possible var val frame] is [extend_if_consistent] with the
    unifier's two extra checks: an unbound variable matched against a
    variable follows the other side's binding, and a binding that would
    make a variable depend on itself is rejected. *)
and extend_if_possible var value frame =
  match binding_in_frame var frame with
  | Some (_, stored) -> unify_match stored value frame
  | None ->
    if is_var value
    then (
      match binding_in_frame value frame with
      | Some (_, stored) -> unify_match var stored frame
      | None -> Some (extend var value frame))
    else if depends_on value var frame
    then None
    else Some (extend var value frame)
;;

(** [check_an_assertion assertion query_pattern query_frame] is the
    singleton stream of the extended frame, or nothing. *)
let check_an_assertion assertion query_pattern query_frame =
  match pattern_match query_pattern assertion query_frame with
  | None -> Streams.the_empty_stream
  | Some frame -> singleton_stream frame
;;

(** [find_assertions pattern frame] is the stream of frames the data base's
    candidate assertions extend the frame by. *)
let find_assertions pattern frame =
  stream_flatmap
    (fun assertion -> check_an_assertion assertion pattern frame)
    (fetch_assertions pattern frame)
;;

(** [apply_a_rule rule query_pattern query_frame] renames the rule,
    unifies the conclusion with the pattern, and evaluates the body in the
    unified frame. The evaluator's clauses are one recursion group, as in
    the book: rules evaluate bodies, bodies are queries, and [qeval]
    dispatches back into the clauses. *)
let rec apply_a_rule env rule query_pattern query_frame =
  let clean_rule = rename_variables_in rule in
  match unify_match query_pattern (rule_conclusion clean_rule) query_frame with
  | None -> Streams.the_empty_stream
  | Some frame -> qeval env (rule_body clean_rule) (singleton_stream frame)

(** [apply_rules pattern frame] is the stream of frames the candidate rules
    extend the frame by. *)
and apply_rules env pattern frame =
  stream_flatmap
    (fun rule -> apply_a_rule env rule pattern frame)
    (fetch_rules pattern frame)

(** {2 4.4.4.2: the evaluator} *)

(** [simple_query env pattern frames] extends every frame by the matches of
    the pattern against assertions and, delayed, against rules (4.71). *)
and simple_query env query_pattern frames =
  stream_flatmap
    (fun frame ->
       stream_append_delayed (find_assertions query_pattern frame) (fun () ->
         apply_rules env query_pattern frame))
    frames

(** [conjoin env conjuncts frames] runs the conjuncts in series: each one
    filters and extends the frame stream the previous produced. *)
and conjoin env conjuncts frames =
  match conjuncts with
  | [] -> frames
  | first :: rest -> conjoin env rest (qeval env first frames)

(** [disjoin env disjuncts frames] merges the disjuncts' streams with
    interleaving, each delayed past the first (4.71, 4.72). *)
and disjoin env disjuncts frames =
  match disjuncts with
  | [] -> Streams.the_empty_stream
  | first :: rest ->
    interleave_delayed (qeval env first frames) (fun () -> disjoin env rest frames)

(** [execute env call] applies the host predicate of a [lisp-value] call to
    its (already evaluated) arguments: the predicate is looked up in the
    session environment, applied to the instantiated values, and judged by
    the object language's truth. *)
and execute env call =
  match Value.view call with
  | Value.Pair (predicate, args) ->
    let args = value_list args in
    (match Env.find_binding env (Value.to_string predicate), args with
     | Some procedure, Ok args ->
       (match Value.view procedure with
        | Value.Primitive_procedure _ ->
          let name = Value.to_string predicate in
          let f =
            snd (List.find (fun (n, _) -> String.equal n name) SE.primitive_table)
          in
          f args >>= fun v -> Ok (SE.true_ v)
        | _ ->
          Error
            (Eval_error.Type_error
               ("lisp-value: not a procedure: " ^ Value.to_string predicate)))
     | _, Error e -> Error e
     | None, _ -> Error (Eval_error.Unbound_variable (Value.to_string predicate)))
  | _ -> Error (Eval_error.Invalid_form "a lisp-value call needs a predicate")

(** [negate env operands frames] keeps only the frames the negated query
    cannot extend. *)
and negate env operands frames =
  stream_flatmap
    (fun frame ->
       if
         Streams.stream_null (qeval env (first_operand operands) (singleton_stream frame))
       then singleton_stream frame
       else Streams.the_empty_stream)
    frames

(** [lisp_value env call frames] keeps only the frames whose instantiation
    makes the host predicate true; an unbound pattern variable is an error.
    The whole call is instantiated, list arguments included. *)
and lisp_value env operands frames =
  let call = List.fold_right Value.pair operands Value.nil in
  stream_flatmap
    (fun frame ->
       let instantiated =
         instantiate call frame (fun v _ ->
           raise_error
             (Eval_error.User_error ("Unknown pat var LISP-VALUE: " ^ Value.to_string v)))
       in
       match execute env instantiated with
       | Ok true -> singleton_stream frame
       | Ok false -> Streams.the_empty_stream
       | Error e -> raise_error e)
    frames

(** [always_true env contents frames] passes the frames through: the body
    of a rule without one. *)
and always_true env contents frames =
  ignore env;
  ignore contents;
  frames

(** [qeval env query frames] is the query evaluator: it dispatches on the
    query's tag through the data-directed table and answers a simple query
    for an untagged pattern. *)
and qeval env query frames =
  let name = type_of query in
  match get name "qeval" with
  | Some qproc ->
    (match
       value_list
         (match Value.view query with
          | Value.Pair (_, cdr) -> cdr
          | _ -> Value.nil)
     with
     | Ok contents -> qproc env contents frames
     | Error e -> raise_error e)
  | None -> simple_query env query frames
;;

(* The book's dispatch registrations, the top-level [(put ... 'qeval ...)]
   forms of 4.4.4.2. *)
let () =
  put "and" "qeval" conjoin;
  put "or" "qeval" disjoin;
  put "not" "qeval" negate;
  put "lisp-value" "qeval" lisp_value;
  put "always-true" "qeval" always_true
;;

(** {2 4.4.4.1: the driver loop} *)

(** One driver input: an [assert!] adds to the data base; a query answers
    with the stream of its instantiations, printed one by one as they are
    forced -- the book's display-stream. *)
type outcome =
  | Asserted
  | Answers of Value.t Streams.stream

let instantiate_query query frame =
  instantiate query frame (fun v _ -> contract_question_mark v)
;;

(** [run env text] reads one input, processes its syntax, and answers: the
    assertion added, or the lazy stream of instantiated query patterns. A
    malformed query answers the typed error. *)
let run env text =
  try
    match read_query text with
    | Error message -> Error (Eval_error.Invalid_form message)
    | Ok raw ->
      let q = query_syntax_process raw in
      let is_assert =
        match Value.view q with
        | Value.Pair (car, _) -> Value.structural_equal car (Value.symbol "assert!")
        | _ -> false
      in
      if is_assert
      then (
        match Value.view q with
        | Value.Pair (_, body_cell) ->
          (match Value.view body_cell with
           | Value.Pair (body, _) ->
             let (_ : Value.t) = add_rule_or_assertion body in
             Ok Asserted
           | _ -> Error (Eval_error.Invalid_form "assert! needs a body"))
        | _ -> Error (Eval_error.Invalid_form "assert! needs a body"))
      else
        Ok
          (Answers
             (Streams.stream_map
                (instantiate_query q)
                (qeval env q (singleton_stream the_empty_frame))))
  with
  | Raised e -> Error e
  | Query_read_error message -> Error (Eval_error.Invalid_form message)
;;

(** [answers outcome] forces the whole answer stream; the book's
    display-stream, for finite queries. *)
let answers = function
  | Asserted -> []
  | Answers s -> stream_to_list s
;;

(** [answers_upto n outcome] forces at most [n] answers -- how a session
    observes a prefix of an unbounded answer stream. *)
let answers_upto n = function
  | Asserted -> []
  | Answers s -> Streams.stream_take n s
;;

(** [query env text] is [run] followed by [answers]: every answer of a
    finite query as a list. *)
let query env text = run env text >>= fun outcome -> Ok (answers outcome)

(** [query_upto n env text] is [run] followed by [answers_upto n]. *)
let query_upto n env text = run env text >>= fun outcome -> Ok (answers_upto n outcome)

(** {2 The session}

    [the_query_system ()] resets the data base and the renaming counter and
    answers a fresh global environment -- the host environment
    [lisp-value] evaluates its predicates in. The dispatch registrations
    are code, not data, and survive the reset. *)
let the_query_system () =
  the_assertions := [];
  the_rules := [];
  Hashtbl.reset the_index;
  rule_counter := 0;
  SE.the_global_environment ()
;;
