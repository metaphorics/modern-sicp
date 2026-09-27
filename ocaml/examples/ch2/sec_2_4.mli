(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs of SICP section 2.4 *)

(** Complex numbers with no representation tag yet, subsection 2.4.1:
    Ben's rectangular form and Alyssa's polar form, each unaware the
    other exists. *)
module Untagged : sig
  (** Ben's representation: (real part, imaginary part). *)
  module Rectangular : sig
    type t = float * float

    val real_part : t -> float
    val imag_part : t -> float
    val magnitude : t -> float
    val angle : t -> float
    val make_from_real_imag : float -> float -> t
    val make_from_mag_ang : float -> float -> t
  end

  (** Alyssa's representation: (magnitude, angle). *)
  module Polar : sig
    type t = float * float

    val real_part : t -> float
    val imag_part : t -> float
    val magnitude : t -> float
    val angle : t -> float
    val make_from_real_imag : float -> float -> t
    val make_from_mag_ang : float -> float -> t
  end

  (** The four selectors and two constructors a complex-number
      representation supplies, bundled as an explicit parameter.
      OCaml has no mutable global environment for [add_complex] to
      look [real_part] up in later, the way the book's discussion
      can just assume these procedures exist somewhere; this edition
      states the assumption as a record instead. *)
  type 'a complex_ops =
    { real_part : 'a -> float
    ; imag_part : 'a -> float
    ; magnitude : 'a -> float
    ; angle : 'a -> float
    ; make_from_real_imag : float -> float -> 'a
    ; make_from_mag_ang : float -> float -> 'a
    }

  val rectangular_ops : Rectangular.t complex_ops
  val polar_ops : Polar.t complex_ops

  (** [add_complex ops z1 z2] and its three companions are one
      implementation, applied below to both [rectangular_ops] and
      [polar_ops], of the book's claim that the same
      [add_complex]/[sub_complex]/[mul_complex]/[div_complex] work
      with either representation. *)
  val add_complex : 'a complex_ops -> 'a -> 'a -> 'a

  val sub_complex : 'a complex_ops -> 'a -> 'a -> 'a
  val mul_complex : 'a complex_ops -> 'a -> 'a -> 'a
  val div_complex : 'a complex_ops -> 'a -> 'a -> 'a
end

(** Tagged data, subsection 2.4.2. [value] is the uniform type this
    edition needs where the book's untyped pair could hold a number,
    a raw (real, imaginary) or (magnitude, angle) pair, or a fully
    tagged complex number -- exactly the range [contents] and 2.4.3's
    dispatch table both need to carry. *)
type value =
  | Num of float
  | Pair of float * float
  | Tagged of tagged

(** A value together with the representation tag that says how to
    read it. *)
and tagged =
  { tag : string
  ; contents : value
  }

(** [attach_tag], [type_tag], and [contents_of] cannot fail the way
    the book's [car]-based versions can: [tagged] is a record, not a
    pair that might hold anything, so a value without a tag has no
    way to reach these functions in the first place. *)
val attach_tag : string -> value -> tagged

val type_tag : tagged -> string
val contents_of : tagged -> value
val is_rectangular : tagged -> bool
val is_polar : tagged -> bool

(** Ben's revised rectangular representation, its procedures renamed
    with the [_rectangular] suffix so they cannot collide with
    Alyssa's; each delegates to [Untagged.Rectangular], the same
    procedures Ben wrote when he was working alone, unwrapping the
    [Pair] a tagged rectangular number carries. *)
val real_part_rectangular : value -> float

val imag_part_rectangular : value -> float
val magnitude_rectangular : value -> float
val angle_rectangular : value -> float
val make_from_real_imag_rectangular : float -> float -> tagged
val make_from_mag_ang_rectangular : float -> float -> tagged

(** Alyssa's revised polar representation, suffixed [_polar]. *)
val real_part_polar : value -> float

val imag_part_polar : value -> float
val magnitude_polar : value -> float
val angle_polar : value -> float
val make_from_real_imag_polar : float -> float -> tagged
val make_from_mag_ang_polar : float -> float -> tagged

(** The generic selectors: each checks [type_tag] and calls the
    matching suffixed procedure. [invalid_arg] on an unknown tag. *)
val real_part : tagged -> float

val imag_part : tagged -> float
val magnitude : tagged -> float
val angle : tagged -> float

(** The generic constructors: rectangular whenever real and imaginary
    parts are given, polar whenever magnitude and angle are given. *)
val make_from_real_imag : float -> float -> tagged

val make_from_mag_ang : float -> float -> tagged

(** [add_complex], [sub_complex], [mul_complex], and [div_complex]:
    textually the bodies from [Untagged], now meaningful without a
    [complex_ops] parameter because [real_part] and friends already
    work for either tag. *)
val add_complex : tagged -> tagged -> tagged

val sub_complex : tagged -> tagged -> tagged
val mul_complex : tagged -> tagged -> tagged
val div_complex : tagged -> tagged -> tagged

(** Data-directed programming, subsection 2.4.3: the same generic
    system, rebuilt so that adding a representation means installing
    one package rather than editing [real_part] and its three
    companions. *)
module Data_directed : sig
  (** [install_rectangular_package ()] and [install_polar_package ()]
      add one package's six operations to the shared table, exactly
      as [put] does in the book. Internally each package reuses
      [Untagged.Rectangular] or [Untagged.Polar], the same procedures
      Ben and Alyssa wrote when they worked in isolation. *)
  val install_rectangular_package : unit -> unit

  val install_polar_package : unit -> unit

  (** [apply_generic op args] looks up [op] under [args]'s type tags
      and applies the stored procedure to their untagged contents.
      [invalid_arg] when no package has installed [op] for those
      tags -- the book's [error "No method for these types"]. *)
  val apply_generic : string -> value list -> value

  val real_part : tagged -> float
  val imag_part : tagged -> float
  val magnitude : tagged -> float
  val angle : tagged -> float
  val make_from_real_imag : float -> float -> tagged
  val make_from_mag_ang : float -> float -> tagged
  val add_complex : tagged -> tagged -> tagged
  val sub_complex : tagged -> tagged -> tagged
  val mul_complex : tagged -> tagged -> tagged
  val div_complex : tagged -> tagged -> tagged
end

(** Message passing: a complex number that dispatches on the
    operation itself, rather than being dispatched on from outside. *)
module Message_passing : sig
  (** The operation such a complex number understands. *)
  type op =
    | Real_part
    | Imag_part
    | Magnitude
    | Angle

  (** A message-passing complex number: send it an [op], get back the
      [float] it names. *)
  type t = op -> float

  val make_from_real_imag : float -> float -> t
  val apply_generic : op -> t -> float
end
