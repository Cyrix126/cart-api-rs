use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::{error::AppError, models::Cart, AppState};
use crate::{models::CartLine, schema::carts::dsl::carts};
use diesel::prelude::*;

/// check if user exist
/// check if product exist
/// check if discount exist but without errors
pub async fn create_cart(
    Path(user): Path<u32>,
    State(state): State<AppState>,
    Json(data): Json<cart_common::Cart>,
) -> Result<impl IntoResponse, AppError> {
    let discount_id = check_newcart_validity(user, &data, &state).await?;
    // construct the cart
    let cart = Cart {
        id: 0,           // primary key
        user_cart_id: 0, // incremented by db based on user_id
        user_id: user as i32,
        discount_id,
    };
    // insert the cart and return the id of cart to insert lines and user cart id to return it.
    let conn = state.pool.get().await?;
    let user_cart_id = conn
        .interact(move |conn| {
            let (cart_id, user_cart_id) = diesel::insert_into(carts)
                .values(cart)
                .returning((crate::schema::carts::id, crate::schema::carts::user_cart_id))
                .get_result(conn)?;
            // insert lines
            let mut lines = vec![];
            for line in data.lines {
                lines.push(crate::models::CartLine {
                    id: 0, // primary key
                    cart_id,
                    product_id: line.product_id as i32,
                    qty: line.qty as i32,
                    created_at: None,
                    updated_at: None,
                });
            }
            diesel::insert_into(crate::schema::cart_lines::table)
                .values(lines)
                .execute(conn)?;
            Ok::<i32, AppError>(user_cart_id)
        })
        .await??;
    Ok(user_cart_id.to_string())
}
pub async fn read_cart(
    State(state): State<AppState>,
    Path(user): Path<u32>,
    Path(cart): Path<u32>,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.pool.get().await?;
    let (cart_row, lines) = conn
        .interact(move |conn| {
            // get cart + lines
            // carts
            let cart_row: Cart = carts
                .filter(
                    crate::schema::carts::user_id
                        .eq(user as i32)
                        .and(crate::schema::carts::user_cart_id.eq(cart as i32)),
                )
                .select(Cart::as_select())
                .first(conn)
                .unwrap();
            let lines = CartLine::belonging_to(&cart_row)
                .select(CartLine::as_select())
                .load(conn)
                .unwrap();
            Ok::<(Cart, Vec<CartLine>), AppError>((cart_row, lines))
        })
        .await??;
    let mut cart_lines = vec![];
    for line in lines {
        cart_lines.push(cart_common::CartLine {
            product_id: line.product_id as u32,
            qty: line.qty as u32,
        })
    }
    // get the applied code
    Ok(Json(cart_common::Cart {
        lines: cart_lines,
        discount_code: if let Some(code_id) = cart_row.discount_id {
            Some(
                state
                    .client_discount
                    .read_discount(code_id as u32)
                    .await?
                    .code,
            )
        } else {
            None
        },
    }))
}
/// this handler will remove any lines present for this cart and replace them.
pub async fn update_cart(
    State(state): State<AppState>,
    Path(user): Path<u32>,
    Path(cart): Path<u32>,
    Json(data): Json<cart_common::Cart>,
) -> Result<impl IntoResponse, AppError> {
    let discount_id = check_newcart_validity(user, &data, &state).await?;
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        // only thing that can change is the discount id for cards table.
        diesel::update(carts)
            .filter(
                crate::schema::carts::user_id
                    .eq(user as i32)
                    .and(crate::schema::carts::user_cart_id.eq(cart as i32)),
            )
            .set(crate::schema::carts::discount_id.eq(discount_id))
            .execute(conn)?;
        // insert lines, replacing existing ones.
        let mut lines = vec![];
        for line in data.lines {
            lines.push(crate::models::CartLine {
                id: 0, // primary key
                cart_id: cart as i32,
                product_id: line.product_id as i32,
                qty: line.qty as i32,
                created_at: None,
                updated_at: None,
            });
        }
        diesel::delete(crate::schema::cart_lines::table)
            .filter(crate::schema::cart_lines::cart_id.eq(cart as i32))
            .execute(conn)?;
        diesel::insert_into(crate::schema::cart_lines::table)
            .values(&lines)
            .execute(conn)?;
        Ok::<(), AppError>(())
    })
    .await??;
    Ok(())
}
pub async fn delete_cart(
    Path(user): Path<u32>,
    Path(cart): Path<u32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    use crate::schema::cart_lines as sc_cart_lines;
    use crate::schema::carts as sc_carts;
    let conn = state.pool.get().await?;
    conn.interact(move |conn| {
        // delete cart and return id
        let prim_key: i32 = diesel::delete(sc_carts::table)
            .filter(
                sc_carts::user_cart_id
                    .eq(cart as i32)
                    .and(sc_carts::user_id.eq(user as i32)),
            )
            .returning(crate::schema::carts::id)
            .get_result(conn)?;
        // delete lines
        diesel::delete(sc_cart_lines::table)
            .filter(sc_cart_lines::cart_id.eq(prim_key))
            .execute(conn)?;
        Ok::<(), AppError>(())
    })
    .await??;
    Ok(())
}

async fn check_newcart_validity(
    user_id: u32,
    cart: &cart_common::Cart,
    state: &AppState,
) -> Result<Option<i32>, AppError> {
    // Return error if user does not exist
    state.client_customer.get_data_from_id(user_id).await?;
    // check if discount code is valid and return the id if it is
    let mut discount_id = None;
    if let Some(code) = &cart.discount_code {
        if let Ok(discount) = state.client_discount.read_discount_by_code(code).await {
            if !discount.is_time_valid() {
                return Err(AppError::DiscountPeriod);
            }
            discount_id = Some(discount.id)
        }
    }
    // check if product exist
    // possible to make it parallel ?
    for line in cart.lines.iter() {
        state
            .client_product
            .get_product_from_id(line.product_id)
            .await?;
    }
    Ok(discount_id)
}
