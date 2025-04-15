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

# Установка конкретной версии Rust (1.72.0)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.72.0
ENV PATH="/root/.cargo/bin:$PATH"

# Установка Solana CLI и BPF toolchain
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.17.15/install)" && \
    export PATH="/root/.local/share/solana/install/active_release/bin:$PATH" && \
    solana-install init v1.17.15 && \
    solana-install update && \
    rustup component add rust-src --toolchain 1.72.0

# Установка дополнительных компонентов для BPF
RUN rustup target add bpfel-unknown-unknown --toolchain 1.72.0

# Создание рабочей директории
WORKDIR /project

# Копирование файлов проекта
COPY . .

# Установка зависимостей и сборка проекта
RUN cd solana && cargo build-bpf

# Установка прав на скрипты (если есть)
RUN if [ -d "/project/scripts" ]; then chmod +x /project/scripts/*.sh; fi