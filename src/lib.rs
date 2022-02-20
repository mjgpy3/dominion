use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, EnumString};

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn bane_setup_has_the_expected_cards() {
        let others = vec![KC::Chapel, KC::Sentry, KC::YoungWitch];
        let setup = Setup::bane(KC::Bandit, others.clone());

        assert!(setup.cards().contains(&KC::Bandit));
        for other in others {
            assert!(setup.cards().contains(&other));
        }

        assert_eq!(setup.cards().len(), 4);
    }

    #[test]
    fn expansion_set_works_for_some_basic_cases() {
        let militia_expansions = expansion_set(&KC::Militia);

        assert!(militia_expansions.contains(&Expansion::Base1));
        assert!(militia_expansions.contains(&Expansion::Base2));
        assert_eq!(militia_expansions.len(), 2);

        let sentry_expansions = expansion_set(&KC::Sentry);

        assert!(sentry_expansions.contains(&Expansion::Base2));
        assert_eq!(sentry_expansions.len(), 1);
    }

    #[test]
    fn forcing_projects_and_expansions_without_projects_is_incoherrent() {
        let err = gen_setup(SetupConfig {
            include_expansions: Some(HashSet::from([Expansion::Base2])),
            project_count: Some(ProjectCount::OneProject),
            include_cards: None,
            ban_cards: None,
        })
        .unwrap_err();

        assert_eq!(err, GenSetupError::CouldNotSatisfyProjectsFromExpansions);
    }

    #[test]
    fn forcing_no_expansions_at_all_is_incoherrent() {
        let err = gen_setup(SetupConfig::including_expansions(HashSet::new())).unwrap_err();

        assert_eq!(err, GenSetupError::CouldNotSatisfyKingdomCards);
    }

    #[test]
    fn banned_cards_dont_come_up_in_the_setup() {
        let setup = gen_setup(SetupConfig {
            include_expansions: Some(HashSet::from([Expansion::Base2])),
            project_count: None,
            include_cards: None,
            ban_cards: Some(HashSet::from([KC::Witch, KC::Militia])),
        })
        .unwrap();

        assert!(!setup.cards().contains(&KC::Witch));
        assert!(!setup.cards().contains(&KC::Militia));
    }

    #[test]
    fn expansion_list_is_respected() {
        let expansion_1 = gen_expansion();
        let expansion_2 = gen_expansion();
        let expansions = HashSet::from([expansion_1, expansion_2]);
        let setup = gen_setup(SetupConfig::including_expansions(expansions.clone())).unwrap();

        for card in setup.cards() {
            assert!(!expansion_set(&card).is_disjoint(&expansions))
        }
    }

    #[test]
    fn cards_are_distinct() {
        let setup = gen_setup(SetupConfig::none()).unwrap();

        let cards = setup.cards();
        let card_set: HashSet<_> = cards.iter().collect();

        assert_eq!(cards.len(), card_set.len());
    }

    #[test]
    fn young_witch_implies_an_11th_bane_card() {
        let setup = gen_setup(SetupConfig::including_cards(HashSet::from([
            KC::YoungWitch,
        ])))
        .unwrap();

        assert!(&setup.bane_card.is_some());
        assert_eq!(11, setup.cards().len());
    }

    #[test]
    fn no_young_witch_no_bane_card() {
        let setup = gen_setup(SetupConfig {
            include_expansions: Some(HashSet::from([Expansion::Cornucopia])),
            project_count: None,
            include_cards: None,
            ban_cards: Some(HashSet::from([KC::YoungWitch])),
        })
        .unwrap();

        assert!(&setup.bane_card.is_none());
        assert_eq!(setup.cards().len(), 10);
    }

    #[test]
    fn banned_cards_are_not_included() {
        let banned_card = gen_kingdom_card();
        let setup = gen_setup(SetupConfig {
            include_expansions: Some(expansion_set(&banned_card)),
            project_count: None,
            include_cards: None,
            ban_cards: Some(HashSet::from([banned_card.clone()])),
        })
        .unwrap();

        assert!(!setup.cards().contains(&banned_card));
    }

    #[test]
    fn included_cards_are_included() {
        let included_card = gen_kingdom_card();
        let setup = gen_setup(SetupConfig::including_cards(HashSet::from([
            included_card.clone()
        ])))
        .unwrap();

        assert!(setup.cards().contains(&included_card));
    }

    #[test]
    fn included_cards_dont_change_kingdom_size() {
        let mut included_card = gen_kingdom_card();

        while included_card == KC::YoungWitch {
            included_card = gen_kingdom_card();
        }

        let setup = gen_setup(SetupConfig {
            include_expansions: None,
            project_count: None,
            include_cards: Some(HashSet::from([included_card.clone()])),
            ban_cards: Some(HashSet::from([KC::YoungWitch])),
        })
        .unwrap();

        assert_eq!(setup.cards().len(), 10);
    }

    #[test]
    fn included_cards_need_not_have_their_expansions_included() {
        let included_card = gen_kingdom_card();
        let expansion = gen_expansion();
        let setup = gen_setup(SetupConfig {
            include_expansions: Some(HashSet::from([expansion])),
            project_count: None,
            include_cards: Some(HashSet::from([included_card.clone()])),
            ban_cards: None,
        })
        .unwrap();

        assert!(setup.cards().contains(&included_card));
    }

    #[test]
    fn cannot_include_more_than_ten_cards() {
        let err =
            gen_setup(SetupConfig::including_cards(KC::iter().take(11).collect())).unwrap_err();

        assert_eq!(err, GenSetupError::TooManyCardsIncluded);
    }

    #[test]
    fn card_bans_and_includes_cannot_intersect() {
        let card = gen_kingdom_card();
        let err = gen_setup(SetupConfig {
            include_expansions: None,
            project_count: None,
            include_cards: Some(HashSet::from([card.clone()])),
            ban_cards: Some(HashSet::from([card.clone()])),
        })
        .unwrap_err();

        assert_eq!(
            err,
            GenSetupError::IntersectingCardBansAndIncludes(vec![card])
        );
    }

    #[test]
    fn bans_can_make_kingdom_cards_incoherent() {
        let err = gen_setup(SetupConfig {
            include_expansions: None,
            project_count: None,
            include_cards: None,
            ban_cards: Some(KC::iter().collect()),
        })
        .unwrap_err();

        assert_eq!(err, GenSetupError::CouldNotSatisfyKingdomCards);
    }

    #[test]
    fn bans_can_make_bane_card_incoherent() {
        let err = gen_setup(SetupConfig {
            include_expansions: Some(HashSet::from([Expansion::Cornucopia])),
            project_count: None,
            include_cards: None,
            // We have enough kingdom cards but not enough to pick a bane card
            ban_cards: Some(HashSet::from([
                KC::Hamlet,
                KC::FortuneTeller,
                KC::Menagerie,
            ])),
        })
        .unwrap_err();

        assert_eq!(err, GenSetupError::CouldNotSatisfyBaneCard);
    }

    fn gen_expansion() -> Expansion {
        let mut rng = rand::thread_rng();

        Expansion::iter()
            .collect::<Vec<Expansion>>()
            .choose(&mut rng)
            .unwrap()
            .clone()
    }

    fn gen_kingdom_card() -> KC {
        let mut rng = rand::thread_rng();

        KC::iter()
            .collect::<Vec<KC>>()
            .choose(&mut rng)
            .unwrap()
            .clone()
    }
}

