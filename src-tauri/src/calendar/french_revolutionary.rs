// french_revolution.rs
// Uses French Revolutionary date and displays item associated with date

use crate::models::CalendarDate;
use chrono::{DateTime, Local, NaiveDate, Datelike};

pub struct FrenchRevolutionaryCalendar;

impl super::Calendar for FrenchRevolutionaryCalendar {
    fn convert(&self, date: &DateTime<Local>, _settings: Option<&crate::models::UserSettings>) -> CalendarDate {
        let naive = date.naive_local().date();
        let year = naive.year();

        let vend_start_this = NaiveDate::from_ymd_opt(year, 9, 22).unwrap();
        let (start_of_year, revolutionary_year) = if naive >= vend_start_this {
            (vend_start_this, year - 1792 + 1)
        } else {
            (NaiveDate::from_ymd_opt(year - 1, 9, 22).unwrap(), year - 1792)
        };

        let days_since_start = (naive - start_of_year).num_days();
        let month_index = (days_since_start / 30) as usize;
        let day_in_month = ((days_since_start % 30) + 1) as usize;

        let month_names = [
            "Vendémiaire", "Brumaire", "Frimaire", "Nivôse", "Pluviôse", "Ventôse",
            "Germinal", "Floréal", "Prairial", "Messidor", "Thermidor", "Fructidor",
        ];

        let month_name = if month_index < 12 {
            month_names[month_index].to_string()
        } else {
            "Sansculottides".to_string()
        };

        let date_str = format!("{} {}, An {}", month_name, day_in_month, revolutionary_year);
        let item = get_french_revolutionary_item(month_index, day_in_month);

        CalendarDate {
            system: "French Revolutionary".to_string(),
            date: date_str,
            additional_info: Some(item),
        }
    }
}

