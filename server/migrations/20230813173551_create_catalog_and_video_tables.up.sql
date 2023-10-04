CREATE TABLE catalog (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    display_name TEXT NOT NULL,
    short_desc TEXT NOT NULL,
    long_desc TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE metadata (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    size BIGINT NOT NULL,
    duration BIGINT NOT NULL,
    bitrate TEXT NOT NULL,
    width TEXT NOT NULL,
    height TEXT NOT NULL,
    framerate TEXT NOT NULL
);

CREATE TABLE video (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    display_name TEXT NOT NULL,
    short_desc TEXT NOT NULL,
    long_desc TEXT NOT NULL,
    catalog_id BIGINT REFERENCES catalog NOT NULL,
    sequent_id BIGINT REFERENCES video,
    metadata_id BIGINT REFERENCES metadata,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
