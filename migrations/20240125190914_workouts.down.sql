-- Add down migration script here
DROP TABLE WorkoutPlan;
CREATE TABLE Workout; 
CREATE TABLE PlanWorkout;
CREATE TABLE MuscleGroups;
CREATE TABLE WorkoutExercises;
