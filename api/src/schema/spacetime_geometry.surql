DEFINE TABLE spacetime_point SCHEMAFULL;


DEFINE FIELD author ON spacetime_point TYPE option<record<user>>
    VALUE $before OR $auth.id 
    DEFAULT $auth.id;

DEFINE FIELD geometry ON spacetime_point TYPE geometry<geometry>;

DEFINE FIELD timestamp ON spacetime_point TYPE float;

DEFINE FIELD duration ON spacetime_point TYPE duration;

DEFINE FIELD title ON spacetime_point TYPE string;

DEFINE FIELD text_body on spacetime_point TYPE string;

DEFINE FIELD created ON spacetime_point TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD updated ON spacetime_point  TYPE datetime
    VALUE time::now()
    DEFAULT time::now();
