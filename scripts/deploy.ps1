# Устанавливаем devnet
solana config set --url devnet

# Собираем программу
Set-Location solana
cargo build-bpf

# Деплоим программу
solana program deploy target/deploy/solana_hello_world.so

# Выводим информацию о кошельке и программе
Write-Host "Wallet address:"
solana address
Write-Host "Program ID:"
solana program show --programs 