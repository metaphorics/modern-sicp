(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Records, mutable fields, and refs: the named definitions behind the
    listings of section 0.6. *)

(** A two-dimensional point with immutable fields. *)
type point =
  { x : float
  ; y : float
  }

(** [origin] is [{x = 0.; y = 0.}]. *)
val origin : point

(** [right_three] is [origin] rebuilt with [x = 3.] through a functional
    update: [{x = 3.; y = 0.}]. The update copied; [origin] is unchanged. *)
val right_three : point

(** A one-field mutable record: the smallest stateful object. *)
type counter = { mutable count : int }

(** [fresh_counter ()] is a counter at zero. *)
val fresh_counter : unit -> counter

(** [bump counter] adds one to the counter's mutable field in place. *)
val bump : counter -> unit

(** [after_deposit ()] is [120]: a [ref] cell holding [100], assigned
    [!balance + 20] in place. *)
val after_deposit : unit -> int

(** [alias_cell ()] is [15]: writing through one name of a cell is visible
    through the other, which is what aliasing means. *)
val alias_cell : unit -> int

(** [copy_point ()] is [(1., 9.)]: a functional record update copies, so
    the original's [x] is untouched. *)
val copy_point : unit -> float * float

(** [cell_identity ()] is [(false, true)]: two [ref] cells built equal are
    distinct under [==], while an alias is physically the same cell. *)
val cell_identity : unit -> bool * bool
