use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="genderenum", rename_all="lowercase")]
pub enum GenderEnum {
    Male,
    Female
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="weightenum", rename_all="lowercase")]
pub enum WeightEnum {
    Kg,
    Lb
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="heightenum", rename_all="snake_case" )]
pub enum HeightEnum {
    Cm,
    FtIn
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="equipmentenum", rename_all="snake_case" )]
pub enum EquipmentEnum {
    Barbell,
    Dumbbell,
    Cable,
    Machine,
    Kettlebell,
    Bands,
    Bar,
    Rings,
    EzBar,
    SmithMachine,
    Bodyweight,
    Other
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="completionenum", rename_all="snake_case" )]
pub enum CompletionEnum {
    Completed,
    Incomplete,
    Notstarted
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="musclegroupenum", rename_all="snake_case" )]
pub enum MuscleGroupEnum {
  Chest,
  LowerBback,
  UpperBback,
  Lats,
  Traps,
  Quads,
  Hamstrings,
  Calves,
  Shoulders,
  Forearms,
  Biceps,
  Triceps,
  Abs,
  Obliques,
  Cardio,
  Compound
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="workoutgoalenum", rename_all="snake_case" )]
pub enum WorkoutGoalEnum {
    BuildMuscle,
    BuildStrength,
    LoseWeight,
    ImporoveEndurance,
    MaintainFitness,
    ToneBody,
    TrainForEvent,
    ImporoveFlexibility,
    ImproveHealth,
    ImproveBalance,
    ImproveCoordination,
    ImprovePower,
    ImproveSpeed,
    Custom
}


#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="diffcultyenum", rename_all="snake_case" )]
pub enum DifficultyEnum {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="visibilityenum", rename_all="snake_case" )]
pub enum VisibilityEnum {
    Private,
    Public
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="ratingenum", rename_all="snake_case" )]
pub enum RatingEnum {
    One,
    Two,
    Three,
    Four,
    Five
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="fatiguelevelenum", rename_all="snake_case" )]
pub enum FatigueLevelEnum {
    VeryLight,
    Light,
    Moderate,
    Heavy,
    VeryHeavy
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="workoutdayenum", rename_all="snake_case" )]
pub enum WorkoutDayEnum {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

#[derive(sqlx::Type, Serialize, Deserialize,Debug, Clone, Copy)]
#[sqlx(type_name="rolecodeenum", rename_all="snake_case" )]
pub enum RoleCodeEnum {
    Admin,
    User
}
