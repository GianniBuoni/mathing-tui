use super::*;

impl TryFrom<Vec<StoreJoinRow>> for StoreJoinTable {
    type Error = String;

    fn try_from(value: Vec<StoreJoinRow>) -> Result<Self, Self::Error> {
        let mut payees = vec![];
        let StoreJoinRow {
            price,
            item,
            item_qty,
            ..
        } = value[0].clone();

        for row in value {
            if price != row.price
                || item_qty != row.item_qty
                || item != row.item
            {
                return Err(
                    "Feilds: \"item, item_qty, price\" must match.".into()
                );
            }
            payees.push(row.payee);
        }

        Ok(Self {
            payees,
            price,
            item,
            item_qty,
        })
    }
}
