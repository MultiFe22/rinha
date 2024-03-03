-- Add migration script here
CREATE TABLE cliente (
    id SMALLINT NOT NULL,
    limite INTEGER NOT NULL,
    saldo INTEGER NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE transacao (
    id SERIAL PRIMARY KEY,
    valor INTEGER NOT NULL,
    tipo CHAR(1) NOT NULL,
    descricao VARCHAR(10) NOT NULL,
    cliente_id SMALLINT NOT NULL,
    FOREIGN KEY (cliente_id) REFERENCES cliente(id)
);

INSERT INTO cliente (id, limite, saldo) VALUES
(1, 100000, 0),
(2, 80000, 0),
(3, 1000000, 0),
(4, 10000000, 0),
(5, 500000, 0);