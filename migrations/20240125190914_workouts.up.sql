-- Add up migration script here
CREATE TABLE WorkoutPlan (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "username" VARCHAR(255) NOT NULL,
  "plan_name" VARCHAR(255) NOT NULL,
  "description" TEXT NOT NULL DEFAULT (''),
  "start_date" timestamptz NOT NULL DEFAULT (now()),
  "end_date" timestamptz NOT NULL DEFAULT (now()),
  "goal" WorkoutGoalEnum NOT NULL DEFAULT ('lose_weight'),
  "difficulty" DifficultyEnum NOT NULL DEFAULT ('beginner'),
  "is_public" VisibilityEnum NOT NULL DEFAULT ('public'),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE Workout (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "workout_name" VARCHAR(255) NOT NULL,
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);


CREATE TABLE PlanWorkout (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "plan_id" UUID NOT NULL,
  "workout_id" UUID NOT NULL,
  "workout_day" WorkoutDayEnum NOT NULL DEFAULT ('monday'),
  "notes" TEXT NOT NULL DEFAULT (''),
  "created_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE MuscleGroups (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "name" VARCHAR(255) NOT NULL,
  "workout_id" UUID NOT NULL,
  "muscle_group" MuscleGroupEnum NOT NULL DEFAULT 'chest'
);

CREATE TABLE WorkoutExercises (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "workout_id" UUID NOT NULL,
  "exercise_name" VARCHAR(255) NOT NULL,
  "sets" INT NOT NULL DEFAULT (0),
  "reps" VARCHAR(255) NOT NULL DEFAULT (0),
  "rest_duration" VARCHAR NOT NULL DEFAULT ('')
);
