(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.55 and the query kit the section 4.4 exercises share.

    The kit builds terms of the query engine from short constructors,
    holds the Microshaft personnel data base of 4.4.1, and renders query
    answers the way the driver prints them. *)

module Kit : sig
  module Q = Sicp_ch4.Sec_4_4
  module Streams = Q.Streams

  (** [at name] is the atom [name]. *)
  val at : string -> Q.term

  (** [v name] is the query variable [?name]. *)
  val v : string -> Q.term

  (** [n k] is the number [k]. *)
  val n : int -> Q.term

  (** [l items] is the proper list of [items]. *)
  val l : Q.term list -> Q.term

  (** [atoms names] is the proper list of the atoms [names]. *)
  val atoms : string list -> Q.term

  (** [person name] is the list of the space-separated words of [name],
      the book's [(Bitdiddle Ben)] shape. *)
  val person : string -> Q.term

  (** [p items] is the simple query whose pattern is the list [items]. *)
  val p : Q.term list -> Q.query

  (** [microshaft] is the personnel data base of 4.4.1, in the book's
      order: addresses, jobs, salaries, supervisors, and the
      [can-do-job] facts. *)
  val microshaft : Q.term list

  (** [session ~rules assertions] is a fresh data base holding
      [assertions] and then [rules], each in order. *)
  val session : ?rules:(Q.term * Q.query) list -> Q.term list -> Q.session

  (** [find_assertions session pattern frame] is the stream of
      extensions of [frame] under which [pattern] matches an assertion,
      in assertion order: the book's [find-assertions], for the variant
      evaluators the exercises assemble. *)
  val find_assertions : Q.session -> Q.term -> Q.frame -> Q.frame Streams.stream

  (** [canonical_query q] numbers every renamed rule variable of [q]
      by first occurrence, the engine's display discipline, so a
      printed answer does not depend on how many rule applications the
      search made before it. *)
  val canonical_query : Q.query -> Q.query

  (** [transcript session queries] runs [queries] through the driver.
      Each query contributes its [? query] line and then one line per
      answer; a failure ends the transcript with an [error: ...] line. *)
  val transcript : Q.session -> Q.query list -> string list

  (** [take k s] is the first [k] elements of [s].  It forces exactly
      those elements and never the tail behind the last one, so a
      stream that diverges after its [k]th element still answers. *)
  val take : int -> 'a Streams.stream -> 'a list

  (** [answers_upto k session q] is the first [k] answers of [q],
      each through [canonical_query]; a failure is one [error: ...]
      line after the answers produced before it. *)
  val answers_upto : int -> Q.session -> Q.query -> string list

  (** [answers_all session q] is every answer of the finite query [q],
      rendered as [answers_upto] renders them. *)
  val answers_all : Q.session -> Q.query -> string list
end

(** [ex_4_55 ()] is the transcript of the exercise's three simple
    queries, the dotted variants they need, and the [same]-person
    observation. *)
val ex_4_55 : unit -> string list
