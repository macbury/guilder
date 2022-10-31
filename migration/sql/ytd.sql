CREATE OR REPLACE FUNCTION ytd(current_price FLOAT, price_at_year_start FLOAT)
RETURNS FLOAT
AS $$
BEGIN
  RETURN COALESCE(ROUND((((current_price / price_at_year_start) - 1.0) * 100.0)::numeric, 2), 0);
END;
$$ LANGUAGE plpgsql;
