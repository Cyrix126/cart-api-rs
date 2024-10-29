// @generated automatically by Diesel CLI.

diesel::table! {
    cart_lines (id) {
        id -> Int4,
        cart_id -> Int4,
        product_id -> Int4,
        qty -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    carts (id) {
        id -> Int4,
        user_cart_id -> Int4,
        user_id -> Int4,
        discount_id -> Nullable<Int4>,
    }
}

diesel::joinable!(cart_lines -> carts (cart_id));
diesel::allow_tables_to_appear_in_same_query!(cart_lines, carts,);
