-- Add migration script here
ALTER TABLE cliente
ADD CONSTRAINT saldo_not_lower_than_neg_limite
CHECK (saldo >= -limite);
