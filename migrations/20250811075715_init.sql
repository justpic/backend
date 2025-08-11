CREATE TABLE users (
    id              UUID PRIMARY KEY,

    created         TIMESTAMPTZ  NOT NULL,

    username        VARCHAR(255) UNIQUE NOT NULL,
    email           VARCHAR(255) UNIQUE NOT NULL,
    password_hash   VARCHAR(1024) NOT NULL,

    role            VARCHAR(64) DEFAULT 'regular' NOT NULL,

    avatar_url      VARCHAR(1024),

    email_confirmed BOOL DEFAULT false NOT NULL,
    nsfw_allowed    BOOL DEFAULT false NOT NULL
);

CREATE TABLE sessions (
    session_id      UUID PRIMARY KEY,
    session_key     VARCHAR(255) UNIQUE,

    user_id UUID    REFERENCES users(id) ON DELETE CASCADE NOT NULL,

    created         TIMESTAMPTZ NOT NULL,
    expires         TIMESTAMPTZ NOT NULL,

    os              VARCHAR(64),
    device          VARCHAR(255),
    user_agent      VARCHAR(512)
);

CREATE TABLE restrictions (
    id  UUID PRIMARY KEY
);

CREATE TABLE pics (
    id  UUID PRIMARY KEY
);

CREATE TABLE collections (
    id  UUID PRIMARY KEY
);

CREATE TABLE tags (
    id  UUID PRIMARY KEY,
    tag VARCHAR(255) UNIQUE,
);