-- https://www.postgresql.org/message-id/1187129510.11237.351.camel%40linda.lfix.co.uk
-- https://jetrockets.com/blog/izca8kaqvq-how-to-delete-polymorphic-models-cascade
CREATE OR REPLACE FUNCTION drop_points_after_asset()
RETURNS TRIGGER
SET SCHEMA 'public'
AS $$
BEGIN
  DELETE FROM points WHERE "points"."resource_id" = OLD.id AND "points"."resource_type" = 'Asset';
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;
