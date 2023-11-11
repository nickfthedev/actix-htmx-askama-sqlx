CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE "user" (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    avatar_path VARCHAR(255)
);