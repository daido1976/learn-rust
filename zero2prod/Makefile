DC := docker compose

psql:
	$(DC) exec postgres psql -U postgres -d zero2prod

# Frequently used commands of sqlx-cli.
# If you want to know more, use `sqlx -h`.
db_setup:
	sqlx database setup

db_reset:
	sqlx database reset

migration_run:
	sqlx migrate run
