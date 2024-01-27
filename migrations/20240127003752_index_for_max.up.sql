-- Add up migration script here
CREATE UNIQUE INDEX "unique_max_weight_goal_per_exercise_user" ON MaxWeightGoal ("exercise_name", "username");
CREATE UNIQUE INDEX "unique_max_reps_goal_per_exercise_user" ON MaxRepGoal ("exercise_name", "username");
