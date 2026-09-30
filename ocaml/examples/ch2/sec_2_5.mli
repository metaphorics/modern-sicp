(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs of SICP section 2.5 *)

(** The named definitions behind the listings of section 2.5: a generic
    arithmetic system spanning ordinary numbers, rationals, and complex
    numbers (2.5.1); cross-type coercion (2.5.2); and a polynomial
    package built on that same generic system (2.5.3). *)

(** [contents] widens 2.4's two-case type to hold everything a
    coefficient, a numerator, or a term list needs: a plain number, a
    rational pair, a rectangular/polar pair, a boolean (the result of
    [=zero?]), or a polynomial whose own coefficients are [value]s --
    which lets a polynomial's coefficient be another polynomial, the
    representational fact 2.5.3's hierarchy discussion needs. *)
type value =
  | Num of float
  | Ratpair of int * int
  | Cpx of float * float
  | Bool of bool
  | Poly of poly
  | Tagged of tagged

and tagged =
  { tag : string
  ; contents : value
  }

and term =
  { order : int
  ; coeff : value
  }

and poly =
  { var : string
  ; term_list : term list
  }

val attach_tag : string -> value -> tagged
val type_tag : tagged -> string
val contents_of : tagged -> value

(** [put op type_tags proc] and [get op type_tags] back the
    operation-and-type table reused by every package below, the same
    table [Sec_2_4.Data_directed] uses. *)
val put : string -> string list -> (value list -> value) -> unit

val get : string -> string list -> (value list -> value) option

(** [apply_generic op args] looks [op] up under [args]'s type tags and
    applies the procedure found to the untagged contents, with no
    coercion fallback. Used only for the "rectangular"/"polar" dispatch
    inside the complex package, which never needs coercion between
    those two tags. *)
val apply_generic : string -> value list -> value

(** [apply_generic_coerce op args] is [apply_generic] extended with the
    coercion fallback of 2.5.2. Every top-level generic operation below
    -- [add], [sub], [mul], [div], [is_zero], and the polynomial
    package's coefficient arithmetic -- calls this one, not
    [apply_generic]. Scheme's [apply-generic] is one binding that 2.5.2
    redefines in place, so every earlier caller picks up the new
    behavior automatically; OCaml resolves a name where it is written,
    so this edition gives the coercion-aware version its own name
    instead of silently shadowing the first. *)
val apply_generic_coerce : string -> value list -> value

val put_coercion : string -> string -> (value -> value) -> unit
val get_coercion : string -> string -> (value -> value) option

(** [install_real_package ()] tags OCaml's own floats as
    ordinary numbers. Also installs [=zero?], which the book defers to
    Exercise 2.80 but 2.5.3's [adjoin_term] already needs to run. *)
val install_real_package : unit -> unit

(** [install_rational_package ()] reuses [Sicp_ch1.Sec_1_2.Gcd.gcd] to
    reduce every rational this package builds, exactly as 2.1.1's
    [make-rat] did. *)
val install_rational_package : unit -> unit

(** Ben's rectangular package and Alyssa's polar package, reusing the
    pure arithmetic of [Sec_2_4.Untagged.Rectangular] and
    [Sec_2_4.Untagged.Polar] -- the same procedures they wrote in
    2.4.1 -- but producing this section's own [Cpx] contents. *)
val install_rectangular_package : unit -> unit

val install_polar_package : unit -> unit

(** [real_part], [imag_part], [magnitude], and [angle] are 2.4.2's
    generic selectors, restated here because this section's [value]
    type differs from 2.4's. They dispatch only on "rectangular" and
    "polar", so [apply_generic] (no coercion) is enough. *)
val real_part : value -> float

val imag_part : value -> float
val magnitude : value -> float
val angle : value -> float

(** [install_complex_package ()] tags a rectangular-or-polar value
    "complex" and installs [add]/[sub]/[mul]/[div] under
    [\["complex"; "complex"\]], each built from [real_part] and
    friends on the once-untagged contents -- the "nested apply_generic"
    Exercise 2.77 asks about. It does not install [real_part] and
    friends under ["complex"] itself; that gap is Exercise 2.77's
    fix, not this package's job. *)
val install_complex_package : unit -> unit

(** [add_complex_to_real] and [install_cross_type_example ()] are
    2.5.2's "cumbersome" illustration: one explicit
    [\["complex"; "real"\]] procedure, shown once and not
    reused once coercion arrives. *)
val add_complex_to_real : value -> float -> value

val install_cross_type_example : unit -> unit

(** [real_to_complex] is the one coercion this section
    installs: an ordinary number viewed as a complex number with zero
    imaginary part. [install_coercions ()] puts it in the coercion
    table under [("real", "complex")]. *)
val real_to_complex : tagged -> value

val install_coercions : unit -> unit
val make_real : float -> value
val make_rational : int -> int -> value
val make_complex_from_real_imag : float -> float -> value
val make_complex_from_mag_ang : float -> float -> value

(** The four generic arithmetic operations, all routed through
    [apply_generic_coerce] so a [real] combined with a
    [complex] number coerces instead of failing. *)
val add : value -> value -> value

val sub : value -> value -> value
val mul : value -> value -> value
val div : value -> value -> value

(** [is_zero v] is the generic [=zero?] this section's polynomial
    package needs; only [install_real_package] installs it,
    so a rational or complex coefficient is Exercise 2.80's job. *)
val is_zero : value -> bool

(** Term lists, 2.5.3: an ordered list of nonzero terms, highest order
    first. [adjoin_term] drops a term whose coefficient [is_zero]. *)
val the_empty_termlist : term list

val is_empty_termlist : term list -> bool
val first_term : term list -> term
val rest_terms : term list -> term list
val make_term : int -> value -> term
val order : term -> int
val coeff : term -> value
val adjoin_term : term -> term list -> term list
val add_terms : term list -> term list -> term list
val mul_term_by_all_terms : term -> term list -> term list
val mul_terms : term list -> term list -> term list
val make_poly : string -> term list -> poly
val variable : poly -> string
val term_list : poly -> term list
val same_variable : string -> string -> bool
val add_poly : poly -> poly -> poly
val mul_poly : poly -> poly -> poly

(** [install_polynomial_package ()] tags a [poly] "polynomial" and
    installs [add]/[mul] under [\["polynomial"; "polynomial"\]], so
    [add]/[mul] above work on polynomials with no change to either
    function -- and, since a term's [coeff] is a [value], on
    polynomials whose coefficients are themselves polynomials. *)
val install_polynomial_package : unit -> unit

val make_polynomial : string -> term list -> value