/// A kingdom card
#[derive(
    EnumIter, Debug, PartialEq, EnumCountMacro, Clone, Eq, Hash, EnumString, PartialOrd, Ord,
)]
pub enum KC {
    ActingTroupe,
    Adventurer,
    Advisor,
    Ambassador,
    Artisan,
    Baker,
    Bandit,
    Baron,
    Bazaar,
    BorderGuard,
    Bridge,
    Bureaucrat,
    Butcher,
    CandlestickMaker,
    Caravan,
    CargoShip,
    Cellar,
    Chancellor,
    Chapel,
    Conspirator,
    CouncilRoom,
    Courtier,
    Courtyard,
    Cutpurse,
    Diplomat,
    Doctor,
    Ducat,
    Duke,
    Embargo,
    Experiment,
    Explorer,
    Fairgrounds,
    FarmingVillage,
    Feast,
    Festival,
    FishingVillage,
    FlagBearer,
    FortuneTeller,
    Gardens,
    GhostShip,
    Hamlet,
    Harbinger,
    Harem,
    Harvest,
    Haven,
    Herald,
    Hideout,
    HornOfPlenty,
    HorseTraders,
    HuntingParty,
    Improve,
    Inventor,
    Ironworks,
    Island,
    Jester,
    Journeyman,
    Laboratory,
    Lackeys,
    Library,
    Lighthouse,
    Lookout,
    Lurker,
    Market,
    Masquerade,
    Masterpiece,
    Menagerie,
    Merchant,
    MerchantGuild,
    MerchantShip,
    Militia,
    Mill,
    Mine,
    MiningVillage,
    Minion,
    Moat,
    Moneylender,
    MountainVillage,
    NativeVillage,
    Navigator,
    Nobles,
    OldWitch,
    Outpost,
    Patrol,
    Patron,
    Pawn,
    PearlDiver,
    PirateShip,
    Plaza,
    Poacher,
    Priest,
    Recruiter,
    Remake,
    Remodel,
    Replace,
    Research,
    Salvager,
    Scepter,
    Scholar,
    Sculptor,
    SeaHag,
    SecretPassage,
    Seer,
    Sentry,
    ShantyTown,
    SilkMerchant,
    Smithy,
    Smugglers,
    Soothsayer,
    Spices,
    Spy,
    Steward,
    Stonemason,
    Swashbuckler,
    Swindler,
    Tactician,
    Taxman,
    Thief,
    ThroneRoom,
    Torturer,
    Tournament,
    TradingPost,
    TreasureMap,
    Treasurer,
    Treasury,
    Upgrade,
    Vassal,
    Village,
    Villain,
    Warehouse,
    Wharf,
    WishingWell,
    Witch,
    Woodcutter,
    Workshop,
    YoungWitch,
}

/// What is a card's base cost?
pub trait BaseCost {
    /// What is a card's base cost?
    fn base_cost(&self) -> u8;
}

