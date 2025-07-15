default:
  just --list
  
up-examples-databases:
    docker compose -f examples/docker-compose.yml up -d

seed-postgres:
    docker cp examples/node-postgres/seed.sql postgres:/seed.sql
    docker exec -u postgres postgres psql postgres postgres -c "CREATE DATABASE squeeel;" || true
    docker exec -u postgres postgres psql squeeel postgres -f /seed.sql

seed-mysql:
    docker cp examples/mysql2/seed.sql mysql:/seed.sql
    docker exec -i mysql mysql -u root -prootpassword -e "CREATE DATABASE IF NOT EXISTS squeeel;"
    docker exec -i mysql mysql -u root -prootpassword squeeel < examples/mysql2/seed.sql

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

update-examples: up-examples-databases gen-postgres-example gen-mysql-example
    
