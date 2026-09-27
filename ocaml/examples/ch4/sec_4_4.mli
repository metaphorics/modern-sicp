(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.4 *)

(** The query system of section 4.4 on the chapter's substrate: assertions,
    rules, patterns, and frames are typed [Value] pairs and symbols; the one
    lazy-stream choice is the chapter 3 memoized stream, re-exported as
    [Streams]; the four layers keep the book's shape -- the driver loop
    ([run]), the evaluator ([qeval] and its data-directed dispatch), the
    matcher and unifier ([pattern_match], [unify_match]), and the frame and
    data-base machinery.

    A frame is an immutable association list of bindings; it has no marked
    inhabitant (an absent variable is absence, not a value), the matcher's
    failure is [None], and frame variables are the internal [(? name)] /
    [(? id name)] list values, so a renamed rule variable never collides
    with an explicitly written one. The data base is chronological -- the
    insertion order the book's pinned transcripts list -- behind the book's
    stream interface, indexed by the leading symbol. The driver's query
    reader is the data-language scanner: [()], dotted-tail patterns, and
    tokens such as [9am] are ordinary data. *)

(** The chapter 3 memoized stream: one evaluated element paired with a
    [Lazy.t] tail. Every [delay]/[force] of the book's listings is
    [cons_stream]'s thunk and the tail's force. *)
module Streams = Sicp_ch3.Sec_3_5.Streams

(** One frame: bindings of internal variables to values, newest first,
    built only through [the_empty_frame] and [extend]. *)
type frame

