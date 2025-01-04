--- create tables
create table series (
    id text primary key,
    title text not null,
    "url" text not null
);

create index series_title_idx on series using hash(id);

create table volumes (
    id uuid primary key default gen_random_uuid(),
    series_id text references series(id),
    title text not null,
    publication_date date not null,
    unique(series_id, title, publication_date)
);

create index volumes_series_id_idx on volumes(series_id);

create index volumes_title_idx on volumes using hash(title);

create index volumes_publication_date_idx on volumes(publication_date);

create table user_volumes (
    user_id uuid,
    volume_id uuid references volumes(id),
    checked boolean not null default false,
    primary key(user_id, volume_id)
);

create index user_volumes_user_id_idx on user_volumes(user_id);

create index user_volumes_volume_id_idx on user_volumes(volume_id);

--- insert test data
insert into
    series (id, title, "url")
values
    (
        'series1',
        'Series 1',
        'http://example.com/series1'
    ),
    (
        'series2',
        'Series 2',
        'http://example.com/series2'
    );

insert into
    volumes (id, series_id, title, publication_date)
values
    (
        '00000000-0000-0000-0000-000000000001',
        'series1',
        'Volume 1',
        '2020-01-01'
    ),
    (
        '00000000-0000-0000-0000-000000000002',
        'series1',
        'Volume 2',
        '2020-02-01'
    ),
    (
        '00000000-0000-0000-0000-000000000003',
        'series1',
        'Volume 3',
        '2020-03-01'
    ),
    (
        '00000000-0000-0000-0000-000000000004',
        'series2',
        'Volume 1',
        '2020-01-01'
    ),
    (
        '00000000-0000-0000-0000-000000000005',
        'series2',
        'Volume 2',
        '2020-02-01'
    ),
    (
        '00000000-0000-0000-0000-000000000006',
        'series2',
        'Volume 3',
        '2020-03-01'
    );

insert into
    user_volumes (user_id, volume_id, checked)
values
    (
        '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000001',
        true
    ),
    (
        '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000002',
        true
    ),
    (
        '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000003',
        false
    ),
    (
        '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000004',
        true
    ),
    (
        '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000005',
        false
    ),
    (
        '00000000-0000-0000-0000-000000000001',
        '00000000-0000-0000-0000-000000000006',
        true
    );
