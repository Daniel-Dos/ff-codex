CREATE TABLE characters
(
    id      SERIAL PRIMARY KEY,
    game_id INTEGER      NOT NULL,
    name    VARCHAR(255) NOT NULL,

    CONSTRAINT uk_character_game_name UNIQUE (game_id, name),

    FOREIGN KEY (game_id) REFERENCES games (id)
);