FROM ubuntu:22.04

# Установка необходимых пакетов
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

# Установка Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

# Установка Solana CLI и BPF toolchain
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.17.15/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# Создание рабочей директории
WORKDIR /project

# Копирование файлов проекта
COPY . /project 