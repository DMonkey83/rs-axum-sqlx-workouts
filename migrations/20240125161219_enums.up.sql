-- Add up migration script here
 CREATE TYPE EquipmentEnum AS ENUM (
  'barbell',
  'dumbbell',
  'cable',
  'machine',
  'kettlebell',
  'bands',
  'bar',
  'rings',
  'ez_bar',
  'smith_machine',
  'bodyweight',
  'other'
);

CREATE TYPE CompletionEnum AS ENUM (
  'completed',
  'incomplete',
  'notstarted'
);

CREATE TYPE MuscleGroupEnum AS ENUM (
  'chest',
  'lower back',
  'upper back',
  'lats',
  'traps',
  'quads',
  'hamstrings',
  'calves',
  'shoulders',
  'forearms',
  'biceps',
  'triceps',
  'abs',
  'obliques',
  'cardio',
  'compound'
);

CREATE TYPE WorkoutGoalEnum AS ENUM (
  'build_muscle',
  'build_strength',
  'lose_weight',
  'improve_endurance',
  'maintain_fitness',
  'tone_body',
  'train_for_event',
  'improve_flexibility',
  'imporove_health',
  'improve_balance',
  'improve_coordination',
  'improve_power',
  'improve_speed',
  'custom'
);

CREATE TYPE DifficultyEnum AS ENUM (
  'beginner',
  'intermediate',
  'advanced'
);

CREATE TYPE VisibilityEnum AS ENUM (
  'public',
  'private'
);

CREATE TYPE RatingEnum AS ENUM (
  'one',
  'two',
  'three',
  'four',
  'five'
);

CREATE TYPE FatigueLevelEnum AS ENUM (
  'very_light',
  'light',
  'moderate',
  'heavy',
  'very_heavy'
);

CREATE TYPE WorkoutDayEnum AS ENUM (
  'monday',
  'tuesday',
  'wednesday',
  'thursday',
  'friday',
  'saturday',
  'sunday'
);

CREATE TYPE RoleCodeEnum AS ENUM (
  'admin',
  'user'
);
