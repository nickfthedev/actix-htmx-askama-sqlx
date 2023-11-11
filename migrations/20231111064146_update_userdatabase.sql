-- Add migration script here
ALTER TABLE "user"
ADD COLUMN username VARCHAR(255) NOT NULL;
