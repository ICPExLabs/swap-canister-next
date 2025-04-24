# Swap Canister Project

## Project Overview

This project involves multiple smart contracts (canisters), primarily the **Swap Canister**, the **Token Archive Canisters** and the **Swap Archive Canisters**, which handle token swaps, liquidity management, data archiving, and more. The project strictly follows Rust best practices:

- No unsafe code
- Comprehensive documentation
- Strict checks for `Send` safety in async code
- Static code analysis rules are enforced

---

## Key Features

### Swap Canister

1. **Token Pair Liquidity Management**

   Add liquidity to token pairs via `pair_liquidity_add`.
   Before adding, the system checks whether the user has enough balance using `tokens_balance_of`.

2. **Token Pair Pool Query**

   Query all token pair pools via `pairs_query`.

3. **Token Pair Pool Creation**

   Create new pools using `pair_create`.

4. **Fee Configuration Query**

   Use `pair_query` to check the fee-receiving account.

5. **Permission Management**

   Use `permission_query` to query current permission holders and `permission_update` to update them.

6. **Pause/Resume Management**

   Use `pause_query` to check pause status and `pause_replace` to update it.

7. **Scheduler Management**

   Query and update scheduling data via `schedule_find` and `schedule_replace`.

---

### Archive Canisters

1. Token Archive Canister handles archiving and retrieving historical token-related data.

2. Swap Archive Canister handles archiving and retrieving historical swap-related data.

---

## Code Structure

### Swap Canister

- `services`: Canister interface definitions
- `utils`: Utility functions
- `types`: All type definitions
- `stable`: Stable storage and migration logic
- `business`: business logic
- `common`: Common shared types and utilities (must be last due to candid interface)

### Archive Canisters

- `types`: Type definitions
- `stable`: Stable storage logic
- `business`: Business logic for archive
- `http`: Archive-related HTTP module
- `common`: Shared types and candid interface

---

## Code Standards

- `#![deny(unsafe_code)]`: No unsafe Rust allowed
- `#![deny(missing_docs)]`: Enforce documentation for all public items
- `#![warn(rustdoc::broken_intra_doc_links)]`: Warn on broken doc links
- `#![warn(clippy::future_not_send)]`: Warn on `!Send` futures
- `#![deny(clippy::unwrap_used)]`, `expect_used`, `panic`: Disallow panic-prone code

---

## Common Function Examples

### Check Token Balance

Function: `token_balance_of(token, account)`

- Ensures the token balance of a user is sufficient before proceeding with an operation.
- Returns a `BusinessError::insufficient_balance` if not enough.

### Add Liquidity to Token Pair

Function: `pair_liquidity_add(args, retries)`

- Adds token0/token1 to a specified pool.
- Returns the amount of liquidity added and actual token amounts consumed.
- Refreshes certified data afterward.

### Swap tokens

Function: `pair_swap_exact_tokens_for_tokens(args, retries)`

- Swap tokens.
- Returns the amount of each tokens changed.

---

## Contributing

If you'd like to contribute:

1. Fork the repository
2. Create a feature branch:
   `git checkout -b feature/your-feature-name`
3. Commit your changes:
   `git commit -m 'Add some feature'`
4. Push the branch:
   `git push origin feature/your-feature-name`
5. Open a Pull Request

---

## License

This project is licensed under the **Business Source License**.
For details, refer to the `LICENSE` file.
