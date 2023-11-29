-- Add a table that describe the relaion between posts and tags
-- Table must restruct the client to only be able to create relations between posts


DEFINE TABLE tag_to_post_edge SCHEMAFULL
    PERMISSIONS
        FOR create, select, update, delete 
            WHERE in.owner == $auth.id AND out.author == $auth.id;

DEFINE FIELD in ON tag_to_post_edge TYPE record<tag>;
DEFINE FIELD out ON tag_to_post_edge TYPE record<post>;

DEFINE FIELD created ON tag_to_post_edge TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD updated ON tag_to_post_edge  TYPE datetime
    VALUE time::now()
    DEFAULT time::now();
