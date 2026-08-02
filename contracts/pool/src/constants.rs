//! Shared TTL constants for persistent and instance storage.
//!
//! These values are passed to `Storage::extend_ttl` (and the instance
//! equivalent) to keep entries alive across contract invocations.
//! `TTL_THRESHOLD` is the minimum number of ledgers an entry must have
//! remaining before it is extended, and `TTL_EXTEND_TO` is the number of
//! ledgers the entry is extended to.

/// Minimum remaining ledgers before a storage entry is eligible for TTL
/// extension.
///
/// Kept at 25% of `TTL_EXTEND_TO` so renewals happen well before expiry
/// instead of in the final ledgers of the lifetime (a low threshold risks
/// expiry during network congestion or when a contract sits idle).
pub const TTL_THRESHOLD: u32 = 500_000;

/// Number of ledgers a storage entry is extended to when refreshed.
pub const TTL_EXTEND_TO: u32 = 2_000_000;
