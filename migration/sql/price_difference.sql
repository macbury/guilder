CREATE OR REPLACE FUNCTION price_difference(current_price double precision, last_price double precision)
RETURNS double precision
AS $$
BEGIN
  RETURN ROUND((COALESCE(current_price, 0) - COALESCE(last_price, 0))::numeric, 2);
END;
$$ LANGUAGE plpgsql;
