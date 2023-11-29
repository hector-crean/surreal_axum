DEFINE TABLE video SCHEMAFULL
    PERMISSIONS
        FOR create WHERE $scope = 'user'
        FOR select, update, delete WHERE author = $auth.id;


DEFINE FIELD author ON video TYPE option<record<user>>
    VALUE $before OR $auth.id 
    DEFAULT $auth.id;

DEFINE FIELD title on video TYPE string;
DEFINE FIELD description on video TYPE string;
DEFINE FIELD s3_key on video TYPE string;
DEFINE FIELD s3_url on video TYPE string;

DEFINE FIELD created ON video TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD updated ON video  TYPE datetime
    VALUE time::now()
    DEFAULT time::now();
