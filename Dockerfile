# Utiliser une image officielle Rust comme base
FROM rust:1.72-slim

# Installer OpenSSL et pkg-config
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Définir le répertoire de travail à l'intérieur du conteneur
WORKDIR /usr/src/app

# Copier le fichier Cargo.toml et Cargo.lock pour le caching des dépendances
COPY Cargo.toml Cargo.lock ./

# Télécharger et compiler les dépendances uniquement
RUN cargo fetch

# Copier le reste des fichiers de l'application dans le conteneur
COPY . .

# Construire l'application
RUN cargo build --release

# Exécuter les tests
RUN cargo test --release

# Exécuter l'application
CMD ["cargo", "run", "--release"]
