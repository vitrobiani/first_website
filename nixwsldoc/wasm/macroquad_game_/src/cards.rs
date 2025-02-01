#[allow(unused_imports, dead_code, unused_variables, unused_mut, unused_macros, unused)]
use std::fmt::format;
// use rand::{rng, seq::SliceRandom};

#[derive(Debug, Clone)]
pub enum Arcana {
    Minor,
    Major,
}

#[derive(Debug, Clone)]
pub enum MajorArcana {
    Fool,
    Magician,
    HighPriestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Strength,
    Hermit,
    TheWheelOfFortune,
    Justice,
    HangedMan,
    Death,
    Temperence,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    World,
}

impl MajorArcana {
    fn iterator() -> impl Iterator<Item = MajorArcana> {
        [
            MajorArcana::Fool,
            MajorArcana::Magician,
            MajorArcana::HighPriestess,
            MajorArcana::Empress,
            MajorArcana::Emperor,
            MajorArcana::Hierophant,
            MajorArcana::Lovers,
            MajorArcana::Chariot,
            MajorArcana::Strength,
            MajorArcana::Hermit,
            MajorArcana::TheWheelOfFortune,
            MajorArcana::Justice,
            MajorArcana::HangedMan,
            MajorArcana::Death,
            MajorArcana::Temperence,
            MajorArcana::Devil,
            MajorArcana::Tower,
            MajorArcana::Star,
            MajorArcana::Moon,
            MajorArcana::Sun,
            MajorArcana::Judgement,
            MajorArcana::World,
        ]
            .iter()
            .cloned()
    }
}

#[derive(Debug, Clone)]
pub enum Suits {
    Wands,
    Cups,
    Swords,
    Pentacles,
}

