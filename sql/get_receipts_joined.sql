SELECT
  r.id as receipt_id , r.item_id, i.name as item_name, i.price as item_price, r.item_qty,
  GROUP_CONCAT(u.name) as payee,
  COUNT(u.id) as payee_count
FROM receipts_users ru
INNER JOIN receipts r
ON ru.receipt_id= r.id
INNER JOIN users u
ON ru.user_id = u.id
INNER JOIN items i
ON r.item_id = i.id
GROUP BY r.id
LIMIT 20 OFFSET ?1;
