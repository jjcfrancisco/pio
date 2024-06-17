run-bin:
    cargo build --release
    cp target/release/pio .
    ./pio

@create-db:
    docker compose up -d

@delete-db:
    docker stop postgis && docker rm postgis

@start-db:
    docker start postgis
