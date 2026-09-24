(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.5 exercise 2.77 *)

(** A complex number tagged twice: "complex" outermost, then
    "rectangular" -- Figure 2.24's own two-level structure. *)
type value =
  | Num of float
  | Cpx of float * float
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

val attach_tag : string -> value -> tagged
val type_tag : tagged -> string
val contents_of : tagged -> value

(** [apply_generic op args] counts every call it makes in the shared
    counter [call_count] reads. *)
val apply_generic : string -> value list -> value

val call_count : unit -> int
val reset_call_count : unit -> unit

(** Installs "real_part"/"imag_part"/"magnitude"/"angle" under
    ["rectangular"], exactly as 2.4.2 did. *)
val install_rectangular_package : unit -> unit

val real_part : value -> float
val imag_part : value -> float
val magnitude : value -> float
val angle : value -> float

(** Alyssa's fix: install the generic selectors themselves under
    ["complex"], so a "complex"-tagged value's contents (still tagged
    "rectangular") reaches [install_rectangular_package]'s entries
    through one more [apply_generic] call. *)
val install_alyssa_complex_fix : unit -> unit

(** [ex_2_77 ()] is [(raised_before_fix, magnitude_after_fix,
    calls_after_fix)] for [z] = the rectangular (3, 4) tagged
    "complex". Before the fix, [magnitude z] has no ["complex"] entry
    to dispatch to and raises; after, it answers 5.0 through exactly
    two [apply_generic] calls -- one that strips "complex" and lands
    back in [apply_generic] via the installed [magnitude], one that
    strips "rectangular" and computes the answer. *)
val ex_2_77 : unit -> bool * float * int
