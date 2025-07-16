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
    docker cp examples/node-postgres/seed.sql {{postgres_container_id}}:/seed.sql
    docker exec -u postgres {{postgres_container_id}} psql postgres postgres -c "CREATE DATABASE squeeel;" || true
    docker exec -u postgres {{postgres_container_id}} psql squeeel postgres -f /seed.sql

seed-mysql:
    docker exec -i mysql {{mysql_container_id}} -u root -prootpassword -e "CREATE DATABASE IF NOT EXISTS squeeel;"
    docker exec -i mysql {{mysql_container_id}} -u root -prootpassword squeeel < examples/mysql2/seed.sql

seed-sqlite:
    rm examples/better-sqlite3/example.db || true
    cp examples/better-sqlite3/seeded_db.db examples/better-sqlite3/example.db

[working-directory: 'squeeel-cli']
gen-postgres-example:
    cargo run -- gen --database-url {{postgres_url}} ../examples/node-postgres

[working-directory: 'squeeel-cli']
gen-mysql-example:
    cargo run -- gen --database-url {{mysql_url}} ../examples/mysql2

[working-directory: 'squeeel-cli']
gen-sqlite-example:
    cargo run -- gen --database-url {{sqlite_url}} ../examples/better-sqlite3

gen-examples: up-examples-docker seed-postgres seed-mysql seed-sqlite gen-postgres-example gen-mysql-example gen-sqlite-example
    
