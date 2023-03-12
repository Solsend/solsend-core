# Solsend Core

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