-- Add up migration script here
CREATE UNIQUE INDEX "unique_workout_plan_per_username" ON WorkoutPlan ("plan_name", "username");

CREATE UNIQUE INDEX "unique_plan_workout_workday_per_plan_workout" ON PlanWorkout ("workout_id", "plan_id", "workout_day");

CREATE UNIQUE INDEX "unique_plan_exercise_name_per_exercise" ON WorkoutExercises ("workout_id", "exercise_name");

CREATE UNIQUE INDEX "unique_equipment_type_per_exercise" ON Exercise ("equipment_required", "exercise_name");

ALTER TABLE MuscleGroups ADD FOREIGN KEY ("workout_id") REFERENCES Workout ("id");

ALTER TABLE UserProfile ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE WorkoutPlan ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE PlanWorkout ADD FOREIGN KEY ("plan_id") REFERENCES WorkoutPlan ("id");

ALTER TABLE PlanWorkout ADD FOREIGN KEY ("workout_id") REFERENCES Workout ("id");

ALTER TABLE WorkoutExercises ADD FOREIGN KEY ("workout_id") REFERENCES Workout ("id");

ALTER TABLE WorkoutExercises ADD FOREIGN KEY ("exercise_name") REFERENCES Exercise ("exercise_name");

ALTER TABLE WorkoutLog ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE WorkoutLog ADD FOREIGN KEY ("plan_id") REFERENCES WorkoutPlan ("id");

ALTER TABLE WorkoutLog ADD FOREIGN KEY ("workout_id") REFERENCES PlanWorkout ("id");

ALTER TABLE ExerciseLog ADD FOREIGN KEY ("log_id") REFERENCES WorkoutLog ("id");

ALTER TABLE ExerciseLog ADD FOREIGN KEY ("exercise_name") REFERENCES Exercise ("exercise_name");

ALTER TABLE Set ADD FOREIGN KEY ("exercise_log_id") REFERENCES ExerciseLog ("id");

ALTER TABLE WeightEntry ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE MaxRepGoal ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE MaxRepGoal ADD FOREIGN KEY ("exercise_name") REFERENCES Exercise ("exercise_name");

ALTER TABLE MaxWeightGoal ADD FOREIGN KEY ("username") REFERENCES "users" ("username");

ALTER TABLE MaxWeightGoal ADD FOREIGN KEY ("exercise_name") REFERENCES Exercise ("exercise_name");
