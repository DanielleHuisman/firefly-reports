pub struct CategoryGroup {
    pub name: &'static str,
    pub category_names: &'static [&'static str],
}

pub static CATEGORY_GROUPS: &[CategoryGroup] = &[
    CategoryGroup {
        name: "Belasting",
        category_names: &["Belasting", "Toeslagen"],
    },
    CategoryGroup {
        name: "Entertainment",
        category_names: &[
            "Adult",
            "Boeken",
            "Concerten",
            "Creators",
            "Films",
            "Games",
            "Musea",
            "Muziek",
            "Natuur",
            "Nieuws",
            "Speelgoed",
            "Theater",
            "Videos",
        ],
    },
    CategoryGroup {
        name: "Eten en drinken",
        category_names: &["Bestellen", "Café", "Restaurant", "Supermarkt"],
    },
    CategoryGroup {
        name: "Financiën",
        category_names: &[
            "Bank",
            "Beleggingen",
            "Rente",
            "Sparen",
            "Studieschuld",
            "Studieschuld rente",
            "Verzekeringen",
        ],
    },
    CategoryGroup {
        name: "Gezamelijke rekening",
        category_names: &[
            "Inleg gezamelijke rekening",
            "Inleg gezamelijke rekening partner",
        ],
    },
    CategoryGroup {
        name: "Kleding",
        category_names: &["Kleding", "Schoenen", "Sierraden", "Tassen"],
    },
    CategoryGroup {
        name: "Sport",
        category_names: &["Personal training"],
    },
    CategoryGroup {
        name: "Techniek",
        category_names: &["Domeinnamen", "Hardware", "Servers", "Software", "Telefoon"],
    },
    CategoryGroup {
        name: "Organisaties",
        category_names: &["Contributie", "Declaraties", "Donaties", "Evenementen"],
    },
    CategoryGroup {
        name: "Vakantie",
        category_names: &["Vakantie"],
    },
    CategoryGroup {
        name: "Vervoer",
        category_names: &[
            "Auto",
            "Brandstof",
            "Deelvervoer",
            "Fiets",
            "Huurauto",
            "Openbaar vervoer",
            "Opslag",
            "Parkeren",
            "Toilet",
            "Wegenwacht",
        ],
    },
    CategoryGroup {
        name: "Verzorging",
        category_names: &["Kapper", "Verzorgingsdiensten", "Verzorgingsproducten"],
    },
    CategoryGroup {
        name: "Werken",
        category_names: &["Bedrijf", "Bedrijfsresultaat", "Pensioen", "Salaris"],
    },
    CategoryGroup {
        name: "Wonen",
        category_names: &[
            "Afval",
            "Energie",
            "Gereedschap",
            "Huishouden",
            "Huur",
            "Interieur",
            "Internet",
            "Kantoorartikelen",
            "Klussen",
            "Tuin",
            "Water",
        ],
    },
    CategoryGroup {
        name: "Zorg",
        category_names: &["Zorgdeclaraties", "Zorgkosten", "Zorgverzekering"],
    },
    CategoryGroup {
        name: "Overig",
        category_names: &["Cadeaus", "Overige diensten", "Verjaardag", "Verzendkosten"],
    },
];
