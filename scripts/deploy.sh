#!/bin/bash

# Устанавливаем devnet
solana config set --url devnet

# Собираем программу
cd solana
cargo build-bpf

# Деплоим программу
solana program deploy target/deploy/solana_hello_world.so

# Выводим информацию о кошельке и программе
echo "Wallet address:"
solana address
echo "Program ID:"
solana program show --programs 