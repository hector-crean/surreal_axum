DEFINE TABLE comment SCHEMAFULL
    PERMISSIONS
        FOR create WHERE $scope = 'user'
        FOR select, update, delete WHERE author = $auth.id;

DEFINE FIELD content ON comment 
    TYPE string 
    DEFAULT "";

DEFINE FIELD color ON comment 
    TYPE string 
    ASSERT $value IN ['pink', 'purple'];

DEFINE FIELD author ON comment 
    TYPE record<user> 
    VALUE $before OR $auth.id 
    DEFAULT $auth.id;

DEFINE FIELD created ON comment 
    VALUE $before OR time::now()
    DEFAULT time::now();

DEFINE FIELD updated ON comment 
    VALUE time::now()
    DEFAULT time::now();