impl BaseCost for KC {
    fn base_cost(&self) -> u8 {
        match self {
            KC::Embargo => 2,
            KC::Haven => 2,
            KC::Lighthouse => 2,
            KC::NativeVillage => 2,
            KC::PearlDiver => 2,
            KC::Ambassador => 3,
            KC::FishingVillage => 3,
            KC::Lookout => 3,
            KC::Smugglers => 3,
            KC::Warehouse => 3,
            KC::Caravan => 4,
            KC::Cutpurse => 4,
            KC::Island => 4,
            KC::Navigator => 4,
            KC::PirateShip => 4,
            KC::Salvager => 4,
            KC::SeaHag => 4,
            KC::TreasureMap => 4,
            KC::Bazaar => 5,
            KC::Explorer => 5,
            KC::GhostShip => 5,
            KC::MerchantShip => 5,
            KC::Outpost => 5,
            KC::Tactician => 5,
            KC::Treasury => 5,
            KC::Wharf => 5,
            KC::Courtyard => 2,
            KC::Lurker => 2,
            KC::Pawn => 2,
            KC::Masquerade => 3,
            KC::ShantyTown => 3,
            KC::Steward => 3,
            KC::Swindler => 3,
            KC::WishingWell => 3,
            KC::Baron => 4,
            KC::Bridge => 4,
            KC::Conspirator => 4,
            KC::Diplomat => 4,
            KC::Ironworks => 4,
            KC::Mill => 4,
            KC::MiningVillage => 4,
            KC::SecretPassage => 4,
            KC::Courtier => 5,
            KC::Duke => 5,
            KC::Minion => 5,
            KC::Patrol => 5,
            KC::Replace => 5,
            KC::Torturer => 5,
            KC::TradingPost => 5,
            KC::Upgrade => 5,
            KC::Harem => 6,
            KC::Nobles => 6,
            KC::Harbinger => 3,
            KC::Merchant => 3,
            KC::Vassal => 3,
            KC::Poacher => 4,
            KC::Sentry => 5,
            KC::Artisan => 6,
            KC::ActingTroupe => 3,
            KC::Adventurer => 6,
            KC::Advisor => 4,
            KC::Baker => 5,
            KC::Bandit => 5,
            KC::BorderGuard => 2,
            KC::Bureaucrat => 4,
            KC::Butcher => 5,
            KC::CandlestickMaker => 2,
            KC::CargoShip => 3,
            KC::Cellar => 2,
            KC::Chancellor => 3,
            KC::Chapel => 2,
            KC::CouncilRoom => 5,
            KC::Doctor => 3,
            KC::Ducat => 2,
            KC::Experiment => 3,
            KC::Fairgrounds => 6,
            KC::FarmingVillage => 4,
            KC::Feast => 4,
            KC::Festival => 5,
            KC::FlagBearer => 4,
            KC::FortuneTeller => 3,
            KC::Gardens => 4,
            KC::Hamlet => 2,
            KC::Harvest => 5,
            KC::Herald => 4,
            KC::Hideout => 4,
            KC::HornOfPlenty => 5,
            KC::HorseTraders => 4,
            KC::HuntingParty => 5,
            KC::Improve => 3,
            KC::Inventor => 4,
            KC::Jester => 5,
            KC::Journeyman => 5,
            KC::Laboratory => 5,
            KC::Lackeys => 2,
            KC::Library => 5,
            KC::Market => 5,
            KC::Masterpiece => 3,
            KC::Menagerie => 3,
            KC::MerchantGuild => 5,
            KC::Militia => 4,
            KC::Mine => 5,
            KC::Moat => 2,
            KC::Moneylender => 4,
            KC::MountainVillage => 4,
            KC::OldWitch => 5,
            KC::Patron => 4,
            KC::Plaza => 4,
            KC::Priest => 4,
            KC::Recruiter => 5,
            KC::Remake => 4,
            KC::Remodel => 4,
            KC::Research => 4,
            KC::Scepter => 5,
            KC::Scholar => 5,
            KC::Sculptor => 5,
            KC::Seer => 5,
            KC::SilkMerchant => 4,
            KC::Smithy => 4,
            KC::Soothsayer => 5,
            KC::Spices => 5,
            KC::Spy => 4,
            KC::Stonemason => 2,
            KC::Swashbuckler => 5,
            KC::Taxman => 4,
            KC::Thief => 4,
            KC::ThroneRoom => 4,
            KC::Tournament => 4,
            KC::Treasurer => 5,
            KC::Village => 3,
            KC::Villain => 5,
            KC::Witch => 5,
            KC::Woodcutter => 3,
            KC::Workshop => 3,
            KC::YoungWitch => 4,
        }
    }
}

/// Supported expansions
#[derive(EnumIter, Debug, PartialEq, EnumCountMacro, Eq, Hash, std::clone::Clone, EnumString, PartialOrd, Ord)]
pub enum Expansion {
    Base1,
    Base2,
    Renaissance,
    Guilds,
    Cornucopia,
    Intrigue2,
    Seaside,
}

/// To what expansions does a card belong?
pub trait Expansions {
    /// To what expansions does a card belong?
    fn expansions(&self) -> Vec<Expansion>;
}

