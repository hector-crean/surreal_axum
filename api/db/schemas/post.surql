DEFINE TABLE post SCHEMAFULL
    PERMISSIONS
        FOR create WHERE $scope = 'user'
        FOR select, update, delete WHERE author = $auth.id;

DEFINE FIELD content ON post TYPE string DEFAULT "";
DEFINE FIELD color ON post TYPE string ASSERT $value IN ['pink', 'purple'];
DEFINE FIELD author ON post TYPE record<user> 
    VALUE $before OR $auth.id 
    DEFAULT $auth.id;

DEFINE FIELD created ON post TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD updated ON post TYPE datetime
    VALUE time::now()
    DEFAULT time::now();