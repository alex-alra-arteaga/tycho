// Copyright (c) 2026 Everlong Labs Limited

//! Everlong FLAMM (Base 8453, c104 @ `80abd43`): a native Tycho integration of the pool's swap
//! venue and lever-up venue, quoted to the wei from tracked state without a VM.
//!
//! Every module is a port of the Go simulator of the same pool, each function tied to its
//! Solidity line. Every refusal is the revert the chain would raise ([`FlammError`]), and every
//! arithmetic step keeps Solidity's operation order and rounding.
//!
//! | module | contract |
//! |---|---|
//! | [`almcurve`] | `AlmCurve.sol`, the normalized reservation curve the swap hook trades on |
//! | [`fee`] | `EverlongStrategy.sol`, the fill-fee law |
//! | [`hook`] | `EverlongHook.sol`, the swap hook's lazy book rescale, fill and commit |
//! | [`context`] | `IFLAMMHooks.sol` / `IFLAMMLeverage.sol`, the frames core hands its hooks |
//! | [`error`] | one variant per deployed custom error, with the selector table |
//! | [`math`] | `Math.mulDiv` / `ceilDiv` / `sqrt`, `Mul512` and checked `int256` arithmetic |

pub mod almcurve;
pub mod context;
pub mod error;
pub mod fee;
pub mod hook;
pub mod math;

pub use context::{LeverContext, PoolContext, SwapContext};
pub use error::FlammError;