impl Expansions for KC {
    fn expansions(&self) -> std::vec::Vec<Expansion> {
        match self {
            KC::Courtyard
            | KC::Lurker
            | KC::Pawn
            | KC::Masquerade
            | KC::ShantyTown
            | KC::Steward
            | KC::Swindler
            | KC::WishingWell
            | KC::Baron
            | KC::Bridge
            | KC::Conspirator
            | KC::Diplomat
            | KC::Ironworks
            | KC::Mill
            | KC::MiningVillage
            | KC::SecretPassage
            | KC::Courtier
            | KC::Duke
            | KC::Minion
            | KC::Patrol
            | KC::Replace
            | KC::Torturer
            | KC::TradingPost
            | KC::Upgrade
            | KC::Harem
            | KC::Nobles => vec![Expansion::Intrigue2],

            KC::Harbinger | KC::Vassal | KC::Sentry | KC::Poacher | KC::Merchant | KC::Artisan => {
                vec![Expansion::Base2]
            }

            KC::Cellar
            | KC::Chapel
            | KC::Moat
            | KC::Village
            | KC::Workshop
            | KC::Bureaucrat
            | KC::Gardens
            | KC::Militia
            | KC::Moneylender
            | KC::Remodel
            | KC::Smithy
            | KC::ThroneRoom
            | KC::Bandit
            | KC::CouncilRoom
            | KC::Festival
            | KC::Laboratory
            | KC::Library
            | KC::Market
            | KC::Mine
            | KC::Witch => vec![Expansion::Base2, Expansion::Base1],

            KC::Chancellor | KC::Woodcutter | KC::Feast | KC::Spy | KC::Thief | KC::Adventurer => {
                vec![Expansion::Base1]
            }

            KC::BorderGuard
            | KC::Ducat
            | KC::Lackeys
            | KC::ActingTroupe
            | KC::CargoShip
            | KC::Experiment
            | KC::Improve
            | KC::FlagBearer
            | KC::Hideout
            | KC::Inventor
            | KC::MountainVillage
            | KC::Patron
            | KC::Priest
            | KC::Research
            | KC::SilkMerchant
            | KC::OldWitch
            | KC::Recruiter
            | KC::Scepter
            | KC::Scholar
            | KC::Sculptor
            | KC::Seer
            | KC::Spices
            | KC::Swashbuckler
            | KC::Treasurer
            | KC::Villain => vec![Expansion::Renaissance],

            KC::CandlestickMaker
            | KC::Stonemason
            | KC::Doctor
            | KC::Masterpiece
            | KC::Advisor
            | KC::Plaza
            | KC::Taxman
            | KC::Herald
            | KC::Baker
            | KC::Butcher
            | KC::Journeyman
            | KC::MerchantGuild
            | KC::Soothsayer => vec![Expansion::Guilds],

            KC::Hamlet
            | KC::FortuneTeller
            | KC::Menagerie
            | KC::FarmingVillage
            | KC::HorseTraders
            | KC::Remake
            | KC::Tournament
            | KC::YoungWitch
            | KC::Harvest
            | KC::HornOfPlenty
            | KC::HuntingParty
            | KC::Jester
            | KC::Fairgrounds => vec![Expansion::Cornucopia],

            KC::Embargo
            | KC::Haven
            | KC::Lighthouse
            | KC::NativeVillage
            | KC::PearlDiver
            | KC::Ambassador
            | KC::FishingVillage
            | KC::Lookout
            | KC::Smugglers
            | KC::Warehouse
            | KC::Bazaar
            | KC::Explorer
            | KC::GhostShip
            | KC::MerchantShip
            | KC::Outpost
            | KC::Tactician
            | KC::Treasury
            | KC::Wharf
            | KC::Caravan
            | KC::Cutpurse
            | KC::Island
            | KC::Navigator
            | KC::PirateShip
            | KC::Salvager
            | KC::SeaHag
            | KC::TreasureMap => vec![Expansion::Seaside],
        }
    }
}

/// Card's type -- how it functions
#[derive(EnumIter, Debug, PartialEq, EnumCountMacro, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum CardType {
    Action,
    Attack,
    Reaction,
    Victory,
    Treasure,
    Duration,
}

/// What types does a card have?
pub trait CardTypes {
    /// What types does a card have?
    fn card_types(&self) -> Vec<CardType>;
}

