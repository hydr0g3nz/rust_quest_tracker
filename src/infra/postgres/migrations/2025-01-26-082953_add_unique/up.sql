-- Your SQL goes here
ALTER TABLE guild_commanders
ADD CONSTRAINT guild_commanders_username_unique UNIQUE (username);