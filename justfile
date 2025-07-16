default:
  just --list

postgres_container_id := "postgres"
mysql_container_id := "mysql"
postgres_url := "postgres://postgres:postgres@localhost:5432/squeeel"
mysql_url := "mysql://root:rootpassword@localhost:3306/squeeel"
sqlite_url := "example.db"

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

[working-directory: 'squeeel-cli']
gen-postgres-example:
    export POSTGRES_URL={{postgres_url}}
    cargo run -- gen ../examples/node-postgres

[working-directory: 'squeeel-cli']
gen-mysql-example:
    export MYSQL_URL={{mysql_url}}
    cargo run -- gen ../examples/mysql2

[working-directory: 'squeeel-cli']
gen-sqlite-example:
    export SQLITE_URL={{sqlite_url}}
    cargo run -- gen ../examples/better-sqlite3

gen-examples: up-examples-docker seed-postgres seed-mysql seed-sqlite gen-postgres-example gen-mysql-example gen-sqlite-example
    
