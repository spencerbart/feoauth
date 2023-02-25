create table if not exists "users" (
    "id" uuid primary key default gen_random_uuid(),
    "aud" varchar(255),
    "role" varchar(255),
    "email" varchar(255),
    "encrypted_password" varchar(255),
    "email_confirmed_at" timestamp with time zone,
    "invited_at" timestamp with time zone,
    "confirmation_token" varchar(255),
    "confirmation_sent_at" timestamp with time zone,
    "recovery_token" varchar(255),
    "recovery_sent_at" timestamp with time zone,
    "email_change_token_new" varchar(255),
    "email_change" varchar(255),
    "email_change_sent_at" timestamp with time zone,
    "last_sign_in_at" timestamp with time zone,
    "raw_app_meta_data" jsonb,
    "raw_user_meta_data" jsonb,
    "is_super_admin" boolean,
    "phone" varchar(255) not null,
    "phone_confirmed_at" timestamp with time zone,
    "phone_change" varchar(255),
    "phone_change_token" varchar(255),
    "phone_change_sent_at" timestamp with time zone,
    "confirmed_at" timestamp with time zone,
    "email_change_token_current" varchar(255),
    "email_change_confirm_status" int2,
    "banned_until" timestamp with time zone,
    "reauthentication_token" varchar(255),
    "reauthentication_sent_at" timestamp with time zone,
    "is_sso_user" boolean,
    "created_at" timestamp with time zone default now(),
    "updated_at" timestamp with time zone default now(),
    "deleted_at" timestamp with time zone
);


create table if not exists "audit_log_entries" (
    "id" uuid primary key default gen_random_uuid(),
    "payload" jsonb,
    "ip_address" varchar(255),
    "created_at" timestamp with time zone default now()
);