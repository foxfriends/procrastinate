CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    display_name VARCHAR(128) NOT NULL,
    full_name VARCHAR(128) NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password CHAR(60) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    author_id UUID NOT NULL REFERENCES users (id),
    reply_to_id UUID REFERENCES messages (id),
    content JSONB NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE emojis (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_by UUID NOT NULL REFERENCES users (id),
    value VARCHAR(32) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE reactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    message_id UUID NOT NULL REFERENCES messages (id),
    emoji_id UUID REFERENCES emojis (id),
    author_id UUID NOT NULL REFERENCES users (id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    UNIQUE (message_id, emoji_id, author_id)
);

CREATE TYPE enum_attachment_type AS ENUM ('File', 'Image', 'Preview');

CREATE TABLE attachments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    attachment_type enum_attachment_type NOT NULL,
    message_id UUID NOT NULL REFERENCES messages (id),
    content TEXT NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
