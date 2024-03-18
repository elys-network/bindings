use super::*;

pub fn get_spot_orders(
    deps: Deps<ElysQuery>,
    pagination: Option<PageRequest>,
    order_owner: Option<String>,
    order_type: Option<SpotOrderType>,
    order_status: Option<Status>,
) -> Result<GetSpotOrdersResp, ContractError> {
    let orders: Vec<SpotOrder> = match order_owner {
        Some(addr) => match USER_SPOT_ORDER.may_load(deps.storage, &addr)? {
            Some(v) => v
                .iter()
                .filter_map(|id| match SPOT_ORDER.load(deps.storage, *id) {
                    Ok(order) if order.status == Status::Pending => Some(order),
                    _ => None,
                })
                .collect(),
            None => vec![],
        },
        None => SPOT_ORDER
            .prefix_range(deps.storage, None, None, Order::Ascending)
            .filter_map(|res| res.ok().map(|r| r.1))
            .collect(),
    };

    if orders.is_empty() {
        return Ok(GetSpotOrdersResp {
            page_response: if let Some(page) = pagination {
                Some(PageResponse::empty(page.count_total))
            } else {
                None
            },
            orders,
        });
    };

    let orders: Vec<SpotOrder> = orders
        .iter()
        .filter(|order| {
            order_type
                .as_ref()
                .map_or(true, |order_type| order_type == &order.order_type)
                && order_status
                    .as_ref()
                    .map_or(true, |status| &order.status == status)
        })
        .cloned()
        .collect();

    let (orders, page_response) = match pagination {
        Some(pagination) => {
            let (orders, page_resp) = pagination.filter(orders)?;
            (orders, Some(page_resp))
        }
        None => (orders, None),
    };

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

    Ok(GetSpotOrdersResp {
        page_response,
        orders,
    })
}
