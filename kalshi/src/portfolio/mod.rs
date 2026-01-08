use super::Kalshi;
use crate::kalshi_error::*;
use std::fmt;
use uuid::Uuid;

use serde::{Deserialize, Deserializer, Serialize};

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
    /// - `Ok((Order, i32))`: A tuple containing the updated `Order` object after cancellation
    ///   and an integer indicating the amount by which the order was reduced on successful cancellation.
    /// - `Err(KalshiError)`: An error if the user is not authenticated or if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an already authenticated instance of `Kalshi`
    /// let order_id = "some_order_id";
    /// let (order, reduced_by) = kalshi_instance.cancel_order(order_id).await.unwrap();
    /// ```
    ///
    pub async fn cancel_order(&self, order_id: &str) -> Result<(Order, i32), KalshiError> {
        let path = format!("{}/orders/{}", PORTFOLIO_PATH, order_id);
        let result: DeleteOrderResponse = self.signed_delete(&path).await?;
        Ok((result.order, result.reduced_by))
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
    pub async fn decrease_order(
        &self,
        order_id: &str,
        reduce_by: Option<i32>,
        reduce_to: Option<i32>,
    ) -> Result<Order, KalshiError> {
        match (reduce_by, reduce_to) {
            (Some(_), Some(_)) => {
                return Err(KalshiError::UserInputError(
                    "Can only provide reduce_by strict exclusive or reduce_to, can't provide both"
                        .to_string(),
                ));
            }
            (None, None) => {
                return Err(KalshiError::UserInputError(
                    "Must provide either reduce_by exclusive or reduce_to, can't provide neither"
                        .to_string(),
                ));
            }
            _ => {}
        }

        let decrease_payload = DecreaseOrderPayload {
            reduce_by: reduce_by,
            reduce_to: reduce_to,
        };

        // v2 portfolio API: POST /orders/{order_id}/decrease
        let path = format!("{}/orders/{}/decrease", PORTFOLIO_PATH, order_id);

        // response is now { "order": { … }, "reduced_by": int }
        let result: DecreaseOrderResponse = self.signed_post(&path, &decrease_payload).await?;
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
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6);

        add_param!(params, "limit", limit);
        add_param!(params, "cursor", cursor);
        add_param!(params, "count_filter", count_filter);
        add_param!(params, "ticker", ticker);
        add_param!(params, "event_ticker", event_ticker);

        let path = if params.is_empty() {
            format!("{}/positions", PORTFOLIO_PATH)
        } else {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}/positions?{}", PORTFOLIO_PATH, query_string)
        };

        let result: GetPositionsResponse = self.signed_get(&path).await?;

        Ok((
            result.cursor,
            result.event_positions,
            result.market_positions,
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
    
    // TODO: rewrite using generics
    pub async fn create_order(
        &self,
        action: Action,
        client_order_id: Option<String>,
        count: i32,
        side: Side,
        ticker: String,
        input_type: OrderType,
        buy_max_cost: Option<i64>,
        expiration_ts: Option<i64>,
        yes_price: Option<i64>,
        no_price: Option<i64>,
        sell_position_floor: Option<i32>,
        yes_price_dollars: Option<String>,
        no_price_dollars: Option<String>,
    ) -> Result<Order, KalshiError> {
        match input_type {
            OrderType::Limit => {
                // Check if user provided both cent and dollar prices for the same side
                if yes_price.is_some() && yes_price_dollars.is_some() {
                    return Err(KalshiError::UserInputError(
                        "Cannot provide both yes_price and yes_price_dollars".to_string(),
                    ));
                }
                if no_price.is_some() && no_price_dollars.is_some() {
                    return Err(KalshiError::UserInputError(
                        "Cannot provide both no_price and no_price_dollars".to_string(),
                    ));
                }
                
                // Check if any price is provided
                let has_price = yes_price.is_some() 
                    || no_price.is_some() 
                    || yes_price_dollars.is_some() 
                    || no_price_dollars.is_some();
                
                if !has_price {
                    return Err(KalshiError::UserInputError(
                        "Must provide a price (yes_price, no_price, yes_price_dollars, or no_price_dollars)".to_string(),
                    ));
                }
                
                // Check if both yes and no prices are provided
                let has_yes = yes_price.is_some() || yes_price_dollars.is_some();
                let has_no = no_price.is_some() || no_price_dollars.is_some();
                if has_yes && has_no {
                    return Err(KalshiError::UserInputError(
                        "Can only provide yes price or no price, not both".to_string(),
                    ));
                }
            },
            _ => {}
        }

        let unwrapped_id = match client_order_id {
            Some(id) => id,
            _ => String::from(Uuid::new_v4()),
        };

        let order_payload = CreateOrderPayload {
            action: action,
            client_order_id: unwrapped_id,
            count: count,
            side: side,
            ticker: ticker,
            r#type: input_type,
            buy_max_cost: buy_max_cost,
            expiration_ts: expiration_ts,
            yes_price: yes_price,
            no_price: no_price,
            sell_position_floor: sell_position_floor,
            yes_price_dollars: yes_price_dollars,
            no_price_dollars: no_price_dollars,
        };

        let path = format!("{}/orders", PORTFOLIO_PATH);
        let result: SingleOrderResponse = self.signed_post(&path, &order_payload).await?;
        Ok(result.order)
    }

    // -----------------------------------------------------------------
    // BATCH-CREATE  (POST  /portfolio/orders/batched)
    // -----------------------------------------------------------------
    pub async fn batch_create_order(
        &self,
        batch: Vec<OrderCreationField>,
    ) -> Result<Vec<Result<Order, KalshiError>>, KalshiError> {
        if batch.is_empty() {
            return Ok(Vec::new());
        }
        if batch.len() > 20 {
            return Err(KalshiError::UserInputError(
                "Batch size exceeds 20; split the request".into(),
            ));
        }

        // Convert the user-supplied OrderCreationField into raw payloads -----------------
        let orders: Vec<CreateOrderPayload> = batch
            .into_iter()
            .map(|field| {
                // unpack the helper struct
                let (
                    action,
                    client_order_id,
                    count,
                    side,
                    ticker,
                    input_type,
                    buy_max_cost,
                    expiration_ts,
                    yes_price,
                    no_price,
                    sell_position_floor,
                    yes_price_dollars,
                    no_price_dollars,
                ) = field.get_params();

                CreateOrderPayload {
                    action,
                    client_order_id: client_order_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
                    count,
                    side,
                    ticker,
                    r#type: input_type,
                    buy_max_cost,
                    expiration_ts,
                    yes_price,
                    no_price,
                    sell_position_floor,
                    yes_price_dollars,
                    no_price_dollars,
                }
            })
            .collect();

        let path = format!("{}/orders/batched", PORTFOLIO_PATH);
        let body = BatchCreateOrderPayload { orders };

        // NB: signed_post already injects auth headers & error mapping
        let response: BatchCreateOrdersResponse = self.signed_post(&path, &body).await?;

        // Convert the wire format into Vec<Result<…>>
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
    ) -> Result<Vec<Result<(Order, i32), KalshiError>>, KalshiError> {
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
            match (item.order, item.reduced_by, item.error) {
                (Some(order), Some(reduced_by), None) => out.push(Ok((order, reduced_by))),
                (_, _, Some(err)) => out.push(Err(KalshiError::UserInputError(
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
    pub async fn amend_order(
        &self,
        order_id: &str,
        new_price: Option<i32>,
        new_quantity: Option<i32>,
    ) -> Result<Order, KalshiError> {
        let path = format!("/portfolio/orders/{}/amend", order_id);
        let body = AmendOrderRequest {
            new_price,
            new_quantity,
        };
        let res: SingleOrderResponse = self.signed_post(&path, &body).await?;
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
}

#[derive(Debug, Deserialize, Serialize)]
struct DecreaseOrderResponse {
    order: Order,
}

#[derive(Debug, Deserialize, Serialize)]
struct DecreaseOrderPayload {
    reduce_by: Option<i32>,
    reduce_to: Option<i32>,
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

#[derive(Debug, Deserialize, Serialize)]
struct CreateOrderPayload {
    action: Action,
    client_order_id: String,
    count: i32,
    side: Side,
    ticker: String,
    r#type: OrderType,
    buy_max_cost: Option<i64>,
    expiration_ts: Option<i64>,
    yes_price: Option<i64>,
    no_price: Option<i64>,
    sell_position_floor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yes_price_dollars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_price_dollars: Option<String>,
}

// PUBLIC STRUCTS
// -------------------------

/// Represents an order in the Kalshi exchange.
///
/// This struct details an individual order, including its identification, status, prices, and various metrics related to its lifecycle.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    /// Unique identifier for the order.
    pub order_id: String,
    /// Identifier of the user who placed the order. Optional.
    #[serde(default)]
    pub user_id: Option<String>,
    /// Ticker of the market associated with the order.
    pub ticker: String,
    /// Current status of the order (e.g., resting, executed).
    pub status: OrderStatus,
    /// Price of the 'Yes' option in the order (cents).
    pub yes_price: i32,
    /// Price of the 'No' option in the order (cents).
    pub no_price: i32,

    /// Timestamp when the order was created. Optional.
    #[serde(default)]
    pub created_time: Option<String>,
    /// Last update time of the order. Optional.
    #[serde(default)]
    pub last_update_time: Option<String>,
    /// Expiration time of the order. Optional (often null).
    #[serde(default)]
    pub expiration_time: Option<String>,

    // === Counts / queue ===
    /// Total fills (Kalshi now reports a single `fill_count`).
    #[serde(default)]
    pub fill_count: Option<i32>,
    /// Initial order size.
    #[serde(default)]
    pub initial_count: Option<i32>,
    /// Remaining count of the order. Optional.
    #[serde(default)]
    pub remaining_count: Option<i32>,
    /// Position of the order in the queue. Optional.
    #[serde(default)]
    pub queue_position: Option<i32>,

    // === Legacy/optional counters kept for back-compat (often missing) ===
    #[serde(default)]
    pub taker_fill_count: Option<i32>,
    #[serde(default)]
    pub place_count: Option<i32>,
    #[serde(default)]
    pub decrease_count: Option<i32>,
    #[serde(default)]
    pub maker_fill_count: Option<i32>,
    #[serde(default)]
    pub fcc_cancel_count: Option<i32>,
    #[serde(default)]
    pub close_cancel_count: Option<i32>,

    // === Fees / costs ===
    /// Fees incurred as a taker (cents).
    #[serde(default)]
    pub taker_fees: Option<i32>,
    /// Taker fees in dollars (string, sometimes null).
    #[serde(default)]
    pub taker_fees_dollars: Option<String>,

    /// Total cost of taker fills (cents).
    #[serde(default)]
    pub taker_fill_cost: Option<i32>,
    /// Taker fill cost in dollars (string).
    #[serde(default)]
    pub taker_fill_cost_dollars: Option<String>,

    /// Maker fees (cents).
    #[serde(default)]
    pub maker_fees: Option<i32>,
    /// Maker fees in dollars (string, sometimes null).
    #[serde(default)]
    pub maker_fees_dollars: Option<String>,

    /// Total cost of maker fills (cents).
    #[serde(default)]
    pub maker_fill_cost: Option<i32>,
    /// Maker fill cost in dollars (string).
    #[serde(default)]
    pub maker_fill_cost_dollars: Option<String>,

    // === Price (dollar string facades Kalshi now sends) ===
    #[serde(default)]
    pub yes_price_dollars: Option<String>,
    #[serde(default)]
    pub no_price_dollars: Option<String>,

    // === Identifiers ===
    pub action: Action,
    pub side: Side,
    /// Type of the order (e.g., "limit").
    #[serde(rename = "type")]
    pub r#type: String,
    /// Client-side identifier for the order.
    pub client_order_id: String,
    /// Group identifier for the order (now nullable).
    #[serde(default)]
    pub order_group_id: Option<String>,

    // === Misc newly-seen ===
    /// Self-trade prevention type (nullable).
    #[serde(default)]
    pub self_trade_prevention_type: Option<String>,
}

/// A completed transaction (a 'fill') in the Kalshi exchange.
///
/// This struct details a single fill instance, including the action taken, the quantity,
/// the involved prices, and the identifiers of the order and trade.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Fill {
    /// The action (buy/sell) of the fill.
    pub action: Action,
    /// The number of contracts or shares involved in the fill.
    pub count: i32,
    /// The timestamp when the fill was created.
    pub created_time: String,
    /// Indicates if the fill was made by a taker.
    pub is_taker: bool,
    /// The price of the 'No' option in the fill.
    pub no_price: i64,
    /// The identifier of the associated order.
    pub order_id: String,
    /// The side (Yes/No) of the fill.
    pub side: Side,
    /// The ticker of the market in which the fill occurred.
    pub ticker: String,
    /// The unique identifier of the trade.
    pub trade_id: String,
    /// The price of the 'Yes' option in the fill.
    pub yes_price: i64,
}

/// A settlement of a market position in the Kalshi exchange.
///
/// This struct provides details of a market settlement, including the result, quantities,
/// costs involved, and the timestamp of settlement.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Settlement {
    /// The result of the market settlement.
    pub market_result: String,
    /// The quantity involved in the 'No' position.
    pub no_count: i64,
    /// The total cost associated with the 'No' position.
    pub no_total_cost: i64,
    /// The revenue generated from the settlement, in cents.
    pub revenue: i64,
    /// The timestamp when the settlement occurred.
    pub settled_time: String,
    /// The ticker of the market that was settled.
    pub ticker: String,
    /// The quantity involved in the 'Yes' position.
    pub yes_count: i64,
    /// The total cost associated with the 'Yes' position, in cents.
    pub yes_total_cost: i64,
}

/// A user's position in a specific event on the Kalshi exchange.
///
/// Details the user's exposure, costs, profits, and the number of resting orders related to a particular event.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct EventPosition {
    /// The total exposure amount in the event.
    pub event_exposure: i64,
    /// The ticker of the event.
    pub event_ticker: String,
    /// The total fees paid in the event in cents.
    pub fees_paid: i64,
    /// The realized profit or loss in the event in cents.
    pub realized_pnl: i64,
    /// The count of resting (active but unfilled) orders in the event.
    #[serde(default)]
    pub resting_order_count: Option<i32>,
    /// The total cost incurred in the event in cents.
    pub total_cost: i64,
}

/// A user's position in a specific market on the Kalshi exchange.
///
/// This struct includes details about the user's market position, including exposure, fees,
/// profits, and the number of resting orders.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketPosition {
    /// The total fees paid in the market in cents.
    pub fees_paid: i64,
    /// The total exposure amount in the market.
    pub market_exposure: i64,
    /// The current position of the user in the market.
    pub position: i32,
    /// The realized profit or loss in the market in cents.
    pub realized_pnl: i64,
    /// The count of resting orders in the market.
    #[serde(default)]
    pub resting_orders_count: Option<i32>,
    /// The ticker of the market.
    pub ticker: String,
    /// The total traded amount in the market.
    pub total_traded: i64,
}

/// Represents the necessary fields for creating an order in the Kalshi exchange.
///
/// This struct is used to encapsulate all the data needed to create a new order. It includes details about the order type,
/// the action being taken (buy/sell), the market ticker, and various other optional parameters that can be specified
/// to fine-tune the order according to the user's needs.
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderCreationField {
    /// The action (buy/sell) of the order.
    pub action: Action,
    /// Client-side identifier for the order. Optional.
    pub client_order_id: Option<String>,
    /// The number of contracts or shares involved in the order.
    pub count: i32,
    /// The side (Yes/No) of the order.
    pub side: Side,
    /// Ticker of the market associated with the order.
    pub ticker: String,
    /// Type of the order (e.g., market, limit).
    pub input_type: OrderType,
    /// The maximum cost the buyer is willing to incur for a 'buy' action. Optional.
    pub buy_max_cost: Option<i64>,
    /// Expiration time of the order. Optional.
    pub expiration_ts: Option<i64>,
    /// Price of the 'Yes' option in the order (in cents). Optional.
    pub yes_price: Option<i64>,
    /// Price of the 'No' option in the order (in cents). Optional.
    pub no_price: Option<i64>,
    /// The minimum position the seller is willing to hold after selling. Optional.
    pub sell_position_floor: Option<i32>,
    /// Price of the 'Yes' option in dollars (e.g., "0.5000"). Optional.
    pub yes_price_dollars: Option<String>,
    /// Price of the 'No' option in dollars (e.g., "0.5000"). Optional.
    pub no_price_dollars: Option<String>,
}

impl OrderParams for OrderCreationField {
    fn get_params(
        self,
    ) -> (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<String>,
        Option<String>,
    ) {
        (
            self.action,
            self.client_order_id,
            self.count,
            self.side,
            self.ticker,
            self.input_type,
            self.buy_max_cost,
            self.expiration_ts,
            self.yes_price,
            self.no_price,
            self.sell_position_floor,
            self.yes_price_dollars,
            self.no_price_dollars,
        )
    }
}

/// The side of a market position in the Kalshi exchange.
///
/// This enum is used to indicate whether a market position, order, or trade is associated with the 'Yes' or 'No' outcome of a market event.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    /// Represents a position, order, or trade associated with the 'Yes' outcome of a market event.
    Yes,
    /// Represents a position, order, or trade associated with the 'No' outcome of a market event.
    No,
}

/// This enum is used to specify the type of action a user wants to take in an order, either buying or selling.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    /// Represents a buy action.
    Buy,
    /// Represents a sell action.
    Sell,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Buy => write!(f, "buy"),
            Action::Sell => write!(f, "sell"),
        }
    }
}

