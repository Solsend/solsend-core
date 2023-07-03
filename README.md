# Solsend Core

This repository contains the core smart contracts for Solsend. It is **not** production-ready code since the code is written using Seahorse in Python, which is still in beta. However, the smart contracts are completely functional and are deployed on devnet as well.

The program ID on devnet is 4FF8j7wY3uxZPbKKRA6RYzmpfkPnbvxGvWxNaBLeino5.

### Deploy

To deploy the smart contracts in this repository:

```
$ solana config set --url devnet
$ solana airdrop 3
$ seahorse build
$ solana address -k target/deploy/solsend_core-keypair.json     # Get the new program id
$ seahorse build
$ anchor deploy
```

### Build

To build the smart contracts in this repository:

```
$ seahorse build
```

NOTE: If you changed the program authority, delete the `target/` folder and then build again.