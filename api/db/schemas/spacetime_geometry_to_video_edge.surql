-- Add a table that describe the relaion between posts and tags
-- Table must restruct the client to only be able to create relations between posts


DEFINE TABLE spacetime_geometry_to_video_edge SCHEMAFULL
    PERMISSIONS
        FOR create, select, update, delete 
            WHERE in.author == $auth.id AND out.author == $auth.id;

DEFINE FIELD in ON spacetime_geometry_to_video_edge TYPE record<spacetime_geometry>;
DEFINE FIELD out ON spacetime_geometry_to_video_edge TYPE record<video>;

DEFINE FIELD created ON spacetime_geometry_to_video_edge  TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD updated ON spacetime_geometry_to_video_edge  TYPE datetime
    VALUE time::now()
    DEFAULT time::now();