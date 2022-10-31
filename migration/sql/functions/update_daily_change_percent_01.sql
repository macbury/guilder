CREATE OR REPLACE FUNCTION daily_change_percent(current_price FLOAT, last_price FLOAT)
RETURNS FLOAT
AS $$
BEGIN
  RETURN COALESCE(ROUND((((current_price - last_price) / last_price) * 100.0)::numeric, 4), 0);
END;
$$ LANGUAGE plpgsql;
