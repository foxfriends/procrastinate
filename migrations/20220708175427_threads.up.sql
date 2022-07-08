ALTER TABLE messages DROP COLUMN reply_to_id;

CREATE TABLE replies (
    message_id UUID NOT NULL REFERENCES messages (id),
    parent_id UUID NOT NULL REFERENCES messages (id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (message_id, parent_id)
);
