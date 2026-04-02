use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Deserializer, Serialize};

// All public types are re-exported from the OpenAPI-generated module.
pub use crate::generated::types::{
    AmendOrderRequest, CreateOrderRequest, CreateOrderRequestAction, CreateOrderRequestSide,
    CreateOrderRequestTimeInForce, CreateSubaccountResponse, DecreaseOrderRequest, EventPosition,
    Fill, FillAction as Action, FillSide as Side, FixedPointCount, FixedPointDollars,
    MarketPosition, Order, OrderAction, OrderGroup, OrderQueuePosition, OrderSide, OrderStatus,
    OrderType, SelfTradePreventionType, Settlement, SubaccountBalance, SubaccountNettingConfig,
    SubaccountTransfer,
};

const PORTFOLIO_PATH: &str = "/portfolio";

impl<'a> Kalshi {
    /// Retrieves the current balance of the authenticated user from the Kalshi exchange.
    ///
    /// This method fetches the user's balance, requiring a valid authentication token.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Returns
    ///
    /// - `Ok(i64)`: The user's current balance on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let balance = kalshi_instance.get_balance().await.unwrap();
    /// ```
    ///
    pub async fn get_balance(&self) -> Result<i64, KalshiError> {
        let result: BalanceResponse = self.signed_get(&format!("{}/balance", PORTFOLIO_PATH)).await?;
        Ok(result.balance)
    }

    /// Retrieves a list of orders from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple orders, allowing for filtering by ticker, event ticker, time range,
    /// status, and pagination. A valid authentication token is required to access this information.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `ticker` - An optional string to filter orders by market ticker.
    /// * `event_ticker` - An optional string to filter orders by event ticker.
    /// * `min_ts` - An optional minimum timestamp for order creation time.
    /// * `max_ts` - An optional maximum timestamp for order creation time.
    /// * `status` - An optional string to filter orders by their status.
    /// * `limit` - An optional integer to limit the number of orders returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Order>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Order` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    /// Retrieves all possible orders (Will crash, need to limit for a successful request).
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let orders = kalshi_instance.get_orders(
    ///     Some("ticker_name"), None, None, None, None, None, None
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_orders(
        &self,
        ticker: Option<String>,
        event_ticker: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        status: Option<OrderStatus>,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Order>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7);

        add_param!(params, "ticker", ticker);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "event_ticker", event_ticker);
        add_param!(params, "status", status.map(|s| s.to_string()));

        let path = if params.is_empty() {
            format!("{}/orders", PORTFOLIO_PATH)
        } else {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}/orders?{}", PORTFOLIO_PATH, query_string)
        };

        let result: MultipleOrderResponse = self.signed_get(&path).await?;
        return Ok((result.cursor, result.orders));
    }

    /// Retrieves detailed information about a specific order from the Kalshi exchange.
    ///
    /// This method fetches data for a single order identified by its order ID. A valid authentication token
    /// is required to access this information. If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `order_id` - A reference to a string representing the order's unique identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Order)`: Detailed information about the specified order on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let order_id = "some_order_id";
    /// let order = kalshi_instance.get_single_order(&order_id).await.unwrap();
    /// ```
    ///
    pub async fn get_single_order(&self, order_id: &String) -> Result<Order, KalshiError> {
        let path = format!("{}/orders/{}", PORTFOLIO_PATH, order_id);
        let result: SingleOrderResponse = self.signed_get(&path).await?;
        return Ok(result.order);
    }

    /// Cancels an existing order on the Kalshi exchange.
    ///
    /// This method cancels an order specified by its ID. A valid authentication token is
    /// required to perform this action. If the user is not logged in or the token is missing,
    /// it returns an error.
    ///
    /// # Arguments
    ///
    /// * `order_id` - A string slice referencing the ID of the order to be canceled.
    ///
    /// # Returns
    ///
    /// - `Ok((Order, i32, String))`: A tuple containing the updated `Order` object after cancellation,
    ///   an integer indicating the amount by which the order was reduced, and a string representation
    ///   of the reduction in fixed-point format.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let order_id = "some_order_id";
    /// let (order, reduced_by, reduced_by_fp) = kalshi_instance.cancel_order(order_id).await.unwrap();
    /// ```
    ///
    pub async fn cancel_order(&self, order_id: &str) -> Result<(Order, i32, String), KalshiError> {
        let path = format!("{}/orders/{}", PORTFOLIO_PATH, order_id);
        let result: DeleteOrderResponse = self.signed_delete(&path).await?;
        Ok((result.order, result.reduced_by, result.reduced_by_fp))
    }
    /// Decreases the size of an existing order on the Kalshi exchange.
    ///
    /// **Endpoint:**  
    /// `POST /portfolio/orders/{order_id}/decrease` (v2)
    ///
    /// This method allows reducing the size of an order either by specifying the amount to reduce
    /// (`reduce_by`) or setting a new target size (`reduce_to`). A valid authentication token is
    /// required for this operation. It's important to provide either `reduce_by` or `reduce_to`,
    /// but not both at the same time.
    ///
    /// # Arguments
    ///
    /// * `order_id` - A string slice referencing the ID of the order to be decreased.
    /// * `reduce_by` - An optional integer specifying how much to reduce the order by.
    /// * `reduce_to` - An optional integer specifying the new size of the order.
    ///
    /// # Returns
    ///
    /// - `Ok(Order)`: The updated `Order` object after decreasing the size.
    /// - `Err(KalshiError)`: An error if the user is not authenticated, if both `reduce_by` and `reduce_to` are provided,
    ///   or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```rust
    /// // shrink order ABC123 by 5 contracts
    /// let order = kalshi_instance
    ///     .decrease_order("ABC123", Some(5), None)
    ///     .await?;
    /// ```
    ///
    pub async fn decrease_order(&self, order_id: &str, req: DecreaseOrderRequest) -> Result<Order, KalshiError> {
        let path = format!("{}/orders/{}/decrease", PORTFOLIO_PATH, order_id);
        let result: DecreaseOrderResponse = self.signed_post(&path, &req).await?;
        Ok(result.order)
    }

    /// Retrieves a list of fills from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple fills, allowing for filtering by ticker, order ID, time range,
    /// and pagination. A valid authentication token is required to access this information.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `ticker` - An optional string to filter fills by market ticker.
    /// * `order_id` - An optional string to filter fills by order ID.
    /// * `min_ts` - An optional minimum timestamp for fill creation time.
    /// * `max_ts` - An optional maximum timestamp for fill creation time.
    /// * `limit` - An optional integer to limit the number of fills returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Fill>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Fill` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    /// Retrieves all filled orders
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let fills = kalshi_instance.get_fills(
    ///     Some("ticker_name"), None, None, None, None, None
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_fills(
        &self,
        ticker: Option<String>,
        order_id: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Fill>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7);

        add_param!(params, "ticker", ticker);
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "min_ts", min_ts);
        add_param!(params, "max_ts", max_ts);
        add_param!(params, "order_id", order_id);

        let path = if params.is_empty() {
            format!("{}/fills", PORTFOLIO_PATH)
        } else {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}/fills?{}", PORTFOLIO_PATH, query_string)
        };

        let result: MultipleFillsResponse = self.signed_get(&path).await?;
        return Ok((result.cursor, result.fills));
    }

    /// Retrieves a list of portfolio settlements from the Kalshi exchange.
    ///
    /// This method fetches settlements in the user's portfolio, with options for pagination using limit and cursor.
    /// A valid authentication token is required to access this information.
    /// If the user is not logged in or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of settlements returned.
    /// * `cursor` - An optional string for pagination cursor.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Settlement>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Settlement` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let settlements = kalshi_instance.get_settlements(None, None).await.unwrap();
    /// ```
    pub async fn get_settlements(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<Settlement>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6);

        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let path = if params.is_empty() {
            format!("{}/settlements", PORTFOLIO_PATH)
        } else {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}/settlements?{}", PORTFOLIO_PATH, query_string)
        };

        let result: PortfolioSettlementResponse = self.signed_get(&path).await?;
        Ok((result.cursor, result.settlements))
    }

    /// Retrieves the user's positions in events and markets from the Kalshi exchange.
    ///
    /// This method fetches the user's positions, providing options for filtering by settlement status,
    /// specific ticker, and event ticker, as well as pagination using limit and cursor. A valid
    /// authentication token is required to access this information. If the user is not logged in
    /// or the token is missing, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of positions returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `settlement_status` - An optional string to filter positions by their settlement status.
    /// * `ticker` - An optional string to filter positions by market ticker.
    /// * `event_ticker` - An optional string to filter positions by event ticker.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<EventPosition>, Vec<MarketPosition>))`: A tuple containing an optional pagination cursor,
    ///   a vector of `EventPosition` objects, and a vector of `MarketPosition` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let positions = kalshi_instance.get_positions(None, None, None, None, None).await.unwrap();
    /// ```
    ///
    /// Retrieves the user's positions in events and markets from the Kalshi exchange using query parameters.
    ///
    /// # Query Parameters
    /// - `cursor`: Option<String>
    ///     - The Cursor represents a pointer to the next page of records in the pagination.
    ///       Use the value returned from the previous response to get the next page.
    /// - `limit`: Option<i32>
    ///     - Parameter to specify the number of results per page. Defaults to 100.
    ///       Required range: 1 <= x <= 1000.
    /// - `count_filter`: Option<String>
    ///     - Restricts the positions to those with any of following fields with non-zero values,
    ///       as a comma separated list. The following values are accepted: `position`, `total_traded`.
    /// - `ticker`: Option<String>
    ///     - Filter by market ticker.
    /// - `event_ticker`: Option<String>
    ///     - Event ticker of desired positions. Multiple event tickers can be provided as a comma-separated list (maximum 10).
    pub async fn get_positions(
        &self,
        limit: Option<i32>,
        cursor: Option<String>,
        count_filter: Option<String>,
        ticker: Option<String>,
        event_ticker: Option<String>,
    ) -> Result<(Option<String>, Vec<EventPosition>, Vec<MarketPosition>), KalshiError> {
        let mut all_event_positions = Vec::new();
        let mut all_market_positions = Vec::new();
        let mut current_cursor = cursor;
        // Use provided limit for each page, or default to 1000 for maximum efficiency
        let page_limit = limit.unwrap_or(1000);

        loop {
            
            let mut params: Vec<(&str, String)> = Vec::with_capacity(6);
            add_param!(params, "limit", Some(page_limit));
            add_param!(params, "cursor", current_cursor);
            add_param!(params, "count_filter", count_filter);
            add_param!(params, "ticker", ticker);
            add_param!(params, "event_ticker", event_ticker);

            let query_string = if params.is_empty() {
                String::new()
            } else {
                let qs = params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                format!("?{}", qs)
            };

            let path = format!("{}/positions{}", PORTFOLIO_PATH, query_string);

            let result: GetPositionsResponse = self.signed_get(&path).await?;
            
            all_event_positions.extend(result.event_positions);
            all_market_positions.extend(result.market_positions);

            current_cursor = result.cursor;
            
            // If there's no cursor or it's empty, we've collected all pages.
            if current_cursor.is_none() || current_cursor.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
                break;
            }
            
            // If a specific limit was requested and we have enough market positions, we can stop.
            if let Some(l) = limit {
                if all_market_positions.len() >= l as usize {
                    break;
                }
            }
        }

        Ok((
            None, // Return None for cursor since we've exhausted it
            all_event_positions,
            all_market_positions,
        ))
    }

    /// Submits an order to the Kalshi exchange.
    ///
    /// This method allows placing an order in the market, requiring details such as action, count, side,
    /// ticker, order type, and other optional parameters. A valid authentication token is
    /// required for this operation. Note that for limit orders, either `no_price` or `yes_price` must be provided,
    /// but not both.
    ///
    /// # Arguments
    ///
    /// * `action` - The action (buy/sell) of the order.
    /// * `client_order_id` - An optional client-side identifier for the order.
    /// * `count` - The number of shares or contracts to trade.
    /// * `side` - The side (Yes/No) of the order.
    /// * `ticker` - The market ticker the order is placed in.
    /// * `input_type` - The type of the order (e.g., market, limit).
    /// * `buy_max_cost` - The maximum cost for a buy order. Optional.
    /// * `expiration_ts` - The expiration timestamp for the order. Optional.
    /// * `no_price` - The price for the 'No' option in a limit order. Optional.
    /// * `sell_position_floor` - The minimum position size to maintain after selling. Optional.
    /// * `yes_price` - The price for the 'Yes' option in a limit order. Optional.
    ///
    /// # Returns
    ///
    /// - `Ok(Order)`: The created `Order` object on successful placement.
    /// - `Err(KalshiError)`: An error if the user is not authenticated, if both `no_price` and `yes_price` are provided for limit orders,
    ///   or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let action = Action::Buy;
    /// let side = Side::Yes;
    /// let order = kalshi_instance.create_order(
    ///     action,
    ///     None,
    ///     10,
    ///     side,
    ///     "example_ticker",
    ///     OrderType::Limit,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     Some(100)
    /// ).await.unwrap();
    /// ```
    ///
    
    pub async fn create_order(&self, req: CreateOrderRequest) -> Result<Order, KalshiError> {
        let path = format!("{}/orders", PORTFOLIO_PATH);
        let result: SingleOrderResponse = self.signed_post(&path, &req).await?;
        Ok(result.order)
    }

    // -----------------------------------------------------------------
    // BATCH-CREATE  (POST  /portfolio/orders/batched)
    // -----------------------------------------------------------------
    pub async fn batch_create_order(
        &self,
        batch: Vec<CreateOrderRequest>,
    ) -> Result<Vec<Result<Order, KalshiError>>, KalshiError> {
        if batch.is_empty() {
            return Ok(Vec::new());
        }
        if batch.len() > 20 {
            return Err(KalshiError::UserInputError(
                "Batch size exceeds 20; split the request".into(),
            ));
        }

        let path = format!("{}/orders/batched", PORTFOLIO_PATH);
        let body = BatchCreateOrderPayload { orders: batch };
        let response: BatchCreateOrdersResponse = self.signed_post(&path, &body).await?;

        let mut out = Vec::with_capacity(response.orders.len());
        for item in response.orders {
            match (item.order, item.error) {
                (Some(order), None) => out.push(Ok(order)),
                (_, Some(err)) => out.push(Err(KalshiError::UserInputError(
                    err.message.unwrap_or_else(|| "unknown error".into()),
                ))),
                _ => out.push(Err(KalshiError::InternalError(
                    "malformed batch-create response".into(),
                ))),
            }
        }
        Ok(out)
    }

    // -----------------------------------------------------------------
    // BATCH-CANCEL (DELETE /portfolio/orders/batched)
    // -----------------------------------------------------------------
    pub async fn batch_cancel_order(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<Result<(Order, i32, String), KalshiError>>, KalshiError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        if ids.len() > 20 {
            return Err(KalshiError::UserInputError(
                "Batch size exceeds 20; split the request".into(),
            ));
        }

        let path = format!("{}/orders/batched", PORTFOLIO_PATH);
        let body = BatchCancelOrderPayload { ids };

        let response: BatchCancelOrdersResponse = self.signed_delete_with_body(&path, &body).await?;

        let mut out = Vec::with_capacity(response.orders.len());
        for item in response.orders {
            match (item.order, item.reduced_by, item.reduced_by_fp, item.error) {
                (Some(order), Some(reduced_by), Some(reduced_by_fp), None) => out.push(Ok((order, reduced_by, reduced_by_fp))),
                (_, _, _, Some(err)) => out.push(Err(KalshiError::UserInputError(
                    err.message.unwrap_or_else(|| "unknown error".into()),
                ))),
                _ => out.push(Err(KalshiError::InternalError(
                    "malformed batch-cancel response".into(),
                ))),
            }
        }
        Ok(out)
    }

    /// Retrieves the total value of all resting orders for the authenticated user.
    ///
    /// This endpoint is primarily intended for use by FCM members.
    ///
    /// # Returns
    ///
    /// - `Ok(i64)`: The total resting order value in cents on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let total_value = kalshi_instance.get_total_resting_order_value().await.unwrap();
    /// println!("Total resting order value: {} cents", total_value);
    /// ```
    ///
    /// # Note
    ///
    /// If you're uncertain about this endpoint, it likely does not apply to you.
    ///
    pub async fn get_total_resting_order_value(&self) -> Result<i64, KalshiError> {
        let path = "/portfolio/summary/total_resting_order_value";
        let res: TotalRestingOrderValueResponse = self.signed_get(path).await?;
        Ok(res.total_resting_order_value)
    }

    /// Retrieves all order groups for the authenticated user.
    ///
    /// Order groups allow you to manage multiple related orders together.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<OrderGroup>)`: A vector of order groups on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let order_groups = kalshi_instance.get_order_groups().await.unwrap();
    /// ```
    ///
    pub async fn get_order_groups(&self) -> Result<Vec<OrderGroup>, KalshiError> {
        let path = "/portfolio/order_groups";
        let res: OrderGroupsResponse = self.signed_get(path).await?;
        Ok(res.order_groups)
    }

    /// Creates a new order group.
    ///
    /// Order groups allow you to manage multiple related orders with shared limits.
    ///
    /// # Arguments
    ///
    /// * `contracts_limit` - The maximum number of contracts allowed across all orders in this group.
    ///
    /// # Returns
    ///
    /// - `Ok(OrderGroup)`: The created order group on successful creation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let order_group = kalshi_instance.create_order_group(100).await.unwrap();
    /// ```
    ///
    pub async fn create_order_group(&self, contracts_limit: i32) -> Result<OrderGroup, KalshiError> {
        let path = "/portfolio/order_groups/create";
        let body = CreateOrderGroupRequest { contracts_limit };
        self.signed_post(path, &body).await
    }

    /// Retrieves a specific order group by ID.
    ///
    /// # Arguments
    ///
    /// * `order_group_id` - The UUID of the order group to retrieve.
    ///
    /// # Returns
    ///
    /// - `Ok(OrderGroup)`: The order group details on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let order_group = kalshi_instance.get_order_group("group-uuid").await.unwrap();
    /// ```
    ///
    pub async fn get_order_group(&self, order_group_id: &str) -> Result<OrderGroup, KalshiError> {
        let path = format!("/portfolio/order_groups/{}", order_group_id);
        let res: OrderGroupResponse = self.signed_get(&path).await?;
        Ok(res.order_group)
    }

    /// Deletes an order group.
    ///
    /// This will remove the order group but not cancel the orders within it.
    ///
    /// # Arguments
    ///
    /// * `order_group_id` - The UUID of the order group to delete.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: Success confirmation.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// kalshi_instance.delete_order_group("group-uuid").await.unwrap();
    /// ```
    ///
    pub async fn delete_order_group(&self, order_group_id: &str) -> Result<(), KalshiError> {
        let path = format!("/portfolio/order_groups/{}", order_group_id);
        let _res: DeleteOrderGroupResponse = self.signed_delete(&path).await?;
        Ok(())
    }

    /// Resets an order group, canceling all orders within it.
    ///
    /// # Arguments
    ///
    /// * `order_group_id` - The UUID of the order group to reset.
    ///
    /// # Returns
    ///
    /// - `Ok(OrderGroup)`: The reset order group on successful reset.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let order_group = kalshi_instance.reset_order_group("group-uuid").await.unwrap();
    /// ```
    ///
    pub async fn reset_order_group(&self, order_group_id: &str) -> Result<OrderGroup, KalshiError> {
        let path = format!("/portfolio/order_groups/{}/reset", order_group_id);
        self.signed_put(&path, None::<&()>).await
    }

    /// Retrieves queue positions for multiple orders.
    ///
    /// This method provides information about where your orders are positioned
    /// in the order book queue, helping you understand order priority.
    ///
    /// # Arguments
    ///
    /// * `order_ids` - A vector of order IDs to get queue positions for.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<OrderQueuePosition>)`: A vector of queue positions on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let order_ids = vec!["order-1".to_string(), "order-2".to_string()];
    /// let positions = kalshi_instance.get_queue_positions(order_ids).await.unwrap();
    /// ```
    ///
    pub async fn get_queue_positions(
        &self,
        order_ids: Vec<String>,
    ) -> Result<Vec<OrderQueuePosition>, KalshiError> {
        let path = "/portfolio/orders/queue_positions";
        let mut params = vec![];
        
        // Add each order_id as a separate query parameter
        for id in order_ids {
            params.push(("order_ids".to_string(), id));
        }

        let url = format!("{}{}", self.base_url, path);
        let final_url = reqwest::Url::parse_with_params(&url, &params)?;
        let res: QueuePositionsResponse = self.client.get(final_url).send().await?.json().await?;
        Ok(res.queue_positions)
    }

    /// Amends an existing order by modifying its price or quantity.
    ///
    /// This is an alternative to decrease_order that allows more flexibility.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order ID to amend.
    /// * `new_price` - Optional new price in cents.
    /// * `new_quantity` - Optional new quantity of contracts.
    ///
    /// # Returns
    ///
    /// - `Ok(Order)`: The amended order on successful modification.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let amended_order = kalshi_instance.amend_order(
    ///     "order-uuid",
    ///     Some(55),
    ///     Some(50)
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn amend_order(&self, order_id: &str, req: AmendOrderRequest) -> Result<Order, KalshiError> {
        let path = format!("/portfolio/orders/{}/amend", order_id);
        let res: SingleOrderResponse = self.signed_post(&path, &req).await?;
        Ok(res.order)
    }

    /// Retrieves the queue position for a single order.
    ///
    /// This method provides information about where a specific order is positioned
    /// in the order book queue.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order ID to get queue position for.
    ///
    /// # Returns
    ///
    /// - `Ok(OrderQueuePosition)`: The queue position on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let position = kalshi_instance.get_order_queue_position("order-uuid").await.unwrap();
    /// println!("Order is at position {} in queue", position.queue_position);
    /// ```
    ///
    pub async fn get_order_queue_position(&self, order_id: &str) -> Result<OrderQueuePosition, KalshiError> {
        let path = format!("/portfolio/orders/{}/queue_position", order_id);
        self.signed_get(&path).await
    }

    /// Retrieves queue positions for all resting orders, optionally filtered.
    ///
    /// # Arguments
    /// * `market_tickers` - Optional comma-separated list of market tickers to filter by.
    /// * `event_ticker` - Optional event ticker to filter by.
    pub async fn get_order_queue_positions(
        &self,
        market_tickers: Option<String>,
        event_ticker: Option<String>,
    ) -> Result<Vec<OrderQueuePosition>, KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        add_param!(params, "market_tickers", market_tickers);
        add_param!(params, "event_ticker", event_ticker);

        let path = if params.is_empty() {
            format!("{}/orders/queue_positions", PORTFOLIO_PATH)
        } else {
            let qs = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
            format!("{}/orders/queue_positions?{}", PORTFOLIO_PATH, qs)
        };

        let res: QueuePositionsResponse = self.signed_get(&path).await?;
        Ok(res.queue_positions)
    }

    /// Updates the contracts limit on an order group.
    ///
    /// If the new limit would immediately trigger the group, all orders are cancelled.
    pub async fn update_order_group_limit(
        &self,
        order_group_id: &str,
        contracts_limit: i32,
    ) -> Result<(), KalshiError> {
        let path = format!("{}/order_groups/{}/limit", PORTFOLIO_PATH, order_group_id);
        let body = UpdateOrderGroupLimitRequest { contracts_limit };
        let _: serde_json::Value = self.signed_put(&path, Some(&body)).await?;
        Ok(())
    }

    /// Triggers an order group, cancelling all orders in it until reset.
    pub async fn trigger_order_group(&self, order_group_id: &str) -> Result<(), KalshiError> {
        let path = format!("{}/order_groups/{}/trigger", PORTFOLIO_PATH, order_group_id);
        let _: serde_json::Value = self.signed_put::<(), serde_json::Value>(&path, None).await?;
        Ok(())
    }

    // -------- Subaccount endpoints --------

    /// Creates a new subaccount. Subaccounts are numbered 1–32.
    pub async fn create_subaccount(&self) -> Result<CreateSubaccountResponse, KalshiError> {
        let path = format!("{}/subaccounts", PORTFOLIO_PATH);
        self.signed_post(&path, &()).await
    }

    /// Retrieves balances for all subaccounts including the primary.
    pub async fn get_subaccount_balances(&self) -> Result<Vec<SubaccountBalance>, KalshiError> {
        let path = format!("{}/subaccounts/balances", PORTFOLIO_PATH);
        let res: SubaccountBalancesResponse = self.signed_get(&path).await?;
        Ok(res.subaccount_balances)
    }

    /// Retrieves netting configuration for all subaccounts.
    pub async fn get_subaccount_netting(&self) -> Result<Vec<SubaccountNettingConfig>, KalshiError> {
        let path = format!("{}/subaccounts/netting", PORTFOLIO_PATH);
        let res: SubaccountNettingResponse = self.signed_get(&path).await?;
        Ok(res.subaccount_netting_configs)
    }

    /// Updates the netting setting for a specific subaccount.
    ///
    /// Use `subaccount_number = 0` for the primary account, 1–32 for subaccounts.
    pub async fn update_subaccount_netting(
        &self,
        subaccount_number: i64,
        enabled: bool,
    ) -> Result<(), KalshiError> {
        let path = format!("{}/subaccounts/netting", PORTFOLIO_PATH);
        let body = UpdateSubaccountNettingRequest { subaccount_number, enabled };
        let _: serde_json::Value = self.signed_put(&path, Some(&body)).await?;
        Ok(())
    }

    /// Transfers funds between subaccounts.
    ///
    /// Use `0` for primary account, `1`–`32` for numbered subaccounts.
    pub async fn apply_subaccount_transfer(
        &self,
        amount_cents: i64,
        from_subaccount: i64,
        to_subaccount: i64,
        client_transfer_id: Option<String>,
    ) -> Result<i64, KalshiError> {
        let path = format!("{}/subaccounts/transfer", PORTFOLIO_PATH);
        let body = ApplySubaccountTransferRequest {
            amount_cents,
            from_subaccount,
            to_subaccount,
            client_transfer_id,
        };
        let res: SubaccountTransferResponse = self.signed_post(&path, &body).await?;
        Ok(res.balance)
    }

    /// Retrieves a paginated list of all subaccount transfers.
    pub async fn get_subaccount_transfers(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Option<String>, Vec<SubaccountTransfer>), KalshiError> {
        let mut params: Vec<(&str, String)> = Vec::new();
        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);

        let path = if params.is_empty() {
            format!("{}/subaccounts/transfers", PORTFOLIO_PATH)
        } else {
            let qs = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
            format!("{}/subaccounts/transfers?{}", PORTFOLIO_PATH, qs)
        };

        let res: SubaccountTransfersListResponse = self.signed_get(&path).await?;
        Ok((
            if res.cursor.is_empty() { None } else { Some(res.cursor) },
            res.transfers,
        ))
    }
}

