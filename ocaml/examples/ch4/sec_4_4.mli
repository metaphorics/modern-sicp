(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 4.4 *)

(** The query system of section 4.4 over closed host constructors
    (grammar sections 10 and 13, row 4.4).

    Terms, queries, rules, and frames are ordinary OCaml variants and
    lists; there is no query text.  A frame is an immutable association
    of variables to terms; unification includes the occurs check; answer
    streams are the chapter 3 memoized streams, combined with the book's
    delayed append and fair interleaving.

    Answer order: a simple query yields its assertion matches (in
    assertion order) before its rule-derived answers (rule order,
    interleaved across rules); [And] feeds each conjunct the frames of
    the previous ones; [Or] interleaves its disjuncts; [Not] and
    [Holds] filter frames.  Special forms beyond these are dispatched by
    name through a handler table, as the book's [put] does. *)

module Eval_error = Sicp_common.Eval_error
module Streams = Sicp_ch3.Sec_3_5.Streams

(** A pattern variable: its name and the rule application that renamed
    it ([0] for a variable of the query itself). *)
type variable =
  { name : string
  ; id : int
  }

(** A query term.  Lists are [Pair]/[Nil] structure, so a rule may
    match a list's head and its tail separately. *)
type term =
  | Atom of string
  | Num of int
  | Str of string
  | Var of variable
  | Nil
  | Pair of term * term

(** A query. *)
type query =
  | Pattern of term
  | And of query list
  | Or of query list
  | Not of query
  | Holds of string * term list
  (** The book's [lisp-value]: a host predicate ([<], [>], [<=], [>=],
      [=], [<>] on numbers and strings) applied to instantiated terms. *)
  | Always_true
  | Form of string * query list
  (** A special form dispatched through the handler table. *)

(** One driver input. *)
type command =
  | Assert of term
  | Rule of term * query
  | Query of query

(** A frame: bindings, newest first. *)
type frame = (variable * term) list

(** [list items] is the proper list term of [items]; [dotted items tail]
    ends it in [tail]. *)
val list : term list -> term

val dotted : term list -> term -> term

(** [var name] is the query variable [?name]. *)
val var : string -> term

(** [render_term t] is [t] in list notation: [[a, ?x, [b]]], a dotted
    tail as [[?u | ?v]], renamed variables as [?name.id]. *)
val render_term : term -> string

(** [render_query q] renders [q]: patterns as terms, the combinators as
    [and(...)], [or(...)], [not(...)], [holds(p, ...)], [always-true],
    and special forms by name. *)
val render_query : query -> string

(** {1 Frames, matching, and unification} *)

val binding_in_frame : variable -> frame -> term option
val extend : variable -> term -> frame -> frame

(** [depends_on t v frame] holds when [t] mentions [v], directly or
    through [frame]: the occurs check. *)
val depends_on : term -> variable -> frame -> bool

(** [pattern_match pattern datum frame] extends [frame] so that
    [pattern] matches the variable-free [datum], or is [None]. *)
val pattern_match : term -> term -> frame -> frame option

(** [unify_match a b frame] extends [frame] so that [a] and [b] unify,
    or is [None]. *)
val unify_match : term -> term -> frame -> frame option

(** [instantiate t frame unbound] replaces every bound variable of [t]
    by its (instantiated) value and hands each unbound one to
    [unbound]. *)
val instantiate : term -> frame -> (variable -> term) -> term

(** [rename_variables_in (conclusion, body) id] renames every variable of
    the rule to application [id]. *)
val rename_variables_in : term * query -> int -> term * query

(** {1 The streams of 4.4.4.6} *)

val stream_append_delayed
  :  'a Streams.stream
  -> (unit -> 'a Streams.stream)
  -> 'a Streams.stream

val interleave_delayed
  :  'a Streams.stream
  -> (unit -> 'a Streams.stream)
  -> 'a Streams.stream

val stream_flatmap : ('a -> 'b Streams.stream) -> 'a Streams.stream -> 'b Streams.stream
val singleton_stream : 'a -> 'a Streams.stream

(** {1 The data base and the evaluator} *)

(** One query session: its assertions, rules, rule-application counter,
    and special-form handlers. *)
type session

(** A special-form handler: the session, the form's operand queries,
    and the input frames, answering the output frames. *)
type handler = session -> query list -> frame Streams.stream -> frame Streams.stream

(** [new_session ()] is an empty data base with no special forms. *)
val new_session : unit -> session

(** [put session name handler] installs the special form [name]. *)
val put : session -> string -> handler -> unit

(** [add_assertion session t] files an assertion after the earlier ones. *)
val add_assertion : session -> term -> unit

(** [add_rule session conclusion body] files a rule after the earlier ones. *)
val add_rule : session -> term -> query -> unit

(** [fetch_assertions session pattern] is every assertion in order. *)
val fetch_assertions : session -> term -> term Streams.stream

(** [fetch_rules session pattern] is every rule in order. *)
val fetch_rules : session -> term -> (term * query) Streams.stream

(** [qeval session q frames] is the stream of frames extending [frames]
    that satisfy [q].  A [Holds] with an unbound variable, an unknown
    predicate, or an unknown special form raises [Query_error]. *)
val qeval : session -> query -> frame Streams.stream -> frame Streams.stream

(** A failure inside lazy answer production. *)
exception Query_error of Eval_error.t

(** [answers session q] is every instantiation of [q]'s own term
    structure by its answer frames, unbound variables left as they are. *)
val answers : session -> query -> (query Streams.stream, Eval_error.t) result

(** [instantiate_query q frame] is [q] with every variable bound in
    [frame] replaced. *)
val instantiate_query : query -> frame -> query

(** [run ~emit session commands] runs a driver session: assertions and
    rules are filed, and each query writes [? query] and then one line
    per answer. *)
val run : emit:(string -> unit) -> session -> command list -> (unit, Eval_error.t) result

(** [read_fixture ~filename text] decodes a constructor-literal query
    program: a list of [Assert term], [Rule (term, query)], and [Query
    query], with terms built from [Atom], [Num], [Str], [Var], [List],
    and [Dotted (items, tail)]. *)
val read_fixture : filename:string -> string -> (command list, string) result
