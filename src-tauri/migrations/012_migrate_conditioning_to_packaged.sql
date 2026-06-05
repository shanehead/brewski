-- Merge "conditioning" status into "packaged". The conditioning_date column
-- is kept so existing data is not lost, but the status value is retired.
UPDATE batches SET status = 'packaged' WHERE status = 'conditioning';
