-- Add up migration script here
CREATE TABLE Exercise (
  "exercise_name" VARCHAR(255) PRIMARY KEY NOT NULL,
  "equipment_required" EquipmentEnum NOT NULL DEFAULT ('barbell'),
  "description" TEXT NOT NULL DEFAULT (''),
  "instructions" TEXT NOT NULL DEFAULT (''),
  "muscle_group_name" MuscleGroupEnum NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE WorkoutLog (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "username" VARCHAR NOT NULL,
  "plan_id" UUID NOT NULL,
  "workout_id" UUID NOT NULL,
  "log_date" timestamptz NOT NULL DEFAULT (now()),
  "rating" RatingEnum NOT NULL DEFAULT ('one'),
  "fatigue_level" FatigueLevelEnum NOT NULL DEFAULT ('very_light'),
  "overall_feeling" TEXT NOT NULL DEFAULT (''),
  "comments" TEXT NOT NULL DEFAULT (''),
  "workout_duration" VARCHAR(10) NOT NULL DEFAULT ('0m'),
  "total_calories_burned" INT NOT NULL DEFAULT (0),
  "total_distance" INT NOT NULL DEFAULT (0),
  "total_repetitions" INT NOT NULL DEFAULT (0),
  "total_sets" INT NOT NULL DEFAULT (0),
  "total_weight_lifted" INT NOT NULL DEFAULT (0),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE ExerciseLog (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "log_id" UUID NOT NULL,
  "exercise_name" VARCHAR(255) NOT NULL,
  "sets_completed" INT NOT NULL DEFAULT (0),
  "repetitions_completed" INT NOT NULL DEFAULT (0),
  "weight_lifted" INT NOT NULL DEFAULT (0),
  "notes" VARCHAR(255) NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE Set (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "exercise_log_id" UUID NOT NULL,
  "set_number" INT NOT NULL DEFAULT (1),
  "weight" int NOT NULL DEFAULT (1),
  "rest_duration" VARCHAR(8) NOT NULL DEFAULT (''),
  "reps_completed" INT NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);
