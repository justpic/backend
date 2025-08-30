CREATE TABLE cards (
	id 						UUID 		PRIMARY KEY,

	title					VARCHAR(255),
	description		VARCHAR(1024),

	source_url		TEXT,

	created 			TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	owner_id 			UUID 				REFERENCES users(id) ON DELETE SET NULL,

	mimetype			VARCHAR(255) NOT NULL DEFAULT 'bin',

	status				VARCHAR(64) NOT NULL DEFAULT 'pending',

	file_url			VARCHAR(1024),

	private				BOOLEAN NOT NULL DEFAULT false,
	ai_generated 	BOOLEAN NOT NULL DEFAULT false,
	nsfw 					BOOLEAN NOT NULL DEFAULT false,
	deleted 			BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX ids_cards_owner_id ON cards(owner_id);
CREATE INDEX idx_cards_private ON cards(private);