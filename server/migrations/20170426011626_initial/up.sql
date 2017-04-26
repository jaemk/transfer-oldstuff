create table items (
    id              serial PRIMARY KEY,
    key             text UNIQUE NOT NULL,
    filepath        text NOT NULL,
    secret          text,
    lifespan        bigint,
    accesses        integer NOT NULL DEFAULT 0,
    date_created    timestamp WITH TIME ZONE NOT NULL DEFAULT NOW(),
    date_viewed     timestamp WITH TIME ZONE NOT NULL DEFAULT NOW()
);
