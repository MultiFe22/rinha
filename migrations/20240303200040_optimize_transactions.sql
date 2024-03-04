-- Add migration script here
CREATE INDEX ON transacao (cliente_id, realizada_em DESC);
