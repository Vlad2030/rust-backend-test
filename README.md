# rust-backend-test

## How to run

### Create `.env` file

example from `.env.example`:

```bash
# backend
BACKEND_TITLE=rust-backend
BACKEND_VERSION=0.0.1
BACKEND_CONTAINER_NAME=rust-backend
BACKEND_HOST=0.0.0.0
BACKEND_PORT=6969
BACKEND_LOG_LEVEL=info
BACKEND_WORKERS=8

# database
DB_CONTAINER_NAME=rust-backend-postgres
DB_CONTAINER_HOST=${DB_CONTAINER_NAME}
DB_CONTAINER_PORT=5432
DB_NAME=backend
DB_USER=admin
DB_PASSWORD=password

# postgres
POSTGRES_USER=${DB_USER}
POSTGRES_PASSWORD=${DB_PASSWORD}
POSTGRES_DB=${DB_NAME}
PGDATA=/data/postgres

```

### Start docker containers

```
docker-compose -f docker-compose.prod.yaml up -d
```

## Migrations

```bash
cd migrations && alembic upgrade head
```

## Tests

```bash
cd tests && pytest -v
```
