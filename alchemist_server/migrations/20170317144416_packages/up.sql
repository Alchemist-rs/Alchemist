CREATE TABLE packages (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    arch VARCHAR NOT NULL,
    ubuntu VARCHAR NOT NULL,
    mint VARCHAR NOT NULL,
    debain VARCHAR NOT NULL,
    gentoo VARCHAR NOT NULL,
    void VARCHAR NOT NULL,
    mac VARCHAR NOT NULL,
    freebsd VARCHAR NOT NULL,
    netbsd VARCHAR NOT NULL,
    openbsd VARCHAR NOT NULL
)
