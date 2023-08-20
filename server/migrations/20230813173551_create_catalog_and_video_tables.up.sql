CREATE TABLE catalog (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    display_name TEXT NOT NULL,
    short_desc TEXT NOT NULL,
    long_desc TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE video (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    display_name TEXT NOT NULL,
    short_desc TEXT NOT NULL,
    long_desc TEXT NOT NULL,
    catalog_id BIGINT REFERENCES catalog NOT NULL,
    sequent_id BIGINT REFERENCES video,

    size BIGINT NOT NULL,
    duration BIGINT NOT NULL,
    bitrate BIGINT NOT NULL,
    width SMALLINT NOT NULL,
    height SMALLINT NOT NULL,
    framerate DOUBLE PRECISION NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
