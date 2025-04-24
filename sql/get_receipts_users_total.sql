SELECT
	ru.receipt_id,
  GROUP_CONCAT(ru.user_id) as user_id,
  i.price * CAST(r.item_qty as REAL) / CAST( COUNT(ru.user_id) as REAL) as total
FROM receipts_users ru
INNER JOIN receipts r ON ru.receipt_id = r.id
INNER JOIN  users u ON ru.user_id = u.id
INNER JOIN  items i on r.item_id = i.id
GROUP by ru.receipt_id;
