
CREATE OR REPLACE FUNCTION high_low_score(low_price FLOAT, current_price FLOAT, high_price FLOAT)
RETURNS FLOAT
AS $$
DECLARE
  score FLOAT;
BEGIN
  score := (((current_price)::double precision - (low_price)::double precision) / (NULLIF((high_price - low_price), 0))::double precision);
  RETURN COALESCE(ROUND(score::numeric * 100.0, 2), 0);
END;
$$ LANGUAGE plpgsql;