fn get_french_revolutionary_item(month: usize, day: usize) -> String {
    match month {
        0 => {
            // Vendémiaire
            let items = [
                "Raisin",
                "Safran",
                "Châtaigne",
                "Colchique",
                "Cheval",
                "Balsamine",
                "Carotte",
                "Amaranthe",
                "Panais",
                "Cuve",
                "Pomme de terre",
                "Immortelle",
                "Potiron",
                "Réséda",
                "Âne",
                "Belle de nuit",
                "Citrouille",
                "Sarrasin",
                "Tournesol",
                "Pressoir",
                "Chanvre",
                "Pêche",
                "Navet",
                "Amaryllis",
                "Bœuf",
                "Aubergine",
                "Piment",
                "Tomate",
                "Orge",
                "Tonneau",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        1 => {
            // Brumaire
            let items = [
                "Pomme",
                "Céleri",
                "Poire",
                "Betterave",
                "Oie",
                "Héliotrope",
                "Figue",
                "Scorsonère",
                "Alisier",
                "Charrue",
                "Salsifis",
                "Mâcre",
                "Topinambour",
                "Endive",
                "Dindon",
                "Chervis",
                "Cresson",
                "Dentelaire",
                "Grenade",
                "Herse",
                "Bacchante",
                "Azerole",
                "Garance",
                "Orange",
                "Faisan",
                "Pistache",
                "Macjonc",
                "Coing",
                "Cormier",
                "Rouleau",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        2 => {
            // Frimaire
            let items = [
                "Raiponce",
                "Turneps",
                "Chicorée",
                "Nèfle",
                "Cochon",
                "Mâche",
                "Chou-fleur",
                "Miel",
                "Genièvre",
                "Pioche",
                "Cire",
                "Raifort",
                "Cèdre",
                "Sapin",
                "Chevreuil",
                "Ajonc",
                "Cyprès",
                "Lierre",
                "Sabine",
                "Hoyau",
                "Érable à sucre",
                "Bruyère",
                "Roseau",
                "Oseille",
                "Grillon",
                "Pignon",
                "Liège",
                "Truffe",
                "Olive",
                "Pelle",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        3 => {
            // Nivôse
            let items = [
                "Tourbe",
                "Houille",
                "Bitume",
                "Soufre",
                "Chien",
                "Lave",
                "Terre végétale",
                "Fumier",
                "Salpêtre",
                "Fléau",
                "Granit",
                "Argile",
                "Ardoise",
                "Grès",
                "Lapin",
                "Silex",
                "Marne",
                "Pierre à chaux",
                "Marbre",
                "Van",
                "Pierre à plâtre",
                "Sel",
                "Fer",
                "Cuivre",
                "Chat",
                "Étain",
                "Plomb",
                "Zinc",
                "Mercure",
                "Crible",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        4 => {
            // Pluviôse
            let items = [
                "Lauréole",
                "Mousse",
                "Fragon",
                "Perce-neige",
                "Taureau",
                "Laurier-thym",
                "Amadouvier",
                "Mézéréon",
                "Peuplier",
                "Coignée",
                "Ellébore",
                "Brocoli",
                "Laurier",
                "Avelinier",
                "Vache",
                "Buis",
                "Lichen",
                "If",
                "Pulmonaire",
                "Serpette",
                "Thlaspi",
                "Thimelé",
                "Chiendent",
                "Trainasse",
                "Lièvre",
                "Guède",
                "Noisetier",
                "Cyclamen",
                "Chélidoine",
                "Traîneau",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        5 => {
            // Ventôse
            let items = [
                "Tussilage",
                "Cornouiller",
                "Violier",
                "Troène",
                "Bouc",
                "Asaret",
                "Alaterne",
                "Violette",
                "Marceau",
                "Bêche",
                "Narcisse",
                "Orme",
                "Fumeterre",
                "Vélar",
                "Chèvre",
                "Épinard",
                "Doronic",
                "Mouron",
                "Cerfeuil",
                "Cordeau",
                "Mandragore",
                "Persil",
                "Cochléaria",
                "Pâquerette",
                "Thon",
                "Pissenlit",
                "Sylvie",
                "Capillaire",
                "Frêne",
                "Plantoir",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        6 => {
            // Germinal
            let items = [
                "Primevère",
                "Platane",
                "Asperge",
                "Tulipe",
                "Poule",
                "Bette",
                "Bouleau",
                "Jonquille",
                "Aulne",
                "Couvoir",
                "Pervenche",
                "Charme",
                "Morille",
                "Hêtre",
                "Abeille",
                "Laitue",
                "Mélèze",
                "Ciguë",
                "Radis",
                "Ruche",
                "Gainier",
                "Romaine",
                "Marronnier",
                "Roquette",
                "Pigeon",
                "Lilas",
                "Anémone",
                "Pensée",
                "Myrtille",
                "Greffoir",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        7 => {
            // Floréal
            let items = [
                "Rose",
                "Chêne",
                "Fougère",
                "Aubépine",
                "Rossignol",
                "Ancolie",
                "Muguet",
                "Champignon",
                "Hyacinthe",
                "Râteau",
                "Rhubarbe",
                "Sainfoin",
                "Bâton d'or",
                "Chamerisier",
                "Ver à soie",
                "Consoude",
                "Pimprenelle",
                "Corbeille d'or",
                "Arroche",
                "Sarcloir",
                "Statice",
                "Fritillaire",
                "Bourrache",
                "Valériane",
                "Carpe",
                "Fusain",
                "Civette",
                "Buglosse",
                "Sénevé",
                "Houlette",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        8 => {
            // Prairial
            let items = [
                "Luzerne",
                "Hémérocalle",
                "Trèfle",
                "Angélique",
                "Canard",
                "Mélisse",
                "Fromental",
                "Martagon",
                "Serpolet",
                "Faux",
                "Fraise",
                "Bétoine",
                "Pois",
                "Acacia",
                "Caille",
                "Œillet",
                "Sureau",
                "Pavot",
                "Tilleul",
                "Fourche",
                "Barbeau",
                "Camomille",
                "Chèvrefeuille",
                "Caille-lait",
                "Tanche",
                "Jasmin",
                "Verveine",
                "Thym",
                "Pivoine",
                "Chariot",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        9 => {
            // Messidor
            let items = [
                "Seigle",
                "Avoine",
                "Oignon",
                "Véronique",
                "Mulet",
                "Romarin",
                "Concombre",
                "Échalote",
                "Absinthe",
                "Faucille",
                "Coriandre",
                "Artichaut",
                "Girofle",
                "Lavande",
                "Chamois",
                "Tabac",
                "Groseille",
                "Gesse",
                "Cerise",
                "Parc",
                "Menthe",
                "Cumin",
                "Haricot",
                "Orcanète",
                "Pintade",
                "Sauge",
                "Ail",
                "Vesce",
                "Blé",
                "Chalémie",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        10 => {
            // Thermidor
            let items = [
                "Épeautre",
                "Bouillon blanc",
                "Melon",
                "Ivraie",
                "Bélier",
                "Prêle",
                "Armoise",
                "Carthame",
                "Mûre",
                "Arrosoir",
                "Panic",
                "Salicorne",
                "Abricot",
                "Basilic",
                "Brebis",
                "Guimauve",
                "Lin",
                "Amande",
                "Gentiane",
                "Écluse",
                "Carline",
                "Câprier",
                "Lentille",
                "Aunée",
                "Loutre",
                "Myrte",
                "Colza",
                "Lupin",
                "Coton",
                "Moulin",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        11 => {
            // Fructidor
            let items = [
                "Prune",
                "Millet",
                "Lycoperdon",
                "Escourgeon",
                "Saumon",
                "Tubéreuse",
                "Sucrion",
                "Apocyn",
                "Réglisse",
                "Échelle",
                "Pastèque",
                "Fenouil",
                "Épine vinette",
                "Noix",
                "Truite",
                "Citron",
                "Cardère",
                "Nerprun",
                "Tagette",
                "Hotte",
                "Églantier",
                "Noisette",
                "Houblon",
                "Sorgho",
                "Écrevisse",
                "Bigarade",
                "Verge d'or",
                "Maïs",
                "Marron",
                "Panier",
            ];
            items.get(day - 1).unwrap_or(&"Item").to_string()
        }
        12 => {
            // Sans-culottides / complementary days
            match day {
                1 => "La Fête de la Vertu".to_string(),
                2 => "La Fête du Génie".to_string(),
                3 => "La Fête du Travail".to_string(),
                4 => "La Fête de l'Opinion".to_string(),
                5 => "La Fête des Récompenses".to_string(),
                6 => "La Fête de la Révolution".to_string(), // leap years only
                _ => "".to_string(),
            }
        }
        _ => format!("{}", day),
    }
}