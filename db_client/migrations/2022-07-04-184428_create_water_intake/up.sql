-- This creates table for adding water-intake, the user-id
-- for now is just kept at 1 by the backend code
CREATE TABLE IF NOT EXISTS water_intake (
	id 		   SERIAL PRIMARY KEY,
	intake     INTEGER NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('water_intake');
