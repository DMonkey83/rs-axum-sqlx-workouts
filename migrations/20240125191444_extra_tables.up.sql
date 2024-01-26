-- Add up migration script here
CREATE TABLE WeightEntry (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "username" VARCHAR(255) NOT NULL,
  "entry_date" timestamptz NOT NULL DEFAULT (now()),
  "weight" INT NOT NULL DEFAULT (0),
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE MaxRepGoal (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "username" VARCHAR(255) NOT NULL,
  "exercise_name" VARCHAR(255) UNIQUE NOT NULL,
  "goal_reps" INT NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE MaxWeightGoal (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "username" VARCHAR(255) NOT NULL,
  "exercise_name" VARCHAR(255) UNIQUE NOT NULL,
  "goal_weight" INT NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);
