-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
	username VARCHAR(255) UNIQUE NOT NULL,
	password VARCHAR(255) NOT NULL,
	email VARCHAR(255) UNIQUE NOT NULL,
	name VARCHAR(255),
	avatar VARCHAR(255),
	github_data JSON DEFAULT '{}',
	extra_data JSON DEFAULT '{}',
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)