# Solana Hello World Program

Простой пример программы на Solana, демонстрирующий работу со счетчиком.

## Технологический стек

### Основные технологии
- **Solana**: v1.17.15
- **Rust**: v1.72.0
- **Docker**: Ubuntu 22.04
- **BPF (Berkeley Packet Filter)**: Для выполнения программ на Solana

### Зависимости проекта
```toml
[dependencies]
solana-program = "1.17.15"
bumpalo = "3.12.0"
arrayref = "0.3.6"
thiserror = "1.0"
```

## Требования к окружению

### Системные требования
- Docker Desktop для Windows
- Git
- Минимум 4GB RAM
- 20GB свободного места на диске

### Версии инструментов
- Docker: последняя стабильная версия
- Solana CLI: v1.17.15
- Rust: v1.72.0
- Ubuntu: 22.04 (в контейнере)

## Установка и настройка

### 1. Клонирование репозитория
```bash
git clone <repository-url>
cd solana-hello-world
```

### 2. Запуск Docker контейнера
```bash
docker-compose up -d
```

### 3. Вход в контейнер
```bash
docker exec -it solana_dev bash
```

## Структура проекта

```
solana-hello-world/
├── Dockerfile              # Конфигурация Docker образа
├── docker-compose.yml      # Конфигурация Docker Compose
├── solana/                 # Исходный код программы
│   ├── Cargo.toml         # Зависимости Rust
│   ├── Cargo.lock         # Версии зависимостей
│   └── src/
│       └── lib.rs         # Основной код программы
├── scripts/               # Скрипты для работы с проектом
│   ├── build.sh          # Скрипт сборки
│   ├── deploy.sh         # Скрипт деплоя
│   └── create_wallet.sh  # Скрипт создания кошелька
└── README.md             # Документация
```

## Разработка

### Сборка проекта
```bash
cd /project/solana
cargo build-bpf
```

### Деплой на devnet
```bash
solana config set --url devnet
solana program deploy target/deploy/solana_hello_world.so
```

### Проверка версий
```bash
solana --version
cargo --version
rustc --version
```

## Архитектура программы

### Основные компоненты
1. **Counter**: Структура данных для хранения счетчика
   - `is_initialized`: Флаг инициализации
   - `count`: Значение счетчика

2. **Process Instruction**: Точка входа программы
   - Проверка владельца аккаунта
   - Проверка подписи
   - Инкрементация счетчика

### Безопасность
- Проверка владельца программы
- Проверка подписи транзакции
- Обработка переполнения счетчика

## Тестирование

### Локальное тестирование
```bash
cargo test
```

### Тестирование на devnet
1. Деплой программы
2. Создание тестовых транзакций
3. Проверка состояния счетчика

## Демонстрация

[Здесь будут скриншоты после деплоя]

## Примеры использования

### Инициализация счетчика
```rust
let mut counter = Counter::unpack_unchecked(&counter_account.data.borrow())?;
if !counter.is_initialized {
    counter.is_initialized = true;
    counter.count = 0;
}
```

### Инкрементация счетчика
```rust
counter.count = counter.count.checked_add(1).ok_or(ProgramError::Overflow)?;
```

## Устранение неполадок

### Частые проблемы
1. **Ошибка сборки BPF**
   - Проверьте версию Rust
   - Убедитесь в наличии всех зависимостей

2. **Ошибка деплоя**
   - Проверьте баланс кошелька
   - Убедитесь в правильности конфигурации devnet

3. **Ошибки выполнения**
   - Проверьте права доступа
   - Убедитесь в правильности структуры данных

## Лицензия

MIT License

## Контакты

[Здесь контактная информация]

## История изменений

### v1.0.0
- Первоначальный релиз
- Базовая функциональность счетчика
- Поддержка Docker 