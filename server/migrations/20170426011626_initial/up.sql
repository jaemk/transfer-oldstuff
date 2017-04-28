create table items (
    id              serial PRIMARY KEY,
    key             text UNIQUE NOT NULL,
    filepath        text NOT NULL,
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
