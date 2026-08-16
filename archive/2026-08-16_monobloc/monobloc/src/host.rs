use crate::Value;

/// # Abstract interface to the host
///
/// Monobloc is an embeddable programming language. Its implementation must be
/// embedded into a host, which uses the language implementation to drive the
/// execution of Monobloc code and moderates its interaction with the outside
/// world.
///
/// Monobloc code has no access to anything in the outside world, except through
/// the host. It runs in a sandbox, which it can only reach outside of by
/// calling host-provided function.
///
/// This trait is the abstract interface that hosts must implement. The language
/// implementation uses it to enable communication between Monobloc code and the
/// host.
pub trait Host {
    /// # Resolve a host function by name
    ///
    /// Return a [`HostFn`] instance that represents the host function with the
    /// provided name, or `None` if no such function exists within the context
    /// of this host.
    ///
    /// All functions in Monobloc currently live in a single, flat namespace.
    /// (The `namespace` field in [`HostFn`] enables a notion of namespaces that
    /// is an attribute of the implementation, but not observable from within
    /// the language.)
    fn resolve_fn(&self, name: &str) -> Option<HostFn>;

    /// # Return the attributes of the given host function
    fn fn_attrs(&self, host_fn: &HostFn) -> &HostFnAttrs;

    /// # Call the provided host function
    fn call_fn(&mut self, host_fn: &HostFn, host_call: &mut dyn HostCall);
}

/// # Represents a specific host function
///
/// Host functions are functions that the host defines. They can be called by
/// Monobloc code.
///
/// This struct serves to identify a specific host function in the interaction
/// between a host and the language implementation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HostFn {
    /// # A unique identifier for the namespace this function is part of
    ///
    /// The namespace defined here is purely an attribute of the language
    /// implementation. Within the language itself, there only exists a single,
    /// flat namespace that all functions are a part of.
    ///
    /// But grouping host functions by namespace eases the definition of
    /// reusable groups of functions, that a given host can then combine with
    /// its own, home-grown host functions, into the single, flat function
    /// namespace that Monobloc code can then access.
    ///
    /// To reduce the probability of interference between host functions by
    /// different authors, namespace IDs are split into three ranges:
    ///
    /// - IDs from `0` to `255` are reserved for built-in host function
    ///   namespaces that ship with the language implementation.
    /// - IDs from `256` to `511` are reserved for namespaces that are not
    ///   intended to be shared with other hosts. Any given host may use these
    ///   IDs freely.
    /// - IDs from `512` to `65535` are free to use for any host functions that
    ///   are intended to be shared between hosts. To minimize the chance of
    ///   collisions, you may choose an ID within that range at random before
    ///   publishing a new group of host functions.
    pub namespace: u16,

    /// # A unique identifier for the host function within its namespace
    pub function: u16,
}

/// # The attributes of a specific host function
///
/// This struct is returned by [`Host::fn_attrs`].
///
/// ## Design Note
///
/// In principle, this struct could be merged into [`HostFn`], which would then
/// be the single struct that both identifies a host function and provides
/// information on all of its attributes.
///
/// However, `HostFn` has to be stored by the language implementation as part of
/// any code representation. Adding more data to it would increase the code
/// size. Separating the concerns of identity and attributes into two structs
/// avoids this overhead.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HostFnAttrs {
    /// # The name that uniquely identifies the host function in code
    pub name: &'static str,

    /// # The number of parameters this function consumes
    pub num_parameters: u8,

    /// # Return continuation and number of its parameters
    ///
    /// Indicates whether the host function accepts a return continuation,
    /// meaning if once the host function is done, evaluation resumes after the
    /// call to the host function.
    ///
    /// A value of `Some` indicates that the host function returns. The `u8`
    /// value indicates the number of return parameters, which may be zero.
    ///
    /// A value of `None` indicates that the host function does not return.
    pub return_: Option<u8>,
}

/// # Abstract interface for handling a call to a host function
///
/// This trait is implemented by a part of the language implementation that is
/// performing a host call at runtime. It is made available to implementations
/// of [`Host`], via [`Host::call_fn`].
///
/// ## Design Note
///
/// The following (perhaps simpler) alternatives to making this a trait were
/// considered and rejected:
///
/// 1. Substituting this with a struct that provides a similar interface would
///    tie the [`Host`] trait to a specific implementation. With a trait, we can
///    have multiple implementations, for example specific ones for testing or
///    for user-specific needs.
/// 2. Storing the inputs and outputs in a struct directly would either require
///    a static allocation that is sized to the host function with the highest
///    number of parameters, or dynamic allocation. Either would represent an
///    undesirable overhead.
/// 3. Passing inputs as a parameter to [`Host::call_fn`] and returning outputs
///    from the method directly would impose similar allocation requirements as
///    alternative 2.
/// 4. Passing a struct with slices that reference inputs and outputs would
///    either limit the implementation in terms of how it can store values, or
///    require at least some kind of pre-allocated space in the language
///    implementation, as well as copying to and from that.
///
/// This trait provides full flexibility and doesn't impose any allocation
/// requirements, though at the cost of dynamic dispatch. The performance
/// implications of this wasn't measured against alternatives, so perhaps
/// something like alternative 4. may be more viable.
pub trait HostCall {
    /// # Access the i-th input to the host call
    ///
    /// `i` is a zero-based index of the parameter. This value must always be
    /// less than the number of parameters that the host function specifies.
    ///
    /// Hosts must uphold this precondition. Implementations may panic, if the
    /// condition has not been met or, for performance reasons, not check it at
    /// all, which could lead to undefined behavior in the calling Monobloc
    /// code.
    fn input(&mut self, i: u8) -> Value;

    /// # Access the i-th output of the host call
    ///
    /// `i` is a zero-based index of a parameter to the return continuation, if
    /// one exists for the host function. This value must always be less than
    /// the number of parameters to the return continuation.
    ///
    /// Hosts must uphold this precondition. Implementations may panic, if the
    /// condition has not been met or, for performance reasons, not check it at
    /// all, which could lead to undefined behavior in the calling Monobloc
    /// code.
    fn output(&mut self, i: u8, value: Value);
}
