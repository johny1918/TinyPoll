CREATE TABLE votes (
                       id SERIAL PRIMARY KEY,
                       poll_id INTEGER REFERENCES polls(id) ON DELETE CASCADE,
                       option_id INTEGER REFERENCES poll_options(id) ON DELETE CASCADE,
                       voter_identifier TEXT NOT NULL,
                       user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
                       created_at TIMESTAMP DEFAULT NOW() NOT NULL,
                       UNIQUE (poll_id, voter_identifier)
);
