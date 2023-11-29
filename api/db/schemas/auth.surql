-- Define a scope for clients to authenticate to
-- Session should last 7 days
-- User should be able to signin with their username and password
-- Users should be able to sign up their name, usernamem and password
DEFINE SCOPE user SESSION 7d
    SIGNIN (
        SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $password)
    )
    SIGNUP (
        CREATE user CONTENT {
            name: $name,
            username: $username,
            password: crypto::argon2::generate($password)
        }
    );