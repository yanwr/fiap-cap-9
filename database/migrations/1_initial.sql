create table customer
(
    id              uuid                not null,
    email           varchar(200)        not null unique,
    password        text                not null,
    created_at      timestamptz         default now(),
    primary key (id)
);
create index index_customer_id_email on customer (email);
create index index_customer_id_email_password on customer (email, password);

create type biometrics_status as enum (
    'in_analysis',
    'reproved',
    'take_again'
    'conclued'
);

create table biometrics
(
    customer_id     uuid                not null,
    image_path      varchar(255)        not null,
    status          biometrics_status   not null,
    created_at      timestamptz         default now(),
    updated_at      timestamptz,

    constraint fk_biometrics_customer foreign key (customer_id) references customer (id)
)
