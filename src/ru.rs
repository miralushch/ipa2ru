use std::iter;
use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Vowels {
    A,  E,  I,  O,  U
}

#[inline]
fn vowels_lookup(vowel: ipa_sounds::Vowels) -> Vowels {
    use ipa_sounds::Vowels::*;
    use Vowels::*;

    match vowel {
        CloseBackRounded            => U,
        CloseBackUnrounded          => U,
        CloseCentralRounded         => U,
        CloseCentralUnrounded       => I,
        CloseFrontRounded           => U,
        CloseFrontUnrounded         => I,
        CloseMidBackRounded         => O,
        CloseMidBackUnrounded       => U,
        CloseMidCentralRounded      => U,
        CloseMidCentralUnrounded    => E,
        CloseMidFrontRounded        => O,
        CloseMidFrontUnrounded      => E,
        MidCentral                  => A,
        NearCloseNearBackRounded    => U,
        NearCloseNearFrontRounded   => U,
        NearCloseNearFrontUnrounded => E,
        NearOpenFrontUrounded       => A,
        OpenBackUnrounded           => A,
        OpenFrontUnrounded          => A,
        OpenMidBackUnrounded        => A,
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Consonants {
    P, B, F, V, K, G,
    T, D, W, X, S, Z,
    L, M, N, R, H, C
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PalatalizedOnlyConsonants {
    J, Q
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phoneme {
    Vowel { phoneme: Vowels },
    Consonant { phoneme: Consonants, is_palatalized: bool },
    PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants },
    Probel
}

#[inline]
fn consonants_lookup(consonant: ipa_sounds::Consonants, is_palatalized: bool) -> Phoneme {
    use PalatalizedOnlyConsonants::*;
    use ipa_sounds::Consonants::*;
    use Consonants::*;
    use Phoneme::*;

    match consonant {
        VoicedAlveolarNasal      => Consonant { phoneme: N, is_palatalized },
        VoicedBilabialNasal      => Consonant { phoneme: M, is_palatalized },
        VoicedPalatalApproximant => PalatalizedOnlyConsonant  { phoneme: J },
        VoicelessBilabialPlosive => Consonant { phoneme: P, is_palatalized },
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhonemeSeq(Vec<Phoneme>);

impl PhonemeSeq {
    fn new(ipa: ipa_sounds::Ipa) -> Self {
        Self (
            ipa.iter()
            .flat_map(|sound| {        
                let (phoneme, is_long) = match *sound {
                    ipa_sounds::Sound::Vowel { phoneme, is_long } => (
                        Phoneme::Vowel { phoneme: vowels_lookup(phoneme) },
                        is_long
                    ),
                    ipa_sounds::Sound::Consonant { phoneme, is_long, is_palatalized } => (
                        consonants_lookup(phoneme, is_palatalized),
                        is_long
                    ),
                    ipa_sounds::Sound::Space => (Phoneme::Probel, false)
                };
                iter::repeat(phoneme).take(is_long as usize + 1)
            })
            .collect::<Vec<Phoneme>>()
        )
    }
}

#[deny(unused_must_use)]
impl fmt::Display for PhonemeSeq {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..self.0.len()).try_for_each(|i| {
            let is_prev_palatalized = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant { phoneme: _, is_palatalized } => is_palatalized,
                    Phoneme::PalatalizedOnlyConsonant { phoneme: _ } => true,
                    Phoneme::Probel => false
                }
            };
            let is_vowel_next = if i == self.0.len() - 1 {
                false
            } else {
                match self.0[i + 1] {
                    Phoneme::Vowel { phoneme: _ } => true,
                    Phoneme::Consonant { phoneme: _, is_palatalized: _ } => false,
                    Phoneme::PalatalizedOnlyConsonant { phoneme: _ } => false,
                    Phoneme::Probel => false
                }
            };
            let is_consonant_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant { phoneme: _, is_palatalized: _ } => true,
                    Phoneme::PalatalizedOnlyConsonant { phoneme: _ } => true,
                    Phoneme::Probel => false
                }
            };
            let is_q_or_wj_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant { phoneme, is_palatalized } => match phoneme {
                        Consonants::W => is_palatalized,
                        _ => false
                    },
                    Phoneme::PalatalizedOnlyConsonant { phoneme } => matches!(phoneme, PalatalizedOnlyConsonants::Q),
                    Phoneme::Probel => false
                }
            };
            write!(formatter, "{}", match self.0[i] {
                Phoneme::Vowel { phoneme } => {
                    let is_vowel_palatalizing = is_prev_palatalized && !is_q_or_wj_prev;
                    match phoneme {
                        Vowels::A => if is_vowel_palatalizing { "я" } else { "а" },
                        Vowels::E => if is_vowel_palatalizing { "е" } else { "э" },
                        Vowels::I => if is_vowel_palatalizing { "и" } else { "ы" },
                        Vowels::O => if is_vowel_palatalizing { "ё" } else { "о" },
                        Vowels::U => if is_vowel_palatalizing { "ю" } else { "у" },
                    }
                },
                Phoneme::Consonant {phoneme, is_palatalized } => {
                    let is_jer = is_palatalized && !is_vowel_next;
                    match phoneme {
                        Consonants::P => if is_jer { "пь" } else { "п" },
                        Consonants::B => if is_jer { "бь" } else { "б" },
                        Consonants::F => if is_jer { "фь" } else { "ф" },
                        Consonants::V => if is_jer { "вь" } else { "в" },
                        Consonants::K => if is_jer { "кь" } else { "к" },
                        Consonants::G => if is_jer { "гь" } else { "г" },
                        Consonants::T => if is_jer { "ть" } else { "т" },
                        Consonants::D => if is_jer { "дь" } else { "д" },
                        Consonants::W => if is_palatalized { "щ" } else { "ш" },
                        Consonants::X => if is_jer { "жь" } else { "ж" },
                        Consonants::S => if is_jer { "сь" } else { "с" },
                        Consonants::Z => if is_jer { "зь" } else { "з" },
                        Consonants::L => if is_jer { "ль" } else { "л" },
                        Consonants::M => if is_jer { "мь" } else { "м" },
                        Consonants::N => if is_jer { "нь" } else { "н" },
                        Consonants::R => if is_jer { "рь" } else { "р" },
                        Consonants::H => if is_jer { "хь" } else { "х" },
                        Consonants::C => if is_jer { "сь" } else { "с" },
                    }
                },
                Phoneme::PalatalizedOnlyConsonant { phoneme } => match phoneme {
                    PalatalizedOnlyConsonants::J => if is_vowel_next && is_consonant_prev {
                        "ъ"
                    } else if !is_vowel_next {
                        "й"
                    } else {
                        ""
                    },
                    PalatalizedOnlyConsonants::Q => "ч"
                },
                Phoneme::Probel => " "
            })
        })
    }
}

