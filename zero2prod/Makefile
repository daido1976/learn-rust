DC := docker compose
TEST_DATABASE_URL := postgres://postgres:password@localhost:15432/test

psql:
	$(DC) exec postgres psql -U postgres -d zero2prod

test_psql:
	$(DC) exec postgres psql -U postgres -d test

# Frequently used commands of sqlx-cli.
# If you want to know more, use `sqlx -h`.
db_setup:
	sqlx database setup

db_reset:
	sqlx database reset -y

migration_run:
	sqlx migrate run

test_db_setup:
	sqlx database setup --database-url $(TEST_DATABASE_URL) -y

test_db_reset:
	sqlx database reset --database-url $(TEST_DATABASE_URL) -y
