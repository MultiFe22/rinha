-- Add migration script here
ALTER TABLE transacao
ADD COLUMN realizada_em TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW();