CREATE TABLE Nodes (
    public_key CHAR(66) PRIMARY KEY NOT NULL,
    capacity bigint NOT NULL,
    alias CHAR(32) NOT NULL,
    first_seen timestamp with time zone NOT NULL
);
