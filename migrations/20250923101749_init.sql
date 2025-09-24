CREATE TYPE VISIBILITY AS ENUM ('public', 'private');

-- USERS
CREATE TYPE USER_ROLE	AS ENUM ('regular', 'moderator', 'admin');

CREATE TABLE users(
	id					UUID					PRIMARY KEY,

	created			TIMESTAMPTZ		NOT NULL	DEFAULT CURRENT_TIMESTAMP,

	username		VARCHAR(256)	NOT NULL UNIQUE,
	email				VARCHAR(512)	NOT NULL UNIQUE,

	password		VARCHAR(1024)	NOT NULL,
	role				USER_ROLE			NOT NULL DEFAULT 'regular'
);

CREATE TABLE profiles(
	id					UUID					PRIMARY KEY,
	
	name				VARCHAR(256) 	NOT NULL,
	description VARCHAR(1024),

	visibility	VISIBILITY		NOT NULL DEFAULT 'public',

	FOREIGN KEY (id) REFERENCES users(id) ON DELETE CASCADE
);

-- CARDS
CREATE SEQUENCE card_id_seq START 100000;

CREATE TABLE cards(
	id					UUID				PRIMARY KEY,
	public_id		BIGINT			NOT NULL UNIQUE DEFAULT nextval('card_id_seq'),

	author_id		UUID,

	created			TIMESTAMPTZ	NOT NULL	DEFAULT CURRENT_TIMESTAMP,

	title				VARCHAR(128),
	description VARCHAR(512),

	source_url	VARCHAR(1024),

	visibility	VISIBILITY	NOT NULL DEFAULT 'public',

	deleted			BOOLEAN			NOT NULL DEFAULT false,

	FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE SET NULL
);


-- FILES
CREATE TYPE FILE_STATE AS ENUM ('pending', 'processing', 'ready', 'failed');

CREATE TABLE files(
	id				VARCHAR(48)		PRIMARY KEY,
	owner_id	UUID,

	created		TIMESTAMPTZ		NOT NULL	DEFAULT CURRENT_TIMESTAMP,

	filename	VARCHAR(512)	NOT NULL	DEFAULT 'file',
	mimetype	VARCHAR(128)	NOT NULL	DEFAULT 'bin',

	status		FILE_STATE		NOT NULL	DEFAULT 'pending',

	width			INT						NOT NULL	DEFAULT 512,
	height		INT						NOT NULL	DEFAULT 512,

	filesize	BIGINT				NOT NULL	DEFAULT 0,
	md5				VARCHAR(48)		NOT	NULL	DEFAULT '',

	sensitive	BOOLEAN				NOT NULL	DEFAULT false,

	FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE card_files(
	file_id			VARCHAR(48) PRIMARY KEY,
	card_id			UUID				NOT NULL,
	position		INT					NOT NULL DEFAULT 0,

	UNIQUE (card_id, position),

	FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE,
	FOREIGN KEY (file_id)	REFERENCES files(id) ON DELETE CASCADE
);