(** The frame assigning no variables, the driver's initial frame. *)
val the_empty_frame : frame

(** [extend variable value frame] is the frame with one more binding in
    front. *)
val extend : Sicp_common.Value.t -> Sicp_common.Value.t -> frame -> frame

(** [binding_in_frame variable frame] is the stored [(variable, value)]
    pair, or [None]. *)
val binding_in_frame
  :  Sicp_common.Value.t
  -> frame
  -> (Sicp_common.Value.t * Sicp_common.Value.t) option

(** [frame_bindings frame] is the bindings as an association list, newest
    first -- the frame's whole content, for history checks and merging. *)
val frame_bindings : frame -> (Sicp_common.Value.t * Sicp_common.Value.t) list

(** {2 Query syntax} *)

(** [is_var exp] holds for the internal pattern variables: list values
    whose car is the symbol [?]. *)
val is_var : Sicp_common.Value.t -> bool

(** [read_query text] reads the one datum of [text] as a value, ignoring
    everything after it, or the scanner's message. *)
val read_query : string -> (Sicp_common.Value.t, string) result

(** [query_syntax_process exp] expands every [?x] symbol to the internal
    [(? x)], the book's driver-loop preprocessing. *)
val query_syntax_process : Sicp_common.Value.t -> Sicp_common.Value.t

(** [contract_question_mark variable] is the external spelling of an
    internal variable: [(? x)] reads back as [?x], [(? 7 x)] as [?x-7]. *)
val contract_question_mark : Sicp_common.Value.t -> Sicp_common.Value.t

(** [make_new_variable variable id] is the renamed variable [(? id name)]
    of one rule application. *)
val make_new_variable : Sicp_common.Value.t -> int -> Sicp_common.Value.t

(** [rename_variables_in rule] copies the rule with every variable renamed
    under one fresh application id. *)
val rename_variables_in : Sicp_common.Value.t -> Sicp_common.Value.t

(** [is_rule statement] holds for [(rule ...)] forms. *)
val is_rule : Sicp_common.Value.t -> bool

(** [rule_conclusion rule] is the conclusion pattern. *)
val rule_conclusion : Sicp_common.Value.t -> Sicp_common.Value.t

(** [rule_body rule] is the body query, or the always-true placeholder for
    a rule without one. *)
val rule_body : Sicp_common.Value.t -> Sicp_common.Value.t

(** [value_list v] is the elements of a proper list value, or the typed
    error. *)
val value_list
  :  Sicp_common.Value.t
  -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

(** [first_operand items] is the first element of a contents list. *)
val first_operand : Sicp_common.Value.t list -> Sicp_common.Value.t

(** {2 The matcher and the unifier} *)

(** [pattern_match pattern datum frame] is the frame extended by the match
    of the datum against the pattern, consistent with the bindings already
    in the frame, or [None] when the match fails. *)
val pattern_match : Sicp_common.Value.t -> Sicp_common.Value.t -> frame -> frame option

(** [unify_match p1 p2 frame] is the symmetric matcher: variables may occur
    on both sides, a binding that would make a variable depend on itself is
    rejected, and the result is the extended frame or [None]. *)
val unify_match : Sicp_common.Value.t -> Sicp_common.Value.t -> frame -> frame option

(** [depends_on exp variable frame] holds when the proposed value mentions
    the variable, directly or through the frame's bindings. *)
val depends_on : Sicp_common.Value.t -> Sicp_common.Value.t -> frame -> bool

(** [instantiate exp frame unbound_var_handler] copies [exp] replacing
    every variable by its frame value, itself instantiated, handing an
    unbound variable to the handler. *)
val instantiate
  :  Sicp_common.Value.t
  -> frame
  -> (Sicp_common.Value.t -> frame -> Sicp_common.Value.t)
  -> Sicp_common.Value.t

(** {2 The stream machinery of 4.4.4.6} *)

(** [singleton_stream x] is the one-element stream. *)
val singleton_stream : 'a -> 'a Streams.stream

(** [stream_append_delayed s1 delayed_s2] appends, forcing the second
    stream only when the first runs out -- the explicit delay that
    postpones looping (4.71). *)
val stream_append_delayed
  :  'a Streams.stream
  -> (unit -> 'a Streams.stream)
  -> 'a Streams.stream

(** [interleave_delayed s1 delayed_s2] alternates the two streams, the
    second forced only when first needed (4.72). *)
val interleave_delayed
  :  'a Streams.stream
  -> (unit -> 'a Streams.stream)
  -> 'a Streams.stream

(** [flatten_stream stream] interleaves a stream of streams; its explicit
    delay is what exercise 4.73 debates. *)
val flatten_stream : 'a Streams.stream Streams.stream -> 'a Streams.stream

(** [stream_flatmap proc s] maps [proc] over [s] and interleaves the
    resulting streams -- the book's combination everywhere in the
    evaluator. *)
val stream_flatmap : ('a -> 'b Streams.stream) -> 'a Streams.stream -> 'b Streams.stream

(** {2 The data base} *)

(** [fetch_assertions pattern frame] is the candidate assertions as a
    stream: the leading-symbol index when the pattern has one, all
    assertions otherwise, in insertion order. *)
val fetch_assertions : Sicp_common.Value.t -> frame -> Sicp_common.Value.t Streams.stream

(** [fetch_rules pattern frame] is the candidate rules: those indexed under
    the pattern's key and under [?], or all rules. *)
val fetch_rules : Sicp_common.Value.t -> frame -> Sicp_common.Value.t Streams.stream

(** [add_rule_or_assertion statement] files one [assert!] body into the
    data base and its index, appending in insertion order. *)
val add_rule_or_assertion : Sicp_common.Value.t -> Sicp_common.Value.t

(** {2 The evaluator} *)

(** One dispatch handler: the session environment (filters evaluate host
    predicates in it), the contents list of the tagged query, and the frame
    stream; it answers the extended frame stream. *)
type qproc =
  Sicp_common.Value.env
  -> Sicp_common.Value.t list
  -> frame Streams.stream
  -> frame Streams.stream

(** [put key1 key2 proc] registers a handler, the book's [(put key 'qeval
    proc)]; exercise 4.75 registers [unique] through it. *)
val put : string -> string -> qproc -> unit

(** [get key1 key2] is the registered handler, or [None]. *)
val get : string -> string -> qproc option

(** [qeval env query frames] is the query evaluator: dispatch on the query's
    tag, a simple query for an untagged pattern. *)
val qeval
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t
  -> frame Streams.stream
  -> frame Streams.stream

(** [simple_query env pattern frames] extends every frame by the matches of
    the pattern against assertions and, delayed, against rules. *)
val simple_query
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t
  -> frame Streams.stream
  -> frame Streams.stream

(** [find_assertions pattern frame] is the stream of frames the candidate
    assertions extend the frame by. *)
val find_assertions : Sicp_common.Value.t -> frame -> frame Streams.stream

(** [apply_rules env pattern frame] is the stream of frames the candidate
    rules extend the frame by. *)
val apply_rules
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t
  -> frame
  -> frame Streams.stream

(** [apply_a_rule env rule pattern frame] renames the rule, unifies the
    conclusion with the pattern in the frame, and evaluates the body in the
    unified frame. *)
val apply_a_rule
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t
  -> Sicp_common.Value.t
  -> frame
  -> frame Streams.stream

(** [conjoin env conjuncts frames] runs the conjuncts in series. *)
val conjoin
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t list
  -> frame Streams.stream
  -> frame Streams.stream

(** [disjoin env disjuncts frames] merges the disjuncts' streams with
    interleaving. *)
val disjoin
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t list
  -> frame Streams.stream
  -> frame Streams.stream

(** [negate env [query] frames] keeps only the frames the query cannot
    extend; the [not] filter. *)
val negate
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t list
  -> frame Streams.stream
  -> frame Streams.stream

(** [lisp_value env call frames] keeps only the frames whose instantiation
    makes the host predicate true; an unbound pattern variable is an
    error. *)
val lisp_value
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t list
  -> frame Streams.stream
  -> frame Streams.stream

(** [execute env call] applies the host predicate named by [call] to its
    already-evaluated argument values and judges the result by the object
    language's truth. *)
val execute
  :  Sicp_common.Value.env
  -> Sicp_common.Value.t
  -> (bool, Sicp_common.Eval_error.t) result

(** {2 The driver} *)

(** One driver input: an [assert!] added to the data base, or a query's
    lazy stream of instantiated query patterns -- printed one by one as
    they are forced, the book's display-stream. *)
type outcome =
  | Asserted
  | Answers of Sicp_common.Value.t Streams.stream

(** [run env text] reads one input, processes its syntax, and answers. A
    malformed query or a failing filter answers the typed error. *)
val run : Sicp_common.Value.env -> string -> (outcome, Sicp_common.Eval_error.t) result

(** [answers outcome] forces the whole answer stream; the book's
    display-stream, for finite queries. An [Asserted] outcome has no
    answers. *)
val answers : outcome -> Sicp_common.Value.t list

(** [answers_upto n outcome] forces at most [n] answers, how a session
    observes a prefix of an unbounded answer stream. *)
val answers_upto : int -> outcome -> Sicp_common.Value.t list

(** [query env text] is [run] followed by [answers]: every answer of a
    finite query as a list. *)
val query
  :  Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

(** [query_upto n env text] is [run] followed by [answers_upto n]. *)
val query_upto
  :  int
  -> Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

(** [the_query_system ()] resets the data base and the renaming counter and
    answers a fresh global environment, the host environment [lisp-value]
    evaluates its predicates in. The dispatch registrations are code and
    survive the reset. *)
val the_query_system : unit -> Sicp_common.Value.env
