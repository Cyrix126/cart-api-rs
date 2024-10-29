# OBJECTIVES

First to pay is first to get. No decrementation until an order has been made.


Very simple discount management.
Only one code can be applied per cart. Discount are managed by another API.

When the Cart API receives a discount code to apply to a cart, it will check the discount API and get the code from there or respond with an error if the code is not valid.

The client interface can itself check if a product does have a stock sufficient and throw error. The cart can still be saved with any quantity.