// PRIVATE STRUCTS
// used in getbalance method
#[derive(Debug, Serialize, Deserialize)]
struct BalanceResponse {
    balance: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct SingleOrderResponse {
    order: Order,
}

#[derive(Debug, Deserialize, Serialize)]
struct MultipleOrderResponse {
    orders: Vec<Order>,
    #[serde(deserialize_with = "empty_string_is_none")]
    cursor: Option<String>,
}

fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteOrderResponse {
    order: Order,
    reduced_by: i32,
    reduced_by_fp: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DecreaseOrderResponse {
    order: Order,
}

#[derive(Debug, Deserialize, Serialize)]
struct MultipleFillsResponse {
    fills: Vec<Fill>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PortfolioSettlementResponse {
    cursor: Option<String>,
    settlements: Vec<Settlement>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetPositionsResponse {
    cursor: Option<String>,
    event_positions: Vec<EventPosition>,
    market_positions: Vec<MarketPosition>,
}

/// Payload for POST /portfolio/orders/batched
#[derive(Debug, Serialize)]
struct BatchCreateOrderPayload {
    orders: Vec<CreateOrderRequest>,
}

/// Payload for DELETE /portfolio/orders/batched
#[derive(Debug, Serialize, Deserialize)]
struct BatchCancelOrderPayload {
    ids: Vec<String>,
}

/// One element in the `orders` array that the batch-create endpoint returns.
#[derive(Debug, Serialize, Deserialize)]
struct ApiError {
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCreateOrderResponseItem {
    order: Option<Order>,
    error: Option<ApiError>,
}

/// One element in the `orders` array that the batch-cancel endpoint returns.
#[derive(Debug, Serialize, Deserialize)]
struct BatchCancelOrderResponseItem {
    order: Option<Order>,
    reduced_by: Option<i32>,
    reduced_by_fp: Option<String>,
    error: Option<ApiError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCreateOrdersResponse {
    orders: Vec<BatchCreateOrderResponseItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BatchCancelOrdersResponse {
    orders: Vec<BatchCancelOrderResponseItem>,
}

// -------- Private response/request wrappers --------

#[derive(Debug, Deserialize)]
struct TotalRestingOrderValueResponse {
    total_resting_order_value: i64,
}

#[derive(Debug, Serialize)]
struct CreateOrderGroupRequest {
    contracts_limit: i32,
}

#[derive(Debug, Deserialize)]
struct OrderGroupsResponse {
    order_groups: Vec<OrderGroup>,
}

#[derive(Debug, Deserialize)]
struct OrderGroupResponse {
    order_group: OrderGroup,
}

#[derive(Debug, Deserialize)]
struct DeleteOrderGroupResponse {}

#[derive(Debug, Deserialize)]
struct QueuePositionsResponse {
    queue_positions: Vec<OrderQueuePosition>,
}

#[derive(Debug, Serialize)]
struct UpdateOrderGroupLimitRequest {
    contracts_limit: i32,
}

#[derive(Debug, Serialize)]
struct ApplySubaccountTransferRequest {
    amount_cents: i64,
    from_subaccount: i64,
    to_subaccount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_transfer_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SubaccountTransferResponse {
    balance: i64,
}

#[derive(Debug, Serialize)]
struct UpdateSubaccountNettingRequest {
    subaccount_number: i64,
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct SubaccountBalancesResponse {
    subaccount_balances: Vec<SubaccountBalance>,
}

#[derive(Debug, Deserialize)]
struct SubaccountNettingResponse {
    subaccount_netting_configs: Vec<SubaccountNettingConfig>,
}

#[derive(Debug, Deserialize)]
struct SubaccountTransfersListResponse {
    #[serde(default)]
    cursor: String,
    transfers: Vec<SubaccountTransfer>,
}

#[cfg(test)]
mod test {
    use crate::portfolio::MultipleOrderResponse;

    #[test]
    fn test_serialize_multiple_order_response() -> serde_json::Result<()> {
        let json = r#"{"orders":[],"cursor":""}"#;
        let result = serde_json::from_str::<MultipleOrderResponse>(json)?;
        assert!(result.orders.is_empty());
        assert!(result.cursor.is_none());
        Ok(())
    }
}
