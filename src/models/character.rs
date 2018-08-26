// use crate::prelude::Either;

use ffxiv_types::World;

use url::Url;

use std::collections::BTreeMap;

pub mod search;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
  #[serde(rename = "ID")]
  pub id: usize,
  pub name: String,
  pub nameday: String,
  pub parse_date: usize,
  #[serde(rename = "PvPTeam")]
  pub pvp_team: Option<serde_json::Value>,
  pub race: Race,
  pub tribe: Tribe,
  pub server: World,
  pub title: Option<usize>,
  pub town: Town,
  #[serde(with = "url_serde")]
  pub avatar: Url,
  pub bio: String,
  pub free_company_id: String,
  pub gender: Gender,
  pub guardian_deity: GuardianDeity,
  pub minions: Vec<usize>,
  pub mounts: Vec<usize>,
  pub class_jobs: BTreeMap<String, ClassJob>,
  pub gear_set: GearSet,
  pub grand_company: Option<GrandCompany>,
  pub active_class_job: ClassJob,
  #[serde(with = "url_serde")]
  pub portrait: Url,
  #[serde(flatten)]
  pub verification: Verification,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterResult {
  pub state: State,
  // pub payload: Either<Character, [!; 0]>,
  pub payload: Option<Character>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassJob {
  #[serde(rename = "ClassID")]
  pub class_id: usize,
  #[serde(rename = "JobID")]
  pub job_id: usize,
  pub level: usize,
  pub exp_level: usize,
  pub exp_level_max: usize,
  pub exp_level_togo: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearSet {
  #[serde(rename = "ClassID")]
  pub class_id: usize,
  #[serde(rename = "JobID")]
  pub job_id: usize,
  pub level: usize,
  pub gear_key: String,
  pub attributes: BTreeMap<Attribute, usize>,
  pub gear: BTreeMap<GearSlot, Gear>,
}

#[derive(Debug, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum GearSlot {
  MainHand,
  Head,
  Body,
  Hands,
  Waist,
  Legs,
  Feet,
  OffHand,
  Earrings,
  Necklace,
  Bracelets,
  Ring1,
  Ring2,
  SoulCrystal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gear {
  #[serde(rename = "ID")]
  pub id: Option<usize>,
  pub dye: Option<usize>,
  pub mirage: Option<serde_json::Value>,
  pub materia: Vec<serde_json::Value>,
  pub creator: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GrandCompany {
  #[serde(rename = "NameID")]
  name_id: usize,
  #[serde(rename = "RankID")]
  rank_id: usize,
}

#[derive(Debug, Deserialize)]
pub struct Verification {
  #[serde(rename = "VerificationToken")]
  pub token: String,
  #[serde(rename = "VerificationTokenPass")]
  pub pass: bool,
}

macro_rules! enum_number {
  ($name:ident { $($variant:ident = $value:expr, )* }) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum $name {
      $($variant = $value,)*
    }

    impl serde::Serialize for $name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer,
      {
        serializer.serialize_u64(*self as u64)
      }
    }

    impl<'de> serde::Deserialize<'de> for $name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>,
      {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
          type Value = $name;

          fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("positive integer")
          }

          fn visit_u64<E>(self, value: u64) -> Result<$name, E>
            where E: serde::de::Error,
          {
            match value {
              $( $value => Ok($name::$variant), )*
              _ => Err(E::custom(
                  format!("unknown {} value: {}",
                  stringify!($name), value))),
            }
          }
        }

        deserializer.deserialize_u64(Visitor)
      }
    }
  }
}

enum_number!(State {
  Adding = 1,
  Cached = 2,
  NotFound = 3,
  Blacklist = 4,
});

enum_number!(Race {
  Hyur = 1,
  Elezen = 2,
  Lalafell = 3,
  Miqote = 4,
  Roegadyn = 5,
  AuRa = 6,
});

enum_number!(Tribe {
  Midlander = 1,
  Highlander = 2,
  Wildwood = 3,
  Duskwight = 4,
  Plainsfolk = 5,
  Dunesfolk = 6,
  SeekerOfTheSun = 7,
  SeekerOfTheMoon = 8,
  SeaWolf = 9,
  Hellsguard = 10,
  Raen = 11,
  Xaela = 12,
});

enum_number!(Town {
  LimsaLominsa = 1,
  Gridania = 2,
  UlDah = 3,
  Ishgard = 4,
  Kugane = 7,
});

enum_number!(Gender {
  Male = 1,
  Female = 2,
});

enum_number!(GuardianDeity {
  Halone = 1,
  Menphina = 2,
  Thaliak = 3,
  Nymeia = 4,
  Llymlaen = 5,
  Oschon = 6,
  Byregot = 7,
  Rhalgr = 8,
  Azeyma = 9,
  NaldThal = 10,
  Nophica = 11,
  Althyk = 12,
});

enum_number!(Attribute {
  Strength = 1,
  Dexterity = 2,
  Vitality = 3,
  Intelligence = 4,
  Mind = 5,
  Hp = 7,
  Mp = 8,
  Tp = 9,
  AttackPower = 20,
  Defense = 21,
  DirectHitRate = 22,
  MagicDefense = 24,
  CriticalHitRate = 27,
  AttackMagicPotency = 33,
  HealingMagicPotency = 34,
  Determination = 44,
  Tenacity = 45,
  SpellSpeed = 46,
});