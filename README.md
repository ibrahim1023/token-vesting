# Vesting Anchor Program

A Solana Anchor program for token vesting, allowing companies to set up token vesting schedules for employees, including cliffs and linear vesting, and enabling employees to claim their vested tokens.

## Features

- Create a vesting account for a company with its own treasury
- Initialize employee vesting schedules with start, cliff, and end times
- Employees can claim vested tokens after cliff and over time
- Secure, PDA-backed accounts and token transfers via CPI

## 🛠️ Tech Stack

- ⚙️ Solana & Anchor framework
- 🧩 `anchor_spl::token_interface` for Token-2022 compatibility
- 🔁 `associated_token` for handling ATAs
- 🧪 Mocha + Chai for integration testing
- 🛡 PDA (Program Derived Addresses) for access control

## 🧪 Running Tests

Run integration tests with:

```bash
anchor test
```

```
vesting
    ✔ should create a vesting account
    ✔ should fund the treasury token account
    ✔ should create an employee vesting account
    ✔ should claim tokens (1016ms)
```

## 🔐 Security Notes

- PDA vaults ensure only the program can move tokens.
- transfer_checked ensures correct decimals and mint match.
- Account constraints (e.g., has_one, seeds) prevent spoofing.
