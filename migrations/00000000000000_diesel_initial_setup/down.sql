-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.
DROP TABLE IF EXISTS auction, item CASCADE;

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();
