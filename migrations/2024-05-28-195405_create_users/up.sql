-- Your SQL goes here
CREATE TABLE "users" (
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"boosty_id" BIGSERIAL NOT NULL,
	"expires_at" TIMESTAMP NOT NULL
);