/// The status of an order in the Kalshi exchange.
///
/// This enum categorizes an order's lifecycle state, from creation to completion or cancellation.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// The order is active but not yet filled or partially filled and still in the order book.
    Resting,
    /// The order has been canceled and is no longer active.
    Canceled,
    /// The order has been fully executed.
    Executed,
    /// The order has been created and is awaiting further processing.
    Pending,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Resting => write!(f, "resting"),
            OrderStatus::Canceled => write!(f, "cancelled"),
            OrderStatus::Executed => write!(f, "executed"),
            OrderStatus::Pending => write!(f, "pending"),
        }
    }
}

/// Defines the type of an order in the Kalshi exchange.
///
/// This enum is used to specify the nature of the order, particularly how it interacts with the market.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// A market order is executed immediately at the current market price.
    Market,
    /// A limit order is set to be executed at a specific price or better.
    Limit,
}

trait OrderParams {
    fn get_params(
        self,
    ) -> (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<String>,
        Option<String>,
    );
}

impl OrderParams
    for (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<String>,
        Option<String>,
    )
{
    fn get_params(
        self,
    ) -> (
        Action,
        Option<String>,
        i32,
        Side,
        String,
        OrderType,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i64>,
        Option<i32>,
        Option<String>,
        Option<String>,
    ) {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12,
        )
    }
}

/// Payload for POST /portfolio/orders/batched
#[derive(Debug, Serialize, Deserialize)]
struct BatchCreateOrderPayload {
    orders: Vec<CreateOrderPayload>,
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

// -------- New Portfolio Endpoints Structs --------

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

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderGroup {
    pub id: String,
    pub contracts_limit: i32,
    pub total_contracts: Option<i32>,
    pub order_ids: Vec<String>,
    pub created_time: String,
}

#[derive(Debug, Deserialize)]
struct QueuePositionsResponse {
    queue_positions: Vec<OrderQueuePosition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderQueuePosition {
    pub order_id: String,
    pub queue_position: Option<i64>,
    pub total_queue_depth: Option<i64>,
}

#[derive(Debug, Serialize)]
struct AmendOrderRequest {
    new_price: Option<i32>,
    new_quantity: Option<i32>,
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
