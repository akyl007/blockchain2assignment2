#!/bin/bash
cd /project/solana
source $HOME/.cargo/env
solana config set --url devnet
solana airdrop 1
solana program deploy target/deploy/solana_hello_world.so 