# faucet-substrate
A faucet that mints tokens to the requestor every N blocks

### Task

Create a faucet that mints tokens to the requestor every, say, 16 blocks, meaning the requestor cannot get tokens if 16 blocks had not passed since the last mint. Also, since this transaction is lightweight and cannot be repeated more than 1 time in 16 blocks, **disable** transaction fees for that. The transaction must be signed.
You can do the minting using the `Currency` trait and setting it to `pallet_balances`’s implementation (Balances in `runtime/lib.rs`). Use `issue` and `resolve_into_existing` methods. The implementation is totally up to you (minting right away, approving mints by superuser/governance - how far you want to go).

### To Do:
- [X] Create a faucet that mints tokens to the requestor 
- [X] This transaction cannot be repeated more than 1 time in 16 blocks
- [X] The transaction must be signed.
- [ ] Disable transaction fees for that
