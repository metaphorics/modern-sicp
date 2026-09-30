(* SPDX-License-Identifier: GPL-3.0-only *)

(** The reader of domain fixtures written as OCaml constructor literals
    (grammar section 10): query programs and register machines are
    ordinary typed constructor data, not a textual grammar of their own.

    A fixture is one OCaml expression built only from constructors,
    lists, tuples, records, and integer, float, and string literals.
    The pinned OCaml parser reads it; [read] then rejects every other
    form -- identifiers, applications, functions, [let], operators,
    labels, and attributes -- so the reader is not a general OCaml-source
    parser.  Each domain decodes the resulting tree into its own closed
    variant and rejects any constructor it does not declare. *)

(** One literal node. *)
type t =
  | Ctor of string * t list
  (** A constructor and its payload fields; a tuple payload is spread
      into the list, and [true] and [false] are the constructors
      ["true"] and ["false"]. *)
  | Int of int
  | Float of float
  | String of string
  | List of t list
  | Tuple of t list
  | Record of (string * t) list

(** [read ~filename text] is the literal of [text], or a one-line reason
    with its position. *)
val read : filename:string -> string -> (t, string) result

(** [describe d] names the shape of [d] for decoding errors. *)
val describe : t -> string
