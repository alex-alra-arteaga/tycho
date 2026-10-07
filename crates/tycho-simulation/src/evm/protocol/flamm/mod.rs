// Copyright (c) 2026 Everlong Labs Limited

//! Everlong FLAMM (Base 8453, c104 @ `80abd43`): a native Tycho integration of the pool's swap
//! venue and lever-up venue, quoted to the wei from tracked state without a VM.
//!
//! Every module is a port of the Go simulator of the same pool, each function tied to its
//! Solidity line. Every entry takes the block timestamp it runs at: the feed checks, the
//! spread's age and, through the Router, the Morpho accrual and the IRM adaptation are evaluated
//! there. Every refusal is the revert the chain would raise ([`FlammError`]), and every
//! arithmetic step keeps Solidity's operation order and rounding.
//!
//! | module | contract |
//! |---|---|
//! | [`almcurve`] | `AlmCurve.sol`, the normalized reservation curve the swap hook trades on |
//! | [`fee`] | `EverlongStrategy.sol`, the fill-fee law |
//! | [`hook`] | `EverlongHook.sol`, the swap hook's lazy book rescale, fill and commit |
//! | [`morpho`] | Morpho Blue v1.0.0 share math and market transitions |
//! | [`irm`] | `AdaptiveCurveIrm` v1.0.0 |
//! | [`account`] | `MorphoBlueAccount.sol`, the venue account's views and Router-driven mutators |
//! | [`router`] | `MMRouterLib.sol` / `MMRouter.sol` and the `FLAMMSwapLib` settlement legs |
//! | [`gate`] | `FLAMMGateLib.sol`, the credit gate, room, frame, NAV and entry / exit gates |
//! | [`levcurve`] | `CollRebalancerMath.sol`, the frozen leverage curve |
//! | [`levhook`] | `EverlongLeverageHook.sol`, the leverage venue's frame and fill |
//! | [`pricefeed`] | `PriceFeed.sol` over the Chainlink rounds and the sequencer feed |
//! | [`context`] | `IFLAMMHooks.sol` / `IFLAMMLeverage.sol`, the frames core hands its hooks |
//! | [`error`] | one variant per deployed custom error, with the selector table |
//! | [`math`] | `Math.mulDiv` / `ceilDiv` / `sqrt`, `Mul512` and checked `int256` arithmetic |

pub mod account;
pub mod almcurve;
pub mod context;
pub mod error;
pub mod fee;
pub mod gate;
pub mod hook;
pub mod irm;
pub mod levcurve;
pub mod levhook;
pub mod math;
pub mod morpho;
pub mod pricefeed;
pub mod router;

pub use context::{LeverContext, PoolContext, SwapContext};
pub use error::FlammError;
