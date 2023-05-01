create table public.images (
    id text not null,
    creator_id text not null,
    created_at timestamp with time zone not null,
    constraint images_pkey primary key (id)
) tablespace pg_default;

create table public.tokens (
    id text not null,
    user_id text not null,
    hash text not null,
    scopes array null,
    created_at timestamp with time zone not null,
    constraint tokens_pkey primary key (id)
) tablespace pg_default;