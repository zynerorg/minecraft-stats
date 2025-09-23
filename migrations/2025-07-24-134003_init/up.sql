-- players table (minimal info)
CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE
);

-- categories like 'minecraft:mined', 'minecraft:killed', etc.
CREATE TABLE stat_categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

-- the main stats table
CREATE TABLE player_stats (
    player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES stat_categories(id) ON DELETE CASCADE,
    target TEXT NOT NULL, -- e.g., minecraft:stone, minecraft:zombie
    count INTEGER NOT NULL,

    PRIMARY KEY (player_id, category_id, target)
);