impl Suits {
    pub fn iterator() -> impl Iterator<Item = Suits> {
        [
            Suits::Wands,
            Suits::Cups,
            Suits::Swords,
            Suits::Pentacles,
        ].iter().cloned()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MinorArcana {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Page,
    Knight,
    Queen,
    King,
}

impl MinorArcana {
    fn iterator() -> impl Iterator<Item = MinorArcana> {
        [
            MinorArcana::Ace,
            MinorArcana::Two,
            MinorArcana::Three,
            MinorArcana::Four,
            MinorArcana::Five,
            MinorArcana::Six,
            MinorArcana::Seven,
            MinorArcana::Eight,
            MinorArcana::Nine,
            MinorArcana::Ten,
            MinorArcana::Page,
            MinorArcana::Knight,
            MinorArcana::Queen,
            MinorArcana::King,
        ].iter().cloned()
    }
}

impl Symbol {
    fn front(&self) -> String {
        match self {
            Symbol::Major(major_arcana) => Symbol::match_major(major_arcana),
            Symbol::Minor(suit, minor_arcana) => match suit {
                Suits::Wands => Symbol::match_wands(minor_arcana),
                Suits::Cups => Symbol::match_cups(minor_arcana),
                Suits::Swords => Symbol::match_swords(minor_arcana),
                Suits::Pentacles => Symbol::match_pentacles(minor_arcana),
            },
        }
    }

    fn match_major(arc: &MajorArcana) -> String {
        match arc {
            MajorArcana::Fool => "00-TheFool.png".to_string(),
            MajorArcana::Magician => "01-TheMagician.png".to_string(),
            MajorArcana::HighPriestess => "02-TheHighPriestess.png".to_string(),
            MajorArcana::Empress => "03-TheEmpress.png".to_string(),
            MajorArcana::Emperor => "04-TheEmperor.png".to_string(),
            MajorArcana::Hierophant => "05-TheHierophant.png".to_string(),
            MajorArcana::Lovers => "06-TheLovers.png".to_string(),
            MajorArcana::Chariot => "07-TheChariot.png".to_string(),
            MajorArcana::Strength => "08-Strength.png".to_string(),
            MajorArcana::Hermit => "09-TheHermit.png".to_string(),
            MajorArcana::TheWheelOfFortune => "10-WheelOfFortune.png".to_string(),
            MajorArcana::Justice => "11-Justice.png".to_string(),
            MajorArcana::HangedMan => "12-TheHangedMan.png".to_string(),
            MajorArcana::Death => "13-Death.png".to_string(),
            MajorArcana::Temperence => "14-Temperance.png".to_string(),
            MajorArcana::Devil => "15-TheDevil.png".to_string(),
            MajorArcana::Tower => "16-TheTower.png".to_string(),
            MajorArcana::Star => "17-TheStar.png".to_string(),
            MajorArcana::Moon => "18-TheMoon.png".to_string(),
            MajorArcana::Sun => "19-TheSun.png".to_string(),
            MajorArcana::Judgement => "20-Judgement.png".to_string(),
            MajorArcana::World => "21-TheWorld.png".to_string(),
        }
    }

    pub fn match_wands(minor: &MinorArcana) -> String {
        match minor {
            MinorArcana::Ace => "Wands01.png".to_string(),
            MinorArcana::Two => "Wands02.png".to_string(),
            MinorArcana::Three => "Wands03.png".to_string(),
            MinorArcana::Four => "Wands04.png".to_string(),
            MinorArcana::Five => "Wands05.png".to_string(),
            MinorArcana::Six => "Wands06.png".to_string(),
            MinorArcana::Seven => "Wands07.png".to_string(),
            MinorArcana::Eight => "Wands08.png".to_string(),
            MinorArcana::Nine => "Wands09.png".to_string(),
            MinorArcana::Ten => "Wands10.png".to_string(),
            MinorArcana::Page => "Wands11.png".to_string(),
            MinorArcana::Knight => "Wands12.png".to_string(),
            MinorArcana::Queen => "Wands13.png".to_string(),
            MinorArcana::King => "Wands14.png".to_string(),
        }
    }

    pub fn match_cups(minor: &MinorArcana) -> String {
        match minor {
            MinorArcana::Ace => "Cups01.png".to_string(),
            MinorArcana::Two => "Cups02.png".to_string(),
            MinorArcana::Three => "Cups03.png".to_string(),
            MinorArcana::Four => "Cups04.png".to_string(),
            MinorArcana::Five => "Cups05.png".to_string(),
            MinorArcana::Six => "Cups06.png".to_string(),
            MinorArcana::Seven => "Cups07.png".to_string(),
            MinorArcana::Eight => "Cups08.png".to_string(),
            MinorArcana::Nine => "Cups09.png".to_string(),
            MinorArcana::Ten => "Cups10.png".to_string(),
            MinorArcana::Page => "Cups11.png".to_string(),
            MinorArcana::Knight => "Cups12.png".to_string(),
            MinorArcana::Queen => "Cups13.png".to_string(),
            MinorArcana::King => "Cups14.png".to_string(),
        }
    }

    pub fn match_swords(minor: &MinorArcana) -> String {
        match minor {
            MinorArcana::Ace => "Swords01.png".to_string(),
            MinorArcana::Two => "Swords02.png".to_string(),
            MinorArcana::Three => "Swords03.png".to_string(),
            MinorArcana::Four => "Swords04.png".to_string(),
            MinorArcana::Five => "Swords05.png".to_string(),
            MinorArcana::Six => "Swords06.png".to_string(),
            MinorArcana::Seven => "Swords07.png".to_string(),
            MinorArcana::Eight => "Swords08.png".to_string(),
            MinorArcana::Nine => "Swords09.png".to_string(),
            MinorArcana::Ten => "Swords10.png".to_string(),
            MinorArcana::Page => "Swords11.png".to_string(),
            MinorArcana::Knight => "Swords12.png".to_string(),
            MinorArcana::Queen => "Swords13.png".to_string(),
            MinorArcana::King => "Swords14.png".to_string(),
        }
    }

    pub fn match_pentacles(minor: &MinorArcana) -> String {
        match minor {
            MinorArcana::Ace => "Pentacles01.png".to_string(),
            MinorArcana::Two => "Pentacles02.png".to_string(),
            MinorArcana::Three => "Pentacles03.png".to_string(),
            MinorArcana::Four => "Pentacles04.png".to_string(),
            MinorArcana::Five => "Pentacles05.png".to_string(),
            MinorArcana::Six => "Pentacles06.png".to_string(),
            MinorArcana::Seven => "Pentacles07.png".to_string(),
            MinorArcana::Eight => "Pentacles08.png".to_string(),
            MinorArcana::Nine => "Pentacles09.png".to_string(),
            MinorArcana::Ten => "Pentacles10.png".to_string(),
            MinorArcana::Page => "Pentacles11.png".to_string(),
            MinorArcana::Knight => "Pentacles12.png".to_string(),
            MinorArcana::Queen => "Pentacles13.png".to_string(),
            MinorArcana::King => "Pentacles14.png".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Major(MajorArcana),
    Minor(Suits, MinorArcana),
}

#[derive(Debug, Clone)]
pub struct Card {
    card_arcana: Arcana,
    card_symbol: Symbol,
    card_front: String,
    card_back: String,
    card_type: String,
    meaning: String,
    upsidedown_meaning: String,
}

impl Card {
    pub fn new(arc: Arcana, symb: Symbol, ct: String) -> Self {
        let front: String = format!("assets/{}/{}", ct, symb.front());
        Card {
            card_arcana: match symb {
                Symbol::Major(_) => Arcana::Major,
                Symbol::Minor(_, _) => Arcana::Minor,
            },
            card_symbol: symb,
            card_back: "assets/Classic/CardBacks.png".to_string(),
            card_front: front,
            card_type: ct,
            meaning: "".to_string(),
            upsidedown_meaning: "".to_string(),
        }
    }

    pub fn front(self) -> String {
        self.card_front
    }

    pub fn back(self) -> String {
        self.card_back
    }
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Deck { cards: Vec::new() }
    }

    pub fn add(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn shuffle(&mut self) {
        // let mut rng = rng();
        // self.cards.shuffle(&mut rng);
    }

    pub fn fill_deck(&mut self) {
        for m in MajorArcana::iterator() {
            self.add(Card::new(
                Arcana::Major,
                Symbol::Major(m),
                "Classic".to_string(),
            ))
        }
        for m in MinorArcana::iterator() {
            for s in Suits::iterator() {
                self.add(Card::new(Arcana::Minor, Symbol::Minor(s,m), "Classic".to_string()));
            }
        }
    }
}