impl CardTypes for KC {
    fn card_types(&self) -> std::vec::Vec<CardType> {
        match self {
            KC::Embargo => vec![CardType::Action],
            KC::Haven => vec![CardType::Action, CardType::Duration],
            KC::Lighthouse => vec![CardType::Action, CardType::Duration],
            KC::NativeVillage => vec![CardType::Action],
            KC::PearlDiver => vec![CardType::Action],
            KC::Ambassador => vec![CardType::Action, CardType::Attack],
            KC::FishingVillage => vec![CardType::Action, CardType::Duration],
            KC::Lookout => vec![CardType::Action],
            KC::Smugglers => vec![CardType::Action],
            KC::Warehouse => vec![CardType::Action],
            KC::Caravan => vec![CardType::Action, CardType::Duration],
            KC::Cutpurse => vec![CardType::Action, CardType::Attack],
            KC::Island => vec![CardType::Action, CardType::Victory],
            KC::Navigator => vec![CardType::Action],
            KC::PirateShip => vec![CardType::Action, CardType::Attack],
            KC::Salvager => vec![CardType::Action],
            KC::SeaHag => vec![CardType::Action, CardType::Attack],
            KC::TreasureMap => vec![CardType::Action],
            KC::Bazaar => vec![CardType::Action],
            KC::Explorer => vec![CardType::Action],
            KC::GhostShip => vec![CardType::Action, CardType::Attack],
            KC::MerchantShip => vec![CardType::Action, CardType::Duration],
            KC::Outpost => vec![CardType::Action, CardType::Duration],
            KC::Tactician => vec![CardType::Action, CardType::Duration],
            KC::Treasury => vec![CardType::Action],
            KC::Wharf => vec![CardType::Action, CardType::Duration],
            KC::Courtyard => vec![CardType::Action],
            KC::Lurker => vec![CardType::Action],
            KC::Pawn => vec![CardType::Action],
            KC::Masquerade => vec![CardType::Action],
            KC::ShantyTown => vec![CardType::Action],
            KC::Steward => vec![CardType::Action],
            KC::Swindler => vec![CardType::Action, CardType::Attack],
            KC::WishingWell => vec![CardType::Action],
            KC::Baron => vec![CardType::Action],
            KC::Bridge => vec![CardType::Action],
            KC::Conspirator => vec![CardType::Action],
            KC::Diplomat => vec![CardType::Action, CardType::Reaction],
            KC::Ironworks => vec![CardType::Action],
            KC::Mill => vec![CardType::Action, CardType::Victory],
            KC::MiningVillage => vec![CardType::Action],
            KC::SecretPassage => vec![CardType::Action],
            KC::Courtier => vec![CardType::Action],
            KC::Duke => vec![CardType::Victory],
            KC::Minion => vec![CardType::Action, CardType::Attack],
            KC::Patrol => vec![CardType::Action],
            KC::Replace => vec![CardType::Action, CardType::Attack],
            KC::Torturer => vec![CardType::Action, CardType::Attack],
            KC::TradingPost => vec![CardType::Action],
            KC::Upgrade => vec![CardType::Action],
            KC::Harem => vec![CardType::Treasure, CardType::Victory],
            KC::Nobles => vec![CardType::Action, CardType::Victory],
            KC::Harbinger => vec![CardType::Action],
            KC::Merchant => vec![CardType::Action],
            KC::Vassal => vec![CardType::Action],
            KC::Poacher => vec![CardType::Action],
            KC::Sentry => vec![CardType::Action],
            KC::Artisan => vec![CardType::Action],
            KC::ActingTroupe => vec![CardType::Action],
            KC::Adventurer => vec![CardType::Action],
            KC::Advisor => vec![CardType::Action],
            KC::Baker => vec![CardType::Action],
            KC::Bandit => vec![CardType::Action, CardType::Attack],
            KC::BorderGuard => vec![CardType::Action],
            KC::Bureaucrat => vec![CardType::Action, CardType::Attack],
            KC::Butcher => vec![CardType::Action],
            KC::CandlestickMaker => vec![CardType::Action],
            KC::CargoShip => vec![CardType::Action, CardType::Duration],
            KC::Cellar => vec![CardType::Action],
            KC::Chancellor => vec![CardType::Action],
            KC::Chapel => vec![CardType::Action],
            KC::CouncilRoom => vec![CardType::Action],
            KC::Doctor => vec![CardType::Action],
            KC::Ducat => vec![CardType::Treasure],
            KC::Experiment => vec![CardType::Action],
            KC::Fairgrounds => vec![CardType::Victory],
            KC::FarmingVillage => vec![CardType::Action],
            KC::Feast => vec![CardType::Action],
            KC::Festival => vec![CardType::Action],
            KC::FlagBearer => vec![CardType::Action],
            KC::FortuneTeller => vec![CardType::Action, CardType::Attack],
            KC::Gardens => vec![CardType::Victory],
            KC::Hamlet => vec![CardType::Action],
            KC::Harvest => vec![CardType::Action],
            KC::Herald => vec![CardType::Action],
            KC::Hideout => vec![CardType::Action],
            KC::HornOfPlenty => vec![CardType::Treasure],
            KC::HorseTraders => vec![CardType::Action, CardType::Reaction],
            KC::HuntingParty => vec![CardType::Action],
            KC::Improve => vec![CardType::Action],
            KC::Inventor => vec![CardType::Action],
            KC::Jester => vec![CardType::Action, CardType::Attack],
            KC::Journeyman => vec![CardType::Action],
            KC::Laboratory => vec![CardType::Action],
            KC::Lackeys => vec![CardType::Action],
            KC::Library => vec![CardType::Action],
            KC::Market => vec![CardType::Action],
            KC::Masterpiece => vec![CardType::Treasure],
            KC::Menagerie => vec![CardType::Action],
            KC::MerchantGuild => vec![CardType::Action],
            KC::Militia => vec![CardType::Action, CardType::Attack],
            KC::Mine => vec![CardType::Action],
            KC::Moat => vec![CardType::Action, CardType::Reaction],
            KC::Moneylender => vec![CardType::Action],
            KC::MountainVillage => vec![CardType::Action],
            KC::OldWitch => vec![CardType::Action, CardType::Attack],
            KC::Patron => vec![CardType::Action, CardType::Reaction],
            KC::Plaza => vec![CardType::Action],
            KC::Priest => vec![CardType::Action],
            KC::Recruiter => vec![CardType::Action],
            KC::Remake => vec![CardType::Action],
            KC::Remodel => vec![CardType::Action],
            KC::Research => vec![CardType::Action, CardType::Duration],
            KC::Scepter => vec![CardType::Treasure],
            KC::Scholar => vec![CardType::Action],
            KC::Sculptor => vec![CardType::Action],
            KC::Seer => vec![CardType::Action],
            KC::SilkMerchant => vec![CardType::Action],
            KC::Smithy => vec![CardType::Action],
            KC::Soothsayer => vec![CardType::Action, CardType::Attack],
            KC::Spices => vec![CardType::Treasure],
            KC::Spy => vec![CardType::Action, CardType::Attack],
            KC::Stonemason => vec![CardType::Action],
            KC::Swashbuckler => vec![CardType::Action],
            KC::Taxman => vec![CardType::Action, CardType::Attack],
            KC::Thief => vec![CardType::Action, CardType::Attack],
            KC::ThroneRoom => vec![CardType::Action],
            KC::Tournament => vec![CardType::Action],
            KC::Treasurer => vec![CardType::Action],
            KC::Village => vec![CardType::Action],
            KC::Villain => vec![CardType::Action, CardType::Attack],
            KC::Witch => vec![CardType::Action, CardType::Attack],
            KC::Woodcutter => vec![CardType::Action],
            KC::Workshop => vec![CardType::Action],
            KC::YoungWitch => vec![CardType::Action, CardType::Attack],
        }
    }
}

