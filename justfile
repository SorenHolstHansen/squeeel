default:
  just --list

postgres_container_id := "postgres"
mysql_container_id := "mysql"

up-examples-docker:
    docker compose -f examples/docker-compose.yml up -d

seed-postgres:
    docker exec -u postgres {{postgres_container_id}} psql postgres postgres -c "CREATE DATABASE squeeel;" || true
    docker exec -u postgres {{postgres_container_id}} psql squeeel postgres < examples/node-postgres/seed.sql

seed-mysql:
    docker exec -i mysql {{mysql_container_id}} -u root -prootpassword -e "CREATE DATABASE IF NOT EXISTS squeeel;"
    docker exec -i mysql {{mysql_container_id}} -u root -prootpassword squeeel < examples/mysql2/seed.sql

seed-sqlite:
    rm examples/better-sqlite3/example.db || true
    cp examples/better-sqlite3/seeded_db.db examples/better-sqlite3/example.db

gen-postgres-example: seed-postgres
    #!/usr/bin/env bash
    set -euxo pipefail
    export POSTGRES_URL=postgres://postgres:postgres@localhost:5432/squeeel
    cd squeeel-cli
    cargo run -- gen ../examples/node-postgres

gen-mysql-example: seed-mysql
    #!/usr/bin/env bash
    set -euxo pipefail
    export MYSQL_URL=mysql://root:rootpassword@localhost:3306/squeeel
    cd squeeel-cli
    cargo run -- gen ../examples/mysql2

gen-sqlite-example: seed-sqlite
    #!/usr/bin/env bash
    set -euxo pipefail
    export SQLITE_URL=example.db
    cd squeeel-cli
    cargo run -- gen ../examples/better-sqlite3

gen-examples: up-examples-docker gen-postgres-example gen-mysql-example gen-sqlite-example
    
