DEFINE TABLE video_post SCHEMAFULL
    PERMISSIONS
        FOR select
            --published posts can be seleted
            WHERE published = true
            -- A user can select all their own posts
            OR user = $auth.id 
        FOR create, update 
            -- A user can create or update their own posts
            WHERE user = $auth.id
        FOR delete 
            -- A user can delete their own posts
            WHERE user = $auth.id
            -- Or an admin can delete any posts
            OR $auth.admin = true


DEFINE FIELD author ON video_post 
    TYPE record<user> 
    VALUE $before OR $auth.id 
    DEFAULT $auth.id;

DEFINE FIELD created_at ON video_post 
    VALUE $before OR time::now()
    DEFAULT time::now();

DEFINE FIELD updated_at ON video_post 
    VALUE time::now()
    DEFAULT time::now();

DEFINE FIELD screen_coordinates ON VIDEO_POST
    TYPE Point;


