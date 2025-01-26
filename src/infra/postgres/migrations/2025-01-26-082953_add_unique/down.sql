-- This file should undo anything in `up.sql`
ALTER TABLE guild_commanders
DROP CONSTRAINT guild_commanders_username_unique;