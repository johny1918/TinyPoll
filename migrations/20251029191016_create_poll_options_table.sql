CREATE TABLE poll_options (
                              id SERIAL PRIMARY KEY,
                              poll_id INTEGER REFERENCES polls(id) ON DELETE CASCADE,
                              option_text TEXT NOT NULL,
                              created_at TIMESTAMP DEFAULT NOW() NOT NULL
);
