DEFINE TABLE spacetime_geometry SCHEMAFULL;


DEFINE FIELD author ON spacetime_geometry TYPE option<record<user>>
    VALUE $before OR $auth.id 
    DEFAULT $auth.id;

DEFINE FIELD geometry ON spacetime_geometry TYPE geometry<feature>;

DEFINE FIELD timestamp ON spacetime_geometry TYPE float;

DEFINE FIELD duration ON spacetime_geometry TYPE duration;

DEFINE FIELD title ON spacetime_geometry TYPE string;

DEFINE FIELD text_body on spacetime_geometry TYPE string;

DEFINE FIELD created ON spacetime_geometry TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD updated ON spacetime_geometry  TYPE datetime
    VALUE time::now()
    DEFAULT time::now();