/// A project card
#[derive(EnumIter, Debug, PartialEq, EnumCountMacro, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum Project {
    Academy,
    Barracks,
    Canal,
    Capitalism,
    Cathedral,
    Citadel,
    CityGate,
    CropRotation,
    Exploration,
    Fair,
    Fleet,
    Guildhall,
    Innovation,
    Pageant,
    Piazza,
    RoadNetwork,
    Sewers,
    Silos,
    SinisterPlot,
    StarChart,
}

impl Expansions for Project {
    fn expansions(&self) -> Vec<Expansion> {
        vec![Expansion::Renaissance]
    }
}

/// A game's setup
#[derive(Debug)]
pub struct Setup {
    pub kingdom_cards: Vec<KC>,
    pub bane_card: Option<KC>,
    pub project_cards: Vec<Project>,
}

impl Setup {
    pub fn new(kingdom_cards: Vec<KC>, bane_card: Option<KC>, project_cards: Vec<Project>) -> Self {
        Self {
            kingdom_cards,
            bane_card,
            project_cards,
        }
    }

    pub fn bane(bane: KC, other_kingdom: Vec<KC>) -> Self {
        Self {
            kingdom_cards: other_kingdom,
            bane_card: Some(bane),
            project_cards: vec![],
        }
    }

    pub fn cards(&self) -> Vec<KC> {
        let mut results = self.kingdom_cards.clone();
        for bane in &self.bane_card {
            results.push(bane.clone());
        }
        results
    }
}

#[derive(EnumString, Debug)]
/// The number of projects allowed in a game
pub enum ProjectCount {
    #[strum(serialize = "0")]
    NoProjects,
    #[strum(serialize = "1")]
    OneProject,
    #[strum(serialize = "2")]
    TwoProjects,
}

impl ProjectCount {
    /// Convert enum to actual count
    ///
    ///```
    ///let count = dominion::ProjectCount::OneProject.count();
    ///assert_eq!(count, 1);
    ///```
    pub fn count(&self) -> usize {
        match self {
            ProjectCount::NoProjects => 0,
            ProjectCount::OneProject => 1,
            ProjectCount::TwoProjects => 2,
        }
    }
}

/// How to setup a game
#[derive(Debug)]
pub struct SetupConfig {
    /// Which specific expansions to include
    pub include_expansions: Option<HashSet<Expansion>>,

    /// Cards to be sure _not_ to include
    pub ban_cards: Option<HashSet<KC>>,

    /// Cards to be sure to include. It is okay for this to be inconsistent with
    /// `include_expansions` (e.g. value here not member of listed expansions).
    pub include_cards: Option<HashSet<KC>>,

    /// How many projects to include (for random of count)
    /// If expansions are provided and we can't pick enough projects to satisfy
    /// this count, we'll return an error
    pub project_count: Option<ProjectCount>,
}

impl SetupConfig {
    /// Give us a totally random game
    pub fn none() -> SetupConfig {
        SetupConfig {
            include_expansions: None,
            ban_cards: None,
            include_cards: None,
            project_count: None,
        }
    }

    /// Only include these expansions
    pub fn including_expansions(expansions: HashSet<Expansion>) -> SetupConfig {
        SetupConfig {
            include_expansions: Some(expansions),
            ban_cards: None,
            include_cards: None,
            project_count: None,
        }
    }

    /// Be sure to include (at least) these cards
    pub fn including_cards(cards: HashSet<KC>) -> SetupConfig {
        SetupConfig {
            include_expansions: None,
            ban_cards: None,
            include_cards: Some(cards),
            project_count: None,
        }
    }
}

/// Errors we may encounter when generating a setup. These are mostly due to
/// incoherent configurations.
#[derive(Debug, PartialEq)]
pub enum GenSetupError {
    /// Asked for some number of projects but didn't supply enough expansions to
    /// choose them.
    CouldNotSatisfyProjectsFromExpansions,

    /// Filtered in such a way as to not allow us to pick enough kingdom cards.
    CouldNotSatisfyKingdomCards,

    /// Filtered in such a way as to not allow us to pick a bane card.
    CouldNotSatisfyBaneCard,

    /// Asked to ban and include one or more cards.
    IntersectingCardBansAndIncludes(Vec<KC>),

    /// Asked to include more cards than the kingdom can handle. Right now this
    /// just errors if given more than 10. Technically we should be able to
    /// handle an 11th if `KC::YoungWitch` is one of them.
    TooManyCardsIncluded,
}

fn expansion_set<T: Expansions>(v: &T) -> HashSet<Expansion> {
    v.expansions().into_iter().collect()
}

