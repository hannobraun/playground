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

    /// # Indicate whether the provided function returns
    fn fn_returns(&self, host_fn: &HostFn) -> bool;
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
