# 📰 On-Chain Journalist Credential System (Soroban)

## 📌 Overview

The **On-Chain Journalist Credential System** is a lightweight Soroban smart contract built on the Stellar network. It enables a trusted authority to issue, manage, and verify journalist credentials in a transparent and tamper-proof manner.

This project demonstrates how decentralized identity systems can be used to combat misinformation, prevent impersonation, and establish verifiable trust in digital media.

---

## 🎯 Purpose

In an era of widespread misinformation, verifying the authenticity of journalists is critical. This smart contract provides:

* A **trustless verification system**
* **Immutable credential storage**
* A **simple and efficient on-chain registry**

---

## ⚙️ Smart Contract Functionality

The contract revolves around a single administrator who manages journalist credentials.

### 🔐 Initialization

```rust
init(env: Env, admin: Address)
```

* Sets the contract administrator.
* Can only be executed once.
* Requires authentication from the admin address.

---

### 🪪 Issue Credential

```rust
issue(env: Env, admin: Address, journalist: Address, name: Symbol)
```

* Assigns a credential to a journalist.
* Stores the journalist’s identity (as a Symbol).
* Only callable by the admin.

---

### ❌ Revoke Credential

```rust
revoke(env: Env, admin: Address, journalist: Address)
```

* Removes a journalist’s credential.
* Ensures outdated or invalid credentials can be revoked.
* Only callable by the admin.

---

### ✅ Verify Credential

```rust
verify(env: Env, journalist: Address) -> bool
```

* Returns `true` if the journalist is verified.
* Publicly accessible (no authentication required).

---

### 📄 Get Journalist Name

```rust
get_name(env: Env, journalist: Address) -> Option<Symbol>
```

* Retrieves the stored name/identifier of a journalist.
* Returns `None` if the journalist is not registered.

---

## 🧱 Architecture

* **Admin-based control model**
* **On-chain storage using `DataKey` enum**
* **Address-based identity mapping**
* **Minimal and gas-efficient design**

---

## 🛠️ Tech Stack

* **Language:** Rust (`#![no_std]`)
* **Framework:** Soroban SDK
* **Blockchain:** Stellar (Soroban smart contracts)

---

## 🚀 Getting Started

### 1. Install Requirements

* Rust (stable)
* Soroban CLI

---

### 2. Build the Contract

```bash
soroban contract build
```

---

### 3. Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/journalist_credential.wasm \
  --network testnet
```

---

### 4. Interact with the Contract

Example: Verify a journalist

```bash
soroban contract invoke \
  --fn verify \
```

---

## ✨ Key Features

* 🔐 **Secure Admin Control**
  Only the authorized admin can issue or revoke credentials.

* 📜 **Immutable Credential Registry**
  Records are stored on-chain and cannot be tampered with.

* 🌐 **Public Verification**
  Anyone can verify journalist credentials without permission.

* ⚡ **Lightweight & Efficient**
  Minimal design ensures low fees and fast execution.

---

## ⚠️ Limitations

* Single admin (no multi-signature support)
* No metadata beyond a simple `Symbol` name
* No expiration or renewal mechanism

---

## 🔮 Future Enhancements

* Multi-admin or DAO governance
* Rich metadata (organization, country, credentials)
* Expiry timestamps for credentials
* NFT-based identity badges
* Integration with decentralized identity (DID) standards

---

## 📄 License

This project is licensed under the **MIT License**.

---
Wallet address: GB2IDF4E4Z5DZMIDAWGQNN4IKQLRQIS3LGGYKKRUNVTSX6CKPERL74ZW

## 🤝 Contributing

Contributions are welcome! Feel free to submit issues or pull requests to improve the contract.

---