/// Generate a valid setup from options (`SetupConfig`)
pub fn gen_setup(config: SetupConfig) -> Result<Setup, GenSetupError> {
    let mut rng = rand::thread_rng();

    for bans in &config.ban_cards {
        for includes in &config.include_cards {
            if !bans.is_disjoint(includes) {
                let cards_in_common = bans.intersection(&includes);
                return Err(GenSetupError::IntersectingCardBansAndIncludes(
                    cards_in_common.cloned().collect(),
                ));
            }
        }
    }

    let desired_expansions = config
        .include_expansions
        .unwrap_or(Expansion::iter().collect());

    let possible_projects: Vec<Project> = Project::iter()
        .filter(|p| !expansion_set(p).is_disjoint(&desired_expansions))
        .collect();

    let banned_cards = config.ban_cards.unwrap_or(HashSet::new());

    let forced_kingdom_cards = config.include_cards.unwrap_or(HashSet::new());

    if forced_kingdom_cards.len() > 10 {
        return Err(GenSetupError::TooManyCardsIncluded);
    }

    let mut possible_kingdom_cards: Vec<KC> = KC::iter()
        .filter(|kc| !expansion_set(kc).is_disjoint(&desired_expansions))
        .filter(|kc| !banned_cards.contains(kc))
        .filter(|kc| !forced_kingdom_cards.contains(kc))
        .collect();

    let project_count = match config.project_count {
        Some(desired) => {
            if possible_projects.len() < desired.count() {
                return Err(GenSetupError::CouldNotSatisfyProjectsFromExpansions);
            } else {
                desired.count()
            }
        }
        None => rng.gen_range(0..3),
    };

    let project_cards = possible_projects
        .choose_multiple(&mut rng, project_count)
        .cloned()
        .collect();

    possible_kingdom_cards.shuffle(&mut rng);

    let random_needed = 10 - &forced_kingdom_cards.len();

    let mut kingdom_cards: Vec<KC> = possible_kingdom_cards
        .iter()
        .take(random_needed)
        .cloned()
        .collect();

    kingdom_cards.append(&mut forced_kingdom_cards.iter().cloned().collect());

    if kingdom_cards.len() < 10 {
        return Err(GenSetupError::CouldNotSatisfyKingdomCards);
    }

    let mut bane_card: Option<KC> = None;

    if kingdom_cards.contains(&KC::YoungWitch) {
        bane_card = possible_kingdom_cards
            .iter()
            .skip(random_needed)
            .next()
            .cloned();

        if bane_card.is_none() {
            return Err(GenSetupError::CouldNotSatisfyBaneCard);
        }
    }

    Ok(Setup {
        project_cards,
        kingdom_cards,
        bane_card,
    })
}

pub mod pretty {
    use super::hist::Hist;
    use super::*;
    use chrono::prelude::*;

    pub fn code(name: &str, setup: &Setup) -> String {
        let now_local = Local::now();

        format!(
            "  , Played {{ name = Just \"{} at {}\"
           , at = Just $ Date {{year={}, month={}, day={}}}
           , setup = {}
           , players = Just []
           , rating = Nothing
           }}
",
            name,
            now_local,
            now_local.year(),
            now_local.month(),
            now_local.day(),
            format_setup(setup)
        )
    }

    fn format_card(card: &KC, bane: &Option<KC>) -> String {
        match bane {
            Some(c) => format!(" - {:?} {}", card, if c == card { " (Bane)" } else { "" }),
            None => format!(" - {:?}", card),
        }
    }

    fn kingdom_card_by_expansion_list(setup: &Setup) -> String {
        let mut cards_by_expansion: HashMap<String, String> = HashMap::new();

        let mut cards = setup.cards();

        cards.sort();

        for card in cards {
            let exp = card
                .expansions()
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join("/");

            cards_by_expansion
                .entry(exp)
                .and_modify(|s| {
                    s.push('\n');
                    s.push_str(&format_card(&card, &setup.bane_card))
                })
                .or_insert(format_card(&card, &setup.bane_card));
        }

        let mut kingdom_cards = String::new();

        for (exp, cs) in cards_by_expansion.into_iter() {
            kingdom_cards.push_str(&exp);
            kingdom_cards.push('\n');
            kingdom_cards.push_str(&cs);
            kingdom_cards.push('\n');
        }

        format!(
            "\
._______________.
|               |
| Kingdom Cards |
|_______________|

{}",
            kingdom_cards
        )
    }

    fn project_card_by_expansion_list(project_cards: &Vec<Project>) -> String {
        if project_cards.is_empty() {
            return "".to_string();
        }

        let mut cards_by_expansion: HashMap<String, String> = HashMap::new();

        let mut cards = project_cards.clone();

        cards.sort();

        for card in cards {
            let exp = card
                .expansions()
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join("/");

            cards_by_expansion
                .entry(exp)
                .and_modify(|s| {
                    s.push('\n');
                    s.push_str(&format!(" - {:?}", card))
                })
                .or_insert(format!(" - {:?}", card));
        }

        let mut project_cards = String::new();

        for (exp, cs) in cards_by_expansion.into_iter() {
            project_cards.push_str(&exp);
            project_cards.push('\n');
            project_cards.push_str(&cs);
            project_cards.push('\n');
        }

        format!(
            "\
._______________.
|               |
| Project Cards |
|_______________|

{}",
            project_cards
        )
    }

    pub fn pretty(setup: &Setup) -> String {
        format!(
            "{}\n{}",
            kingdom_card_by_expansion_list(&setup),
            project_card_by_expansion_list(&setup.project_cards)
        )
    }

    pub fn hists(setup: &Setup) -> String {
        let cost_zeros = KC::iter()
            .map(|c| Hist::n(c.base_cost(), 0))
            .fold(Hist::empty(), |s, c| s + c);
        let costs = setup
            .cards()
            .iter()
            .fold(cost_zeros, |s, c| s + Hist::one(c.base_cost()));

        let types_zeros = CardType::iter()
            .map(|t| Hist::n(t, 0))
            .fold(Hist::empty(), |s, c| s + c);
        let types = setup.cards().iter().fold(types_zeros, |s, c| {
            s + c
                .card_types()
                .iter()
                .fold(Hist::empty(), |s, t| s + Hist::one(t.clone()))
        });

        let expansions = setup.cards().iter().fold(Hist::empty(), |s, c| {
            s + c
                .expansions()
                .iter()
                .fold(Hist::empty(), |s, e| s + Hist::one(e.clone()))
        });

        format!("\
Cards' Costs:
-------------
{}

Cards' types:
-------------
{}

Expansions' cards:
-----------------
{}

", costs.pretty(), types.pretty(), expansions.pretty())
    }

    fn format_setup(setup: &Setup) -> String {
        match (&setup.bane_card, setup.project_cards.len()) {
            (None, 0) => format!("S.standard {:?}", setup.kingdom_cards),
            (Some(bane), 0) => format!("S.bane {:?} {:?}", bane, setup.kingdom_cards),
            (None, _) => format!(
                "S.standardWithProjects {:?} {:?}",
                setup.project_cards, setup.kingdom_cards
            ),
            (Some(bane), _) => format!(
                "S.baneWithProjects {:?} {:?} {:?}",
                bane, setup.project_cards, setup.kingdom_cards
            ),
        }
    }

    pub fn gen_error(err: GenSetupError) -> String {
        match err {
            GenSetupError::CouldNotSatisfyProjectsFromExpansions => {
                "The requested project count could not be satisfied! Ensure you're not specifying expansions which preclude projects.".to_string()
            }

            GenSetupError::CouldNotSatisfyKingdomCards => "Could not pick 10 kingdom cards! Ensure your filters don't over-limit cards.".to_string(),

            GenSetupError::CouldNotSatisfyBaneCard => "Could not pick a bane card! Ensure your filters don't over-limit cards.".to_string(),

            GenSetupError::IntersectingCardBansAndIncludes(cards) => format!("I can't ban and include cards! The following exist in the ban and include lists: {:?}", cards),

            GenSetupError::TooManyCardsIncluded => "Too many cards were asked to be included! I currently can't generate a kingdom with more than 10 cards.".to_string(),
        }
    }
}

