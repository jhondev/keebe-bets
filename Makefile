help:
	@echo "Available make commands:"
	@cat Makefile | grep '^[a-z][^:]*:' | cut -d: -f1 | sort | sed 's/^/  /'

setup:
	cd api && pnpm install && go install github.com/sqlc-dev/sqlc/cmd/sqlc@v1.24.0 \
		&& cp ./.env.example ./.env
	cd web && pnpm install

run-api:
	cd api && go run cmd/*.go

.PHONY: web
web:
	cd web && pnpm dev

migrate:
	cd web && pnpx prisma migrate dev && pnpx prisma migrate deploy

reset:
	cd web && pnpx prisma migrate reset
	make seed

seed:
	cd web && pnpm seed

gen-db:
	cd web && pnpx prisma generate

studio:
	cd web && pnpx prisma studio