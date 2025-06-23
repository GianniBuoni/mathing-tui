SELECT
  ru.receipt_id as receipt_id,
  GROUP_CONCAT( CAST(ru.user_id as TEXT)) as user_ids,
  COUNT(ru.user_id) as user_count,
  i.id as item_id, i.name as item_name, i.price as item_price,
  r.item_qty as item_qty
from receipts_users ru
INNER JOIN users u ON ru.user_id= u.id
INNER JOIN receipts r ON ru.receipt_id = r.id
INNER JOIN items i ON r.item_id = i.id
GROUP BY ru.receipt_id