#[derive(Clone)]
pub struct Ru(PhonemeSeq);

impl fmt::Display for Ru {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<ipa_sounds::Ipa> for Ru {
    fn from(ipa: ipa_sounds::Ipa) -> Self {
        Self(PhonemeSeq::new(ipa))
    }
}

impl TryFrom<&str> for Ru {
    type Error = ipa_sounds::Error;

    fn try_from(ipa_str: &str) -> Result<Self, Self::Error> {
        ipa_sounds::Ipa::try_from(ipa_str).map(Self::from)
    }
}

impl TryFrom<String> for Ru {
    type Error = ipa_sounds::Error;

    fn try_from(ipa_string: String) -> Result<Self, Self::Error> {
        ipa_sounds::Ipa::try_from(ipa_string).map(Self::from)
    }
}

#[cfg(test)]
mod ru_phoneme_seq_fmt_tests {
    use super::*;

    #[test]
    fn test_na() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::N, is_palatalized: true },
            Phoneme::Vowel { phoneme: Vowels::A },
        ])), "ня");
    }

    #[test]
    fn test_jer() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::P, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::O },
            Phoneme::Consonant { phoneme: Consonants::D, is_palatalized: false },
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
            Phoneme::Vowel { phoneme: Vowels::E },
            Phoneme::Consonant { phoneme: Consonants::Z, is_palatalized: false },
            Phoneme::Consonant { phoneme: Consonants::D, is_palatalized: false },
        ])), "подъезд");
    }

    #[test]
    fn test_huj() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::H, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::U },
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
        ])), "хуй");
    }

    #[test]
    fn test_intervokalnij_jot() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Vowel { phoneme: Vowels::A },
            Phoneme::Consonant { phoneme: Consonants::H, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::U },
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
            Phoneme::Vowel { phoneme: Vowels::E },
            Phoneme::Consonant { phoneme: Consonants::T, is_palatalized: true },
        ])), "ахуеть");
    }

    #[test]
    fn test_naqalnij_jot() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
            Phoneme::Vowel { phoneme: Vowels::E },
            Phoneme::Consonant { phoneme: Consonants::B, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
            Phoneme::Consonant { phoneme: Consonants::T, is_palatalized: true },
        ])), "ебать");
    }

    #[test]
    fn test_wuwa() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::W, is_palatalized: true },
            Phoneme::Vowel { phoneme: Vowels::U },
            Phoneme::Consonant { phoneme: Consonants::W, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
        ])), "щуша");
    }

    #[test]
    fn test_qakra() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::Q },
            Phoneme::Vowel { phoneme: Vowels::A },
            Phoneme::Consonant { phoneme: Consonants::K, is_palatalized: false },
            Phoneme::Consonant { phoneme: Consonants::R, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
        ])), "чакра");
    }
}

#[cfg(test)]
mod ru_integration_tests {
    use super::*;

    #[test]
    fn test_na() {
        assert_eq!(
            Ru::try_from("nʲæ").map(|ru| format!("{}", ru)),
            Ok("ня".to_owned())
        );
    }

    #[test]
    fn test_na_nan() {
        assert_eq!(
            Ru::try_from("nʲæ nʲæn").map(|ru| format!("{}", ru)),
            Ok("ня нян".to_owned())
        );
    }

    #[test]
    fn test_maau() {
        assert_eq!(
            Ru::try_from("mʲæːu").map(|ru| format!("{}", ru)),
            Ok("мяау".to_owned())
        );
    }

    #[test]
    fn test_mmaau() {
        assert_eq!(
            Ru::try_from("mʲːæːu").map(|ru| format!("{}", ru)),
            Ok("мьмяау".to_owned())
        );
    }
}
