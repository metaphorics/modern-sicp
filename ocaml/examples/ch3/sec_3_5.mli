(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The named definitions behind the listings of section 3.5, grouped by
    subsection. Streams here keep the book's memoized [delay] semantics:
    [cons_stream] takes the tail as a thunk and stores it behind exactly
    one [Lazy.t], so a tail is computed at most once no matter how often
    it is forced. Every deterministic value a listing displays is
    asserted by [run_sec_3_5] through [Replay]; the book's result
    comments are true by construction. *)

(** 3.5.1: the stream type and the operations the section builds on it.
    A stream is one evaluated element paired with a memoized promise for
    the rest, or the empty stream. *)
module Streams : sig
  type 'a stream =
    | Cons of 'a * 'a stream Lazy.t
    | Empty

  val the_empty_stream : 'a stream

  (** [stream_null s] holds exactly when [s] is [the_empty_stream]. *)
  val stream_null : 'a stream -> bool

  (** [cons_stream head tail] pairs [head] with one memoized delay for
      [tail ()]; the tail thunk runs at most once, whatever the number
      of later accesses. *)
  val cons_stream : 'a -> (unit -> 'a stream) -> 'a stream

  val stream_car : 'a stream -> 'a
  val stream_cdr : 'a stream -> 'a stream
  val stream_ref : 'a stream -> int -> 'a
  val stream_map : ('a -> 'b) -> 'a stream -> 'b stream
  val stream_for_each : ('a -> unit) -> 'a stream -> unit

  (** [stream_enumerate_interval low high] enumerates only as far as it
      is asked to. *)
  val stream_enumerate_interval : int -> int -> int stream

  val stream_filter : ('a -> bool) -> 'a stream -> 'a stream

  (** [stream_take n s] is the list of the first [n] elements, or all
      of them when the stream runs out first. *)
  val stream_take : int -> 'a stream -> 'a list

  (** [display_line] writes a newline and then the shown element, and
      [display_stream show out s] walks the whole stream that way. *)
  val display_line : Buffer.t -> string -> unit

  val display_stream : ('a -> string) -> Buffer.t -> 'a stream -> unit
end

(** 3.5.1: implementing [delay] and [force]; the memoized wrapper whose
    standard-library form is [Lazy.t]. *)
module Memo : sig
  (** [memo_proc f] answers a thunk that computes [f ()] on its first
      call and returns the remembered value on every later one. *)
  val memo_proc : (unit -> 'a) -> unit -> 'a
end

(** 3.5.2: infinite streams, by generating procedure and by implicit
    self-reference. *)
module Infinite : sig
  open Streams

  val integers_starting_from : int -> int stream
  val integers : int stream

  (** [divisible x y] holds when [y] divides [x] exactly. *)
  val divisible : int -> int -> bool

  val no_sevens : int stream
  val fibgen : int -> int -> int stream
  val fibs : int stream
  val sieve : int stream -> int stream
  val primes : int stream
  val ones : int stream

  (** The two-stream elementwise map behind [add_streams]; the n-ary
      generalization is exercise 3.50. *)
  val stream_map2 : ('a -> 'b -> 'c) -> 'a stream -> 'b stream -> 'c stream

  val add_streams : int stream -> int stream -> int stream

  (** The elementwise sum for floating-point streams. *)
  val add_streams_float : float stream -> float stream -> float stream

  (** The implicit definitions: 1 followed by [ones] plus the stream
      itself, and the two shifted Fibonacci streams added. Both agree
      element for element with [integers] and [fibs]. *)
  val integers_implicit : int stream

  val fibs_implicit : int stream
  val scale_stream : int stream -> int -> int stream
  val double : int stream

  (** The alternate primes: [primes_alt] filters the integers through
      [is_prime], which consults [primes_alt] itself. *)
  val primes_alt : int stream

  val is_prime : int -> bool
end

(** 3.5.3: power series as streams of coefficients. *)
module Series : sig
  open Streams

  val integrate_series : float stream -> float stream
  val exp_series : float stream
end

(** 3.5.3: formulating iterations as stream processes, and sequence
    acceleration. *)
module Convergence : sig
  open Streams

  val average : float -> float -> float
  val sqrt_improve : float -> float -> float

  (** [sqrt_stream x] holds the infinitely many Newton guesses for the
      square root of [x], starting at 1.0. *)
  val sqrt_stream : float -> float stream

  val pi_summands : int -> float stream
  val partial_sums : float stream -> float stream
  val pi_stream : float stream
  val euler_transform : float stream -> float stream
  val make_tableau : ('a stream -> 'a stream) -> 'a stream -> 'a stream stream
  val accelerated_sequence : ('a stream -> 'a stream) -> 'a stream -> 'a stream
end

(** 3.5.3: infinite streams of pairs. *)
module Pairs : sig
  open Streams

  val stream_append : 'a stream -> 'a stream -> 'a stream
  val interleave : 'a stream -> 'a stream -> 'a stream

  (** [pairs s t] enumerates the on-or-above-diagonal half of the
      infinite array of pairs, interleaving the first row with the
      recursively defined remainder. *)
  val pairs : 'a stream -> 'b stream -> ('a * 'b) stream
end

(** 3.5.3: streams as signals. *)
module Signals : sig
  open Streams

  (** [integral integrand initial_value dt] is the running sum
      C + x_1 dt + x_2 dt + ... behind the text's feedback loop. *)
  val integral : float stream -> float -> float -> float stream
end

(** 3.5.4: the integrator whose input is itself a promise. *)
module Solve : sig
  open Streams

  val integral_delayed : float stream Lazy.t -> float -> float -> float stream

  (** [solve f y0 dt] generates the solution stream of dy/dt = f(y). *)
  val solve : (float -> float) -> float -> float -> float stream
end

(** 3.5.5: random streams and the Monte Carlo estimate of [pi], with no
    assignment anywhere. *)
module Random_streams : sig
  open Streams

  val random_init : Int64.t

  (** [rand_update x] is the seeded generator of 3.1.2 made stateless:
      the state is a value, not a hidden [ref]. *)
  val rand_update : Int64.t -> Int64.t

  val random_numbers : Int64.t stream
  val map_successive_pairs : ('a -> 'a -> 'b) -> 'a stream -> 'b stream
  val cesaro_stream : bool stream

  (** [monte_carlo experiment_stream passed failed] is the stream of
      running success ratios of the experiment. *)
  val monte_carlo : bool stream -> int -> int -> float stream

  val pi_stream : float stream

  (** [stream_withdraw balance amounts] is the stream of balances a
      withdrawal processor would report; a mathematical function with
      the observable behavior of 3.1.3's object. *)
  val stream_withdraw : float -> float stream -> float stream
end
