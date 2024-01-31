use super::*;

pub fn get_perpetual_orders(
    deps: Deps<ElysQuery>,
    pagination: Option<PageRequest>,
    order_owner: Option<String>,
    order_type: Option<PerpetualOrderType>,
    order_status: Option<Status>,
) -> Result<GetPerpetualOrdersResp, ContractError> {
    let orders: Vec<PerpetualOrder> = PERPETUAL_ORDER
        .prefix_range(deps.storage, None, None, Order::Ascending)
        .filter_map(|res| res.ok().map(|r| r.1))
        .collect();

    let (orders, page_response) = match pagination {
        Some(pagination) => {
            let (orders, page_resp) = pagination.filter(orders)?;
            (orders, Some(page_resp))
        }
        None => (orders, None),
    };

    if orders.is_empty() {
        return Ok(GetPerpetualOrdersResp {
            page_response,
            orders,
        });
    };

    let orders: Vec<PerpetualOrder> = orders
        .iter()
        .filter(|order| {
            order_owner
                .as_ref()
                .map_or(true, |owner| owner == &order.owner)
                && order_type
                    .as_ref()
                    .map_or(true, |order_type| order_type == &order.order_type)
                && order_status
                    .as_ref()
                    .map_or(true, |status| &order.status == status)
        })
        .cloned()
        .collect();

    let page_response = if let Some(page_response) = page_response {
        match page_response.total {
            Some(_) => Some(PageResponse {
                next_key: page_response.next_key,
                total: Some(orders.len() as u64),
            }),
            None => Some(page_response),
        }
    } else {
        None
    };

    Ok(GetPerpetualOrdersResp {
        page_response,
        orders,
    })
}
