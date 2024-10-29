 CREATE TABLE carts (
   id SERIAL PRIMARY KEY,
   user_cart_id SERIAL NOT NULL,
   user_id SERIAL NOT NULL,
   discount_id SERIAL,
 );

CREATE TABLE cart_lines (
  id SERIAL PRIMARY KEY,
  cart_id SERIAL NOT NULL REFERENCES carts(id),
  product_id SERIAL NOT NULL,
  qty SERIAL NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

 CREATE OR REPLACE FUNCTION update_modified_column()
 RETURNS TRIGGER AS $$
 BEGIN
 NEW.updated_at = now();
 RETURN NEW;
 END;
 $$ language 'plpgsql';

 CREATE TRIGGER update_modified_time BEFORE UPDATE ON cart_lines FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

CREATE OR REPLACE FUNCTION update_id_cart()
RETURNS TRIGGER AS $$
BEGIN
IF EXISTS (SELECT 1 FROM cart WHERE id_user = NEW.id_user) THEN
    NEW.id_cart = (SELECT MAX(id_cart) + 1 FROM cart WHERE id_user = NEW.id_user);
ELSE
    NEW.id_cart = 1;
END IF;
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_id_cart_trigger BEFORE INSERT ON cart FOR EACH ROW EXECUTE PROCEDURE update_id_cart();
