-- Add up migration script here
CREATE TABLE Roles (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  code RoleCodeEnum NOT NULL,
  name VARCHAR(255) NOT NULL,
  created_at timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE UserRoles (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  username VARCHAR(255) NOT NULL,
  role_id UUID NOT NULL,
  created_at timestamptz NOT NULL DEFAULT (now())
);
