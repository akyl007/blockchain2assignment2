#!/bin/bash
cd /project
source $HOME/.cargo/env
solana-keygen new --no-bip39-passphrase -o wallet.json
solana config set --url devnet
solana airdrop 1 