pub mod hist {
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::ops::Add;

    /// A histogram for counting instances
    pub struct Hist<T> {
        hist: HashMap<T, usize>,
    }

    impl<T> Hist<T> {
        /// No values counted, the empty `Hist`
        ///
        /// ```
        /// let c: dominion::hist::Hist<&str> = dominion::hist::Hist::empty();
        /// assert_eq!(c.count(&"hi"), 0);
        /// ```
        pub fn empty() -> Self {
            Hist {
                hist: HashMap::new(),
            }
        }
    }

    impl<T: Ord + Debug + Hash> Hist<T> {
        pub fn pretty(self: &Self) -> String {
            let mut keys: Vec<_> = self.hist.keys().collect();
            keys.sort();
            let displayed_keys = keys.iter().map(|k| format!("{:?}", k));
            displayed_keys
                .max_by_key(|k| k.len())
                .map(|k| k.len())
                .map(|key_length| {
                    keys.iter()
                        .map(|k| {
                            let n = self.count(k);
                            let k = format!("{:?}", k);
                            format!("{:key_length$}: {:■<n$} ({})", k, "", n)
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or(String::new())
        }
    }

    impl<T: Eq + Hash> Hist<T> {
        /// Count a single instance
        ///
        /// ```
        /// let c = dominion::hist::Hist::one("car".to_string());
        /// assert_eq!(c.count(&"car".to_string()), 1);
        /// ```
        pub fn one(v: T) -> Self {
            Hist::n(v, 1)
        }

        /// Count `n` instances
        ///
        /// ```
        /// let c = dominion::hist::Hist::n("car".to_string(), 5);
        /// assert_eq!(c.count(&"car".to_string()), 5);
        /// ```
        pub fn n(v: T, n: usize) -> Self {
            Hist {
                hist: HashMap::from([(v, n)]),
            }
        }

        /// Get element count
        pub fn count(self: &Self, v: &T) -> usize {
            self.hist.get(&v).unwrap_or(&0).clone()
        }
    }

    impl<T: Clone + Eq + Hash> Add<Hist<T>> for Hist<T> {
        type Output = Hist<T>;

        /// Merge counts of elements
        ///
        /// ```
        /// use dominion::hist::Hist;
        /// let c = Hist::one("car".to_string()) + Hist::one("truck".to_string()) + Hist::one("car".to_string());
        /// assert_eq!(c.count(&"car".to_string()), 2);
        /// assert_eq!(c.count(&"truck".to_string()), 1);
        /// assert_eq!(c.count(&"bus".to_string()), 0);
        /// ```
        fn add(self: Hist<T>, other: Hist<T>) -> Hist<T> {
            let mut hist = self.hist.clone();

            for (v, count) in other.hist {
                hist.entry(v).and_modify(|c| *c += count).or_insert(count);
            }

            Hist { hist }
        }
    }
}
