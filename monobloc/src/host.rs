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
    fn call_fn(&mut self, host_fn: &HostFn);
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

    /// # Indicates whether the host function returns
    pub returns: bool,
}
