-- Add carbonation packaging fields to batches
ALTER TABLE batches ADD COLUMN packaging_temp_c REAL;
ALTER TABLE batches ADD COLUMN carbonation_sugar_type TEXT;
ALTER TABLE batches ADD COLUMN priming_sugar_g REAL;
ALTER TABLE batches ADD COLUMN serving_pressure_kpa REAL;
