create table items (
    id              serial PRIMARY KEY,
    unique_key      text NOT NULL,
    iv              bytea NOT NULL,
    bytes           bytea NOT NULL,
    confirm         bytea NOT NULL,
    filename        bytea,
    lifespan        bigint,
    dl_limit        integer,
    accesses        integer NOT NULL DEFAULT 0,
    date_created    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_viewed     timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);

create table creds (
    id              serial PRIMARY KEY,
    access_key      bytea,
    salt            bytea,
    item_id         integer NOT NULL UNIQUE REFERENCES "items" ("id") ON DELETE CASCADE,
    date_created    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
