

# solxplain

**solxplain** is a Rust-based CLI tool for inspecting Solana accounts and transactions, turning raw RPC responses and execution logs into **structured, human-readable output**.



## Features

### Transaction Inspection

* Fetch transactions by signature
* Display execution logs in call-order
* Group logs into nested program invocations
* Show program success/failure boundaries
* Works on devnet, mainnet and testnet

### Account Inspection

* Fetch account state by public key
* Display:

  * lamports
  * data length
  * owner
  * executable flag
  * rent epoch
* Useful for quick debugging and sanity checks

### Design Principles

* Deterministic output
* No instruction guessing
* Logs are structured

---

## Installation

Clone and build from source:

```bash
git clone https://github.com/idorocodes/solxplain
cd solxplain
cargo build --release
```

Binary will be available at:

```bash
target/release/solxplain
```

On Windows:

```bash
cargo install solxplain 
```

---

## Usage

### Inspect an Account

```bash
solxplain account <ACCOUNT_PUBKEY> --cluster devnet
```

Example output:

```
Account:
Lamports: 16251629640
Data Length: 0
Owner: 11111111111111111111111111111111
Executable: false
Rent Epoch: 18446744073709551615
```

---

### Inspect a Transaction

```bash
solxplain tx <TX_SIGNATURE> --cluster devnet
```

Example output (simplified):

```
Transaction Slot: 432102040
Status: Success

Program: Compute Budget
  Result: Success

Program: Slink Program
  Log: Instruction: Create
  Inner Program: System Program
    Result: Success
  Result: Success
```



---

## How It Works (High Level)

1. Fetch data via Solana RPC
2. Read transaction log messages in order
3. Print log

No instruction decoding is assumed unless explicitly known.



## Roadmap (Optional / Future)

* JSON output (`--json`)
* Program name registry expansion
* SPL Token account decoding
* Instruction-level summaries
* Mainnet support
* Web UI wrapper


