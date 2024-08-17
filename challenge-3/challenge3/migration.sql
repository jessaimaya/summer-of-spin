CREATE TABLE game (
    gameId CHAR(36) PRIMARY KEY,
    message TEXT NOT NULL,
    grid TEXT NOT NULL,
    currentRow INTEGER,
    solved INTEGER,
    word TEXT NOT NULL
);