(* SPDX-License-Identifier: GPL-3.0-only *)

(** The fixed standard-library surface of grammar sections 3, 6, and 7:
    the eight unqualified prelude values and the direct-call members of
    [Array], [List], and [Hashtbl].  [initial_env] binds every admitted
    name as a runtime primitive; nothing else is reachable from guest
    source.

    The printing primitives write through the run's [emit] sink, so the
    driver keeps guest stdout separate from its own diagnostics.  The
    numeric primitives are the pinned Stdlib functions themselves, so
    engine output and native output share one rounding and formatting
    behavior. *)

(** [initial_env ~emit ()] is the guest global environment: the eight
    unqualified prelude values, [Array.make], [Array.get], [Array.set],
    [Array.length], the [List] members, and the [Hashtbl] members.  Each
    printing primitive writes its bytes through [emit]. *)
val initial_env : emit:(string -> unit) -> unit -> Value.env

(** [unqualified] is the unqualified prelude of grammar section 7. *)
val unqualified : string list

(** [qualified] is the qualified direct-call surface of grammar
    sections 3 and 6. *)
val qualified : string list
