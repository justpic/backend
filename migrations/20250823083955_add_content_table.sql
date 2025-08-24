CREATE TABLE picks (
	id 						UUID 		PRIMARY KEY,

	title					VARCHAR(255),
	description		VARCHAR(1024),

	source_url		TEXT,

	created 			TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	owner_id 			UUID 		NOT NULL REFERENCES users(id) ON DELETE SET NULL,

	mimetype			VARCHAR(255) NOT NULL DEFAULT 'bin',

	private				BOOLEAN NOT NULL DEFAULT false,
	ai_generated 	BOOLEAN NOT NULL DEFAULT false,
	nsfw 					BOOLEAN NOT NULL DEFAULT false,
	deleted 			BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX ids_picks_owner_id ON picks(owner_id);
CREATE INDEX idx_picks_private ON picks(private);