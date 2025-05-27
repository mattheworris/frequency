# XCM Trapped Assets

## Context and Scope

Cross-chain transfers with XCM can fail leaving assets stranded on the executing chain. This document summarises how Frequency handles these "trapped" assets and how they can be recovered.

## Where do trapped assets exist?

When XCM execution fails after assets have been withdrawn from the sender, the `pallet_xcm` logic deposits those assets into the local chain. The assets are held in the XCM pallet's holding account on the chain where the failure occurred.

## How to inspect why assets were trapped

`pallet_xcm` emits an `AssetsTrapped` event whenever assets are moved into the holding account. The event lists the location and the assets so it can be queried from the blockchain's event log. Node logs may include additional diagnostic messages but the on-chain event is sufficient to detect a trap.

## How are assets un-trapped?

The assets remain owned by the origin location recorded in the trap event. Anyone authorised for that location may submit a `claim_assets` call to `pallet_xcm` specifying the trapped assets and destination account. On successful claim the pallet releases the assets from the holding account and transfers them to the desired beneficiary.

### Determining who is authorised

When `claim_assets` is invoked the runtime must verify that the caller represents the same XCM location that originally owned the assets. This is handled via the `LocationToAccountId` mapping which converts a location into a local account:

```rust
pub type LocationToAccountId = (
    ParentIsPreset<AccountId>,
    SiblingParachainConvertsVia<Sibling, AccountId>,
    AccountId32Aliases<RelayNetwork, AccountId>,
);
```

Incoming XCM origins are turned into dispatch origins using this mapping through `XcmOriginToTransactDispatchOrigin`:

```rust
pub type XcmOriginToTransactDispatchOrigin = (
    SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
    RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
    SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
    SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
    XcmPassthrough<RuntimeOrigin>,
);
```

`pallet_xcm` relies on these conversions to check that the caller's origin resolves to the same location recorded in the trap event. Only when this match succeeds will the claim be accepted and the trapped assets released.

## Situations where assets cannot be unlocked

Trapped assets may remain locked if their origin location cannot be resolved or if `claim_assets` is called with incorrect parameters. Misconfigured asset registrations or a mismatch in XCM versioning may also prevent recovery. These cases should be avoided by ensuring asset and location registration is consistent across chains.

## Are pallet facilities sufficient?

`pallet_xcm` together with `pallet_assets` provide the primitives required to trap and later claim assets. No additional pallets are necessary for standard recovery flows.
