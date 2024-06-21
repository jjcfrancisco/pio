@run-bin area:
    cargo build --release
    cp target/release/pio .
    ./pio -u postgresql://pio:password@localhost:25432/master -o {{ area }}-latest.osm.pbf

@drop-table:
    psql postgresql://pio:password@localhost:25432/master -c "DROP TABLE pio;"

@count-rows:
    psql postgresql://pio:password@localhost:25432/master -c "SELECT COUNT(*) FROM pio;"

@show-rows:
    psql postgresql://pio:password@localhost:25432/master -c "SELECT * FROM pio LIMIT 10;"

@create-db:
    docker compose up -d

@delete-db:
    docker stop postgis && docker rm postgis

@start-db:
    docker start postgis
