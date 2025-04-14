# Solana Hello World Docker

Простой проект для запуска Solana Hello World программы в Docker контейнере на Windows без WSL.

## Требования

- Docker Desktop для Windows
- Git

## Установка и запуск

1. Клонируйте репозиторий:
```bash
git clone https://github.com/your-username/solana-hello-docker.git
cd solana-hello-docker
```

2. Соберите и запустите Docker контейнер:
```bash
docker-compose up -d
```

3. Войдите в контейнер:
```bash
docker exec -it solana_dev bash
```

## Использование

### Создание кошелька
```bash
./scripts/create_wallet.sh
```

### Сборка программы
```bash
./scripts/build.sh
```

### Деплой программы
```bash
./scripts/deploy.sh
```

## Структура проекта

```
solana-hello-docker/
├── Dockerfile
├── docker-compose.yml
├── solana/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── scripts/
│   ├── build.sh
│   ├── deploy.sh
│   └── create_wallet.sh
└── README.md
```

## Проверка версий

Внутри контейнера вы можете проверить версии установленных инструментов:

```bash
solana --version
cargo --version
rustc --version
```

## Примечания

- Программа деплоится на Solana Devnet
- Кошелек сохраняется в файле `wallet.json`
- Для получения SOL на Devnet используется airdrop

## Лицензия

MIT 