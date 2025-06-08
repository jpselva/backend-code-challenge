CREATE TABLE Nodes (
    public_key CHAR(66) PRIMARY KEY,
    capacity bigint,
    alias CHAR(32),
    first_seen timestamp with time zone
);
