use std::fmt::{self, Display, Formatter};

use crate::header::location::country::{Country, CountryError};

#[derive(thiserror::Error, Debug)]
pub enum SubregionError {
    #[error("Nonexistent Subregion")]
    NonexistentSubregion,
    #[error("CountryError: {0}")]
    CountryError(#[from] CountryError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Subregion {
    Japan(JapanSubregions),
    Antarctica(AntarcticaSubregions),
    CaribbeanNetherlands(CaribbeanNetherlandsSubregions),
    FalklandIslands(FalklandIslandsSubregions),
    Scotland(ScotlandSubregions),
    Wales(WalesSubregions),
    SintMaarten(SintMaartenSubregions),
    Anguilla(AnguillaSubregions),
    AntiguaAndBarbuda(AntiguaAndBarbudaSubregions),
    Argentina(ArgentinaSubregions),
    Aruba(ArubaSubregions),
    Bahamas(BahamasSubregions),
    Barbados(BarbadosSubregions),
    Belize(BelizeSubregions),
    Bolivia(BoliviaSubregions),
    Brazil(BrazilSubregions),
    BritishVirginIslands(BritishVirginIslandsSubregions),
    Canada(CanadaSubregions),
    CaymanIslands(CaymanIslandsSubregions),
    Chile(ChileSubregions),
    Colombia(ColombiaSubregions),
    CostaRica(CostaRicaSubregions),
    Dominica(DominicaSubregions),
    DominicanRepublic(DominicanRepublicSubregions),
    Ecuador(EcuadorSubregions),
    ElSalvador(ElSalvadorSubregions),
    FrenchGuiana(FrenchGuianaSubregions),
    Grenada(GrenadaSubregions),
    Guadeloupe(GuadeloupeSubregions),
    Guatemala(GuatemalaSubregions),
    Guyana(GuyanaSubregions),
    Haiti(HaitiSubregions),
    Honduras(HondurasSubregions),
    Jamaica(JamaicaSubregions),
    Martinique(MartiniqueSubregions),
    Mexico(MexicoSubregions),
    Montserrat(MontserratSubregions),
    Curacao(CuracaoSubregions),
    Nicaragua(NicaraguaSubregions),
    Panama(PanamaSubregions),
    Paraguay(ParaguaySubregions),
    Peru(PeruSubregions),
    StKittsAndNevis(StKittsAndNevisSubregions),
    StLucia(StLuciaSubregions),
    StVincentAndTheGrenadines(StVincentAndTheGrenadinesSubregions),
    Suriname(SurinameSubregions),
    TrinidadAndTobago(TrinidadAndTobagoSubregions),
    TurksAndCaicosIslands(TurksAndCaicosIslandsSubregions),
    UnitedStates(UnitedStatesSubregions),
    Uruguay(UruguaySubregions),
    USVirginIslands(USVirginIslandsSubregions),
    Venezuela(VenezuelaSubregions),
    Armenia(ArmeniaSubregions),
    Belarus(BelarusSubregions),
    Georgia(GeorgiaSubregions),
    Kosovo(KosovoSubregions),
    Abkhazia(AbkhaziaSubregions),
    Artsakh(ArtsakhSubregions),
    NorthernCyprus(NorthernCyprusSubregions),
    SouthOssetia(SouthOssetiaSubregions),
    Transnistria(TransnistriaSubregions),
    Aland(AlandSubregions),
    FaroeIslands(FaroeIslandsSubregions),
    Albania(AlbaniaSubregions),
    Australia(AustraliaSubregions),
    Austria(AustriaSubregions),
    Belgium(BelgiumSubregions),
    BosniaAndHerzegovina(BosniaAndHerzegovinaSubregions),
    Botswana(BotswanaSubregions),
    Bulgaria(BulgariaSubregions),
    Croatia(CroatiaSubregions),
    Cyprus(CyprusSubregions),
    Czechia(CzechiaSubregions),
    Denmark(DenmarkSubregions),
    Estonia(EstoniaSubregions),
    Finland(FinlandSubregions),
    France(FranceSubregions),
    Germany(GermanySubregions),
    Greece(GreeceSubregions),
    Hungary(HungarySubregions),
    Iceland(IcelandSubregions),
    Ireland(IrelandSubregions),
    Italy(ItalySubregions),
    Latvia(LatviaSubregions),
    Lesotho(LesothoSubregions),
    Liechtenstein(LiechtensteinSubregions),
    Lithuania(LithuaniaSubregions),
    Luxembourg(LuxembourgSubregions),
    NorthMacedonia(NorthMacedoniaSubregions),
    Malta(MaltaSubregions),
    Montenegro(MontenegroSubregions),
    Mozambique(MozambiqueSubregions),
    Namibia(NamibiaSubregions),
    Netherlands(NetherlandsSubregions),
    NewZealand(NewZealandSubregions),
    Norway(NorwaySubregions),
    Poland(PolandSubregions),
    Portugal(PortugalSubregions),
    Romania(RomaniaSubregions),
    Russia(RussiaSubregions),
    Serbia(SerbiaSubregions),
    Slovakia(SlovakiaSubregions),
    Slovenia(SloveniaSubregions),
    SouthAfrica(SouthAfricaSubregions),
    Spain(SpainSubregions),
    Eswatini(EswatiniSubregions),
    Sweden(SwedenSubregions),
    Switzerland(SwitzerlandSubregions),
    Turkey(TurkeySubregions),
    UnitedKingdom(UnitedKingdomSubregions),
    Zambia(ZambiaSubregions),
    Zimbabwe(ZimbabweSubregions),
    Azerbaijan(AzerbaijanSubregions),
    Mauritania(MauritaniaSubregions),
    Mali(MaliSubregions),
    Niger(NigerSubregions),
    Chad(ChadSubregions),
    Sudan(SudanSubregions),
    Eritrea(EritreaSubregions),
    Djibouti(DjiboutiSubregions),
    Somalia(SomaliaSubregions),
    Andorra(AndorraSubregions),
    Gibraltar(GibraltarSubregions),
    Guernsey(GuernseySubregions),
    IsleOfMan(IsleOfManSubregions),
    Jersey(JerseySubregions),
    Monaco(MonacoSubregions),
    Taiwan(TaiwanSubregions),
    Cambodia(CambodiaSubregions),
    Laos(LaosSubregions),
    Mongolia(MongoliaSubregions),
    Myanmar(MyanmarSubregions),
    Nepal(NepalSubregions),
    Vietnam(VietnamSubregions),
    NorthKorea(NorthKoreaSubregions),
    SouthKorea(SouthKoreaSubregions),
    Bangladesh(BangladeshSubregions),
    Bhutan(BhutanSubregions),
    Brunei(BruneiSubregions),
    Maldives(MaldivesSubregions),
    SriLanka(SriLankaSubregions),
    TimorLeste(TimorLesteSubregions),
    BritishIndianOceanTerritory(BritishIndianOceanTerritorySubregions),
    HongKong(HongKongSubregions),
    Macao(MacaoSubregions),
    CookIslands(CookIslandsSubregions),
    Niue(NiueSubregions),
    NorfolkIsland(NorfolkIslandSubregions),
    NorthernMarianaIslands(NorthernMarianaIslandsSubregions),
    AmericanSamoa(AmericanSamoaSubregions),
    Guam(GuamSubregions),
    Indonesia(IndonesiaSubregions),
    Singapore(SingaporeSubregions),
    Thailand(ThailandSubregions),
    Philippines(PhilippinesSubregions),
    Malaysia(MalaysiaSubregions),
    SaintBarthelemy(SaintBarthelemySubregions),
    SaintMartin(SaintMartinSubregions),
    SaintPierreAndMiquelon(SaintPierreAndMiquelonSubregions),
    China(ChinaSubregions),
    Afghanistan(AfghanistanSubregions),
    Kazakhstan(KazakhstanSubregions),
    Kyrgyzstan(KyrgyzstanSubregions),
    Pakistan(PakistanSubregions),
    Tajikistan(TajikistanSubregions),
    Turkmenistan(TurkmenistanSubregions),
    Uzbekistan(UzbekistanSubregions),
    UnitedArabEmirates(UnitedArabEmiratesSubregions),
    India(IndiaSubregions),
    Egypt(EgyptSubregions),
    Oman(OmanSubregions),
    Qatar(QatarSubregions),
    Kuwait(KuwaitSubregions),
    SaudiArabia(SaudiArabiaSubregions),
    Syria(SyriaSubregions),
    Bahrain(BahrainSubregions),
    Jordan(JordanSubregions),
    Iran(IranSubregions),
    Iraq(IraqSubregions),
    Israel(IsraelSubregions),
    Lebanon(LebanonSubregions),
    Palestine(PalestineSubregions),
    Yemen(YemenSubregions),
    SanMarino(SanMarinoSubregions),
    VaticanCity(VaticanCitySubregions),
    Bermuda(BermudaSubregions),
    FrenchPolynesia(FrenchPolynesiaSubregions),
    Reunion(ReunionSubregions),
    Mayotte(MayotteSubregions),
    NewCaledonia(NewCaledoniaSubregions),
    WallisAndFutuna(WallisAndFutunaSubregions),
    Nigeria(NigeriaSubregions),
    Angola(AngolaSubregions),
    Ghana(GhanaSubregions),
    Togo(TogoSubregions),
    Benin(BeninSubregions),
    BurkinaFaso(BurkinaFasoSubregions),
    CoteDIvoire(CoteDIvoireSubregions),
    Liberia(LiberiaSubregions),
    SierraLeone(SierraLeoneSubregions),
    Guinea(GuineaSubregions),
    GuineaBissau(GuineaBissauSubregions),
    Senegal(SenegalSubregions),
    TheGambia(TheGambiaSubregions),
    CapeVerde(CapeVerdeSubregions),
    SaintHelenaAscensionAndTristanDaCunha(SaintHelenaAscensionAndTristanDaCunhaSubregions),
    Moldova(MoldovaSubregions),
    Ukraine(UkraineSubregions),
    Cameroon(CameroonSubregions),
    CentralAfricanRepublic(CentralAfricanRepublicSubregions),
    DemocraticRepublicOfTheCongo(DemocraticRepublicOfTheCongoSubregions),
    RepublicOfTheCongo(RepublicOfTheCongoSubregions),
    EquatorialGuinea(EquatorialGuineaSubregions),
    Gabon(GabonSubregions),
    SaoTomeAndPrincipe(SaoTomeAndPrincipeSubregions),
    Algeria(AlgeriaSubregions),
    Ethiopia(EthiopiaSubregions),
    Libya(LibyaSubregions),
    Morocco(MoroccoSubregions),
    SouthSudan(SouthSudanSubregions),
    Tunisia(TunisiaSubregions),
    SahrawiArabDemocraticRepublic(SahrawiArabDemocraticRepublicSubregions),
    Cuba(CubaSubregions),
    Burundi(BurundiSubregions),
    Comoros(ComorosSubregions),
    Kenya(KenyaSubregions),
    Madagascar(MadagascarSubregions),
    Malawi(MalawiSubregions),
    Mauritius(MauritiusSubregions),
    Rwanda(RwandaSubregions),
    Seychelles(SeychellesSubregions),
    Tanzania(TanzaniaSubregions),
    Uganda(UgandaSubregions),
    FrenchSouthernAndAntarcticLands(FrenchSouthernAndAntarcticLandsSubregions),
    PitcairnIslands(PitcairnIslandsSubregions),
    BritishAntarcticTerritory(BritishAntarcticTerritorySubregions),
    SouthGeorgiaAndTheSouthSandwichIslands(SouthGeorgiaAndTheSouthSandwichIslandsSubregions),
    FederatedStatesOfMicronesia(FederatedStatesOfMicronesiaSubregions),
    Fiji(FijiSubregions),
    Kiribati(KiribatiSubregions),
    MarshallIslands(MarshallIslandsSubregions),
    Nauru(NauruSubregions),
    Palau(PalauSubregions),
    PapuaNewGuinea(PapuaNewGuineaSubregions),
    Samoa(SamoaSubregions),
    SolomonIslands(SolomonIslandsSubregions),
    Tokelau(TokelauSubregions),
    Tonga(TongaSubregions),
    Tuvalu(TuvaluSubregions),
    Vanuatu(VanuatuSubregions),
    ChristmasIsland(ChristmasIslandSubregions),
    CocosKeelingIslands(CocosKeelingIslandsSubregions),
    PuertoRico(PuertoRicoSubregions),
    Greenland(GreenlandSubregions),
    NotSet,
}

impl fmt::Display for Subregion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Subregion::Japan(v) => v.fmt(f),
            Subregion::Antarctica(v) => v.fmt(f),
            Subregion::CaribbeanNetherlands(v) => v.fmt(f),
            Subregion::FalklandIslands(v) => v.fmt(f),
            Subregion::Scotland(v) => v.fmt(f),
            Subregion::Wales(v) => v.fmt(f),
            Subregion::SintMaarten(v) => v.fmt(f),
            Subregion::Anguilla(v) => v.fmt(f),
            Subregion::AntiguaAndBarbuda(v) => v.fmt(f),
            Subregion::Argentina(v) => v.fmt(f),
            Subregion::Aruba(v) => v.fmt(f),
            Subregion::Bahamas(v) => v.fmt(f),
            Subregion::Barbados(v) => v.fmt(f),
            Subregion::Belize(v) => v.fmt(f),
            Subregion::Bolivia(v) => v.fmt(f),
            Subregion::Brazil(v) => v.fmt(f),
            Subregion::BritishVirginIslands(v) => v.fmt(f),
            Subregion::Canada(v) => v.fmt(f),
            Subregion::CaymanIslands(v) => v.fmt(f),
            Subregion::Chile(v) => v.fmt(f),
            Subregion::Colombia(v) => v.fmt(f),
            Subregion::CostaRica(v) => v.fmt(f),
            Subregion::Dominica(v) => v.fmt(f),
            Subregion::DominicanRepublic(v) => v.fmt(f),
            Subregion::Ecuador(v) => v.fmt(f),
            Subregion::ElSalvador(v) => v.fmt(f),
            Subregion::FrenchGuiana(v) => v.fmt(f),
            Subregion::Grenada(v) => v.fmt(f),
            Subregion::Guadeloupe(v) => v.fmt(f),
            Subregion::Guatemala(v) => v.fmt(f),
            Subregion::Guyana(v) => v.fmt(f),
            Subregion::Haiti(v) => v.fmt(f),
            Subregion::Honduras(v) => v.fmt(f),
            Subregion::Jamaica(v) => v.fmt(f),
            Subregion::Martinique(v) => v.fmt(f),
            Subregion::Mexico(v) => v.fmt(f),
            Subregion::Montserrat(v) => v.fmt(f),
            Subregion::Curacao(v) => v.fmt(f),
            Subregion::Nicaragua(v) => v.fmt(f),
            Subregion::Panama(v) => v.fmt(f),
            Subregion::Paraguay(v) => v.fmt(f),
            Subregion::Peru(v) => v.fmt(f),
            Subregion::StKittsAndNevis(v) => v.fmt(f),
            Subregion::StLucia(v) => v.fmt(f),
            Subregion::StVincentAndTheGrenadines(v) => v.fmt(f),
            Subregion::Suriname(v) => v.fmt(f),
            Subregion::TrinidadAndTobago(v) => v.fmt(f),
            Subregion::TurksAndCaicosIslands(v) => v.fmt(f),
            Subregion::UnitedStates(v) => v.fmt(f),
            Subregion::Uruguay(v) => v.fmt(f),
            Subregion::USVirginIslands(v) => v.fmt(f),
            Subregion::Venezuela(v) => v.fmt(f),
            Subregion::Armenia(v) => v.fmt(f),
            Subregion::Belarus(v) => v.fmt(f),
            Subregion::Georgia(v) => v.fmt(f),
            Subregion::Kosovo(v) => v.fmt(f),
            Subregion::Abkhazia(v) => v.fmt(f),
            Subregion::Artsakh(v) => v.fmt(f),
            Subregion::NorthernCyprus(v) => v.fmt(f),
            Subregion::SouthOssetia(v) => v.fmt(f),
            Subregion::Transnistria(v) => v.fmt(f),
            Subregion::Aland(v) => v.fmt(f),
            Subregion::FaroeIslands(v) => v.fmt(f),
            Subregion::Albania(v) => v.fmt(f),
            Subregion::Australia(v) => v.fmt(f),
            Subregion::Austria(v) => v.fmt(f),
            Subregion::Belgium(v) => v.fmt(f),
            Subregion::BosniaAndHerzegovina(v) => v.fmt(f),
            Subregion::Botswana(v) => v.fmt(f),
            Subregion::Bulgaria(v) => v.fmt(f),
            Subregion::Croatia(v) => v.fmt(f),
            Subregion::Cyprus(v) => v.fmt(f),
            Subregion::Czechia(v) => v.fmt(f),
            Subregion::Denmark(v) => v.fmt(f),
            Subregion::Estonia(v) => v.fmt(f),
            Subregion::Finland(v) => v.fmt(f),
            Subregion::France(v) => v.fmt(f),
            Subregion::Germany(v) => v.fmt(f),
            Subregion::Greece(v) => v.fmt(f),
            Subregion::Hungary(v) => v.fmt(f),
            Subregion::Iceland(v) => v.fmt(f),
            Subregion::Ireland(v) => v.fmt(f),
            Subregion::Italy(v) => v.fmt(f),
            Subregion::Latvia(v) => v.fmt(f),
            Subregion::Lesotho(v) => v.fmt(f),
            Subregion::Liechtenstein(v) => v.fmt(f),
            Subregion::Lithuania(v) => v.fmt(f),
            Subregion::Luxembourg(v) => v.fmt(f),
            Subregion::NorthMacedonia(v) => v.fmt(f),
            Subregion::Malta(v) => v.fmt(f),
            Subregion::Montenegro(v) => v.fmt(f),
            Subregion::Mozambique(v) => v.fmt(f),
            Subregion::Namibia(v) => v.fmt(f),
            Subregion::Netherlands(v) => v.fmt(f),
            Subregion::NewZealand(v) => v.fmt(f),
            Subregion::Norway(v) => v.fmt(f),
            Subregion::Poland(v) => v.fmt(f),
            Subregion::Portugal(v) => v.fmt(f),
            Subregion::Romania(v) => v.fmt(f),
            Subregion::Russia(v) => v.fmt(f),
            Subregion::Serbia(v) => v.fmt(f),
            Subregion::Slovakia(v) => v.fmt(f),
            Subregion::Slovenia(v) => v.fmt(f),
            Subregion::SouthAfrica(v) => v.fmt(f),
            Subregion::Spain(v) => v.fmt(f),
            Subregion::Eswatini(v) => v.fmt(f),
            Subregion::Sweden(v) => v.fmt(f),
            Subregion::Switzerland(v) => v.fmt(f),
            Subregion::Turkey(v) => v.fmt(f),
            Subregion::UnitedKingdom(v) => v.fmt(f),
            Subregion::Zambia(v) => v.fmt(f),
            Subregion::Zimbabwe(v) => v.fmt(f),
            Subregion::Azerbaijan(v) => v.fmt(f),
            Subregion::Mauritania(v) => v.fmt(f),
            Subregion::Mali(v) => v.fmt(f),
            Subregion::Niger(v) => v.fmt(f),
            Subregion::Chad(v) => v.fmt(f),
            Subregion::Sudan(v) => v.fmt(f),
            Subregion::Eritrea(v) => v.fmt(f),
            Subregion::Djibouti(v) => v.fmt(f),
            Subregion::Somalia(v) => v.fmt(f),
            Subregion::Andorra(v) => v.fmt(f),
            Subregion::Gibraltar(v) => v.fmt(f),
            Subregion::Guernsey(v) => v.fmt(f),
            Subregion::IsleOfMan(v) => v.fmt(f),
            Subregion::Jersey(v) => v.fmt(f),
            Subregion::Monaco(v) => v.fmt(f),
            Subregion::Taiwan(v) => v.fmt(f),
            Subregion::Cambodia(v) => v.fmt(f),
            Subregion::Laos(v) => v.fmt(f),
            Subregion::Mongolia(v) => v.fmt(f),
            Subregion::Myanmar(v) => v.fmt(f),
            Subregion::Nepal(v) => v.fmt(f),
            Subregion::Vietnam(v) => v.fmt(f),
            Subregion::NorthKorea(v) => v.fmt(f),
            Subregion::SouthKorea(v) => v.fmt(f),
            Subregion::Bangladesh(v) => v.fmt(f),
            Subregion::Bhutan(v) => v.fmt(f),
            Subregion::Brunei(v) => v.fmt(f),
            Subregion::Maldives(v) => v.fmt(f),
            Subregion::SriLanka(v) => v.fmt(f),
            Subregion::TimorLeste(v) => v.fmt(f),
            Subregion::BritishIndianOceanTerritory(v) => v.fmt(f),
            Subregion::HongKong(v) => v.fmt(f),
            Subregion::Macao(v) => v.fmt(f),
            Subregion::CookIslands(v) => v.fmt(f),
            Subregion::Niue(v) => v.fmt(f),
            Subregion::NorfolkIsland(v) => v.fmt(f),
            Subregion::NorthernMarianaIslands(v) => v.fmt(f),
            Subregion::AmericanSamoa(v) => v.fmt(f),
            Subregion::Guam(v) => v.fmt(f),
            Subregion::Indonesia(v) => v.fmt(f),
            Subregion::Singapore(v) => v.fmt(f),
            Subregion::Thailand(v) => v.fmt(f),
            Subregion::Philippines(v) => v.fmt(f),
            Subregion::Malaysia(v) => v.fmt(f),
            Subregion::SaintBarthelemy(v) => v.fmt(f),
            Subregion::SaintMartin(v) => v.fmt(f),
            Subregion::SaintPierreAndMiquelon(v) => v.fmt(f),
            Subregion::China(v) => v.fmt(f),
            Subregion::Afghanistan(v) => v.fmt(f),
            Subregion::Kazakhstan(v) => v.fmt(f),
            Subregion::Kyrgyzstan(v) => v.fmt(f),
            Subregion::Pakistan(v) => v.fmt(f),
            Subregion::Tajikistan(v) => v.fmt(f),
            Subregion::Turkmenistan(v) => v.fmt(f),
            Subregion::Uzbekistan(v) => v.fmt(f),
            Subregion::UnitedArabEmirates(v) => v.fmt(f),
            Subregion::India(v) => v.fmt(f),
            Subregion::Egypt(v) => v.fmt(f),
            Subregion::Oman(v) => v.fmt(f),
            Subregion::Qatar(v) => v.fmt(f),
            Subregion::Kuwait(v) => v.fmt(f),
            Subregion::SaudiArabia(v) => v.fmt(f),
            Subregion::Syria(v) => v.fmt(f),
            Subregion::Bahrain(v) => v.fmt(f),
            Subregion::Jordan(v) => v.fmt(f),
            Subregion::Iran(v) => v.fmt(f),
            Subregion::Iraq(v) => v.fmt(f),
            Subregion::Israel(v) => v.fmt(f),
            Subregion::Lebanon(v) => v.fmt(f),
            Subregion::Palestine(v) => v.fmt(f),
            Subregion::Yemen(v) => v.fmt(f),
            Subregion::SanMarino(v) => v.fmt(f),
            Subregion::VaticanCity(v) => v.fmt(f),
            Subregion::Bermuda(v) => v.fmt(f),
            Subregion::FrenchPolynesia(v) => v.fmt(f),
            Subregion::Reunion(v) => v.fmt(f),
            Subregion::Mayotte(v) => v.fmt(f),
            Subregion::NewCaledonia(v) => v.fmt(f),
            Subregion::WallisAndFutuna(v) => v.fmt(f),
            Subregion::Nigeria(v) => v.fmt(f),
            Subregion::Angola(v) => v.fmt(f),
            Subregion::Ghana(v) => v.fmt(f),
            Subregion::Togo(v) => v.fmt(f),
            Subregion::Benin(v) => v.fmt(f),
            Subregion::BurkinaFaso(v) => v.fmt(f),
            Subregion::CoteDIvoire(v) => v.fmt(f),
            Subregion::Liberia(v) => v.fmt(f),
            Subregion::SierraLeone(v) => v.fmt(f),
            Subregion::Guinea(v) => v.fmt(f),
            Subregion::GuineaBissau(v) => v.fmt(f),
            Subregion::Senegal(v) => v.fmt(f),
            Subregion::TheGambia(v) => v.fmt(f),
            Subregion::CapeVerde(v) => v.fmt(f),
            Subregion::SaintHelenaAscensionAndTristanDaCunha(v) => v.fmt(f),
            Subregion::Moldova(v) => v.fmt(f),
            Subregion::Ukraine(v) => v.fmt(f),
            Subregion::Cameroon(v) => v.fmt(f),
            Subregion::CentralAfricanRepublic(v) => v.fmt(f),
            Subregion::DemocraticRepublicOfTheCongo(v) => v.fmt(f),
            Subregion::RepublicOfTheCongo(v) => v.fmt(f),
            Subregion::EquatorialGuinea(v) => v.fmt(f),
            Subregion::Gabon(v) => v.fmt(f),
            Subregion::SaoTomeAndPrincipe(v) => v.fmt(f),
            Subregion::Algeria(v) => v.fmt(f),
            Subregion::Ethiopia(v) => v.fmt(f),
            Subregion::Libya(v) => v.fmt(f),
            Subregion::Morocco(v) => v.fmt(f),
            Subregion::SouthSudan(v) => v.fmt(f),
            Subregion::Tunisia(v) => v.fmt(f),
            Subregion::SahrawiArabDemocraticRepublic(v) => v.fmt(f),
            Subregion::Cuba(v) => v.fmt(f),
            Subregion::Burundi(v) => v.fmt(f),
            Subregion::Comoros(v) => v.fmt(f),
            Subregion::Kenya(v) => v.fmt(f),
            Subregion::Madagascar(v) => v.fmt(f),
            Subregion::Malawi(v) => v.fmt(f),
            Subregion::Mauritius(v) => v.fmt(f),
            Subregion::Rwanda(v) => v.fmt(f),
            Subregion::Seychelles(v) => v.fmt(f),
            Subregion::Tanzania(v) => v.fmt(f),
            Subregion::Uganda(v) => v.fmt(f),
            Subregion::FrenchSouthernAndAntarcticLands(v) => v.fmt(f),
            Subregion::PitcairnIslands(v) => v.fmt(f),
            Subregion::BritishAntarcticTerritory(v) => v.fmt(f),
            Subregion::SouthGeorgiaAndTheSouthSandwichIslands(v) => v.fmt(f),
            Subregion::FederatedStatesOfMicronesia(v) => v.fmt(f),
            Subregion::Fiji(v) => v.fmt(f),
            Subregion::Kiribati(v) => v.fmt(f),
            Subregion::MarshallIslands(v) => v.fmt(f),
            Subregion::Nauru(v) => v.fmt(f),
            Subregion::Palau(v) => v.fmt(f),
            Subregion::PapuaNewGuinea(v) => v.fmt(f),
            Subregion::Samoa(v) => v.fmt(f),
            Subregion::SolomonIslands(v) => v.fmt(f),
            Subregion::Tokelau(v) => v.fmt(f),
            Subregion::Tonga(v) => v.fmt(f),
            Subregion::Tuvalu(v) => v.fmt(f),
            Subregion::Vanuatu(v) => v.fmt(f),
            Subregion::ChristmasIsland(v) => v.fmt(f),
            Subregion::CocosKeelingIslands(v) => v.fmt(f),
            Subregion::PuertoRico(v) => v.fmt(f),
            Subregion::Greenland(v) => v.fmt(f),

            Subregion::NotSet => write!(f, "Not Set"),
        }
    }
}

macro_rules! map_subregion {
    ($country:expr, $sub_id:expr, {
        $(
            $c:ident => $sr:ident
        ),+ $(,)?
    }) => {
        match $country {
            $(
                Country::$c => Subregion::$c($sr::try_from($sub_id)?),
            )+
            Country::NotSet => Subregion::NotSet,
        }
    };
}

impl Subregion {
    pub fn new(country: Country, subregion_id: u8) -> Result<Self, SubregionError> {
        let subregion = map_subregion!(country, subregion_id, {
            Japan => JapanSubregions,
            Antarctica => AntarcticaSubregions,
            CaribbeanNetherlands => CaribbeanNetherlandsSubregions,
            FalklandIslands => FalklandIslandsSubregions,
            Scotland => ScotlandSubregions,
            Wales => WalesSubregions,
            SintMaarten => SintMaartenSubregions,
            Anguilla => AnguillaSubregions,
            AntiguaAndBarbuda => AntiguaAndBarbudaSubregions,
            // Finish the rest of this!
            Argentina => ArgentinaSubregions,
            Aruba => ArubaSubregions,
            Bahamas => BahamasSubregions,
            Barbados => BarbadosSubregions,
            Belize => BelizeSubregions,
            Bolivia => BoliviaSubregions,
            Brazil => BrazilSubregions,
            BritishVirginIslands => BritishVirginIslandsSubregions,
            Canada => CanadaSubregions,
            CaymanIslands => CaymanIslandsSubregions,
            Chile => ChileSubregions,
            Colombia => ColombiaSubregions,
            CostaRica => CostaRicaSubregions,
            Dominica => DominicaSubregions,
            DominicanRepublic => DominicanRepublicSubregions,
            Ecuador => EcuadorSubregions,
            ElSalvador => ElSalvadorSubregions,
            FrenchGuiana => FrenchGuianaSubregions,
            Grenada => GrenadaSubregions,
            Guadeloupe => GuadeloupeSubregions,
            Guatemala => GuatemalaSubregions,
            Guyana => GuyanaSubregions,
            Haiti => HaitiSubregions,
            Honduras => HondurasSubregions,
            Jamaica => JamaicaSubregions,
            Martinique => MartiniqueSubregions,
            Mexico => MexicoSubregions,
            Montserrat => MontserratSubregions,
            Curacao => CuracaoSubregions,
            Nicaragua => NicaraguaSubregions,
            Panama => PanamaSubregions,
            Paraguay => ParaguaySubregions,
            Peru => PeruSubregions,
            StKittsAndNevis => StKittsAndNevisSubregions,
            StLucia => StLuciaSubregions,
            StVincentAndTheGrenadines => StVincentAndTheGrenadinesSubregions,
            Suriname => SurinameSubregions,
            TrinidadAndTobago => TrinidadAndTobagoSubregions,
            TurksAndCaicosIslands => TurksAndCaicosIslandsSubregions,
            UnitedStates => UnitedStatesSubregions,
            Uruguay => UruguaySubregions,
            USVirginIslands => USVirginIslandsSubregions,
            Venezuela => VenezuelaSubregions,
            Armenia => ArmeniaSubregions,
            Belarus => BelarusSubregions,
            Georgia => GeorgiaSubregions,
            Kosovo => KosovoSubregions,
            Abkhazia => AbkhaziaSubregions,
            Artsakh => ArtsakhSubregions,
            NorthernCyprus => NorthernCyprusSubregions,
            SouthOssetia => SouthOssetiaSubregions,
            Transnistria => TransnistriaSubregions,
            Aland => AlandSubregions,
            FaroeIslands => FaroeIslandsSubregions,
            Albania => AlbaniaSubregions,
            Australia => AustraliaSubregions,
            Austria => AustriaSubregions,
            Belgium => BelgiumSubregions,
            BosniaAndHerzegovina => BosniaAndHerzegovinaSubregions,
            Botswana => BotswanaSubregions,
            Bulgaria => BulgariaSubregions,
            Croatia => CroatiaSubregions,
            Cyprus => CyprusSubregions,
            Czechia => CzechiaSubregions,
            Denmark => DenmarkSubregions,
            Estonia => EstoniaSubregions,
            Finland => FinlandSubregions,
            France => FranceSubregions,
            Germany => GermanySubregions,
            Greece => GreeceSubregions,
            Hungary => HungarySubregions,
            Iceland => IcelandSubregions,
            Ireland => IrelandSubregions,
            Italy => ItalySubregions,
            Latvia => LatviaSubregions,
            Lesotho => LesothoSubregions,
            Liechtenstein => LiechtensteinSubregions,
            Lithuania => LithuaniaSubregions,
            Luxembourg => LuxembourgSubregions,
            NorthMacedonia => NorthMacedoniaSubregions,
            Malta => MaltaSubregions,
            Montenegro => MontenegroSubregions,
            Mozambique => MozambiqueSubregions,
            Namibia => NamibiaSubregions,
            Netherlands => NetherlandsSubregions,
            NewZealand => NewZealandSubregions,
            Norway => NorwaySubregions,
            Poland => PolandSubregions,
            Portugal => PortugalSubregions,
            Romania => RomaniaSubregions,
            Russia => RussiaSubregions,
            Serbia => SerbiaSubregions,
            Slovakia => SlovakiaSubregions,
            Slovenia => SloveniaSubregions,
            SouthAfrica => SouthAfricaSubregions,
            Spain => SpainSubregions,
            Eswatini => EswatiniSubregions,
            Sweden => SwedenSubregions,
            Switzerland => SwitzerlandSubregions,
            Turkey => TurkeySubregions,
            UnitedKingdom => UnitedKingdomSubregions,
            Zambia => ZambiaSubregions,
            Zimbabwe => ZimbabweSubregions,
            Azerbaijan => AzerbaijanSubregions,
            Mauritania => MauritaniaSubregions,
            Mali => MaliSubregions,
            Niger => NigerSubregions,
            Chad => ChadSubregions,
            Sudan => SudanSubregions,
            Eritrea => EritreaSubregions,
            Djibouti => DjiboutiSubregions,
            Somalia => SomaliaSubregions,
            Andorra => AndorraSubregions,
            Gibraltar => GibraltarSubregions,
            Guernsey => GuernseySubregions,
            IsleOfMan => IsleOfManSubregions,
            Jersey => JerseySubregions,
            Monaco => MonacoSubregions,
            Taiwan => TaiwanSubregions,
            Cambodia => CambodiaSubregions,
            Laos => LaosSubregions,
            Mongolia => MongoliaSubregions,
            Myanmar => MyanmarSubregions,
            Nepal => NepalSubregions,
            Vietnam => VietnamSubregions,
            NorthKorea => NorthKoreaSubregions,
            SouthKorea => SouthKoreaSubregions,
            Bangladesh => BangladeshSubregions,
            Bhutan => BhutanSubregions,
            Brunei => BruneiSubregions,
            Maldives => MaldivesSubregions,
            SriLanka => SriLankaSubregions,
            TimorLeste => TimorLesteSubregions,
            BritishIndianOceanTerritory => BritishIndianOceanTerritorySubregions,
            HongKong => HongKongSubregions,
            Macao => MacaoSubregions,
            CookIslands => CookIslandsSubregions,
            Niue => NiueSubregions,
            NorfolkIsland => NorfolkIslandSubregions,
            NorthernMarianaIslands => NorthernMarianaIslandsSubregions,
            AmericanSamoa => AmericanSamoaSubregions,
            Guam => GuamSubregions,
            Indonesia => IndonesiaSubregions,
            Singapore => SingaporeSubregions,
            Thailand => ThailandSubregions,
            Philippines => PhilippinesSubregions,
            Malaysia => MalaysiaSubregions,
            SaintBarthelemy => SaintBarthelemySubregions,
            SaintMartin => SaintMartinSubregions,
            SaintPierreAndMiquelon => SaintPierreAndMiquelonSubregions,
            China => ChinaSubregions,
            Afghanistan => AfghanistanSubregions,
            Kazakhstan => KazakhstanSubregions,
            Kyrgyzstan => KyrgyzstanSubregions,
            Pakistan => PakistanSubregions,
            Tajikistan => TajikistanSubregions,
            Turkmenistan => TurkmenistanSubregions,
            Uzbekistan => UzbekistanSubregions,
            UnitedArabEmirates => UnitedArabEmiratesSubregions,
            India => IndiaSubregions,
            Egypt => EgyptSubregions,
            Oman => OmanSubregions,
            Qatar => QatarSubregions,
            Kuwait => KuwaitSubregions,
            SaudiArabia => SaudiArabiaSubregions,
            Syria => SyriaSubregions,
            Bahrain => BahrainSubregions,
            Jordan => JordanSubregions,
            Iran => IranSubregions,
            Iraq => IraqSubregions,
            Israel => IsraelSubregions,
            Lebanon => LebanonSubregions,
            Palestine => PalestineSubregions,
            Yemen => YemenSubregions,
            SanMarino => SanMarinoSubregions,
            VaticanCity => VaticanCitySubregions,
            Bermuda => BermudaSubregions,
            FrenchPolynesia => FrenchPolynesiaSubregions,
            Reunion => ReunionSubregions,
            Mayotte => MayotteSubregions,
            NewCaledonia => NewCaledoniaSubregions,
            WallisAndFutuna => WallisAndFutunaSubregions,
            Nigeria => NigeriaSubregions,
            Angola => AngolaSubregions,
            Ghana => GhanaSubregions,
            Togo => TogoSubregions,
            Benin => BeninSubregions,
            BurkinaFaso => BurkinaFasoSubregions,
            CoteDIvoire => CoteDIvoireSubregions,
            Liberia => LiberiaSubregions,
            SierraLeone => SierraLeoneSubregions,
            Guinea => GuineaSubregions,
            GuineaBissau => GuineaBissauSubregions,
            Senegal => SenegalSubregions,
            TheGambia => TheGambiaSubregions,
            CapeVerde => CapeVerdeSubregions,
            SaintHelenaAscensionAndTristanDaCunha => SaintHelenaAscensionAndTristanDaCunhaSubregions,
            Moldova => MoldovaSubregions,
            Ukraine => UkraineSubregions,
            Cameroon => CameroonSubregions,
            CentralAfricanRepublic => CentralAfricanRepublicSubregions,
            DemocraticRepublicOfTheCongo => DemocraticRepublicOfTheCongoSubregions,
            RepublicOfTheCongo => RepublicOfTheCongoSubregions,
            EquatorialGuinea => EquatorialGuineaSubregions,
            Gabon => GabonSubregions,
            SaoTomeAndPrincipe => SaoTomeAndPrincipeSubregions,
            Algeria => AlgeriaSubregions,
            Ethiopia => EthiopiaSubregions,
            Libya => LibyaSubregions,
            Morocco => MoroccoSubregions,
            SouthSudan => SouthSudanSubregions,
            Tunisia => TunisiaSubregions,
            SahrawiArabDemocraticRepublic => SahrawiArabDemocraticRepublicSubregions,
            Cuba => CubaSubregions,
            Burundi => BurundiSubregions,
            Comoros => ComorosSubregions,
            Kenya => KenyaSubregions,
            Madagascar => MadagascarSubregions,
            Malawi => MalawiSubregions,
            Mauritius => MauritiusSubregions,
            Rwanda => RwandaSubregions,
            Seychelles => SeychellesSubregions,
            Tanzania => TanzaniaSubregions,
            Uganda => UgandaSubregions,
            FrenchSouthernAndAntarcticLands => FrenchSouthernAndAntarcticLandsSubregions,
            PitcairnIslands => PitcairnIslandsSubregions,
            BritishAntarcticTerritory => BritishAntarcticTerritorySubregions,
            SouthGeorgiaAndTheSouthSandwichIslands => SouthGeorgiaAndTheSouthSandwichIslandsSubregions,
            FederatedStatesOfMicronesia => FederatedStatesOfMicronesiaSubregions,
            Fiji => FijiSubregions,
            Kiribati => KiribatiSubregions,
            MarshallIslands => MarshallIslandsSubregions,
            Nauru => NauruSubregions,
            Palau => PalauSubregions,
            PapuaNewGuinea => PapuaNewGuineaSubregions,
            Samoa => SamoaSubregions,
            SolomonIslands => SolomonIslandsSubregions,
            Tokelau => TokelauSubregions,
            Tonga => TongaSubregions,
            Tuvalu => TuvaluSubregions,
            Vanuatu => VanuatuSubregions,
            ChristmasIsland => ChristmasIslandSubregions,
            CocosKeelingIslands => CocosKeelingIslandsSubregions,
            PuertoRico => PuertoRicoSubregions,
            Greenland => GreenlandSubregions,
        });

        Ok(subregion)
    }
}

// Abkhazia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbkhaziaSubregions {
    Abkhazia,
    SukhumiDistrict,
    GagraDistrict,
    GaliDistrict,
    GudautaDistrict,
    GulripshiDistrict,
    OchamchiraDistrict,
    TkvarcheliDistrict,
}

impl TryFrom<u8> for AbkhaziaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Abkhazia),
            0x02 => Ok(Self::SukhumiDistrict),
            0x03 => Ok(Self::GagraDistrict),
            0x04 => Ok(Self::GaliDistrict),
            0x05 => Ok(Self::GudautaDistrict),
            0x06 => Ok(Self::GulripshiDistrict),
            0x07 => Ok(Self::OchamchiraDistrict),
            0x08 => Ok(Self::TkvarcheliDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AbkhaziaSubregions> for u8 {
    fn from(value: AbkhaziaSubregions) -> u8 {
        match value {
            AbkhaziaSubregions::Abkhazia => 0x01,
            AbkhaziaSubregions::SukhumiDistrict => 0x02,
            AbkhaziaSubregions::GagraDistrict => 0x03,
            AbkhaziaSubregions::GaliDistrict => 0x04,
            AbkhaziaSubregions::GudautaDistrict => 0x05,
            AbkhaziaSubregions::GulripshiDistrict => 0x06,
            AbkhaziaSubregions::OchamchiraDistrict => 0x07,
            AbkhaziaSubregions::TkvarcheliDistrict => 0x08,
        }
    }
}

impl Display for AbkhaziaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abkhazia => write!(f, "Abkhazia"),
            Self::SukhumiDistrict => write!(f, "Sukhumi District"),
            Self::GagraDistrict => write!(f, "Gagra District"),
            Self::GaliDistrict => write!(f, "Gali District"),
            Self::GudautaDistrict => write!(f, "Gudauta District"),
            Self::GulripshiDistrict => write!(f, "Gulripshi District"),
            Self::OchamchiraDistrict => write!(f, "Ochamchira District"),
            Self::TkvarcheliDistrict => write!(f, "Tkvarcheli District"),
        }
    }
}

// Afghanistan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AfghanistanSubregions {
    Afghanistan,
    Kabul,
    Badakhshan,
    Badghis,
    Baghlan,
    Balkh,
    Bamyan,
    Daykundi,
    Farah,
    Faryab,
    Ghazni,
    Ghor,
    Helmand,
    Herat,
    Jowzjan,
    Kandahar,
    Kapisa,
    Khost,
    Kunar,
    Kunduz,
    Laghman,
    Logar,
    Nangarhar,
    Nimruz,
    Nuristan,
    Paktia,
    Paktika,
    Panjshir,
    Parwan,
    Samangan,
    SarEPol,
    Takhar,
    Urozgan,
    Wardak,
    Zabul,
}

impl TryFrom<u8> for AfghanistanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Afghanistan),
            0x02 => Ok(Self::Kabul),
            0x03 => Ok(Self::Badakhshan),
            0x04 => Ok(Self::Badghis),
            0x05 => Ok(Self::Baghlan),
            0x06 => Ok(Self::Balkh),
            0x07 => Ok(Self::Bamyan),
            0x08 => Ok(Self::Daykundi),
            0x09 => Ok(Self::Farah),
            0x0A => Ok(Self::Faryab),
            0x0B => Ok(Self::Ghazni),
            0x0C => Ok(Self::Ghor),
            0x0D => Ok(Self::Helmand),
            0x0E => Ok(Self::Herat),
            0x0F => Ok(Self::Jowzjan),
            0x10 => Ok(Self::Kandahar),
            0x11 => Ok(Self::Kapisa),
            0x12 => Ok(Self::Khost),
            0x13 => Ok(Self::Kunar),
            0x14 => Ok(Self::Kunduz),
            0x15 => Ok(Self::Laghman),
            0x16 => Ok(Self::Logar),
            0x17 => Ok(Self::Nangarhar),
            0x18 => Ok(Self::Nimruz),
            0x19 => Ok(Self::Nuristan),
            0x1A => Ok(Self::Paktia),
            0x1B => Ok(Self::Paktika),
            0x1C => Ok(Self::Panjshir),
            0x1D => Ok(Self::Parwan),
            0x1E => Ok(Self::Samangan),
            0x1F => Ok(Self::SarEPol),
            0x20 => Ok(Self::Takhar),
            0x21 => Ok(Self::Urozgan),
            0x22 => Ok(Self::Wardak),
            0x23 => Ok(Self::Zabul),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AfghanistanSubregions> for u8 {
    fn from(value: AfghanistanSubregions) -> u8 {
        match value {
            AfghanistanSubregions::Afghanistan => 0x01,
            AfghanistanSubregions::Kabul => 0x02,
            AfghanistanSubregions::Badakhshan => 0x03,
            AfghanistanSubregions::Badghis => 0x04,
            AfghanistanSubregions::Baghlan => 0x05,
            AfghanistanSubregions::Balkh => 0x06,
            AfghanistanSubregions::Bamyan => 0x07,
            AfghanistanSubregions::Daykundi => 0x08,
            AfghanistanSubregions::Farah => 0x09,
            AfghanistanSubregions::Faryab => 0x0A,
            AfghanistanSubregions::Ghazni => 0x0B,
            AfghanistanSubregions::Ghor => 0x0C,
            AfghanistanSubregions::Helmand => 0x0D,
            AfghanistanSubregions::Herat => 0x0E,
            AfghanistanSubregions::Jowzjan => 0x0F,
            AfghanistanSubregions::Kandahar => 0x10,
            AfghanistanSubregions::Kapisa => 0x11,
            AfghanistanSubregions::Khost => 0x12,
            AfghanistanSubregions::Kunar => 0x13,
            AfghanistanSubregions::Kunduz => 0x14,
            AfghanistanSubregions::Laghman => 0x15,
            AfghanistanSubregions::Logar => 0x16,
            AfghanistanSubregions::Nangarhar => 0x17,
            AfghanistanSubregions::Nimruz => 0x18,
            AfghanistanSubregions::Nuristan => 0x19,
            AfghanistanSubregions::Paktia => 0x1A,
            AfghanistanSubregions::Paktika => 0x1B,
            AfghanistanSubregions::Panjshir => 0x1C,
            AfghanistanSubregions::Parwan => 0x1D,
            AfghanistanSubregions::Samangan => 0x1E,
            AfghanistanSubregions::SarEPol => 0x1F,
            AfghanistanSubregions::Takhar => 0x20,
            AfghanistanSubregions::Urozgan => 0x21,
            AfghanistanSubregions::Wardak => 0x22,
            AfghanistanSubregions::Zabul => 0x23,
        }
    }
}

impl Display for AfghanistanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Afghanistan => write!(f, "Afghanistan"),
            Self::Kabul => write!(f, "Kabul"),
            Self::Badakhshan => write!(f, "Badakhshan"),
            Self::Badghis => write!(f, "Badghis"),
            Self::Baghlan => write!(f, "Baghlan"),
            Self::Balkh => write!(f, "Balkh"),
            Self::Bamyan => write!(f, "Bamyan"),
            Self::Daykundi => write!(f, "Daykundi"),
            Self::Farah => write!(f, "Farah"),
            Self::Faryab => write!(f, "Faryab"),
            Self::Ghazni => write!(f, "Ghazni"),
            Self::Ghor => write!(f, "Ghor"),
            Self::Helmand => write!(f, "Helmand"),
            Self::Herat => write!(f, "Herat"),
            Self::Jowzjan => write!(f, "Jowzjan"),
            Self::Kandahar => write!(f, "Kandahar"),
            Self::Kapisa => write!(f, "Kapisa"),
            Self::Khost => write!(f, "Khost"),
            Self::Kunar => write!(f, "Kunar"),
            Self::Kunduz => write!(f, "Kunduz"),
            Self::Laghman => write!(f, "Laghman"),
            Self::Logar => write!(f, "Logar"),
            Self::Nangarhar => write!(f, "Nangarhar"),
            Self::Nimruz => write!(f, "Nimruz"),
            Self::Nuristan => write!(f, "Nuristan"),
            Self::Paktia => write!(f, "Paktia"),
            Self::Paktika => write!(f, "Paktika"),
            Self::Panjshir => write!(f, "Panjshir"),
            Self::Parwan => write!(f, "Parwan"),
            Self::Samangan => write!(f, "Samangan"),
            Self::SarEPol => write!(f, "Sar-e Pol"),
            Self::Takhar => write!(f, "Takhar"),
            Self::Urozgan => write!(f, "Urozgan"),
            Self::Wardak => write!(f, "Wardak"),
            Self::Zabul => write!(f, "Zabul"),
        }
    }
}

// Albania Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlbaniaSubregions {
    Albania,
    Tirana,
    Berat,
    Diber,
    Durres,
    Elbasan,
    Fier,
    Gjirokaster,
    Korce,
    Kukes,
    Lezhe,
    Shkoder,
    Vlore,
}

impl TryFrom<u8> for AlbaniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Albania),
            0x02 => Ok(Self::Tirana),
            0x03 => Ok(Self::Berat),
            0x04 => Ok(Self::Diber),
            0x05 => Ok(Self::Durres),
            0x06 => Ok(Self::Elbasan),
            0x07 => Ok(Self::Fier),
            0x08 => Ok(Self::Gjirokaster),
            0x09 => Ok(Self::Korce),
            0x0A => Ok(Self::Kukes),
            0x0B => Ok(Self::Lezhe),
            0x0C => Ok(Self::Shkoder),
            0x0D => Ok(Self::Vlore),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AlbaniaSubregions> for u8 {
    fn from(value: AlbaniaSubregions) -> u8 {
        match value {
            AlbaniaSubregions::Albania => 0x01,
            AlbaniaSubregions::Tirana => 0x02,
            AlbaniaSubregions::Berat => 0x03,
            AlbaniaSubregions::Diber => 0x04,
            AlbaniaSubregions::Durres => 0x05,
            AlbaniaSubregions::Elbasan => 0x06,
            AlbaniaSubregions::Fier => 0x07,
            AlbaniaSubregions::Gjirokaster => 0x08,
            AlbaniaSubregions::Korce => 0x09,
            AlbaniaSubregions::Kukes => 0x0A,
            AlbaniaSubregions::Lezhe => 0x0B,
            AlbaniaSubregions::Shkoder => 0x0C,
            AlbaniaSubregions::Vlore => 0x0D,
        }
    }
}

impl Display for AlbaniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Albania => write!(f, "Albania"),
            Self::Tirana => write!(f, "Tirana"),
            Self::Berat => write!(f, "Berat"),
            Self::Diber => write!(f, "Dibër"),
            Self::Durres => write!(f, "Durrës"),
            Self::Elbasan => write!(f, "Elbasan"),
            Self::Fier => write!(f, "Fier"),
            Self::Gjirokaster => write!(f, "Gjirokastër"),
            Self::Korce => write!(f, "Korcë"),
            Self::Kukes => write!(f, "Kukës"),
            Self::Lezhe => write!(f, "Lezhë"),
            Self::Shkoder => write!(f, "Shkodër"),
            Self::Vlore => write!(f, "Vlorë"),
        }
    }
}

// Algeria Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlgeriaSubregions {
    Algeria,
    AlgiersProvince,
    AdrarProvince,
    AinDeflaProvince,
    AinTemouchentProvince,
    AnnabaProvince,
    BatnaProvince,
    BecharProvince,
    BejaiaProvince,
    BiskraProvince,
    BlidaProvince,
    BordjBouArreridjProvince,
    BouiraProvince,
    BoumerdesProvince,
    ChlefProvince,
    ConstantineProvince,
    DjelfaProvince,
    ElBayadhProvince,
    ElOuedProvince,
    ElTarefProvince,
    GhardaiaProvince,
    GuelmaProvince,
    IlliziProvince,
    JijelProvince,
    KhenchelaProvince,
    LaghouatProvince,
    MsilaProvince,
    MascaraProvince,
    MedeaProvince,
    MilaProvince,
    MostaganemProvince,
    NaamaProvince,
    OranProvince,
    OuarglaProvince,
    OumElBouaghiProvince,
    RelizaneProvince,
    SaidaProvince,
    SetifProvince,
    SidiBelAbbesProvince,
    SkikdaProvince,
    SoukAhrasProvince,
    TamanrassetProvince,
    TebessaProvince,
    TiaretProvince,
    TindoufProvince,
    TipazaProvince,
    TissemsiltProvince,
    TiziOuzouProvince,
    TlemcenProvince,
}

impl TryFrom<u8> for AlgeriaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Algeria),
            0x02 => Ok(Self::AlgiersProvince),
            0x03 => Ok(Self::AdrarProvince),
            0x04 => Ok(Self::AinDeflaProvince),
            0x05 => Ok(Self::AinTemouchentProvince),
            0x06 => Ok(Self::AnnabaProvince),
            0x07 => Ok(Self::BatnaProvince),
            0x08 => Ok(Self::BecharProvince),
            0x09 => Ok(Self::BejaiaProvince),
            0x0A => Ok(Self::BiskraProvince),
            0x0B => Ok(Self::BlidaProvince),
            0x0C => Ok(Self::BordjBouArreridjProvince),
            0x0D => Ok(Self::BouiraProvince),
            0x0E => Ok(Self::BoumerdesProvince),
            0x0F => Ok(Self::ChlefProvince),
            0x10 => Ok(Self::ConstantineProvince),
            0x11 => Ok(Self::DjelfaProvince),
            0x12 => Ok(Self::ElBayadhProvince),
            0x13 => Ok(Self::ElOuedProvince),
            0x14 => Ok(Self::ElTarefProvince),
            0x15 => Ok(Self::GhardaiaProvince),
            0x16 => Ok(Self::GuelmaProvince),
            0x17 => Ok(Self::IlliziProvince),
            0x18 => Ok(Self::JijelProvince),
            0x19 => Ok(Self::KhenchelaProvince),
            0x1A => Ok(Self::LaghouatProvince),
            0x1B => Ok(Self::MsilaProvince),
            0x1C => Ok(Self::MascaraProvince),
            0x1D => Ok(Self::MedeaProvince),
            0x1E => Ok(Self::MilaProvince),
            0x1F => Ok(Self::MostaganemProvince),
            0x20 => Ok(Self::NaamaProvince),
            0x21 => Ok(Self::OranProvince),
            0x22 => Ok(Self::OuarglaProvince),
            0x23 => Ok(Self::OumElBouaghiProvince),
            0x24 => Ok(Self::RelizaneProvince),
            0x25 => Ok(Self::SaidaProvince),
            0x26 => Ok(Self::SetifProvince),
            0x27 => Ok(Self::SidiBelAbbesProvince),
            0x28 => Ok(Self::SkikdaProvince),
            0x29 => Ok(Self::SoukAhrasProvince),
            0x2A => Ok(Self::TamanrassetProvince),
            0x2B => Ok(Self::TebessaProvince),
            0x2C => Ok(Self::TiaretProvince),
            0x2D => Ok(Self::TindoufProvince),
            0x2E => Ok(Self::TipazaProvince),
            0x2F => Ok(Self::TissemsiltProvince),
            0x30 => Ok(Self::TiziOuzouProvince),
            0x31 => Ok(Self::TlemcenProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AlgeriaSubregions> for u8 {
    fn from(value: AlgeriaSubregions) -> u8 {
        match value {
            AlgeriaSubregions::Algeria => 0x01,
            AlgeriaSubregions::AlgiersProvince => 0x02,
            AlgeriaSubregions::AdrarProvince => 0x03,
            AlgeriaSubregions::AinDeflaProvince => 0x04,
            AlgeriaSubregions::AinTemouchentProvince => 0x05,
            AlgeriaSubregions::AnnabaProvince => 0x06,
            AlgeriaSubregions::BatnaProvince => 0x07,
            AlgeriaSubregions::BecharProvince => 0x08,
            AlgeriaSubregions::BejaiaProvince => 0x09,
            AlgeriaSubregions::BiskraProvince => 0x0A,
            AlgeriaSubregions::BlidaProvince => 0x0B,
            AlgeriaSubregions::BordjBouArreridjProvince => 0x0C,
            AlgeriaSubregions::BouiraProvince => 0x0D,
            AlgeriaSubregions::BoumerdesProvince => 0x0E,
            AlgeriaSubregions::ChlefProvince => 0x0F,
            AlgeriaSubregions::ConstantineProvince => 0x10,
            AlgeriaSubregions::DjelfaProvince => 0x11,
            AlgeriaSubregions::ElBayadhProvince => 0x12,
            AlgeriaSubregions::ElOuedProvince => 0x13,
            AlgeriaSubregions::ElTarefProvince => 0x14,
            AlgeriaSubregions::GhardaiaProvince => 0x15,
            AlgeriaSubregions::GuelmaProvince => 0x16,
            AlgeriaSubregions::IlliziProvince => 0x17,
            AlgeriaSubregions::JijelProvince => 0x18,
            AlgeriaSubregions::KhenchelaProvince => 0x19,
            AlgeriaSubregions::LaghouatProvince => 0x1A,
            AlgeriaSubregions::MsilaProvince => 0x1B,
            AlgeriaSubregions::MascaraProvince => 0x1C,
            AlgeriaSubregions::MedeaProvince => 0x1D,
            AlgeriaSubregions::MilaProvince => 0x1E,
            AlgeriaSubregions::MostaganemProvince => 0x1F,
            AlgeriaSubregions::NaamaProvince => 0x20,
            AlgeriaSubregions::OranProvince => 0x21,
            AlgeriaSubregions::OuarglaProvince => 0x22,
            AlgeriaSubregions::OumElBouaghiProvince => 0x23,
            AlgeriaSubregions::RelizaneProvince => 0x24,
            AlgeriaSubregions::SaidaProvince => 0x25,
            AlgeriaSubregions::SetifProvince => 0x26,
            AlgeriaSubregions::SidiBelAbbesProvince => 0x27,
            AlgeriaSubregions::SkikdaProvince => 0x28,
            AlgeriaSubregions::SoukAhrasProvince => 0x29,
            AlgeriaSubregions::TamanrassetProvince => 0x2A,
            AlgeriaSubregions::TebessaProvince => 0x2B,
            AlgeriaSubregions::TiaretProvince => 0x2C,
            AlgeriaSubregions::TindoufProvince => 0x2D,
            AlgeriaSubregions::TipazaProvince => 0x2E,
            AlgeriaSubregions::TissemsiltProvince => 0x2F,
            AlgeriaSubregions::TiziOuzouProvince => 0x30,
            AlgeriaSubregions::TlemcenProvince => 0x31,
        }
    }
}

impl Display for AlgeriaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Algeria => write!(f, "Algeria"),
            Self::AlgiersProvince => write!(f, "Algiers Province"),
            Self::AdrarProvince => write!(f, "Adrar Province"),
            Self::AinDeflaProvince => write!(f, "Aïn Defla Province"),
            Self::AinTemouchentProvince => write!(f, "Aïn Témouchent Province"),
            Self::AnnabaProvince => write!(f, "Annaba Province"),
            Self::BatnaProvince => write!(f, "Batna Province"),
            Self::BecharProvince => write!(f, "Béchar Province"),
            Self::BejaiaProvince => write!(f, "Bejaia Province"),
            Self::BiskraProvince => write!(f, "Biskra Province"),
            Self::BlidaProvince => write!(f, "Blida Province"),
            Self::BordjBouArreridjProvince => write!(f, "Bordj Bou Arréridj Province"),
            Self::BouiraProvince => write!(f, "Bouïra Province"),
            Self::BoumerdesProvince => write!(f, "Boumerdès Province"),
            Self::ChlefProvince => write!(f, "Chlef Province"),
            Self::ConstantineProvince => write!(f, "Constantine Province"),
            Self::DjelfaProvince => write!(f, "Djelfa Province"),
            Self::ElBayadhProvince => write!(f, "El Bayadh Province"),
            Self::ElOuedProvince => write!(f, "El Oued Province"),
            Self::ElTarefProvince => write!(f, "El Taref Province"),
            Self::GhardaiaProvince => write!(f, "Ghardaïa Province"),
            Self::GuelmaProvince => write!(f, "Guelma Province"),
            Self::IlliziProvince => write!(f, "Illizi Province"),
            Self::JijelProvince => write!(f, "Jijel Province"),
            Self::KhenchelaProvince => write!(f, "Khenchela Province"),
            Self::LaghouatProvince => write!(f, "Laghouat Province"),
            Self::MsilaProvince => write!(f, "M'Sila Province"),
            Self::MascaraProvince => write!(f, "Mascara Province"),
            Self::MedeaProvince => write!(f, "Médéa Province"),
            Self::MilaProvince => write!(f, "Mila Province"),
            Self::MostaganemProvince => write!(f, "Mostaganem Province"),
            Self::NaamaProvince => write!(f, "Naâma Province"),
            Self::OranProvince => write!(f, "Oran Province"),
            Self::OuarglaProvince => write!(f, "Ouargla Province"),
            Self::OumElBouaghiProvince => write!(f, "Oum El Bouaghi Province"),
            Self::RelizaneProvince => write!(f, "Relizane Province"),
            Self::SaidaProvince => write!(f, "Saïda Province"),
            Self::SetifProvince => write!(f, "Sétif Province"),
            Self::SidiBelAbbesProvince => write!(f, "Sidi Bel Abbès Province"),
            Self::SkikdaProvince => write!(f, "Skikda Province"),
            Self::SoukAhrasProvince => write!(f, "Souk Ahras Province"),
            Self::TamanrassetProvince => write!(f, "Tamanrasset Province"),
            Self::TebessaProvince => write!(f, "Tébessa Province"),
            Self::TiaretProvince => write!(f, "Tiaret Province"),
            Self::TindoufProvince => write!(f, "Tindouf Province"),
            Self::TipazaProvince => write!(f, "Tipaza Province"),
            Self::TissemsiltProvince => write!(f, "Tissemsilt Province"),
            Self::TiziOuzouProvince => write!(f, "Tizi Ouzou Province"),
            Self::TlemcenProvince => write!(f, "Tlemcen Province"),
        }
    }
}

// American Samoa Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AmericanSamoaSubregions {
    AmericanSamoa,
    EasternDistrict,
    ManuaDistrict,
    RoseAtoll,
    SwainsIsland,
    WesternDistrict,
}

impl TryFrom<u8> for AmericanSamoaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::AmericanSamoa),
            0x02 => Ok(Self::EasternDistrict),
            0x03 => Ok(Self::ManuaDistrict),
            0x04 => Ok(Self::RoseAtoll),
            0x05 => Ok(Self::SwainsIsland),
            0x06 => Ok(Self::WesternDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AmericanSamoaSubregions> for u8 {
    fn from(value: AmericanSamoaSubregions) -> u8 {
        match value {
            AmericanSamoaSubregions::AmericanSamoa => 0x01,
            AmericanSamoaSubregions::EasternDistrict => 0x02,
            AmericanSamoaSubregions::ManuaDistrict => 0x03,
            AmericanSamoaSubregions::RoseAtoll => 0x04,
            AmericanSamoaSubregions::SwainsIsland => 0x05,
            AmericanSamoaSubregions::WesternDistrict => 0x06,
        }
    }
}

impl Display for AmericanSamoaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AmericanSamoa => write!(f, "American Samoa"),
            Self::EasternDistrict => write!(f, "Eastern District"),
            Self::ManuaDistrict => write!(f, "Manu'a District"),
            Self::RoseAtoll => write!(f, "Rose Atoll"),
            Self::SwainsIsland => write!(f, "Swains Island"),
            Self::WesternDistrict => write!(f, "Western District"),
        }
    }
}

// Andorra Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AndorraSubregions {
    Andorra,
}

impl TryFrom<u8> for AndorraSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Andorra),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AndorraSubregions> for u8 {
    fn from(value: AndorraSubregions) -> u8 {
        match value {
            AndorraSubregions::Andorra => 0x01,
        }
    }
}

impl Display for AndorraSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Andorra => write!(f, "Andorra"),
        }
    }
}

// Angola Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngolaSubregions {
    Angola,
    Luanda,
    Bengo,
    Benguela,
    Bie,
    Cabinda,
    CuandoCubango,
    CuanzaNorte,
    CuanzaSul,
    Cunene,
    Huambo,
    Huila,
    LundaNorte,
    LundaSul,
    Malanje,
    Moxico,
    Namibe,
    Uige,
    Zaire,
}

impl TryFrom<u8> for AngolaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Angola),
            0x02 => Ok(Self::Luanda),
            0x03 => Ok(Self::Bengo),
            0x04 => Ok(Self::Benguela),
            0x05 => Ok(Self::Bie),
            0x06 => Ok(Self::Cabinda),
            0x07 => Ok(Self::CuandoCubango),
            0x08 => Ok(Self::CuanzaNorte),
            0x09 => Ok(Self::CuanzaSul),
            0x0A => Ok(Self::Cunene),
            0x0B => Ok(Self::Huambo),
            0x0C => Ok(Self::Huila),
            0x0D => Ok(Self::LundaNorte),
            0x0E => Ok(Self::LundaSul),
            0x0F => Ok(Self::Malanje),
            0x10 => Ok(Self::Moxico),
            0x11 => Ok(Self::Namibe),
            0x12 => Ok(Self::Uige),
            0x13 => Ok(Self::Zaire),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AngolaSubregions> for u8 {
    fn from(value: AngolaSubregions) -> u8 {
        match value {
            AngolaSubregions::Angola => 0x01,
            AngolaSubregions::Luanda => 0x02,
            AngolaSubregions::Bengo => 0x03,
            AngolaSubregions::Benguela => 0x04,
            AngolaSubregions::Bie => 0x05,
            AngolaSubregions::Cabinda => 0x06,
            AngolaSubregions::CuandoCubango => 0x07,
            AngolaSubregions::CuanzaNorte => 0x08,
            AngolaSubregions::CuanzaSul => 0x09,
            AngolaSubregions::Cunene => 0x0A,
            AngolaSubregions::Huambo => 0x0B,
            AngolaSubregions::Huila => 0x0C,
            AngolaSubregions::LundaNorte => 0x0D,
            AngolaSubregions::LundaSul => 0x0E,
            AngolaSubregions::Malanje => 0x0F,
            AngolaSubregions::Moxico => 0x10,
            AngolaSubregions::Namibe => 0x11,
            AngolaSubregions::Uige => 0x12,
            AngolaSubregions::Zaire => 0x13,
        }
    }
}

impl Display for AngolaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Angola => write!(f, "Angola"),
            Self::Luanda => write!(f, "Luanda"),
            Self::Bengo => write!(f, "Bengo"),
            Self::Benguela => write!(f, "Benguela"),
            Self::Bie => write!(f, "Bié"),
            Self::Cabinda => write!(f, "Cabinda"),
            Self::CuandoCubango => write!(f, "Cuando Cubango"),
            Self::CuanzaNorte => write!(f, "Cuanza Norte"),
            Self::CuanzaSul => write!(f, "Cuanza Sul"),
            Self::Cunene => write!(f, "Cunene"),
            Self::Huambo => write!(f, "Huambo"),
            Self::Huila => write!(f, "Huíla"),
            Self::LundaNorte => write!(f, "Lunda Norte"),
            Self::LundaSul => write!(f, "Lunda Sul"),
            Self::Malanje => write!(f, "Malanje"),
            Self::Moxico => write!(f, "Moxico"),
            Self::Namibe => write!(f, "Namibe"),
            Self::Uige => write!(f, "Uíge"),
            Self::Zaire => write!(f, "Zaire"),
        }
    }
}

// Anguilla Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnguillaSubregions {
    Anguilla,
}

impl TryFrom<u8> for AnguillaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Anguilla),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AnguillaSubregions> for u8 {
    fn from(value: AnguillaSubregions) -> u8 {
        match value {
            AnguillaSubregions::Anguilla => 0x01,
        }
    }
}

impl Display for AnguillaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anguilla => write!(f, "Anguilla"),
        }
    }
}

// Antarctica Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntarcticaSubregions {
    Antarctica,
    QueenMaudLand,
    AdelieLand,
    CoatsLand,
    EllsworthLand,
    EnderbyLand,
    GeorgeVLand,
    GrahamLand,
    KempLand,
    KingEdwardViiLand,
    MacRobertsonLand,
    MarieByrdLand,
    OatesLand,
    PalmerLand,
    PrincessElizabethLand,
    QueenMaryLand,
    VictoriaLand,
    WilhelmIiLand,
    WilkesLand,
}

impl TryFrom<u8> for AntarcticaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Antarctica),
            0x02 => Ok(Self::QueenMaudLand),
            0x03 => Ok(Self::AdelieLand),
            0x04 => Ok(Self::CoatsLand),
            0x05 => Ok(Self::EllsworthLand),
            0x06 => Ok(Self::EnderbyLand),
            0x07 => Ok(Self::GeorgeVLand),
            0x08 => Ok(Self::GrahamLand),
            0x09 => Ok(Self::KempLand),
            0x0A => Ok(Self::KingEdwardViiLand),
            0x0B => Ok(Self::MacRobertsonLand),
            0x0C => Ok(Self::MarieByrdLand),
            0x0D => Ok(Self::OatesLand),
            0x0E => Ok(Self::PalmerLand),
            0x0F => Ok(Self::PrincessElizabethLand),
            0x10 => Ok(Self::QueenMaryLand),
            0x11 => Ok(Self::VictoriaLand),
            0x12 => Ok(Self::WilhelmIiLand),
            0x13 => Ok(Self::WilkesLand),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AntarcticaSubregions> for u8 {
    fn from(value: AntarcticaSubregions) -> u8 {
        match value {
            AntarcticaSubregions::Antarctica => 0x01,
            AntarcticaSubregions::QueenMaudLand => 0x02,
            AntarcticaSubregions::AdelieLand => 0x03,
            AntarcticaSubregions::CoatsLand => 0x04,
            AntarcticaSubregions::EllsworthLand => 0x05,
            AntarcticaSubregions::EnderbyLand => 0x06,
            AntarcticaSubregions::GeorgeVLand => 0x07,
            AntarcticaSubregions::GrahamLand => 0x08,
            AntarcticaSubregions::KempLand => 0x09,
            AntarcticaSubregions::KingEdwardViiLand => 0x0A,
            AntarcticaSubregions::MacRobertsonLand => 0x0B,
            AntarcticaSubregions::MarieByrdLand => 0x0C,
            AntarcticaSubregions::OatesLand => 0x0D,
            AntarcticaSubregions::PalmerLand => 0x0E,
            AntarcticaSubregions::PrincessElizabethLand => 0x0F,
            AntarcticaSubregions::QueenMaryLand => 0x10,
            AntarcticaSubregions::VictoriaLand => 0x11,
            AntarcticaSubregions::WilhelmIiLand => 0x12,
            AntarcticaSubregions::WilkesLand => 0x13,
        }
    }
}

impl Display for AntarcticaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Antarctica => write!(f, "Antarctica"),
            Self::QueenMaudLand => write!(f, "Queen Maud Land"),
            Self::AdelieLand => write!(f, "Adélie Land"),
            Self::CoatsLand => write!(f, "Coats Land"),
            Self::EllsworthLand => write!(f, "Ellsworth Land"),
            Self::EnderbyLand => write!(f, "Enderby Land"),
            Self::GeorgeVLand => write!(f, "George V Land"),
            Self::GrahamLand => write!(f, "Graham Land"),
            Self::KempLand => write!(f, "Kemp Land"),
            Self::KingEdwardViiLand => write!(f, "King Edward VII Land"),
            Self::MacRobertsonLand => write!(f, "Mac. Robertson Land"),
            Self::MarieByrdLand => write!(f, "Marie Byrd Land"),
            Self::OatesLand => write!(f, "Oates Land"),
            Self::PalmerLand => write!(f, "Palmer Land"),
            Self::PrincessElizabethLand => write!(f, "Princess Elizabeth Land"),
            Self::QueenMaryLand => write!(f, "Queen Mary Land"),
            Self::VictoriaLand => write!(f, "Victoria Land"),
            Self::WilhelmIiLand => write!(f, "Wilhelm II Land"),
            Self::WilkesLand => write!(f, "Wilkes Land"),
        }
    }
}

// Antigua and Barbuda Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntiguaAndBarbudaSubregions {
    AntiguaAndBarbuda,
    SaintJohn,
    Barbuda,
    SaintGeorge,
    SaintMary,
    SaintPaul,
    SaintPeter,
    SaintPhilip,
}

impl TryFrom<u8> for AntiguaAndBarbudaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::AntiguaAndBarbuda),
            0x02 => Ok(Self::SaintJohn),
            0x03 => Ok(Self::Barbuda),
            0x04 => Ok(Self::SaintGeorge),
            0x05 => Ok(Self::SaintMary),
            0x06 => Ok(Self::SaintPaul),
            0x07 => Ok(Self::SaintPeter),
            0x08 => Ok(Self::SaintPhilip),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AntiguaAndBarbudaSubregions> for u8 {
    fn from(value: AntiguaAndBarbudaSubregions) -> u8 {
        match value {
            AntiguaAndBarbudaSubregions::AntiguaAndBarbuda => 0x01,
            AntiguaAndBarbudaSubregions::SaintJohn => 0x02,
            AntiguaAndBarbudaSubregions::Barbuda => 0x03,
            AntiguaAndBarbudaSubregions::SaintGeorge => 0x04,
            AntiguaAndBarbudaSubregions::SaintMary => 0x05,
            AntiguaAndBarbudaSubregions::SaintPaul => 0x06,
            AntiguaAndBarbudaSubregions::SaintPeter => 0x07,
            AntiguaAndBarbudaSubregions::SaintPhilip => 0x08,
        }
    }
}

impl Display for AntiguaAndBarbudaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AntiguaAndBarbuda => write!(f, "Antigua and Barbuda"),
            Self::SaintJohn => write!(f, "Saint John"),
            Self::Barbuda => write!(f, "Barbuda"),
            Self::SaintGeorge => write!(f, "Saint George"),
            Self::SaintMary => write!(f, "Saint Mary"),
            Self::SaintPaul => write!(f, "Saint Paul"),
            Self::SaintPeter => write!(f, "Saint Peter"),
            Self::SaintPhilip => write!(f, "Saint Philip"),
        }
    }
}

// Argentina Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgentinaSubregions {
    Argentina,
    DistritoFederal,
    BuenosAires,
    Catamarca,
    Chaco,
    Chubut,
    Cordoba,
    Corrientes,
    EntreRios,
    Formosa,
    Jujuy,
    LaPampa,
    LaRioja,
    Mendoza,
    Misiones,
    Neuquen,
    RioNegro,
    Salta,
    SanJuan,
    SanLuis,
    SantaCruz,
    SantaFe,
    SantiagoDelEstero,
    TierraDelFuegoAntartidaEIslasDelAtlanticoSur,
    Tucuman,
}

impl TryFrom<u8> for ArgentinaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Argentina),
            0x02 => Ok(Self::DistritoFederal),
            0x03 => Ok(Self::BuenosAires),
            0x04 => Ok(Self::Catamarca),
            0x05 => Ok(Self::Chaco),
            0x06 => Ok(Self::Chubut),
            0x07 => Ok(Self::Cordoba),
            0x08 => Ok(Self::Corrientes),
            0x09 => Ok(Self::EntreRios),
            0x0A => Ok(Self::Formosa),
            0x0B => Ok(Self::Jujuy),
            0x0C => Ok(Self::LaPampa),
            0x0D => Ok(Self::LaRioja),
            0x0E => Ok(Self::Mendoza),
            0x0F => Ok(Self::Misiones),
            0x10 => Ok(Self::Neuquen),
            0x11 => Ok(Self::RioNegro),
            0x12 => Ok(Self::Salta),
            0x13 => Ok(Self::SanJuan),
            0x14 => Ok(Self::SanLuis),
            0x15 => Ok(Self::SantaCruz),
            0x16 => Ok(Self::SantaFe),
            0x17 => Ok(Self::SantiagoDelEstero),
            0x18 => Ok(Self::TierraDelFuegoAntartidaEIslasDelAtlanticoSur),
            0x19 => Ok(Self::Tucuman),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ArgentinaSubregions> for u8 {
    fn from(value: ArgentinaSubregions) -> u8 {
        match value {
            ArgentinaSubregions::Argentina => 0x01,
            ArgentinaSubregions::DistritoFederal => 0x02,
            ArgentinaSubregions::BuenosAires => 0x03,
            ArgentinaSubregions::Catamarca => 0x04,
            ArgentinaSubregions::Chaco => 0x05,
            ArgentinaSubregions::Chubut => 0x06,
            ArgentinaSubregions::Cordoba => 0x07,
            ArgentinaSubregions::Corrientes => 0x08,
            ArgentinaSubregions::EntreRios => 0x09,
            ArgentinaSubregions::Formosa => 0x0A,
            ArgentinaSubregions::Jujuy => 0x0B,
            ArgentinaSubregions::LaPampa => 0x0C,
            ArgentinaSubregions::LaRioja => 0x0D,
            ArgentinaSubregions::Mendoza => 0x0E,
            ArgentinaSubregions::Misiones => 0x0F,
            ArgentinaSubregions::Neuquen => 0x10,
            ArgentinaSubregions::RioNegro => 0x11,
            ArgentinaSubregions::Salta => 0x12,
            ArgentinaSubregions::SanJuan => 0x13,
            ArgentinaSubregions::SanLuis => 0x14,
            ArgentinaSubregions::SantaCruz => 0x15,
            ArgentinaSubregions::SantaFe => 0x16,
            ArgentinaSubregions::SantiagoDelEstero => 0x17,
            ArgentinaSubregions::TierraDelFuegoAntartidaEIslasDelAtlanticoSur => 0x18,
            ArgentinaSubregions::Tucuman => 0x19,
        }
    }
}

impl Display for ArgentinaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Argentina => write!(f, "Argentina"),
            Self::DistritoFederal => write!(f, "Distrito Federal"),
            Self::BuenosAires => write!(f, "Buenos Aires"),
            Self::Catamarca => write!(f, "Catamarca"),
            Self::Chaco => write!(f, "Chaco"),
            Self::Chubut => write!(f, "Chubut"),
            Self::Cordoba => write!(f, "Córdoba"),
            Self::Corrientes => write!(f, "Corrientes"),
            Self::EntreRios => write!(f, "Entre Ríos"),
            Self::Formosa => write!(f, "Formosa"),
            Self::Jujuy => write!(f, "Jujuy"),
            Self::LaPampa => write!(f, "La Pampa"),
            Self::LaRioja => write!(f, "La Rioja"),
            Self::Mendoza => write!(f, "Mendoza"),
            Self::Misiones => write!(f, "Misiones"),
            Self::Neuquen => write!(f, "Neuquén"),
            Self::RioNegro => write!(f, "Río Negro"),
            Self::Salta => write!(f, "Salta"),
            Self::SanJuan => write!(f, "San Juan"),
            Self::SanLuis => write!(f, "San Luis"),
            Self::SantaCruz => write!(f, "Santa Cruz"),
            Self::SantaFe => write!(f, "Santa Fe"),
            Self::SantiagoDelEstero => write!(f, "Santiago del Estero"),
            Self::TierraDelFuegoAntartidaEIslasDelAtlanticoSur => {
                write!(f, "Tierra del Fuego, Antártida e Islas del Atlántico Sur")
            }
            Self::Tucuman => write!(f, "Tucumán"),
        }
    }
}

// Armenia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArmeniaSubregions {
    Armenia,
    Yerevan,
    Aragatsotn,
    Ararat,
    Armavir,
    Gegharkunik,
    Kotayk,
    Lori,
    Shirak,
    Syunik,
    Tavush,
    VayotsDzor,
}

impl TryFrom<u8> for ArmeniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Armenia),
            0x02 => Ok(Self::Yerevan),
            0x03 => Ok(Self::Aragatsotn),
            0x04 => Ok(Self::Ararat),
            0x05 => Ok(Self::Armavir),
            0x06 => Ok(Self::Gegharkunik),
            0x07 => Ok(Self::Kotayk),
            0x08 => Ok(Self::Lori),
            0x09 => Ok(Self::Shirak),
            0x0A => Ok(Self::Syunik),
            0x0B => Ok(Self::Tavush),
            0x0C => Ok(Self::VayotsDzor),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ArmeniaSubregions> for u8 {
    fn from(value: ArmeniaSubregions) -> u8 {
        match value {
            ArmeniaSubregions::Armenia => 0x01,
            ArmeniaSubregions::Yerevan => 0x02,
            ArmeniaSubregions::Aragatsotn => 0x03,
            ArmeniaSubregions::Ararat => 0x04,
            ArmeniaSubregions::Armavir => 0x05,
            ArmeniaSubregions::Gegharkunik => 0x06,
            ArmeniaSubregions::Kotayk => 0x07,
            ArmeniaSubregions::Lori => 0x08,
            ArmeniaSubregions::Shirak => 0x09,
            ArmeniaSubregions::Syunik => 0x0A,
            ArmeniaSubregions::Tavush => 0x0B,
            ArmeniaSubregions::VayotsDzor => 0x0C,
        }
    }
}

impl Display for ArmeniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Armenia => write!(f, "Armenia"),
            Self::Yerevan => write!(f, "Yerevan"),
            Self::Aragatsotn => write!(f, "Aragatsotn"),
            Self::Ararat => write!(f, "Ararat"),
            Self::Armavir => write!(f, "Armavir"),
            Self::Gegharkunik => write!(f, "Gegharkunik"),
            Self::Kotayk => write!(f, "Kotayk"),
            Self::Lori => write!(f, "Lori"),
            Self::Shirak => write!(f, "Shirak"),
            Self::Syunik => write!(f, "Syunik"),
            Self::Tavush => write!(f, "Tavush"),
            Self::VayotsDzor => write!(f, "Vayots Dzor"),
        }
    }
}

// Artsakh Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArtsakhSubregions {
    Artsakh,
    Stepanakert,
    AskeranProvince,
    HadrutProvince,
    KashataghProvince,
    MartakertProvince,
    MartuniProvince,
    ShahumyanProvince,
    ShushiProvince,
}

impl TryFrom<u8> for ArtsakhSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Artsakh),
            0x02 => Ok(Self::Stepanakert),
            0x03 => Ok(Self::AskeranProvince),
            0x04 => Ok(Self::HadrutProvince),
            0x05 => Ok(Self::KashataghProvince),
            0x06 => Ok(Self::MartakertProvince),
            0x07 => Ok(Self::MartuniProvince),
            0x08 => Ok(Self::ShahumyanProvince),
            0x09 => Ok(Self::ShushiProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ArtsakhSubregions> for u8 {
    fn from(value: ArtsakhSubregions) -> u8 {
        match value {
            ArtsakhSubregions::Artsakh => 0x01,
            ArtsakhSubregions::Stepanakert => 0x02,
            ArtsakhSubregions::AskeranProvince => 0x03,
            ArtsakhSubregions::HadrutProvince => 0x04,
            ArtsakhSubregions::KashataghProvince => 0x05,
            ArtsakhSubregions::MartakertProvince => 0x06,
            ArtsakhSubregions::MartuniProvince => 0x07,
            ArtsakhSubregions::ShahumyanProvince => 0x08,
            ArtsakhSubregions::ShushiProvince => 0x09,
        }
    }
}

impl Display for ArtsakhSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Artsakh => write!(f, "Artsakh"),
            Self::Stepanakert => write!(f, "Stepanakert"),
            Self::AskeranProvince => write!(f, "Askeran Province"),
            Self::HadrutProvince => write!(f, "Hadrut Province"),
            Self::KashataghProvince => write!(f, "Kashatagh Province"),
            Self::MartakertProvince => write!(f, "Martakert Province"),
            Self::MartuniProvince => write!(f, "Martuni Province"),
            Self::ShahumyanProvince => write!(f, "Shahumyan Province"),
            Self::ShushiProvince => write!(f, "Shushi Province"),
        }
    }
}

// Aruba Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArubaSubregions {
    Aruba,
}

impl TryFrom<u8> for ArubaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Aruba),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ArubaSubregions> for u8 {
    fn from(value: ArubaSubregions) -> u8 {
        match value {
            ArubaSubregions::Aruba => 0x01,
        }
    }
}

impl Display for ArubaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aruba => write!(f, "Aruba"),
        }
    }
}

// Australia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AustraliaSubregions {
    Australia,
    AustralianCapitalTerritory,
    NewSouthWales,
    NorthernTerritory,
    Queensland,
    SouthAustralia,
    Tasmania,
    Victoria,
    WesternAustralia,
    AustralianAntarcticTerritory,
    ChristmasIsland,
    CocosKeelingIslands,
    CoralSeaIslands,
    HeardIslandAndMcdonaldIslands,
    NorfolkIsland,
}

impl TryFrom<u8> for AustraliaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Australia),
            0x02 => Ok(Self::AustralianCapitalTerritory),
            0x03 => Ok(Self::NewSouthWales),
            0x04 => Ok(Self::NorthernTerritory),
            0x05 => Ok(Self::Queensland),
            0x06 => Ok(Self::SouthAustralia),
            0x07 => Ok(Self::Tasmania),
            0x08 => Ok(Self::Victoria),
            0x09 => Ok(Self::WesternAustralia),
            0x0A => Ok(Self::AustralianAntarcticTerritory),
            0x0B => Ok(Self::ChristmasIsland),
            0x0C => Ok(Self::CocosKeelingIslands),
            0x0D => Ok(Self::CoralSeaIslands),
            0x0E => Ok(Self::HeardIslandAndMcdonaldIslands),
            0x0F => Ok(Self::NorfolkIsland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AustraliaSubregions> for u8 {
    fn from(value: AustraliaSubregions) -> u8 {
        match value {
            AustraliaSubregions::Australia => 0x01,
            AustraliaSubregions::AustralianCapitalTerritory => 0x02,
            AustraliaSubregions::NewSouthWales => 0x03,
            AustraliaSubregions::NorthernTerritory => 0x04,
            AustraliaSubregions::Queensland => 0x05,
            AustraliaSubregions::SouthAustralia => 0x06,
            AustraliaSubregions::Tasmania => 0x07,
            AustraliaSubregions::Victoria => 0x08,
            AustraliaSubregions::WesternAustralia => 0x09,
            AustraliaSubregions::AustralianAntarcticTerritory => 0x0A,
            AustraliaSubregions::ChristmasIsland => 0x0B,
            AustraliaSubregions::CocosKeelingIslands => 0x0C,
            AustraliaSubregions::CoralSeaIslands => 0x0D,
            AustraliaSubregions::HeardIslandAndMcdonaldIslands => 0x0E,
            AustraliaSubregions::NorfolkIsland => 0x0F,
        }
    }
}

impl Display for AustraliaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Australia => write!(f, "Australia"),
            Self::AustralianCapitalTerritory => write!(f, "Australian Capital Territory"),
            Self::NewSouthWales => write!(f, "New South Wales"),
            Self::NorthernTerritory => write!(f, "Northern Territory"),
            Self::Queensland => write!(f, "Queensland"),
            Self::SouthAustralia => write!(f, "South Australia"),
            Self::Tasmania => write!(f, "Tasmania"),
            Self::Victoria => write!(f, "Victoria"),
            Self::WesternAustralia => write!(f, "Western Australia"),
            Self::AustralianAntarcticTerritory => write!(f, "Australian Antarctic Territory"),
            Self::ChristmasIsland => write!(f, "Christmas Island"),
            Self::CocosKeelingIslands => write!(f, "Cocos (Keeling) Islands"),
            Self::CoralSeaIslands => write!(f, "Coral Sea Islands"),
            Self::HeardIslandAndMcdonaldIslands => write!(f, "Heard Island and McDonald Islands"),
            Self::NorfolkIsland => write!(f, "Norfolk Island"),
        }
    }
}

// Austria Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AustriaSubregions {
    Austria,
    Vienna,
    Burgenland,
    Carinthia,
    LowerAustria,
    UpperAustria,
    Salzburg,
    Styria,
    Tyrol,
    Vorarlberg,
}

impl TryFrom<u8> for AustriaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Austria),
            0x02 => Ok(Self::Vienna),
            0x03 => Ok(Self::Burgenland),
            0x04 => Ok(Self::Carinthia),
            0x05 => Ok(Self::LowerAustria),
            0x06 => Ok(Self::UpperAustria),
            0x07 => Ok(Self::Salzburg),
            0x08 => Ok(Self::Styria),
            0x09 => Ok(Self::Tyrol),
            0x0A => Ok(Self::Vorarlberg),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AustriaSubregions> for u8 {
    fn from(value: AustriaSubregions) -> u8 {
        match value {
            AustriaSubregions::Austria => 0x01,
            AustriaSubregions::Vienna => 0x02,
            AustriaSubregions::Burgenland => 0x03,
            AustriaSubregions::Carinthia => 0x04,
            AustriaSubregions::LowerAustria => 0x05,
            AustriaSubregions::UpperAustria => 0x06,
            AustriaSubregions::Salzburg => 0x07,
            AustriaSubregions::Styria => 0x08,
            AustriaSubregions::Tyrol => 0x09,
            AustriaSubregions::Vorarlberg => 0x0A,
        }
    }
}

impl Display for AustriaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Austria => write!(f, "Austria"),
            Self::Vienna => write!(f, "Vienna"),
            Self::Burgenland => write!(f, "Burgenland"),
            Self::Carinthia => write!(f, "Carinthia"),
            Self::LowerAustria => write!(f, "Lower Austria"),
            Self::UpperAustria => write!(f, "Upper Austria"),
            Self::Salzburg => write!(f, "Salzburg"),
            Self::Styria => write!(f, "Styria"),
            Self::Tyrol => write!(f, "Tyrol"),
            Self::Vorarlberg => write!(f, "Vorarlberg"),
        }
    }
}

// Azerbaijan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AzerbaijanSubregions {
    Azerbaijan,
    Absheron,
    Aran,
    GanjaQazakh,
    GubaKhachmaz,
    KalbajarLachin,
    Lankaran,
    MountainousShirvan,
    Nakhchivan,
    ShakiZaqatala,
    YukhariKarabakh,
}

impl TryFrom<u8> for AzerbaijanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Azerbaijan),
            0x02 => Ok(Self::Absheron),
            0x03 => Ok(Self::Aran),
            0x04 => Ok(Self::GanjaQazakh),
            0x05 => Ok(Self::GubaKhachmaz),
            0x06 => Ok(Self::KalbajarLachin),
            0x07 => Ok(Self::Lankaran),
            0x08 => Ok(Self::MountainousShirvan),
            0x09 => Ok(Self::Nakhchivan),
            0x0A => Ok(Self::ShakiZaqatala),
            0x0B => Ok(Self::YukhariKarabakh),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AzerbaijanSubregions> for u8 {
    fn from(value: AzerbaijanSubregions) -> u8 {
        match value {
            AzerbaijanSubregions::Azerbaijan => 0x01,
            AzerbaijanSubregions::Absheron => 0x02,
            AzerbaijanSubregions::Aran => 0x03,
            AzerbaijanSubregions::GanjaQazakh => 0x04,
            AzerbaijanSubregions::GubaKhachmaz => 0x05,
            AzerbaijanSubregions::KalbajarLachin => 0x06,
            AzerbaijanSubregions::Lankaran => 0x07,
            AzerbaijanSubregions::MountainousShirvan => 0x08,
            AzerbaijanSubregions::Nakhchivan => 0x09,
            AzerbaijanSubregions::ShakiZaqatala => 0x0A,
            AzerbaijanSubregions::YukhariKarabakh => 0x0B,
        }
    }
}

impl Display for AzerbaijanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Azerbaijan => write!(f, "Azerbaijan"),
            Self::Absheron => write!(f, "Absheron"),
            Self::Aran => write!(f, "Aran"),
            Self::GanjaQazakh => write!(f, "Ganja-Qazakh"),
            Self::GubaKhachmaz => write!(f, "Guba-Khachmaz"),
            Self::KalbajarLachin => write!(f, "Kalbajar-Lachin"),
            Self::Lankaran => write!(f, "Lankaran"),
            Self::MountainousShirvan => write!(f, "Mountainous Shirvan"),
            Self::Nakhchivan => write!(f, "Nakhchivan"),
            Self::ShakiZaqatala => write!(f, "Shaki-Zaqatala"),
            Self::YukhariKarabakh => write!(f, "Yukhari-Karabakh"),
        }
    }
}

// Bahamas Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BahamasSubregions {
    Bahamas,
}

impl TryFrom<u8> for BahamasSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bahamas),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BahamasSubregions> for u8 {
    fn from(value: BahamasSubregions) -> u8 {
        match value {
            BahamasSubregions::Bahamas => 0x01,
        }
    }
}

impl Display for BahamasSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bahamas => write!(f, "Bahamas"),
        }
    }
}

// Bahrain Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BahrainSubregions {
    Bahrain,
    CapitalGovernorate,
    MuharraqGovernorate,
    NorthernGovernorate,
    SouthernGovernorate,
}

impl TryFrom<u8> for BahrainSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bahrain),
            0x02 => Ok(Self::CapitalGovernorate),
            0x03 => Ok(Self::MuharraqGovernorate),
            0x04 => Ok(Self::NorthernGovernorate),
            0x05 => Ok(Self::SouthernGovernorate),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BahrainSubregions> for u8 {
    fn from(value: BahrainSubregions) -> u8 {
        match value {
            BahrainSubregions::Bahrain => 0x01,
            BahrainSubregions::CapitalGovernorate => 0x02,
            BahrainSubregions::MuharraqGovernorate => 0x03,
            BahrainSubregions::NorthernGovernorate => 0x04,
            BahrainSubregions::SouthernGovernorate => 0x05,
        }
    }
}

impl Display for BahrainSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bahrain => write!(f, "Bahrain"),
            Self::CapitalGovernorate => write!(f, "Capital Governorate"),
            Self::MuharraqGovernorate => write!(f, "Muharraq Governorate"),
            Self::NorthernGovernorate => write!(f, "Northern Governorate"),
            Self::SouthernGovernorate => write!(f, "Southern Governorate"),
        }
    }
}

// Bangladesh Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BangladeshSubregions {
    Bangladesh,
    DhakaDivision,
    BarisalDivision,
    ChittagongDivision,
    KhulnaDivision,
    MymensinghDivision,
    RajshahiDivision,
    RangpurDivision,
    SylhetDivision,
}

impl TryFrom<u8> for BangladeshSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bangladesh),
            0x02 => Ok(Self::DhakaDivision),
            0x03 => Ok(Self::BarisalDivision),
            0x04 => Ok(Self::ChittagongDivision),
            0x05 => Ok(Self::KhulnaDivision),
            0x06 => Ok(Self::MymensinghDivision),
            0x07 => Ok(Self::RajshahiDivision),
            0x08 => Ok(Self::RangpurDivision),
            0x09 => Ok(Self::SylhetDivision),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BangladeshSubregions> for u8 {
    fn from(value: BangladeshSubregions) -> u8 {
        match value {
            BangladeshSubregions::Bangladesh => 0x01,
            BangladeshSubregions::DhakaDivision => 0x02,
            BangladeshSubregions::BarisalDivision => 0x03,
            BangladeshSubregions::ChittagongDivision => 0x04,
            BangladeshSubregions::KhulnaDivision => 0x05,
            BangladeshSubregions::MymensinghDivision => 0x06,
            BangladeshSubregions::RajshahiDivision => 0x07,
            BangladeshSubregions::RangpurDivision => 0x08,
            BangladeshSubregions::SylhetDivision => 0x09,
        }
    }
}

impl Display for BangladeshSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bangladesh => write!(f, "Bangladesh"),
            Self::DhakaDivision => write!(f, "Dhaka Division"),
            Self::BarisalDivision => write!(f, "Barisal Division"),
            Self::ChittagongDivision => write!(f, "Chittagong Division"),
            Self::KhulnaDivision => write!(f, "Khulna Division"),
            Self::MymensinghDivision => write!(f, "Mymensingh Division"),
            Self::RajshahiDivision => write!(f, "Rajshahi Division"),
            Self::RangpurDivision => write!(f, "Rangpur Division"),
            Self::SylhetDivision => write!(f, "Sylhet Division"),
        }
    }
}

// Barbados Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarbadosSubregions {
    Barbados,
}

impl TryFrom<u8> for BarbadosSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Barbados),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BarbadosSubregions> for u8 {
    fn from(value: BarbadosSubregions) -> u8 {
        match value {
            BarbadosSubregions::Barbados => 0x01,
        }
    }
}

impl Display for BarbadosSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Barbados => write!(f, "Barbados"),
        }
    }
}

// Belarus Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelarusSubregions {
    Belarus,
    Minsk,
    Brest,
    Gomel,
    Grodno,
    Mogilev,
    Vitebsk,
}

impl TryFrom<u8> for BelarusSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Belarus),
            0x02 => Ok(Self::Minsk),
            0x03 => Ok(Self::Brest),
            0x04 => Ok(Self::Gomel),
            0x05 => Ok(Self::Grodno),
            0x06 => Ok(Self::Mogilev),
            0x07 => Ok(Self::Vitebsk),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BelarusSubregions> for u8 {
    fn from(value: BelarusSubregions) -> u8 {
        match value {
            BelarusSubregions::Belarus => 0x01,
            BelarusSubregions::Minsk => 0x02,
            BelarusSubregions::Brest => 0x03,
            BelarusSubregions::Gomel => 0x04,
            BelarusSubregions::Grodno => 0x05,
            BelarusSubregions::Mogilev => 0x06,
            BelarusSubregions::Vitebsk => 0x07,
        }
    }
}

impl Display for BelarusSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Belarus => write!(f, "Belarus"),
            Self::Minsk => write!(f, "Minsk"),
            Self::Brest => write!(f, "Brest"),
            Self::Gomel => write!(f, "Gomel"),
            Self::Grodno => write!(f, "Grodno"),
            Self::Mogilev => write!(f, "Mogilev"),
            Self::Vitebsk => write!(f, "Vitebsk"),
        }
    }
}

// Belgium Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelgiumSubregions {
    Belgium,
    BrusselsRegion,
    Flanders,
    Wallonia,
}

impl TryFrom<u8> for BelgiumSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Belgium),
            0x02 => Ok(Self::BrusselsRegion),
            0x03 => Ok(Self::Flanders),
            0x04 => Ok(Self::Wallonia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BelgiumSubregions> for u8 {
    fn from(value: BelgiumSubregions) -> u8 {
        match value {
            BelgiumSubregions::Belgium => 0x01,
            BelgiumSubregions::BrusselsRegion => 0x02,
            BelgiumSubregions::Flanders => 0x03,
            BelgiumSubregions::Wallonia => 0x04,
        }
    }
}

impl Display for BelgiumSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Belgium => write!(f, "Belgium"),
            Self::BrusselsRegion => write!(f, "Brussels Region"),
            Self::Flanders => write!(f, "Flanders"),
            Self::Wallonia => write!(f, "Wallonia"),
        }
    }
}

// Belize Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BelizeSubregions {
    Belize,
    Cayo,
    Corozal,
    OrangeWalk,
    StannCreek,
    Toledo,
}

impl TryFrom<u8> for BelizeSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Belize),
            0x02 => Ok(Self::Cayo),
            0x03 => Ok(Self::Belize),
            0x04 => Ok(Self::Corozal),
            0x05 => Ok(Self::OrangeWalk),
            0x06 => Ok(Self::StannCreek),
            0x07 => Ok(Self::Toledo),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BelizeSubregions> for u8 {
    fn from(value: BelizeSubregions) -> u8 {
        match value {
            BelizeSubregions::Belize => 0x01,
            BelizeSubregions::Cayo => 0x02,
            BelizeSubregions::Corozal => 0x04,
            BelizeSubregions::OrangeWalk => 0x05,
            BelizeSubregions::StannCreek => 0x06,
            BelizeSubregions::Toledo => 0x07,
        }
    }
}

impl Display for BelizeSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Belize => write!(f, "Belize"),
            Self::Cayo => write!(f, "Cayo"),
            Self::Corozal => write!(f, "Corozal"),
            Self::OrangeWalk => write!(f, "Orange Walk"),
            Self::StannCreek => write!(f, "Stann Creek"),
            Self::Toledo => write!(f, "Toledo"),
        }
    }
}

// Benin Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BeninSubregions {
    Benin,
    Oueme,
    Alibori,
    Atakora,
    Atlantique,
    Borgou,
    Collines,
    Donga,
    Kouffo,
    Littoral,
    Mono,
    Plateau,
    Zou,
}

impl TryFrom<u8> for BeninSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Benin),
            0x02 => Ok(Self::Oueme),
            0x03 => Ok(Self::Alibori),
            0x04 => Ok(Self::Atakora),
            0x05 => Ok(Self::Atlantique),
            0x06 => Ok(Self::Borgou),
            0x07 => Ok(Self::Collines),
            0x08 => Ok(Self::Donga),
            0x09 => Ok(Self::Kouffo),
            0x0A => Ok(Self::Littoral),
            0x0B => Ok(Self::Mono),
            0x0C => Ok(Self::Plateau),
            0x0D => Ok(Self::Zou),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BeninSubregions> for u8 {
    fn from(value: BeninSubregions) -> u8 {
        match value {
            BeninSubregions::Benin => 0x01,
            BeninSubregions::Oueme => 0x02,
            BeninSubregions::Alibori => 0x03,
            BeninSubregions::Atakora => 0x04,
            BeninSubregions::Atlantique => 0x05,
            BeninSubregions::Borgou => 0x06,
            BeninSubregions::Collines => 0x07,
            BeninSubregions::Donga => 0x08,
            BeninSubregions::Kouffo => 0x09,
            BeninSubregions::Littoral => 0x0A,
            BeninSubregions::Mono => 0x0B,
            BeninSubregions::Plateau => 0x0C,
            BeninSubregions::Zou => 0x0D,
        }
    }
}

impl Display for BeninSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Benin => write!(f, "Benin"),
            Self::Oueme => write!(f, "Ouémé"),
            Self::Alibori => write!(f, "Alibori"),
            Self::Atakora => write!(f, "Atakora"),
            Self::Atlantique => write!(f, "Atlantique"),
            Self::Borgou => write!(f, "Borgou"),
            Self::Collines => write!(f, "Collines"),
            Self::Donga => write!(f, "Donga"),
            Self::Kouffo => write!(f, "Kouffo"),
            Self::Littoral => write!(f, "Littoral"),
            Self::Mono => write!(f, "Mono"),
            Self::Plateau => write!(f, "Plateau"),
            Self::Zou => write!(f, "Zou"),
        }
    }
}

// Bermuda Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BermudaSubregions {
    Bermuda,
}

impl TryFrom<u8> for BermudaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bermuda),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BermudaSubregions> for u8 {
    fn from(value: BermudaSubregions) -> u8 {
        match value {
            BermudaSubregions::Bermuda => 0x01,
        }
    }
}

impl Display for BermudaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bermuda => write!(f, "Bermuda"),
        }
    }
}

// Bhutan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BhutanSubregions {
    Bhutan,
    ThimphuDistrict,
    BumthangDistrict,
    ChukhaDistrict,
    DaganaDistrict,
    GasaDistrict,
    HaaDistrict,
    LhuntseDistrict,
    Mongar,
    Paro,
    PemagatshelDistrict,
    PunakhaDistrict,
    SamdrupJongkharDistrict,
    SamtseDistrict,
    SarpangDistrict,
    TrashigangDistrict,
    TrashiyangtseDistrict,
    TrongsaDistrict,
    TsirangDistrict,
    WangduePhodrangDistrict,
    ZhemgangDistrict,
}

impl TryFrom<u8> for BhutanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bhutan),
            0x02 => Ok(Self::ThimphuDistrict),
            0x03 => Ok(Self::BumthangDistrict),
            0x04 => Ok(Self::ChukhaDistrict),
            0x05 => Ok(Self::DaganaDistrict),
            0x06 => Ok(Self::GasaDistrict),
            0x07 => Ok(Self::HaaDistrict),
            0x08 => Ok(Self::LhuntseDistrict),
            0x09 => Ok(Self::Mongar),
            0x0A => Ok(Self::Paro),
            0x0B => Ok(Self::PemagatshelDistrict),
            0x0C => Ok(Self::PunakhaDistrict),
            0x0D => Ok(Self::SamdrupJongkharDistrict),
            0x0E => Ok(Self::SamtseDistrict),
            0x0F => Ok(Self::SarpangDistrict),
            0x10 => Ok(Self::TrashigangDistrict),
            0x11 => Ok(Self::TrashiyangtseDistrict),
            0x12 => Ok(Self::TrongsaDistrict),
            0x13 => Ok(Self::TsirangDistrict),
            0x14 => Ok(Self::WangduePhodrangDistrict),
            0x15 => Ok(Self::ZhemgangDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BhutanSubregions> for u8 {
    fn from(value: BhutanSubregions) -> u8 {
        match value {
            BhutanSubregions::Bhutan => 0x01,
            BhutanSubregions::ThimphuDistrict => 0x02,
            BhutanSubregions::BumthangDistrict => 0x03,
            BhutanSubregions::ChukhaDistrict => 0x04,
            BhutanSubregions::DaganaDistrict => 0x05,
            BhutanSubregions::GasaDistrict => 0x06,
            BhutanSubregions::HaaDistrict => 0x07,
            BhutanSubregions::LhuntseDistrict => 0x08,
            BhutanSubregions::Mongar => 0x09,
            BhutanSubregions::Paro => 0x0A,
            BhutanSubregions::PemagatshelDistrict => 0x0B,
            BhutanSubregions::PunakhaDistrict => 0x0C,
            BhutanSubregions::SamdrupJongkharDistrict => 0x0D,
            BhutanSubregions::SamtseDistrict => 0x0E,
            BhutanSubregions::SarpangDistrict => 0x0F,
            BhutanSubregions::TrashigangDistrict => 0x10,
            BhutanSubregions::TrashiyangtseDistrict => 0x11,
            BhutanSubregions::TrongsaDistrict => 0x12,
            BhutanSubregions::TsirangDistrict => 0x13,
            BhutanSubregions::WangduePhodrangDistrict => 0x14,
            BhutanSubregions::ZhemgangDistrict => 0x15,
        }
    }
}

impl Display for BhutanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bhutan => write!(f, "Bhutan"),
            Self::ThimphuDistrict => write!(f, "Thimphu District"),
            Self::BumthangDistrict => write!(f, "Bumthang District"),
            Self::ChukhaDistrict => write!(f, "Chukha District"),
            Self::DaganaDistrict => write!(f, "Dagana District"),
            Self::GasaDistrict => write!(f, "Gasa District"),
            Self::HaaDistrict => write!(f, "Haa District"),
            Self::LhuntseDistrict => write!(f, "Lhuntse District"),
            Self::Mongar => write!(f, "Mongar"),
            Self::Paro => write!(f, "Paro"),
            Self::PemagatshelDistrict => write!(f, "Pemagatshel District"),
            Self::PunakhaDistrict => write!(f, "Punakha District"),
            Self::SamdrupJongkharDistrict => write!(f, "Samdrup Jongkhar District"),
            Self::SamtseDistrict => write!(f, "Samtse District"),
            Self::SarpangDistrict => write!(f, "Sarpang District"),
            Self::TrashigangDistrict => write!(f, "Trashigang District"),
            Self::TrashiyangtseDistrict => write!(f, "Trashiyangtse District"),
            Self::TrongsaDistrict => write!(f, "Trongsa District"),
            Self::TsirangDistrict => write!(f, "Tsirang District"),
            Self::WangduePhodrangDistrict => write!(f, "Wangdue Phodrang District"),
            Self::ZhemgangDistrict => write!(f, "Zhemgang District"),
        }
    }
}

// Bolivia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoliviaSubregions {
    Bolivia,
    LaPaz,
    Chuquisaca,
    Cochabamba,
    ElBeni,
    Oruro,
    Pando,
    Potosi,
    SantaCruz,
    Tarija,
}

impl TryFrom<u8> for BoliviaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bolivia),
            0x02 => Ok(Self::LaPaz),
            0x03 => Ok(Self::Chuquisaca),
            0x04 => Ok(Self::Cochabamba),
            0x05 => Ok(Self::ElBeni),
            0x06 => Ok(Self::Oruro),
            0x07 => Ok(Self::Pando),
            0x08 => Ok(Self::Potosi),
            0x09 => Ok(Self::SantaCruz),
            0x0A => Ok(Self::Tarija),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BoliviaSubregions> for u8 {
    fn from(value: BoliviaSubregions) -> u8 {
        match value {
            BoliviaSubregions::Bolivia => 0x01,
            BoliviaSubregions::LaPaz => 0x02,
            BoliviaSubregions::Chuquisaca => 0x03,
            BoliviaSubregions::Cochabamba => 0x04,
            BoliviaSubregions::ElBeni => 0x05,
            BoliviaSubregions::Oruro => 0x06,
            BoliviaSubregions::Pando => 0x07,
            BoliviaSubregions::Potosi => 0x08,
            BoliviaSubregions::SantaCruz => 0x09,
            BoliviaSubregions::Tarija => 0x0A,
        }
    }
}

impl Display for BoliviaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bolivia => write!(f, "Bolivia"),
            Self::LaPaz => write!(f, "La Paz"),
            Self::Chuquisaca => write!(f, "Chuquisaca"),
            Self::Cochabamba => write!(f, "Cochabamba"),
            Self::ElBeni => write!(f, "El Beni"),
            Self::Oruro => write!(f, "Oruro"),
            Self::Pando => write!(f, "Pando"),
            Self::Potosi => write!(f, "Potosí"),
            Self::SantaCruz => write!(f, "Santa Cruz"),
            Self::Tarija => write!(f, "Tarija"),
        }
    }
}

// Bosnia & Herzegovina Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BosniaAndHerzegovinaSubregions {
    BosniaAndHerzegovina,
    FedOfBosniaAndHerzegovina,
    RepublikaSrpska,
    BrckoDistrikt,
}

impl TryFrom<u8> for BosniaAndHerzegovinaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::BosniaAndHerzegovina),
            0x02 => Ok(Self::FedOfBosniaAndHerzegovina),
            0x03 => Ok(Self::RepublikaSrpska),
            0x04 => Ok(Self::BrckoDistrikt),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BosniaAndHerzegovinaSubregions> for u8 {
    fn from(value: BosniaAndHerzegovinaSubregions) -> u8 {
        match value {
            BosniaAndHerzegovinaSubregions::BosniaAndHerzegovina => 0x01,
            BosniaAndHerzegovinaSubregions::FedOfBosniaAndHerzegovina => 0x02,
            BosniaAndHerzegovinaSubregions::RepublikaSrpska => 0x03,
            BosniaAndHerzegovinaSubregions::BrckoDistrikt => 0x04,
        }
    }
}

impl Display for BosniaAndHerzegovinaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BosniaAndHerzegovina => write!(f, "Bosnia & Herzegovina"),
            Self::FedOfBosniaAndHerzegovina => write!(f, "Fed. of Bosnia and Herzegovina"),
            Self::RepublikaSrpska => write!(f, "Republika Srpska"),
            Self::BrckoDistrikt => write!(f, "Brcko Distrikt"),
        }
    }
}

// Botswana Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BotswanaSubregions {
    Botswana,
    SouthEast,
    Central,
    Ghanzi,
    Kgalagadi,
    Kgatleng,
    Kweneng,
    NorthEast,
    NorthWest,
    Southern,
}

impl TryFrom<u8> for BotswanaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Botswana),
            0x02 => Ok(Self::SouthEast),
            0x03 => Ok(Self::Central),
            0x04 => Ok(Self::Ghanzi),
            0x05 => Ok(Self::Kgalagadi),
            0x06 => Ok(Self::Kgatleng),
            0x07 => Ok(Self::Kweneng),
            0x08 => Ok(Self::NorthEast),
            0x09 => Ok(Self::NorthWest),
            0x0A => Ok(Self::Southern),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BotswanaSubregions> for u8 {
    fn from(value: BotswanaSubregions) -> u8 {
        match value {
            BotswanaSubregions::Botswana => 0x01,
            BotswanaSubregions::SouthEast => 0x02,
            BotswanaSubregions::Central => 0x03,
            BotswanaSubregions::Ghanzi => 0x04,
            BotswanaSubregions::Kgalagadi => 0x05,
            BotswanaSubregions::Kgatleng => 0x06,
            BotswanaSubregions::Kweneng => 0x07,
            BotswanaSubregions::NorthEast => 0x08,
            BotswanaSubregions::NorthWest => 0x09,
            BotswanaSubregions::Southern => 0x0A,
        }
    }
}

impl Display for BotswanaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Botswana => write!(f, "Botswana"),
            Self::SouthEast => write!(f, "South-East"),
            Self::Central => write!(f, "Central"),
            Self::Ghanzi => write!(f, "Ghanzi"),
            Self::Kgalagadi => write!(f, "Kgalagadi"),
            Self::Kgatleng => write!(f, "Kgatleng"),
            Self::Kweneng => write!(f, "Kweneng"),
            Self::NorthEast => write!(f, "North-East"),
            Self::NorthWest => write!(f, "North-West"),
            Self::Southern => write!(f, "Southern"),
        }
    }
}

// Brazil Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrazilSubregions {
    Brazil,
    DistritoFederal,
    Acre,
    Alagoas,
    Amapa,
    Amazonas,
    Bahia,
    Ceara,
    EspiritoSanto,
    MatoGrossoDoSul,
    Maranhao,
    MatoGrosso,
    MinasGerais,
    Para,
    Paraiba,
    Parana,
    Piaui,
    RioDeJaneiro,
    RioGrandeDoNorte,
    RioGrandeDoSul,
    Rondonia,
    Roraima,
    SantaCatarina,
    SaoPaulo,
    Sergipe,
    Goias,
    Pernambuco,
    Tocantins,
}

impl TryFrom<u8> for BrazilSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Brazil),
            0x02 => Ok(Self::DistritoFederal),
            0x03 => Ok(Self::Acre),
            0x04 => Ok(Self::Alagoas),
            0x05 => Ok(Self::Amapa),
            0x06 => Ok(Self::Amazonas),
            0x07 => Ok(Self::Bahia),
            0x08 => Ok(Self::Ceara),
            0x09 => Ok(Self::EspiritoSanto),
            0x0A => Ok(Self::MatoGrossoDoSul),
            0x0B => Ok(Self::Maranhao),
            0x0C => Ok(Self::MatoGrosso),
            0x0D => Ok(Self::MinasGerais),
            0x0E => Ok(Self::Para),
            0x0F => Ok(Self::Paraiba),
            0x10 => Ok(Self::Parana),
            0x11 => Ok(Self::Piaui),
            0x12 => Ok(Self::RioDeJaneiro),
            0x13 => Ok(Self::RioGrandeDoNorte),
            0x14 => Ok(Self::RioGrandeDoSul),
            0x15 => Ok(Self::Rondonia),
            0x16 => Ok(Self::Roraima),
            0x17 => Ok(Self::SantaCatarina),
            0x18 => Ok(Self::SaoPaulo),
            0x19 => Ok(Self::Sergipe),
            0x1A => Ok(Self::Goias),
            0x1B => Ok(Self::Pernambuco),
            0x1C => Ok(Self::Tocantins),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BrazilSubregions> for u8 {
    fn from(value: BrazilSubregions) -> u8 {
        match value {
            BrazilSubregions::Brazil => 0x01,
            BrazilSubregions::DistritoFederal => 0x02,
            BrazilSubregions::Acre => 0x03,
            BrazilSubregions::Alagoas => 0x04,
            BrazilSubregions::Amapa => 0x05,
            BrazilSubregions::Amazonas => 0x06,
            BrazilSubregions::Bahia => 0x07,
            BrazilSubregions::Ceara => 0x08,
            BrazilSubregions::EspiritoSanto => 0x09,
            BrazilSubregions::MatoGrossoDoSul => 0x0A,
            BrazilSubregions::Maranhao => 0x0B,
            BrazilSubregions::MatoGrosso => 0x0C,
            BrazilSubregions::MinasGerais => 0x0D,
            BrazilSubregions::Para => 0x0E,
            BrazilSubregions::Paraiba => 0x0F,
            BrazilSubregions::Parana => 0x10,
            BrazilSubregions::Piaui => 0x11,
            BrazilSubregions::RioDeJaneiro => 0x12,
            BrazilSubregions::RioGrandeDoNorte => 0x13,
            BrazilSubregions::RioGrandeDoSul => 0x14,
            BrazilSubregions::Rondonia => 0x15,
            BrazilSubregions::Roraima => 0x16,
            BrazilSubregions::SantaCatarina => 0x17,
            BrazilSubregions::SaoPaulo => 0x18,
            BrazilSubregions::Sergipe => 0x19,
            BrazilSubregions::Goias => 0x1A,
            BrazilSubregions::Pernambuco => 0x1B,
            BrazilSubregions::Tocantins => 0x1C,
        }
    }
}

impl Display for BrazilSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brazil => write!(f, "Brazil"),
            Self::DistritoFederal => write!(f, "Distrito Federal"),
            Self::Acre => write!(f, "Acre"),
            Self::Alagoas => write!(f, "Alagoas"),
            Self::Amapa => write!(f, "Amapá"),
            Self::Amazonas => write!(f, "Amazonas"),
            Self::Bahia => write!(f, "Bahia"),
            Self::Ceara => write!(f, "Ceará"),
            Self::EspiritoSanto => write!(f, "Espírito Santo"),
            Self::MatoGrossoDoSul => write!(f, "Mato Grosso do Sul"),
            Self::Maranhao => write!(f, "Maranhão"),
            Self::MatoGrosso => write!(f, "Mato Grosso"),
            Self::MinasGerais => write!(f, "Minas Gerais"),
            Self::Para => write!(f, "Pará"),
            Self::Paraiba => write!(f, "Paraíba"),
            Self::Parana => write!(f, "Paraná"),
            Self::Piaui => write!(f, "Piauí"),
            Self::RioDeJaneiro => write!(f, "Rio de Janeiro"),
            Self::RioGrandeDoNorte => write!(f, "Rio Grande do Norte"),
            Self::RioGrandeDoSul => write!(f, "Rio Grande do Sul"),
            Self::Rondonia => write!(f, "Rondônia"),
            Self::Roraima => write!(f, "Roraima"),
            Self::SantaCatarina => write!(f, "Santa Catarina"),
            Self::SaoPaulo => write!(f, "São Paulo"),
            Self::Sergipe => write!(f, "Sergipe"),
            Self::Goias => write!(f, "Goiás"),
            Self::Pernambuco => write!(f, "Pernambuco"),
            Self::Tocantins => write!(f, "Tocantins"),
        }
    }
}

// British Antarctic Territory Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BritishAntarcticTerritorySubregions {
    BritishAntarcticTerritory,
    AdelaideIsland,
    AlexanderIsland,
    BruntIceShelf,
    EllsworthLand,
    GrahamLand,
    QueenElizabethLand,
    SignyIsland,
    SouthOrkneyIslands,
    SouthShetlandIslands,
}

impl TryFrom<u8> for BritishAntarcticTerritorySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::BritishAntarcticTerritory),
            0x02 => Ok(Self::AdelaideIsland),
            0x03 => Ok(Self::AlexanderIsland),
            0x04 => Ok(Self::BruntIceShelf),
            0x05 => Ok(Self::EllsworthLand),
            0x06 => Ok(Self::GrahamLand),
            0x07 => Ok(Self::QueenElizabethLand),
            0x08 => Ok(Self::SignyIsland),
            0x09 => Ok(Self::SouthOrkneyIslands),
            0x0A => Ok(Self::SouthShetlandIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BritishAntarcticTerritorySubregions> for u8 {
    fn from(value: BritishAntarcticTerritorySubregions) -> u8 {
        match value {
            BritishAntarcticTerritorySubregions::BritishAntarcticTerritory => 0x01,
            BritishAntarcticTerritorySubregions::AdelaideIsland => 0x02,
            BritishAntarcticTerritorySubregions::AlexanderIsland => 0x03,
            BritishAntarcticTerritorySubregions::BruntIceShelf => 0x04,
            BritishAntarcticTerritorySubregions::EllsworthLand => 0x05,
            BritishAntarcticTerritorySubregions::GrahamLand => 0x06,
            BritishAntarcticTerritorySubregions::QueenElizabethLand => 0x07,
            BritishAntarcticTerritorySubregions::SignyIsland => 0x08,
            BritishAntarcticTerritorySubregions::SouthOrkneyIslands => 0x09,
            BritishAntarcticTerritorySubregions::SouthShetlandIslands => 0x0A,
        }
    }
}

impl Display for BritishAntarcticTerritorySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BritishAntarcticTerritory => write!(f, "British Antarctic Territory"),
            Self::AdelaideIsland => write!(f, "Adelaide Island"),
            Self::AlexanderIsland => write!(f, "Alexander Island"),
            Self::BruntIceShelf => write!(f, "Brunt Ice Shelf"),
            Self::EllsworthLand => write!(f, "Ellsworth Land"),
            Self::GrahamLand => write!(f, "Graham Land"),
            Self::QueenElizabethLand => write!(f, "Queen Elizabeth Land"),
            Self::SignyIsland => write!(f, "Signy Island"),
            Self::SouthOrkneyIslands => write!(f, "South Orkney Islands"),
            Self::SouthShetlandIslands => write!(f, "South Shetland Islands"),
        }
    }
}

// British Indian Ocean Territory Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BritishIndianOceanTerritorySubregions {
    BritishIndianOceanTerritory,
}

impl TryFrom<u8> for BritishIndianOceanTerritorySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::BritishIndianOceanTerritory),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BritishIndianOceanTerritorySubregions> for u8 {
    fn from(value: BritishIndianOceanTerritorySubregions) -> u8 {
        match value {
            BritishIndianOceanTerritorySubregions::BritishIndianOceanTerritory => 0x01,
        }
    }
}

impl Display for BritishIndianOceanTerritorySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BritishIndianOceanTerritory => write!(f, "British Indian Ocean Territory"),
        }
    }
}

// British Virgin Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BritishVirginIslandsSubregions {
    BritishVirginIslands,
    Tortola,
    Anegada,
    JostVanDyke,
    VirginGorda,
    Other,
}

impl TryFrom<u8> for BritishVirginIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::BritishVirginIslands),
            0x02 => Ok(Self::Tortola),
            0x03 => Ok(Self::Anegada),
            0x04 => Ok(Self::JostVanDyke),
            0x05 => Ok(Self::VirginGorda),
            0x06 => Ok(Self::Other),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BritishVirginIslandsSubregions> for u8 {
    fn from(value: BritishVirginIslandsSubregions) -> u8 {
        match value {
            BritishVirginIslandsSubregions::BritishVirginIslands => 0x01,
            BritishVirginIslandsSubregions::Tortola => 0x02,
            BritishVirginIslandsSubregions::Anegada => 0x03,
            BritishVirginIslandsSubregions::JostVanDyke => 0x04,
            BritishVirginIslandsSubregions::VirginGorda => 0x05,
            BritishVirginIslandsSubregions::Other => 0x06,
        }
    }
}

impl Display for BritishVirginIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BritishVirginIslands => write!(f, "British Virgin Islands"),
            Self::Tortola => write!(f, "Tortola"),
            Self::Anegada => write!(f, "Anegada"),
            Self::JostVanDyke => write!(f, "Jost Van Dyke"),
            Self::VirginGorda => write!(f, "Virgin Gorda"),
            Self::Other => write!(f, "Other"),
        }
    }
}

// Brunei Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BruneiSubregions {
    Brunei,
    BruneiMuaraDistrict,
    BelaitDistrict,
    TemburongDistrict,
    TutongDistrict,
}

impl TryFrom<u8> for BruneiSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Brunei),
            0x02 => Ok(Self::BruneiMuaraDistrict),
            0x03 => Ok(Self::BelaitDistrict),
            0x04 => Ok(Self::TemburongDistrict),
            0x05 => Ok(Self::TutongDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BruneiSubregions> for u8 {
    fn from(value: BruneiSubregions) -> u8 {
        match value {
            BruneiSubregions::Brunei => 0x01,
            BruneiSubregions::BruneiMuaraDistrict => 0x02,
            BruneiSubregions::BelaitDistrict => 0x03,
            BruneiSubregions::TemburongDistrict => 0x04,
            BruneiSubregions::TutongDistrict => 0x05,
        }
    }
}

impl Display for BruneiSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brunei => write!(f, "Brunei"),
            Self::BruneiMuaraDistrict => write!(f, "Brunei-Muara District"),
            Self::BelaitDistrict => write!(f, "Belait District"),
            Self::TemburongDistrict => write!(f, "Temburong District"),
            Self::TutongDistrict => write!(f, "Tutong District"),
        }
    }
}

// Bulgaria Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulgariaSubregions {
    Bulgaria,
    SofiaCity,
    SofiaProvince,
    Blagoevgrad,
    Pleven,
    Vidin,
    Varna,
    Burgas,
    Dobric,
    Gabrovo,
    Haskovo,
    Yambol,
    Kardzhali,
    Kyustendil,
    Lovech,
    Montana,
    Pazardzhik,
    Pernik,
    Plovdiv,
    Razgrad,
    Ruse,
    Silistra,
    Sliven,
    Smolyan,
    StaraZagora,
    Shumen,
    Targovishte,
    VelikoTarnovo,
    Vratsa,
}

impl TryFrom<u8> for BulgariaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bulgaria),
            0x02 => Ok(Self::SofiaCity),
            0x03 => Ok(Self::SofiaProvince),
            0x04 => Ok(Self::Blagoevgrad),
            0x05 => Ok(Self::Pleven),
            0x06 => Ok(Self::Vidin),
            0x07 => Ok(Self::Varna),
            0x08 => Ok(Self::Burgas),
            0x09 => Ok(Self::Dobric),
            0x0A => Ok(Self::Gabrovo),
            0x0B => Ok(Self::Haskovo),
            0x0C => Ok(Self::Yambol),
            0x0D => Ok(Self::Kardzhali),
            0x0E => Ok(Self::Kyustendil),
            0x0F => Ok(Self::Lovech),
            0x10 => Ok(Self::Montana),
            0x11 => Ok(Self::Pazardzhik),
            0x12 => Ok(Self::Pernik),
            0x13 => Ok(Self::Plovdiv),
            0x14 => Ok(Self::Razgrad),
            0x15 => Ok(Self::Ruse),
            0x16 => Ok(Self::Silistra),
            0x17 => Ok(Self::Sliven),
            0x18 => Ok(Self::Smolyan),
            0x19 => Ok(Self::StaraZagora),
            0x1A => Ok(Self::Shumen),
            0x1B => Ok(Self::Targovishte),
            0x1C => Ok(Self::VelikoTarnovo),
            0x1D => Ok(Self::Vratsa),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BulgariaSubregions> for u8 {
    fn from(value: BulgariaSubregions) -> u8 {
        match value {
            BulgariaSubregions::Bulgaria => 0x01,
            BulgariaSubregions::SofiaCity => 0x02,
            BulgariaSubregions::SofiaProvince => 0x03,
            BulgariaSubregions::Blagoevgrad => 0x04,
            BulgariaSubregions::Pleven => 0x05,
            BulgariaSubregions::Vidin => 0x06,
            BulgariaSubregions::Varna => 0x07,
            BulgariaSubregions::Burgas => 0x08,
            BulgariaSubregions::Dobric => 0x09,
            BulgariaSubregions::Gabrovo => 0x0A,
            BulgariaSubregions::Haskovo => 0x0B,
            BulgariaSubregions::Yambol => 0x0C,
            BulgariaSubregions::Kardzhali => 0x0D,
            BulgariaSubregions::Kyustendil => 0x0E,
            BulgariaSubregions::Lovech => 0x0F,
            BulgariaSubregions::Montana => 0x10,
            BulgariaSubregions::Pazardzhik => 0x11,
            BulgariaSubregions::Pernik => 0x12,
            BulgariaSubregions::Plovdiv => 0x13,
            BulgariaSubregions::Razgrad => 0x14,
            BulgariaSubregions::Ruse => 0x15,
            BulgariaSubregions::Silistra => 0x16,
            BulgariaSubregions::Sliven => 0x17,
            BulgariaSubregions::Smolyan => 0x18,
            BulgariaSubregions::StaraZagora => 0x19,
            BulgariaSubregions::Shumen => 0x1A,
            BulgariaSubregions::Targovishte => 0x1B,
            BulgariaSubregions::VelikoTarnovo => 0x1C,
            BulgariaSubregions::Vratsa => 0x1D,
        }
    }
}

impl Display for BulgariaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bulgaria => write!(f, "Bulgaria"),
            Self::SofiaCity => write!(f, "Sofia City"),
            Self::SofiaProvince => write!(f, "Sofia Province"),
            Self::Blagoevgrad => write!(f, "Blagoevgrad"),
            Self::Pleven => write!(f, "Pleven"),
            Self::Vidin => write!(f, "Vidin"),
            Self::Varna => write!(f, "Varna"),
            Self::Burgas => write!(f, "Burgas"),
            Self::Dobric => write!(f, "Dobric"),
            Self::Gabrovo => write!(f, "Gabrovo"),
            Self::Haskovo => write!(f, "Haskovo"),
            Self::Yambol => write!(f, "Yambol"),
            Self::Kardzhali => write!(f, "Kardzhali"),
            Self::Kyustendil => write!(f, "Kyustendil"),
            Self::Lovech => write!(f, "Lovech"),
            Self::Montana => write!(f, "Montana"),
            Self::Pazardzhik => write!(f, "Pazardzhik"),
            Self::Pernik => write!(f, "Pernik"),
            Self::Plovdiv => write!(f, "Plovdiv"),
            Self::Razgrad => write!(f, "Razgrad"),
            Self::Ruse => write!(f, "Ruse"),
            Self::Silistra => write!(f, "Silistra"),
            Self::Sliven => write!(f, "Sliven"),
            Self::Smolyan => write!(f, "Smolyan"),
            Self::StaraZagora => write!(f, "Stara Zagora"),
            Self::Shumen => write!(f, "Shumen"),
            Self::Targovishte => write!(f, "Targovishte"),
            Self::VelikoTarnovo => write!(f, "Veliko Tarnovo"),
            Self::Vratsa => write!(f, "Vratsa"),
        }
    }
}

// Burkina Faso Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BurkinaFasoSubregions {
    BurkinaFaso,
    Centre,
    BoucleDuMouhoun,
    Cascades,
    CentreEst,
    CentreNord,
    CentreOuest,
    CentreSud,
    Est,
    HautsBassins,
    Nord,
    PlateauCentral,
    Sahel,
    SudOuest,
}

impl TryFrom<u8> for BurkinaFasoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::BurkinaFaso),
            0x02 => Ok(Self::Centre),
            0x03 => Ok(Self::BoucleDuMouhoun),
            0x04 => Ok(Self::Cascades),
            0x05 => Ok(Self::CentreEst),
            0x06 => Ok(Self::CentreNord),
            0x07 => Ok(Self::CentreOuest),
            0x08 => Ok(Self::CentreSud),
            0x09 => Ok(Self::Est),
            0x0A => Ok(Self::HautsBassins),
            0x0B => Ok(Self::Nord),
            0x0C => Ok(Self::PlateauCentral),
            0x0D => Ok(Self::Sahel),
            0x0E => Ok(Self::SudOuest),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BurkinaFasoSubregions> for u8 {
    fn from(value: BurkinaFasoSubregions) -> u8 {
        match value {
            BurkinaFasoSubregions::BurkinaFaso => 0x01,
            BurkinaFasoSubregions::Centre => 0x02,
            BurkinaFasoSubregions::BoucleDuMouhoun => 0x03,
            BurkinaFasoSubregions::Cascades => 0x04,
            BurkinaFasoSubregions::CentreEst => 0x05,
            BurkinaFasoSubregions::CentreNord => 0x06,
            BurkinaFasoSubregions::CentreOuest => 0x07,
            BurkinaFasoSubregions::CentreSud => 0x08,
            BurkinaFasoSubregions::Est => 0x09,
            BurkinaFasoSubregions::HautsBassins => 0x0A,
            BurkinaFasoSubregions::Nord => 0x0B,
            BurkinaFasoSubregions::PlateauCentral => 0x0C,
            BurkinaFasoSubregions::Sahel => 0x0D,
            BurkinaFasoSubregions::SudOuest => 0x0E,
        }
    }
}

impl Display for BurkinaFasoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BurkinaFaso => write!(f, "Burkina Faso"),
            Self::Centre => write!(f, "Centre"),
            Self::BoucleDuMouhoun => write!(f, "Boucle du Mouhoun"),
            Self::Cascades => write!(f, "Cascades"),
            Self::CentreEst => write!(f, "Centre-Est"),
            Self::CentreNord => write!(f, "Centre-Nord"),
            Self::CentreOuest => write!(f, "Centre-Ouest"),
            Self::CentreSud => write!(f, "Centre-Sud"),
            Self::Est => write!(f, "Est"),
            Self::HautsBassins => write!(f, "Hauts-Bassins"),
            Self::Nord => write!(f, "Nord"),
            Self::PlateauCentral => write!(f, "Plateau-Central"),
            Self::Sahel => write!(f, "Sahel"),
            Self::SudOuest => write!(f, "Sud-Ouest"),
        }
    }
}

// Burundi Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BurundiSubregions {
    Burundi,
    Eastern,
    Northern,
    Southern,
    Western,
}

impl TryFrom<u8> for BurundiSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Burundi),
            0x02 => Ok(Self::Eastern),
            0x03 => Ok(Self::Northern),
            0x04 => Ok(Self::Southern),
            0x05 => Ok(Self::Western),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<BurundiSubregions> for u8 {
    fn from(value: BurundiSubregions) -> u8 {
        match value {
            BurundiSubregions::Burundi => 0x01,
            BurundiSubregions::Eastern => 0x02,
            BurundiSubregions::Northern => 0x03,
            BurundiSubregions::Southern => 0x04,
            BurundiSubregions::Western => 0x05,
        }
    }
}

impl Display for BurundiSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Burundi => write!(f, "Burundi"),
            Self::Eastern => write!(f, "Eastern"),
            Self::Northern => write!(f, "Northern"),
            Self::Southern => write!(f, "Southern"),
            Self::Western => write!(f, "Western"),
        }
    }
}

// Cambodia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CambodiaSubregions {
    Cambodia,
    PhnomPenh,
    BanteayMeanchey,
    Battambang,
    KampongCham,
    KampongChhnang,
    KampongSpeu,
    KampongThom,
    Kampot,
    Kandal,
    Kep,
    KohKong,
    Kratie,
    Mondulkiri,
    OddarMeanchey,
    Pailin,
    PreahSihanouk,
    PreahVihear,
    PreyVeng,
    Pursat,
    RatanakKiri,
    SiemReap,
    SteungTreng,
    SvayRieng,
    Takeo,
    TboungKhmum,
}

impl TryFrom<u8> for CambodiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Cambodia),
            0x02 => Ok(Self::PhnomPenh),
            0x03 => Ok(Self::BanteayMeanchey),
            0x04 => Ok(Self::Battambang),
            0x05 => Ok(Self::KampongCham),
            0x06 => Ok(Self::KampongChhnang),
            0x07 => Ok(Self::KampongSpeu),
            0x08 => Ok(Self::KampongThom),
            0x09 => Ok(Self::Kampot),
            0x0A => Ok(Self::Kandal),
            0x0B => Ok(Self::Kep),
            0x0C => Ok(Self::KohKong),
            0x0D => Ok(Self::Kratie),
            0x0E => Ok(Self::Mondulkiri),
            0x0F => Ok(Self::OddarMeanchey),
            0x10 => Ok(Self::Pailin),
            0x11 => Ok(Self::PreahSihanouk),
            0x12 => Ok(Self::PreahVihear),
            0x13 => Ok(Self::PreyVeng),
            0x14 => Ok(Self::Pursat),
            0x15 => Ok(Self::RatanakKiri),
            0x16 => Ok(Self::SiemReap),
            0x17 => Ok(Self::SteungTreng),
            0x18 => Ok(Self::SvayRieng),
            0x19 => Ok(Self::Takeo),
            0x1A => Ok(Self::TboungKhmum),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CambodiaSubregions> for u8 {
    fn from(value: CambodiaSubregions) -> u8 {
        match value {
            CambodiaSubregions::Cambodia => 0x01,
            CambodiaSubregions::PhnomPenh => 0x02,
            CambodiaSubregions::BanteayMeanchey => 0x03,
            CambodiaSubregions::Battambang => 0x04,
            CambodiaSubregions::KampongCham => 0x05,
            CambodiaSubregions::KampongChhnang => 0x06,
            CambodiaSubregions::KampongSpeu => 0x07,
            CambodiaSubregions::KampongThom => 0x08,
            CambodiaSubregions::Kampot => 0x09,
            CambodiaSubregions::Kandal => 0x0A,
            CambodiaSubregions::Kep => 0x0B,
            CambodiaSubregions::KohKong => 0x0C,
            CambodiaSubregions::Kratie => 0x0D,
            CambodiaSubregions::Mondulkiri => 0x0E,
            CambodiaSubregions::OddarMeanchey => 0x0F,
            CambodiaSubregions::Pailin => 0x10,
            CambodiaSubregions::PreahSihanouk => 0x11,
            CambodiaSubregions::PreahVihear => 0x12,
            CambodiaSubregions::PreyVeng => 0x13,
            CambodiaSubregions::Pursat => 0x14,
            CambodiaSubregions::RatanakKiri => 0x15,
            CambodiaSubregions::SiemReap => 0x16,
            CambodiaSubregions::SteungTreng => 0x17,
            CambodiaSubregions::SvayRieng => 0x18,
            CambodiaSubregions::Takeo => 0x19,
            CambodiaSubregions::TboungKhmum => 0x1A,
        }
    }
}

impl Display for CambodiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cambodia => write!(f, "Cambodia"),
            Self::PhnomPenh => write!(f, "Phnom Penh"),
            Self::BanteayMeanchey => write!(f, "Banteay Meanchey"),
            Self::Battambang => write!(f, "Battambang"),
            Self::KampongCham => write!(f, "Kampong Cham"),
            Self::KampongChhnang => write!(f, "Kampong Chhnang"),
            Self::KampongSpeu => write!(f, "Kampong Speu"),
            Self::KampongThom => write!(f, "Kampong Thom"),
            Self::Kampot => write!(f, "Kampot"),
            Self::Kandal => write!(f, "Kandal"),
            Self::Kep => write!(f, "Kep"),
            Self::KohKong => write!(f, "Koh Kong"),
            Self::Kratie => write!(f, "Kratié"),
            Self::Mondulkiri => write!(f, "Mondulkiri"),
            Self::OddarMeanchey => write!(f, "Oddar Meanchey"),
            Self::Pailin => write!(f, "Pailin"),
            Self::PreahSihanouk => write!(f, "Preah Sihanouk"),
            Self::PreahVihear => write!(f, "Preah Vihear"),
            Self::PreyVeng => write!(f, "Prey Veng"),
            Self::Pursat => write!(f, "Pursat"),
            Self::RatanakKiri => write!(f, "Ratanak Kiri"),
            Self::SiemReap => write!(f, "Siem Reap"),
            Self::SteungTreng => write!(f, "Steung Treng"),
            Self::SvayRieng => write!(f, "Svay Rieng"),
            Self::Takeo => write!(f, "Takéo"),
            Self::TboungKhmum => write!(f, "Tboung Khmum"),
        }
    }
}

// Cameroon Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameroonSubregions {
    Cameroon,
    CentralCameroon,
    Adamawa,
    EastCameroon,
    FarNorthCameroon,
    LittoralCameroon,
    NorthCameroon,
    NorthwestCameroon,
    SouthCameroon,
    SouthwestCameroon,
    WestCameroon,
}

impl TryFrom<u8> for CameroonSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Cameroon),
            0x02 => Ok(Self::CentralCameroon),
            0x03 => Ok(Self::Adamawa),
            0x04 => Ok(Self::EastCameroon),
            0x05 => Ok(Self::FarNorthCameroon),
            0x06 => Ok(Self::LittoralCameroon),
            0x07 => Ok(Self::NorthCameroon),
            0x08 => Ok(Self::NorthwestCameroon),
            0x09 => Ok(Self::SouthCameroon),
            0x0A => Ok(Self::SouthwestCameroon),
            0x0B => Ok(Self::WestCameroon),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CameroonSubregions> for u8 {
    fn from(value: CameroonSubregions) -> u8 {
        match value {
            CameroonSubregions::Cameroon => 0x01,
            CameroonSubregions::CentralCameroon => 0x02,
            CameroonSubregions::Adamawa => 0x03,
            CameroonSubregions::EastCameroon => 0x04,
            CameroonSubregions::FarNorthCameroon => 0x05,
            CameroonSubregions::LittoralCameroon => 0x06,
            CameroonSubregions::NorthCameroon => 0x07,
            CameroonSubregions::NorthwestCameroon => 0x08,
            CameroonSubregions::SouthCameroon => 0x09,
            CameroonSubregions::SouthwestCameroon => 0x0A,
            CameroonSubregions::WestCameroon => 0x0B,
        }
    }
}

impl Display for CameroonSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cameroon => write!(f, "Cameroon"),
            Self::CentralCameroon => write!(f, "Central Cameroon"),
            Self::Adamawa => write!(f, "Adamawa"),
            Self::EastCameroon => write!(f, "East Cameroon"),
            Self::FarNorthCameroon => write!(f, "Far North Cameroon"),
            Self::LittoralCameroon => write!(f, "Littoral Cameroon"),
            Self::NorthCameroon => write!(f, "North Cameroon"),
            Self::NorthwestCameroon => write!(f, "Northwest Cameroon"),
            Self::SouthCameroon => write!(f, "South Cameroon"),
            Self::SouthwestCameroon => write!(f, "Southwest Cameroon"),
            Self::WestCameroon => write!(f, "West Cameroon"),
        }
    }
}

// Canada Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanadaSubregions {
    Canada,
    Ontario,
    Alberta,
    BritishColumbia,
    Manitoba,
    NewBrunswick,
    Newfoundland,
    NovaScotia,
    PrinceEdwardIsland,
    Quebec,
    Saskatchewan,
    Yukon,
    NorthwestTerritories,
    Nunavut,
}

impl TryFrom<u8> for CanadaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Canada),
            0x02 => Ok(Self::Ontario),
            0x03 => Ok(Self::Alberta),
            0x04 => Ok(Self::BritishColumbia),
            0x05 => Ok(Self::Manitoba),
            0x06 => Ok(Self::NewBrunswick),
            0x07 => Ok(Self::Newfoundland),
            0x08 => Ok(Self::NovaScotia),
            0x09 => Ok(Self::PrinceEdwardIsland),
            0x0A => Ok(Self::Quebec),
            0x0B => Ok(Self::Saskatchewan),
            0x0C => Ok(Self::Yukon),
            0x0D => Ok(Self::NorthwestTerritories),
            0x0E => Ok(Self::Nunavut),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CanadaSubregions> for u8 {
    fn from(value: CanadaSubregions) -> u8 {
        match value {
            CanadaSubregions::Canada => 0x01,
            CanadaSubregions::Ontario => 0x02,
            CanadaSubregions::Alberta => 0x03,
            CanadaSubregions::BritishColumbia => 0x04,
            CanadaSubregions::Manitoba => 0x05,
            CanadaSubregions::NewBrunswick => 0x06,
            CanadaSubregions::Newfoundland => 0x07,
            CanadaSubregions::NovaScotia => 0x08,
            CanadaSubregions::PrinceEdwardIsland => 0x09,
            CanadaSubregions::Quebec => 0x0A,
            CanadaSubregions::Saskatchewan => 0x0B,
            CanadaSubregions::Yukon => 0x0C,
            CanadaSubregions::NorthwestTerritories => 0x0D,
            CanadaSubregions::Nunavut => 0x0E,
        }
    }
}

impl Display for CanadaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Canada => write!(f, "Canada"),
            Self::Ontario => write!(f, "Ontario"),
            Self::Alberta => write!(f, "Alberta"),
            Self::BritishColumbia => write!(f, "British Columbia"),
            Self::Manitoba => write!(f, "Manitoba"),
            Self::NewBrunswick => write!(f, "New Brunswick"),
            Self::Newfoundland => write!(f, "Newfoundland"),
            Self::NovaScotia => write!(f, "Nova Scotia"),
            Self::PrinceEdwardIsland => write!(f, "Prince Edward Island"),
            Self::Quebec => write!(f, "Quebec"),
            Self::Saskatchewan => write!(f, "Saskatchewan"),
            Self::Yukon => write!(f, "Yukon"),
            Self::NorthwestTerritories => write!(f, "Northwest Territories"),
            Self::Nunavut => write!(f, "Nunavut"),
        }
    }
}

// Cape Verde Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapeVerdeSubregions {
    CapeVerde,
    Santiago,
    BoaVista,
    Brava,
    Fogo,
    Maio,
    Sal,
    SantoAntao,
    SaoNicolau,
    SaoVicente,
}

impl TryFrom<u8> for CapeVerdeSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CapeVerde),
            0x02 => Ok(Self::Santiago),
            0x03 => Ok(Self::BoaVista),
            0x04 => Ok(Self::Brava),
            0x05 => Ok(Self::Fogo),
            0x06 => Ok(Self::Maio),
            0x07 => Ok(Self::Sal),
            0x08 => Ok(Self::SantoAntao),
            0x09 => Ok(Self::SaoNicolau),
            0x0A => Ok(Self::SaoVicente),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CapeVerdeSubregions> for u8 {
    fn from(value: CapeVerdeSubregions) -> u8 {
        match value {
            CapeVerdeSubregions::CapeVerde => 0x01,
            CapeVerdeSubregions::Santiago => 0x02,
            CapeVerdeSubregions::BoaVista => 0x03,
            CapeVerdeSubregions::Brava => 0x04,
            CapeVerdeSubregions::Fogo => 0x05,
            CapeVerdeSubregions::Maio => 0x06,
            CapeVerdeSubregions::Sal => 0x07,
            CapeVerdeSubregions::SantoAntao => 0x08,
            CapeVerdeSubregions::SaoNicolau => 0x09,
            CapeVerdeSubregions::SaoVicente => 0x0A,
        }
    }
}

impl Display for CapeVerdeSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CapeVerde => write!(f, "Cape Verde"),
            Self::Santiago => write!(f, "Santiago"),
            Self::BoaVista => write!(f, "Boa Vista"),
            Self::Brava => write!(f, "Brava"),
            Self::Fogo => write!(f, "Fogo"),
            Self::Maio => write!(f, "Maio"),
            Self::Sal => write!(f, "Sal"),
            Self::SantoAntao => write!(f, "Santo Antão"),
            Self::SaoNicolau => write!(f, "São Nicolau"),
            Self::SaoVicente => write!(f, "São Vicente"),
        }
    }
}

// Caribbean Netherlands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaribbeanNetherlandsSubregions {
    CaribbeanNetherlands,
    Bonaire,
    Saba,
    SintEustatius,
}

impl TryFrom<u8> for CaribbeanNetherlandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CaribbeanNetherlands),
            0x02 => Ok(Self::Bonaire),
            0x03 => Ok(Self::Saba),
            0x04 => Ok(Self::SintEustatius),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CaribbeanNetherlandsSubregions> for u8 {
    fn from(value: CaribbeanNetherlandsSubregions) -> u8 {
        match value {
            CaribbeanNetherlandsSubregions::CaribbeanNetherlands => 0x01,
            CaribbeanNetherlandsSubregions::Bonaire => 0x02,
            CaribbeanNetherlandsSubregions::Saba => 0x03,
            CaribbeanNetherlandsSubregions::SintEustatius => 0x04,
        }
    }
}

impl Display for CaribbeanNetherlandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CaribbeanNetherlands => write!(f, "Caribbean Netherlands"),
            Self::Bonaire => write!(f, "Bonaire"),
            Self::Saba => write!(f, "Saba"),
            Self::SintEustatius => write!(f, "Sint Eustatius"),
        }
    }
}

// Cayman Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaymanIslandsSubregions {
    CaymanIslands,
    GrandCayman,
    CaymanBrac,
    LittleCayman,
}

impl TryFrom<u8> for CaymanIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CaymanIslands),
            0x02 => Ok(Self::GrandCayman),
            0x03 => Ok(Self::CaymanBrac),
            0x04 => Ok(Self::LittleCayman),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CaymanIslandsSubregions> for u8 {
    fn from(value: CaymanIslandsSubregions) -> u8 {
        match value {
            CaymanIslandsSubregions::CaymanIslands => 0x01,
            CaymanIslandsSubregions::GrandCayman => 0x02,
            CaymanIslandsSubregions::CaymanBrac => 0x03,
            CaymanIslandsSubregions::LittleCayman => 0x04,
        }
    }
}

impl Display for CaymanIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CaymanIslands => write!(f, "Cayman Islands"),
            Self::GrandCayman => write!(f, "Grand Cayman"),
            Self::CaymanBrac => write!(f, "Cayman Brac"),
            Self::LittleCayman => write!(f, "Little Cayman"),
        }
    }
}

// Central African Republic Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CentralAfricanRepublicSubregions {
    CentralAfricanRepublic,
    Bangui,
    BaminguiBangoran,
    BasseKotto,
    HautMbomou,
    HauteKotto,
    Kemo,
    Lobaye,
    MambereKadei,
    Mbomou,
    NanaGrebizi,
    NanaMambere,
    OmbellaMpoko,
    Ouaka,
    Ouham,
    OuhamPende,
    SanghaMbaere,
    Vakaga,
}

impl TryFrom<u8> for CentralAfricanRepublicSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CentralAfricanRepublic),
            0x02 => Ok(Self::Bangui),
            0x03 => Ok(Self::BaminguiBangoran),
            0x04 => Ok(Self::BasseKotto),
            0x05 => Ok(Self::HautMbomou),
            0x06 => Ok(Self::HauteKotto),
            0x07 => Ok(Self::Kemo),
            0x08 => Ok(Self::Lobaye),
            0x09 => Ok(Self::MambereKadei),
            0x0A => Ok(Self::Mbomou),
            0x0B => Ok(Self::NanaGrebizi),
            0x0C => Ok(Self::NanaMambere),
            0x0D => Ok(Self::OmbellaMpoko),
            0x0E => Ok(Self::Ouaka),
            0x0F => Ok(Self::Ouham),
            0x10 => Ok(Self::OuhamPende),
            0x11 => Ok(Self::SanghaMbaere),
            0x12 => Ok(Self::Vakaga),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CentralAfricanRepublicSubregions> for u8 {
    fn from(value: CentralAfricanRepublicSubregions) -> u8 {
        match value {
            CentralAfricanRepublicSubregions::CentralAfricanRepublic => 0x01,
            CentralAfricanRepublicSubregions::Bangui => 0x02,
            CentralAfricanRepublicSubregions::BaminguiBangoran => 0x03,
            CentralAfricanRepublicSubregions::BasseKotto => 0x04,
            CentralAfricanRepublicSubregions::HautMbomou => 0x05,
            CentralAfricanRepublicSubregions::HauteKotto => 0x06,
            CentralAfricanRepublicSubregions::Kemo => 0x07,
            CentralAfricanRepublicSubregions::Lobaye => 0x08,
            CentralAfricanRepublicSubregions::MambereKadei => 0x09,
            CentralAfricanRepublicSubregions::Mbomou => 0x0A,
            CentralAfricanRepublicSubregions::NanaGrebizi => 0x0B,
            CentralAfricanRepublicSubregions::NanaMambere => 0x0C,
            CentralAfricanRepublicSubregions::OmbellaMpoko => 0x0D,
            CentralAfricanRepublicSubregions::Ouaka => 0x0E,
            CentralAfricanRepublicSubregions::Ouham => 0x0F,
            CentralAfricanRepublicSubregions::OuhamPende => 0x10,
            CentralAfricanRepublicSubregions::SanghaMbaere => 0x11,
            CentralAfricanRepublicSubregions::Vakaga => 0x12,
        }
    }
}

impl Display for CentralAfricanRepublicSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CentralAfricanRepublic => write!(f, "Central African Republic"),
            Self::Bangui => write!(f, "Bangui"),
            Self::BaminguiBangoran => write!(f, "Bamingui-Bangoran"),
            Self::BasseKotto => write!(f, "Basse-Kotto"),
            Self::HautMbomou => write!(f, "Haut-Mbomou"),
            Self::HauteKotto => write!(f, "Haute-Kotto"),
            Self::Kemo => write!(f, "Kémo"),
            Self::Lobaye => write!(f, "Lobaye"),
            Self::MambereKadei => write!(f, "Mambéré-Kadéï"),
            Self::Mbomou => write!(f, "Mbomou"),
            Self::NanaGrebizi => write!(f, "Nana-Grébizi"),
            Self::NanaMambere => write!(f, "Nana-Mambéré"),
            Self::OmbellaMpoko => write!(f, "Ombella-M'Poko"),
            Self::Ouaka => write!(f, "Ouaka"),
            Self::Ouham => write!(f, "Ouham"),
            Self::OuhamPende => write!(f, "Ouham-Pendé"),
            Self::SanghaMbaere => write!(f, "Sangha-Mbaéré"),
            Self::Vakaga => write!(f, "Vakaga"),
        }
    }
}

// Chad Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChadSubregions {
    Chad,
    Ndjamena,
    BahrElGazel,
    Batha,
    Borkou,
    ChariBaguirmi,
    EnnediEst,
    EnnediOuest,
    Guera,
    HadjerLamis,
    Kanem,
    Lac,
    LogoneOccidental,
    LogoneOriental,
    Mandoul,
    MayoKebbiEst,
    MayoKebbiOuest,
    MoyenChari,
    Ouaddai,
    Salamat,
    Sila,
    Tandjile,
    Tibesti,
    WadiFira,
}

impl TryFrom<u8> for ChadSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Chad),
            0x02 => Ok(Self::Ndjamena),
            0x03 => Ok(Self::BahrElGazel),
            0x04 => Ok(Self::Batha),
            0x05 => Ok(Self::Borkou),
            0x06 => Ok(Self::ChariBaguirmi),
            0x07 => Ok(Self::EnnediEst),
            0x08 => Ok(Self::EnnediOuest),
            0x09 => Ok(Self::Guera),
            0x0A => Ok(Self::HadjerLamis),
            0x0B => Ok(Self::Kanem),
            0x0C => Ok(Self::Lac),
            0x0D => Ok(Self::LogoneOccidental),
            0x0E => Ok(Self::LogoneOriental),
            0x0F => Ok(Self::Mandoul),
            0x10 => Ok(Self::MayoKebbiEst),
            0x11 => Ok(Self::MayoKebbiOuest),
            0x12 => Ok(Self::MoyenChari),
            0x13 => Ok(Self::Ouaddai),
            0x14 => Ok(Self::Salamat),
            0x15 => Ok(Self::Sila),
            0x16 => Ok(Self::Tandjile),
            0x17 => Ok(Self::Tibesti),
            0x18 => Ok(Self::WadiFira),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ChadSubregions> for u8 {
    fn from(value: ChadSubregions) -> u8 {
        match value {
            ChadSubregions::Chad => 0x01,
            ChadSubregions::Ndjamena => 0x02,
            ChadSubregions::BahrElGazel => 0x03,
            ChadSubregions::Batha => 0x04,
            ChadSubregions::Borkou => 0x05,
            ChadSubregions::ChariBaguirmi => 0x06,
            ChadSubregions::EnnediEst => 0x07,
            ChadSubregions::EnnediOuest => 0x08,
            ChadSubregions::Guera => 0x09,
            ChadSubregions::HadjerLamis => 0x0A,
            ChadSubregions::Kanem => 0x0B,
            ChadSubregions::Lac => 0x0C,
            ChadSubregions::LogoneOccidental => 0x0D,
            ChadSubregions::LogoneOriental => 0x0E,
            ChadSubregions::Mandoul => 0x0F,
            ChadSubregions::MayoKebbiEst => 0x10,
            ChadSubregions::MayoKebbiOuest => 0x11,
            ChadSubregions::MoyenChari => 0x12,
            ChadSubregions::Ouaddai => 0x13,
            ChadSubregions::Salamat => 0x14,
            ChadSubregions::Sila => 0x15,
            ChadSubregions::Tandjile => 0x16,
            ChadSubregions::Tibesti => 0x17,
            ChadSubregions::WadiFira => 0x18,
        }
    }
}

impl Display for ChadSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chad => write!(f, "Chad"),
            Self::Ndjamena => write!(f, "N'Djamena"),
            Self::BahrElGazel => write!(f, "Bahr el Gazel"),
            Self::Batha => write!(f, "Batha"),
            Self::Borkou => write!(f, "Borkou"),
            Self::ChariBaguirmi => write!(f, "Chari-Baguirmi"),
            Self::EnnediEst => write!(f, "Ennedi-Est"),
            Self::EnnediOuest => write!(f, "Ennedi-Ouest"),
            Self::Guera => write!(f, "Guéra"),
            Self::HadjerLamis => write!(f, "Hadjer-Lamis"),
            Self::Kanem => write!(f, "Kanem"),
            Self::Lac => write!(f, "Lac"),
            Self::LogoneOccidental => write!(f, "Logone Occidental"),
            Self::LogoneOriental => write!(f, "Logone Oriental"),
            Self::Mandoul => write!(f, "Mandoul"),
            Self::MayoKebbiEst => write!(f, "Mayo-Kebbi Est"),
            Self::MayoKebbiOuest => write!(f, "Mayo-Kebbi Ouest"),
            Self::MoyenChari => write!(f, "Moyen-Chari"),
            Self::Ouaddai => write!(f, "Ouaddaï"),
            Self::Salamat => write!(f, "Salamat"),
            Self::Sila => write!(f, "Sila"),
            Self::Tandjile => write!(f, "Tandjilé"),
            Self::Tibesti => write!(f, "Tibesti"),
            Self::WadiFira => write!(f, "Wadi Fira"),
        }
    }
}

// Chile Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChileSubregions {
    Chile,
    RegionMetropolitana,
    Valparaiso,
    AisenDelGeneralCarlosIbanezDelCampo,
    Antofagasta,
    Araucania,
    Atacama,
    BioBio,
    Coquimbo,
    LibertadorGeneralBernardoOhiggins,
    LosLagos,
    MagallanesYAntarticaChilena,
    Maule,
    Tarapaca,
}

impl TryFrom<u8> for ChileSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Chile),
            0x02 => Ok(Self::RegionMetropolitana),
            0x03 => Ok(Self::Valparaiso),
            0x04 => Ok(Self::AisenDelGeneralCarlosIbanezDelCampo),
            0x05 => Ok(Self::Antofagasta),
            0x06 => Ok(Self::Araucania),
            0x07 => Ok(Self::Atacama),
            0x08 => Ok(Self::BioBio),
            0x09 => Ok(Self::Coquimbo),
            0x0A => Ok(Self::LibertadorGeneralBernardoOhiggins),
            0x0B => Ok(Self::LosLagos),
            0x0C => Ok(Self::MagallanesYAntarticaChilena),
            0x0D => Ok(Self::Maule),
            0x0E => Ok(Self::Tarapaca),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ChileSubregions> for u8 {
    fn from(value: ChileSubregions) -> u8 {
        match value {
            ChileSubregions::Chile => 0x01,
            ChileSubregions::RegionMetropolitana => 0x02,
            ChileSubregions::Valparaiso => 0x03,
            ChileSubregions::AisenDelGeneralCarlosIbanezDelCampo => 0x04,
            ChileSubregions::Antofagasta => 0x05,
            ChileSubregions::Araucania => 0x06,
            ChileSubregions::Atacama => 0x07,
            ChileSubregions::BioBio => 0x08,
            ChileSubregions::Coquimbo => 0x09,
            ChileSubregions::LibertadorGeneralBernardoOhiggins => 0x0A,
            ChileSubregions::LosLagos => 0x0B,
            ChileSubregions::MagallanesYAntarticaChilena => 0x0C,
            ChileSubregions::Maule => 0x0D,
            ChileSubregions::Tarapaca => 0x0E,
        }
    }
}

impl Display for ChileSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chile => write!(f, "Chile"),
            Self::RegionMetropolitana => write!(f, "Región Metropolitana"),
            Self::Valparaiso => write!(f, "Valparaíso"),
            Self::AisenDelGeneralCarlosIbanezDelCampo => {
                write!(f, "Aisén del General Carlos Ibáñez del Campo")
            }
            Self::Antofagasta => write!(f, "Antofagasta"),
            Self::Araucania => write!(f, "Araucanía"),
            Self::Atacama => write!(f, "Atacama"),
            Self::BioBio => write!(f, "Bío-Bío"),
            Self::Coquimbo => write!(f, "Coquimbo"),
            Self::LibertadorGeneralBernardoOhiggins => {
                write!(f, "Libertador General Bernardo O'Higgins")
            }
            Self::LosLagos => write!(f, "Los Lagos"),
            Self::MagallanesYAntarticaChilena => write!(f, "Magallanes y Antártica Chilena"),
            Self::Maule => write!(f, "Maule"),
            Self::Tarapaca => write!(f, "Tarapacá"),
        }
    }
}

// China Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChinaSubregions {
    China,
    Beijing,
    Chongqing,
    Shanghai,
    Tianjin,
    Anhui,
    Fujian,
    Gansu,
    Guangdong,
    Guizhou,
    Hainan,
    Hebei,
    Heilongjiang,
    Henan,
    Hubei,
    Hunan,
    Jiangsu,
    Jiangxi,
    Jilin,
    Liaoning,
    Qinghai,
    Shanxi,
    Shandong,
    Sichuan,
    Yunnan,
    Zhejiang,
    Taiwan,
    GuangxiZhuangzu,
    NeiMenggu,
    NingxiaHuizu,
    XinjiangWeiwuerZu,
    Xizang,
}

impl TryFrom<u8> for ChinaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::China),
            0x02 => Ok(Self::Beijing),
            0x03 => Ok(Self::Chongqing),
            0x04 => Ok(Self::Shanghai),
            0x05 => Ok(Self::Tianjin),
            0x06 => Ok(Self::Anhui),
            0x07 => Ok(Self::Fujian),
            0x08 => Ok(Self::Gansu),
            0x09 => Ok(Self::Guangdong),
            0x0A => Ok(Self::Guizhou),
            0x0B => Ok(Self::Hainan),
            0x0C => Ok(Self::Hebei),
            0x0D => Ok(Self::Heilongjiang),
            0x0E => Ok(Self::Henan),
            0x0F => Ok(Self::Hubei),
            0x10 => Ok(Self::Hunan),
            0x11 => Ok(Self::Jiangsu),
            0x12 => Ok(Self::Jiangxi),
            0x13 => Ok(Self::Jilin),
            0x14 => Ok(Self::Liaoning),
            0x15 => Ok(Self::Qinghai),
            0x16 => Ok(Self::Shanxi),
            0x17 => Ok(Self::Shandong),
            0x18 => Ok(Self::Shanxi),
            0x19 => Ok(Self::Sichuan),
            0x1A => Ok(Self::Yunnan),
            0x1B => Ok(Self::Zhejiang),
            0x1C => Ok(Self::Taiwan),
            0x1D => Ok(Self::GuangxiZhuangzu),
            0x1E => Ok(Self::NeiMenggu),
            0x1F => Ok(Self::NingxiaHuizu),
            0x20 => Ok(Self::XinjiangWeiwuerZu),
            0x21 => Ok(Self::Xizang),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ChinaSubregions> for u8 {
    fn from(value: ChinaSubregions) -> u8 {
        match value {
            ChinaSubregions::China => 0x01,
            ChinaSubregions::Beijing => 0x02,
            ChinaSubregions::Chongqing => 0x03,
            ChinaSubregions::Shanghai => 0x04,
            ChinaSubregions::Tianjin => 0x05,
            ChinaSubregions::Anhui => 0x06,
            ChinaSubregions::Fujian => 0x07,
            ChinaSubregions::Gansu => 0x08,
            ChinaSubregions::Guangdong => 0x09,
            ChinaSubregions::Guizhou => 0x0A,
            ChinaSubregions::Hainan => 0x0B,
            ChinaSubregions::Hebei => 0x0C,
            ChinaSubregions::Heilongjiang => 0x0D,
            ChinaSubregions::Henan => 0x0E,
            ChinaSubregions::Hubei => 0x0F,
            ChinaSubregions::Hunan => 0x10,
            ChinaSubregions::Jiangsu => 0x11,
            ChinaSubregions::Jiangxi => 0x12,
            ChinaSubregions::Jilin => 0x13,
            ChinaSubregions::Liaoning => 0x14,
            ChinaSubregions::Qinghai => 0x15,
            ChinaSubregions::Shanxi => 0x16,
            ChinaSubregions::Shandong => 0x17,
            ChinaSubregions::Sichuan => 0x19,
            ChinaSubregions::Yunnan => 0x1A,
            ChinaSubregions::Zhejiang => 0x1B,
            ChinaSubregions::Taiwan => 0x1C,
            ChinaSubregions::GuangxiZhuangzu => 0x1D,
            ChinaSubregions::NeiMenggu => 0x1E,
            ChinaSubregions::NingxiaHuizu => 0x1F,
            ChinaSubregions::XinjiangWeiwuerZu => 0x20,
            ChinaSubregions::Xizang => 0x21,
        }
    }
}

impl Display for ChinaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::China => write!(f, "China"),
            Self::Beijing => write!(f, "Beijing"),
            Self::Chongqing => write!(f, "Chongqing"),
            Self::Shanghai => write!(f, "Shanghai"),
            Self::Tianjin => write!(f, "Tianjin"),
            Self::Anhui => write!(f, "Anhui"),
            Self::Fujian => write!(f, "Fujian"),
            Self::Gansu => write!(f, "Gansu"),
            Self::Guangdong => write!(f, "Guangdong"),
            Self::Guizhou => write!(f, "Guizhou"),
            Self::Hainan => write!(f, "Hainan"),
            Self::Hebei => write!(f, "Hebei"),
            Self::Heilongjiang => write!(f, "Heilongjiang"),
            Self::Henan => write!(f, "Henan"),
            Self::Hubei => write!(f, "Hubei"),
            Self::Hunan => write!(f, "Húnán"),
            Self::Jiangsu => write!(f, "Jiangsu"),
            Self::Jiangxi => write!(f, "Jiangxi"),
            Self::Jilin => write!(f, "Jilin"),
            Self::Liaoning => write!(f, "Liaoning"),
            Self::Qinghai => write!(f, "Qinghai"),
            Self::Shanxi => write!(f, "Shanxi"),
            Self::Shandong => write!(f, "Shandong"),
            Self::Sichuan => write!(f, "Sichuan"),
            Self::Yunnan => write!(f, "Yunnan"),
            Self::Zhejiang => write!(f, "Zhejiang"),
            Self::Taiwan => write!(f, "Taiwan"),
            Self::GuangxiZhuangzu => write!(f, "Guangxi-Zhuangzu"),
            Self::NeiMenggu => write!(f, "Nei-Menggu"),
            Self::NingxiaHuizu => write!(f, "Ningxia-huizu"),
            Self::XinjiangWeiwuerZu => write!(f, "Xinjiang-Weiwu'er-zu"),
            Self::Xizang => write!(f, "Xizang"),
        }
    }
}

// Christmas Island Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChristmasIslandSubregions {
    ChristmasIsland,
}

impl TryFrom<u8> for ChristmasIslandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::ChristmasIsland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ChristmasIslandSubregions> for u8 {
    fn from(value: ChristmasIslandSubregions) -> u8 {
        match value {
            ChristmasIslandSubregions::ChristmasIsland => 0x01,
        }
    }
}

impl Display for ChristmasIslandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChristmasIsland => write!(f, "Christmas Island"),
        }
    }
}

// Cocos (Keeling) Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CocosKeelingIslandsSubregions {
    CocosKeelingIslands,
    WestIsland,
    HomeIsland,
}

impl TryFrom<u8> for CocosKeelingIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CocosKeelingIslands),
            0x02 => Ok(Self::WestIsland),
            0x03 => Ok(Self::HomeIsland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CocosKeelingIslandsSubregions> for u8 {
    fn from(value: CocosKeelingIslandsSubregions) -> u8 {
        match value {
            CocosKeelingIslandsSubregions::CocosKeelingIslands => 0x01,
            CocosKeelingIslandsSubregions::WestIsland => 0x02,
            CocosKeelingIslandsSubregions::HomeIsland => 0x03,
        }
    }
}

impl Display for CocosKeelingIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CocosKeelingIslands => write!(f, "Cocos (Keeling) Islands"),
            Self::WestIsland => write!(f, "West Island"),
            Self::HomeIsland => write!(f, "Home Island"),
        }
    }
}

// Colombia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColombiaSubregions {
    Colombia,
    DistritoCapital,
    Cundinamarca,
    Amazonas,
    Antioquia,
    Arauca,
    Atlantico,
    Bolivar,
    Boyaca,
    Caldas,
    Caqueta,
    Cauca,
    Cesar,
    Choco,
    Cordoba,
    Guaviare,
    Guainia,
    Huila,
    LaGuajira,
    Magdalena,
    Meta,
    Narino,
    NorteDeSantander,
    Putumayo,
    Quindio,
    Risaralda,
    ArchipielagoDeSanAndresProvidenciaYSantaCatalina,
    Santander,
    Sucre,
    Tolima,
    ValleDelCauca,
    Vaupes,
    Vichada,
    Casanare,
}

impl TryFrom<u8> for ColombiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Colombia),
            0x02 => Ok(Self::DistritoCapital),
            0x03 => Ok(Self::Cundinamarca),
            0x04 => Ok(Self::Amazonas),
            0x05 => Ok(Self::Antioquia),
            0x06 => Ok(Self::Arauca),
            0x07 => Ok(Self::Atlantico),
            0x08 => Ok(Self::Bolivar),
            0x09 => Ok(Self::Boyaca),
            0x0A => Ok(Self::Caldas),
            0x0B => Ok(Self::Caqueta),
            0x0C => Ok(Self::Cauca),
            0x0D => Ok(Self::Cesar),
            0x0E => Ok(Self::Choco),
            0x0F => Ok(Self::Cordoba),
            0x10 => Ok(Self::Guaviare),
            0x11 => Ok(Self::Guainia),
            0x12 => Ok(Self::Huila),
            0x13 => Ok(Self::LaGuajira),
            0x14 => Ok(Self::Magdalena),
            0x15 => Ok(Self::Meta),
            0x16 => Ok(Self::Narino),
            0x17 => Ok(Self::NorteDeSantander),
            0x18 => Ok(Self::Putumayo),
            0x19 => Ok(Self::Quindio),
            0x1A => Ok(Self::Risaralda),
            0x1B => Ok(Self::ArchipielagoDeSanAndresProvidenciaYSantaCatalina),
            0x1C => Ok(Self::Santander),
            0x1D => Ok(Self::Sucre),
            0x1E => Ok(Self::Tolima),
            0x1F => Ok(Self::ValleDelCauca),
            0x20 => Ok(Self::Vaupes),
            0x21 => Ok(Self::Vichada),
            0x22 => Ok(Self::Casanare),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ColombiaSubregions> for u8 {
    fn from(value: ColombiaSubregions) -> u8 {
        match value {
            ColombiaSubregions::Colombia => 0x01,
            ColombiaSubregions::DistritoCapital => 0x02,
            ColombiaSubregions::Cundinamarca => 0x03,
            ColombiaSubregions::Amazonas => 0x04,
            ColombiaSubregions::Antioquia => 0x05,
            ColombiaSubregions::Arauca => 0x06,
            ColombiaSubregions::Atlantico => 0x07,
            ColombiaSubregions::Bolivar => 0x08,
            ColombiaSubregions::Boyaca => 0x09,
            ColombiaSubregions::Caldas => 0x0A,
            ColombiaSubregions::Caqueta => 0x0B,
            ColombiaSubregions::Cauca => 0x0C,
            ColombiaSubregions::Cesar => 0x0D,
            ColombiaSubregions::Choco => 0x0E,
            ColombiaSubregions::Cordoba => 0x0F,
            ColombiaSubregions::Guaviare => 0x10,
            ColombiaSubregions::Guainia => 0x11,
            ColombiaSubregions::Huila => 0x12,
            ColombiaSubregions::LaGuajira => 0x13,
            ColombiaSubregions::Magdalena => 0x14,
            ColombiaSubregions::Meta => 0x15,
            ColombiaSubregions::Narino => 0x16,
            ColombiaSubregions::NorteDeSantander => 0x17,
            ColombiaSubregions::Putumayo => 0x18,
            ColombiaSubregions::Quindio => 0x19,
            ColombiaSubregions::Risaralda => 0x1A,
            ColombiaSubregions::ArchipielagoDeSanAndresProvidenciaYSantaCatalina => 0x1B,
            ColombiaSubregions::Santander => 0x1C,
            ColombiaSubregions::Sucre => 0x1D,
            ColombiaSubregions::Tolima => 0x1E,
            ColombiaSubregions::ValleDelCauca => 0x1F,
            ColombiaSubregions::Vaupes => 0x20,
            ColombiaSubregions::Vichada => 0x21,
            ColombiaSubregions::Casanare => 0x22,
        }
    }
}

impl Display for ColombiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Colombia => write!(f, "Colombia"),
            Self::DistritoCapital => write!(f, "Distrito Capital"),
            Self::Cundinamarca => write!(f, "Cundinamarca"),
            Self::Amazonas => write!(f, "Amazonas"),
            Self::Antioquia => write!(f, "Antioquia"),
            Self::Arauca => write!(f, "Arauca"),
            Self::Atlantico => write!(f, "Atlántico"),
            Self::Bolivar => write!(f, "Bolívar"),
            Self::Boyaca => write!(f, "Boyacá"),
            Self::Caldas => write!(f, "Caldas"),
            Self::Caqueta => write!(f, "Caquetá"),
            Self::Cauca => write!(f, "Cauca"),
            Self::Cesar => write!(f, "Cesar"),
            Self::Choco => write!(f, "Chocó"),
            Self::Cordoba => write!(f, "Córdoba"),
            Self::Guaviare => write!(f, "Guaviare"),
            Self::Guainia => write!(f, "Guainía"),
            Self::Huila => write!(f, "Huila"),
            Self::LaGuajira => write!(f, "La Guajira"),
            Self::Magdalena => write!(f, "Magdalena"),
            Self::Meta => write!(f, "Meta"),
            Self::Narino => write!(f, "Nariño"),
            Self::NorteDeSantander => write!(f, "Norte de Santander"),
            Self::Putumayo => write!(f, "Putumayo"),
            Self::Quindio => write!(f, "Quindío"),
            Self::Risaralda => write!(f, "Risaralda"),
            Self::ArchipielagoDeSanAndresProvidenciaYSantaCatalina => write!(
                f,
                "Archipiélago de San Andrés, Providencia y Santa Catalina"
            ),
            Self::Santander => write!(f, "Santander"),
            Self::Sucre => write!(f, "Sucre"),
            Self::Tolima => write!(f, "Tolima"),
            Self::ValleDelCauca => write!(f, "Valle del Cauca"),
            Self::Vaupes => write!(f, "Vaupés"),
            Self::Vichada => write!(f, "Vichada"),
            Self::Casanare => write!(f, "Casanare"),
        }
    }
}

// Comoros Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComorosSubregions {
    Comoros,
    GrandeComore,
    Anjouan,
    Moheli,
}

impl TryFrom<u8> for ComorosSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Comoros),
            0x02 => Ok(Self::GrandeComore),
            0x03 => Ok(Self::Anjouan),
            0x04 => Ok(Self::Moheli),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ComorosSubregions> for u8 {
    fn from(value: ComorosSubregions) -> u8 {
        match value {
            ComorosSubregions::Comoros => 0x01,
            ComorosSubregions::GrandeComore => 0x02,
            ComorosSubregions::Anjouan => 0x03,
            ComorosSubregions::Moheli => 0x04,
        }
    }
}

impl Display for ComorosSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Comoros => write!(f, "Comoros"),
            Self::GrandeComore => write!(f, "Grande Comore"),
            Self::Anjouan => write!(f, "Anjouan"),
            Self::Moheli => write!(f, "Mohéli"),
        }
    }
}

// Cook Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CookIslandsSubregions {
    CookIslands,
    Southern,
    Northern,
}

impl TryFrom<u8> for CookIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CookIslands),
            0x02 => Ok(Self::Southern),
            0x03 => Ok(Self::Northern),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CookIslandsSubregions> for u8 {
    fn from(value: CookIslandsSubregions) -> u8 {
        match value {
            CookIslandsSubregions::CookIslands => 0x01,
            CookIslandsSubregions::Southern => 0x02,
            CookIslandsSubregions::Northern => 0x03,
        }
    }
}

impl Display for CookIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CookIslands => write!(f, "Cook Islands"),
            Self::Southern => write!(f, "Southern"),
            Self::Northern => write!(f, "Northern"),
        }
    }
}

// Costa Rica Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CostaRicaSubregions {
    CostaRica,
    SanJose,
    Alajuela,
    Cartago,
    Guanacaste,
    Heredia,
    Limon,
    Puntarenas,
}

impl TryFrom<u8> for CostaRicaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CostaRica),
            0x02 => Ok(Self::SanJose),
            0x03 => Ok(Self::Alajuela),
            0x04 => Ok(Self::Cartago),
            0x05 => Ok(Self::Guanacaste),
            0x06 => Ok(Self::Heredia),
            0x07 => Ok(Self::Limon),
            0x08 => Ok(Self::Puntarenas),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CostaRicaSubregions> for u8 {
    fn from(value: CostaRicaSubregions) -> u8 {
        match value {
            CostaRicaSubregions::CostaRica => 0x01,
            CostaRicaSubregions::SanJose => 0x02,
            CostaRicaSubregions::Alajuela => 0x03,
            CostaRicaSubregions::Cartago => 0x04,
            CostaRicaSubregions::Guanacaste => 0x05,
            CostaRicaSubregions::Heredia => 0x06,
            CostaRicaSubregions::Limon => 0x07,
            CostaRicaSubregions::Puntarenas => 0x08,
        }
    }
}

impl Display for CostaRicaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CostaRica => write!(f, "Costa Rica"),
            Self::SanJose => write!(f, "San José"),
            Self::Alajuela => write!(f, "Alajuela"),
            Self::Cartago => write!(f, "Cartago"),
            Self::Guanacaste => write!(f, "Guanacaste"),
            Self::Heredia => write!(f, "Heredia"),
            Self::Limon => write!(f, "Limón"),
            Self::Puntarenas => write!(f, "Puntarenas"),
        }
    }
}

// Croatia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CroatiaSubregions {
    Croatia,
    ZagrebRegion,
    CentralCroatia,
    AdriaticCroatia,
    EastCroatia,
}

impl TryFrom<u8> for CroatiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Croatia),
            0x02 => Ok(Self::ZagrebRegion),
            0x03 => Ok(Self::CentralCroatia),
            0x04 => Ok(Self::AdriaticCroatia),
            0x05 => Ok(Self::EastCroatia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CroatiaSubregions> for u8 {
    fn from(value: CroatiaSubregions) -> u8 {
        match value {
            CroatiaSubregions::Croatia => 0x01,
            CroatiaSubregions::ZagrebRegion => 0x02,
            CroatiaSubregions::CentralCroatia => 0x03,
            CroatiaSubregions::AdriaticCroatia => 0x04,
            CroatiaSubregions::EastCroatia => 0x05,
        }
    }
}

impl Display for CroatiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Croatia => write!(f, "Croatia"),
            Self::ZagrebRegion => write!(f, "Zagreb Region"),
            Self::CentralCroatia => write!(f, "Central Croatia"),
            Self::AdriaticCroatia => write!(f, "Adriatic Croatia"),
            Self::EastCroatia => write!(f, "East Croatia"),
        }
    }
}

// Cuba Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CubaSubregions {
    Cuba,
    LaHabana,
    Artemisa,
    Granma,
    Camaguey,
    CiegoDeAvila,
    Cienfuegos,
    Guantanamo,
    Holguin,
    LasTunas,
    Matanzas,
    IslaDeLaJuventud,
    PinarDelRio,
    Mayabeque,
    SanctiSpiritus,
    VillaClara,
    SantiagoDeCuba,
}

impl TryFrom<u8> for CubaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Cuba),
            0x02 => Ok(Self::LaHabana),
            0x03 => Ok(Self::Artemisa),
            0x04 => Ok(Self::Granma),
            0x05 => Ok(Self::Camaguey),
            0x06 => Ok(Self::CiegoDeAvila),
            0x07 => Ok(Self::Cienfuegos),
            0x08 => Ok(Self::Guantanamo),
            0x09 => Ok(Self::Holguin),
            0x0A => Ok(Self::LasTunas),
            0x0B => Ok(Self::Matanzas),
            0x0C => Ok(Self::IslaDeLaJuventud),
            0x0D => Ok(Self::PinarDelRio),
            0x0E => Ok(Self::Mayabeque),
            0x0F => Ok(Self::SanctiSpiritus),
            0x10 => Ok(Self::VillaClara),
            0x11 => Ok(Self::SantiagoDeCuba),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CubaSubregions> for u8 {
    fn from(value: CubaSubregions) -> u8 {
        match value {
            CubaSubregions::Cuba => 0x01,
            CubaSubregions::LaHabana => 0x02,
            CubaSubregions::Artemisa => 0x03,
            CubaSubregions::Granma => 0x04,
            CubaSubregions::Camaguey => 0x05,
            CubaSubregions::CiegoDeAvila => 0x06,
            CubaSubregions::Cienfuegos => 0x07,
            CubaSubregions::Guantanamo => 0x08,
            CubaSubregions::Holguin => 0x09,
            CubaSubregions::LasTunas => 0x0A,
            CubaSubregions::Matanzas => 0x0B,
            CubaSubregions::IslaDeLaJuventud => 0x0C,
            CubaSubregions::PinarDelRio => 0x0D,
            CubaSubregions::Mayabeque => 0x0E,
            CubaSubregions::SanctiSpiritus => 0x0F,
            CubaSubregions::VillaClara => 0x10,
            CubaSubregions::SantiagoDeCuba => 0x11,
        }
    }
}

impl Display for CubaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cuba => write!(f, "Cuba"),
            Self::LaHabana => write!(f, "La Habana"),
            Self::Artemisa => write!(f, "Artemisa"),
            Self::Granma => write!(f, "Granma"),
            Self::Camaguey => write!(f, "Camagüey"),
            Self::CiegoDeAvila => write!(f, "Ciego de Ávila"),
            Self::Cienfuegos => write!(f, "Cienfuegos"),
            Self::Guantanamo => write!(f, "Guantánamo"),
            Self::Holguin => write!(f, "Holguín"),
            Self::LasTunas => write!(f, "Las Tunas"),
            Self::Matanzas => write!(f, "Matanzas"),
            Self::IslaDeLaJuventud => write!(f, "Isla de la Juventud"),
            Self::PinarDelRio => write!(f, "Pinar del Río"),
            Self::Mayabeque => write!(f, "Mayabeque"),
            Self::SanctiSpiritus => write!(f, "Sancti Spíritus"),
            Self::VillaClara => write!(f, "Villa Clara"),
            Self::SantiagoDeCuba => write!(f, "Santiago de Cuba"),
        }
    }
}

// Curaçao Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CuracaoSubregions {
    Curacao,
}

impl TryFrom<u8> for CuracaoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Curacao),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CuracaoSubregions> for u8 {
    fn from(value: CuracaoSubregions) -> u8 {
        match value {
            CuracaoSubregions::Curacao => 0x01,
        }
    }
}

impl Display for CuracaoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Curacao => write!(f, "Curaçao"),
        }
    }
}

// Cyprus Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CyprusSubregions {
    Cyprus,
    Nicosia,
    Famagusta,
    Kyrenia,
    Larnaca,
    Limassol,
    Paphos,
}

impl TryFrom<u8> for CyprusSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Cyprus),
            0x02 => Ok(Self::Nicosia),
            0x03 => Ok(Self::Famagusta),
            0x04 => Ok(Self::Kyrenia),
            0x05 => Ok(Self::Larnaca),
            0x06 => Ok(Self::Limassol),
            0x07 => Ok(Self::Paphos),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CyprusSubregions> for u8 {
    fn from(value: CyprusSubregions) -> u8 {
        match value {
            CyprusSubregions::Cyprus => 0x01,
            CyprusSubregions::Nicosia => 0x02,
            CyprusSubregions::Famagusta => 0x03,
            CyprusSubregions::Kyrenia => 0x04,
            CyprusSubregions::Larnaca => 0x05,
            CyprusSubregions::Limassol => 0x06,
            CyprusSubregions::Paphos => 0x07,
        }
    }
}

impl Display for CyprusSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cyprus => write!(f, "Cyprus"),
            Self::Nicosia => write!(f, "Nicosia"),
            Self::Famagusta => write!(f, "Famagusta"),
            Self::Kyrenia => write!(f, "Kyrenia"),
            Self::Larnaca => write!(f, "Larnaca"),
            Self::Limassol => write!(f, "Limassol"),
            Self::Paphos => write!(f, "Paphos"),
        }
    }
}

// Czechia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CzechiaSubregions {
    Czechia,
    Prague,
    CentralBohemianRegion,
    SouthBohemianRegion,
    PlzenRegion,
    KarlovyVaryRegion,
    UStiNadLabemRegion,
    LiberecRegion,
    HradecKraloveRegion,
    PardubiceRegion,
    OlomoucRegion,
    MoravianSilesianRegion,
    SouthMoravianRegion,
    ZlinRegion,
    VysocinaRegion,
}

impl TryFrom<u8> for CzechiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Czechia),
            0x02 => Ok(Self::Prague),
            0x03 => Ok(Self::CentralBohemianRegion),
            0x04 => Ok(Self::SouthBohemianRegion),
            0x05 => Ok(Self::PlzenRegion),
            0x06 => Ok(Self::KarlovyVaryRegion),
            0x07 => Ok(Self::UStiNadLabemRegion),
            0x08 => Ok(Self::LiberecRegion),
            0x09 => Ok(Self::HradecKraloveRegion),
            0x0A => Ok(Self::PardubiceRegion),
            0x0B => Ok(Self::OlomoucRegion),
            0x0C => Ok(Self::MoravianSilesianRegion),
            0x0D => Ok(Self::SouthMoravianRegion),
            0x0E => Ok(Self::ZlinRegion),
            0x0F => Ok(Self::VysocinaRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CzechiaSubregions> for u8 {
    fn from(value: CzechiaSubregions) -> u8 {
        match value {
            CzechiaSubregions::Czechia => 0x01,
            CzechiaSubregions::Prague => 0x02,
            CzechiaSubregions::CentralBohemianRegion => 0x03,
            CzechiaSubregions::SouthBohemianRegion => 0x04,
            CzechiaSubregions::PlzenRegion => 0x05,
            CzechiaSubregions::KarlovyVaryRegion => 0x06,
            CzechiaSubregions::UStiNadLabemRegion => 0x07,
            CzechiaSubregions::LiberecRegion => 0x08,
            CzechiaSubregions::HradecKraloveRegion => 0x09,
            CzechiaSubregions::PardubiceRegion => 0x0A,
            CzechiaSubregions::OlomoucRegion => 0x0B,
            CzechiaSubregions::MoravianSilesianRegion => 0x0C,
            CzechiaSubregions::SouthMoravianRegion => 0x0D,
            CzechiaSubregions::ZlinRegion => 0x0E,
            CzechiaSubregions::VysocinaRegion => 0x0F,
        }
    }
}

impl Display for CzechiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Czechia => write!(f, "Czechia"),
            Self::Prague => write!(f, "Prague"),
            Self::CentralBohemianRegion => write!(f, "Central Bohemian Region"),
            Self::SouthBohemianRegion => write!(f, "South Bohemian Region"),
            Self::PlzenRegion => write!(f, "Plzen Region"),
            Self::KarlovyVaryRegion => write!(f, "Karlovy Vary Region"),
            Self::UStiNadLabemRegion => write!(f, "Ústi nad Labem Region"),
            Self::LiberecRegion => write!(f, "Liberec Region"),
            Self::HradecKraloveRegion => write!(f, "Hradec Králové Region"),
            Self::PardubiceRegion => write!(f, "Pardubice Region"),
            Self::OlomoucRegion => write!(f, "Olomouc Region"),
            Self::MoravianSilesianRegion => write!(f, "Moravian-Silesian Region"),
            Self::SouthMoravianRegion => write!(f, "South Moravian Region"),
            Self::ZlinRegion => write!(f, "Zlín Region"),
            Self::VysocinaRegion => write!(f, "Vysocina Region"),
        }
    }
}

// Côte d'Ivoire Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoteDIvoireSubregions {
    CoteDIvoire,
    Yamoussoukro,
    Abidjan,
    BasSassandra,
    Comoe,
    Denguele,
    GohDjiboua,
    Lacs,
    Lagunes,
    Montagnes,
    SassandraMarahoue,
    Savanes,
    ValleeDuBandama,
    Woroba,
    Zanzan,
}

impl TryFrom<u8> for CoteDIvoireSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::CoteDIvoire),
            0x02 => Ok(Self::Yamoussoukro),
            0x03 => Ok(Self::Abidjan),
            0x04 => Ok(Self::BasSassandra),
            0x05 => Ok(Self::Comoe),
            0x06 => Ok(Self::Denguele),
            0x07 => Ok(Self::GohDjiboua),
            0x08 => Ok(Self::Lacs),
            0x09 => Ok(Self::Lagunes),
            0x0A => Ok(Self::Montagnes),
            0x0B => Ok(Self::SassandraMarahoue),
            0x0C => Ok(Self::Savanes),
            0x0D => Ok(Self::ValleeDuBandama),
            0x0E => Ok(Self::Woroba),
            0x0F => Ok(Self::Zanzan),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<CoteDIvoireSubregions> for u8 {
    fn from(value: CoteDIvoireSubregions) -> u8 {
        match value {
            CoteDIvoireSubregions::CoteDIvoire => 0x01,
            CoteDIvoireSubregions::Yamoussoukro => 0x02,
            CoteDIvoireSubregions::Abidjan => 0x03,
            CoteDIvoireSubregions::BasSassandra => 0x04,
            CoteDIvoireSubregions::Comoe => 0x05,
            CoteDIvoireSubregions::Denguele => 0x06,
            CoteDIvoireSubregions::GohDjiboua => 0x07,
            CoteDIvoireSubregions::Lacs => 0x08,
            CoteDIvoireSubregions::Lagunes => 0x09,
            CoteDIvoireSubregions::Montagnes => 0x0A,
            CoteDIvoireSubregions::SassandraMarahoue => 0x0B,
            CoteDIvoireSubregions::Savanes => 0x0C,
            CoteDIvoireSubregions::ValleeDuBandama => 0x0D,
            CoteDIvoireSubregions::Woroba => 0x0E,
            CoteDIvoireSubregions::Zanzan => 0x0F,
        }
    }
}

impl Display for CoteDIvoireSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CoteDIvoire => write!(f, "Côte d'Ivoire"),
            Self::Yamoussoukro => write!(f, "Yamoussoukro"),
            Self::Abidjan => write!(f, "Abidjan"),
            Self::BasSassandra => write!(f, "Bas-Sassandra"),
            Self::Comoe => write!(f, "Comoé"),
            Self::Denguele => write!(f, "Denguélé"),
            Self::GohDjiboua => write!(f, "Gôh-Djiboua"),
            Self::Lacs => write!(f, "Lacs"),
            Self::Lagunes => write!(f, "Lagunes"),
            Self::Montagnes => write!(f, "Montagnes"),
            Self::SassandraMarahoue => write!(f, "Sassandra-Marahoué"),
            Self::Savanes => write!(f, "Savanes"),
            Self::ValleeDuBandama => write!(f, "Vallée du Bandama"),
            Self::Woroba => write!(f, "Woroba"),
            Self::Zanzan => write!(f, "Zanzan"),
        }
    }
}

// Democratic Republic of the Congo Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DemocraticRepublicOfTheCongoSubregions {
    DemocraticRepublicOfTheCongo,
    Kinshasa,
    BasUele,
    Equateur,
    HautKatangaProvince,
    HautLomami,
    HautUele,
    IturiProvince,
    KasaiCentral,
    KasaiOriental,
    KasaiProvince,
    KongoCentral,
    Kwango,
    KwiluProvince,
    LomamiProvince,
    LualabaProvince,
    MaiNdombeProvince,
    Maniema,
    Mongala,
    NordUbangi,
    NorthKivu,
    Sankuru,
    SouthKivu,
    SudUbangi,
    TanganyikaProvince,
    Tshopo,
    Tshuapa,
}

impl TryFrom<u8> for DemocraticRepublicOfTheCongoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::DemocraticRepublicOfTheCongo),
            0x02 => Ok(Self::Kinshasa),
            0x03 => Ok(Self::BasUele),
            0x04 => Ok(Self::Equateur),
            0x05 => Ok(Self::HautKatangaProvince),
            0x06 => Ok(Self::HautLomami),
            0x07 => Ok(Self::HautUele),
            0x08 => Ok(Self::IturiProvince),
            0x09 => Ok(Self::KasaiCentral),
            0x0A => Ok(Self::KasaiOriental),
            0x0B => Ok(Self::KasaiProvince),
            0x0C => Ok(Self::KongoCentral),
            0x0D => Ok(Self::Kwango),
            0x0E => Ok(Self::KwiluProvince),
            0x0F => Ok(Self::LomamiProvince),
            0x10 => Ok(Self::LualabaProvince),
            0x11 => Ok(Self::MaiNdombeProvince),
            0x12 => Ok(Self::Maniema),
            0x13 => Ok(Self::Mongala),
            0x14 => Ok(Self::NordUbangi),
            0x15 => Ok(Self::NorthKivu),
            0x16 => Ok(Self::Sankuru),
            0x17 => Ok(Self::SouthKivu),
            0x18 => Ok(Self::SudUbangi),
            0x19 => Ok(Self::TanganyikaProvince),
            0x1A => Ok(Self::Tshopo),
            0x1B => Ok(Self::Tshuapa),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<DemocraticRepublicOfTheCongoSubregions> for u8 {
    fn from(value: DemocraticRepublicOfTheCongoSubregions) -> u8 {
        match value {
            DemocraticRepublicOfTheCongoSubregions::DemocraticRepublicOfTheCongo => 0x01,
            DemocraticRepublicOfTheCongoSubregions::Kinshasa => 0x02,
            DemocraticRepublicOfTheCongoSubregions::BasUele => 0x03,
            DemocraticRepublicOfTheCongoSubregions::Equateur => 0x04,
            DemocraticRepublicOfTheCongoSubregions::HautKatangaProvince => 0x05,
            DemocraticRepublicOfTheCongoSubregions::HautLomami => 0x06,
            DemocraticRepublicOfTheCongoSubregions::HautUele => 0x07,
            DemocraticRepublicOfTheCongoSubregions::IturiProvince => 0x08,
            DemocraticRepublicOfTheCongoSubregions::KasaiCentral => 0x09,
            DemocraticRepublicOfTheCongoSubregions::KasaiOriental => 0x0A,
            DemocraticRepublicOfTheCongoSubregions::KasaiProvince => 0x0B,
            DemocraticRepublicOfTheCongoSubregions::KongoCentral => 0x0C,
            DemocraticRepublicOfTheCongoSubregions::Kwango => 0x0D,
            DemocraticRepublicOfTheCongoSubregions::KwiluProvince => 0x0E,
            DemocraticRepublicOfTheCongoSubregions::LomamiProvince => 0x0F,
            DemocraticRepublicOfTheCongoSubregions::LualabaProvince => 0x10,
            DemocraticRepublicOfTheCongoSubregions::MaiNdombeProvince => 0x11,
            DemocraticRepublicOfTheCongoSubregions::Maniema => 0x12,
            DemocraticRepublicOfTheCongoSubregions::Mongala => 0x13,
            DemocraticRepublicOfTheCongoSubregions::NordUbangi => 0x14,
            DemocraticRepublicOfTheCongoSubregions::NorthKivu => 0x15,
            DemocraticRepublicOfTheCongoSubregions::Sankuru => 0x16,
            DemocraticRepublicOfTheCongoSubregions::SouthKivu => 0x17,
            DemocraticRepublicOfTheCongoSubregions::SudUbangi => 0x18,
            DemocraticRepublicOfTheCongoSubregions::TanganyikaProvince => 0x19,
            DemocraticRepublicOfTheCongoSubregions::Tshopo => 0x1A,
            DemocraticRepublicOfTheCongoSubregions::Tshuapa => 0x1B,
        }
    }
}

impl Display for DemocraticRepublicOfTheCongoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DemocraticRepublicOfTheCongo => write!(f, "Democratic Republic of the Congo"),
            Self::Kinshasa => write!(f, "Kinshasa"),
            Self::BasUele => write!(f, "Bas-Uele"),
            Self::Equateur => write!(f, "Équateur"),
            Self::HautKatangaProvince => write!(f, "Haut-Katanga Province"),
            Self::HautLomami => write!(f, "Haut-Lomami"),
            Self::HautUele => write!(f, "Haut-Uele"),
            Self::IturiProvince => write!(f, "Ituri Province"),
            Self::KasaiCentral => write!(f, "Kasaï-Central"),
            Self::KasaiOriental => write!(f, "Kasaï-Oriental"),
            Self::KasaiProvince => write!(f, "Kasaï Province"),
            Self::KongoCentral => write!(f, "Kongo Central"),
            Self::Kwango => write!(f, "Kwango"),
            Self::KwiluProvince => write!(f, "Kwilu Province"),
            Self::LomamiProvince => write!(f, "Lomami Province"),
            Self::LualabaProvince => write!(f, "Lualaba Province"),
            Self::MaiNdombeProvince => write!(f, "Mai-Ndombe Province"),
            Self::Maniema => write!(f, "Maniema"),
            Self::Mongala => write!(f, "Mongala"),
            Self::NordUbangi => write!(f, "Nord-Ubangi"),
            Self::NorthKivu => write!(f, "North Kivu"),
            Self::Sankuru => write!(f, "Sankuru"),
            Self::SouthKivu => write!(f, "South Kivu"),
            Self::SudUbangi => write!(f, "Sud-Ubangi"),
            Self::TanganyikaProvince => write!(f, "Tanganyika Province"),
            Self::Tshopo => write!(f, "Tshopo"),
            Self::Tshuapa => write!(f, "Tshuapa"),
        }
    }
}

// Denmark Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DenmarkSubregions {
    Denmark,
    Copenhagen,
    Frederiksberg,
    CopenhagenCounty,
    Frederiksborg,
    Roskilde,
    WestZealand,
    Storstrøm,
    Funen,
    SouthJutland,
    Ribe,
    Vejle,
    Ringkjøbing,
    Viborg,
    NorthJutland,
    Århus,
    Bornholm,
    Greenland,
}

impl TryFrom<u8> for DenmarkSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Denmark),
            0x02 => Ok(Self::Copenhagen),
            0x03 => Ok(Self::Frederiksberg),
            0x04 => Ok(Self::CopenhagenCounty),
            0x05 => Ok(Self::Frederiksborg),
            0x06 => Ok(Self::Roskilde),
            0x07 => Ok(Self::WestZealand),
            0x08 => Ok(Self::Storstrøm),
            0x09 => Ok(Self::Funen),
            0x0A => Ok(Self::SouthJutland),
            0x0B => Ok(Self::Ribe),
            0x0C => Ok(Self::Vejle),
            0x0D => Ok(Self::Ringkjøbing),
            0x0E => Ok(Self::Viborg),
            0x0F => Ok(Self::NorthJutland),
            0x10 => Ok(Self::Århus),
            0x11 => Ok(Self::Bornholm),
            0x12 => Ok(Self::Greenland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<DenmarkSubregions> for u8 {
    fn from(value: DenmarkSubregions) -> u8 {
        match value {
            DenmarkSubregions::Denmark => 0x01,
            DenmarkSubregions::Copenhagen => 0x02,
            DenmarkSubregions::Frederiksberg => 0x03,
            DenmarkSubregions::CopenhagenCounty => 0x04,
            DenmarkSubregions::Frederiksborg => 0x05,
            DenmarkSubregions::Roskilde => 0x06,
            DenmarkSubregions::WestZealand => 0x07,
            DenmarkSubregions::Storstrøm => 0x08,
            DenmarkSubregions::Funen => 0x09,
            DenmarkSubregions::SouthJutland => 0x0A,
            DenmarkSubregions::Ribe => 0x0B,
            DenmarkSubregions::Vejle => 0x0C,
            DenmarkSubregions::Ringkjøbing => 0x0D,
            DenmarkSubregions::Viborg => 0x0E,
            DenmarkSubregions::NorthJutland => 0x0F,
            DenmarkSubregions::Århus => 0x10,
            DenmarkSubregions::Bornholm => 0x11,
            DenmarkSubregions::Greenland => 0x12,
        }
    }
}

impl Display for DenmarkSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Denmark => write!(f, "Denmark"),
            Self::Copenhagen => write!(f, "Copenhagen"),
            Self::Frederiksberg => write!(f, "Frederiksberg"),
            Self::CopenhagenCounty => write!(f, "Copenhagen County"),
            Self::Frederiksborg => write!(f, "Frederiksborg"),
            Self::Roskilde => write!(f, "Roskilde"),
            Self::WestZealand => write!(f, "West Zealand"),
            Self::Storstrøm => write!(f, "Storstrøm"),
            Self::Funen => write!(f, "Funen"),
            Self::SouthJutland => write!(f, "South Jutland"),
            Self::Ribe => write!(f, "Ribe"),
            Self::Vejle => write!(f, "Vejle"),
            Self::Ringkjøbing => write!(f, "Ringkjøbing"),
            Self::Viborg => write!(f, "Viborg"),
            Self::NorthJutland => write!(f, "North Jutland"),
            Self::Århus => write!(f, "Århus"),
            Self::Bornholm => write!(f, "Bornholm"),
            Self::Greenland => write!(f, "Greenland"),
        }
    }
}

// Djibouti Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DjiboutiSubregions {
    Djibouti,
    DjiboutiRegion,
    AliSabiehRegion,
    ArtaRegion,
    DikhilRegion,
    ObockRegion,
    TadjourahRegion,
}

impl TryFrom<u8> for DjiboutiSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Djibouti),
            0x02 => Ok(Self::DjiboutiRegion),
            0x03 => Ok(Self::AliSabiehRegion),
            0x04 => Ok(Self::ArtaRegion),
            0x05 => Ok(Self::DikhilRegion),
            0x06 => Ok(Self::ObockRegion),
            0x07 => Ok(Self::TadjourahRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<DjiboutiSubregions> for u8 {
    fn from(value: DjiboutiSubregions) -> u8 {
        match value {
            DjiboutiSubregions::Djibouti => 0x01,
            DjiboutiSubregions::DjiboutiRegion => 0x02,
            DjiboutiSubregions::AliSabiehRegion => 0x03,
            DjiboutiSubregions::ArtaRegion => 0x04,
            DjiboutiSubregions::DikhilRegion => 0x05,
            DjiboutiSubregions::ObockRegion => 0x06,
            DjiboutiSubregions::TadjourahRegion => 0x07,
        }
    }
}

impl Display for DjiboutiSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Djibouti => write!(f, "Djibouti"),
            Self::DjiboutiRegion => write!(f, "Djibouti Region"),
            Self::AliSabiehRegion => write!(f, "Ali Sabieh Region"),
            Self::ArtaRegion => write!(f, "Arta Region"),
            Self::DikhilRegion => write!(f, "Dikhil Region"),
            Self::ObockRegion => write!(f, "Obock Region"),
            Self::TadjourahRegion => write!(f, "Tadjourah Region"),
        }
    }
}

// Dominica Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DominicaSubregions {
    Dominica,
}

impl TryFrom<u8> for DominicaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Dominica),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<DominicaSubregions> for u8 {
    fn from(value: DominicaSubregions) -> u8 {
        match value {
            DominicaSubregions::Dominica => 0x01,
        }
    }
}

impl Display for DominicaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dominica => write!(f, "Dominica"),
        }
    }
}

// Dominican Republic Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DominicanRepublicSubregions {
    DominicanRepublic,
    DistritoNacional,
    Azua,
    Baoruco,
    Barahona,
    Dajabon,
    Duarte,
    Espaillat,
    Independencia,
    LaAltagracia,
    EliasPina,
    LaRomana,
    MariaTrinidadSanchez,
    MonteCristi,
    Pedernales,
    Peravia,
    PuertoPlata,
    Salcedo,
    Samana,
    SanchezRamirez,
    SanJuan,
    SanPedroDeMacoris,
    Santiago,
    SantiagoRodriguez,
    Valverde,
    ElSeibo,
    HatoMayor,
    LaVega,
    MonsenorNouel,
    MontePlata,
    SanCristobal,
}

impl TryFrom<u8> for DominicanRepublicSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::DominicanRepublic),
            0x02 => Ok(Self::DistritoNacional),
            0x03 => Ok(Self::Azua),
            0x04 => Ok(Self::Baoruco),
            0x05 => Ok(Self::Barahona),
            0x06 => Ok(Self::Dajabon),
            0x07 => Ok(Self::Duarte),
            0x08 => Ok(Self::Espaillat),
            0x09 => Ok(Self::Independencia),
            0x0A => Ok(Self::LaAltagracia),
            0x0B => Ok(Self::EliasPina),
            0x0C => Ok(Self::LaRomana),
            0x0D => Ok(Self::MariaTrinidadSanchez),
            0x0E => Ok(Self::MonteCristi),
            0x0F => Ok(Self::Pedernales),
            0x10 => Ok(Self::Peravia),
            0x11 => Ok(Self::PuertoPlata),
            0x12 => Ok(Self::Salcedo),
            0x13 => Ok(Self::Samana),
            0x14 => Ok(Self::SanchezRamirez),
            0x15 => Ok(Self::SanJuan),
            0x16 => Ok(Self::SanPedroDeMacoris),
            0x17 => Ok(Self::Santiago),
            0x18 => Ok(Self::SantiagoRodriguez),
            0x19 => Ok(Self::Valverde),
            0x1A => Ok(Self::ElSeibo),
            0x1B => Ok(Self::HatoMayor),
            0x1C => Ok(Self::LaVega),
            0x1D => Ok(Self::MonsenorNouel),
            0x1E => Ok(Self::MontePlata),
            0x1F => Ok(Self::SanCristobal),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<DominicanRepublicSubregions> for u8 {
    fn from(value: DominicanRepublicSubregions) -> u8 {
        match value {
            DominicanRepublicSubregions::DominicanRepublic => 0x01,
            DominicanRepublicSubregions::DistritoNacional => 0x02,
            DominicanRepublicSubregions::Azua => 0x03,
            DominicanRepublicSubregions::Baoruco => 0x04,
            DominicanRepublicSubregions::Barahona => 0x05,
            DominicanRepublicSubregions::Dajabon => 0x06,
            DominicanRepublicSubregions::Duarte => 0x07,
            DominicanRepublicSubregions::Espaillat => 0x08,
            DominicanRepublicSubregions::Independencia => 0x09,
            DominicanRepublicSubregions::LaAltagracia => 0x0A,
            DominicanRepublicSubregions::EliasPina => 0x0B,
            DominicanRepublicSubregions::LaRomana => 0x0C,
            DominicanRepublicSubregions::MariaTrinidadSanchez => 0x0D,
            DominicanRepublicSubregions::MonteCristi => 0x0E,
            DominicanRepublicSubregions::Pedernales => 0x0F,
            DominicanRepublicSubregions::Peravia => 0x10,
            DominicanRepublicSubregions::PuertoPlata => 0x11,
            DominicanRepublicSubregions::Salcedo => 0x12,
            DominicanRepublicSubregions::Samana => 0x13,
            DominicanRepublicSubregions::SanchezRamirez => 0x14,
            DominicanRepublicSubregions::SanJuan => 0x15,
            DominicanRepublicSubregions::SanPedroDeMacoris => 0x16,
            DominicanRepublicSubregions::Santiago => 0x17,
            DominicanRepublicSubregions::SantiagoRodriguez => 0x18,
            DominicanRepublicSubregions::Valverde => 0x19,
            DominicanRepublicSubregions::ElSeibo => 0x1A,
            DominicanRepublicSubregions::HatoMayor => 0x1B,
            DominicanRepublicSubregions::LaVega => 0x1C,
            DominicanRepublicSubregions::MonsenorNouel => 0x1D,
            DominicanRepublicSubregions::MontePlata => 0x1E,
            DominicanRepublicSubregions::SanCristobal => 0x1F,
        }
    }
}

impl Display for DominicanRepublicSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DominicanRepublic => write!(f, "Dominican Republic"),
            Self::DistritoNacional => write!(f, "Distrito Nacional"),
            Self::Azua => write!(f, "Azua"),
            Self::Baoruco => write!(f, "Baoruco"),
            Self::Barahona => write!(f, "Barahona"),
            Self::Dajabon => write!(f, "Dajabón"),
            Self::Duarte => write!(f, "Duarte"),
            Self::Espaillat => write!(f, "Espaillat"),
            Self::Independencia => write!(f, "Independencia"),
            Self::LaAltagracia => write!(f, "La Altagracia"),
            Self::EliasPina => write!(f, "Elías Piña"),
            Self::LaRomana => write!(f, "La Romana"),
            Self::MariaTrinidadSanchez => write!(f, "María Trinidad Sánchez"),
            Self::MonteCristi => write!(f, "Monte Cristi"),
            Self::Pedernales => write!(f, "Pedernales"),
            Self::Peravia => write!(f, "Peravia"),
            Self::PuertoPlata => write!(f, "Puerto Plata"),
            Self::Salcedo => write!(f, "Salcedo"),
            Self::Samana => write!(f, "Samaná"),
            Self::SanchezRamirez => write!(f, "Sánchez Ramírez"),
            Self::SanJuan => write!(f, "San Juan"),
            Self::SanPedroDeMacoris => write!(f, "San Pedro de Macorís"),
            Self::Santiago => write!(f, "Santiago"),
            Self::SantiagoRodriguez => write!(f, "Santiago Rodríguez"),
            Self::Valverde => write!(f, "Valverde"),
            Self::ElSeibo => write!(f, "El Seíbo"),
            Self::HatoMayor => write!(f, "Hato Mayor"),
            Self::LaVega => write!(f, "La Vega"),
            Self::MonsenorNouel => write!(f, "Monseñor Nouel"),
            Self::MontePlata => write!(f, "Monte Plata"),
            Self::SanCristobal => write!(f, "San Cristóbal"),
        }
    }
}

// Ecuador Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EcuadorSubregions {
    Ecuador,
    Pichincha,
    Galapagos,
    Azuay,
    Bolivar,
    Canar,
    Carchi,
    Chimborazo,
    Cotopaxi,
    ElOro,
    Esmeraldas,
    Guayas,
    Imbabura,
    Loja,
    LosRios,
    Manabi,
    MoronaSantiago,
    Pastaza,
    Tungurahua,
    ZamoraChinchipe,
    Sucumbios,
    Napo,
    Orellana,
}

impl TryFrom<u8> for EcuadorSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Ecuador),
            0x02 => Ok(Self::Pichincha),
            0x03 => Ok(Self::Galapagos),
            0x04 => Ok(Self::Azuay),
            0x05 => Ok(Self::Bolivar),
            0x06 => Ok(Self::Canar),
            0x07 => Ok(Self::Carchi),
            0x08 => Ok(Self::Chimborazo),
            0x09 => Ok(Self::Cotopaxi),
            0x0A => Ok(Self::ElOro),
            0x0B => Ok(Self::Esmeraldas),
            0x0C => Ok(Self::Guayas),
            0x0D => Ok(Self::Imbabura),
            0x0E => Ok(Self::Loja),
            0x0F => Ok(Self::LosRios),
            0x10 => Ok(Self::Manabi),
            0x11 => Ok(Self::MoronaSantiago),
            0x12 => Ok(Self::Pastaza),
            0x13 => Ok(Self::Tungurahua),
            0x14 => Ok(Self::ZamoraChinchipe),
            0x15 => Ok(Self::Sucumbios),
            0x16 => Ok(Self::Napo),
            0x17 => Ok(Self::Orellana),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<EcuadorSubregions> for u8 {
    fn from(value: EcuadorSubregions) -> u8 {
        match value {
            EcuadorSubregions::Ecuador => 0x01,
            EcuadorSubregions::Pichincha => 0x02,
            EcuadorSubregions::Galapagos => 0x03,
            EcuadorSubregions::Azuay => 0x04,
            EcuadorSubregions::Bolivar => 0x05,
            EcuadorSubregions::Canar => 0x06,
            EcuadorSubregions::Carchi => 0x07,
            EcuadorSubregions::Chimborazo => 0x08,
            EcuadorSubregions::Cotopaxi => 0x09,
            EcuadorSubregions::ElOro => 0x0A,
            EcuadorSubregions::Esmeraldas => 0x0B,
            EcuadorSubregions::Guayas => 0x0C,
            EcuadorSubregions::Imbabura => 0x0D,
            EcuadorSubregions::Loja => 0x0E,
            EcuadorSubregions::LosRios => 0x0F,
            EcuadorSubregions::Manabi => 0x10,
            EcuadorSubregions::MoronaSantiago => 0x11,
            EcuadorSubregions::Pastaza => 0x12,
            EcuadorSubregions::Tungurahua => 0x13,
            EcuadorSubregions::ZamoraChinchipe => 0x14,
            EcuadorSubregions::Sucumbios => 0x15,
            EcuadorSubregions::Napo => 0x16,
            EcuadorSubregions::Orellana => 0x17,
        }
    }
}

impl Display for EcuadorSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ecuador => write!(f, "Ecuador"),
            Self::Pichincha => write!(f, "Pichincha"),
            Self::Galapagos => write!(f, "Galápagos"),
            Self::Azuay => write!(f, "Azuay"),
            Self::Bolivar => write!(f, "Bolívar"),
            Self::Canar => write!(f, "Cañar"),
            Self::Carchi => write!(f, "Carchi"),
            Self::Chimborazo => write!(f, "Chimborazo"),
            Self::Cotopaxi => write!(f, "Cotopaxi"),
            Self::ElOro => write!(f, "El Oro"),
            Self::Esmeraldas => write!(f, "Esmeraldas"),
            Self::Guayas => write!(f, "Guayas"),
            Self::Imbabura => write!(f, "Imbabura"),
            Self::Loja => write!(f, "Loja"),
            Self::LosRios => write!(f, "Los Ríos"),
            Self::Manabi => write!(f, "Manabí"),
            Self::MoronaSantiago => write!(f, "Morona-Santiago"),
            Self::Pastaza => write!(f, "Pastaza"),
            Self::Tungurahua => write!(f, "Tungurahua"),
            Self::ZamoraChinchipe => write!(f, "Zamora-Chinchipe"),
            Self::Sucumbios => write!(f, "Sucumbios"),
            Self::Napo => write!(f, "Napo"),
            Self::Orellana => write!(f, "Orellana"),
        }
    }
}

// Egypt Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EgyptSubregions {
    Egypt,
    AlQahirah,
    AdDaqahliyah,
    AlBahrAlAhmar,
    AlBuhayrah,
    AlFayyum,
    AlGharbiyah,
    AlIskandariyah,
    AlIsmailiyah,
    AlJizah,
    AlMinufiyah,
    AlMinya,
    AlQalyubiyah,
    AlWadiAlJadid,
    AshSharqiyah,
    AsSuways,
    Aswan,
    Asyut,
    BaniSuwayf,
    BurSaid,
    Dumyat,
    KafrAshShaykh,
    Matruh,
    Qina,
    Suhaj,
    JanubSina,
    ShamalSina,
    Luxor,
}

impl TryFrom<u8> for EgyptSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Egypt),
            0x02 => Ok(Self::AlQahirah),
            0x03 => Ok(Self::AdDaqahliyah),
            0x04 => Ok(Self::AlBahrAlAhmar),
            0x05 => Ok(Self::AlBuhayrah),
            0x06 => Ok(Self::AlFayyum),
            0x07 => Ok(Self::AlGharbiyah),
            0x08 => Ok(Self::AlIskandariyah),
            0x09 => Ok(Self::AlIsmailiyah),
            0x0A => Ok(Self::AlJizah),
            0x0B => Ok(Self::AlMinufiyah),
            0x0C => Ok(Self::AlMinya),
            0x0D => Ok(Self::AlQalyubiyah),
            0x0E => Ok(Self::AlWadiAlJadid),
            0x0F => Ok(Self::AshSharqiyah),
            0x10 => Ok(Self::AsSuways),
            0x11 => Ok(Self::Aswan),
            0x12 => Ok(Self::Asyut),
            0x13 => Ok(Self::BaniSuwayf),
            0x14 => Ok(Self::BurSaid),
            0x15 => Ok(Self::Dumyat),
            0x16 => Ok(Self::KafrAshShaykh),
            0x17 => Ok(Self::Matruh),
            0x18 => Ok(Self::Qina),
            0x19 => Ok(Self::Suhaj),
            0x1A => Ok(Self::JanubSina),
            0x1B => Ok(Self::ShamalSina),
            0x1C => Ok(Self::Luxor),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<EgyptSubregions> for u8 {
    fn from(value: EgyptSubregions) -> u8 {
        match value {
            EgyptSubregions::Egypt => 0x01,
            EgyptSubregions::AlQahirah => 0x02,
            EgyptSubregions::AdDaqahliyah => 0x03,
            EgyptSubregions::AlBahrAlAhmar => 0x04,
            EgyptSubregions::AlBuhayrah => 0x05,
            EgyptSubregions::AlFayyum => 0x06,
            EgyptSubregions::AlGharbiyah => 0x07,
            EgyptSubregions::AlIskandariyah => 0x08,
            EgyptSubregions::AlIsmailiyah => 0x09,
            EgyptSubregions::AlJizah => 0x0A,
            EgyptSubregions::AlMinufiyah => 0x0B,
            EgyptSubregions::AlMinya => 0x0C,
            EgyptSubregions::AlQalyubiyah => 0x0D,
            EgyptSubregions::AlWadiAlJadid => 0x0E,
            EgyptSubregions::AshSharqiyah => 0x0F,
            EgyptSubregions::AsSuways => 0x10,
            EgyptSubregions::Aswan => 0x11,
            EgyptSubregions::Asyut => 0x12,
            EgyptSubregions::BaniSuwayf => 0x13,
            EgyptSubregions::BurSaid => 0x14,
            EgyptSubregions::Dumyat => 0x15,
            EgyptSubregions::KafrAshShaykh => 0x16,
            EgyptSubregions::Matruh => 0x17,
            EgyptSubregions::Qina => 0x18,
            EgyptSubregions::Suhaj => 0x19,
            EgyptSubregions::JanubSina => 0x1A,
            EgyptSubregions::ShamalSina => 0x1B,
            EgyptSubregions::Luxor => 0x1C,
        }
    }
}

impl Display for EgyptSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Egypt => write!(f, "Egypt"),
            Self::AlQahirah => write!(f, "Al Qahirah"),
            Self::AdDaqahliyah => write!(f, "Ad Daqahliyah"),
            Self::AlBahrAlAhmar => write!(f, "Al Bahr al Ahmar"),
            Self::AlBuhayrah => write!(f, "Al Buhayrah"),
            Self::AlFayyum => write!(f, "Al Fayyum"),
            Self::AlGharbiyah => write!(f, "Al Gharbiyah"),
            Self::AlIskandariyah => write!(f, "Al Iskandariyah"),
            Self::AlIsmailiyah => write!(f, "Al Isma‘iliyah"),
            Self::AlJizah => write!(f, "Al Jizah"),
            Self::AlMinufiyah => write!(f, "Al Minufiyah"),
            Self::AlMinya => write!(f, "Al Minya"),
            Self::AlQalyubiyah => write!(f, "Al Qalyubiyah"),
            Self::AlWadiAlJadid => write!(f, "Al Wadi al Jadid"),
            Self::AshSharqiyah => write!(f, "Ash Sharqiyah"),
            Self::AsSuways => write!(f, "As Suways"),
            Self::Aswan => write!(f, "Aswan"),
            Self::Asyut => write!(f, "Asyut"),
            Self::BaniSuwayf => write!(f, "Bani Suwayf"),
            Self::BurSaid => write!(f, "Bur Sa‘id"),
            Self::Dumyat => write!(f, "Dumyat"),
            Self::KafrAshShaykh => write!(f, "Kafr ash Shaykh"),
            Self::Matruh => write!(f, "Matruh"),
            Self::Qina => write!(f, "Qina"),
            Self::Suhaj => write!(f, "Suhaj"),
            Self::JanubSina => write!(f, "Janub Sina'"),
            Self::ShamalSina => write!(f, "Shamal Sina'"),
            Self::Luxor => write!(f, "Luxor"),
        }
    }
}

// El Salvador Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElSalvadorSubregions {
    ElSalvador,
    SanSalvador,
    Ahuachapan,
    Cabanas,
    Chalatenango,
    Cuscatlan,
    LaLibertad,
    LaPaz,
    LaUnion,
    Morazan,
    SanMiguel,
    SantaAna,
    SanVicente,
    Sonsonate,
    USulutan,
}

impl TryFrom<u8> for ElSalvadorSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::ElSalvador),
            0x02 => Ok(Self::SanSalvador),
            0x03 => Ok(Self::Ahuachapan),
            0x04 => Ok(Self::Cabanas),
            0x05 => Ok(Self::Chalatenango),
            0x06 => Ok(Self::Cuscatlan),
            0x07 => Ok(Self::LaLibertad),
            0x08 => Ok(Self::LaPaz),
            0x09 => Ok(Self::LaUnion),
            0x0A => Ok(Self::Morazan),
            0x0B => Ok(Self::SanMiguel),
            0x0C => Ok(Self::SantaAna),
            0x0D => Ok(Self::SanVicente),
            0x0E => Ok(Self::Sonsonate),
            0x0F => Ok(Self::USulutan),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ElSalvadorSubregions> for u8 {
    fn from(value: ElSalvadorSubregions) -> u8 {
        match value {
            ElSalvadorSubregions::ElSalvador => 0x01,
            ElSalvadorSubregions::SanSalvador => 0x02,
            ElSalvadorSubregions::Ahuachapan => 0x03,
            ElSalvadorSubregions::Cabanas => 0x04,
            ElSalvadorSubregions::Chalatenango => 0x05,
            ElSalvadorSubregions::Cuscatlan => 0x06,
            ElSalvadorSubregions::LaLibertad => 0x07,
            ElSalvadorSubregions::LaPaz => 0x08,
            ElSalvadorSubregions::LaUnion => 0x09,
            ElSalvadorSubregions::Morazan => 0x0A,
            ElSalvadorSubregions::SanMiguel => 0x0B,
            ElSalvadorSubregions::SantaAna => 0x0C,
            ElSalvadorSubregions::SanVicente => 0x0D,
            ElSalvadorSubregions::Sonsonate => 0x0E,
            ElSalvadorSubregions::USulutan => 0x0F,
        }
    }
}

impl Display for ElSalvadorSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ElSalvador => write!(f, "El Salvador"),
            Self::SanSalvador => write!(f, "San Salvador"),
            Self::Ahuachapan => write!(f, "Ahuachapán"),
            Self::Cabanas => write!(f, "Cabañas"),
            Self::Chalatenango => write!(f, "Chalatenango"),
            Self::Cuscatlan => write!(f, "Cuscatlán"),
            Self::LaLibertad => write!(f, "La Libertad"),
            Self::LaPaz => write!(f, "La Paz"),
            Self::LaUnion => write!(f, "La Unión"),
            Self::Morazan => write!(f, "Morazán"),
            Self::SanMiguel => write!(f, "San Miguel"),
            Self::SantaAna => write!(f, "Santa Ana"),
            Self::SanVicente => write!(f, "San Vicente"),
            Self::Sonsonate => write!(f, "Sonsonate"),
            Self::USulutan => write!(f, "Usulután"),
        }
    }
}

// Equatorial Guinea Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EquatorialGuineaSubregions {
    EquatorialGuinea,
    InsularRegion,
    ContinentalRegion,
}

impl TryFrom<u8> for EquatorialGuineaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::EquatorialGuinea),
            0x02 => Ok(Self::InsularRegion),
            0x03 => Ok(Self::ContinentalRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<EquatorialGuineaSubregions> for u8 {
    fn from(value: EquatorialGuineaSubregions) -> u8 {
        match value {
            EquatorialGuineaSubregions::EquatorialGuinea => 0x01,
            EquatorialGuineaSubregions::InsularRegion => 0x02,
            EquatorialGuineaSubregions::ContinentalRegion => 0x03,
        }
    }
}

impl Display for EquatorialGuineaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EquatorialGuinea => write!(f, "Equatorial Guinea"),
            Self::InsularRegion => write!(f, "Insular Region"),
            Self::ContinentalRegion => write!(f, "Continental Region"),
        }
    }
}

// Eritrea Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EritreaSubregions {
    Eritrea,
    CentralRegion,
    AnsebaRegion,
    GashBarkaRegion,
    NorthernRedSeaRegion,
    SouthernRedSeaRegion,
    SouthernRegion,
}

impl TryFrom<u8> for EritreaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Eritrea),
            0x02 => Ok(Self::CentralRegion),
            0x03 => Ok(Self::AnsebaRegion),
            0x04 => Ok(Self::GashBarkaRegion),
            0x05 => Ok(Self::NorthernRedSeaRegion),
            0x06 => Ok(Self::SouthernRedSeaRegion),
            0x07 => Ok(Self::SouthernRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<EritreaSubregions> for u8 {
    fn from(value: EritreaSubregions) -> u8 {
        match value {
            EritreaSubregions::Eritrea => 0x01,
            EritreaSubregions::CentralRegion => 0x02,
            EritreaSubregions::AnsebaRegion => 0x03,
            EritreaSubregions::GashBarkaRegion => 0x04,
            EritreaSubregions::NorthernRedSeaRegion => 0x05,
            EritreaSubregions::SouthernRedSeaRegion => 0x06,
            EritreaSubregions::SouthernRegion => 0x07,
        }
    }
}

impl Display for EritreaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eritrea => write!(f, "Eritrea"),
            Self::CentralRegion => write!(f, "Central Region"),
            Self::AnsebaRegion => write!(f, "Anseba Region"),
            Self::GashBarkaRegion => write!(f, "Gash-Barka Region"),
            Self::NorthernRedSeaRegion => write!(f, "Northern Red Sea Region"),
            Self::SouthernRedSeaRegion => write!(f, "Southern Red Sea Region"),
            Self::SouthernRegion => write!(f, "Southern Region"),
        }
    }
}

// Estonia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EstoniaSubregions {
    Estonia,
    Harju,
    Hiiu,
    IdaViru,
    Jogeva,
    Jarva,
    Laane,
    LaaneViru,
    Polva,
    Parnu,
    Rapla,
    Saare,
    Tartu,
    Valga,
    Viljandi,
    Voru,
}

impl TryFrom<u8> for EstoniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Estonia),
            0x02 => Ok(Self::Harju),
            0x03 => Ok(Self::Hiiu),
            0x04 => Ok(Self::IdaViru),
            0x05 => Ok(Self::Jogeva),
            0x06 => Ok(Self::Jarva),
            0x07 => Ok(Self::Laane),
            0x08 => Ok(Self::LaaneViru),
            0x09 => Ok(Self::Polva),
            0x0A => Ok(Self::Parnu),
            0x0B => Ok(Self::Rapla),
            0x0C => Ok(Self::Saare),
            0x0D => Ok(Self::Tartu),
            0x0E => Ok(Self::Valga),
            0x0F => Ok(Self::Viljandi),
            0x10 => Ok(Self::Voru),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<EstoniaSubregions> for u8 {
    fn from(value: EstoniaSubregions) -> u8 {
        match value {
            EstoniaSubregions::Estonia => 0x01,
            EstoniaSubregions::Harju => 0x02,
            EstoniaSubregions::Hiiu => 0x03,
            EstoniaSubregions::IdaViru => 0x04,
            EstoniaSubregions::Jogeva => 0x05,
            EstoniaSubregions::Jarva => 0x06,
            EstoniaSubregions::Laane => 0x07,
            EstoniaSubregions::LaaneViru => 0x08,
            EstoniaSubregions::Polva => 0x09,
            EstoniaSubregions::Parnu => 0x0A,
            EstoniaSubregions::Rapla => 0x0B,
            EstoniaSubregions::Saare => 0x0C,
            EstoniaSubregions::Tartu => 0x0D,
            EstoniaSubregions::Valga => 0x0E,
            EstoniaSubregions::Viljandi => 0x0F,
            EstoniaSubregions::Voru => 0x10,
        }
    }
}

impl Display for EstoniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Estonia => write!(f, "Estonia"),
            Self::Harju => write!(f, "Harju"),
            Self::Hiiu => write!(f, "Hiiu"),
            Self::IdaViru => write!(f, "Ida-Viru"),
            Self::Jogeva => write!(f, "Jõgeva"),
            Self::Jarva => write!(f, "Järva"),
            Self::Laane => write!(f, "Lääne"),
            Self::LaaneViru => write!(f, "Lääne-Viru"),
            Self::Polva => write!(f, "Põlva"),
            Self::Parnu => write!(f, "Pärnu"),
            Self::Rapla => write!(f, "Rapla"),
            Self::Saare => write!(f, "Saare"),
            Self::Tartu => write!(f, "Tartu"),
            Self::Valga => write!(f, "Valga"),
            Self::Viljandi => write!(f, "Viljandi"),
            Self::Voru => write!(f, "Võru"),
        }
    }
}

// Eswatini Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EswatiniSubregions {
    Eswatini,
    Hhohho,
    Lubombo,
    Manzini,
    Shiselweni,
}

impl TryFrom<u8> for EswatiniSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Eswatini),
            0x02 => Ok(Self::Hhohho),
            0x03 => Ok(Self::Lubombo),
            0x04 => Ok(Self::Manzini),
            0x05 => Ok(Self::Shiselweni),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<EswatiniSubregions> for u8 {
    fn from(value: EswatiniSubregions) -> u8 {
        match value {
            EswatiniSubregions::Eswatini => 0x01,
            EswatiniSubregions::Hhohho => 0x02,
            EswatiniSubregions::Lubombo => 0x03,
            EswatiniSubregions::Manzini => 0x04,
            EswatiniSubregions::Shiselweni => 0x05,
        }
    }
}

impl Display for EswatiniSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eswatini => write!(f, "Eswatini"),
            Self::Hhohho => write!(f, "Hhohho"),
            Self::Lubombo => write!(f, "Lubombo"),
            Self::Manzini => write!(f, "Manzini"),
            Self::Shiselweni => write!(f, "Shiselweni"),
        }
    }
}

// Ethiopia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EthiopiaSubregions {
    Ethiopia,
    OromiaRegion,
    AfarRegion,
    AmharaRegion,
    BenishangulGumuzRegion,
    DireDawa,
    GambelaRegion,
    HarariRegion,
    SomaliRegion,
    SouthernNationsNationalitiesAndPeoplesRegion,
    TigrayRegion,
}

impl TryFrom<u8> for EthiopiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Ethiopia),
            0x02 => Ok(Self::OromiaRegion),
            0x03 => Ok(Self::AfarRegion),
            0x04 => Ok(Self::AmharaRegion),
            0x05 => Ok(Self::BenishangulGumuzRegion),
            0x06 => Ok(Self::DireDawa),
            0x07 => Ok(Self::GambelaRegion),
            0x08 => Ok(Self::HarariRegion),
            0x09 => Ok(Self::SomaliRegion),
            0x0A => Ok(Self::SouthernNationsNationalitiesAndPeoplesRegion),
            0x0B => Ok(Self::TigrayRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<EthiopiaSubregions> for u8 {
    fn from(value: EthiopiaSubregions) -> u8 {
        match value {
            EthiopiaSubregions::Ethiopia => 0x01,
            EthiopiaSubregions::OromiaRegion => 0x02,
            EthiopiaSubregions::AfarRegion => 0x03,
            EthiopiaSubregions::AmharaRegion => 0x04,
            EthiopiaSubregions::BenishangulGumuzRegion => 0x05,
            EthiopiaSubregions::DireDawa => 0x06,
            EthiopiaSubregions::GambelaRegion => 0x07,
            EthiopiaSubregions::HarariRegion => 0x08,
            EthiopiaSubregions::SomaliRegion => 0x09,
            EthiopiaSubregions::SouthernNationsNationalitiesAndPeoplesRegion => 0x0A,
            EthiopiaSubregions::TigrayRegion => 0x0B,
        }
    }
}

impl Display for EthiopiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ethiopia => write!(f, "Ethiopia"),
            Self::OromiaRegion => write!(f, "Oromia Region"),
            Self::AfarRegion => write!(f, "Afar Region"),
            Self::AmharaRegion => write!(f, "Amhara Region"),
            Self::BenishangulGumuzRegion => write!(f, "Benishangul-Gumuz Region"),
            Self::DireDawa => write!(f, "Dire Dawa"),
            Self::GambelaRegion => write!(f, "Gambela Region"),
            Self::HarariRegion => write!(f, "Harari Region"),
            Self::SomaliRegion => write!(f, "Somali Region"),
            Self::SouthernNationsNationalitiesAndPeoplesRegion => {
                write!(f, "Southern Nations, Nationalities, and Peoples' Region")
            }
            Self::TigrayRegion => write!(f, "Tigray Region"),
        }
    }
}

// Falkland Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FalklandIslandsSubregions {
    FalklandIslands,
    EastFalkland,
    WestFalkland,
}

impl TryFrom<u8> for FalklandIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::FalklandIslands),
            0x02 => Ok(Self::EastFalkland),
            0x03 => Ok(Self::WestFalkland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FalklandIslandsSubregions> for u8 {
    fn from(value: FalklandIslandsSubregions) -> u8 {
        match value {
            FalklandIslandsSubregions::FalklandIslands => 0x01,
            FalklandIslandsSubregions::EastFalkland => 0x02,
            FalklandIslandsSubregions::WestFalkland => 0x03,
        }
    }
}

impl Display for FalklandIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FalklandIslands => write!(f, "Falkland Islands"),
            Self::EastFalkland => write!(f, "East Falkland"),
            Self::WestFalkland => write!(f, "West Falkland"),
        }
    }
}

// Faroe Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaroeIslandsSubregions {
    FaroeIslands,
    Streymoy,
    Eysturoy,
    Nordoyar,
    Sandoy,
    Suduroy,
    Vagar,
}

impl TryFrom<u8> for FaroeIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::FaroeIslands),
            0x02 => Ok(Self::Streymoy),
            0x03 => Ok(Self::Eysturoy),
            0x04 => Ok(Self::Nordoyar),
            0x05 => Ok(Self::Sandoy),
            0x06 => Ok(Self::Suduroy),
            0x07 => Ok(Self::Vagar),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FaroeIslandsSubregions> for u8 {
    fn from(value: FaroeIslandsSubregions) -> u8 {
        match value {
            FaroeIslandsSubregions::FaroeIslands => 0x01,
            FaroeIslandsSubregions::Streymoy => 0x02,
            FaroeIslandsSubregions::Eysturoy => 0x03,
            FaroeIslandsSubregions::Nordoyar => 0x04,
            FaroeIslandsSubregions::Sandoy => 0x05,
            FaroeIslandsSubregions::Suduroy => 0x06,
            FaroeIslandsSubregions::Vagar => 0x07,
        }
    }
}

impl Display for FaroeIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FaroeIslands => write!(f, "Faroe Islands"),
            Self::Streymoy => write!(f, "Streymoy"),
            Self::Eysturoy => write!(f, "Eysturoy"),
            Self::Nordoyar => write!(f, "Nordoyar"),
            Self::Sandoy => write!(f, "Sandoy"),
            Self::Suduroy => write!(f, "Suduroy"),
            Self::Vagar => write!(f, "Vágar"),
        }
    }
}

// Federated States of Micronesia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FederatedStatesOfMicronesiaSubregions {
    FederatedStatesOfMicronesia,
    Pohnpei,
    Chuuk,
    Kosrae,
    Yap,
}

impl TryFrom<u8> for FederatedStatesOfMicronesiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::FederatedStatesOfMicronesia),
            0x02 => Ok(Self::Pohnpei),
            0x03 => Ok(Self::Chuuk),
            0x04 => Ok(Self::Kosrae),
            0x05 => Ok(Self::Yap),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FederatedStatesOfMicronesiaSubregions> for u8 {
    fn from(value: FederatedStatesOfMicronesiaSubregions) -> u8 {
        match value {
            FederatedStatesOfMicronesiaSubregions::FederatedStatesOfMicronesia => 0x01,
            FederatedStatesOfMicronesiaSubregions::Pohnpei => 0x02,
            FederatedStatesOfMicronesiaSubregions::Chuuk => 0x03,
            FederatedStatesOfMicronesiaSubregions::Kosrae => 0x04,
            FederatedStatesOfMicronesiaSubregions::Yap => 0x05,
        }
    }
}

impl Display for FederatedStatesOfMicronesiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FederatedStatesOfMicronesia => write!(f, "Federated States of Micronesia"),
            Self::Pohnpei => write!(f, "Pohnpei"),
            Self::Chuuk => write!(f, "Chuuk"),
            Self::Kosrae => write!(f, "Kosrae"),
            Self::Yap => write!(f, "Yap"),
        }
    }
}

// Fiji Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FijiSubregions {
    Fiji,
    CentralDivision,
    EasternDivision,
    NorthernDivision,
    WesternDivision,
}

impl TryFrom<u8> for FijiSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Fiji),
            0x02 => Ok(Self::CentralDivision),
            0x03 => Ok(Self::EasternDivision),
            0x04 => Ok(Self::NorthernDivision),
            0x05 => Ok(Self::WesternDivision),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FijiSubregions> for u8 {
    fn from(value: FijiSubregions) -> u8 {
        match value {
            FijiSubregions::Fiji => 0x01,
            FijiSubregions::CentralDivision => 0x02,
            FijiSubregions::EasternDivision => 0x03,
            FijiSubregions::NorthernDivision => 0x04,
            FijiSubregions::WesternDivision => 0x05,
        }
    }
}

impl Display for FijiSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fiji => write!(f, "Fiji"),
            Self::CentralDivision => write!(f, "Central Division"),
            Self::EasternDivision => write!(f, "Eastern Division"),
            Self::NorthernDivision => write!(f, "Northern Division"),
            Self::WesternDivision => write!(f, "Western Division"),
        }
    }
}

// Finland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FinlandSubregions {
    Finland,
    SouthernFinland,
    WesternFinland,
    EasternFinland,
    ProvinceOfOulu,
    Lapland,
    Åland,
}

impl TryFrom<u8> for FinlandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Finland),
            0x02 => Ok(Self::SouthernFinland),
            0x03 => Ok(Self::WesternFinland),
            0x04 => Ok(Self::EasternFinland),
            0x05 => Ok(Self::ProvinceOfOulu),
            0x06 => Ok(Self::Lapland),
            0x07 => Ok(Self::Åland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FinlandSubregions> for u8 {
    fn from(value: FinlandSubregions) -> u8 {
        match value {
            FinlandSubregions::Finland => 0x01,
            FinlandSubregions::SouthernFinland => 0x02,
            FinlandSubregions::WesternFinland => 0x03,
            FinlandSubregions::EasternFinland => 0x04,
            FinlandSubregions::ProvinceOfOulu => 0x05,
            FinlandSubregions::Lapland => 0x06,
            FinlandSubregions::Åland => 0x07,
        }
    }
}

impl Display for FinlandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Finland => write!(f, "Finland"),
            Self::SouthernFinland => write!(f, "Southern Finland"),
            Self::WesternFinland => write!(f, "Western Finland"),
            Self::EasternFinland => write!(f, "Eastern Finland"),
            Self::ProvinceOfOulu => write!(f, "Province of Oulu"),
            Self::Lapland => write!(f, "Lapland"),
            Self::Åland => write!(f, "Åland"),
        }
    }
}

// France Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FranceSubregions {
    France,
    IleDeFrance,
    Alsace,
    Aquitaine,
    Auvergne,
    LowerNormandy,
    Burgundy,
    Brittany,
    CentreLoireValley,
    ChampagneArdenne,
    Corsica,
    FrancheComte,
    UpperNormandy,
    LanguedocRoussillon,
    Limousin,
    Lorraine,
    MidiPyrenees,
    NordPasDeCalais,
    WesternLoireValley,
    Picardy,
    PoitouCharentes,
    ProvenceAlpesCoteDazur,
    RhoneAlpes,
    Guadeloupe,
    Martinique,
    FrenchGuiana,
    Reunion,
    Mayotte,
}

impl TryFrom<u8> for FranceSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::France),
            0x02 => Ok(Self::IleDeFrance),
            0x03 => Ok(Self::Alsace),
            0x04 => Ok(Self::Aquitaine),
            0x05 => Ok(Self::Auvergne),
            0x06 => Ok(Self::LowerNormandy),
            0x07 => Ok(Self::Burgundy),
            0x08 => Ok(Self::Brittany),
            0x09 => Ok(Self::CentreLoireValley),
            0x0A => Ok(Self::ChampagneArdenne),
            0x0B => Ok(Self::Corsica),
            0x0C => Ok(Self::FrancheComte),
            0x0D => Ok(Self::UpperNormandy),
            0x0E => Ok(Self::LanguedocRoussillon),
            0x0F => Ok(Self::Limousin),
            0x10 => Ok(Self::Lorraine),
            0x11 => Ok(Self::MidiPyrenees),
            0x12 => Ok(Self::NordPasDeCalais),
            0x13 => Ok(Self::WesternLoireValley),
            0x14 => Ok(Self::Picardy),
            0x15 => Ok(Self::PoitouCharentes),
            0x16 => Ok(Self::ProvenceAlpesCoteDazur),
            0x17 => Ok(Self::RhoneAlpes),
            0x18 => Ok(Self::Guadeloupe),
            0x19 => Ok(Self::Martinique),
            0x1A => Ok(Self::FrenchGuiana),
            0x1B => Ok(Self::Reunion),
            0x1C => Ok(Self::Mayotte),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FranceSubregions> for u8 {
    fn from(value: FranceSubregions) -> u8 {
        match value {
            FranceSubregions::France => 0x01,
            FranceSubregions::IleDeFrance => 0x02,
            FranceSubregions::Alsace => 0x03,
            FranceSubregions::Aquitaine => 0x04,
            FranceSubregions::Auvergne => 0x05,
            FranceSubregions::LowerNormandy => 0x06,
            FranceSubregions::Burgundy => 0x07,
            FranceSubregions::Brittany => 0x08,
            FranceSubregions::CentreLoireValley => 0x09,
            FranceSubregions::ChampagneArdenne => 0x0A,
            FranceSubregions::Corsica => 0x0B,
            FranceSubregions::FrancheComte => 0x0C,
            FranceSubregions::UpperNormandy => 0x0D,
            FranceSubregions::LanguedocRoussillon => 0x0E,
            FranceSubregions::Limousin => 0x0F,
            FranceSubregions::Lorraine => 0x10,
            FranceSubregions::MidiPyrenees => 0x11,
            FranceSubregions::NordPasDeCalais => 0x12,
            FranceSubregions::WesternLoireValley => 0x13,
            FranceSubregions::Picardy => 0x14,
            FranceSubregions::PoitouCharentes => 0x15,
            FranceSubregions::ProvenceAlpesCoteDazur => 0x16,
            FranceSubregions::RhoneAlpes => 0x17,
            FranceSubregions::Guadeloupe => 0x18,
            FranceSubregions::Martinique => 0x19,
            FranceSubregions::FrenchGuiana => 0x1A,
            FranceSubregions::Reunion => 0x1B,
            FranceSubregions::Mayotte => 0x1C,
        }
    }
}

impl Display for FranceSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::France => write!(f, "France"),
            Self::IleDeFrance => write!(f, "Île-de-France"),
            Self::Alsace => write!(f, "Alsace"),
            Self::Aquitaine => write!(f, "Aquitaine"),
            Self::Auvergne => write!(f, "Auvergne"),
            Self::LowerNormandy => write!(f, "Lower Normandy"),
            Self::Burgundy => write!(f, "Burgundy"),
            Self::Brittany => write!(f, "Brittany"),
            Self::CentreLoireValley => write!(f, "Centre Loire Valley"),
            Self::ChampagneArdenne => write!(f, "Champagne-Ardenne"),
            Self::Corsica => write!(f, "Corsica"),
            Self::FrancheComte => write!(f, "Franche-Comté"),
            Self::UpperNormandy => write!(f, "Upper Normandy"),
            Self::LanguedocRoussillon => write!(f, "Languedoc-Roussillon"),
            Self::Limousin => write!(f, "Limousin"),
            Self::Lorraine => write!(f, "Lorraine"),
            Self::MidiPyrenees => write!(f, "Midi-Pyrénées"),
            Self::NordPasDeCalais => write!(f, "Nord-Pas-de-Calais"),
            Self::WesternLoireValley => write!(f, "Western Loire Valley"),
            Self::Picardy => write!(f, "Picardy"),
            Self::PoitouCharentes => write!(f, "Poitou-Charentes"),
            Self::ProvenceAlpesCoteDazur => write!(f, "Provence-Alpes-Côte d'Azur"),
            Self::RhoneAlpes => write!(f, "Rhône-Alpes"),
            Self::Guadeloupe => write!(f, "Guadeloupe"),
            Self::Martinique => write!(f, "Martinique"),
            Self::FrenchGuiana => write!(f, "French Guiana"),
            Self::Reunion => write!(f, "Réunion"),
            Self::Mayotte => write!(f, "Mayotte"),
        }
    }
}

// French Guiana Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrenchGuianaSubregions {
    FrenchGuiana,
    ArrondissementOfCayenne,
    ArrondissementOfSaintLaurentDuMaroni,
}

impl TryFrom<u8> for FrenchGuianaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::FrenchGuiana),
            0x02 => Ok(Self::ArrondissementOfCayenne),
            0x03 => Ok(Self::ArrondissementOfSaintLaurentDuMaroni),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FrenchGuianaSubregions> for u8 {
    fn from(value: FrenchGuianaSubregions) -> u8 {
        match value {
            FrenchGuianaSubregions::FrenchGuiana => 0x01,
            FrenchGuianaSubregions::ArrondissementOfCayenne => 0x02,
            FrenchGuianaSubregions::ArrondissementOfSaintLaurentDuMaroni => 0x03,
        }
    }
}

impl Display for FrenchGuianaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrenchGuiana => write!(f, "French Guiana"),
            Self::ArrondissementOfCayenne => write!(f, "Arrondissement of Cayenne"),
            Self::ArrondissementOfSaintLaurentDuMaroni => {
                write!(f, "Arrondissement of Saint-Laurent-du-Maroni")
            }
        }
    }
}

// French Polynesia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrenchPolynesiaSubregions {
    FrenchPolynesia,
    WindwardIslands,
    AustralIslands,
    LeewardIslands,
    MarquesasIslands,
    TuamotuGambier,
}

impl TryFrom<u8> for FrenchPolynesiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::FrenchPolynesia),
            0x02 => Ok(Self::WindwardIslands),
            0x03 => Ok(Self::AustralIslands),
            0x04 => Ok(Self::LeewardIslands),
            0x05 => Ok(Self::MarquesasIslands),
            0x06 => Ok(Self::TuamotuGambier),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FrenchPolynesiaSubregions> for u8 {
    fn from(value: FrenchPolynesiaSubregions) -> u8 {
        match value {
            FrenchPolynesiaSubregions::FrenchPolynesia => 0x01,
            FrenchPolynesiaSubregions::WindwardIslands => 0x02,
            FrenchPolynesiaSubregions::AustralIslands => 0x03,
            FrenchPolynesiaSubregions::LeewardIslands => 0x04,
            FrenchPolynesiaSubregions::MarquesasIslands => 0x05,
            FrenchPolynesiaSubregions::TuamotuGambier => 0x06,
        }
    }
}

impl Display for FrenchPolynesiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrenchPolynesia => write!(f, "French Polynesia"),
            Self::WindwardIslands => write!(f, "Windward Islands"),
            Self::AustralIslands => write!(f, "Austral Islands"),
            Self::LeewardIslands => write!(f, "Leeward Islands"),
            Self::MarquesasIslands => write!(f, "Marquesas Islands"),
            Self::TuamotuGambier => write!(f, "Tuamotu-Gambier"),
        }
    }
}

// French Southern and Antarctic Lands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrenchSouthernAndAntarcticLandsSubregions {
    FrenchSouthernAndAntarcticLands,
    ScatteredIslandsInTheIndianOcean,
    AdelieLand,
    CrozetIslands,
    IleSaintPaulileAmsterdam,
    KerguelenIslands,
}

impl TryFrom<u8> for FrenchSouthernAndAntarcticLandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::FrenchSouthernAndAntarcticLands),
            0x02 => Ok(Self::ScatteredIslandsInTheIndianOcean),
            0x03 => Ok(Self::AdelieLand),
            0x04 => Ok(Self::CrozetIslands),
            0x05 => Ok(Self::IleSaintPaulileAmsterdam),
            0x06 => Ok(Self::KerguelenIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<FrenchSouthernAndAntarcticLandsSubregions> for u8 {
    fn from(value: FrenchSouthernAndAntarcticLandsSubregions) -> u8 {
        match value {
            FrenchSouthernAndAntarcticLandsSubregions::FrenchSouthernAndAntarcticLands => 0x01,
            FrenchSouthernAndAntarcticLandsSubregions::ScatteredIslandsInTheIndianOcean => 0x02,
            FrenchSouthernAndAntarcticLandsSubregions::AdelieLand => 0x03,
            FrenchSouthernAndAntarcticLandsSubregions::CrozetIslands => 0x04,
            FrenchSouthernAndAntarcticLandsSubregions::IleSaintPaulileAmsterdam => 0x05,
            FrenchSouthernAndAntarcticLandsSubregions::KerguelenIslands => 0x06,
        }
    }
}

impl Display for FrenchSouthernAndAntarcticLandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FrenchSouthernAndAntarcticLands => {
                write!(f, "French Southern and Antarctic Lands")
            }
            Self::ScatteredIslandsInTheIndianOcean => {
                write!(f, "Scattered Islands in the Indian Ocean")
            }
            Self::AdelieLand => write!(f, "Adélie Land"),
            Self::CrozetIslands => write!(f, "Crozet Islands"),
            Self::IleSaintPaulileAmsterdam => write!(f, "Île Saint-Paul/Île Amsterdam"),
            Self::KerguelenIslands => write!(f, "Kerguelen Islands"),
        }
    }
}

// Gabon Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GabonSubregions {
    Gabon,
    Estuaire,
    HautOgooue,
    MoyenOgooue,
    Ngounie,
    Nyanga,
    OgooueIvindo,
    OgooueLolo,
    OgooueMaritime,
    WoleuNtem,
}

impl TryFrom<u8> for GabonSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Gabon),
            0x02 => Ok(Self::Estuaire),
            0x03 => Ok(Self::HautOgooue),
            0x04 => Ok(Self::MoyenOgooue),
            0x05 => Ok(Self::Ngounie),
            0x06 => Ok(Self::Nyanga),
            0x07 => Ok(Self::OgooueIvindo),
            0x08 => Ok(Self::OgooueLolo),
            0x09 => Ok(Self::OgooueMaritime),
            0x0A => Ok(Self::WoleuNtem),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GabonSubregions> for u8 {
    fn from(value: GabonSubregions) -> u8 {
        match value {
            GabonSubregions::Gabon => 0x01,
            GabonSubregions::Estuaire => 0x02,
            GabonSubregions::HautOgooue => 0x03,
            GabonSubregions::MoyenOgooue => 0x04,
            GabonSubregions::Ngounie => 0x05,
            GabonSubregions::Nyanga => 0x06,
            GabonSubregions::OgooueIvindo => 0x07,
            GabonSubregions::OgooueLolo => 0x08,
            GabonSubregions::OgooueMaritime => 0x09,
            GabonSubregions::WoleuNtem => 0x0A,
        }
    }
}

impl Display for GabonSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gabon => write!(f, "Gabon"),
            Self::Estuaire => write!(f, "Estuaire"),
            Self::HautOgooue => write!(f, "Haut-Ogooué"),
            Self::MoyenOgooue => write!(f, "Moyen-Ogooué"),
            Self::Ngounie => write!(f, "Ngounié"),
            Self::Nyanga => write!(f, "Nyanga"),
            Self::OgooueIvindo => write!(f, "Ogooué-Ivindo"),
            Self::OgooueLolo => write!(f, "Ogooué-Lolo"),
            Self::OgooueMaritime => write!(f, "Ogooué-Maritime"),
            Self::WoleuNtem => write!(f, "Woleu-Ntem"),
        }
    }
}

// Georgia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeorgiaSubregions {
    Georgia,
    Tbilisi,
    Abkhazia,
    Adjara,
    Guria,
    Imereti,
    Kakheti,
    KvemoKartli,
    MtskhetaMtianeti,
    RachaLechkhumiAndKvemoSvaneti,
    SamegreloZemoSvaneti,
    SamtskheJavakheti,
    ShidaKartli,
}

impl TryFrom<u8> for GeorgiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Georgia),
            0x02 => Ok(Self::Tbilisi),
            0x03 => Ok(Self::Abkhazia),
            0x04 => Ok(Self::Adjara),
            0x05 => Ok(Self::Guria),
            0x06 => Ok(Self::Imereti),
            0x07 => Ok(Self::Kakheti),
            0x08 => Ok(Self::KvemoKartli),
            0x09 => Ok(Self::MtskhetaMtianeti),
            0x0A => Ok(Self::RachaLechkhumiAndKvemoSvaneti),
            0x0B => Ok(Self::SamegreloZemoSvaneti),
            0x0C => Ok(Self::SamtskheJavakheti),
            0x0D => Ok(Self::ShidaKartli),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GeorgiaSubregions> for u8 {
    fn from(value: GeorgiaSubregions) -> u8 {
        match value {
            GeorgiaSubregions::Georgia => 0x01,
            GeorgiaSubregions::Tbilisi => 0x02,
            GeorgiaSubregions::Abkhazia => 0x03,
            GeorgiaSubregions::Adjara => 0x04,
            GeorgiaSubregions::Guria => 0x05,
            GeorgiaSubregions::Imereti => 0x06,
            GeorgiaSubregions::Kakheti => 0x07,
            GeorgiaSubregions::KvemoKartli => 0x08,
            GeorgiaSubregions::MtskhetaMtianeti => 0x09,
            GeorgiaSubregions::RachaLechkhumiAndKvemoSvaneti => 0x0A,
            GeorgiaSubregions::SamegreloZemoSvaneti => 0x0B,
            GeorgiaSubregions::SamtskheJavakheti => 0x0C,
            GeorgiaSubregions::ShidaKartli => 0x0D,
        }
    }
}

impl Display for GeorgiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Georgia => write!(f, "Georgia"),
            Self::Tbilisi => write!(f, "Tbilisi"),
            Self::Abkhazia => write!(f, "Abkhazia"),
            Self::Adjara => write!(f, "Adjara"),
            Self::Guria => write!(f, "Guria"),
            Self::Imereti => write!(f, "Imereti"),
            Self::Kakheti => write!(f, "Kakheti"),
            Self::KvemoKartli => write!(f, "Kvemo Kartli"),
            Self::MtskhetaMtianeti => write!(f, "Mtskheta-Mtianeti"),
            Self::RachaLechkhumiAndKvemoSvaneti => write!(f, "Racha-Lechkhumi and Kvemo Svaneti"),
            Self::SamegreloZemoSvaneti => write!(f, "Samegrelo-Zemo Svaneti"),
            Self::SamtskheJavakheti => write!(f, "Samtskhe-Javakheti"),
            Self::ShidaKartli => write!(f, "Shida Kartli"),
        }
    }
}

// Germany Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GermanySubregions {
    Germany,
    Berlin,
    Hesse,
    BadenWurttemberg,
    Bavaria,
    Brandenburg,
    Bremen,
    Hamburg,
    MecklenburgWesternPomerania,
    LowerSaxony,
    NorthRhineWestphalia,
    RhinelandPalatinate,
    Saarland,
    Saxony,
    SaxonyAnhalt,
    SchleswigHolstein,
    Thuringia,
}

impl TryFrom<u8> for GermanySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Germany),
            0x02 => Ok(Self::Berlin),
            0x03 => Ok(Self::Hesse),
            0x04 => Ok(Self::BadenWurttemberg),
            0x05 => Ok(Self::Bavaria),
            0x06 => Ok(Self::Brandenburg),
            0x07 => Ok(Self::Bremen),
            0x08 => Ok(Self::Hamburg),
            0x09 => Ok(Self::MecklenburgWesternPomerania),
            0x0A => Ok(Self::LowerSaxony),
            0x0B => Ok(Self::NorthRhineWestphalia),
            0x0C => Ok(Self::RhinelandPalatinate),
            0x0D => Ok(Self::Saarland),
            0x0E => Ok(Self::Saxony),
            0x0F => Ok(Self::SaxonyAnhalt),
            0x10 => Ok(Self::SchleswigHolstein),
            0x11 => Ok(Self::Thuringia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GermanySubregions> for u8 {
    fn from(value: GermanySubregions) -> u8 {
        match value {
            GermanySubregions::Germany => 0x01,
            GermanySubregions::Berlin => 0x02,
            GermanySubregions::Hesse => 0x03,
            GermanySubregions::BadenWurttemberg => 0x04,
            GermanySubregions::Bavaria => 0x05,
            GermanySubregions::Brandenburg => 0x06,
            GermanySubregions::Bremen => 0x07,
            GermanySubregions::Hamburg => 0x08,
            GermanySubregions::MecklenburgWesternPomerania => 0x09,
            GermanySubregions::LowerSaxony => 0x0A,
            GermanySubregions::NorthRhineWestphalia => 0x0B,
            GermanySubregions::RhinelandPalatinate => 0x0C,
            GermanySubregions::Saarland => 0x0D,
            GermanySubregions::Saxony => 0x0E,
            GermanySubregions::SaxonyAnhalt => 0x0F,
            GermanySubregions::SchleswigHolstein => 0x10,
            GermanySubregions::Thuringia => 0x11,
        }
    }
}

impl Display for GermanySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Germany => write!(f, "Germany"),
            Self::Berlin => write!(f, "Berlin"),
            Self::Hesse => write!(f, "Hesse"),
            Self::BadenWurttemberg => write!(f, "Baden-Württemberg"),
            Self::Bavaria => write!(f, "Bavaria"),
            Self::Brandenburg => write!(f, "Brandenburg"),
            Self::Bremen => write!(f, "Bremen"),
            Self::Hamburg => write!(f, "Hamburg"),
            Self::MecklenburgWesternPomerania => write!(f, "Mecklenburg-Western Pomerania"),
            Self::LowerSaxony => write!(f, "Lower Saxony"),
            Self::NorthRhineWestphalia => write!(f, "North Rhine-Westphalia"),
            Self::RhinelandPalatinate => write!(f, "Rhineland-Palatinate"),
            Self::Saarland => write!(f, "Saarland"),
            Self::Saxony => write!(f, "Saxony"),
            Self::SaxonyAnhalt => write!(f, "Saxony-Anhalt"),
            Self::SchleswigHolstein => write!(f, "Schleswig-Holstein"),
            Self::Thuringia => write!(f, "Thuringia"),
        }
    }
}

// Ghana Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GhanaSubregions {
    Ghana,
    GreaterAccra,
    Ashanti,
    BrongAhafo,
    Central,
    Eastern,
    Northern,
    UpperEast,
    UpperWest,
    Volta,
    Western,
}

impl TryFrom<u8> for GhanaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Ghana),
            0x02 => Ok(Self::GreaterAccra),
            0x03 => Ok(Self::Ashanti),
            0x04 => Ok(Self::BrongAhafo),
            0x05 => Ok(Self::Central),
            0x06 => Ok(Self::Eastern),
            0x07 => Ok(Self::Northern),
            0x08 => Ok(Self::UpperEast),
            0x09 => Ok(Self::UpperWest),
            0x0A => Ok(Self::Volta),
            0x0B => Ok(Self::Western),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GhanaSubregions> for u8 {
    fn from(value: GhanaSubregions) -> u8 {
        match value {
            GhanaSubregions::Ghana => 0x01,
            GhanaSubregions::GreaterAccra => 0x02,
            GhanaSubregions::Ashanti => 0x03,
            GhanaSubregions::BrongAhafo => 0x04,
            GhanaSubregions::Central => 0x05,
            GhanaSubregions::Eastern => 0x06,
            GhanaSubregions::Northern => 0x07,
            GhanaSubregions::UpperEast => 0x08,
            GhanaSubregions::UpperWest => 0x09,
            GhanaSubregions::Volta => 0x0A,
            GhanaSubregions::Western => 0x0B,
        }
    }
}

impl Display for GhanaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ghana => write!(f, "Ghana"),
            Self::GreaterAccra => write!(f, "Greater Accra"),
            Self::Ashanti => write!(f, "Ashanti"),
            Self::BrongAhafo => write!(f, "Brong Ahafo"),
            Self::Central => write!(f, "Central"),
            Self::Eastern => write!(f, "Eastern"),
            Self::Northern => write!(f, "Northern"),
            Self::UpperEast => write!(f, "Upper East"),
            Self::UpperWest => write!(f, "Upper West"),
            Self::Volta => write!(f, "Volta"),
            Self::Western => write!(f, "Western"),
        }
    }
}

// Gibraltar Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GibraltarSubregions {
    Gibraltar,
}

impl TryFrom<u8> for GibraltarSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Gibraltar),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GibraltarSubregions> for u8 {
    fn from(value: GibraltarSubregions) -> u8 {
        match value {
            GibraltarSubregions::Gibraltar => 0x01,
        }
    }
}

impl Display for GibraltarSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gibraltar => write!(f, "Gibraltar"),
        }
    }
}

// Greece Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GreeceSubregions {
    Greece,
    Attica,
    CentralGreece,
    CentralMacedonia,
    Crete,
    EastMacedoniaAndThrace,
    Epirus,
    IonianIslands,
    NorthAegean,
    Peloponnese,
    SouthAegean,
    Thessaly,
    WestGreece,
    WestMacedonia,
}

impl TryFrom<u8> for GreeceSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Greece),
            0x02 => Ok(Self::Attica),
            0x03 => Ok(Self::CentralGreece),
            0x04 => Ok(Self::CentralMacedonia),
            0x05 => Ok(Self::Crete),
            0x06 => Ok(Self::EastMacedoniaAndThrace),
            0x07 => Ok(Self::Epirus),
            0x08 => Ok(Self::IonianIslands),
            0x09 => Ok(Self::NorthAegean),
            0x0A => Ok(Self::Peloponnese),
            0x0B => Ok(Self::SouthAegean),
            0x0C => Ok(Self::Thessaly),
            0x0D => Ok(Self::WestGreece),
            0x0E => Ok(Self::WestMacedonia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GreeceSubregions> for u8 {
    fn from(value: GreeceSubregions) -> u8 {
        match value {
            GreeceSubregions::Greece => 0x01,
            GreeceSubregions::Attica => 0x02,
            GreeceSubregions::CentralGreece => 0x03,
            GreeceSubregions::CentralMacedonia => 0x04,
            GreeceSubregions::Crete => 0x05,
            GreeceSubregions::EastMacedoniaAndThrace => 0x06,
            GreeceSubregions::Epirus => 0x07,
            GreeceSubregions::IonianIslands => 0x08,
            GreeceSubregions::NorthAegean => 0x09,
            GreeceSubregions::Peloponnese => 0x0A,
            GreeceSubregions::SouthAegean => 0x0B,
            GreeceSubregions::Thessaly => 0x0C,
            GreeceSubregions::WestGreece => 0x0D,
            GreeceSubregions::WestMacedonia => 0x0E,
        }
    }
}

impl Display for GreeceSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Greece => write!(f, "Greece"),
            Self::Attica => write!(f, "Attica"),
            Self::CentralGreece => write!(f, "Central Greece"),
            Self::CentralMacedonia => write!(f, "Central Macedonia"),
            Self::Crete => write!(f, "Crete"),
            Self::EastMacedoniaAndThrace => write!(f, "East Macedonia and Thrace"),
            Self::Epirus => write!(f, "Epirus"),
            Self::IonianIslands => write!(f, "Ionian Islands"),
            Self::NorthAegean => write!(f, "North Aegean"),
            Self::Peloponnese => write!(f, "Peloponnese"),
            Self::SouthAegean => write!(f, "South Aegean"),
            Self::Thessaly => write!(f, "Thessaly"),
            Self::WestGreece => write!(f, "West Greece"),
            Self::WestMacedonia => write!(f, "West Macedonia"),
        }
    }
}

// Greenland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GreenlandSubregions {
    Greenland,
    Semersooq,
    Avannaata,
    Kujalleq,
    Qeqertalik,
    Qeqqata,
}

impl TryFrom<u8> for GreenlandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Greenland),
            0x02 => Ok(Self::Semersooq),
            0x03 => Ok(Self::Avannaata),
            0x04 => Ok(Self::Kujalleq),
            0x05 => Ok(Self::Qeqertalik),
            0x06 => Ok(Self::Qeqqata),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GreenlandSubregions> for u8 {
    fn from(value: GreenlandSubregions) -> u8 {
        match value {
            GreenlandSubregions::Greenland => 0x01,
            GreenlandSubregions::Semersooq => 0x02,
            GreenlandSubregions::Avannaata => 0x03,
            GreenlandSubregions::Kujalleq => 0x04,
            GreenlandSubregions::Qeqertalik => 0x05,
            GreenlandSubregions::Qeqqata => 0x06,
        }
    }
}

impl Display for GreenlandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Greenland => write!(f, "Greenland"),
            Self::Semersooq => write!(f, "Semersooq"),
            Self::Avannaata => write!(f, "Avannaata"),
            Self::Kujalleq => write!(f, "Kujalleq"),
            Self::Qeqertalik => write!(f, "Qeqertalik"),
            Self::Qeqqata => write!(f, "Qeqqata"),
        }
    }
}

// Grenada Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrenadaSubregions {
    Grenada,
}

impl TryFrom<u8> for GrenadaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Grenada),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GrenadaSubregions> for u8 {
    fn from(value: GrenadaSubregions) -> u8 {
        match value {
            GrenadaSubregions::Grenada => 0x01,
        }
    }
}

impl Display for GrenadaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grenada => write!(f, "Grenada"),
        }
    }
}

// Guadeloupe Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuadeloupeSubregions {
    Guadeloupe,
}

impl TryFrom<u8> for GuadeloupeSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Guadeloupe),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GuadeloupeSubregions> for u8 {
    fn from(value: GuadeloupeSubregions) -> u8 {
        match value {
            GuadeloupeSubregions::Guadeloupe => 0x01,
        }
    }
}

impl Display for GuadeloupeSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Guadeloupe => write!(f, "Guadeloupe"),
        }
    }
}

// Guam Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuamSubregions {
    Guam,
}

impl TryFrom<u8> for GuamSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Guam),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GuamSubregions> for u8 {
    fn from(value: GuamSubregions) -> u8 {
        match value {
            GuamSubregions::Guam => 0x01,
        }
    }
}

impl Display for GuamSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Guam => write!(f, "Guam"),
        }
    }
}

// Guatemala Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuatemalaSubregions {
    Guatemala,
    AltaVerapaz,
    BajaVerapaz,
    Chimaltenango,
    Chiquimula,
    ElProgreso,
    Escuintla,
    Huehuetenango,
    Izabal,
    Jalapa,
    Jutiapa,
    Peten,
    Quetzaltenango,
    Quiche,
    Retalhuleu,
    Sacatepequez,
    SanMarcos,
    SantaRosa,
    Solola,
    Suchitepequez,
    Totonicapan,
    Zacapa,
}

impl TryFrom<u8> for GuatemalaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Guatemala),
            0x02 => Ok(Self::Guatemala),
            0x03 => Ok(Self::AltaVerapaz),
            0x04 => Ok(Self::BajaVerapaz),
            0x05 => Ok(Self::Chimaltenango),
            0x06 => Ok(Self::Chiquimula),
            0x07 => Ok(Self::ElProgreso),
            0x08 => Ok(Self::Escuintla),
            0x09 => Ok(Self::Huehuetenango),
            0x0A => Ok(Self::Izabal),
            0x0B => Ok(Self::Jalapa),
            0x0C => Ok(Self::Jutiapa),
            0x0D => Ok(Self::Peten),
            0x0E => Ok(Self::Quetzaltenango),
            0x0F => Ok(Self::Quiche),
            0x10 => Ok(Self::Retalhuleu),
            0x11 => Ok(Self::Sacatepequez),
            0x12 => Ok(Self::SanMarcos),
            0x13 => Ok(Self::SantaRosa),
            0x14 => Ok(Self::Solola),
            0x15 => Ok(Self::Suchitepequez),
            0x16 => Ok(Self::Totonicapan),
            0x17 => Ok(Self::Zacapa),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GuatemalaSubregions> for u8 {
    fn from(value: GuatemalaSubregions) -> u8 {
        match value {
            GuatemalaSubregions::Guatemala => 0x01,
            GuatemalaSubregions::AltaVerapaz => 0x03,
            GuatemalaSubregions::BajaVerapaz => 0x04,
            GuatemalaSubregions::Chimaltenango => 0x05,
            GuatemalaSubregions::Chiquimula => 0x06,
            GuatemalaSubregions::ElProgreso => 0x07,
            GuatemalaSubregions::Escuintla => 0x08,
            GuatemalaSubregions::Huehuetenango => 0x09,
            GuatemalaSubregions::Izabal => 0x0A,
            GuatemalaSubregions::Jalapa => 0x0B,
            GuatemalaSubregions::Jutiapa => 0x0C,
            GuatemalaSubregions::Peten => 0x0D,
            GuatemalaSubregions::Quetzaltenango => 0x0E,
            GuatemalaSubregions::Quiche => 0x0F,
            GuatemalaSubregions::Retalhuleu => 0x10,
            GuatemalaSubregions::Sacatepequez => 0x11,
            GuatemalaSubregions::SanMarcos => 0x12,
            GuatemalaSubregions::SantaRosa => 0x13,
            GuatemalaSubregions::Solola => 0x14,
            GuatemalaSubregions::Suchitepequez => 0x15,
            GuatemalaSubregions::Totonicapan => 0x16,
            GuatemalaSubregions::Zacapa => 0x17,
        }
    }
}

impl Display for GuatemalaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Guatemala => write!(f, "Guatemala"),
            Self::AltaVerapaz => write!(f, "Alta Verapaz"),
            Self::BajaVerapaz => write!(f, "Baja Verapaz"),
            Self::Chimaltenango => write!(f, "Chimaltenango"),
            Self::Chiquimula => write!(f, "Chiquimula"),
            Self::ElProgreso => write!(f, "El Progreso"),
            Self::Escuintla => write!(f, "Escuintla"),
            Self::Huehuetenango => write!(f, "Huehuetenango"),
            Self::Izabal => write!(f, "Izabal"),
            Self::Jalapa => write!(f, "Jalapa"),
            Self::Jutiapa => write!(f, "Jutiapa"),
            Self::Peten => write!(f, "Petén"),
            Self::Quetzaltenango => write!(f, "Quetzaltenango"),
            Self::Quiche => write!(f, "Quiché"),
            Self::Retalhuleu => write!(f, "Retalhuleu"),
            Self::Sacatepequez => write!(f, "Sacatepéquez"),
            Self::SanMarcos => write!(f, "San Marcos"),
            Self::SantaRosa => write!(f, "Santa Rosa"),
            Self::Solola => write!(f, "Sololá"),
            Self::Suchitepequez => write!(f, "Suchitepéquez"),
            Self::Totonicapan => write!(f, "Totonicapán"),
            Self::Zacapa => write!(f, "Zacapa"),
        }
    }
}

// Guernsey Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuernseySubregions {
    Guernsey,
}

impl TryFrom<u8> for GuernseySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Guernsey),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GuernseySubregions> for u8 {
    fn from(value: GuernseySubregions) -> u8 {
        match value {
            GuernseySubregions::Guernsey => 0x01,
        }
    }
}

impl Display for GuernseySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Guernsey => write!(f, "Guernsey"),
        }
    }
}

// Guinea Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuineaSubregions {
    Guinea,
    Conakry,
    Boke,
    Faranah,
    Kankan,
    Kindia,
    Labe,
    Mamou,
    Nzerekore,
}

impl TryFrom<u8> for GuineaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Guinea),
            0x02 => Ok(Self::Conakry),
            0x03 => Ok(Self::Boke),
            0x04 => Ok(Self::Faranah),
            0x05 => Ok(Self::Kankan),
            0x06 => Ok(Self::Kindia),
            0x07 => Ok(Self::Labe),
            0x08 => Ok(Self::Mamou),
            0x09 => Ok(Self::Nzerekore),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GuineaSubregions> for u8 {
    fn from(value: GuineaSubregions) -> u8 {
        match value {
            GuineaSubregions::Guinea => 0x01,
            GuineaSubregions::Conakry => 0x02,
            GuineaSubregions::Boke => 0x03,
            GuineaSubregions::Faranah => 0x04,
            GuineaSubregions::Kankan => 0x05,
            GuineaSubregions::Kindia => 0x06,
            GuineaSubregions::Labe => 0x07,
            GuineaSubregions::Mamou => 0x08,
            GuineaSubregions::Nzerekore => 0x09,
        }
    }
}

impl Display for GuineaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Guinea => write!(f, "Guinea"),
            Self::Conakry => write!(f, "Conakry"),
            Self::Boke => write!(f, "Boké"),
            Self::Faranah => write!(f, "Faranah"),
            Self::Kankan => write!(f, "Kankan"),
            Self::Kindia => write!(f, "Kindia"),
            Self::Labe => write!(f, "Labé"),
            Self::Mamou => write!(f, "Mamou"),
            Self::Nzerekore => write!(f, "Nzérékoré"),
        }
    }
}

// Guinea-Bissau Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuineaBissauSubregions {
    GuineaBissau,
    Bissau,
    East,
    North,
    South,
}

impl TryFrom<u8> for GuineaBissauSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::GuineaBissau),
            0x02 => Ok(Self::Bissau),
            0x03 => Ok(Self::East),
            0x04 => Ok(Self::North),
            0x05 => Ok(Self::South),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GuineaBissauSubregions> for u8 {
    fn from(value: GuineaBissauSubregions) -> u8 {
        match value {
            GuineaBissauSubregions::GuineaBissau => 0x01,
            GuineaBissauSubregions::Bissau => 0x02,
            GuineaBissauSubregions::East => 0x03,
            GuineaBissauSubregions::North => 0x04,
            GuineaBissauSubregions::South => 0x05,
        }
    }
}

impl Display for GuineaBissauSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GuineaBissau => write!(f, "Guinea-Bissau"),
            Self::Bissau => write!(f, "Bissau"),
            Self::East => write!(f, "East"),
            Self::North => write!(f, "North"),
            Self::South => write!(f, "South"),
        }
    }
}

// Guyana Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuyanaSubregions {
    Guyana,
    DemeraraMahaica,
    BarimaWaini,
    CuyuniMazaruni,
    EastBerbiceCorentyne,
    EssequiboIslandsWestDemerara,
    MahaicaBerbice,
    PomeroonSupenaam,
    PotaroSiparuni,
    UpperDemeraraBerbice,
    UpperTakutuUpperEssequibo,
}

impl TryFrom<u8> for GuyanaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Guyana),
            0x02 => Ok(Self::DemeraraMahaica),
            0x03 => Ok(Self::BarimaWaini),
            0x04 => Ok(Self::CuyuniMazaruni),
            0x05 => Ok(Self::EastBerbiceCorentyne),
            0x06 => Ok(Self::EssequiboIslandsWestDemerara),
            0x07 => Ok(Self::MahaicaBerbice),
            0x08 => Ok(Self::PomeroonSupenaam),
            0x09 => Ok(Self::PotaroSiparuni),
            0x0A => Ok(Self::UpperDemeraraBerbice),
            0x0B => Ok(Self::UpperTakutuUpperEssequibo),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<GuyanaSubregions> for u8 {
    fn from(value: GuyanaSubregions) -> u8 {
        match value {
            GuyanaSubregions::Guyana => 0x01,
            GuyanaSubregions::DemeraraMahaica => 0x02,
            GuyanaSubregions::BarimaWaini => 0x03,
            GuyanaSubregions::CuyuniMazaruni => 0x04,
            GuyanaSubregions::EastBerbiceCorentyne => 0x05,
            GuyanaSubregions::EssequiboIslandsWestDemerara => 0x06,
            GuyanaSubregions::MahaicaBerbice => 0x07,
            GuyanaSubregions::PomeroonSupenaam => 0x08,
            GuyanaSubregions::PotaroSiparuni => 0x09,
            GuyanaSubregions::UpperDemeraraBerbice => 0x0A,
            GuyanaSubregions::UpperTakutuUpperEssequibo => 0x0B,
        }
    }
}

impl Display for GuyanaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Guyana => write!(f, "Guyana"),
            Self::DemeraraMahaica => write!(f, "Demerara-Mahaica"),
            Self::BarimaWaini => write!(f, "Barima-Waini"),
            Self::CuyuniMazaruni => write!(f, "Cuyuni-Mazaruni"),
            Self::EastBerbiceCorentyne => write!(f, "East Berbice-Corentyne"),
            Self::EssequiboIslandsWestDemerara => write!(f, "Essequibo Islands-West Demerara"),
            Self::MahaicaBerbice => write!(f, "Mahaica-Berbice"),
            Self::PomeroonSupenaam => write!(f, "Pomeroon-Supenaam"),
            Self::PotaroSiparuni => write!(f, "Potaro-Siparuni"),
            Self::UpperDemeraraBerbice => write!(f, "Upper Demerara-Berbice"),
            Self::UpperTakutuUpperEssequibo => write!(f, "Upper Takutu-Upper Essequibo"),
        }
    }
}

// Haiti Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HaitiSubregions {
    Haiti,
    Ouest,
    NordOuest,
    Artibonite,
    Centre,
    Grandanse,
    Nord,
    NordEst,
    Sud,
    SudEst,
}

impl TryFrom<u8> for HaitiSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Haiti),
            0x02 => Ok(Self::Ouest),
            0x03 => Ok(Self::NordOuest),
            0x04 => Ok(Self::Artibonite),
            0x05 => Ok(Self::Centre),
            0x06 => Ok(Self::Grandanse),
            0x07 => Ok(Self::Nord),
            0x08 => Ok(Self::NordEst),
            0x09 => Ok(Self::Sud),
            0x0A => Ok(Self::SudEst),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<HaitiSubregions> for u8 {
    fn from(value: HaitiSubregions) -> u8 {
        match value {
            HaitiSubregions::Haiti => 0x01,
            HaitiSubregions::Ouest => 0x02,
            HaitiSubregions::NordOuest => 0x03,
            HaitiSubregions::Artibonite => 0x04,
            HaitiSubregions::Centre => 0x05,
            HaitiSubregions::Grandanse => 0x06,
            HaitiSubregions::Nord => 0x07,
            HaitiSubregions::NordEst => 0x08,
            HaitiSubregions::Sud => 0x09,
            HaitiSubregions::SudEst => 0x0A,
        }
    }
}

impl Display for HaitiSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Haiti => write!(f, "Haiti"),
            Self::Ouest => write!(f, "Ouest"),
            Self::NordOuest => write!(f, "Nord-Ouest"),
            Self::Artibonite => write!(f, "Artibonite"),
            Self::Centre => write!(f, "Centre"),
            Self::Grandanse => write!(f, "Grand'Anse"),
            Self::Nord => write!(f, "Nord"),
            Self::NordEst => write!(f, "Nord-Est"),
            Self::Sud => write!(f, "Sud"),
            Self::SudEst => write!(f, "Sud-Est"),
        }
    }
}

// Honduras Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HondurasSubregions {
    Honduras,
    FranciscoMorazan,
    Atlantida,
    Choluteca,
    Colon,
    Comayagua,
    Copan,
    Cortes,
    ElParaiso,
    GraciasADios,
    Intibuca,
    IslasDeLaBahia,
    LaPaz,
    Lempira,
    Ocotepeque,
    Olancho,
    SantaBarbara,
    Valle,
    Yoro,
}

impl TryFrom<u8> for HondurasSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Honduras),
            0x02 => Ok(Self::FranciscoMorazan),
            0x03 => Ok(Self::Atlantida),
            0x04 => Ok(Self::Choluteca),
            0x05 => Ok(Self::Colon),
            0x06 => Ok(Self::Comayagua),
            0x07 => Ok(Self::Copan),
            0x08 => Ok(Self::Cortes),
            0x09 => Ok(Self::ElParaiso),
            0x0A => Ok(Self::GraciasADios),
            0x0B => Ok(Self::Intibuca),
            0x0C => Ok(Self::IslasDeLaBahia),
            0x0D => Ok(Self::LaPaz),
            0x0E => Ok(Self::Lempira),
            0x0F => Ok(Self::Ocotepeque),
            0x10 => Ok(Self::Olancho),
            0x11 => Ok(Self::SantaBarbara),
            0x12 => Ok(Self::Valle),
            0x13 => Ok(Self::Yoro),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<HondurasSubregions> for u8 {
    fn from(value: HondurasSubregions) -> u8 {
        match value {
            HondurasSubregions::Honduras => 0x01,
            HondurasSubregions::FranciscoMorazan => 0x02,
            HondurasSubregions::Atlantida => 0x03,
            HondurasSubregions::Choluteca => 0x04,
            HondurasSubregions::Colon => 0x05,
            HondurasSubregions::Comayagua => 0x06,
            HondurasSubregions::Copan => 0x07,
            HondurasSubregions::Cortes => 0x08,
            HondurasSubregions::ElParaiso => 0x09,
            HondurasSubregions::GraciasADios => 0x0A,
            HondurasSubregions::Intibuca => 0x0B,
            HondurasSubregions::IslasDeLaBahia => 0x0C,
            HondurasSubregions::LaPaz => 0x0D,
            HondurasSubregions::Lempira => 0x0E,
            HondurasSubregions::Ocotepeque => 0x0F,
            HondurasSubregions::Olancho => 0x10,
            HondurasSubregions::SantaBarbara => 0x11,
            HondurasSubregions::Valle => 0x12,
            HondurasSubregions::Yoro => 0x13,
        }
    }
}

impl Display for HondurasSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Honduras => write!(f, "Honduras"),
            Self::FranciscoMorazan => write!(f, "Francisco Morazán"),
            Self::Atlantida => write!(f, "Atlántida"),
            Self::Choluteca => write!(f, "Choluteca"),
            Self::Colon => write!(f, "Colón"),
            Self::Comayagua => write!(f, "Comayagua"),
            Self::Copan => write!(f, "Copán"),
            Self::Cortes => write!(f, "Cortés"),
            Self::ElParaiso => write!(f, "El Paraíso"),
            Self::GraciasADios => write!(f, "Gracias a Dios"),
            Self::Intibuca => write!(f, "Intibucá"),
            Self::IslasDeLaBahia => write!(f, "Islas de la Bahía"),
            Self::LaPaz => write!(f, "La Paz"),
            Self::Lempira => write!(f, "Lempira"),
            Self::Ocotepeque => write!(f, "Ocotepeque"),
            Self::Olancho => write!(f, "Olancho"),
            Self::SantaBarbara => write!(f, "Santa Bárbara"),
            Self::Valle => write!(f, "Valle"),
            Self::Yoro => write!(f, "Yoro"),
        }
    }
}

// Hong Kong Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HongKongSubregions {
    HongKong,
}

impl TryFrom<u8> for HongKongSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::HongKong),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<HongKongSubregions> for u8 {
    fn from(value: HongKongSubregions) -> u8 {
        match value {
            HongKongSubregions::HongKong => 0x01,
        }
    }
}

impl Display for HongKongSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HongKong => write!(f, "Hong Kong"),
        }
    }
}

// Hungary Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HungarySubregions {
    Hungary,
    Budapest,
    BacsKiskun,
    Baranya,
    Bekes,
    BorsodAbaujZemplen,
    Csongrad,
    Fejer,
    GyorMosonSopron,
    HajduBihar,
    Heves,
    JaszNagykunSzolnok,
    KomaromEsztergom,
    Nograd,
    Pest,
    Somogy,
    SzabolcsSzatmarBereg,
    Tolna,
    Vas,
    Veszprem,
    Zala,
}

impl TryFrom<u8> for HungarySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Hungary),
            0x02 => Ok(Self::Budapest),
            0x03 => Ok(Self::BacsKiskun),
            0x04 => Ok(Self::Baranya),
            0x05 => Ok(Self::Bekes),
            0x06 => Ok(Self::BorsodAbaujZemplen),
            0x07 => Ok(Self::Csongrad),
            0x08 => Ok(Self::Fejer),
            0x09 => Ok(Self::GyorMosonSopron),
            0x0A => Ok(Self::HajduBihar),
            0x0B => Ok(Self::Heves),
            0x0C => Ok(Self::JaszNagykunSzolnok),
            0x0D => Ok(Self::KomaromEsztergom),
            0x0E => Ok(Self::Nograd),
            0x0F => Ok(Self::Pest),
            0x10 => Ok(Self::Somogy),
            0x11 => Ok(Self::SzabolcsSzatmarBereg),
            0x12 => Ok(Self::Tolna),
            0x13 => Ok(Self::Vas),
            0x14 => Ok(Self::Veszprem),
            0x15 => Ok(Self::Zala),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<HungarySubregions> for u8 {
    fn from(value: HungarySubregions) -> u8 {
        match value {
            HungarySubregions::Hungary => 0x01,
            HungarySubregions::Budapest => 0x02,
            HungarySubregions::BacsKiskun => 0x03,
            HungarySubregions::Baranya => 0x04,
            HungarySubregions::Bekes => 0x05,
            HungarySubregions::BorsodAbaujZemplen => 0x06,
            HungarySubregions::Csongrad => 0x07,
            HungarySubregions::Fejer => 0x08,
            HungarySubregions::GyorMosonSopron => 0x09,
            HungarySubregions::HajduBihar => 0x0A,
            HungarySubregions::Heves => 0x0B,
            HungarySubregions::JaszNagykunSzolnok => 0x0C,
            HungarySubregions::KomaromEsztergom => 0x0D,
            HungarySubregions::Nograd => 0x0E,
            HungarySubregions::Pest => 0x0F,
            HungarySubregions::Somogy => 0x10,
            HungarySubregions::SzabolcsSzatmarBereg => 0x11,
            HungarySubregions::Tolna => 0x12,
            HungarySubregions::Vas => 0x13,
            HungarySubregions::Veszprem => 0x14,
            HungarySubregions::Zala => 0x15,
        }
    }
}

impl Display for HungarySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hungary => write!(f, "Hungary"),
            Self::Budapest => write!(f, "Budapest"),
            Self::BacsKiskun => write!(f, "Bács-Kiskun"),
            Self::Baranya => write!(f, "Baranya"),
            Self::Bekes => write!(f, "Békés"),
            Self::BorsodAbaujZemplen => write!(f, "Borsod-Abaúj-Zemplén"),
            Self::Csongrad => write!(f, "Csongrád"),
            Self::Fejer => write!(f, "Fejér"),
            Self::GyorMosonSopron => write!(f, "Gyor-Moson-Sopron"),
            Self::HajduBihar => write!(f, "Hajdú-Bihar"),
            Self::Heves => write!(f, "Heves"),
            Self::JaszNagykunSzolnok => write!(f, "Jász-Nagykun-Szolnok"),
            Self::KomaromEsztergom => write!(f, "Komárom-Esztergom"),
            Self::Nograd => write!(f, "Nógrád"),
            Self::Pest => write!(f, "Pest"),
            Self::Somogy => write!(f, "Somogy"),
            Self::SzabolcsSzatmarBereg => write!(f, "Szabolcs-Szatmár-Bereg"),
            Self::Tolna => write!(f, "Tolna"),
            Self::Vas => write!(f, "Vas"),
            Self::Veszprem => write!(f, "Veszprém"),
            Self::Zala => write!(f, "Zala"),
        }
    }
}

// Iceland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IcelandSubregions {
    Iceland,
    CapitalRegion,
    Eastland,
    NorthlandEast,
    NorthlandWest,
    Southland,
    SouthernPeninsula,
    WesternFjords,
    Westland,
}

impl TryFrom<u8> for IcelandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Iceland),
            0x02 => Ok(Self::CapitalRegion),
            0x03 => Ok(Self::Eastland),
            0x04 => Ok(Self::NorthlandEast),
            0x05 => Ok(Self::NorthlandWest),
            0x06 => Ok(Self::Southland),
            0x07 => Ok(Self::SouthernPeninsula),
            0x08 => Ok(Self::WesternFjords),
            0x09 => Ok(Self::Westland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IcelandSubregions> for u8 {
    fn from(value: IcelandSubregions) -> u8 {
        match value {
            IcelandSubregions::Iceland => 0x01,
            IcelandSubregions::CapitalRegion => 0x02,
            IcelandSubregions::Eastland => 0x03,
            IcelandSubregions::NorthlandEast => 0x04,
            IcelandSubregions::NorthlandWest => 0x05,
            IcelandSubregions::Southland => 0x06,
            IcelandSubregions::SouthernPeninsula => 0x07,
            IcelandSubregions::WesternFjords => 0x08,
            IcelandSubregions::Westland => 0x09,
        }
    }
}

impl Display for IcelandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Iceland => write!(f, "Iceland"),
            Self::CapitalRegion => write!(f, "Capital Region"),
            Self::Eastland => write!(f, "Eastland"),
            Self::NorthlandEast => write!(f, "Northland East"),
            Self::NorthlandWest => write!(f, "Northland West"),
            Self::Southland => write!(f, "Southland"),
            Self::SouthernPeninsula => write!(f, "Southern Peninsula"),
            Self::WesternFjords => write!(f, "Western Fjords"),
            Self::Westland => write!(f, "Westland"),
        }
    }
}

// India Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndiaSubregions {
    India,
    Delhi,
    AndamanAndNicobarIslands,
    AndhraPradesh,
    Assam,
    Chandigarh,
    DadraAndNagarHaveli,
    Gujarat,
    Haryana,
    HimachalPradesh,
    Kerala,
    Lakshadweep,
    Maharashtra,
    Manipur,
    Meghalaya,
    Karnataka,
    Nagaland,
    Odisha,
    Puducherry,
    Punjab,
    Rajasthan,
    TamilNadu,
    Tripura,
    WestBengal,
    Sikkim,
    Mizoram,
    DamanAndDiu,
    Goa,
    Bihar,
    MadhyaPradesh,
    UttarPradesh,
    Chhattisgarh,
    Jharkhand,
    Uttarakhand,
    Other,
}

impl TryFrom<u8> for IndiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::India),
            0x02 => Ok(Self::Delhi),
            0x03 => Ok(Self::AndamanAndNicobarIslands),
            0x04 => Ok(Self::AndhraPradesh),
            0x05 => Ok(Self::Assam),
            0x06 => Ok(Self::Chandigarh),
            0x07 => Ok(Self::DadraAndNagarHaveli),
            0x08 => Ok(Self::Gujarat),
            0x09 => Ok(Self::Haryana),
            0x0A => Ok(Self::HimachalPradesh),
            0x0C => Ok(Self::Kerala),
            0x0D => Ok(Self::Lakshadweep),
            0x0E => Ok(Self::Maharashtra),
            0x0F => Ok(Self::Manipur),
            0x10 => Ok(Self::Meghalaya),
            0x11 => Ok(Self::Karnataka),
            0x12 => Ok(Self::Nagaland),
            0x13 => Ok(Self::Odisha),
            0x14 => Ok(Self::Puducherry),
            0x15 => Ok(Self::Punjab),
            0x16 => Ok(Self::Rajasthan),
            0x17 => Ok(Self::TamilNadu),
            0x18 => Ok(Self::Tripura),
            0x19 => Ok(Self::WestBengal),
            0x1A => Ok(Self::Sikkim),
            0x1C => Ok(Self::Mizoram),
            0x1D => Ok(Self::DamanAndDiu),
            0x1E => Ok(Self::Goa),
            0x1F => Ok(Self::Bihar),
            0x20 => Ok(Self::MadhyaPradesh),
            0x21 => Ok(Self::UttarPradesh),
            0x22 => Ok(Self::Chhattisgarh),
            0x23 => Ok(Self::Jharkhand),
            0x24 => Ok(Self::Uttarakhand),
            0x25 => Ok(Self::Other),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IndiaSubregions> for u8 {
    fn from(value: IndiaSubregions) -> u8 {
        match value {
            IndiaSubregions::India => 0x01,
            IndiaSubregions::Delhi => 0x02,
            IndiaSubregions::AndamanAndNicobarIslands => 0x03,
            IndiaSubregions::AndhraPradesh => 0x04,
            IndiaSubregions::Assam => 0x05,
            IndiaSubregions::Chandigarh => 0x06,
            IndiaSubregions::DadraAndNagarHaveli => 0x07,
            IndiaSubregions::Gujarat => 0x08,
            IndiaSubregions::Haryana => 0x09,
            IndiaSubregions::HimachalPradesh => 0x0A,
            IndiaSubregions::Kerala => 0x0C,
            IndiaSubregions::Lakshadweep => 0x0D,
            IndiaSubregions::Maharashtra => 0x0E,
            IndiaSubregions::Manipur => 0x0F,
            IndiaSubregions::Meghalaya => 0x10,
            IndiaSubregions::Karnataka => 0x11,
            IndiaSubregions::Nagaland => 0x12,
            IndiaSubregions::Odisha => 0x13,
            IndiaSubregions::Puducherry => 0x14,
            IndiaSubregions::Punjab => 0x15,
            IndiaSubregions::Rajasthan => 0x16,
            IndiaSubregions::TamilNadu => 0x17,
            IndiaSubregions::Tripura => 0x18,
            IndiaSubregions::WestBengal => 0x19,
            IndiaSubregions::Sikkim => 0x1A,
            IndiaSubregions::Mizoram => 0x1C,
            IndiaSubregions::DamanAndDiu => 0x1D,
            IndiaSubregions::Goa => 0x1E,
            IndiaSubregions::Bihar => 0x1F,
            IndiaSubregions::MadhyaPradesh => 0x20,
            IndiaSubregions::UttarPradesh => 0x21,
            IndiaSubregions::Chhattisgarh => 0x22,
            IndiaSubregions::Jharkhand => 0x23,
            IndiaSubregions::Uttarakhand => 0x24,
            IndiaSubregions::Other => 0x25,
        }
    }
}

impl Display for IndiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::India => write!(f, "India"),
            Self::Delhi => write!(f, "Delhi"),
            Self::AndamanAndNicobarIslands => write!(f, "Andaman and Nicobar Islands"),
            Self::AndhraPradesh => write!(f, "Andhra Pradesh"),
            Self::Assam => write!(f, "Assam"),
            Self::Chandigarh => write!(f, "Chandigarh"),
            Self::DadraAndNagarHaveli => write!(f, "Dadra and Nagar Haveli"),
            Self::Gujarat => write!(f, "Gujarat"),
            Self::Haryana => write!(f, "Haryana"),
            Self::HimachalPradesh => write!(f, "Himachal Pradesh"),
            Self::Kerala => write!(f, "Kerala"),
            Self::Lakshadweep => write!(f, "Lakshadweep"),
            Self::Maharashtra => write!(f, "Maharashtra"),
            Self::Manipur => write!(f, "Manipur"),
            Self::Meghalaya => write!(f, "Meghalaya"),
            Self::Karnataka => write!(f, "Karnataka"),
            Self::Nagaland => write!(f, "Nagaland"),
            Self::Odisha => write!(f, "Odisha"),
            Self::Puducherry => write!(f, "Puducherry"),
            Self::Punjab => write!(f, "Punjab"),
            Self::Rajasthan => write!(f, "Rajasthan"),
            Self::TamilNadu => write!(f, "Tamil Nadu"),
            Self::Tripura => write!(f, "Tripura"),
            Self::WestBengal => write!(f, "West Bengal"),
            Self::Sikkim => write!(f, "Sikkim"),
            Self::Mizoram => write!(f, "Mizoram"),
            Self::DamanAndDiu => write!(f, "Daman and Diu"),
            Self::Goa => write!(f, "Goa"),
            Self::Bihar => write!(f, "Bihar"),
            Self::MadhyaPradesh => write!(f, "Madhya Pradesh"),
            Self::UttarPradesh => write!(f, "Uttar Pradesh"),
            Self::Chhattisgarh => write!(f, "Chhattisgarh"),
            Self::Jharkhand => write!(f, "Jharkhand"),
            Self::Uttarakhand => write!(f, "Uttarakhand"),
            Self::Other => write!(f, "Other"),
        }
    }
}

// Indonesia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndonesiaSubregions {
    Indonesia,
    JakartaRaya,
    Aceh,
    Bali,
    Bengkulu,
    Jambi,
    CentralJava,
    EastJava,
    Papua,
    Yogyakarta,
    WestKalimantan,
    SouthKalimantan,
    KalimantanTengah,
    KalimantanTimur,
    Lampung,
    NusaTenggaraBarat,
    EastNusaTenggara,
    Riau,
    SulawesiSelatan,
    SulawesiTengah,
    SulawesiTenggara,
    WestSumatra,
    NorthSumatra,
    Maluku,
    MalukuUtara,
    JawaBarat,
    SulawesiUtara,
    SumateraSelatan,
    Banten,
    Gorontalo,
    KepulauanBangkaBelitung,
    IrianJayaBarat,
    KepulauanRiau,
    SulawesiBarat,
}

impl TryFrom<u8> for IndonesiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Indonesia),
            0x02 => Ok(Self::JakartaRaya),
            0x03 => Ok(Self::Aceh),
            0x04 => Ok(Self::Bali),
            0x05 => Ok(Self::Bengkulu),
            0x06 => Ok(Self::Jambi),
            0x07 => Ok(Self::CentralJava),
            0x08 => Ok(Self::EastJava),
            0x09 => Ok(Self::Papua),
            0x0A => Ok(Self::Yogyakarta),
            0x0B => Ok(Self::WestKalimantan),
            0x0C => Ok(Self::SouthKalimantan),
            0x0D => Ok(Self::KalimantanTengah),
            0x0E => Ok(Self::KalimantanTimur),
            0x0F => Ok(Self::Lampung),
            0x10 => Ok(Self::NusaTenggaraBarat),
            0x11 => Ok(Self::EastNusaTenggara),
            0x12 => Ok(Self::Riau),
            0x13 => Ok(Self::SulawesiSelatan),
            0x14 => Ok(Self::SulawesiTengah),
            0x15 => Ok(Self::SulawesiTenggara),
            0x16 => Ok(Self::WestSumatra),
            0x17 => Ok(Self::NorthSumatra),
            0x18 => Ok(Self::Maluku),
            0x19 => Ok(Self::MalukuUtara),
            0x1A => Ok(Self::JawaBarat),
            0x1B => Ok(Self::SulawesiUtara),
            0x1C => Ok(Self::SumateraSelatan),
            0x1D => Ok(Self::Banten),
            0x1E => Ok(Self::Gorontalo),
            0x1F => Ok(Self::KepulauanBangkaBelitung),
            0x20 => Ok(Self::IrianJayaBarat),
            0x21 => Ok(Self::KepulauanRiau),
            0x22 => Ok(Self::SulawesiBarat),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IndonesiaSubregions> for u8 {
    fn from(value: IndonesiaSubregions) -> u8 {
        match value {
            IndonesiaSubregions::Indonesia => 0x01,
            IndonesiaSubregions::JakartaRaya => 0x02,
            IndonesiaSubregions::Aceh => 0x03,
            IndonesiaSubregions::Bali => 0x04,
            IndonesiaSubregions::Bengkulu => 0x05,
            IndonesiaSubregions::Jambi => 0x06,
            IndonesiaSubregions::CentralJava => 0x07,
            IndonesiaSubregions::EastJava => 0x08,
            IndonesiaSubregions::Papua => 0x09,
            IndonesiaSubregions::Yogyakarta => 0x0A,
            IndonesiaSubregions::WestKalimantan => 0x0B,
            IndonesiaSubregions::SouthKalimantan => 0x0C,
            IndonesiaSubregions::KalimantanTengah => 0x0D,
            IndonesiaSubregions::KalimantanTimur => 0x0E,
            IndonesiaSubregions::Lampung => 0x0F,
            IndonesiaSubregions::NusaTenggaraBarat => 0x10,
            IndonesiaSubregions::EastNusaTenggara => 0x11,
            IndonesiaSubregions::Riau => 0x12,
            IndonesiaSubregions::SulawesiSelatan => 0x13,
            IndonesiaSubregions::SulawesiTengah => 0x14,
            IndonesiaSubregions::SulawesiTenggara => 0x15,
            IndonesiaSubregions::WestSumatra => 0x16,
            IndonesiaSubregions::NorthSumatra => 0x17,
            IndonesiaSubregions::Maluku => 0x18,
            IndonesiaSubregions::MalukuUtara => 0x19,
            IndonesiaSubregions::JawaBarat => 0x1A,
            IndonesiaSubregions::SulawesiUtara => 0x1B,
            IndonesiaSubregions::SumateraSelatan => 0x1C,
            IndonesiaSubregions::Banten => 0x1D,
            IndonesiaSubregions::Gorontalo => 0x1E,
            IndonesiaSubregions::KepulauanBangkaBelitung => 0x1F,
            IndonesiaSubregions::IrianJayaBarat => 0x20,
            IndonesiaSubregions::KepulauanRiau => 0x21,
            IndonesiaSubregions::SulawesiBarat => 0x22,
        }
    }
}

impl Display for IndonesiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Indonesia => write!(f, "Indonesia"),
            Self::JakartaRaya => write!(f, "Jakarta Raya"),
            Self::Aceh => write!(f, "Aceh"),
            Self::Bali => write!(f, "Bali"),
            Self::Bengkulu => write!(f, "Bengkulu"),
            Self::Jambi => write!(f, "Jambi"),
            Self::CentralJava => write!(f, "Central Java"),
            Self::EastJava => write!(f, "East Java"),
            Self::Papua => write!(f, "Papua"),
            Self::Yogyakarta => write!(f, "Yogyakarta"),
            Self::WestKalimantan => write!(f, "West Kalimantan"),
            Self::SouthKalimantan => write!(f, "South Kalimantan"),
            Self::KalimantanTengah => write!(f, "Kalimantan Tengah"),
            Self::KalimantanTimur => write!(f, "Kalimantan Timur"),
            Self::Lampung => write!(f, "Lampung"),
            Self::NusaTenggaraBarat => write!(f, "Nusa Tenggara Barat"),
            Self::EastNusaTenggara => write!(f, "East Nusa Tenggara"),
            Self::Riau => write!(f, "Riau"),
            Self::SulawesiSelatan => write!(f, "Sulawesi Selatan"),
            Self::SulawesiTengah => write!(f, "Sulawesi Tengah"),
            Self::SulawesiTenggara => write!(f, "Sulawesi Tenggara"),
            Self::WestSumatra => write!(f, "West Sumatra"),
            Self::NorthSumatra => write!(f, "North Sumatra"),
            Self::Maluku => write!(f, "Maluku"),
            Self::MalukuUtara => write!(f, "Maluku Utara"),
            Self::JawaBarat => write!(f, "Jawa Barat"),
            Self::SulawesiUtara => write!(f, "Sulawesi Utara"),
            Self::SumateraSelatan => write!(f, "Sumatera Selatan"),
            Self::Banten => write!(f, "Banten"),
            Self::Gorontalo => write!(f, "Gorontalo"),
            Self::KepulauanBangkaBelitung => write!(f, "Kepulauan Bangka Belitung"),
            Self::IrianJayaBarat => write!(f, "Irian Jaya Barat"),
            Self::KepulauanRiau => write!(f, "Kepulauan Riau"),
            Self::SulawesiBarat => write!(f, "Sulawesi Barat"),
        }
    }
}

// Iran Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IranSubregions {
    Iran,
    Region1,
    Region2,
    Region3,
    Region4,
    Region5,
}

impl TryFrom<u8> for IranSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Iran),
            0x02 => Ok(Self::Region1),
            0x03 => Ok(Self::Region2),
            0x04 => Ok(Self::Region3),
            0x05 => Ok(Self::Region4),
            0x06 => Ok(Self::Region5),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IranSubregions> for u8 {
    fn from(value: IranSubregions) -> u8 {
        match value {
            IranSubregions::Iran => 0x01,
            IranSubregions::Region1 => 0x02,
            IranSubregions::Region2 => 0x03,
            IranSubregions::Region3 => 0x04,
            IranSubregions::Region4 => 0x05,
            IranSubregions::Region5 => 0x06,
        }
    }
}

impl Display for IranSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Iran => write!(f, "Iran"),
            Self::Region1 => write!(f, "Region 1"),
            Self::Region2 => write!(f, "Region 2"),
            Self::Region3 => write!(f, "Region 3"),
            Self::Region4 => write!(f, "Region 4"),
            Self::Region5 => write!(f, "Region 5"),
        }
    }
}

// Iraq Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IraqSubregions {
    Iraq,
    BaghdadGovernorate,
    AlAnbarGovernorate,
    AlQādisiyyahGovernorate,
    BabilGovernorate,
    BasraGovernorate,
    DhiQarGovernorate,
    DiyalaGovernorate,
    DohukGovernorate,
    ErbilGovernorate,
    HalabjaGovernorate,
    KarbalaGovernorate,
    KirkukGovernorate,
    MaysanGovernorate,
    MuthannaGovernorate,
    NajafGovernorate,
    NinevehGovernorate,
    SaladinGovernorate,
    SulaymaniyahGovernorate,
    WasitGovernorate,
}

impl TryFrom<u8> for IraqSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Iraq),
            0x02 => Ok(Self::BaghdadGovernorate),
            0x03 => Ok(Self::AlAnbarGovernorate),
            0x04 => Ok(Self::AlQādisiyyahGovernorate),
            0x05 => Ok(Self::BabilGovernorate),
            0x06 => Ok(Self::BasraGovernorate),
            0x07 => Ok(Self::DhiQarGovernorate),
            0x08 => Ok(Self::DiyalaGovernorate),
            0x09 => Ok(Self::DohukGovernorate),
            0x0A => Ok(Self::ErbilGovernorate),
            0x0B => Ok(Self::HalabjaGovernorate),
            0x0C => Ok(Self::KarbalaGovernorate),
            0x0D => Ok(Self::KirkukGovernorate),
            0x0E => Ok(Self::MaysanGovernorate),
            0x0F => Ok(Self::MuthannaGovernorate),
            0x10 => Ok(Self::NajafGovernorate),
            0x11 => Ok(Self::NinevehGovernorate),
            0x12 => Ok(Self::SaladinGovernorate),
            0x13 => Ok(Self::SulaymaniyahGovernorate),
            0x14 => Ok(Self::WasitGovernorate),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IraqSubregions> for u8 {
    fn from(value: IraqSubregions) -> u8 {
        match value {
            IraqSubregions::Iraq => 0x01,
            IraqSubregions::BaghdadGovernorate => 0x02,
            IraqSubregions::AlAnbarGovernorate => 0x03,
            IraqSubregions::AlQādisiyyahGovernorate => 0x04,
            IraqSubregions::BabilGovernorate => 0x05,
            IraqSubregions::BasraGovernorate => 0x06,
            IraqSubregions::DhiQarGovernorate => 0x07,
            IraqSubregions::DiyalaGovernorate => 0x08,
            IraqSubregions::DohukGovernorate => 0x09,
            IraqSubregions::ErbilGovernorate => 0x0A,
            IraqSubregions::HalabjaGovernorate => 0x0B,
            IraqSubregions::KarbalaGovernorate => 0x0C,
            IraqSubregions::KirkukGovernorate => 0x0D,
            IraqSubregions::MaysanGovernorate => 0x0E,
            IraqSubregions::MuthannaGovernorate => 0x0F,
            IraqSubregions::NajafGovernorate => 0x10,
            IraqSubregions::NinevehGovernorate => 0x11,
            IraqSubregions::SaladinGovernorate => 0x12,
            IraqSubregions::SulaymaniyahGovernorate => 0x13,
            IraqSubregions::WasitGovernorate => 0x14,
        }
    }
}

impl Display for IraqSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Iraq => write!(f, "Iraq"),
            Self::BaghdadGovernorate => write!(f, "Baghdad Governorate"),
            Self::AlAnbarGovernorate => write!(f, "Al Anbar Governorate"),
            Self::AlQādisiyyahGovernorate => write!(f, "Al-Qādisiyyah Governorate"),
            Self::BabilGovernorate => write!(f, "Babil Governorate"),
            Self::BasraGovernorate => write!(f, "Basra Governorate"),
            Self::DhiQarGovernorate => write!(f, "Dhi Qar Governorate"),
            Self::DiyalaGovernorate => write!(f, "Diyala Governorate"),
            Self::DohukGovernorate => write!(f, "Dohuk Governorate"),
            Self::ErbilGovernorate => write!(f, "Erbil Governorate"),
            Self::HalabjaGovernorate => write!(f, "Halabja Governorate"),
            Self::KarbalaGovernorate => write!(f, "Karbala Governorate"),
            Self::KirkukGovernorate => write!(f, "Kirkuk Governorate"),
            Self::MaysanGovernorate => write!(f, "Maysan Governorate"),
            Self::MuthannaGovernorate => write!(f, "Muthanna Governorate"),
            Self::NajafGovernorate => write!(f, "Najaf Governorate"),
            Self::NinevehGovernorate => write!(f, "Nineveh Governorate"),
            Self::SaladinGovernorate => write!(f, "Saladin Governorate"),
            Self::SulaymaniyahGovernorate => write!(f, "Sulaymaniyah Governorate"),
            Self::WasitGovernorate => write!(f, "Wasit Governorate"),
        }
    }
}

// Ireland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrelandSubregions {
    Ireland,
    Dublin,
    Border,
    West,
    Midland,
    MidEast,
    MidWest,
    SouthEast,
    SouthWest,
}

impl TryFrom<u8> for IrelandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Ireland),
            0x02 => Ok(Self::Dublin),
            0x03 => Ok(Self::Border),
            0x04 => Ok(Self::West),
            0x05 => Ok(Self::Midland),
            0x06 => Ok(Self::MidEast),
            0x07 => Ok(Self::MidWest),
            0x08 => Ok(Self::SouthEast),
            0x09 => Ok(Self::SouthWest),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IrelandSubregions> for u8 {
    fn from(value: IrelandSubregions) -> u8 {
        match value {
            IrelandSubregions::Ireland => 0x01,
            IrelandSubregions::Dublin => 0x02,
            IrelandSubregions::Border => 0x03,
            IrelandSubregions::West => 0x04,
            IrelandSubregions::Midland => 0x05,
            IrelandSubregions::MidEast => 0x06,
            IrelandSubregions::MidWest => 0x07,
            IrelandSubregions::SouthEast => 0x08,
            IrelandSubregions::SouthWest => 0x09,
        }
    }
}

impl Display for IrelandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ireland => write!(f, "Ireland"),
            Self::Dublin => write!(f, "Dublin"),
            Self::Border => write!(f, "Border"),
            Self::West => write!(f, "West"),
            Self::Midland => write!(f, "Midland"),
            Self::MidEast => write!(f, "Mid-East"),
            Self::MidWest => write!(f, "Mid-West"),
            Self::SouthEast => write!(f, "South-East"),
            Self::SouthWest => write!(f, "South-West"),
        }
    }
}

// Isle of Man Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsleOfManSubregions {
    IsleOfMan,
}

impl TryFrom<u8> for IsleOfManSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::IsleOfMan),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IsleOfManSubregions> for u8 {
    fn from(value: IsleOfManSubregions) -> u8 {
        match value {
            IsleOfManSubregions::IsleOfMan => 0x01,
        }
    }
}

impl Display for IsleOfManSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IsleOfMan => write!(f, "Isle of Man"),
        }
    }
}

// Israel Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsraelSubregions {
    Israel,
    JerusalemDistrict,
    CentralDistrict,
    HaifaDistrict,
    JudeaAndSamariaArea,
    NorthernDistrict,
    SouthernDistrict,
    TelAvivDistrict,
}

impl TryFrom<u8> for IsraelSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Israel),
            0x02 => Ok(Self::JerusalemDistrict),
            0x03 => Ok(Self::CentralDistrict),
            0x04 => Ok(Self::HaifaDistrict),
            0x05 => Ok(Self::JudeaAndSamariaArea),
            0x06 => Ok(Self::NorthernDistrict),
            0x07 => Ok(Self::SouthernDistrict),
            0x08 => Ok(Self::TelAvivDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<IsraelSubregions> for u8 {
    fn from(value: IsraelSubregions) -> u8 {
        match value {
            IsraelSubregions::Israel => 0x01,
            IsraelSubregions::JerusalemDistrict => 0x02,
            IsraelSubregions::CentralDistrict => 0x03,
            IsraelSubregions::HaifaDistrict => 0x04,
            IsraelSubregions::JudeaAndSamariaArea => 0x05,
            IsraelSubregions::NorthernDistrict => 0x06,
            IsraelSubregions::SouthernDistrict => 0x07,
            IsraelSubregions::TelAvivDistrict => 0x08,
        }
    }
}

impl Display for IsraelSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Israel => write!(f, "Israel"),
            Self::JerusalemDistrict => write!(f, "Jerusalem District"),
            Self::CentralDistrict => write!(f, "Central District"),
            Self::HaifaDistrict => write!(f, "Haifa District"),
            Self::JudeaAndSamariaArea => write!(f, "Judea and Samaria Area"),
            Self::NorthernDistrict => write!(f, "Northern District"),
            Self::SouthernDistrict => write!(f, "Southern District"),
            Self::TelAvivDistrict => write!(f, "Tel Aviv District"),
        }
    }
}

// Italy Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItalySubregions {
    Italy,
    Lazio,
    AostaValley,
    Piedmont,
    Liguria,
    Lombardy,
    TrentinoSouthTyrol,
    Veneto,
    FriuliVeneziaGiulia,
    EmiliaRomagna,
    Tuscany,
    Umbria,
    Marche,
    Abruzzo,
    Molise,
    Campania,
    Apulia,
    Basilicata,
    Calabria,
    Sicily,
    Sardinia,
}

impl TryFrom<u8> for ItalySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Italy),
            0x02 => Ok(Self::Lazio),
            0x03 => Ok(Self::AostaValley),
            0x04 => Ok(Self::Piedmont),
            0x05 => Ok(Self::Liguria),
            0x06 => Ok(Self::Lombardy),
            0x07 => Ok(Self::TrentinoSouthTyrol),
            0x08 => Ok(Self::Veneto),
            0x09 => Ok(Self::FriuliVeneziaGiulia),
            0x0A => Ok(Self::EmiliaRomagna),
            0x0B => Ok(Self::Tuscany),
            0x0C => Ok(Self::Umbria),
            0x0D => Ok(Self::Marche),
            0x0E => Ok(Self::Abruzzo),
            0x0F => Ok(Self::Molise),
            0x10 => Ok(Self::Campania),
            0x11 => Ok(Self::Apulia),
            0x12 => Ok(Self::Basilicata),
            0x13 => Ok(Self::Calabria),
            0x14 => Ok(Self::Sicily),
            0x15 => Ok(Self::Sardinia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ItalySubregions> for u8 {
    fn from(value: ItalySubregions) -> u8 {
        match value {
            ItalySubregions::Italy => 0x01,
            ItalySubregions::Lazio => 0x02,
            ItalySubregions::AostaValley => 0x03,
            ItalySubregions::Piedmont => 0x04,
            ItalySubregions::Liguria => 0x05,
            ItalySubregions::Lombardy => 0x06,
            ItalySubregions::TrentinoSouthTyrol => 0x07,
            ItalySubregions::Veneto => 0x08,
            ItalySubregions::FriuliVeneziaGiulia => 0x09,
            ItalySubregions::EmiliaRomagna => 0x0A,
            ItalySubregions::Tuscany => 0x0B,
            ItalySubregions::Umbria => 0x0C,
            ItalySubregions::Marche => 0x0D,
            ItalySubregions::Abruzzo => 0x0E,
            ItalySubregions::Molise => 0x0F,
            ItalySubregions::Campania => 0x10,
            ItalySubregions::Apulia => 0x11,
            ItalySubregions::Basilicata => 0x12,
            ItalySubregions::Calabria => 0x13,
            ItalySubregions::Sicily => 0x14,
            ItalySubregions::Sardinia => 0x15,
        }
    }
}

impl Display for ItalySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Italy => write!(f, "Italy"),
            Self::Lazio => write!(f, "Lazio"),
            Self::AostaValley => write!(f, "Aosta Valley"),
            Self::Piedmont => write!(f, "Piedmont"),
            Self::Liguria => write!(f, "Liguria"),
            Self::Lombardy => write!(f, "Lombardy"),
            Self::TrentinoSouthTyrol => write!(f, "Trentino-South Tyrol"),
            Self::Veneto => write!(f, "Veneto"),
            Self::FriuliVeneziaGiulia => write!(f, "Friuli Venezia Giulia"),
            Self::EmiliaRomagna => write!(f, "Emilia-Romagna"),
            Self::Tuscany => write!(f, "Tuscany"),
            Self::Umbria => write!(f, "Umbria"),
            Self::Marche => write!(f, "Marche"),
            Self::Abruzzo => write!(f, "Abruzzo"),
            Self::Molise => write!(f, "Molise"),
            Self::Campania => write!(f, "Campania"),
            Self::Apulia => write!(f, "Apulia"),
            Self::Basilicata => write!(f, "Basilicata"),
            Self::Calabria => write!(f, "Calabria"),
            Self::Sicily => write!(f, "Sicily"),
            Self::Sardinia => write!(f, "Sardinia"),
        }
    }
}

// Jamaica Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JamaicaSubregions {
    Jamaica,
    SaintThomas,
    Clarendon,
    Hanover,
    Manchester,
    Portland,
    SaintAndrew,
    SaintAnn,
    SaintCatherine,
    SaintElizabeth,
    SaintJames,
    SaintMary,
    Trelawny,
    Westmoreland,
    KingstonSaintJohn,
}

impl TryFrom<u8> for JamaicaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Jamaica),
            0x02 => Ok(Self::SaintThomas),
            0x03 => Ok(Self::Clarendon),
            0x04 => Ok(Self::Hanover),
            0x05 => Ok(Self::Manchester),
            0x06 => Ok(Self::Portland),
            0x07 => Ok(Self::SaintAndrew),
            0x08 => Ok(Self::SaintAnn),
            0x09 => Ok(Self::SaintCatherine),
            0x0A => Ok(Self::SaintElizabeth),
            0x0B => Ok(Self::SaintJames),
            0x0C => Ok(Self::SaintMary),
            0x0D => Ok(Self::Trelawny),
            0x0E => Ok(Self::Westmoreland),
            0x0F => Ok(Self::KingstonSaintJohn),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<JamaicaSubregions> for u8 {
    fn from(value: JamaicaSubregions) -> u8 {
        match value {
            JamaicaSubregions::Jamaica => 0x01,
            JamaicaSubregions::SaintThomas => 0x02,
            JamaicaSubregions::Clarendon => 0x03,
            JamaicaSubregions::Hanover => 0x04,
            JamaicaSubregions::Manchester => 0x05,
            JamaicaSubregions::Portland => 0x06,
            JamaicaSubregions::SaintAndrew => 0x07,
            JamaicaSubregions::SaintAnn => 0x08,
            JamaicaSubregions::SaintCatherine => 0x09,
            JamaicaSubregions::SaintElizabeth => 0x0A,
            JamaicaSubregions::SaintJames => 0x0B,
            JamaicaSubregions::SaintMary => 0x0C,
            JamaicaSubregions::Trelawny => 0x0D,
            JamaicaSubregions::Westmoreland => 0x0E,
            JamaicaSubregions::KingstonSaintJohn => 0x0F,
        }
    }
}

impl Display for JamaicaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jamaica => write!(f, "Jamaica"),
            Self::SaintThomas => write!(f, "Saint Thomas"),
            Self::Clarendon => write!(f, "Clarendon"),
            Self::Hanover => write!(f, "Hanover"),
            Self::Manchester => write!(f, "Manchester"),
            Self::Portland => write!(f, "Portland"),
            Self::SaintAndrew => write!(f, "Saint Andrew"),
            Self::SaintAnn => write!(f, "Saint Ann"),
            Self::SaintCatherine => write!(f, "Saint Catherine"),
            Self::SaintElizabeth => write!(f, "Saint Elizabeth"),
            Self::SaintJames => write!(f, "Saint James"),
            Self::SaintMary => write!(f, "Saint Mary"),
            Self::Trelawny => write!(f, "Trelawny"),
            Self::Westmoreland => write!(f, "Westmoreland"),
            Self::KingstonSaintJohn => write!(f, "Kingston-Saint John"),
        }
    }
}

// Japan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JapanSubregions {
    Japan,
    Tokyo,
    Hokkaido,
    Aomori,
    Iwate,
    Miyagi,
    Akita,
    Yamagata,
    Fukushima,
    Ibaraki,
    Tochigi,
    Gunma,
    Saitama,
    Chiba,
    Kanagawa,
    Toyama,
    Ishikawa,
    Fukui,
    Yamanashi,
    Nagano,
    Niigata,
    Gifu,
    Shizuoka,
    Aichi,
    Mie,
    Shiga,
    Kyoto,
    Osaka,
    Hyogo,
    Nara,
    Wakayama,
    Tottori,
    Shimane,
    Okayama,
    Hiroshima,
    Yamaguchi,
    Tokushima,
    Kagawa,
    Ehime,
    Kochi,
    Fukuoka,
    Saga,
    Nagasaki,
    Kumamoto,
    Oita,
    Miyazaki,
    Kagoshima,
    Okinawa,
}

impl TryFrom<u8> for JapanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Japan),
            0x02 => Ok(Self::Tokyo),
            0x03 => Ok(Self::Hokkaido),
            0x04 => Ok(Self::Aomori),
            0x05 => Ok(Self::Iwate),
            0x06 => Ok(Self::Miyagi),
            0x07 => Ok(Self::Akita),
            0x08 => Ok(Self::Yamagata),
            0x09 => Ok(Self::Fukushima),
            0x0A => Ok(Self::Ibaraki),
            0x0B => Ok(Self::Tochigi),
            0x0C => Ok(Self::Gunma),
            0x0D => Ok(Self::Saitama),
            0x0E => Ok(Self::Chiba),
            0x0F => Ok(Self::Kanagawa),
            0x10 => Ok(Self::Toyama),
            0x11 => Ok(Self::Ishikawa),
            0x12 => Ok(Self::Fukui),
            0x13 => Ok(Self::Yamanashi),
            0x14 => Ok(Self::Nagano),
            0x15 => Ok(Self::Niigata),
            0x16 => Ok(Self::Gifu),
            0x17 => Ok(Self::Shizuoka),
            0x18 => Ok(Self::Aichi),
            0x19 => Ok(Self::Mie),
            0x1A => Ok(Self::Shiga),
            0x1B => Ok(Self::Kyoto),
            0x1C => Ok(Self::Osaka),
            0x1D => Ok(Self::Hyogo),
            0x1E => Ok(Self::Nara),
            0x1F => Ok(Self::Wakayama),
            0x20 => Ok(Self::Tottori),
            0x21 => Ok(Self::Shimane),
            0x22 => Ok(Self::Okayama),
            0x23 => Ok(Self::Hiroshima),
            0x24 => Ok(Self::Yamaguchi),
            0x25 => Ok(Self::Tokushima),
            0x26 => Ok(Self::Kagawa),
            0x27 => Ok(Self::Ehime),
            0x28 => Ok(Self::Kochi),
            0x29 => Ok(Self::Fukuoka),
            0x2A => Ok(Self::Saga),
            0x2B => Ok(Self::Nagasaki),
            0x2C => Ok(Self::Kumamoto),
            0x2D => Ok(Self::Oita),
            0x2E => Ok(Self::Miyazaki),
            0x2F => Ok(Self::Kagoshima),
            0x30 => Ok(Self::Okinawa),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<JapanSubregions> for u8 {
    fn from(value: JapanSubregions) -> u8 {
        match value {
            JapanSubregions::Japan => 0x01,
            JapanSubregions::Tokyo => 0x02,
            JapanSubregions::Hokkaido => 0x03,
            JapanSubregions::Aomori => 0x04,
            JapanSubregions::Iwate => 0x05,
            JapanSubregions::Miyagi => 0x06,
            JapanSubregions::Akita => 0x07,
            JapanSubregions::Yamagata => 0x08,
            JapanSubregions::Fukushima => 0x09,
            JapanSubregions::Ibaraki => 0x0A,
            JapanSubregions::Tochigi => 0x0B,
            JapanSubregions::Gunma => 0x0C,
            JapanSubregions::Saitama => 0x0D,
            JapanSubregions::Chiba => 0x0E,
            JapanSubregions::Kanagawa => 0x0F,
            JapanSubregions::Toyama => 0x10,
            JapanSubregions::Ishikawa => 0x11,
            JapanSubregions::Fukui => 0x12,
            JapanSubregions::Yamanashi => 0x13,
            JapanSubregions::Nagano => 0x14,
            JapanSubregions::Niigata => 0x15,
            JapanSubregions::Gifu => 0x16,
            JapanSubregions::Shizuoka => 0x17,
            JapanSubregions::Aichi => 0x18,
            JapanSubregions::Mie => 0x19,
            JapanSubregions::Shiga => 0x1A,
            JapanSubregions::Kyoto => 0x1B,
            JapanSubregions::Osaka => 0x1C,
            JapanSubregions::Hyogo => 0x1D,
            JapanSubregions::Nara => 0x1E,
            JapanSubregions::Wakayama => 0x1F,
            JapanSubregions::Tottori => 0x20,
            JapanSubregions::Shimane => 0x21,
            JapanSubregions::Okayama => 0x22,
            JapanSubregions::Hiroshima => 0x23,
            JapanSubregions::Yamaguchi => 0x24,
            JapanSubregions::Tokushima => 0x25,
            JapanSubregions::Kagawa => 0x26,
            JapanSubregions::Ehime => 0x27,
            JapanSubregions::Kochi => 0x28,
            JapanSubregions::Fukuoka => 0x29,
            JapanSubregions::Saga => 0x2A,
            JapanSubregions::Nagasaki => 0x2B,
            JapanSubregions::Kumamoto => 0x2C,
            JapanSubregions::Oita => 0x2D,
            JapanSubregions::Miyazaki => 0x2E,
            JapanSubregions::Kagoshima => 0x2F,
            JapanSubregions::Okinawa => 0x30,
        }
    }
}

impl Display for JapanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Japan => write!(f, "Japan"),
            Self::Tokyo => write!(f, "Tokyo"),
            Self::Hokkaido => write!(f, "Hokkaido"),
            Self::Aomori => write!(f, "Aomori"),
            Self::Iwate => write!(f, "Iwate"),
            Self::Miyagi => write!(f, "Miyagi"),
            Self::Akita => write!(f, "Akita"),
            Self::Yamagata => write!(f, "Yamagata"),
            Self::Fukushima => write!(f, "Fukushima"),
            Self::Ibaraki => write!(f, "Ibaraki"),
            Self::Tochigi => write!(f, "Tochigi"),
            Self::Gunma => write!(f, "Gunma"),
            Self::Saitama => write!(f, "Saitama"),
            Self::Chiba => write!(f, "Chiba"),
            Self::Kanagawa => write!(f, "Kanagawa"),
            Self::Toyama => write!(f, "Toyama"),
            Self::Ishikawa => write!(f, "Ishikawa"),
            Self::Fukui => write!(f, "Fukui"),
            Self::Yamanashi => write!(f, "Yamanashi"),
            Self::Nagano => write!(f, "Nagano"),
            Self::Niigata => write!(f, "Niigata"),
            Self::Gifu => write!(f, "Gifu"),
            Self::Shizuoka => write!(f, "Shizuoka"),
            Self::Aichi => write!(f, "Aichi"),
            Self::Mie => write!(f, "Mie"),
            Self::Shiga => write!(f, "Shiga"),
            Self::Kyoto => write!(f, "Kyoto"),
            Self::Osaka => write!(f, "Osaka"),
            Self::Hyogo => write!(f, "Hyogo"),
            Self::Nara => write!(f, "Nara"),
            Self::Wakayama => write!(f, "Wakayama"),
            Self::Tottori => write!(f, "Tottori"),
            Self::Shimane => write!(f, "Shimane"),
            Self::Okayama => write!(f, "Okayama"),
            Self::Hiroshima => write!(f, "Hiroshima"),
            Self::Yamaguchi => write!(f, "Yamaguchi"),
            Self::Tokushima => write!(f, "Tokushima"),
            Self::Kagawa => write!(f, "Kagawa"),
            Self::Ehime => write!(f, "Ehime"),
            Self::Kochi => write!(f, "Kochi"),
            Self::Fukuoka => write!(f, "Fukuoka"),
            Self::Saga => write!(f, "Saga"),
            Self::Nagasaki => write!(f, "Nagasaki"),
            Self::Kumamoto => write!(f, "Kumamoto"),
            Self::Oita => write!(f, "Oita"),
            Self::Miyazaki => write!(f, "Miyazaki"),
            Self::Kagoshima => write!(f, "Kagoshima"),
            Self::Okinawa => write!(f, "Okinawa"),
        }
    }
}

// Jersey Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JerseySubregions {
    Jersey,
}

impl TryFrom<u8> for JerseySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Jersey),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<JerseySubregions> for u8 {
    fn from(value: JerseySubregions) -> u8 {
        match value {
            JerseySubregions::Jersey => 0x01,
        }
    }
}

impl Display for JerseySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jersey => write!(f, "Jersey"),
        }
    }
}

// Jordan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JordanSubregions {
    Jordan,
    AmmanGovernorate,
    AjlounGovernorate,
    AqabaGovernorate,
    BalqaGovernorate,
    IrbidGovernorate,
    JerashGovernorate,
    KarakGovernorate,
    MaanGovernorate,
    MadabaGovernorate,
    MafraqGovernorate,
    TafilahGovernorate,
    ZarqaGovernorate,
}

impl TryFrom<u8> for JordanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Jordan),
            0x02 => Ok(Self::AmmanGovernorate),
            0x03 => Ok(Self::AjlounGovernorate),
            0x04 => Ok(Self::AqabaGovernorate),
            0x05 => Ok(Self::BalqaGovernorate),
            0x06 => Ok(Self::IrbidGovernorate),
            0x07 => Ok(Self::JerashGovernorate),
            0x08 => Ok(Self::KarakGovernorate),
            0x09 => Ok(Self::MaanGovernorate),
            0x0A => Ok(Self::MadabaGovernorate),
            0x0B => Ok(Self::MafraqGovernorate),
            0x0C => Ok(Self::TafilahGovernorate),
            0x0D => Ok(Self::ZarqaGovernorate),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<JordanSubregions> for u8 {
    fn from(value: JordanSubregions) -> u8 {
        match value {
            JordanSubregions::Jordan => 0x01,
            JordanSubregions::AmmanGovernorate => 0x02,
            JordanSubregions::AjlounGovernorate => 0x03,
            JordanSubregions::AqabaGovernorate => 0x04,
            JordanSubregions::BalqaGovernorate => 0x05,
            JordanSubregions::IrbidGovernorate => 0x06,
            JordanSubregions::JerashGovernorate => 0x07,
            JordanSubregions::KarakGovernorate => 0x08,
            JordanSubregions::MaanGovernorate => 0x09,
            JordanSubregions::MadabaGovernorate => 0x0A,
            JordanSubregions::MafraqGovernorate => 0x0B,
            JordanSubregions::TafilahGovernorate => 0x0C,
            JordanSubregions::ZarqaGovernorate => 0x0D,
        }
    }
}

impl Display for JordanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jordan => write!(f, "Jordan"),
            Self::AmmanGovernorate => write!(f, "Amman Governorate"),
            Self::AjlounGovernorate => write!(f, "Ajloun Governorate"),
            Self::AqabaGovernorate => write!(f, "Aqaba Governorate"),
            Self::BalqaGovernorate => write!(f, "Balqa Governorate"),
            Self::IrbidGovernorate => write!(f, "Irbid Governorate"),
            Self::JerashGovernorate => write!(f, "Jerash Governorate"),
            Self::KarakGovernorate => write!(f, "Karak Governorate"),
            Self::MaanGovernorate => write!(f, "Ma'an Governorate"),
            Self::MadabaGovernorate => write!(f, "Madaba Governorate"),
            Self::MafraqGovernorate => write!(f, "Mafraq Governorate"),
            Self::TafilahGovernorate => write!(f, "Tafilah Governorate"),
            Self::ZarqaGovernorate => write!(f, "Zarqa Governorate"),
        }
    }
}

// Kazakhstan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KazakhstanSubregions {
    Kazakhstan,
    NurSultan,
    AkmolaRegion,
    AktobeRegion,
    Almaty,
    AlmatyRegion,
    AtyrauRegion,
    Baikonur,
    EastKazakhstanRegion,
    JambylRegion,
    KaragandaRegion,
    KostanayRegion,
    KyzylordaRegion,
    MagystauRegion,
    NorthKazakhstanRegion,
    PavlodarRegion,
    Shymkent,
    TurkistanRegion,
    WestKazakhstanRegion,
}

impl TryFrom<u8> for KazakhstanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Kazakhstan),
            0x02 => Ok(Self::NurSultan),
            0x03 => Ok(Self::AkmolaRegion),
            0x04 => Ok(Self::AktobeRegion),
            0x05 => Ok(Self::Almaty),
            0x06 => Ok(Self::AlmatyRegion),
            0x07 => Ok(Self::AtyrauRegion),
            0x08 => Ok(Self::Baikonur),
            0x09 => Ok(Self::EastKazakhstanRegion),
            0x0A => Ok(Self::JambylRegion),
            0x0B => Ok(Self::KaragandaRegion),
            0x0C => Ok(Self::KostanayRegion),
            0x0D => Ok(Self::KyzylordaRegion),
            0x0E => Ok(Self::MagystauRegion),
            0x0F => Ok(Self::NorthKazakhstanRegion),
            0x10 => Ok(Self::PavlodarRegion),
            0x11 => Ok(Self::Shymkent),
            0x12 => Ok(Self::TurkistanRegion),
            0x13 => Ok(Self::WestKazakhstanRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<KazakhstanSubregions> for u8 {
    fn from(value: KazakhstanSubregions) -> u8 {
        match value {
            KazakhstanSubregions::Kazakhstan => 0x01,
            KazakhstanSubregions::NurSultan => 0x02,
            KazakhstanSubregions::AkmolaRegion => 0x03,
            KazakhstanSubregions::AktobeRegion => 0x04,
            KazakhstanSubregions::Almaty => 0x05,
            KazakhstanSubregions::AlmatyRegion => 0x06,
            KazakhstanSubregions::AtyrauRegion => 0x07,
            KazakhstanSubregions::Baikonur => 0x08,
            KazakhstanSubregions::EastKazakhstanRegion => 0x09,
            KazakhstanSubregions::JambylRegion => 0x0A,
            KazakhstanSubregions::KaragandaRegion => 0x0B,
            KazakhstanSubregions::KostanayRegion => 0x0C,
            KazakhstanSubregions::KyzylordaRegion => 0x0D,
            KazakhstanSubregions::MagystauRegion => 0x0E,
            KazakhstanSubregions::NorthKazakhstanRegion => 0x0F,
            KazakhstanSubregions::PavlodarRegion => 0x10,
            KazakhstanSubregions::Shymkent => 0x11,
            KazakhstanSubregions::TurkistanRegion => 0x12,
            KazakhstanSubregions::WestKazakhstanRegion => 0x13,
        }
    }
}

impl Display for KazakhstanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kazakhstan => write!(f, "Kazakhstan"),
            Self::NurSultan => write!(f, "Nur-Sultan"),
            Self::AkmolaRegion => write!(f, "Akmola Region"),
            Self::AktobeRegion => write!(f, "Aktobe Region"),
            Self::Almaty => write!(f, "Almaty"),
            Self::AlmatyRegion => write!(f, "Almaty Region"),
            Self::AtyrauRegion => write!(f, "Atyrau Region"),
            Self::Baikonur => write!(f, "Baikonur"),
            Self::EastKazakhstanRegion => write!(f, "East Kazakhstan Region"),
            Self::JambylRegion => write!(f, "Jambyl Region"),
            Self::KaragandaRegion => write!(f, "Karaganda Region"),
            Self::KostanayRegion => write!(f, "Kostanay Region"),
            Self::KyzylordaRegion => write!(f, "Kyzylorda Region"),
            Self::MagystauRegion => write!(f, "Magystau Region"),
            Self::NorthKazakhstanRegion => write!(f, "North Kazakhstan Region"),
            Self::PavlodarRegion => write!(f, "Pavlodar Region"),
            Self::Shymkent => write!(f, "Shymkent"),
            Self::TurkistanRegion => write!(f, "Turkistan Region"),
            Self::WestKazakhstanRegion => write!(f, "West Kazakhstan Region"),
        }
    }
}

// Kenya Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KenyaSubregions {
    Kenya,
    Nairobi,
    Central,
    Coast,
    Eastern,
    NorthEastern,
    Nyanza,
    RiftValley,
    Western,
}

impl TryFrom<u8> for KenyaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Kenya),
            0x02 => Ok(Self::Nairobi),
            0x03 => Ok(Self::Central),
            0x04 => Ok(Self::Coast),
            0x05 => Ok(Self::Eastern),
            0x06 => Ok(Self::NorthEastern),
            0x07 => Ok(Self::Nyanza),
            0x08 => Ok(Self::RiftValley),
            0x09 => Ok(Self::Western),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<KenyaSubregions> for u8 {
    fn from(value: KenyaSubregions) -> u8 {
        match value {
            KenyaSubregions::Kenya => 0x01,
            KenyaSubregions::Nairobi => 0x02,
            KenyaSubregions::Central => 0x03,
            KenyaSubregions::Coast => 0x04,
            KenyaSubregions::Eastern => 0x05,
            KenyaSubregions::NorthEastern => 0x06,
            KenyaSubregions::Nyanza => 0x07,
            KenyaSubregions::RiftValley => 0x08,
            KenyaSubregions::Western => 0x09,
        }
    }
}

impl Display for KenyaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kenya => write!(f, "Kenya"),
            Self::Nairobi => write!(f, "Nairobi"),
            Self::Central => write!(f, "Central"),
            Self::Coast => write!(f, "Coast"),
            Self::Eastern => write!(f, "Eastern"),
            Self::NorthEastern => write!(f, "North Eastern"),
            Self::Nyanza => write!(f, "Nyanza"),
            Self::RiftValley => write!(f, "Rift Valley"),
            Self::Western => write!(f, "Western"),
        }
    }
}

// Kiribati Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KiribatiSubregions {
    Kiribati,
    GilbertIslands,
    LineIslands,
    PhoenixIslands,
}

impl TryFrom<u8> for KiribatiSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Kiribati),
            0x02 => Ok(Self::GilbertIslands),
            0x03 => Ok(Self::LineIslands),
            0x04 => Ok(Self::PhoenixIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<KiribatiSubregions> for u8 {
    fn from(value: KiribatiSubregions) -> u8 {
        match value {
            KiribatiSubregions::Kiribati => 0x01,
            KiribatiSubregions::GilbertIslands => 0x02,
            KiribatiSubregions::LineIslands => 0x03,
            KiribatiSubregions::PhoenixIslands => 0x04,
        }
    }
}

impl Display for KiribatiSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kiribati => write!(f, "Kiribati"),
            Self::GilbertIslands => write!(f, "Gilbert Islands"),
            Self::LineIslands => write!(f, "Line Islands"),
            Self::PhoenixIslands => write!(f, "Phoenix Islands"),
        }
    }
}

// Kosovo Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KosovoSubregions {
    Kosovo,
    DistrictOfPristina,
    DistrictOfFerizaj,
    DistrictOfGjakova,
    DistrictOfGjilan,
    DistrictOfMitrovica,
    DistrictOfPeć,
    DistrictOfPrizren,
}

impl TryFrom<u8> for KosovoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Kosovo),
            0x02 => Ok(Self::DistrictOfPristina),
            0x03 => Ok(Self::DistrictOfFerizaj),
            0x04 => Ok(Self::DistrictOfGjakova),
            0x05 => Ok(Self::DistrictOfGjilan),
            0x06 => Ok(Self::DistrictOfMitrovica),
            0x07 => Ok(Self::DistrictOfPeć),
            0x08 => Ok(Self::DistrictOfPrizren),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<KosovoSubregions> for u8 {
    fn from(value: KosovoSubregions) -> u8 {
        match value {
            KosovoSubregions::Kosovo => 0x01,
            KosovoSubregions::DistrictOfPristina => 0x02,
            KosovoSubregions::DistrictOfFerizaj => 0x03,
            KosovoSubregions::DistrictOfGjakova => 0x04,
            KosovoSubregions::DistrictOfGjilan => 0x05,
            KosovoSubregions::DistrictOfMitrovica => 0x06,
            KosovoSubregions::DistrictOfPeć => 0x07,
            KosovoSubregions::DistrictOfPrizren => 0x08,
        }
    }
}

impl Display for KosovoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kosovo => write!(f, "Kosovo"),
            Self::DistrictOfPristina => write!(f, "District of Pristina"),
            Self::DistrictOfFerizaj => write!(f, "District of Ferizaj"),
            Self::DistrictOfGjakova => write!(f, "District of Gjakova"),
            Self::DistrictOfGjilan => write!(f, "District of Gjilan"),
            Self::DistrictOfMitrovica => write!(f, "District of Mitrovica"),
            Self::DistrictOfPeć => write!(f, "District of Peć"),
            Self::DistrictOfPrizren => write!(f, "District of Prizren"),
        }
    }
}

// Kuwait Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KuwaitSubregions {
    Kuwait,
    AlAsimahGovernorate,
    AhmadiGovernorate,
    FarwaniyaGovernorate,
    HawalliGovernorate,
    JahraGovernorate,
    MubarakAlKabeerGovernorate,
}

impl TryFrom<u8> for KuwaitSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Kuwait),
            0x02 => Ok(Self::AlAsimahGovernorate),
            0x03 => Ok(Self::AhmadiGovernorate),
            0x04 => Ok(Self::FarwaniyaGovernorate),
            0x05 => Ok(Self::HawalliGovernorate),
            0x06 => Ok(Self::JahraGovernorate),
            0x07 => Ok(Self::MubarakAlKabeerGovernorate),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<KuwaitSubregions> for u8 {
    fn from(value: KuwaitSubregions) -> u8 {
        match value {
            KuwaitSubregions::Kuwait => 0x01,
            KuwaitSubregions::AlAsimahGovernorate => 0x02,
            KuwaitSubregions::AhmadiGovernorate => 0x03,
            KuwaitSubregions::FarwaniyaGovernorate => 0x04,
            KuwaitSubregions::HawalliGovernorate => 0x05,
            KuwaitSubregions::JahraGovernorate => 0x06,
            KuwaitSubregions::MubarakAlKabeerGovernorate => 0x07,
        }
    }
}

impl Display for KuwaitSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kuwait => write!(f, "Kuwait"),
            Self::AlAsimahGovernorate => write!(f, "Al Asimah Governorate"),
            Self::AhmadiGovernorate => write!(f, "Ahmadi Governorate"),
            Self::FarwaniyaGovernorate => write!(f, "Farwaniya Governorate"),
            Self::HawalliGovernorate => write!(f, "Hawalli Governorate"),
            Self::JahraGovernorate => write!(f, "Jahra Governorate"),
            Self::MubarakAlKabeerGovernorate => write!(f, "Mubarak Al-Kabeer Governorate"),
        }
    }
}

// Kyrgyzstan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KyrgyzstanSubregions {
    Kyrgyzstan,
    ChuyRegion,
    BatkenRegion,
    IssykKulRegion,
    JalalAbadRegion,
    NarynRegion,
    OshRegion,
    TalasRegion,
}

impl TryFrom<u8> for KyrgyzstanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Kyrgyzstan),
            0x02 => Ok(Self::ChuyRegion),
            0x03 => Ok(Self::BatkenRegion),
            0x04 => Ok(Self::IssykKulRegion),
            0x05 => Ok(Self::JalalAbadRegion),
            0x06 => Ok(Self::NarynRegion),
            0x07 => Ok(Self::OshRegion),
            0x08 => Ok(Self::TalasRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<KyrgyzstanSubregions> for u8 {
    fn from(value: KyrgyzstanSubregions) -> u8 {
        match value {
            KyrgyzstanSubregions::Kyrgyzstan => 0x01,
            KyrgyzstanSubregions::ChuyRegion => 0x02,
            KyrgyzstanSubregions::BatkenRegion => 0x03,
            KyrgyzstanSubregions::IssykKulRegion => 0x04,
            KyrgyzstanSubregions::JalalAbadRegion => 0x05,
            KyrgyzstanSubregions::NarynRegion => 0x06,
            KyrgyzstanSubregions::OshRegion => 0x07,
            KyrgyzstanSubregions::TalasRegion => 0x08,
        }
    }
}

impl Display for KyrgyzstanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kyrgyzstan => write!(f, "Kyrgyzstan"),
            Self::ChuyRegion => write!(f, "Chuy Region"),
            Self::BatkenRegion => write!(f, "Batken Region"),
            Self::IssykKulRegion => write!(f, "Issyk-Kul Region"),
            Self::JalalAbadRegion => write!(f, "Jalal-Abad Region"),
            Self::NarynRegion => write!(f, "Naryn Region"),
            Self::OshRegion => write!(f, "Osh Region"),
            Self::TalasRegion => write!(f, "Talas Region"),
        }
    }
}

// Laos Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LaosSubregions {
    Laos,
    VientianePrefecture,
    Attapeu,
    Bokeo,
    Bolikhamsai,
    Champasak,
    HuaPhan,
    Khammouane,
    LuangNamtha,
    LuangPrabang,
    Oudomxay,
    Phongsali,
    Sayabouly,
    Salavan,
    Savannakhet,
    Sekong,
    VientianeProvince,
    XiengKhouang,
    XaisombounProvince,
}

impl TryFrom<u8> for LaosSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Laos),
            0x02 => Ok(Self::VientianePrefecture),
            0x03 => Ok(Self::Attapeu),
            0x04 => Ok(Self::Bokeo),
            0x05 => Ok(Self::Bolikhamsai),
            0x06 => Ok(Self::Champasak),
            0x07 => Ok(Self::HuaPhan),
            0x08 => Ok(Self::Khammouane),
            0x09 => Ok(Self::LuangNamtha),
            0x0A => Ok(Self::LuangPrabang),
            0x0B => Ok(Self::Oudomxay),
            0x0C => Ok(Self::Phongsali),
            0x0D => Ok(Self::Sayabouly),
            0x0E => Ok(Self::Salavan),
            0x0F => Ok(Self::Savannakhet),
            0x10 => Ok(Self::Sekong),
            0x11 => Ok(Self::VientianeProvince),
            0x12 => Ok(Self::XiengKhouang),
            0x13 => Ok(Self::XaisombounProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LaosSubregions> for u8 {
    fn from(value: LaosSubregions) -> u8 {
        match value {
            LaosSubregions::Laos => 0x01,
            LaosSubregions::VientianePrefecture => 0x02,
            LaosSubregions::Attapeu => 0x03,
            LaosSubregions::Bokeo => 0x04,
            LaosSubregions::Bolikhamsai => 0x05,
            LaosSubregions::Champasak => 0x06,
            LaosSubregions::HuaPhan => 0x07,
            LaosSubregions::Khammouane => 0x08,
            LaosSubregions::LuangNamtha => 0x09,
            LaosSubregions::LuangPrabang => 0x0A,
            LaosSubregions::Oudomxay => 0x0B,
            LaosSubregions::Phongsali => 0x0C,
            LaosSubregions::Sayabouly => 0x0D,
            LaosSubregions::Salavan => 0x0E,
            LaosSubregions::Savannakhet => 0x0F,
            LaosSubregions::Sekong => 0x10,
            LaosSubregions::VientianeProvince => 0x11,
            LaosSubregions::XiengKhouang => 0x12,
            LaosSubregions::XaisombounProvince => 0x13,
        }
    }
}

impl Display for LaosSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Laos => write!(f, "Laos"),
            Self::VientianePrefecture => write!(f, "Vientiane Prefecture"),
            Self::Attapeu => write!(f, "Attapeu"),
            Self::Bokeo => write!(f, "Bokeo"),
            Self::Bolikhamsai => write!(f, "Bolikhamsai"),
            Self::Champasak => write!(f, "Champasak"),
            Self::HuaPhan => write!(f, "Hua Phan"),
            Self::Khammouane => write!(f, "Khammouane"),
            Self::LuangNamtha => write!(f, "Luang Namtha"),
            Self::LuangPrabang => write!(f, "Luang Prabang"),
            Self::Oudomxay => write!(f, "Oudomxay"),
            Self::Phongsali => write!(f, "Phongsali"),
            Self::Sayabouly => write!(f, "Sayabouly"),
            Self::Salavan => write!(f, "Salavan"),
            Self::Savannakhet => write!(f, "Savannakhet"),
            Self::Sekong => write!(f, "Sekong"),
            Self::VientianeProvince => write!(f, "Vientiane Province"),
            Self::XiengKhouang => write!(f, "Xieng Khouang"),
            Self::XaisombounProvince => write!(f, "Xaisomboun Province"),
        }
    }
}

// Latvia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LatviaSubregions {
    Latvia,
    Riga,
    Kuldiga,
    Liepaja,
    Talsi,
    Ventspils,
    Saldus,
    Tukums,
    Aizkraukle,
    Aluksne,
    Cesis,
    Gulbene,
    Madona,
    Limbazi,
    Ogre,
    Valka,
    Valmiera,
    Balvi,
    Daugavpils,
    Jekabpils,
    Kraslava,
    Ludza,
    Preili,
    Rezekne,
    Bauska,
    Dobele,
    Jelgava,
}

impl TryFrom<u8> for LatviaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Latvia),
            0x02 => Ok(Self::Riga),
            0x03 => Ok(Self::Kuldiga),
            0x04 => Ok(Self::Liepaja),
            0x05 => Ok(Self::Talsi),
            0x06 => Ok(Self::Ventspils),
            0x07 => Ok(Self::Saldus),
            0x08 => Ok(Self::Tukums),
            0x09 => Ok(Self::Aizkraukle),
            0x0A => Ok(Self::Aluksne),
            0x0B => Ok(Self::Cesis),
            0x0C => Ok(Self::Gulbene),
            0x0D => Ok(Self::Madona),
            0x0E => Ok(Self::Limbazi),
            0x0F => Ok(Self::Ogre),
            0x10 => Ok(Self::Valka),
            0x11 => Ok(Self::Valmiera),
            0x12 => Ok(Self::Balvi),
            0x13 => Ok(Self::Daugavpils),
            0x14 => Ok(Self::Jekabpils),
            0x15 => Ok(Self::Kraslava),
            0x16 => Ok(Self::Ludza),
            0x17 => Ok(Self::Preili),
            0x18 => Ok(Self::Rezekne),
            0x19 => Ok(Self::Bauska),
            0x1A => Ok(Self::Dobele),
            0x1B => Ok(Self::Jelgava),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LatviaSubregions> for u8 {
    fn from(value: LatviaSubregions) -> u8 {
        match value {
            LatviaSubregions::Latvia => 0x01,
            LatviaSubregions::Riga => 0x02,
            LatviaSubregions::Kuldiga => 0x03,
            LatviaSubregions::Liepaja => 0x04,
            LatviaSubregions::Talsi => 0x05,
            LatviaSubregions::Ventspils => 0x06,
            LatviaSubregions::Saldus => 0x07,
            LatviaSubregions::Tukums => 0x08,
            LatviaSubregions::Aizkraukle => 0x09,
            LatviaSubregions::Aluksne => 0x0A,
            LatviaSubregions::Cesis => 0x0B,
            LatviaSubregions::Gulbene => 0x0C,
            LatviaSubregions::Madona => 0x0D,
            LatviaSubregions::Limbazi => 0x0E,
            LatviaSubregions::Ogre => 0x0F,
            LatviaSubregions::Valka => 0x10,
            LatviaSubregions::Valmiera => 0x11,
            LatviaSubregions::Balvi => 0x12,
            LatviaSubregions::Daugavpils => 0x13,
            LatviaSubregions::Jekabpils => 0x14,
            LatviaSubregions::Kraslava => 0x15,
            LatviaSubregions::Ludza => 0x16,
            LatviaSubregions::Preili => 0x17,
            LatviaSubregions::Rezekne => 0x18,
            LatviaSubregions::Bauska => 0x19,
            LatviaSubregions::Dobele => 0x1A,
            LatviaSubregions::Jelgava => 0x1B,
        }
    }
}

impl Display for LatviaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Latvia => write!(f, "Latvia"),
            Self::Riga => write!(f, "Riga"),
            Self::Kuldiga => write!(f, "Kuldiga"),
            Self::Liepaja => write!(f, "Liepaja"),
            Self::Talsi => write!(f, "Talsi"),
            Self::Ventspils => write!(f, "Ventspils"),
            Self::Saldus => write!(f, "Saldus"),
            Self::Tukums => write!(f, "Tukums"),
            Self::Aizkraukle => write!(f, "Aizkraukle"),
            Self::Aluksne => write!(f, "Aluksne"),
            Self::Cesis => write!(f, "Cesis"),
            Self::Gulbene => write!(f, "Gulbene"),
            Self::Madona => write!(f, "Madona"),
            Self::Limbazi => write!(f, "Limbazi"),
            Self::Ogre => write!(f, "Ogre"),
            Self::Valka => write!(f, "Valka"),
            Self::Valmiera => write!(f, "Valmiera"),
            Self::Balvi => write!(f, "Balvi"),
            Self::Daugavpils => write!(f, "Daugavpils"),
            Self::Jekabpils => write!(f, "Jekabpils"),
            Self::Kraslava => write!(f, "Kraslava"),
            Self::Ludza => write!(f, "Ludza"),
            Self::Preili => write!(f, "Preili"),
            Self::Rezekne => write!(f, "Rezekne"),
            Self::Bauska => write!(f, "Bauska"),
            Self::Dobele => write!(f, "Dobele"),
            Self::Jelgava => write!(f, "Jelgava"),
        }
    }
}

// Lebanon Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LebanonSubregions {
    Lebanon,
    BeirutGovernorate,
    AkkarGovernorate,
    BaalbekHermelGovernorate,
    BeqaaGovernorate,
    MountLebanonGovernorate,
    NabatiehGovernorate,
    NorthGovernorate,
    SouthGovernorate,
}

impl TryFrom<u8> for LebanonSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Lebanon),
            0x02 => Ok(Self::BeirutGovernorate),
            0x03 => Ok(Self::AkkarGovernorate),
            0x04 => Ok(Self::BaalbekHermelGovernorate),
            0x05 => Ok(Self::BeqaaGovernorate),
            0x06 => Ok(Self::MountLebanonGovernorate),
            0x07 => Ok(Self::NabatiehGovernorate),
            0x08 => Ok(Self::NorthGovernorate),
            0x09 => Ok(Self::SouthGovernorate),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LebanonSubregions> for u8 {
    fn from(value: LebanonSubregions) -> u8 {
        match value {
            LebanonSubregions::Lebanon => 0x01,
            LebanonSubregions::BeirutGovernorate => 0x02,
            LebanonSubregions::AkkarGovernorate => 0x03,
            LebanonSubregions::BaalbekHermelGovernorate => 0x04,
            LebanonSubregions::BeqaaGovernorate => 0x05,
            LebanonSubregions::MountLebanonGovernorate => 0x06,
            LebanonSubregions::NabatiehGovernorate => 0x07,
            LebanonSubregions::NorthGovernorate => 0x08,
            LebanonSubregions::SouthGovernorate => 0x09,
        }
    }
}

impl Display for LebanonSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lebanon => write!(f, "Lebanon"),
            Self::BeirutGovernorate => write!(f, "Beirut Governorate"),
            Self::AkkarGovernorate => write!(f, "Akkar Governorate"),
            Self::BaalbekHermelGovernorate => write!(f, "Baalbek-Hermel Governorate"),
            Self::BeqaaGovernorate => write!(f, "Beqaa Governorate"),
            Self::MountLebanonGovernorate => write!(f, "Mount Lebanon Governorate"),
            Self::NabatiehGovernorate => write!(f, "Nabatieh Governorate"),
            Self::NorthGovernorate => write!(f, "North Governorate"),
            Self::SouthGovernorate => write!(f, "South Governorate"),
        }
    }
}

// Lesotho Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LesothoSubregions {
    Lesotho,
    Maseru,
    Berea,
    ButhaButhe,
    Leribe,
    Mafeteng,
    MohalesHoek,
    Mokhotlong,
    QachasNek,
    Quthing,
    ThabaTseka,
}

impl TryFrom<u8> for LesothoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Lesotho),
            0x02 => Ok(Self::Maseru),
            0x03 => Ok(Self::Berea),
            0x04 => Ok(Self::ButhaButhe),
            0x05 => Ok(Self::Leribe),
            0x06 => Ok(Self::Mafeteng),
            0x07 => Ok(Self::MohalesHoek),
            0x08 => Ok(Self::Mokhotlong),
            0x09 => Ok(Self::QachasNek),
            0x0A => Ok(Self::Quthing),
            0x0B => Ok(Self::ThabaTseka),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LesothoSubregions> for u8 {
    fn from(value: LesothoSubregions) -> u8 {
        match value {
            LesothoSubregions::Lesotho => 0x01,
            LesothoSubregions::Maseru => 0x02,
            LesothoSubregions::Berea => 0x03,
            LesothoSubregions::ButhaButhe => 0x04,
            LesothoSubregions::Leribe => 0x05,
            LesothoSubregions::Mafeteng => 0x06,
            LesothoSubregions::MohalesHoek => 0x07,
            LesothoSubregions::Mokhotlong => 0x08,
            LesothoSubregions::QachasNek => 0x09,
            LesothoSubregions::Quthing => 0x0A,
            LesothoSubregions::ThabaTseka => 0x0B,
        }
    }
}

impl Display for LesothoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lesotho => write!(f, "Lesotho"),
            Self::Maseru => write!(f, "Maseru"),
            Self::Berea => write!(f, "Berea"),
            Self::ButhaButhe => write!(f, "Butha-Buthe"),
            Self::Leribe => write!(f, "Leribe"),
            Self::Mafeteng => write!(f, "Mafeteng"),
            Self::MohalesHoek => write!(f, "Mohale's Hoek"),
            Self::Mokhotlong => write!(f, "Mokhotlong"),
            Self::QachasNek => write!(f, "Qacha's Nek"),
            Self::Quthing => write!(f, "Quthing"),
            Self::ThabaTseka => write!(f, "Thaba-Tseka"),
        }
    }
}

// Liberia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiberiaSubregions {
    Liberia,
    Montserrado,
    Bomi,
    Bong,
    Gbarpolu,
    GrandBassa,
    GrandCapeMount,
    GrandGedeh,
    GrandKru,
    Lofa,
    Margibi,
    Maryland,
    Nimba,
    Rivercess,
    RiverGee,
    Sinoe,
}

impl TryFrom<u8> for LiberiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Liberia),
            0x02 => Ok(Self::Montserrado),
            0x03 => Ok(Self::Bomi),
            0x04 => Ok(Self::Bong),
            0x05 => Ok(Self::Gbarpolu),
            0x06 => Ok(Self::GrandBassa),
            0x07 => Ok(Self::GrandCapeMount),
            0x08 => Ok(Self::GrandGedeh),
            0x09 => Ok(Self::GrandKru),
            0x0A => Ok(Self::Lofa),
            0x0B => Ok(Self::Margibi),
            0x0C => Ok(Self::Maryland),
            0x0D => Ok(Self::Nimba),
            0x0E => Ok(Self::Rivercess),
            0x0F => Ok(Self::RiverGee),
            0x10 => Ok(Self::Sinoe),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LiberiaSubregions> for u8 {
    fn from(value: LiberiaSubregions) -> u8 {
        match value {
            LiberiaSubregions::Liberia => 0x01,
            LiberiaSubregions::Montserrado => 0x02,
            LiberiaSubregions::Bomi => 0x03,
            LiberiaSubregions::Bong => 0x04,
            LiberiaSubregions::Gbarpolu => 0x05,
            LiberiaSubregions::GrandBassa => 0x06,
            LiberiaSubregions::GrandCapeMount => 0x07,
            LiberiaSubregions::GrandGedeh => 0x08,
            LiberiaSubregions::GrandKru => 0x09,
            LiberiaSubregions::Lofa => 0x0A,
            LiberiaSubregions::Margibi => 0x0B,
            LiberiaSubregions::Maryland => 0x0C,
            LiberiaSubregions::Nimba => 0x0D,
            LiberiaSubregions::Rivercess => 0x0E,
            LiberiaSubregions::RiverGee => 0x0F,
            LiberiaSubregions::Sinoe => 0x10,
        }
    }
}

impl Display for LiberiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Liberia => write!(f, "Liberia"),
            Self::Montserrado => write!(f, "Montserrado"),
            Self::Bomi => write!(f, "Bomi"),
            Self::Bong => write!(f, "Bong"),
            Self::Gbarpolu => write!(f, "Gbarpolu"),
            Self::GrandBassa => write!(f, "Grand Bassa"),
            Self::GrandCapeMount => write!(f, "Grand Cape Mount"),
            Self::GrandGedeh => write!(f, "Grand Gedeh"),
            Self::GrandKru => write!(f, "Grand Kru"),
            Self::Lofa => write!(f, "Lofa"),
            Self::Margibi => write!(f, "Margibi"),
            Self::Maryland => write!(f, "Maryland"),
            Self::Nimba => write!(f, "Nimba"),
            Self::Rivercess => write!(f, "Rivercess"),
            Self::RiverGee => write!(f, "River Gee"),
            Self::Sinoe => write!(f, "Sinoe"),
        }
    }
}

// Libya Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibyaSubregions {
    Libya,
    Tripolitania,
    Cyrenaica,
    Fezzan,
}

impl TryFrom<u8> for LibyaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Libya),
            0x02 => Ok(Self::Tripolitania),
            0x03 => Ok(Self::Cyrenaica),
            0x04 => Ok(Self::Fezzan),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LibyaSubregions> for u8 {
    fn from(value: LibyaSubregions) -> u8 {
        match value {
            LibyaSubregions::Libya => 0x01,
            LibyaSubregions::Tripolitania => 0x02,
            LibyaSubregions::Cyrenaica => 0x03,
            LibyaSubregions::Fezzan => 0x04,
        }
    }
}

impl Display for LibyaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Libya => write!(f, "Libya"),
            Self::Tripolitania => write!(f, "Tripolitania"),
            Self::Cyrenaica => write!(f, "Cyrenaica"),
            Self::Fezzan => write!(f, "Fezzan"),
        }
    }
}

// Liechtenstein Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiechtensteinSubregions {
    Liechtenstein,
    UpperCountry,
    LowerCountry,
}

impl TryFrom<u8> for LiechtensteinSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Liechtenstein),
            0x02 => Ok(Self::UpperCountry),
            0x03 => Ok(Self::LowerCountry),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LiechtensteinSubregions> for u8 {
    fn from(value: LiechtensteinSubregions) -> u8 {
        match value {
            LiechtensteinSubregions::Liechtenstein => 0x01,
            LiechtensteinSubregions::UpperCountry => 0x02,
            LiechtensteinSubregions::LowerCountry => 0x03,
        }
    }
}

impl Display for LiechtensteinSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Liechtenstein => write!(f, "Liechtenstein"),
            Self::UpperCountry => write!(f, "Upper Country"),
            Self::LowerCountry => write!(f, "Lower Country"),
        }
    }
}

// Lithuania Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LithuaniaSubregions {
    Lithuania,
    Vilnius,
    Alytus,
    Kaunas,
    Klaipeda,
    Marijampole,
    Panevezys,
    Šiauliai,
    Taurage,
    Telšiai,
    Utena,
}

impl TryFrom<u8> for LithuaniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Lithuania),
            0x02 => Ok(Self::Vilnius),
            0x03 => Ok(Self::Alytus),
            0x04 => Ok(Self::Kaunas),
            0x05 => Ok(Self::Klaipeda),
            0x06 => Ok(Self::Marijampole),
            0x07 => Ok(Self::Panevezys),
            0x08 => Ok(Self::Šiauliai),
            0x09 => Ok(Self::Taurage),
            0x0A => Ok(Self::Telšiai),
            0x0B => Ok(Self::Utena),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LithuaniaSubregions> for u8 {
    fn from(value: LithuaniaSubregions) -> u8 {
        match value {
            LithuaniaSubregions::Lithuania => 0x01,
            LithuaniaSubregions::Vilnius => 0x02,
            LithuaniaSubregions::Alytus => 0x03,
            LithuaniaSubregions::Kaunas => 0x04,
            LithuaniaSubregions::Klaipeda => 0x05,
            LithuaniaSubregions::Marijampole => 0x06,
            LithuaniaSubregions::Panevezys => 0x07,
            LithuaniaSubregions::Šiauliai => 0x08,
            LithuaniaSubregions::Taurage => 0x09,
            LithuaniaSubregions::Telšiai => 0x0A,
            LithuaniaSubregions::Utena => 0x0B,
        }
    }
}

impl Display for LithuaniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lithuania => write!(f, "Lithuania"),
            Self::Vilnius => write!(f, "Vilnius"),
            Self::Alytus => write!(f, "Alytus"),
            Self::Kaunas => write!(f, "Kaunas"),
            Self::Klaipeda => write!(f, "Klaipeda"),
            Self::Marijampole => write!(f, "Marijampole"),
            Self::Panevezys => write!(f, "Panevezys"),
            Self::Šiauliai => write!(f, "Šiauliai"),
            Self::Taurage => write!(f, "Taurage"),
            Self::Telšiai => write!(f, "Telšiai"),
            Self::Utena => write!(f, "Utena"),
        }
    }
}

// Luxembourg Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LuxembourgSubregions {
    Luxembourg,
    Diekirch,
    Grevenmacher,
}

impl TryFrom<u8> for LuxembourgSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Luxembourg),
            0x02 => Ok(Self::Luxembourg),
            0x03 => Ok(Self::Diekirch),
            0x04 => Ok(Self::Grevenmacher),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<LuxembourgSubregions> for u8 {
    fn from(value: LuxembourgSubregions) -> u8 {
        match value {
            LuxembourgSubregions::Luxembourg => 0x01,
            LuxembourgSubregions::Diekirch => 0x03,
            LuxembourgSubregions::Grevenmacher => 0x04,
        }
    }
}

impl Display for LuxembourgSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Luxembourg => write!(f, "Luxembourg"),
            Self::Diekirch => write!(f, "Diekirch"),
            Self::Grevenmacher => write!(f, "Grevenmacher"),
        }
    }
}

// Macao Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MacaoSubregions {
    Macao,
}

impl TryFrom<u8> for MacaoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Macao),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MacaoSubregions> for u8 {
    fn from(value: MacaoSubregions) -> u8 {
        match value {
            MacaoSubregions::Macao => 0x01,
        }
    }
}

impl Display for MacaoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Macao => write!(f, "Macao"),
        }
    }
}

// Madagascar Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MadagascarSubregions {
    Madagascar,
    Analamanga,
    AlaotraMangoro,
    AmoroniMania,
    Analanjirofo,
    Androy,
    Anosy,
    AtsimoAndrefana,
    AtsimoAtsinanana,
    Atsinanana,
    Betsiboka,
    Boeny,
    Bongolava,
    Diana,
    HauteMatsiatra,
    Ihorombe,
    Itasy,
    Melaky,
    Menabe,
    Sava,
    Sofia,
    Vakinankaratra,
    VatovavyFitovinany,
}

impl TryFrom<u8> for MadagascarSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Madagascar),
            0x02 => Ok(Self::Analamanga),
            0x03 => Ok(Self::AlaotraMangoro),
            0x04 => Ok(Self::AmoroniMania),
            0x05 => Ok(Self::Analanjirofo),
            0x06 => Ok(Self::Androy),
            0x07 => Ok(Self::Anosy),
            0x08 => Ok(Self::AtsimoAndrefana),
            0x09 => Ok(Self::AtsimoAtsinanana),
            0x0A => Ok(Self::Atsinanana),
            0x0B => Ok(Self::Betsiboka),
            0x0C => Ok(Self::Boeny),
            0x0D => Ok(Self::Bongolava),
            0x0E => Ok(Self::Diana),
            0x0F => Ok(Self::HauteMatsiatra),
            0x10 => Ok(Self::Ihorombe),
            0x11 => Ok(Self::Itasy),
            0x12 => Ok(Self::Melaky),
            0x13 => Ok(Self::Menabe),
            0x14 => Ok(Self::Sava),
            0x15 => Ok(Self::Sofia),
            0x16 => Ok(Self::Vakinankaratra),
            0x17 => Ok(Self::VatovavyFitovinany),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MadagascarSubregions> for u8 {
    fn from(value: MadagascarSubregions) -> u8 {
        match value {
            MadagascarSubregions::Madagascar => 0x01,
            MadagascarSubregions::Analamanga => 0x02,
            MadagascarSubregions::AlaotraMangoro => 0x03,
            MadagascarSubregions::AmoroniMania => 0x04,
            MadagascarSubregions::Analanjirofo => 0x05,
            MadagascarSubregions::Androy => 0x06,
            MadagascarSubregions::Anosy => 0x07,
            MadagascarSubregions::AtsimoAndrefana => 0x08,
            MadagascarSubregions::AtsimoAtsinanana => 0x09,
            MadagascarSubregions::Atsinanana => 0x0A,
            MadagascarSubregions::Betsiboka => 0x0B,
            MadagascarSubregions::Boeny => 0x0C,
            MadagascarSubregions::Bongolava => 0x0D,
            MadagascarSubregions::Diana => 0x0E,
            MadagascarSubregions::HauteMatsiatra => 0x0F,
            MadagascarSubregions::Ihorombe => 0x10,
            MadagascarSubregions::Itasy => 0x11,
            MadagascarSubregions::Melaky => 0x12,
            MadagascarSubregions::Menabe => 0x13,
            MadagascarSubregions::Sava => 0x14,
            MadagascarSubregions::Sofia => 0x15,
            MadagascarSubregions::Vakinankaratra => 0x16,
            MadagascarSubregions::VatovavyFitovinany => 0x17,
        }
    }
}

impl Display for MadagascarSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Madagascar => write!(f, "Madagascar"),
            Self::Analamanga => write!(f, "Analamanga"),
            Self::AlaotraMangoro => write!(f, "Alaotra-Mangoro"),
            Self::AmoroniMania => write!(f, "Amoron'i Mania"),
            Self::Analanjirofo => write!(f, "Analanjirofo"),
            Self::Androy => write!(f, "Androy"),
            Self::Anosy => write!(f, "Anosy"),
            Self::AtsimoAndrefana => write!(f, "Atsimo-Andrefana"),
            Self::AtsimoAtsinanana => write!(f, "Atsimo-Atsinanana"),
            Self::Atsinanana => write!(f, "Atsinanana"),
            Self::Betsiboka => write!(f, "Betsiboka"),
            Self::Boeny => write!(f, "Boeny"),
            Self::Bongolava => write!(f, "Bongolava"),
            Self::Diana => write!(f, "Diana"),
            Self::HauteMatsiatra => write!(f, "Haute Matsiatra"),
            Self::Ihorombe => write!(f, "Ihorombe"),
            Self::Itasy => write!(f, "Itasy"),
            Self::Melaky => write!(f, "Melaky"),
            Self::Menabe => write!(f, "Menabe"),
            Self::Sava => write!(f, "Sava"),
            Self::Sofia => write!(f, "Sofia"),
            Self::Vakinankaratra => write!(f, "Vakinankaratra"),
            Self::VatovavyFitovinany => write!(f, "Vatovavy-Fitovinany"),
        }
    }
}

// Malawi Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MalawiSubregions {
    Malawi,
    CentralRegion,
    NorthernRegion,
    SouthernRegion,
}

impl TryFrom<u8> for MalawiSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Malawi),
            0x02 => Ok(Self::CentralRegion),
            0x03 => Ok(Self::NorthernRegion),
            0x04 => Ok(Self::SouthernRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MalawiSubregions> for u8 {
    fn from(value: MalawiSubregions) -> u8 {
        match value {
            MalawiSubregions::Malawi => 0x01,
            MalawiSubregions::CentralRegion => 0x02,
            MalawiSubregions::NorthernRegion => 0x03,
            MalawiSubregions::SouthernRegion => 0x04,
        }
    }
}

impl Display for MalawiSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malawi => write!(f, "Malawi"),
            Self::CentralRegion => write!(f, "Central Region"),
            Self::NorthernRegion => write!(f, "Northern Region"),
            Self::SouthernRegion => write!(f, "Southern Region"),
        }
    }
}

// Malaysia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MalaysiaSubregions {
    Malaysia,
    KualaLumpur,
    Johor,
    Kedah,
    Kelantan,
    Melaka,
    NegeriSembilan,
    Pahang,
    Perak,
    Perlis,
    Penang,
    Sarawak,
    Selangor,
    Terengganu,
    Labuan,
    Sabah,
    Putrajaya,
}

impl TryFrom<u8> for MalaysiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Malaysia),
            0x02 => Ok(Self::KualaLumpur),
            0x03 => Ok(Self::Johor),
            0x04 => Ok(Self::Kedah),
            0x05 => Ok(Self::Kelantan),
            0x06 => Ok(Self::Melaka),
            0x07 => Ok(Self::NegeriSembilan),
            0x08 => Ok(Self::Pahang),
            0x09 => Ok(Self::Perak),
            0x0A => Ok(Self::Perlis),
            0x0B => Ok(Self::Penang),
            0x0C => Ok(Self::Sarawak),
            0x0D => Ok(Self::Selangor),
            0x0E => Ok(Self::Terengganu),
            0x0F => Ok(Self::Labuan),
            0x10 => Ok(Self::Sabah),
            0x11 => Ok(Self::Putrajaya),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MalaysiaSubregions> for u8 {
    fn from(value: MalaysiaSubregions) -> u8 {
        match value {
            MalaysiaSubregions::Malaysia => 0x01,
            MalaysiaSubregions::KualaLumpur => 0x02,
            MalaysiaSubregions::Johor => 0x03,
            MalaysiaSubregions::Kedah => 0x04,
            MalaysiaSubregions::Kelantan => 0x05,
            MalaysiaSubregions::Melaka => 0x06,
            MalaysiaSubregions::NegeriSembilan => 0x07,
            MalaysiaSubregions::Pahang => 0x08,
            MalaysiaSubregions::Perak => 0x09,
            MalaysiaSubregions::Perlis => 0x0A,
            MalaysiaSubregions::Penang => 0x0B,
            MalaysiaSubregions::Sarawak => 0x0C,
            MalaysiaSubregions::Selangor => 0x0D,
            MalaysiaSubregions::Terengganu => 0x0E,
            MalaysiaSubregions::Labuan => 0x0F,
            MalaysiaSubregions::Sabah => 0x10,
            MalaysiaSubregions::Putrajaya => 0x11,
        }
    }
}

impl Display for MalaysiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malaysia => write!(f, "Malaysia"),
            Self::KualaLumpur => write!(f, "Kuala Lumpur"),
            Self::Johor => write!(f, "Johor"),
            Self::Kedah => write!(f, "Kedah"),
            Self::Kelantan => write!(f, "Kelantan"),
            Self::Melaka => write!(f, "Melaka"),
            Self::NegeriSembilan => write!(f, "Negeri Sembilan"),
            Self::Pahang => write!(f, "Pahang"),
            Self::Perak => write!(f, "Perak"),
            Self::Perlis => write!(f, "Perlis"),
            Self::Penang => write!(f, "Penang"),
            Self::Sarawak => write!(f, "Sarawak"),
            Self::Selangor => write!(f, "Selangor"),
            Self::Terengganu => write!(f, "Terengganu"),
            Self::Labuan => write!(f, "Labuan"),
            Self::Sabah => write!(f, "Sabah"),
            Self::Putrajaya => write!(f, "Putrajaya"),
        }
    }
}

// Maldives Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaldivesSubregions {
    Maldives,
    NorthCentral,
    Central,
    North,
    SouthCentral,
    South,
    UpperNorth,
    UpperSouth,
}

impl TryFrom<u8> for MaldivesSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Maldives),
            0x02 => Ok(Self::NorthCentral),
            0x03 => Ok(Self::Central),
            0x04 => Ok(Self::North),
            0x05 => Ok(Self::SouthCentral),
            0x06 => Ok(Self::South),
            0x07 => Ok(Self::UpperNorth),
            0x08 => Ok(Self::UpperSouth),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MaldivesSubregions> for u8 {
    fn from(value: MaldivesSubregions) -> u8 {
        match value {
            MaldivesSubregions::Maldives => 0x01,
            MaldivesSubregions::NorthCentral => 0x02,
            MaldivesSubregions::Central => 0x03,
            MaldivesSubregions::North => 0x04,
            MaldivesSubregions::SouthCentral => 0x05,
            MaldivesSubregions::South => 0x06,
            MaldivesSubregions::UpperNorth => 0x07,
            MaldivesSubregions::UpperSouth => 0x08,
        }
    }
}

impl Display for MaldivesSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Maldives => write!(f, "Maldives"),
            Self::NorthCentral => write!(f, "North Central"),
            Self::Central => write!(f, "Central"),
            Self::North => write!(f, "North"),
            Self::SouthCentral => write!(f, "South Central"),
            Self::South => write!(f, "South"),
            Self::UpperNorth => write!(f, "Upper North"),
            Self::UpperSouth => write!(f, "Upper South"),
        }
    }
}

// Mali Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaliSubregions {
    Mali,
    Bamako,
    GaoRegion,
    KayesRegion,
    KidalRegion,
    KoulikoroRegion,
    MenakaRegion,
    MoptiRegion,
    SegouRegion,
    SikassoRegion,
    TaoudenitRegion,
    TombouctouRegion,
}

impl TryFrom<u8> for MaliSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Mali),
            0x02 => Ok(Self::Bamako),
            0x03 => Ok(Self::GaoRegion),
            0x04 => Ok(Self::KayesRegion),
            0x05 => Ok(Self::KidalRegion),
            0x06 => Ok(Self::KoulikoroRegion),
            0x07 => Ok(Self::MenakaRegion),
            0x08 => Ok(Self::MoptiRegion),
            0x09 => Ok(Self::SegouRegion),
            0x0A => Ok(Self::SikassoRegion),
            0x0B => Ok(Self::TaoudenitRegion),
            0x0C => Ok(Self::TombouctouRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MaliSubregions> for u8 {
    fn from(value: MaliSubregions) -> u8 {
        match value {
            MaliSubregions::Mali => 0x01,
            MaliSubregions::Bamako => 0x02,
            MaliSubregions::GaoRegion => 0x03,
            MaliSubregions::KayesRegion => 0x04,
            MaliSubregions::KidalRegion => 0x05,
            MaliSubregions::KoulikoroRegion => 0x06,
            MaliSubregions::MenakaRegion => 0x07,
            MaliSubregions::MoptiRegion => 0x08,
            MaliSubregions::SegouRegion => 0x09,
            MaliSubregions::SikassoRegion => 0x0A,
            MaliSubregions::TaoudenitRegion => 0x0B,
            MaliSubregions::TombouctouRegion => 0x0C,
        }
    }
}

impl Display for MaliSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mali => write!(f, "Mali"),
            Self::Bamako => write!(f, "Bamako"),
            Self::GaoRegion => write!(f, "Gao Region"),
            Self::KayesRegion => write!(f, "Kayes Region"),
            Self::KidalRegion => write!(f, "Kidal Region"),
            Self::KoulikoroRegion => write!(f, "Koulikoro Region"),
            Self::MenakaRegion => write!(f, "Ménaka Region"),
            Self::MoptiRegion => write!(f, "Mopti Region"),
            Self::SegouRegion => write!(f, "Ségou Region"),
            Self::SikassoRegion => write!(f, "Sikasso Region"),
            Self::TaoudenitRegion => write!(f, "Taoudénit Region"),
            Self::TombouctouRegion => write!(f, "Tombouctou Region"),
        }
    }
}

// Malta Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaltaSubregions {
    Malta,
    GozoAndComino,
}

impl TryFrom<u8> for MaltaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Malta),
            0x02 => Ok(Self::Malta),
            0x03 => Ok(Self::GozoAndComino),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MaltaSubregions> for u8 {
    fn from(value: MaltaSubregions) -> u8 {
        match value {
            MaltaSubregions::Malta => 0x01,
            MaltaSubregions::GozoAndComino => 0x03,
        }
    }
}

impl Display for MaltaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malta => write!(f, "Malta"),
            Self::GozoAndComino => write!(f, "Gozo and Comino"),
        }
    }
}

// Marshall Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarshallIslandsSubregions {
    MarshallIslands,
    RatakChain,
    RalikChain,
}

impl TryFrom<u8> for MarshallIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::MarshallIslands),
            0x02 => Ok(Self::RatakChain),
            0x03 => Ok(Self::RalikChain),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MarshallIslandsSubregions> for u8 {
    fn from(value: MarshallIslandsSubregions) -> u8 {
        match value {
            MarshallIslandsSubregions::MarshallIslands => 0x01,
            MarshallIslandsSubregions::RatakChain => 0x02,
            MarshallIslandsSubregions::RalikChain => 0x03,
        }
    }
}

impl Display for MarshallIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MarshallIslands => write!(f, "Marshall Islands"),
            Self::RatakChain => write!(f, "Ratak Chain"),
            Self::RalikChain => write!(f, "Ralik Chain"),
        }
    }
}

// Martinique Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MartiniqueSubregions {
    Martinique,
}

impl TryFrom<u8> for MartiniqueSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Martinique),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MartiniqueSubregions> for u8 {
    fn from(value: MartiniqueSubregions) -> u8 {
        match value {
            MartiniqueSubregions::Martinique => 0x01,
        }
    }
}

impl Display for MartiniqueSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Martinique => write!(f, "Martinique"),
        }
    }
}

// Mauritania Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MauritaniaSubregions {
    Mauritania,
    Nouakchott,
    AdrarRegion,
    AssabaRegion,
    BraknaRegion,
    DakhletNouadhibouRegion,
    GorgolRegion,
    GuidimakaRegion,
    HodhEchCharguiRegion,
    HodhElGharbiRegion,
    InchiriRegion,
    TagantRegion,
    TirisZemmourRegion,
    TrarzaRegion,
}

impl TryFrom<u8> for MauritaniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Mauritania),
            0x02 => Ok(Self::Nouakchott),
            0x03 => Ok(Self::AdrarRegion),
            0x04 => Ok(Self::AssabaRegion),
            0x05 => Ok(Self::BraknaRegion),
            0x06 => Ok(Self::DakhletNouadhibouRegion),
            0x07 => Ok(Self::GorgolRegion),
            0x08 => Ok(Self::GuidimakaRegion),
            0x09 => Ok(Self::HodhEchCharguiRegion),
            0x0A => Ok(Self::HodhElGharbiRegion),
            0x0B => Ok(Self::InchiriRegion),
            0x0C => Ok(Self::TagantRegion),
            0x0D => Ok(Self::TirisZemmourRegion),
            0x0E => Ok(Self::TrarzaRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MauritaniaSubregions> for u8 {
    fn from(value: MauritaniaSubregions) -> u8 {
        match value {
            MauritaniaSubregions::Mauritania => 0x01,
            MauritaniaSubregions::Nouakchott => 0x02,
            MauritaniaSubregions::AdrarRegion => 0x03,
            MauritaniaSubregions::AssabaRegion => 0x04,
            MauritaniaSubregions::BraknaRegion => 0x05,
            MauritaniaSubregions::DakhletNouadhibouRegion => 0x06,
            MauritaniaSubregions::GorgolRegion => 0x07,
            MauritaniaSubregions::GuidimakaRegion => 0x08,
            MauritaniaSubregions::HodhEchCharguiRegion => 0x09,
            MauritaniaSubregions::HodhElGharbiRegion => 0x0A,
            MauritaniaSubregions::InchiriRegion => 0x0B,
            MauritaniaSubregions::TagantRegion => 0x0C,
            MauritaniaSubregions::TirisZemmourRegion => 0x0D,
            MauritaniaSubregions::TrarzaRegion => 0x0E,
        }
    }
}

impl Display for MauritaniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mauritania => write!(f, "Mauritania"),
            Self::Nouakchott => write!(f, "Nouakchott"),
            Self::AdrarRegion => write!(f, "Adrar Region"),
            Self::AssabaRegion => write!(f, "Assaba Region"),
            Self::BraknaRegion => write!(f, "Brakna Region"),
            Self::DakhletNouadhibouRegion => write!(f, "Dakhlet Nouadhibou Region"),
            Self::GorgolRegion => write!(f, "Gorgol Region"),
            Self::GuidimakaRegion => write!(f, "Guidimaka Region"),
            Self::HodhEchCharguiRegion => write!(f, "Hodh Ech Chargui Region"),
            Self::HodhElGharbiRegion => write!(f, "Hodh El Gharbi Region"),
            Self::InchiriRegion => write!(f, "Inchiri Region"),
            Self::TagantRegion => write!(f, "Tagant Region"),
            Self::TirisZemmourRegion => write!(f, "Tiris Zemmour Region"),
            Self::TrarzaRegion => write!(f, "Trarza Region"),
        }
    }
}

// Mauritius Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MauritiusSubregions {
    Mauritius,
}

impl TryFrom<u8> for MauritiusSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Mauritius),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MauritiusSubregions> for u8 {
    fn from(value: MauritiusSubregions) -> u8 {
        match value {
            MauritiusSubregions::Mauritius => 0x01,
        }
    }
}

impl Display for MauritiusSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mauritius => write!(f, "Mauritius"),
        }
    }
}

// Mayotte Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MayotteSubregions {
    Mayotte,
}

impl TryFrom<u8> for MayotteSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Mayotte),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MayotteSubregions> for u8 {
    fn from(value: MayotteSubregions) -> u8 {
        match value {
            MayotteSubregions::Mayotte => 0x01,
        }
    }
}

impl Display for MayotteSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mayotte => write!(f, "Mayotte"),
        }
    }
}

// Mexico Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MexicoSubregions {
    Mexico,
    DistritoFederal,
    Aguascalientes,
    BajaCalifornia,
    BajaCaliforniaSur,
    Campeche,
    Chiapas,
    Chihuahua,
    CoahuilaDeZaragoza,
    Colima,
    Durango,
    Guanajuato,
    Guerrero,
    Hidalgo,
    Jalisco,
    MichoacanDeOcampo,
    Morelos,
    Nayarit,
    NuevoLeon,
    Oaxaca,
    Puebla,
    QueretaroDeArteaga,
    QuintanaRoo,
    SanLuisPotosi,
    Sinaloa,
    Sonora,
    Tabasco,
    Tamaulipas,
    Tlaxcala,
    VeracruzLlave,
    Yucatan,
    Zacatecas,
}

impl TryFrom<u8> for MexicoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Mexico),
            0x02 => Ok(Self::DistritoFederal),
            0x03 => Ok(Self::Aguascalientes),
            0x04 => Ok(Self::BajaCalifornia),
            0x05 => Ok(Self::BajaCaliforniaSur),
            0x06 => Ok(Self::Campeche),
            0x07 => Ok(Self::Chiapas),
            0x08 => Ok(Self::Chihuahua),
            0x09 => Ok(Self::CoahuilaDeZaragoza),
            0x0A => Ok(Self::Colima),
            0x0B => Ok(Self::Durango),
            0x0C => Ok(Self::Guanajuato),
            0x0D => Ok(Self::Guerrero),
            0x0E => Ok(Self::Hidalgo),
            0x0F => Ok(Self::Jalisco),
            0x10 => Ok(Self::Mexico),
            0x11 => Ok(Self::MichoacanDeOcampo),
            0x12 => Ok(Self::Morelos),
            0x13 => Ok(Self::Nayarit),
            0x14 => Ok(Self::NuevoLeon),
            0x15 => Ok(Self::Oaxaca),
            0x16 => Ok(Self::Puebla),
            0x17 => Ok(Self::QueretaroDeArteaga),
            0x18 => Ok(Self::QuintanaRoo),
            0x19 => Ok(Self::SanLuisPotosi),
            0x1A => Ok(Self::Sinaloa),
            0x1B => Ok(Self::Sonora),
            0x1C => Ok(Self::Tabasco),
            0x1D => Ok(Self::Tamaulipas),
            0x1E => Ok(Self::Tlaxcala),
            0x1F => Ok(Self::VeracruzLlave),
            0x20 => Ok(Self::Yucatan),
            0x21 => Ok(Self::Zacatecas),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MexicoSubregions> for u8 {
    fn from(value: MexicoSubregions) -> u8 {
        match value {
            MexicoSubregions::Mexico => 0x01,
            MexicoSubregions::DistritoFederal => 0x02,
            MexicoSubregions::Aguascalientes => 0x03,
            MexicoSubregions::BajaCalifornia => 0x04,
            MexicoSubregions::BajaCaliforniaSur => 0x05,
            MexicoSubregions::Campeche => 0x06,
            MexicoSubregions::Chiapas => 0x07,
            MexicoSubregions::Chihuahua => 0x08,
            MexicoSubregions::CoahuilaDeZaragoza => 0x09,
            MexicoSubregions::Colima => 0x0A,
            MexicoSubregions::Durango => 0x0B,
            MexicoSubregions::Guanajuato => 0x0C,
            MexicoSubregions::Guerrero => 0x0D,
            MexicoSubregions::Hidalgo => 0x0E,
            MexicoSubregions::Jalisco => 0x0F,
            MexicoSubregions::MichoacanDeOcampo => 0x11,
            MexicoSubregions::Morelos => 0x12,
            MexicoSubregions::Nayarit => 0x13,
            MexicoSubregions::NuevoLeon => 0x14,
            MexicoSubregions::Oaxaca => 0x15,
            MexicoSubregions::Puebla => 0x16,
            MexicoSubregions::QueretaroDeArteaga => 0x17,
            MexicoSubregions::QuintanaRoo => 0x18,
            MexicoSubregions::SanLuisPotosi => 0x19,
            MexicoSubregions::Sinaloa => 0x1A,
            MexicoSubregions::Sonora => 0x1B,
            MexicoSubregions::Tabasco => 0x1C,
            MexicoSubregions::Tamaulipas => 0x1D,
            MexicoSubregions::Tlaxcala => 0x1E,
            MexicoSubregions::VeracruzLlave => 0x1F,
            MexicoSubregions::Yucatan => 0x20,
            MexicoSubregions::Zacatecas => 0x21,
        }
    }
}

impl Display for MexicoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mexico => write!(f, "México"),
            Self::DistritoFederal => write!(f, "Distrito Federal"),
            Self::Aguascalientes => write!(f, "Aguascalientes"),
            Self::BajaCalifornia => write!(f, "Baja California"),
            Self::BajaCaliforniaSur => write!(f, "Baja California Sur"),
            Self::Campeche => write!(f, "Campeche"),
            Self::Chiapas => write!(f, "Chiapas"),
            Self::Chihuahua => write!(f, "Chihuahua"),
            Self::CoahuilaDeZaragoza => write!(f, "Coahuila de Zaragoza"),
            Self::Colima => write!(f, "Colima"),
            Self::Durango => write!(f, "Durango"),
            Self::Guanajuato => write!(f, "Guanajuato"),
            Self::Guerrero => write!(f, "Guerrero"),
            Self::Hidalgo => write!(f, "Hidalgo"),
            Self::Jalisco => write!(f, "Jalisco"),
            Self::MichoacanDeOcampo => write!(f, "Michoacán de Ocampo"),
            Self::Morelos => write!(f, "Morelos"),
            Self::Nayarit => write!(f, "Nayarit"),
            Self::NuevoLeon => write!(f, "Nuevo León"),
            Self::Oaxaca => write!(f, "Oaxaca"),
            Self::Puebla => write!(f, "Puebla"),
            Self::QueretaroDeArteaga => write!(f, "Querétaro de Arteaga"),
            Self::QuintanaRoo => write!(f, "Quintana Roo"),
            Self::SanLuisPotosi => write!(f, "San Luis Potosí"),
            Self::Sinaloa => write!(f, "Sinaloa"),
            Self::Sonora => write!(f, "Sonora"),
            Self::Tabasco => write!(f, "Tabasco"),
            Self::Tamaulipas => write!(f, "Tamaulipas"),
            Self::Tlaxcala => write!(f, "Tlaxcala"),
            Self::VeracruzLlave => write!(f, "Veracruz-Llave"),
            Self::Yucatan => write!(f, "Yucatán"),
            Self::Zacatecas => write!(f, "Zacatecas"),
        }
    }
}

// Moldova Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoldovaSubregions {
    Moldova,
    Chisinau,
    AneniiNoiDistrict,
    Balti,
    BasarabeascaDistrict,
    Bender,
    BriceniDistrict,
    CahulDistrict,
    CalarasiDistrict,
    CantemirDistrict,
    CauseniDistrict,
    CimisliaDistrict,
    CriuleniDistrict,
    DonduseniDistrict,
    DrochiaDistrict,
    DubasariDistrict,
    EdinetDistrict,
    FalestiDistrict,
    FlorestiDistrict,
    Gagauzia,
    GlodeniDistrict,
    HincestiDistrict,
    IaloveniDistrict,
    LeovaDistrict,
    NisporeniDistrict,
    OcnitaDistrict,
    OrheiDistrict,
    RezinaDistrict,
    RiscaniDistrict,
    SingereiDistrict,
    SoldanestiDistrict,
    SorocaDistrict,
    StefanVodaDistrict,
    StraseniDistrict,
    TaracliaDistrict,
    TelenestiDistrict,
    Transnistria,
    UngheniDistrict,
}

impl TryFrom<u8> for MoldovaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Moldova),
            0x02 => Ok(Self::Chisinau),
            0x03 => Ok(Self::AneniiNoiDistrict),
            0x04 => Ok(Self::Balti),
            0x05 => Ok(Self::BasarabeascaDistrict),
            0x06 => Ok(Self::Bender),
            0x07 => Ok(Self::BriceniDistrict),
            0x08 => Ok(Self::CahulDistrict),
            0x09 => Ok(Self::CalarasiDistrict),
            0x0A => Ok(Self::CantemirDistrict),
            0x0B => Ok(Self::CauseniDistrict),
            0x0C => Ok(Self::CimisliaDistrict),
            0x0D => Ok(Self::CriuleniDistrict),
            0x0E => Ok(Self::DonduseniDistrict),
            0x0F => Ok(Self::DrochiaDistrict),
            0x10 => Ok(Self::DubasariDistrict),
            0x11 => Ok(Self::EdinetDistrict),
            0x12 => Ok(Self::FalestiDistrict),
            0x13 => Ok(Self::FlorestiDistrict),
            0x14 => Ok(Self::Gagauzia),
            0x15 => Ok(Self::GlodeniDistrict),
            0x16 => Ok(Self::HincestiDistrict),
            0x17 => Ok(Self::IaloveniDistrict),
            0x18 => Ok(Self::LeovaDistrict),
            0x19 => Ok(Self::NisporeniDistrict),
            0x1A => Ok(Self::OcnitaDistrict),
            0x1B => Ok(Self::OrheiDistrict),
            0x1C => Ok(Self::RezinaDistrict),
            0x1D => Ok(Self::RiscaniDistrict),
            0x1E => Ok(Self::SingereiDistrict),
            0x1F => Ok(Self::SoldanestiDistrict),
            0x20 => Ok(Self::SorocaDistrict),
            0x21 => Ok(Self::StefanVodaDistrict),
            0x22 => Ok(Self::StraseniDistrict),
            0x23 => Ok(Self::TaracliaDistrict),
            0x24 => Ok(Self::TelenestiDistrict),
            0x25 => Ok(Self::Transnistria),
            0x26 => Ok(Self::UngheniDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MoldovaSubregions> for u8 {
    fn from(value: MoldovaSubregions) -> u8 {
        match value {
            MoldovaSubregions::Moldova => 0x01,
            MoldovaSubregions::Chisinau => 0x02,
            MoldovaSubregions::AneniiNoiDistrict => 0x03,
            MoldovaSubregions::Balti => 0x04,
            MoldovaSubregions::BasarabeascaDistrict => 0x05,
            MoldovaSubregions::Bender => 0x06,
            MoldovaSubregions::BriceniDistrict => 0x07,
            MoldovaSubregions::CahulDistrict => 0x08,
            MoldovaSubregions::CalarasiDistrict => 0x09,
            MoldovaSubregions::CantemirDistrict => 0x0A,
            MoldovaSubregions::CauseniDistrict => 0x0B,
            MoldovaSubregions::CimisliaDistrict => 0x0C,
            MoldovaSubregions::CriuleniDistrict => 0x0D,
            MoldovaSubregions::DonduseniDistrict => 0x0E,
            MoldovaSubregions::DrochiaDistrict => 0x0F,
            MoldovaSubregions::DubasariDistrict => 0x10,
            MoldovaSubregions::EdinetDistrict => 0x11,
            MoldovaSubregions::FalestiDistrict => 0x12,
            MoldovaSubregions::FlorestiDistrict => 0x13,
            MoldovaSubregions::Gagauzia => 0x14,
            MoldovaSubregions::GlodeniDistrict => 0x15,
            MoldovaSubregions::HincestiDistrict => 0x16,
            MoldovaSubregions::IaloveniDistrict => 0x17,
            MoldovaSubregions::LeovaDistrict => 0x18,
            MoldovaSubregions::NisporeniDistrict => 0x19,
            MoldovaSubregions::OcnitaDistrict => 0x1A,
            MoldovaSubregions::OrheiDistrict => 0x1B,
            MoldovaSubregions::RezinaDistrict => 0x1C,
            MoldovaSubregions::RiscaniDistrict => 0x1D,
            MoldovaSubregions::SingereiDistrict => 0x1E,
            MoldovaSubregions::SoldanestiDistrict => 0x1F,
            MoldovaSubregions::SorocaDistrict => 0x20,
            MoldovaSubregions::StefanVodaDistrict => 0x21,
            MoldovaSubregions::StraseniDistrict => 0x22,
            MoldovaSubregions::TaracliaDistrict => 0x23,
            MoldovaSubregions::TelenestiDistrict => 0x24,
            MoldovaSubregions::Transnistria => 0x25,
            MoldovaSubregions::UngheniDistrict => 0x26,
        }
    }
}

impl Display for MoldovaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Moldova => write!(f, "Moldova"),
            Self::Chisinau => write!(f, "Chisinau"),
            Self::AneniiNoiDistrict => write!(f, "Anenii Noi District"),
            Self::Balti => write!(f, "Balti"),
            Self::BasarabeascaDistrict => write!(f, "Basarabeasca District"),
            Self::Bender => write!(f, "Bender"),
            Self::BriceniDistrict => write!(f, "Briceni District"),
            Self::CahulDistrict => write!(f, "Cahul District"),
            Self::CalarasiDistrict => write!(f, "Calarasi District"),
            Self::CantemirDistrict => write!(f, "Cantemir District"),
            Self::CauseniDistrict => write!(f, "Causeni District"),
            Self::CimisliaDistrict => write!(f, "Cimislia District"),
            Self::CriuleniDistrict => write!(f, "Criuleni District"),
            Self::DonduseniDistrict => write!(f, "Donduseni District"),
            Self::DrochiaDistrict => write!(f, "Drochia District"),
            Self::DubasariDistrict => write!(f, "Dubasari District"),
            Self::EdinetDistrict => write!(f, "Edinet District"),
            Self::FalestiDistrict => write!(f, "Falesti District"),
            Self::FlorestiDistrict => write!(f, "Floresti District"),
            Self::Gagauzia => write!(f, "Gagauzia"),
            Self::GlodeniDistrict => write!(f, "Glodeni District"),
            Self::HincestiDistrict => write!(f, "Hîncesti District"),
            Self::IaloveniDistrict => write!(f, "Ialoveni District"),
            Self::LeovaDistrict => write!(f, "Leova District"),
            Self::NisporeniDistrict => write!(f, "Nisporeni District"),
            Self::OcnitaDistrict => write!(f, "Ocnita District"),
            Self::OrheiDistrict => write!(f, "Orhei District"),
            Self::RezinaDistrict => write!(f, "Rezina District"),
            Self::RiscaniDistrict => write!(f, "Rîscani District"),
            Self::SingereiDistrict => write!(f, "Sîngerei District"),
            Self::SoldanestiDistrict => write!(f, "Soldanesti District"),
            Self::SorocaDistrict => write!(f, "Soroca District"),
            Self::StefanVodaDistrict => write!(f, "Stefan Voda District"),
            Self::StraseniDistrict => write!(f, "Straseni District"),
            Self::TaracliaDistrict => write!(f, "Taraclia District"),
            Self::TelenestiDistrict => write!(f, "Telenesti District"),
            Self::Transnistria => write!(f, "Transnistria"),
            Self::UngheniDistrict => write!(f, "Ungheni District"),
        }
    }
}

// Monaco Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MonacoSubregions {
    Monaco,
}

impl TryFrom<u8> for MonacoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Monaco),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MonacoSubregions> for u8 {
    fn from(value: MonacoSubregions) -> u8 {
        match value {
            MonacoSubregions::Monaco => 0x01,
        }
    }
}

impl Display for MonacoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Monaco => write!(f, "Monaco"),
        }
    }
}

// Mongolia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MongoliaSubregions {
    Mongolia,
    Ulaanbaatar,
    ArkhangaiProvince,
    BayanOlgiiProvince,
    BayankhongorProvince,
    BulganProvince,
    DarkhanUulProvince,
    DornodProvince,
    DornogoviProvince,
    DundgoviProvince,
    GoviAltaiProvince,
    GovisumberProvince,
    KhentiiProvince,
    KhovdProvince,
    KhovsgolProvince,
    OmnogoviProvince,
    OrkhonProvince,
    OvorkhangaiProvince,
    SelengeProvince,
    SukhbaatarProvince,
    TovProvince,
    UvsProvince,
    ZavkhanProvince,
}

impl TryFrom<u8> for MongoliaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Mongolia),
            0x02 => Ok(Self::Ulaanbaatar),
            0x03 => Ok(Self::ArkhangaiProvince),
            0x04 => Ok(Self::BayanOlgiiProvince),
            0x05 => Ok(Self::BayankhongorProvince),
            0x06 => Ok(Self::BulganProvince),
            0x07 => Ok(Self::DarkhanUulProvince),
            0x08 => Ok(Self::DornodProvince),
            0x09 => Ok(Self::DornogoviProvince),
            0x0A => Ok(Self::DundgoviProvince),
            0x0B => Ok(Self::GoviAltaiProvince),
            0x0C => Ok(Self::GovisumberProvince),
            0x0D => Ok(Self::KhentiiProvince),
            0x0E => Ok(Self::KhovdProvince),
            0x0F => Ok(Self::KhovsgolProvince),
            0x10 => Ok(Self::OmnogoviProvince),
            0x11 => Ok(Self::OrkhonProvince),
            0x12 => Ok(Self::OvorkhangaiProvince),
            0x13 => Ok(Self::SelengeProvince),
            0x14 => Ok(Self::SukhbaatarProvince),
            0x15 => Ok(Self::TovProvince),
            0x16 => Ok(Self::UvsProvince),
            0x17 => Ok(Self::ZavkhanProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MongoliaSubregions> for u8 {
    fn from(value: MongoliaSubregions) -> u8 {
        match value {
            MongoliaSubregions::Mongolia => 0x01,
            MongoliaSubregions::Ulaanbaatar => 0x02,
            MongoliaSubregions::ArkhangaiProvince => 0x03,
            MongoliaSubregions::BayanOlgiiProvince => 0x04,
            MongoliaSubregions::BayankhongorProvince => 0x05,
            MongoliaSubregions::BulganProvince => 0x06,
            MongoliaSubregions::DarkhanUulProvince => 0x07,
            MongoliaSubregions::DornodProvince => 0x08,
            MongoliaSubregions::DornogoviProvince => 0x09,
            MongoliaSubregions::DundgoviProvince => 0x0A,
            MongoliaSubregions::GoviAltaiProvince => 0x0B,
            MongoliaSubregions::GovisumberProvince => 0x0C,
            MongoliaSubregions::KhentiiProvince => 0x0D,
            MongoliaSubregions::KhovdProvince => 0x0E,
            MongoliaSubregions::KhovsgolProvince => 0x0F,
            MongoliaSubregions::OmnogoviProvince => 0x10,
            MongoliaSubregions::OrkhonProvince => 0x11,
            MongoliaSubregions::OvorkhangaiProvince => 0x12,
            MongoliaSubregions::SelengeProvince => 0x13,
            MongoliaSubregions::SukhbaatarProvince => 0x14,
            MongoliaSubregions::TovProvince => 0x15,
            MongoliaSubregions::UvsProvince => 0x16,
            MongoliaSubregions::ZavkhanProvince => 0x17,
        }
    }
}

impl Display for MongoliaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mongolia => write!(f, "Mongolia"),
            Self::Ulaanbaatar => write!(f, "Ulaanbaatar"),
            Self::ArkhangaiProvince => write!(f, "Arkhangai Province"),
            Self::BayanOlgiiProvince => write!(f, "Bayan-Ölgii Province"),
            Self::BayankhongorProvince => write!(f, "Bayankhongor Province"),
            Self::BulganProvince => write!(f, "Bulgan Province"),
            Self::DarkhanUulProvince => write!(f, "Darkhan-Uul Province"),
            Self::DornodProvince => write!(f, "Dornod Province"),
            Self::DornogoviProvince => write!(f, "Dornogovi Province"),
            Self::DundgoviProvince => write!(f, "Dundgovi Province"),
            Self::GoviAltaiProvince => write!(f, "Govi-Altai Province"),
            Self::GovisumberProvince => write!(f, "Govisümber Province"),
            Self::KhentiiProvince => write!(f, "Khentii Province"),
            Self::KhovdProvince => write!(f, "Khovd Province"),
            Self::KhovsgolProvince => write!(f, "Khövsgöl Province"),
            Self::OmnogoviProvince => write!(f, "Ömnögovi Province"),
            Self::OrkhonProvince => write!(f, "Orkhon Province"),
            Self::OvorkhangaiProvince => write!(f, "Övörkhangai Province"),
            Self::SelengeProvince => write!(f, "Selenge Province"),
            Self::SukhbaatarProvince => write!(f, "Sükhbaatar Province"),
            Self::TovProvince => write!(f, "Töv Province"),
            Self::UvsProvince => write!(f, "Uvs Province"),
            Self::ZavkhanProvince => write!(f, "Zavkhan Province"),
        }
    }
}

// Montenegro Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MontenegroSubregions {
    Montenegro,
    Podgorica,
    Andrijevica,
    Bar,
    Berane,
    BijeloPolje,
    Budva,
    Cetinje,
    Danilovgrad,
    HercegNovi,
    Kolasin,
    Kotor,
    Mojkovac,
    Niksic,
    Plav,
    Plužine,
    Pljevlja,
    Rozaje,
    Šavnik,
    Tivat,
    Ulcinj,
    Zabljak,
}

impl TryFrom<u8> for MontenegroSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Montenegro),
            0x02 => Ok(Self::Podgorica),
            0x03 => Ok(Self::Andrijevica),
            0x04 => Ok(Self::Bar),
            0x05 => Ok(Self::Berane),
            0x06 => Ok(Self::BijeloPolje),
            0x07 => Ok(Self::Budva),
            0x08 => Ok(Self::Cetinje),
            0x09 => Ok(Self::Danilovgrad),
            0x0A => Ok(Self::HercegNovi),
            0x0B => Ok(Self::Kolasin),
            0x0C => Ok(Self::Kotor),
            0x0D => Ok(Self::Mojkovac),
            0x0E => Ok(Self::Niksic),
            0x0F => Ok(Self::Plav),
            0x10 => Ok(Self::Plužine),
            0x11 => Ok(Self::Pljevlja),
            0x12 => Ok(Self::Rozaje),
            0x13 => Ok(Self::Šavnik),
            0x14 => Ok(Self::Tivat),
            0x15 => Ok(Self::Ulcinj),
            0x16 => Ok(Self::Zabljak),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MontenegroSubregions> for u8 {
    fn from(value: MontenegroSubregions) -> u8 {
        match value {
            MontenegroSubregions::Montenegro => 0x01,
            MontenegroSubregions::Podgorica => 0x02,
            MontenegroSubregions::Andrijevica => 0x03,
            MontenegroSubregions::Bar => 0x04,
            MontenegroSubregions::Berane => 0x05,
            MontenegroSubregions::BijeloPolje => 0x06,
            MontenegroSubregions::Budva => 0x07,
            MontenegroSubregions::Cetinje => 0x08,
            MontenegroSubregions::Danilovgrad => 0x09,
            MontenegroSubregions::HercegNovi => 0x0A,
            MontenegroSubregions::Kolasin => 0x0B,
            MontenegroSubregions::Kotor => 0x0C,
            MontenegroSubregions::Mojkovac => 0x0D,
            MontenegroSubregions::Niksic => 0x0E,
            MontenegroSubregions::Plav => 0x0F,
            MontenegroSubregions::Plužine => 0x10,
            MontenegroSubregions::Pljevlja => 0x11,
            MontenegroSubregions::Rozaje => 0x12,
            MontenegroSubregions::Šavnik => 0x13,
            MontenegroSubregions::Tivat => 0x14,
            MontenegroSubregions::Ulcinj => 0x15,
            MontenegroSubregions::Zabljak => 0x16,
        }
    }
}

impl Display for MontenegroSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Montenegro => write!(f, "Montenegro"),
            Self::Podgorica => write!(f, "Podgorica"),
            Self::Andrijevica => write!(f, "Andrijevica"),
            Self::Bar => write!(f, "Bar"),
            Self::Berane => write!(f, "Berane"),
            Self::BijeloPolje => write!(f, "Bijelo Polje"),
            Self::Budva => write!(f, "Budva"),
            Self::Cetinje => write!(f, "Cetinje"),
            Self::Danilovgrad => write!(f, "Danilovgrad"),
            Self::HercegNovi => write!(f, "Herceg Novi"),
            Self::Kolasin => write!(f, "Kolasin"),
            Self::Kotor => write!(f, "Kotor"),
            Self::Mojkovac => write!(f, "Mojkovac"),
            Self::Niksic => write!(f, "Niksic"),
            Self::Plav => write!(f, "Plav"),
            Self::Plužine => write!(f, "Plužine"),
            Self::Pljevlja => write!(f, "Pljevlja"),
            Self::Rozaje => write!(f, "Rozaje"),
            Self::Šavnik => write!(f, "Šavnik"),
            Self::Tivat => write!(f, "Tivat"),
            Self::Ulcinj => write!(f, "Ulcinj"),
            Self::Zabljak => write!(f, "Zabljak"),
        }
    }
}

// Montserrat Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MontserratSubregions {
    Montserrat,
}

impl TryFrom<u8> for MontserratSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Montserrat),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MontserratSubregions> for u8 {
    fn from(value: MontserratSubregions) -> u8 {
        match value {
            MontserratSubregions::Montserrat => 0x01,
        }
    }
}

impl Display for MontserratSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Montserrat => write!(f, "Montserrat"),
        }
    }
}

// Morocco Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoroccoSubregions {
    Morocco,
    RabatSaleKenitra,
    BeniMellalKhenifra,
    CasablancaSettat,
    DakhlaOuedEdDahab,
    DraaTafilalet,
    FesMeknes,
    GuelmimOuedNoun,
    LaayouneSakiaElHamra,
    MarrakeshSafi,
    Oriental,
    SoussMassa,
    TangerTetouanAlHoceima,
}

impl TryFrom<u8> for MoroccoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Morocco),
            0x02 => Ok(Self::RabatSaleKenitra),
            0x03 => Ok(Self::BeniMellalKhenifra),
            0x04 => Ok(Self::CasablancaSettat),
            0x05 => Ok(Self::DakhlaOuedEdDahab),
            0x06 => Ok(Self::DraaTafilalet),
            0x07 => Ok(Self::FesMeknes),
            0x08 => Ok(Self::GuelmimOuedNoun),
            0x09 => Ok(Self::LaayouneSakiaElHamra),
            0x0A => Ok(Self::MarrakeshSafi),
            0x0B => Ok(Self::Oriental),
            0x0C => Ok(Self::SoussMassa),
            0x0D => Ok(Self::TangerTetouanAlHoceima),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MoroccoSubregions> for u8 {
    fn from(value: MoroccoSubregions) -> u8 {
        match value {
            MoroccoSubregions::Morocco => 0x01,
            MoroccoSubregions::RabatSaleKenitra => 0x02,
            MoroccoSubregions::BeniMellalKhenifra => 0x03,
            MoroccoSubregions::CasablancaSettat => 0x04,
            MoroccoSubregions::DakhlaOuedEdDahab => 0x05,
            MoroccoSubregions::DraaTafilalet => 0x06,
            MoroccoSubregions::FesMeknes => 0x07,
            MoroccoSubregions::GuelmimOuedNoun => 0x08,
            MoroccoSubregions::LaayouneSakiaElHamra => 0x09,
            MoroccoSubregions::MarrakeshSafi => 0x0A,
            MoroccoSubregions::Oriental => 0x0B,
            MoroccoSubregions::SoussMassa => 0x0C,
            MoroccoSubregions::TangerTetouanAlHoceima => 0x0D,
        }
    }
}

impl Display for MoroccoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Morocco => write!(f, "Morocco"),
            Self::RabatSaleKenitra => write!(f, "Rabat-Salé-Kénitra"),
            Self::BeniMellalKhenifra => write!(f, "Béni Mellal-Khénifra"),
            Self::CasablancaSettat => write!(f, "Casablanca-Settat"),
            Self::DakhlaOuedEdDahab => write!(f, "Dakhla-Oued Ed-Dahab"),
            Self::DraaTafilalet => write!(f, "Drâa-Tafilalet"),
            Self::FesMeknes => write!(f, "Fès-Meknès"),
            Self::GuelmimOuedNoun => write!(f, "Guelmim-Oued Noun"),
            Self::LaayouneSakiaElHamra => write!(f, "Laâyoune-Sakia El Hamra"),
            Self::MarrakeshSafi => write!(f, "Marrakesh-Safi"),
            Self::Oriental => write!(f, "Oriental"),
            Self::SoussMassa => write!(f, "Souss-Massa"),
            Self::TangerTetouanAlHoceima => write!(f, "Tanger-Tetouan-Al Hoceima"),
        }
    }
}

// Mozambique Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MozambiqueSubregions {
    Mozambique,
    MaputoCity,
    MaputoProvince,
    CaboDelgado,
    Gaza,
    Inhambane,
    Manica,
    Nampula,
    Niassa,
    Sofala,
    Tete,
    Zambezia,
}

impl TryFrom<u8> for MozambiqueSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Mozambique),
            0x02 => Ok(Self::MaputoCity),
            0x03 => Ok(Self::MaputoProvince),
            0x04 => Ok(Self::CaboDelgado),
            0x05 => Ok(Self::Gaza),
            0x06 => Ok(Self::Inhambane),
            0x07 => Ok(Self::Manica),
            0x08 => Ok(Self::Nampula),
            0x09 => Ok(Self::Niassa),
            0x0A => Ok(Self::Sofala),
            0x0B => Ok(Self::Tete),
            0x0C => Ok(Self::Zambezia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MozambiqueSubregions> for u8 {
    fn from(value: MozambiqueSubregions) -> u8 {
        match value {
            MozambiqueSubregions::Mozambique => 0x01,
            MozambiqueSubregions::MaputoCity => 0x02,
            MozambiqueSubregions::MaputoProvince => 0x03,
            MozambiqueSubregions::CaboDelgado => 0x04,
            MozambiqueSubregions::Gaza => 0x05,
            MozambiqueSubregions::Inhambane => 0x06,
            MozambiqueSubregions::Manica => 0x07,
            MozambiqueSubregions::Nampula => 0x08,
            MozambiqueSubregions::Niassa => 0x09,
            MozambiqueSubregions::Sofala => 0x0A,
            MozambiqueSubregions::Tete => 0x0B,
            MozambiqueSubregions::Zambezia => 0x0C,
        }
    }
}

impl Display for MozambiqueSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mozambique => write!(f, "Mozambique"),
            Self::MaputoCity => write!(f, "Maputo (City)"),
            Self::MaputoProvince => write!(f, "Maputo (Province)"),
            Self::CaboDelgado => write!(f, "Cabo Delgado"),
            Self::Gaza => write!(f, "Gaza"),
            Self::Inhambane => write!(f, "Inhambane"),
            Self::Manica => write!(f, "Manica"),
            Self::Nampula => write!(f, "Nampula"),
            Self::Niassa => write!(f, "Niassa"),
            Self::Sofala => write!(f, "Sofala"),
            Self::Tete => write!(f, "Tete"),
            Self::Zambezia => write!(f, "Zambezia"),
        }
    }
}

// Myanmar Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MyanmarSubregions {
    Myanmar,
    NaypyidawUnionTerritory,
    AyeyarwadyRegion,
    BagoRegion,
    ChinState,
    KachinState,
    KayahState,
    KayinState,
    MagwayRegion,
    MandalayRegion,
    MonState,
    RakhineState,
    SagaingRegion,
    ShanState,
    TanintharyiRegion,
    YangonRegion,
}

impl TryFrom<u8> for MyanmarSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Myanmar),
            0x02 => Ok(Self::NaypyidawUnionTerritory),
            0x03 => Ok(Self::AyeyarwadyRegion),
            0x04 => Ok(Self::BagoRegion),
            0x05 => Ok(Self::ChinState),
            0x06 => Ok(Self::KachinState),
            0x07 => Ok(Self::KayahState),
            0x08 => Ok(Self::KayinState),
            0x09 => Ok(Self::MagwayRegion),
            0x0A => Ok(Self::MandalayRegion),
            0x0B => Ok(Self::MonState),
            0x0C => Ok(Self::RakhineState),
            0x0D => Ok(Self::SagaingRegion),
            0x0E => Ok(Self::ShanState),
            0x0F => Ok(Self::TanintharyiRegion),
            0x10 => Ok(Self::YangonRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<MyanmarSubregions> for u8 {
    fn from(value: MyanmarSubregions) -> u8 {
        match value {
            MyanmarSubregions::Myanmar => 0x01,
            MyanmarSubregions::NaypyidawUnionTerritory => 0x02,
            MyanmarSubregions::AyeyarwadyRegion => 0x03,
            MyanmarSubregions::BagoRegion => 0x04,
            MyanmarSubregions::ChinState => 0x05,
            MyanmarSubregions::KachinState => 0x06,
            MyanmarSubregions::KayahState => 0x07,
            MyanmarSubregions::KayinState => 0x08,
            MyanmarSubregions::MagwayRegion => 0x09,
            MyanmarSubregions::MandalayRegion => 0x0A,
            MyanmarSubregions::MonState => 0x0B,
            MyanmarSubregions::RakhineState => 0x0C,
            MyanmarSubregions::SagaingRegion => 0x0D,
            MyanmarSubregions::ShanState => 0x0E,
            MyanmarSubregions::TanintharyiRegion => 0x0F,
            MyanmarSubregions::YangonRegion => 0x10,
        }
    }
}

impl Display for MyanmarSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Myanmar => write!(f, "Myanmar"),
            Self::NaypyidawUnionTerritory => write!(f, "Naypyidaw Union Territory"),
            Self::AyeyarwadyRegion => write!(f, "Ayeyarwady Region"),
            Self::BagoRegion => write!(f, "Bago Region"),
            Self::ChinState => write!(f, "Chin State"),
            Self::KachinState => write!(f, "Kachin State"),
            Self::KayahState => write!(f, "Kayah State"),
            Self::KayinState => write!(f, "Kayin State"),
            Self::MagwayRegion => write!(f, "Magway Region"),
            Self::MandalayRegion => write!(f, "Mandalay Region"),
            Self::MonState => write!(f, "Mon State"),
            Self::RakhineState => write!(f, "Rakhine State"),
            Self::SagaingRegion => write!(f, "Sagaing Region"),
            Self::ShanState => write!(f, "Shan State"),
            Self::TanintharyiRegion => write!(f, "Tanintharyi Region"),
            Self::YangonRegion => write!(f, "Yangon Region"),
        }
    }
}

// Namibia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamibiaSubregions {
    Namibia,
    Khomas,
    Caprivi,
    Erongo,
    Hardap,
    Karas,
    Okavango,
    Kunene,
    Ohangwena,
    Omaheke,
    Omusati,
    Oshana,
    Oshikoto,
    Otjozondjupa,
}

impl TryFrom<u8> for NamibiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Namibia),
            0x02 => Ok(Self::Khomas),
            0x03 => Ok(Self::Caprivi),
            0x04 => Ok(Self::Erongo),
            0x05 => Ok(Self::Hardap),
            0x06 => Ok(Self::Karas),
            0x07 => Ok(Self::Okavango),
            0x08 => Ok(Self::Kunene),
            0x09 => Ok(Self::Ohangwena),
            0x0A => Ok(Self::Omaheke),
            0x0B => Ok(Self::Omusati),
            0x0C => Ok(Self::Oshana),
            0x0D => Ok(Self::Oshikoto),
            0x0E => Ok(Self::Otjozondjupa),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NamibiaSubregions> for u8 {
    fn from(value: NamibiaSubregions) -> u8 {
        match value {
            NamibiaSubregions::Namibia => 0x01,
            NamibiaSubregions::Khomas => 0x02,
            NamibiaSubregions::Caprivi => 0x03,
            NamibiaSubregions::Erongo => 0x04,
            NamibiaSubregions::Hardap => 0x05,
            NamibiaSubregions::Karas => 0x06,
            NamibiaSubregions::Okavango => 0x07,
            NamibiaSubregions::Kunene => 0x08,
            NamibiaSubregions::Ohangwena => 0x09,
            NamibiaSubregions::Omaheke => 0x0A,
            NamibiaSubregions::Omusati => 0x0B,
            NamibiaSubregions::Oshana => 0x0C,
            NamibiaSubregions::Oshikoto => 0x0D,
            NamibiaSubregions::Otjozondjupa => 0x0E,
        }
    }
}

impl Display for NamibiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Namibia => write!(f, "Namibia"),
            Self::Khomas => write!(f, "Khomas"),
            Self::Caprivi => write!(f, "Caprivi"),
            Self::Erongo => write!(f, "Erongo"),
            Self::Hardap => write!(f, "Hardap"),
            Self::Karas => write!(f, "Karas"),
            Self::Okavango => write!(f, "Okavango"),
            Self::Kunene => write!(f, "Kunene"),
            Self::Ohangwena => write!(f, "Ohangwena"),
            Self::Omaheke => write!(f, "Omaheke"),
            Self::Omusati => write!(f, "Omusati"),
            Self::Oshana => write!(f, "Oshana"),
            Self::Oshikoto => write!(f, "Oshikoto"),
            Self::Otjozondjupa => write!(f, "Otjozondjupa"),
        }
    }
}

// Nauru Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NauruSubregions {
    Nauru,
}

impl TryFrom<u8> for NauruSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Nauru),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NauruSubregions> for u8 {
    fn from(value: NauruSubregions) -> u8 {
        match value {
            NauruSubregions::Nauru => 0x01,
        }
    }
}

impl Display for NauruSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nauru => write!(f, "Nauru"),
        }
    }
}

// Nepal Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NepalSubregions {
    Nepal,
    ProvinceNo3,
    ProvinceNo1,
    ProvinceNo2,
    ProvinceNo5,
    GandakiPradesh,
    KarnaliPradesh,
    SudurpashchimPradesh,
}

impl TryFrom<u8> for NepalSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Nepal),
            0x02 => Ok(Self::ProvinceNo3),
            0x03 => Ok(Self::ProvinceNo1),
            0x04 => Ok(Self::ProvinceNo2),
            0x05 => Ok(Self::ProvinceNo5),
            0x06 => Ok(Self::GandakiPradesh),
            0x07 => Ok(Self::KarnaliPradesh),
            0x08 => Ok(Self::SudurpashchimPradesh),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NepalSubregions> for u8 {
    fn from(value: NepalSubregions) -> u8 {
        match value {
            NepalSubregions::Nepal => 0x01,
            NepalSubregions::ProvinceNo3 => 0x02,
            NepalSubregions::ProvinceNo1 => 0x03,
            NepalSubregions::ProvinceNo2 => 0x04,
            NepalSubregions::ProvinceNo5 => 0x05,
            NepalSubregions::GandakiPradesh => 0x06,
            NepalSubregions::KarnaliPradesh => 0x07,
            NepalSubregions::SudurpashchimPradesh => 0x08,
        }
    }
}

impl Display for NepalSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nepal => write!(f, "Nepal"),
            Self::ProvinceNo3 => write!(f, "Province No. 3"),
            Self::ProvinceNo1 => write!(f, "Province No. 1"),
            Self::ProvinceNo2 => write!(f, "Province No. 2"),
            Self::ProvinceNo5 => write!(f, "Province No. 5"),
            Self::GandakiPradesh => write!(f, "Gandaki Pradesh"),
            Self::KarnaliPradesh => write!(f, "Karnali Pradesh"),
            Self::SudurpashchimPradesh => write!(f, "Sudurpashchim Pradesh"),
        }
    }
}

// Netherlands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetherlandsSubregions {
    Netherlands,
    NorthHolland,
    Drenthe,
    Flevoland,
    Friesland,
    Gelderland,
    Groningen,
    Limburg,
    NorthBrabant,
    Overijssel,
    SouthHolland,
    Utrecht,
    Zeeland,
}

impl TryFrom<u8> for NetherlandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Netherlands),
            0x02 => Ok(Self::NorthHolland),
            0x03 => Ok(Self::Drenthe),
            0x04 => Ok(Self::Flevoland),
            0x05 => Ok(Self::Friesland),
            0x06 => Ok(Self::Gelderland),
            0x07 => Ok(Self::Groningen),
            0x08 => Ok(Self::Limburg),
            0x09 => Ok(Self::NorthBrabant),
            0x0A => Ok(Self::Overijssel),
            0x0B => Ok(Self::SouthHolland),
            0x0C => Ok(Self::Utrecht),
            0x0D => Ok(Self::Zeeland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NetherlandsSubregions> for u8 {
    fn from(value: NetherlandsSubregions) -> u8 {
        match value {
            NetherlandsSubregions::Netherlands => 0x01,
            NetherlandsSubregions::NorthHolland => 0x02,
            NetherlandsSubregions::Drenthe => 0x03,
            NetherlandsSubregions::Flevoland => 0x04,
            NetherlandsSubregions::Friesland => 0x05,
            NetherlandsSubregions::Gelderland => 0x06,
            NetherlandsSubregions::Groningen => 0x07,
            NetherlandsSubregions::Limburg => 0x08,
            NetherlandsSubregions::NorthBrabant => 0x09,
            NetherlandsSubregions::Overijssel => 0x0A,
            NetherlandsSubregions::SouthHolland => 0x0B,
            NetherlandsSubregions::Utrecht => 0x0C,
            NetherlandsSubregions::Zeeland => 0x0D,
        }
    }
}

impl Display for NetherlandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Netherlands => write!(f, "Netherlands"),
            Self::NorthHolland => write!(f, "North Holland"),
            Self::Drenthe => write!(f, "Drenthe"),
            Self::Flevoland => write!(f, "Flevoland"),
            Self::Friesland => write!(f, "Friesland"),
            Self::Gelderland => write!(f, "Gelderland"),
            Self::Groningen => write!(f, "Groningen"),
            Self::Limburg => write!(f, "Limburg"),
            Self::NorthBrabant => write!(f, "North Brabant"),
            Self::Overijssel => write!(f, "Overijssel"),
            Self::SouthHolland => write!(f, "South Holland"),
            Self::Utrecht => write!(f, "Utrecht"),
            Self::Zeeland => write!(f, "Zeeland"),
        }
    }
}

// New Caledonia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewCaledoniaSubregions {
    NewCaledonia,
    SouthProvince,
    LoyaltyIslandsProvince,
    NorthProvince,
}

impl TryFrom<u8> for NewCaledoniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::NewCaledonia),
            0x02 => Ok(Self::SouthProvince),
            0x03 => Ok(Self::LoyaltyIslandsProvince),
            0x04 => Ok(Self::NorthProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NewCaledoniaSubregions> for u8 {
    fn from(value: NewCaledoniaSubregions) -> u8 {
        match value {
            NewCaledoniaSubregions::NewCaledonia => 0x01,
            NewCaledoniaSubregions::SouthProvince => 0x02,
            NewCaledoniaSubregions::LoyaltyIslandsProvince => 0x03,
            NewCaledoniaSubregions::NorthProvince => 0x04,
        }
    }
}

impl Display for NewCaledoniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewCaledonia => write!(f, "New Caledonia"),
            Self::SouthProvince => write!(f, "South Province"),
            Self::LoyaltyIslandsProvince => write!(f, "Loyalty Islands Province"),
            Self::NorthProvince => write!(f, "North Province"),
        }
    }
}

// New Zealand Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewZealandSubregions {
    NewZealand,
    Wellington,
    Auckland,
    BayOfPlenty,
    Canterbury,
    Otago,
    HawkesBay,
    ManawatuWanganui,
    Nelson,
    Northland,
    Rotorua,
    Southland,
    Taranaki,
    Waikato,
    CookIslands,
    Niue,
    Tokelau,
    RossDependency,
}

impl TryFrom<u8> for NewZealandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::NewZealand),
            0x02 => Ok(Self::Wellington),
            0x03 => Ok(Self::Auckland),
            0x04 => Ok(Self::BayOfPlenty),
            0x05 => Ok(Self::Canterbury),
            0x06 => Ok(Self::Otago),
            0x07 => Ok(Self::HawkesBay),
            0x08 => Ok(Self::ManawatuWanganui),
            0x09 => Ok(Self::Nelson),
            0x0A => Ok(Self::Northland),
            0x0B => Ok(Self::Rotorua),
            0x0C => Ok(Self::Southland),
            0x0D => Ok(Self::Taranaki),
            0x0E => Ok(Self::Waikato),
            0x0F => Ok(Self::CookIslands),
            0x10 => Ok(Self::Niue),
            0x11 => Ok(Self::Tokelau),
            0x12 => Ok(Self::RossDependency),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NewZealandSubregions> for u8 {
    fn from(value: NewZealandSubregions) -> u8 {
        match value {
            NewZealandSubregions::NewZealand => 0x01,
            NewZealandSubregions::Wellington => 0x02,
            NewZealandSubregions::Auckland => 0x03,
            NewZealandSubregions::BayOfPlenty => 0x04,
            NewZealandSubregions::Canterbury => 0x05,
            NewZealandSubregions::Otago => 0x06,
            NewZealandSubregions::HawkesBay => 0x07,
            NewZealandSubregions::ManawatuWanganui => 0x08,
            NewZealandSubregions::Nelson => 0x09,
            NewZealandSubregions::Northland => 0x0A,
            NewZealandSubregions::Rotorua => 0x0B,
            NewZealandSubregions::Southland => 0x0C,
            NewZealandSubregions::Taranaki => 0x0D,
            NewZealandSubregions::Waikato => 0x0E,
            NewZealandSubregions::CookIslands => 0x0F,
            NewZealandSubregions::Niue => 0x10,
            NewZealandSubregions::Tokelau => 0x11,
            NewZealandSubregions::RossDependency => 0x12,
        }
    }
}

impl Display for NewZealandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewZealand => write!(f, "New Zealand"),
            Self::Wellington => write!(f, "Wellington"),
            Self::Auckland => write!(f, "Auckland"),
            Self::BayOfPlenty => write!(f, "Bay of Plenty"),
            Self::Canterbury => write!(f, "Canterbury"),
            Self::Otago => write!(f, "Otago"),
            Self::HawkesBay => write!(f, "Hawke's Bay"),
            Self::ManawatuWanganui => write!(f, "Manawatu-Wanganui"),
            Self::Nelson => write!(f, "Nelson"),
            Self::Northland => write!(f, "Northland"),
            Self::Rotorua => write!(f, "Rotorua"),
            Self::Southland => write!(f, "Southland"),
            Self::Taranaki => write!(f, "Taranaki"),
            Self::Waikato => write!(f, "Waikato"),
            Self::CookIslands => write!(f, "Cook Islands"),
            Self::Niue => write!(f, "Niue"),
            Self::Tokelau => write!(f, "Tokelau"),
            Self::RossDependency => write!(f, "Ross Dependency"),
        }
    }
}

// Nicaragua Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NicaraguaSubregions {
    Nicaragua,
    Managua,
    Boaco,
    Carazo,
    Chinandega,
    Chontales,
    Esteli,
    Granada,
    Jinotega,
    Leon,
    Madriz,
    Masaya,
    Matagalpa,
    NuevaSegovia,
    RioSanJuan,
    Rivas,
    AtlanticoNorte,
    AtlanticoSur,
}

impl TryFrom<u8> for NicaraguaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Nicaragua),
            0x02 => Ok(Self::Managua),
            0x03 => Ok(Self::Boaco),
            0x04 => Ok(Self::Carazo),
            0x05 => Ok(Self::Chinandega),
            0x06 => Ok(Self::Chontales),
            0x07 => Ok(Self::Esteli),
            0x08 => Ok(Self::Granada),
            0x09 => Ok(Self::Jinotega),
            0x0A => Ok(Self::Leon),
            0x0B => Ok(Self::Madriz),
            0x0C => Ok(Self::Masaya),
            0x0D => Ok(Self::Matagalpa),
            0x0E => Ok(Self::NuevaSegovia),
            0x0F => Ok(Self::RioSanJuan),
            0x10 => Ok(Self::Rivas),
            0x11 => Ok(Self::AtlanticoNorte),
            0x12 => Ok(Self::AtlanticoSur),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NicaraguaSubregions> for u8 {
    fn from(value: NicaraguaSubregions) -> u8 {
        match value {
            NicaraguaSubregions::Nicaragua => 0x01,
            NicaraguaSubregions::Managua => 0x02,
            NicaraguaSubregions::Boaco => 0x03,
            NicaraguaSubregions::Carazo => 0x04,
            NicaraguaSubregions::Chinandega => 0x05,
            NicaraguaSubregions::Chontales => 0x06,
            NicaraguaSubregions::Esteli => 0x07,
            NicaraguaSubregions::Granada => 0x08,
            NicaraguaSubregions::Jinotega => 0x09,
            NicaraguaSubregions::Leon => 0x0A,
            NicaraguaSubregions::Madriz => 0x0B,
            NicaraguaSubregions::Masaya => 0x0C,
            NicaraguaSubregions::Matagalpa => 0x0D,
            NicaraguaSubregions::NuevaSegovia => 0x0E,
            NicaraguaSubregions::RioSanJuan => 0x0F,
            NicaraguaSubregions::Rivas => 0x10,
            NicaraguaSubregions::AtlanticoNorte => 0x11,
            NicaraguaSubregions::AtlanticoSur => 0x12,
        }
    }
}

impl Display for NicaraguaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nicaragua => write!(f, "Nicaragua"),
            Self::Managua => write!(f, "Managua"),
            Self::Boaco => write!(f, "Boaco"),
            Self::Carazo => write!(f, "Carazo"),
            Self::Chinandega => write!(f, "Chinandega"),
            Self::Chontales => write!(f, "Chontales"),
            Self::Esteli => write!(f, "Estelí"),
            Self::Granada => write!(f, "Granada"),
            Self::Jinotega => write!(f, "Jinotega"),
            Self::Leon => write!(f, "León"),
            Self::Madriz => write!(f, "Madriz"),
            Self::Masaya => write!(f, "Masaya"),
            Self::Matagalpa => write!(f, "Matagalpa"),
            Self::NuevaSegovia => write!(f, "Nueva Segovia"),
            Self::RioSanJuan => write!(f, "Río San Juan"),
            Self::Rivas => write!(f, "Rivas"),
            Self::AtlanticoNorte => write!(f, "Atlántico Norte"),
            Self::AtlanticoSur => write!(f, "Atlántico Sur"),
        }
    }
}

// Niger Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NigerSubregions {
    Niger,
    Niamey,
    AgadezRegion,
    DiffaRegion,
    DossoRegion,
    MaradiRegion,
    TahouaRegion,
    TillaberiRegion,
    ZinderRegion,
}

impl TryFrom<u8> for NigerSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Niger),
            0x02 => Ok(Self::Niamey),
            0x03 => Ok(Self::AgadezRegion),
            0x04 => Ok(Self::DiffaRegion),
            0x05 => Ok(Self::DossoRegion),
            0x06 => Ok(Self::MaradiRegion),
            0x07 => Ok(Self::TahouaRegion),
            0x08 => Ok(Self::TillaberiRegion),
            0x09 => Ok(Self::ZinderRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NigerSubregions> for u8 {
    fn from(value: NigerSubregions) -> u8 {
        match value {
            NigerSubregions::Niger => 0x01,
            NigerSubregions::Niamey => 0x02,
            NigerSubregions::AgadezRegion => 0x03,
            NigerSubregions::DiffaRegion => 0x04,
            NigerSubregions::DossoRegion => 0x05,
            NigerSubregions::MaradiRegion => 0x06,
            NigerSubregions::TahouaRegion => 0x07,
            NigerSubregions::TillaberiRegion => 0x08,
            NigerSubregions::ZinderRegion => 0x09,
        }
    }
}

impl Display for NigerSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Niger => write!(f, "Niger"),
            Self::Niamey => write!(f, "Niamey"),
            Self::AgadezRegion => write!(f, "Agadez Region"),
            Self::DiffaRegion => write!(f, "Diffa Region"),
            Self::DossoRegion => write!(f, "Dosso Region"),
            Self::MaradiRegion => write!(f, "Maradi Region"),
            Self::TahouaRegion => write!(f, "Tahoua Region"),
            Self::TillaberiRegion => write!(f, "Tillabéri Region"),
            Self::ZinderRegion => write!(f, "Zinder Region"),
        }
    }
}

// Nigeria Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NigeriaSubregions {
    Nigeria,
    FederalCapitalTerritory,
    Abia,
    Adamawa,
    AkwaIbom,
    Anambra,
    Bauchi,
    Bayelsa,
    Benue,
    Borno,
    CrossRiver,
    Delta,
    Ebonyi,
    Enugu,
    Edo,
    Ekiti,
    Gombe,
    Imo,
    Jigawa,
    Kaduna,
    Kano,
    Katsina,
    Kebbi,
    Kogi,
    Kwara,
    Lagos,
    Nasarawa,
    Niger,
    Ogun,
    Ondo,
    Osun,
    Oyo,
    Plateau,
    Rivers,
    Sokoto,
    Taraba,
    Yobe,
    Zamfara,
}

impl TryFrom<u8> for NigeriaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Nigeria),
            0x02 => Ok(Self::FederalCapitalTerritory),
            0x03 => Ok(Self::Abia),
            0x04 => Ok(Self::Adamawa),
            0x05 => Ok(Self::AkwaIbom),
            0x06 => Ok(Self::Anambra),
            0x07 => Ok(Self::Bauchi),
            0x08 => Ok(Self::Bayelsa),
            0x09 => Ok(Self::Benue),
            0x0A => Ok(Self::Borno),
            0x0B => Ok(Self::CrossRiver),
            0x0C => Ok(Self::Delta),
            0x0D => Ok(Self::Ebonyi),
            0x0E => Ok(Self::Enugu),
            0x0F => Ok(Self::Edo),
            0x10 => Ok(Self::Ekiti),
            0x11 => Ok(Self::Gombe),
            0x12 => Ok(Self::Imo),
            0x13 => Ok(Self::Jigawa),
            0x14 => Ok(Self::Kaduna),
            0x15 => Ok(Self::Kano),
            0x16 => Ok(Self::Katsina),
            0x17 => Ok(Self::Kebbi),
            0x18 => Ok(Self::Kogi),
            0x19 => Ok(Self::Kwara),
            0x1A => Ok(Self::Lagos),
            0x1B => Ok(Self::Nasarawa),
            0x1C => Ok(Self::Niger),
            0x1D => Ok(Self::Ogun),
            0x1E => Ok(Self::Ondo),
            0x1F => Ok(Self::Osun),
            0x20 => Ok(Self::Oyo),
            0x21 => Ok(Self::Plateau),
            0x22 => Ok(Self::Rivers),
            0x23 => Ok(Self::Sokoto),
            0x24 => Ok(Self::Taraba),
            0x25 => Ok(Self::Yobe),
            0x26 => Ok(Self::Zamfara),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NigeriaSubregions> for u8 {
    fn from(value: NigeriaSubregions) -> u8 {
        match value {
            NigeriaSubregions::Nigeria => 0x01,
            NigeriaSubregions::FederalCapitalTerritory => 0x02,
            NigeriaSubregions::Abia => 0x03,
            NigeriaSubregions::Adamawa => 0x04,
            NigeriaSubregions::AkwaIbom => 0x05,
            NigeriaSubregions::Anambra => 0x06,
            NigeriaSubregions::Bauchi => 0x07,
            NigeriaSubregions::Bayelsa => 0x08,
            NigeriaSubregions::Benue => 0x09,
            NigeriaSubregions::Borno => 0x0A,
            NigeriaSubregions::CrossRiver => 0x0B,
            NigeriaSubregions::Delta => 0x0C,
            NigeriaSubregions::Ebonyi => 0x0D,
            NigeriaSubregions::Enugu => 0x0E,
            NigeriaSubregions::Edo => 0x0F,
            NigeriaSubregions::Ekiti => 0x10,
            NigeriaSubregions::Gombe => 0x11,
            NigeriaSubregions::Imo => 0x12,
            NigeriaSubregions::Jigawa => 0x13,
            NigeriaSubregions::Kaduna => 0x14,
            NigeriaSubregions::Kano => 0x15,
            NigeriaSubregions::Katsina => 0x16,
            NigeriaSubregions::Kebbi => 0x17,
            NigeriaSubregions::Kogi => 0x18,
            NigeriaSubregions::Kwara => 0x19,
            NigeriaSubregions::Lagos => 0x1A,
            NigeriaSubregions::Nasarawa => 0x1B,
            NigeriaSubregions::Niger => 0x1C,
            NigeriaSubregions::Ogun => 0x1D,
            NigeriaSubregions::Ondo => 0x1E,
            NigeriaSubregions::Osun => 0x1F,
            NigeriaSubregions::Oyo => 0x20,
            NigeriaSubregions::Plateau => 0x21,
            NigeriaSubregions::Rivers => 0x22,
            NigeriaSubregions::Sokoto => 0x23,
            NigeriaSubregions::Taraba => 0x24,
            NigeriaSubregions::Yobe => 0x25,
            NigeriaSubregions::Zamfara => 0x26,
        }
    }
}

impl Display for NigeriaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nigeria => write!(f, "Nigeria"),
            Self::FederalCapitalTerritory => write!(f, "Federal Capital Territory"),
            Self::Abia => write!(f, "Abia"),
            Self::Adamawa => write!(f, "Adamawa"),
            Self::AkwaIbom => write!(f, "Akwa Ibom"),
            Self::Anambra => write!(f, "Anambra"),
            Self::Bauchi => write!(f, "Bauchi"),
            Self::Bayelsa => write!(f, "Bayelsa"),
            Self::Benue => write!(f, "Benue"),
            Self::Borno => write!(f, "Borno"),
            Self::CrossRiver => write!(f, "Cross River"),
            Self::Delta => write!(f, "Delta"),
            Self::Ebonyi => write!(f, "Ebonyi"),
            Self::Enugu => write!(f, "Enugu"),
            Self::Edo => write!(f, "Edo"),
            Self::Ekiti => write!(f, "Ekiti"),
            Self::Gombe => write!(f, "Gombe"),
            Self::Imo => write!(f, "Imo"),
            Self::Jigawa => write!(f, "Jigawa"),
            Self::Kaduna => write!(f, "Kaduna"),
            Self::Kano => write!(f, "Kano"),
            Self::Katsina => write!(f, "Katsina"),
            Self::Kebbi => write!(f, "Kebbi"),
            Self::Kogi => write!(f, "Kogi"),
            Self::Kwara => write!(f, "Kwara"),
            Self::Lagos => write!(f, "Lagos"),
            Self::Nasarawa => write!(f, "Nasarawa"),
            Self::Niger => write!(f, "Niger"),
            Self::Ogun => write!(f, "Ogun"),
            Self::Ondo => write!(f, "Ondo"),
            Self::Osun => write!(f, "Osun"),
            Self::Oyo => write!(f, "Oyo"),
            Self::Plateau => write!(f, "Plateau"),
            Self::Rivers => write!(f, "Rivers"),
            Self::Sokoto => write!(f, "Sokoto"),
            Self::Taraba => write!(f, "Taraba"),
            Self::Yobe => write!(f, "Yobe"),
            Self::Zamfara => write!(f, "Zamfara"),
        }
    }
}

// Niue Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NiueSubregions {
    Niue,
}

impl TryFrom<u8> for NiueSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Niue),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NiueSubregions> for u8 {
    fn from(value: NiueSubregions) -> u8 {
        match value {
            NiueSubregions::Niue => 0x01,
        }
    }
}

impl Display for NiueSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Niue => write!(f, "Niue"),
        }
    }
}

// Norfolk Island Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorfolkIslandSubregions {
    NorfolkIsland,
}

impl TryFrom<u8> for NorfolkIslandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::NorfolkIsland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NorfolkIslandSubregions> for u8 {
    fn from(value: NorfolkIslandSubregions) -> u8 {
        match value {
            NorfolkIslandSubregions::NorfolkIsland => 0x01,
        }
    }
}

impl Display for NorfolkIslandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorfolkIsland => write!(f, "Norfolk Island"),
        }
    }
}

// North Korea Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorthKoreaSubregions {
    NorthKorea,
    Pyongyang,
    ChagangProvince,
    KangwonProvince,
    NorthHamgyongProvince,
    NorthHwanghaeProvince,
    NorthPyonganProvince,
    Rason,
    RyanggangProvince,
    SouthHamgyongProvince,
    SouthHwanghaeProvince,
    SouthPyonganProvince,
}

impl TryFrom<u8> for NorthKoreaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::NorthKorea),
            0x02 => Ok(Self::Pyongyang),
            0x03 => Ok(Self::ChagangProvince),
            0x04 => Ok(Self::KangwonProvince),
            0x05 => Ok(Self::NorthHamgyongProvince),
            0x06 => Ok(Self::NorthHwanghaeProvince),
            0x07 => Ok(Self::NorthPyonganProvince),
            0x08 => Ok(Self::Rason),
            0x09 => Ok(Self::RyanggangProvince),
            0x0A => Ok(Self::SouthHamgyongProvince),
            0x0B => Ok(Self::SouthHwanghaeProvince),
            0x0C => Ok(Self::SouthPyonganProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NorthKoreaSubregions> for u8 {
    fn from(value: NorthKoreaSubregions) -> u8 {
        match value {
            NorthKoreaSubregions::NorthKorea => 0x01,
            NorthKoreaSubregions::Pyongyang => 0x02,
            NorthKoreaSubregions::ChagangProvince => 0x03,
            NorthKoreaSubregions::KangwonProvince => 0x04,
            NorthKoreaSubregions::NorthHamgyongProvince => 0x05,
            NorthKoreaSubregions::NorthHwanghaeProvince => 0x06,
            NorthKoreaSubregions::NorthPyonganProvince => 0x07,
            NorthKoreaSubregions::Rason => 0x08,
            NorthKoreaSubregions::RyanggangProvince => 0x09,
            NorthKoreaSubregions::SouthHamgyongProvince => 0x0A,
            NorthKoreaSubregions::SouthHwanghaeProvince => 0x0B,
            NorthKoreaSubregions::SouthPyonganProvince => 0x0C,
        }
    }
}

impl Display for NorthKoreaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorthKorea => write!(f, "North Korea"),
            Self::Pyongyang => write!(f, "Pyongyang"),
            Self::ChagangProvince => write!(f, "Chagang Province"),
            Self::KangwonProvince => write!(f, "Kangwon Province"),
            Self::NorthHamgyongProvince => write!(f, "North Hamgyong Province"),
            Self::NorthHwanghaeProvince => write!(f, "North Hwanghae Province"),
            Self::NorthPyonganProvince => write!(f, "North Pyongan Province"),
            Self::Rason => write!(f, "Rason"),
            Self::RyanggangProvince => write!(f, "Ryanggang Province"),
            Self::SouthHamgyongProvince => write!(f, "South Hamgyong Province"),
            Self::SouthHwanghaeProvince => write!(f, "South Hwanghae Province"),
            Self::SouthPyonganProvince => write!(f, "South Pyongan Province"),
        }
    }
}

// North Macedonia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorthMacedoniaSubregions {
    NorthMacedonia,
    Skopje,
    Eastern,
    Northeastern,
    Pelagonia,
    Polog,
    Southeastern,
    Southwestern,
    Vardar,
}

impl TryFrom<u8> for NorthMacedoniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::NorthMacedonia),
            0x02 => Ok(Self::Skopje),
            0x03 => Ok(Self::Eastern),
            0x04 => Ok(Self::Northeastern),
            0x05 => Ok(Self::Pelagonia),
            0x06 => Ok(Self::Polog),
            0x07 => Ok(Self::Southeastern),
            0x08 => Ok(Self::Southwestern),
            0x09 => Ok(Self::Vardar),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NorthMacedoniaSubregions> for u8 {
    fn from(value: NorthMacedoniaSubregions) -> u8 {
        match value {
            NorthMacedoniaSubregions::NorthMacedonia => 0x01,
            NorthMacedoniaSubregions::Skopje => 0x02,
            NorthMacedoniaSubregions::Eastern => 0x03,
            NorthMacedoniaSubregions::Northeastern => 0x04,
            NorthMacedoniaSubregions::Pelagonia => 0x05,
            NorthMacedoniaSubregions::Polog => 0x06,
            NorthMacedoniaSubregions::Southeastern => 0x07,
            NorthMacedoniaSubregions::Southwestern => 0x08,
            NorthMacedoniaSubregions::Vardar => 0x09,
        }
    }
}

impl Display for NorthMacedoniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorthMacedonia => write!(f, "North Macedonia"),
            Self::Skopje => write!(f, "Skopje"),
            Self::Eastern => write!(f, "Eastern"),
            Self::Northeastern => write!(f, "Northeastern"),
            Self::Pelagonia => write!(f, "Pelagonia"),
            Self::Polog => write!(f, "Polog"),
            Self::Southeastern => write!(f, "Southeastern"),
            Self::Southwestern => write!(f, "Southwestern"),
            Self::Vardar => write!(f, "Vardar"),
        }
    }
}

// Northern Cyprus Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorthernCyprusSubregions {
    NorthernCyprus,
    LefkoşaDistrict,
    GazimağusaDistrict,
    GirneDistrict,
    GuzelyurtDistrict,
    İskeleDistrict,
    LefkeDistrict,
}

impl TryFrom<u8> for NorthernCyprusSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::NorthernCyprus),
            0x02 => Ok(Self::LefkoşaDistrict),
            0x03 => Ok(Self::GazimağusaDistrict),
            0x04 => Ok(Self::GirneDistrict),
            0x05 => Ok(Self::GuzelyurtDistrict),
            0x06 => Ok(Self::İskeleDistrict),
            0x07 => Ok(Self::LefkeDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NorthernCyprusSubregions> for u8 {
    fn from(value: NorthernCyprusSubregions) -> u8 {
        match value {
            NorthernCyprusSubregions::NorthernCyprus => 0x01,
            NorthernCyprusSubregions::LefkoşaDistrict => 0x02,
            NorthernCyprusSubregions::GazimağusaDistrict => 0x03,
            NorthernCyprusSubregions::GirneDistrict => 0x04,
            NorthernCyprusSubregions::GuzelyurtDistrict => 0x05,
            NorthernCyprusSubregions::İskeleDistrict => 0x06,
            NorthernCyprusSubregions::LefkeDistrict => 0x07,
        }
    }
}

impl Display for NorthernCyprusSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorthernCyprus => write!(f, "Northern Cyprus"),
            Self::LefkoşaDistrict => write!(f, "Lefkoşa District"),
            Self::GazimağusaDistrict => write!(f, "Gazimağusa District"),
            Self::GirneDistrict => write!(f, "Girne District"),
            Self::GuzelyurtDistrict => write!(f, "Güzelyurt District"),
            Self::İskeleDistrict => write!(f, "İskele District"),
            Self::LefkeDistrict => write!(f, "Lefke District"),
        }
    }
}

// Northern Mariana Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorthernMarianaIslandsSubregions {
    NorthernMarianaIslands,
    Saipan,
    NorthernIslandsMunicipality,
    Tinian,
    Rota,
}

impl TryFrom<u8> for NorthernMarianaIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::NorthernMarianaIslands),
            0x02 => Ok(Self::Saipan),
            0x03 => Ok(Self::NorthernIslandsMunicipality),
            0x04 => Ok(Self::Tinian),
            0x05 => Ok(Self::Rota),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NorthernMarianaIslandsSubregions> for u8 {
    fn from(value: NorthernMarianaIslandsSubregions) -> u8 {
        match value {
            NorthernMarianaIslandsSubregions::NorthernMarianaIslands => 0x01,
            NorthernMarianaIslandsSubregions::Saipan => 0x02,
            NorthernMarianaIslandsSubregions::NorthernIslandsMunicipality => 0x03,
            NorthernMarianaIslandsSubregions::Tinian => 0x04,
            NorthernMarianaIslandsSubregions::Rota => 0x05,
        }
    }
}

impl Display for NorthernMarianaIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorthernMarianaIslands => write!(f, "Northern Mariana Islands"),
            Self::Saipan => write!(f, "Saipan"),
            Self::NorthernIslandsMunicipality => write!(f, "Northern Islands Municipality"),
            Self::Tinian => write!(f, "Tinian"),
            Self::Rota => write!(f, "Rota"),
        }
    }
}

// Norway Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NorwaySubregions {
    Norway,
    Østlandet,
    NordNorge,
    Trøndelag,
    Vestlandet,
    Sørlandet,
    BearIsland,
    BouvetIsland,
    JanMayen,
    PeterIIsland,
    QueenMaudLand,
    Svalbard,
}

impl TryFrom<u8> for NorwaySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Norway),
            0x02 => Ok(Self::Østlandet),
            0x03 => Ok(Self::NordNorge),
            0x04 => Ok(Self::Trøndelag),
            0x05 => Ok(Self::Vestlandet),
            0x06 => Ok(Self::Sørlandet),
            0x07 => Ok(Self::BearIsland),
            0x08 => Ok(Self::BouvetIsland),
            0x09 => Ok(Self::JanMayen),
            0x0A => Ok(Self::PeterIIsland),
            0x0B => Ok(Self::QueenMaudLand),
            0x0C => Ok(Self::Svalbard),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<NorwaySubregions> for u8 {
    fn from(value: NorwaySubregions) -> u8 {
        match value {
            NorwaySubregions::Norway => 0x01,
            NorwaySubregions::Østlandet => 0x02,
            NorwaySubregions::NordNorge => 0x03,
            NorwaySubregions::Trøndelag => 0x04,
            NorwaySubregions::Vestlandet => 0x05,
            NorwaySubregions::Sørlandet => 0x06,
            NorwaySubregions::BearIsland => 0x07,
            NorwaySubregions::BouvetIsland => 0x08,
            NorwaySubregions::JanMayen => 0x09,
            NorwaySubregions::PeterIIsland => 0x0A,
            NorwaySubregions::QueenMaudLand => 0x0B,
            NorwaySubregions::Svalbard => 0x0C,
        }
    }
}

impl Display for NorwaySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Norway => write!(f, "Norway"),
            Self::Østlandet => write!(f, "Østlandet"),
            Self::NordNorge => write!(f, "Nord-Norge"),
            Self::Trøndelag => write!(f, "Trøndelag"),
            Self::Vestlandet => write!(f, "Vestlandet"),
            Self::Sørlandet => write!(f, "Sørlandet"),
            Self::BearIsland => write!(f, "Bear Island"),
            Self::BouvetIsland => write!(f, "Bouvet Island"),
            Self::JanMayen => write!(f, "Jan Mayen"),
            Self::PeterIIsland => write!(f, "Peter I Island"),
            Self::QueenMaudLand => write!(f, "Queen Maud Land"),
            Self::Svalbard => write!(f, "Svalbard"),
        }
    }
}

// Oman Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OmanSubregions {
    Oman,
    Masqat,
    AdDakhiliyah,
    AlBatinah,
    AlWusta,
    AshSharqiyah,
    AzZahirah,
    Musandam,
    Zufar,
}

impl TryFrom<u8> for OmanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Oman),
            0x02 => Ok(Self::Masqat),
            0x03 => Ok(Self::AdDakhiliyah),
            0x04 => Ok(Self::AlBatinah),
            0x05 => Ok(Self::AlWusta),
            0x06 => Ok(Self::AshSharqiyah),
            0x07 => Ok(Self::AzZahirah),
            0x08 => Ok(Self::Musandam),
            0x09 => Ok(Self::Zufar),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<OmanSubregions> for u8 {
    fn from(value: OmanSubregions) -> u8 {
        match value {
            OmanSubregions::Oman => 0x01,
            OmanSubregions::Masqat => 0x02,
            OmanSubregions::AdDakhiliyah => 0x03,
            OmanSubregions::AlBatinah => 0x04,
            OmanSubregions::AlWusta => 0x05,
            OmanSubregions::AshSharqiyah => 0x06,
            OmanSubregions::AzZahirah => 0x07,
            OmanSubregions::Musandam => 0x08,
            OmanSubregions::Zufar => 0x09,
        }
    }
}

impl Display for OmanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Oman => write!(f, "Oman"),
            Self::Masqat => write!(f, "Masqat"),
            Self::AdDakhiliyah => write!(f, "Ad Dakhiliyah"),
            Self::AlBatinah => write!(f, "Al Batinah"),
            Self::AlWusta => write!(f, "Al Wusta"),
            Self::AshSharqiyah => write!(f, "Ash Sharqiyah"),
            Self::AzZahirah => write!(f, "Az Zahirah"),
            Self::Musandam => write!(f, "Musandam"),
            Self::Zufar => write!(f, "Zufar"),
        }
    }
}

// Pakistan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PakistanSubregions {
    Pakistan,
    IslamabadCapitalTerritory,
    AzadJammuAndKashmir,
    Balochistan,
    GilgitBaltistan,
    KhyberPakhtunkhwa,
    Punjab,
    Sindh,
}

impl TryFrom<u8> for PakistanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Pakistan),
            0x02 => Ok(Self::IslamabadCapitalTerritory),
            0x03 => Ok(Self::AzadJammuAndKashmir),
            0x04 => Ok(Self::Balochistan),
            0x05 => Ok(Self::GilgitBaltistan),
            0x06 => Ok(Self::KhyberPakhtunkhwa),
            0x07 => Ok(Self::Punjab),
            0x08 => Ok(Self::Sindh),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PakistanSubregions> for u8 {
    fn from(value: PakistanSubregions) -> u8 {
        match value {
            PakistanSubregions::Pakistan => 0x01,
            PakistanSubregions::IslamabadCapitalTerritory => 0x02,
            PakistanSubregions::AzadJammuAndKashmir => 0x03,
            PakistanSubregions::Balochistan => 0x04,
            PakistanSubregions::GilgitBaltistan => 0x05,
            PakistanSubregions::KhyberPakhtunkhwa => 0x06,
            PakistanSubregions::Punjab => 0x07,
            PakistanSubregions::Sindh => 0x08,
        }
    }
}

impl Display for PakistanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pakistan => write!(f, "Pakistan"),
            Self::IslamabadCapitalTerritory => write!(f, "Islamabad Capital Territory"),
            Self::AzadJammuAndKashmir => write!(f, "Azad Jammu and Kashmir"),
            Self::Balochistan => write!(f, "Balochistan"),
            Self::GilgitBaltistan => write!(f, "Gilgit-Baltistan"),
            Self::KhyberPakhtunkhwa => write!(f, "Khyber Pakhtunkhwa"),
            Self::Punjab => write!(f, "Punjab"),
            Self::Sindh => write!(f, "Sindh"),
        }
    }
}

// Palau Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PalauSubregions {
    Palau,
    Melekeok,
    Aimeliik,
    Airai,
    Angaur,
    Hatohobei,
    Kayangel,
    Koror,
    Ngaraard,
    Ngarchelong,
    Ngardmau,
    Ngatpang,
    Ngchesar,
    Ngeremlengui,
    Ngiwal,
    Peleliu,
    Sonsorol,
}

impl TryFrom<u8> for PalauSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Palau),
            0x02 => Ok(Self::Melekeok),
            0x03 => Ok(Self::Aimeliik),
            0x04 => Ok(Self::Airai),
            0x05 => Ok(Self::Angaur),
            0x06 => Ok(Self::Hatohobei),
            0x07 => Ok(Self::Kayangel),
            0x08 => Ok(Self::Koror),
            0x09 => Ok(Self::Ngaraard),
            0x0A => Ok(Self::Ngarchelong),
            0x0B => Ok(Self::Ngardmau),
            0x0C => Ok(Self::Ngatpang),
            0x0D => Ok(Self::Ngchesar),
            0x0E => Ok(Self::Ngeremlengui),
            0x0F => Ok(Self::Ngiwal),
            0x10 => Ok(Self::Peleliu),
            0x11 => Ok(Self::Sonsorol),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PalauSubregions> for u8 {
    fn from(value: PalauSubregions) -> u8 {
        match value {
            PalauSubregions::Palau => 0x01,
            PalauSubregions::Melekeok => 0x02,
            PalauSubregions::Aimeliik => 0x03,
            PalauSubregions::Airai => 0x04,
            PalauSubregions::Angaur => 0x05,
            PalauSubregions::Hatohobei => 0x06,
            PalauSubregions::Kayangel => 0x07,
            PalauSubregions::Koror => 0x08,
            PalauSubregions::Ngaraard => 0x09,
            PalauSubregions::Ngarchelong => 0x0A,
            PalauSubregions::Ngardmau => 0x0B,
            PalauSubregions::Ngatpang => 0x0C,
            PalauSubregions::Ngchesar => 0x0D,
            PalauSubregions::Ngeremlengui => 0x0E,
            PalauSubregions::Ngiwal => 0x0F,
            PalauSubregions::Peleliu => 0x10,
            PalauSubregions::Sonsorol => 0x11,
        }
    }
}

impl Display for PalauSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Palau => write!(f, "Palau"),
            Self::Melekeok => write!(f, "Melekeok"),
            Self::Aimeliik => write!(f, "Aimeliik"),
            Self::Airai => write!(f, "Airai"),
            Self::Angaur => write!(f, "Angaur"),
            Self::Hatohobei => write!(f, "Hatohobei"),
            Self::Kayangel => write!(f, "Kayangel"),
            Self::Koror => write!(f, "Koror"),
            Self::Ngaraard => write!(f, "Ngaraard"),
            Self::Ngarchelong => write!(f, "Ngarchelong"),
            Self::Ngardmau => write!(f, "Ngardmau"),
            Self::Ngatpang => write!(f, "Ngatpang"),
            Self::Ngchesar => write!(f, "Ngchesar"),
            Self::Ngeremlengui => write!(f, "Ngeremlengui"),
            Self::Ngiwal => write!(f, "Ngiwal"),
            Self::Peleliu => write!(f, "Peleliu"),
            Self::Sonsorol => write!(f, "Sonsorol"),
        }
    }
}

// Palestine Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PalestineSubregions {
    Palestine,
    WestBank,
    GazaStrip,
}

impl TryFrom<u8> for PalestineSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Palestine),
            0x02 => Ok(Self::WestBank),
            0x03 => Ok(Self::GazaStrip),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PalestineSubregions> for u8 {
    fn from(value: PalestineSubregions) -> u8 {
        match value {
            PalestineSubregions::Palestine => 0x01,
            PalestineSubregions::WestBank => 0x02,
            PalestineSubregions::GazaStrip => 0x03,
        }
    }
}

impl Display for PalestineSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Palestine => write!(f, "Palestine"),
            Self::WestBank => write!(f, "West Bank"),
            Self::GazaStrip => write!(f, "Gaza Strip"),
        }
    }
}

// Panama Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanamaSubregions {
    Panama,
    BocasDelToro,
    Chiriqui,
    Cocle,
    Colon,
    Darien,
    Herrera,
    LosSantos,
    SanBlas,
    Veraguas,
}

impl TryFrom<u8> for PanamaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Panama),
            0x02 => Ok(Self::Panama),
            0x03 => Ok(Self::BocasDelToro),
            0x04 => Ok(Self::Chiriqui),
            0x05 => Ok(Self::Cocle),
            0x06 => Ok(Self::Colon),
            0x07 => Ok(Self::Darien),
            0x08 => Ok(Self::Herrera),
            0x09 => Ok(Self::LosSantos),
            0x0A => Ok(Self::SanBlas),
            0x0B => Ok(Self::Veraguas),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PanamaSubregions> for u8 {
    fn from(value: PanamaSubregions) -> u8 {
        match value {
            PanamaSubregions::Panama => 0x01,
            PanamaSubregions::BocasDelToro => 0x03,
            PanamaSubregions::Chiriqui => 0x04,
            PanamaSubregions::Cocle => 0x05,
            PanamaSubregions::Colon => 0x06,
            PanamaSubregions::Darien => 0x07,
            PanamaSubregions::Herrera => 0x08,
            PanamaSubregions::LosSantos => 0x09,
            PanamaSubregions::SanBlas => 0x0A,
            PanamaSubregions::Veraguas => 0x0B,
        }
    }
}

impl Display for PanamaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Panama => write!(f, "Panamá"),
            Self::BocasDelToro => write!(f, "Bocas del Toro"),
            Self::Chiriqui => write!(f, "Chiriquí"),
            Self::Cocle => write!(f, "Coclé"),
            Self::Colon => write!(f, "Colón"),
            Self::Darien => write!(f, "Darién"),
            Self::Herrera => write!(f, "Herrera"),
            Self::LosSantos => write!(f, "Los Santos"),
            Self::SanBlas => write!(f, "San Blas"),
            Self::Veraguas => write!(f, "Veraguas"),
        }
    }
}

// Papua New Guinea Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PapuaNewGuineaSubregions {
    PapuaNewGuinea,
    SouthernRegion,
    HighlandsRegion,
    IslandsRegion,
    MomaseRegion,
}

impl TryFrom<u8> for PapuaNewGuineaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::PapuaNewGuinea),
            0x02 => Ok(Self::SouthernRegion),
            0x03 => Ok(Self::HighlandsRegion),
            0x04 => Ok(Self::IslandsRegion),
            0x05 => Ok(Self::MomaseRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PapuaNewGuineaSubregions> for u8 {
    fn from(value: PapuaNewGuineaSubregions) -> u8 {
        match value {
            PapuaNewGuineaSubregions::PapuaNewGuinea => 0x01,
            PapuaNewGuineaSubregions::SouthernRegion => 0x02,
            PapuaNewGuineaSubregions::HighlandsRegion => 0x03,
            PapuaNewGuineaSubregions::IslandsRegion => 0x04,
            PapuaNewGuineaSubregions::MomaseRegion => 0x05,
        }
    }
}

impl Display for PapuaNewGuineaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PapuaNewGuinea => write!(f, "Papua New Guinea"),
            Self::SouthernRegion => write!(f, "Southern Region"),
            Self::HighlandsRegion => write!(f, "Highlands Region"),
            Self::IslandsRegion => write!(f, "Islands Region"),
            Self::MomaseRegion => write!(f, "Momase Region"),
        }
    }
}

// Paraguay Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParaguaySubregions {
    Paraguay,
    Central,
    AltoParana,
    Amambay,
    Caaguazu,
    Caazapa,
    Concepcion,
    Cordillera,
    Guaira,
    Itapua,
    Misiones,
    Neembucu,
    Paraguari,
    PresidenteHayes,
    SanPedro,
    Canindeyu,
    Asuncion,
    AltoParaguay,
    Boqueron,
}

impl TryFrom<u8> for ParaguaySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Paraguay),
            0x02 => Ok(Self::Central),
            0x03 => Ok(Self::AltoParana),
            0x04 => Ok(Self::Amambay),
            0x05 => Ok(Self::Caaguazu),
            0x06 => Ok(Self::Caazapa),
            0x07 => Ok(Self::Concepcion),
            0x08 => Ok(Self::Cordillera),
            0x09 => Ok(Self::Guaira),
            0x0A => Ok(Self::Itapua),
            0x0B => Ok(Self::Misiones),
            0x0C => Ok(Self::Neembucu),
            0x0D => Ok(Self::Paraguari),
            0x0E => Ok(Self::PresidenteHayes),
            0x0F => Ok(Self::SanPedro),
            0x10 => Ok(Self::Canindeyu),
            0x11 => Ok(Self::Asuncion),
            0x12 => Ok(Self::AltoParaguay),
            0x13 => Ok(Self::Boqueron),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ParaguaySubregions> for u8 {
    fn from(value: ParaguaySubregions) -> u8 {
        match value {
            ParaguaySubregions::Paraguay => 0x01,
            ParaguaySubregions::Central => 0x02,
            ParaguaySubregions::AltoParana => 0x03,
            ParaguaySubregions::Amambay => 0x04,
            ParaguaySubregions::Caaguazu => 0x05,
            ParaguaySubregions::Caazapa => 0x06,
            ParaguaySubregions::Concepcion => 0x07,
            ParaguaySubregions::Cordillera => 0x08,
            ParaguaySubregions::Guaira => 0x09,
            ParaguaySubregions::Itapua => 0x0A,
            ParaguaySubregions::Misiones => 0x0B,
            ParaguaySubregions::Neembucu => 0x0C,
            ParaguaySubregions::Paraguari => 0x0D,
            ParaguaySubregions::PresidenteHayes => 0x0E,
            ParaguaySubregions::SanPedro => 0x0F,
            ParaguaySubregions::Canindeyu => 0x10,
            ParaguaySubregions::Asuncion => 0x11,
            ParaguaySubregions::AltoParaguay => 0x12,
            ParaguaySubregions::Boqueron => 0x13,
        }
    }
}

impl Display for ParaguaySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paraguay => write!(f, "Paraguay"),
            Self::Central => write!(f, "Central"),
            Self::AltoParana => write!(f, "Alto Paraná"),
            Self::Amambay => write!(f, "Amambay"),
            Self::Caaguazu => write!(f, "Caaguazú"),
            Self::Caazapa => write!(f, "Caazapá"),
            Self::Concepcion => write!(f, "Concepción"),
            Self::Cordillera => write!(f, "Cordillera"),
            Self::Guaira => write!(f, "Guairá"),
            Self::Itapua => write!(f, "Itapúa"),
            Self::Misiones => write!(f, "Misiones"),
            Self::Neembucu => write!(f, "Ñeembucú"),
            Self::Paraguari => write!(f, "Paraguarí"),
            Self::PresidenteHayes => write!(f, "Presidente Hayes"),
            Self::SanPedro => write!(f, "San Pedro"),
            Self::Canindeyu => write!(f, "Canindeyú"),
            Self::Asuncion => write!(f, "Asunción"),
            Self::AltoParaguay => write!(f, "Alto Paraguay"),
            Self::Boqueron => write!(f, "Boquerón"),
        }
    }
}

// Peru Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeruSubregions {
    Peru,
    Lima,
    Amazonas,
    Ancash,
    Apurimac,
    Arequipa,
    Ayacucho,
    Cajamarca,
    Callao,
    Cuzco,
    Huancavelica,
    Huanuco,
    Ica,
    Junin,
    LaLibertad,
    Lambayeque,
    Loreto,
    MadreDeDios,
    Moquegua,
    Pasco,
    Piura,
    Puno,
    SanMartin,
    Tacna,
    Tumbes,
    Ucayali,
}

impl TryFrom<u8> for PeruSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Peru),
            0x02 => Ok(Self::Lima),
            0x03 => Ok(Self::Amazonas),
            0x04 => Ok(Self::Ancash),
            0x05 => Ok(Self::Apurimac),
            0x06 => Ok(Self::Arequipa),
            0x07 => Ok(Self::Ayacucho),
            0x08 => Ok(Self::Cajamarca),
            0x09 => Ok(Self::Callao),
            0x0A => Ok(Self::Cuzco),
            0x0B => Ok(Self::Huancavelica),
            0x0C => Ok(Self::Huanuco),
            0x0D => Ok(Self::Ica),
            0x0E => Ok(Self::Junin),
            0x0F => Ok(Self::LaLibertad),
            0x10 => Ok(Self::Lambayeque),
            0x11 => Ok(Self::Loreto),
            0x12 => Ok(Self::MadreDeDios),
            0x13 => Ok(Self::Moquegua),
            0x14 => Ok(Self::Pasco),
            0x15 => Ok(Self::Piura),
            0x16 => Ok(Self::Puno),
            0x17 => Ok(Self::SanMartin),
            0x18 => Ok(Self::Tacna),
            0x19 => Ok(Self::Tumbes),
            0x1A => Ok(Self::Ucayali),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PeruSubregions> for u8 {
    fn from(value: PeruSubregions) -> u8 {
        match value {
            PeruSubregions::Peru => 0x01,
            PeruSubregions::Lima => 0x02,
            PeruSubregions::Amazonas => 0x03,
            PeruSubregions::Ancash => 0x04,
            PeruSubregions::Apurimac => 0x05,
            PeruSubregions::Arequipa => 0x06,
            PeruSubregions::Ayacucho => 0x07,
            PeruSubregions::Cajamarca => 0x08,
            PeruSubregions::Callao => 0x09,
            PeruSubregions::Cuzco => 0x0A,
            PeruSubregions::Huancavelica => 0x0B,
            PeruSubregions::Huanuco => 0x0C,
            PeruSubregions::Ica => 0x0D,
            PeruSubregions::Junin => 0x0E,
            PeruSubregions::LaLibertad => 0x0F,
            PeruSubregions::Lambayeque => 0x10,
            PeruSubregions::Loreto => 0x11,
            PeruSubregions::MadreDeDios => 0x12,
            PeruSubregions::Moquegua => 0x13,
            PeruSubregions::Pasco => 0x14,
            PeruSubregions::Piura => 0x15,
            PeruSubregions::Puno => 0x16,
            PeruSubregions::SanMartin => 0x17,
            PeruSubregions::Tacna => 0x18,
            PeruSubregions::Tumbes => 0x19,
            PeruSubregions::Ucayali => 0x1A,
        }
    }
}

impl Display for PeruSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Peru => write!(f, "Peru"),
            Self::Lima => write!(f, "Lima"),
            Self::Amazonas => write!(f, "Amazonas"),
            Self::Ancash => write!(f, "Ancash"),
            Self::Apurimac => write!(f, "Apurímac"),
            Self::Arequipa => write!(f, "Arequipa"),
            Self::Ayacucho => write!(f, "Ayacucho"),
            Self::Cajamarca => write!(f, "Cajamarca"),
            Self::Callao => write!(f, "Callao"),
            Self::Cuzco => write!(f, "Cuzco"),
            Self::Huancavelica => write!(f, "Huancavelica"),
            Self::Huanuco => write!(f, "Huánuco"),
            Self::Ica => write!(f, "Ica"),
            Self::Junin => write!(f, "Junín"),
            Self::LaLibertad => write!(f, "La Libertad"),
            Self::Lambayeque => write!(f, "Lambayeque"),
            Self::Loreto => write!(f, "Loreto"),
            Self::MadreDeDios => write!(f, "Madre de Dios"),
            Self::Moquegua => write!(f, "Moquegua"),
            Self::Pasco => write!(f, "Pasco"),
            Self::Piura => write!(f, "Piura"),
            Self::Puno => write!(f, "Puno"),
            Self::SanMartin => write!(f, "San Martín"),
            Self::Tacna => write!(f, "Tacna"),
            Self::Tumbes => write!(f, "Tumbes"),
            Self::Ucayali => write!(f, "Ucayali"),
        }
    }
}

// Philippines Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhilippinesSubregions {
    Philippines,
    Manila,
    AutonomousRegionInMuslimMindanao,
    Bicol,
    Cagayan,
    Calabarzon,
    Caraga,
    CentralLuzon,
    CentralVisayas,
    Cordillera,
    Davao,
    EasternVisayas,
    Ilocos,
    Mimaro,
    Mindanao,
    NorthernMindanao,
    Soccsksargen,
    WesternVisayas,
    ZamboangaPeninsula,
}

impl TryFrom<u8> for PhilippinesSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Philippines),
            0x02 => Ok(Self::Manila),
            0x03 => Ok(Self::AutonomousRegionInMuslimMindanao),
            0x04 => Ok(Self::Bicol),
            0x05 => Ok(Self::Cagayan),
            0x06 => Ok(Self::Calabarzon),
            0x07 => Ok(Self::Caraga),
            0x08 => Ok(Self::CentralLuzon),
            0x09 => Ok(Self::CentralVisayas),
            0x0A => Ok(Self::Cordillera),
            0x0B => Ok(Self::Davao),
            0x0C => Ok(Self::EasternVisayas),
            0x0D => Ok(Self::Ilocos),
            0x0E => Ok(Self::Mimaro),
            0x0F => Ok(Self::Mindanao),
            0x10 => Ok(Self::NorthernMindanao),
            0x11 => Ok(Self::Soccsksargen),
            0x12 => Ok(Self::WesternVisayas),
            0x13 => Ok(Self::ZamboangaPeninsula),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PhilippinesSubregions> for u8 {
    fn from(value: PhilippinesSubregions) -> u8 {
        match value {
            PhilippinesSubregions::Philippines => 0x01,
            PhilippinesSubregions::Manila => 0x02,
            PhilippinesSubregions::AutonomousRegionInMuslimMindanao => 0x03,
            PhilippinesSubregions::Bicol => 0x04,
            PhilippinesSubregions::Cagayan => 0x05,
            PhilippinesSubregions::Calabarzon => 0x06,
            PhilippinesSubregions::Caraga => 0x07,
            PhilippinesSubregions::CentralLuzon => 0x08,
            PhilippinesSubregions::CentralVisayas => 0x09,
            PhilippinesSubregions::Cordillera => 0x0A,
            PhilippinesSubregions::Davao => 0x0B,
            PhilippinesSubregions::EasternVisayas => 0x0C,
            PhilippinesSubregions::Ilocos => 0x0D,
            PhilippinesSubregions::Mimaro => 0x0E,
            PhilippinesSubregions::Mindanao => 0x0F,
            PhilippinesSubregions::NorthernMindanao => 0x10,
            PhilippinesSubregions::Soccsksargen => 0x11,
            PhilippinesSubregions::WesternVisayas => 0x12,
            PhilippinesSubregions::ZamboangaPeninsula => 0x13,
        }
    }
}

impl Display for PhilippinesSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Philippines => write!(f, "Philippines"),
            Self::Manila => write!(f, "Manila"),
            Self::AutonomousRegionInMuslimMindanao => {
                write!(f, "Autonomous Region in Muslim Mindanao")
            }
            Self::Bicol => write!(f, "Bicol"),
            Self::Cagayan => write!(f, "Cagayan"),
            Self::Calabarzon => write!(f, "Calabarzon"),
            Self::Caraga => write!(f, "Caraga"),
            Self::CentralLuzon => write!(f, "Central Luzon"),
            Self::CentralVisayas => write!(f, "Central Visayas"),
            Self::Cordillera => write!(f, "Cordillera"),
            Self::Davao => write!(f, "Davao"),
            Self::EasternVisayas => write!(f, "Eastern Visayas"),
            Self::Ilocos => write!(f, "Ilocos"),
            Self::Mimaro => write!(f, "Mimaro"),
            Self::Mindanao => write!(f, "Mindanao"),
            Self::NorthernMindanao => write!(f, "Northern Mindanao"),
            Self::Soccsksargen => write!(f, "Soccsksargen"),
            Self::WesternVisayas => write!(f, "Western Visayas"),
            Self::ZamboangaPeninsula => write!(f, "Zamboanga Peninsula"),
        }
    }
}

// Pitcairn Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitcairnIslandsSubregions {
    PitcairnIslands,
}

impl TryFrom<u8> for PitcairnIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::PitcairnIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PitcairnIslandsSubregions> for u8 {
    fn from(value: PitcairnIslandsSubregions) -> u8 {
        match value {
            PitcairnIslandsSubregions::PitcairnIslands => 0x01,
        }
    }
}

impl Display for PitcairnIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PitcairnIslands => write!(f, "Pitcairn Islands"),
        }
    }
}

// Poland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolandSubregions {
    Poland,
    Masovia,
    LowerSilesia,
    KuyaviaPomerania,
    Lodz,
    Lublin,
    Lubusz,
    LesserPoland,
    Opole,
    Subcarpathia,
    Podlasie,
    Pomerania,
    Silesia,
    Swietokrzyskie,
    WarmiaMasuria,
    GreaterPoland,
    WesternPomerania,
}

impl TryFrom<u8> for PolandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Poland),
            0x02 => Ok(Self::Masovia),
            0x03 => Ok(Self::LowerSilesia),
            0x04 => Ok(Self::KuyaviaPomerania),
            0x05 => Ok(Self::Lodz),
            0x06 => Ok(Self::Lublin),
            0x07 => Ok(Self::Lubusz),
            0x08 => Ok(Self::LesserPoland),
            0x09 => Ok(Self::Opole),
            0x0A => Ok(Self::Subcarpathia),
            0x0B => Ok(Self::Podlasie),
            0x0C => Ok(Self::Pomerania),
            0x0D => Ok(Self::Silesia),
            0x0E => Ok(Self::Swietokrzyskie),
            0x0F => Ok(Self::WarmiaMasuria),
            0x10 => Ok(Self::GreaterPoland),
            0x11 => Ok(Self::WesternPomerania),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PolandSubregions> for u8 {
    fn from(value: PolandSubregions) -> u8 {
        match value {
            PolandSubregions::Poland => 0x01,
            PolandSubregions::Masovia => 0x02,
            PolandSubregions::LowerSilesia => 0x03,
            PolandSubregions::KuyaviaPomerania => 0x04,
            PolandSubregions::Lodz => 0x05,
            PolandSubregions::Lublin => 0x06,
            PolandSubregions::Lubusz => 0x07,
            PolandSubregions::LesserPoland => 0x08,
            PolandSubregions::Opole => 0x09,
            PolandSubregions::Subcarpathia => 0x0A,
            PolandSubregions::Podlasie => 0x0B,
            PolandSubregions::Pomerania => 0x0C,
            PolandSubregions::Silesia => 0x0D,
            PolandSubregions::Swietokrzyskie => 0x0E,
            PolandSubregions::WarmiaMasuria => 0x0F,
            PolandSubregions::GreaterPoland => 0x10,
            PolandSubregions::WesternPomerania => 0x11,
        }
    }
}

impl Display for PolandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Poland => write!(f, "Poland"),
            Self::Masovia => write!(f, "Masovia"),
            Self::LowerSilesia => write!(f, "Lower Silesia"),
            Self::KuyaviaPomerania => write!(f, "Kuyavia-Pomerania"),
            Self::Lodz => write!(f, "Lodz"),
            Self::Lublin => write!(f, "Lublin"),
            Self::Lubusz => write!(f, "Lubusz"),
            Self::LesserPoland => write!(f, "Lesser Poland"),
            Self::Opole => write!(f, "Opole"),
            Self::Subcarpathia => write!(f, "Subcarpathia"),
            Self::Podlasie => write!(f, "Podlasie"),
            Self::Pomerania => write!(f, "Pomerania"),
            Self::Silesia => write!(f, "Silesia"),
            Self::Swietokrzyskie => write!(f, "Swietokrzyskie"),
            Self::WarmiaMasuria => write!(f, "Warmia-Masuria"),
            Self::GreaterPoland => write!(f, "Greater Poland"),
            Self::WesternPomerania => write!(f, "Western Pomerania"),
        }
    }
}

// Portugal Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortugalSubregions {
    Portugal,
    Lisbon,
    Alentejo,
    Algarve,
    Centro,
    Norte,
    Madeira,
    Azores,
}

impl TryFrom<u8> for PortugalSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Portugal),
            0x02 => Ok(Self::Lisbon),
            0x03 => Ok(Self::Alentejo),
            0x04 => Ok(Self::Algarve),
            0x05 => Ok(Self::Centro),
            0x06 => Ok(Self::Norte),
            0x07 => Ok(Self::Madeira),
            0x08 => Ok(Self::Azores),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PortugalSubregions> for u8 {
    fn from(value: PortugalSubregions) -> u8 {
        match value {
            PortugalSubregions::Portugal => 0x01,
            PortugalSubregions::Lisbon => 0x02,
            PortugalSubregions::Alentejo => 0x03,
            PortugalSubregions::Algarve => 0x04,
            PortugalSubregions::Centro => 0x05,
            PortugalSubregions::Norte => 0x06,
            PortugalSubregions::Madeira => 0x07,
            PortugalSubregions::Azores => 0x08,
        }
    }
}

impl Display for PortugalSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Portugal => write!(f, "Portugal"),
            Self::Lisbon => write!(f, "Lisbon"),
            Self::Alentejo => write!(f, "Alentejo"),
            Self::Algarve => write!(f, "Algarve"),
            Self::Centro => write!(f, "Centro"),
            Self::Norte => write!(f, "Norte"),
            Self::Madeira => write!(f, "Madeira"),
            Self::Azores => write!(f, "Azores"),
        }
    }
}

// Puerto Rico Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PuertoRicoSubregions {
    PuertoRico,
}

impl TryFrom<u8> for PuertoRicoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::PuertoRico),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<PuertoRicoSubregions> for u8 {
    fn from(value: PuertoRicoSubregions) -> u8 {
        match value {
            PuertoRicoSubregions::PuertoRico => 0x01,
        }
    }
}

impl Display for PuertoRicoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PuertoRico => write!(f, "Puerto Rico"),
        }
    }
}

// Qatar Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QatarSubregions {
    Qatar,
    AdDawhah,
    AlDaayen,
    AlKhor,
    AlRayyan,
    AlShamal,
    AlWakrah,
    AlShahaniya,
    UmmSalal,
}

impl TryFrom<u8> for QatarSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Qatar),
            0x02 => Ok(Self::AdDawhah),
            0x03 => Ok(Self::AlDaayen),
            0x04 => Ok(Self::AlKhor),
            0x05 => Ok(Self::AlRayyan),
            0x06 => Ok(Self::AlShamal),
            0x07 => Ok(Self::AlWakrah),
            0x08 => Ok(Self::AlShahaniya),
            0x09 => Ok(Self::UmmSalal),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<QatarSubregions> for u8 {
    fn from(value: QatarSubregions) -> u8 {
        match value {
            QatarSubregions::Qatar => 0x01,
            QatarSubregions::AdDawhah => 0x02,
            QatarSubregions::AlDaayen => 0x03,
            QatarSubregions::AlKhor => 0x04,
            QatarSubregions::AlRayyan => 0x05,
            QatarSubregions::AlShamal => 0x06,
            QatarSubregions::AlWakrah => 0x07,
            QatarSubregions::AlShahaniya => 0x08,
            QatarSubregions::UmmSalal => 0x09,
        }
    }
}

impl Display for QatarSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Qatar => write!(f, "Qatar"),
            Self::AdDawhah => write!(f, "Ad-Dawhah"),
            Self::AlDaayen => write!(f, "Al Daayen"),
            Self::AlKhor => write!(f, "Al Khor"),
            Self::AlRayyan => write!(f, "Al Rayyan"),
            Self::AlShamal => write!(f, "Al Shamal"),
            Self::AlWakrah => write!(f, "Al Wakrah"),
            Self::AlShahaniya => write!(f, "Al Shahaniya"),
            Self::UmmSalal => write!(f, "Umm Salal"),
        }
    }
}

// Republic of the Congo Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepublicOfTheCongoSubregions {
    RepublicOfTheCongo,
    Brazzaville,
    Bouenza,
    Cuvette,
    CuvetteOuest,
    Kouilou,
    Lekoumou,
    Likouala,
    Niari,
    Plateaux,
    PointeNoire,
    Pool,
    Sangha,
}

impl TryFrom<u8> for RepublicOfTheCongoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::RepublicOfTheCongo),
            0x02 => Ok(Self::Brazzaville),
            0x03 => Ok(Self::Bouenza),
            0x04 => Ok(Self::Cuvette),
            0x06 => Ok(Self::CuvetteOuest),
            0x07 => Ok(Self::Kouilou),
            0x08 => Ok(Self::Lekoumou),
            0x09 => Ok(Self::Likouala),
            0x0A => Ok(Self::Niari),
            0x0B => Ok(Self::Plateaux),
            0x0C => Ok(Self::PointeNoire),
            0x0D => Ok(Self::Pool),
            0x0E => Ok(Self::Sangha),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<RepublicOfTheCongoSubregions> for u8 {
    fn from(value: RepublicOfTheCongoSubregions) -> u8 {
        match value {
            RepublicOfTheCongoSubregions::RepublicOfTheCongo => 0x01,
            RepublicOfTheCongoSubregions::Brazzaville => 0x02,
            RepublicOfTheCongoSubregions::Bouenza => 0x03,
            RepublicOfTheCongoSubregions::Cuvette => 0x04,
            RepublicOfTheCongoSubregions::CuvetteOuest => 0x06,
            RepublicOfTheCongoSubregions::Kouilou => 0x07,
            RepublicOfTheCongoSubregions::Lekoumou => 0x08,
            RepublicOfTheCongoSubregions::Likouala => 0x09,
            RepublicOfTheCongoSubregions::Niari => 0x0A,
            RepublicOfTheCongoSubregions::Plateaux => 0x0B,
            RepublicOfTheCongoSubregions::PointeNoire => 0x0C,
            RepublicOfTheCongoSubregions::Pool => 0x0D,
            RepublicOfTheCongoSubregions::Sangha => 0x0E,
        }
    }
}

impl Display for RepublicOfTheCongoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepublicOfTheCongo => write!(f, "Republic of the Congo"),
            Self::Brazzaville => write!(f, "Brazzaville"),
            Self::Bouenza => write!(f, "Bouenza"),
            Self::Cuvette => write!(f, "Cuvette"),
            Self::CuvetteOuest => write!(f, "Cuvette-Ouest"),
            Self::Kouilou => write!(f, "Kouilou"),
            Self::Lekoumou => write!(f, "Lékoumou"),
            Self::Likouala => write!(f, "Likouala"),
            Self::Niari => write!(f, "Niari"),
            Self::Plateaux => write!(f, "Plateaux"),
            Self::PointeNoire => write!(f, "Pointe-Noire"),
            Self::Pool => write!(f, "Pool"),
            Self::Sangha => write!(f, "Sangha"),
        }
    }
}

// Romania Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RomaniaSubregions {
    Romania,
    Bucharest,
    Alba,
    Arad,
    Arges,
    Bacau,
    Bihor,
    BistritaNasaud,
    Botosani,
    Braila,
    Brasov,
    Buzau,
    Calarasi,
    CarasSeverin,
    Cluj,
    Constanta,
    Covasna,
    Dambovita,
    Dolj,
    Galati,
    Giurgiu,
    Gorj,
    Harghita,
    Hunedoara,
    Ialomita,
    Iasi,
    Ilfov,
    Maramures,
    Mehedinti,
    Mures,
    Neamt,
    Olt,
    Prahova,
    Salaj,
    SatuMare,
    Sibiu,
    Suceava,
    Teleorman,
    Timis,
    Tulcea,
    Valcea,
    Vaslui,
    Vrancea,
}

impl TryFrom<u8> for RomaniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Romania),
            0x02 => Ok(Self::Bucharest),
            0x03 => Ok(Self::Alba),
            0x04 => Ok(Self::Arad),
            0x05 => Ok(Self::Arges),
            0x06 => Ok(Self::Bacau),
            0x07 => Ok(Self::Bihor),
            0x08 => Ok(Self::BistritaNasaud),
            0x09 => Ok(Self::Botosani),
            0x0A => Ok(Self::Braila),
            0x0B => Ok(Self::Brasov),
            0x0C => Ok(Self::Buzau),
            0x0D => Ok(Self::Calarasi),
            0x0E => Ok(Self::CarasSeverin),
            0x0F => Ok(Self::Cluj),
            0x10 => Ok(Self::Constanta),
            0x11 => Ok(Self::Covasna),
            0x12 => Ok(Self::Dambovita),
            0x13 => Ok(Self::Dolj),
            0x14 => Ok(Self::Galati),
            0x15 => Ok(Self::Giurgiu),
            0x16 => Ok(Self::Gorj),
            0x17 => Ok(Self::Harghita),
            0x18 => Ok(Self::Hunedoara),
            0x19 => Ok(Self::Ialomita),
            0x1A => Ok(Self::Iasi),
            0x1B => Ok(Self::Ilfov),
            0x1C => Ok(Self::Maramures),
            0x1D => Ok(Self::Mehedinti),
            0x1E => Ok(Self::Mures),
            0x1F => Ok(Self::Neamt),
            0x20 => Ok(Self::Olt),
            0x21 => Ok(Self::Prahova),
            0x22 => Ok(Self::Salaj),
            0x23 => Ok(Self::SatuMare),
            0x24 => Ok(Self::Sibiu),
            0x25 => Ok(Self::Suceava),
            0x26 => Ok(Self::Teleorman),
            0x27 => Ok(Self::Timis),
            0x28 => Ok(Self::Tulcea),
            0x29 => Ok(Self::Valcea),
            0x2A => Ok(Self::Vaslui),
            0x2B => Ok(Self::Vrancea),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<RomaniaSubregions> for u8 {
    fn from(value: RomaniaSubregions) -> u8 {
        match value {
            RomaniaSubregions::Romania => 0x01,
            RomaniaSubregions::Bucharest => 0x02,
            RomaniaSubregions::Alba => 0x03,
            RomaniaSubregions::Arad => 0x04,
            RomaniaSubregions::Arges => 0x05,
            RomaniaSubregions::Bacau => 0x06,
            RomaniaSubregions::Bihor => 0x07,
            RomaniaSubregions::BistritaNasaud => 0x08,
            RomaniaSubregions::Botosani => 0x09,
            RomaniaSubregions::Braila => 0x0A,
            RomaniaSubregions::Brasov => 0x0B,
            RomaniaSubregions::Buzau => 0x0C,
            RomaniaSubregions::Calarasi => 0x0D,
            RomaniaSubregions::CarasSeverin => 0x0E,
            RomaniaSubregions::Cluj => 0x0F,
            RomaniaSubregions::Constanta => 0x10,
            RomaniaSubregions::Covasna => 0x11,
            RomaniaSubregions::Dambovita => 0x12,
            RomaniaSubregions::Dolj => 0x13,
            RomaniaSubregions::Galati => 0x14,
            RomaniaSubregions::Giurgiu => 0x15,
            RomaniaSubregions::Gorj => 0x16,
            RomaniaSubregions::Harghita => 0x17,
            RomaniaSubregions::Hunedoara => 0x18,
            RomaniaSubregions::Ialomita => 0x19,
            RomaniaSubregions::Iasi => 0x1A,
            RomaniaSubregions::Ilfov => 0x1B,
            RomaniaSubregions::Maramures => 0x1C,
            RomaniaSubregions::Mehedinti => 0x1D,
            RomaniaSubregions::Mures => 0x1E,
            RomaniaSubregions::Neamt => 0x1F,
            RomaniaSubregions::Olt => 0x20,
            RomaniaSubregions::Prahova => 0x21,
            RomaniaSubregions::Salaj => 0x22,
            RomaniaSubregions::SatuMare => 0x23,
            RomaniaSubregions::Sibiu => 0x24,
            RomaniaSubregions::Suceava => 0x25,
            RomaniaSubregions::Teleorman => 0x26,
            RomaniaSubregions::Timis => 0x27,
            RomaniaSubregions::Tulcea => 0x28,
            RomaniaSubregions::Valcea => 0x29,
            RomaniaSubregions::Vaslui => 0x2A,
            RomaniaSubregions::Vrancea => 0x2B,
        }
    }
}

impl Display for RomaniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Romania => write!(f, "Romania"),
            Self::Bucharest => write!(f, "Bucharest"),
            Self::Alba => write!(f, "Alba"),
            Self::Arad => write!(f, "Arad"),
            Self::Arges => write!(f, "Arges"),
            Self::Bacau => write!(f, "Bacau"),
            Self::Bihor => write!(f, "Bihor"),
            Self::BistritaNasaud => write!(f, "Bistrita-Nasaud"),
            Self::Botosani => write!(f, "Botosani"),
            Self::Braila => write!(f, "Braila"),
            Self::Brasov => write!(f, "Brasov"),
            Self::Buzau => write!(f, "Buzau"),
            Self::Calarasi => write!(f, "Calarasi"),
            Self::CarasSeverin => write!(f, "Caras-Severin"),
            Self::Cluj => write!(f, "Cluj"),
            Self::Constanta => write!(f, "Constanta"),
            Self::Covasna => write!(f, "Covasna"),
            Self::Dambovita => write!(f, "Dâmbovita"),
            Self::Dolj => write!(f, "Dolj"),
            Self::Galati => write!(f, "Galati"),
            Self::Giurgiu => write!(f, "Giurgiu"),
            Self::Gorj => write!(f, "Gorj"),
            Self::Harghita => write!(f, "Harghita"),
            Self::Hunedoara => write!(f, "Hunedoara"),
            Self::Ialomita => write!(f, "Ialomita"),
            Self::Iasi => write!(f, "Iasi"),
            Self::Ilfov => write!(f, "Ilfov"),
            Self::Maramures => write!(f, "Maramures"),
            Self::Mehedinti => write!(f, "Mehedinti"),
            Self::Mures => write!(f, "Mures"),
            Self::Neamt => write!(f, "Neamt"),
            Self::Olt => write!(f, "Olt"),
            Self::Prahova => write!(f, "Prahova"),
            Self::Salaj => write!(f, "Salaj"),
            Self::SatuMare => write!(f, "Satu Mare"),
            Self::Sibiu => write!(f, "Sibiu"),
            Self::Suceava => write!(f, "Suceava"),
            Self::Teleorman => write!(f, "Teleorman"),
            Self::Timis => write!(f, "Timis"),
            Self::Tulcea => write!(f, "Tulcea"),
            Self::Valcea => write!(f, "Vâlcea"),
            Self::Vaslui => write!(f, "Vaslui"),
            Self::Vrancea => write!(f, "Vrancea"),
        }
    }
}

// Russia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RussiaSubregions {
    Russia,
    CentralFederalDistrict,
    FarEasternFederalDistrict,
    NorthwesternFederalDistrict,
    SiberianFederalDistrict,
    SouthernFederalDistrict,
    UralsFederalDistrict,
    VolgaFederalDistrict,
}

impl TryFrom<u8> for RussiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Russia),
            0x02 => Ok(Self::CentralFederalDistrict),
            0x03 => Ok(Self::FarEasternFederalDistrict),
            0x04 => Ok(Self::NorthwesternFederalDistrict),
            0x05 => Ok(Self::SiberianFederalDistrict),
            0x06 => Ok(Self::SouthernFederalDistrict),
            0x07 => Ok(Self::UralsFederalDistrict),
            0x08 => Ok(Self::VolgaFederalDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<RussiaSubregions> for u8 {
    fn from(value: RussiaSubregions) -> u8 {
        match value {
            RussiaSubregions::Russia => 0x01,
            RussiaSubregions::CentralFederalDistrict => 0x02,
            RussiaSubregions::FarEasternFederalDistrict => 0x03,
            RussiaSubregions::NorthwesternFederalDistrict => 0x04,
            RussiaSubregions::SiberianFederalDistrict => 0x05,
            RussiaSubregions::SouthernFederalDistrict => 0x06,
            RussiaSubregions::UralsFederalDistrict => 0x07,
            RussiaSubregions::VolgaFederalDistrict => 0x08,
        }
    }
}

impl Display for RussiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Russia => write!(f, "Russia"),
            Self::CentralFederalDistrict => write!(f, "Central Federal District"),
            Self::FarEasternFederalDistrict => write!(f, "Far Eastern Federal District"),
            Self::NorthwesternFederalDistrict => write!(f, "Northwestern Federal District"),
            Self::SiberianFederalDistrict => write!(f, "Siberian Federal District"),
            Self::SouthernFederalDistrict => write!(f, "Southern Federal District"),
            Self::UralsFederalDistrict => write!(f, "Urals Federal District"),
            Self::VolgaFederalDistrict => write!(f, "Volga Federal District"),
        }
    }
}

// Rwanda Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RwandaSubregions {
    Rwanda,
    KigaliProvince,
    EasternProvince,
    NorthernProvince,
    SouthernProvince,
    WesternProvince,
}

impl TryFrom<u8> for RwandaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Rwanda),
            0x02 => Ok(Self::KigaliProvince),
            0x03 => Ok(Self::EasternProvince),
            0x04 => Ok(Self::NorthernProvince),
            0x05 => Ok(Self::SouthernProvince),
            0x06 => Ok(Self::WesternProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<RwandaSubregions> for u8 {
    fn from(value: RwandaSubregions) -> u8 {
        match value {
            RwandaSubregions::Rwanda => 0x01,
            RwandaSubregions::KigaliProvince => 0x02,
            RwandaSubregions::EasternProvince => 0x03,
            RwandaSubregions::NorthernProvince => 0x04,
            RwandaSubregions::SouthernProvince => 0x05,
            RwandaSubregions::WesternProvince => 0x06,
        }
    }
}

impl Display for RwandaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rwanda => write!(f, "Rwanda"),
            Self::KigaliProvince => write!(f, "Kigali Province"),
            Self::EasternProvince => write!(f, "Eastern Province"),
            Self::NorthernProvince => write!(f, "Northern Province"),
            Self::SouthernProvince => write!(f, "Southern Province"),
            Self::WesternProvince => write!(f, "Western Province"),
        }
    }
}

// Réunion Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReunionSubregions {
    Reunion,
}

impl TryFrom<u8> for ReunionSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Reunion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ReunionSubregions> for u8 {
    fn from(value: ReunionSubregions) -> u8 {
        match value {
            ReunionSubregions::Reunion => 0x01,
        }
    }
}

impl Display for ReunionSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reunion => write!(f, "Réunion"),
        }
    }
}

// Sahrawi Arab Democratic Republic Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SahrawiArabDemocraticRepublicSubregions {
    SahrawiArabDemocraticRepublic,
    Tifariti,
    Agounit,
    BirLehlou,
    BirTiguisit,
    Dougaj,
    Meharrize,
    Mijek,
    SouthernProvinces,
    Zug,
}

impl TryFrom<u8> for SahrawiArabDemocraticRepublicSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SahrawiArabDemocraticRepublic),
            0x02 => Ok(Self::Tifariti),
            0x03 => Ok(Self::Agounit),
            0x04 => Ok(Self::BirLehlou),
            0x05 => Ok(Self::BirTiguisit),
            0x06 => Ok(Self::Dougaj),
            0x07 => Ok(Self::Meharrize),
            0x08 => Ok(Self::Mijek),
            0x09 => Ok(Self::SouthernProvinces),
            0x0A => Ok(Self::Zug),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SahrawiArabDemocraticRepublicSubregions> for u8 {
    fn from(value: SahrawiArabDemocraticRepublicSubregions) -> u8 {
        match value {
            SahrawiArabDemocraticRepublicSubregions::SahrawiArabDemocraticRepublic => 0x01,
            SahrawiArabDemocraticRepublicSubregions::Tifariti => 0x02,
            SahrawiArabDemocraticRepublicSubregions::Agounit => 0x03,
            SahrawiArabDemocraticRepublicSubregions::BirLehlou => 0x04,
            SahrawiArabDemocraticRepublicSubregions::BirTiguisit => 0x05,
            SahrawiArabDemocraticRepublicSubregions::Dougaj => 0x06,
            SahrawiArabDemocraticRepublicSubregions::Meharrize => 0x07,
            SahrawiArabDemocraticRepublicSubregions::Mijek => 0x08,
            SahrawiArabDemocraticRepublicSubregions::SouthernProvinces => 0x09,
            SahrawiArabDemocraticRepublicSubregions::Zug => 0x0A,
        }
    }
}

impl Display for SahrawiArabDemocraticRepublicSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SahrawiArabDemocraticRepublic => write!(f, "Sahrawi Arab Democratic Republic"),
            Self::Tifariti => write!(f, "Tifariti"),
            Self::Agounit => write!(f, "Agounit"),
            Self::BirLehlou => write!(f, "Bir Lehlou"),
            Self::BirTiguisit => write!(f, "Bir Tiguisit"),
            Self::Dougaj => write!(f, "Dougaj"),
            Self::Meharrize => write!(f, "Meharrize"),
            Self::Mijek => write!(f, "Mijek"),
            Self::SouthernProvinces => write!(f, "Southern Provinces"),
            Self::Zug => write!(f, "Zug"),
        }
    }
}

// Saint Barthélemy Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintBarthelemySubregions {
    SaintBarthelemy,
    SousLeVentParish,
    AuVentParish,
}

impl TryFrom<u8> for SaintBarthelemySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SaintBarthelemy),
            0x02 => Ok(Self::SousLeVentParish),
            0x03 => Ok(Self::AuVentParish),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SaintBarthelemySubregions> for u8 {
    fn from(value: SaintBarthelemySubregions) -> u8 {
        match value {
            SaintBarthelemySubregions::SaintBarthelemy => 0x01,
            SaintBarthelemySubregions::SousLeVentParish => 0x02,
            SaintBarthelemySubregions::AuVentParish => 0x03,
        }
    }
}

impl Display for SaintBarthelemySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaintBarthelemy => write!(f, "Saint Barthélemy"),
            Self::SousLeVentParish => write!(f, "Sous le Vent Parish"),
            Self::AuVentParish => write!(f, "Au Vent Parish"),
        }
    }
}

// Saint Helena, Ascension and Tristan da Cunha Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintHelenaAscensionAndTristanDaCunhaSubregions {
    SaintHelenaAscensionAndTristanDaCunha,
    SaintHelena,
    AscensionIsland,
    TristanDaCunha,
}

impl TryFrom<u8> for SaintHelenaAscensionAndTristanDaCunhaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SaintHelenaAscensionAndTristanDaCunha),
            0x02 => Ok(Self::SaintHelena),
            0x03 => Ok(Self::AscensionIsland),
            0x04 => Ok(Self::TristanDaCunha),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SaintHelenaAscensionAndTristanDaCunhaSubregions> for u8 {
    fn from(value: SaintHelenaAscensionAndTristanDaCunhaSubregions) -> u8 {
        match value {
            SaintHelenaAscensionAndTristanDaCunhaSubregions::SaintHelenaAscensionAndTristanDaCunha => 0x01,
            SaintHelenaAscensionAndTristanDaCunhaSubregions::SaintHelena => 0x02,
            SaintHelenaAscensionAndTristanDaCunhaSubregions::AscensionIsland => 0x03,
            SaintHelenaAscensionAndTristanDaCunhaSubregions::TristanDaCunha => 0x04,
        }
    }
}

impl Display for SaintHelenaAscensionAndTristanDaCunhaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaintHelenaAscensionAndTristanDaCunha => {
                write!(f, "Saint Helena, Ascension and Tristan da Cunha")
            }
            Self::SaintHelena => write!(f, "Saint Helena"),
            Self::AscensionIsland => write!(f, "Ascension Island"),
            Self::TristanDaCunha => write!(f, "Tristan da Cunha"),
        }
    }
}

// Saint Martin Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintMartinSubregions {
    SaintMartin,
}

impl TryFrom<u8> for SaintMartinSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SaintMartin),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SaintMartinSubregions> for u8 {
    fn from(value: SaintMartinSubregions) -> u8 {
        match value {
            SaintMartinSubregions::SaintMartin => 0x01,
        }
    }
}

impl Display for SaintMartinSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaintMartin => write!(f, "Saint Martin"),
        }
    }
}

// Saint Pierre and Miquelon Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaintPierreAndMiquelonSubregions {
    SaintPierreAndMiquelon,
    SaintPierre,
    Miquelon,
}

impl TryFrom<u8> for SaintPierreAndMiquelonSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SaintPierreAndMiquelon),
            0x02 => Ok(Self::SaintPierre),
            0x03 => Ok(Self::Miquelon),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SaintPierreAndMiquelonSubregions> for u8 {
    fn from(value: SaintPierreAndMiquelonSubregions) -> u8 {
        match value {
            SaintPierreAndMiquelonSubregions::SaintPierreAndMiquelon => 0x01,
            SaintPierreAndMiquelonSubregions::SaintPierre => 0x02,
            SaintPierreAndMiquelonSubregions::Miquelon => 0x03,
        }
    }
}

impl Display for SaintPierreAndMiquelonSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaintPierreAndMiquelon => write!(f, "Saint Pierre and Miquelon"),
            Self::SaintPierre => write!(f, "Saint Pierre"),
            Self::Miquelon => write!(f, "Miquelon"),
        }
    }
}

// Samoa Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SamoaSubregions {
    Samoa,
    Upolu,
    Savaii,
}

impl TryFrom<u8> for SamoaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Samoa),
            0x02 => Ok(Self::Upolu),
            0x03 => Ok(Self::Savaii),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SamoaSubregions> for u8 {
    fn from(value: SamoaSubregions) -> u8 {
        match value {
            SamoaSubregions::Samoa => 0x01,
            SamoaSubregions::Upolu => 0x02,
            SamoaSubregions::Savaii => 0x03,
        }
    }
}

impl Display for SamoaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Samoa => write!(f, "Samoa"),
            Self::Upolu => write!(f, "Upolu"),
            Self::Savaii => write!(f, "Savai'i"),
        }
    }
}

// San Marino Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SanMarinoSubregions {
    SanMarino,
}

impl TryFrom<u8> for SanMarinoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SanMarino),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SanMarinoSubregions> for u8 {
    fn from(value: SanMarinoSubregions) -> u8 {
        match value {
            SanMarinoSubregions::SanMarino => 0x01,
        }
    }
}

impl Display for SanMarinoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SanMarino => write!(f, "San Marino"),
        }
    }
}

// Saudi Arabia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaudiArabiaSubregions {
    SaudiArabia,
    ArRiyad,
    AlBahah,
    AlMadinah,
    AshSharqiyah,
    AlQasim,
    Asir,
    Hail,
    Makkah,
    AlHududAshShamaliyah,
    Najran,
    Jizan,
    Tabuk,
    AlJawf,
}

impl TryFrom<u8> for SaudiArabiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SaudiArabia),
            0x02 => Ok(Self::ArRiyad),
            0x03 => Ok(Self::AlBahah),
            0x04 => Ok(Self::AlMadinah),
            0x05 => Ok(Self::AshSharqiyah),
            0x06 => Ok(Self::AlQasim),
            0x07 => Ok(Self::Asir),
            0x08 => Ok(Self::Hail),
            0x09 => Ok(Self::Makkah),
            0x0A => Ok(Self::AlHududAshShamaliyah),
            0x0B => Ok(Self::Najran),
            0x0C => Ok(Self::Jizan),
            0x0D => Ok(Self::Tabuk),
            0x0E => Ok(Self::AlJawf),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SaudiArabiaSubregions> for u8 {
    fn from(value: SaudiArabiaSubregions) -> u8 {
        match value {
            SaudiArabiaSubregions::SaudiArabia => 0x01,
            SaudiArabiaSubregions::ArRiyad => 0x02,
            SaudiArabiaSubregions::AlBahah => 0x03,
            SaudiArabiaSubregions::AlMadinah => 0x04,
            SaudiArabiaSubregions::AshSharqiyah => 0x05,
            SaudiArabiaSubregions::AlQasim => 0x06,
            SaudiArabiaSubregions::Asir => 0x07,
            SaudiArabiaSubregions::Hail => 0x08,
            SaudiArabiaSubregions::Makkah => 0x09,
            SaudiArabiaSubregions::AlHududAshShamaliyah => 0x0A,
            SaudiArabiaSubregions::Najran => 0x0B,
            SaudiArabiaSubregions::Jizan => 0x0C,
            SaudiArabiaSubregions::Tabuk => 0x0D,
            SaudiArabiaSubregions::AlJawf => 0x0E,
        }
    }
}

impl Display for SaudiArabiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaudiArabia => write!(f, "Saudi Arabia"),
            Self::ArRiyad => write!(f, "Ar Riyad"),
            Self::AlBahah => write!(f, "Al Bahah"),
            Self::AlMadinah => write!(f, "Al Madinah"),
            Self::AshSharqiyah => write!(f, "Ash Sharqiyah"),
            Self::AlQasim => write!(f, "Al Qasim"),
            Self::Asir => write!(f, "Asir"),
            Self::Hail => write!(f, "Ha'il"),
            Self::Makkah => write!(f, "Makkah"),
            Self::AlHududAshShamaliyah => write!(f, "Al Hudud ash Shamaliyah"),
            Self::Najran => write!(f, "Najran"),
            Self::Jizan => write!(f, "Jizan"),
            Self::Tabuk => write!(f, "Tabuk"),
            Self::AlJawf => write!(f, "Al Jawf"),
        }
    }
}

// Scotland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScotlandSubregions {
    Scotland,
    Edinburgh,
    Aberdeen,
    Aberdeenshire,
    Angus,
    ArgyllAndBute,
    Clackmannanshire,
    DumfriesAndGalloway,
    Dundee,
    EastAyrshire,
    EastDunbartonshire,
    EastLothian,
    EastRenfrewshire,
    Falkirk,
    Fife,
    Glasgow,
    Highland,
    Inverclyde,
    Midlothian,
    Moray,
    NaHEileananSiar,
    NorthAyrshire,
    NorthLanarkshire,
    OrkneyIslands,
    PerthAndKinross,
    Renfrewshire,
    ScottishBorders,
    ShetlandIslands,
    SouthAyrshire,
    SouthLanarkshire,
    Stirling,
    WestDunbartonshire,
    WestLothian,
}

impl TryFrom<u8> for ScotlandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Scotland),
            0x02 => Ok(Self::Edinburgh),
            0x03 => Ok(Self::Aberdeen),
            0x04 => Ok(Self::Aberdeenshire),
            0x05 => Ok(Self::Angus),
            0x06 => Ok(Self::ArgyllAndBute),
            0x07 => Ok(Self::Clackmannanshire),
            0x08 => Ok(Self::DumfriesAndGalloway),
            0x09 => Ok(Self::Dundee),
            0x0A => Ok(Self::EastAyrshire),
            0x0B => Ok(Self::EastDunbartonshire),
            0x0C => Ok(Self::EastLothian),
            0x0D => Ok(Self::EastRenfrewshire),
            0x0E => Ok(Self::Falkirk),
            0x0F => Ok(Self::Fife),
            0x10 => Ok(Self::Glasgow),
            0x11 => Ok(Self::Highland),
            0x12 => Ok(Self::Inverclyde),
            0x13 => Ok(Self::Midlothian),
            0x14 => Ok(Self::Moray),
            0x15 => Ok(Self::NaHEileananSiar),
            0x16 => Ok(Self::NorthAyrshire),
            0x17 => Ok(Self::NorthLanarkshire),
            0x18 => Ok(Self::OrkneyIslands),
            0x19 => Ok(Self::PerthAndKinross),
            0x1A => Ok(Self::Renfrewshire),
            0x1B => Ok(Self::ScottishBorders),
            0x1C => Ok(Self::ShetlandIslands),
            0x1D => Ok(Self::SouthAyrshire),
            0x1E => Ok(Self::SouthLanarkshire),
            0x1F => Ok(Self::Stirling),
            0x20 => Ok(Self::WestDunbartonshire),
            0x21 => Ok(Self::WestLothian),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ScotlandSubregions> for u8 {
    fn from(value: ScotlandSubregions) -> u8 {
        match value {
            ScotlandSubregions::Scotland => 0x01,
            ScotlandSubregions::Edinburgh => 0x02,
            ScotlandSubregions::Aberdeen => 0x03,
            ScotlandSubregions::Aberdeenshire => 0x04,
            ScotlandSubregions::Angus => 0x05,
            ScotlandSubregions::ArgyllAndBute => 0x06,
            ScotlandSubregions::Clackmannanshire => 0x07,
            ScotlandSubregions::DumfriesAndGalloway => 0x08,
            ScotlandSubregions::Dundee => 0x09,
            ScotlandSubregions::EastAyrshire => 0x0A,
            ScotlandSubregions::EastDunbartonshire => 0x0B,
            ScotlandSubregions::EastLothian => 0x0C,
            ScotlandSubregions::EastRenfrewshire => 0x0D,
            ScotlandSubregions::Falkirk => 0x0E,
            ScotlandSubregions::Fife => 0x0F,
            ScotlandSubregions::Glasgow => 0x10,
            ScotlandSubregions::Highland => 0x11,
            ScotlandSubregions::Inverclyde => 0x12,
            ScotlandSubregions::Midlothian => 0x13,
            ScotlandSubregions::Moray => 0x14,
            ScotlandSubregions::NaHEileananSiar => 0x15,
            ScotlandSubregions::NorthAyrshire => 0x16,
            ScotlandSubregions::NorthLanarkshire => 0x17,
            ScotlandSubregions::OrkneyIslands => 0x18,
            ScotlandSubregions::PerthAndKinross => 0x19,
            ScotlandSubregions::Renfrewshire => 0x1A,
            ScotlandSubregions::ScottishBorders => 0x1B,
            ScotlandSubregions::ShetlandIslands => 0x1C,
            ScotlandSubregions::SouthAyrshire => 0x1D,
            ScotlandSubregions::SouthLanarkshire => 0x1E,
            ScotlandSubregions::Stirling => 0x1F,
            ScotlandSubregions::WestDunbartonshire => 0x20,
            ScotlandSubregions::WestLothian => 0x21,
        }
    }
}

impl Display for ScotlandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scotland => write!(f, "Scotland"),
            Self::Edinburgh => write!(f, "Edinburgh"),
            Self::Aberdeen => write!(f, "Aberdeen"),
            Self::Aberdeenshire => write!(f, "Aberdeenshire"),
            Self::Angus => write!(f, "Angus"),
            Self::ArgyllAndBute => write!(f, "Argyll and Bute"),
            Self::Clackmannanshire => write!(f, "Clackmannanshire"),
            Self::DumfriesAndGalloway => write!(f, "Dumfries and Galloway"),
            Self::Dundee => write!(f, "Dundee"),
            Self::EastAyrshire => write!(f, "East Ayrshire"),
            Self::EastDunbartonshire => write!(f, "East Dunbartonshire"),
            Self::EastLothian => write!(f, "East Lothian"),
            Self::EastRenfrewshire => write!(f, "East Renfrewshire"),
            Self::Falkirk => write!(f, "Falkirk"),
            Self::Fife => write!(f, "Fife"),
            Self::Glasgow => write!(f, "Glasgow"),
            Self::Highland => write!(f, "Highland"),
            Self::Inverclyde => write!(f, "Inverclyde"),
            Self::Midlothian => write!(f, "Midlothian"),
            Self::Moray => write!(f, "Moray"),
            Self::NaHEileananSiar => write!(f, "Na h-Eileanan Siar"),
            Self::NorthAyrshire => write!(f, "North Ayrshire"),
            Self::NorthLanarkshire => write!(f, "North Lanarkshire"),
            Self::OrkneyIslands => write!(f, "Orkney Islands"),
            Self::PerthAndKinross => write!(f, "Perth and Kinross"),
            Self::Renfrewshire => write!(f, "Renfrewshire"),
            Self::ScottishBorders => write!(f, "Scottish Borders"),
            Self::ShetlandIslands => write!(f, "Shetland Islands"),
            Self::SouthAyrshire => write!(f, "South Ayrshire"),
            Self::SouthLanarkshire => write!(f, "South Lanarkshire"),
            Self::Stirling => write!(f, "Stirling"),
            Self::WestDunbartonshire => write!(f, "West Dunbartonshire"),
            Self::WestLothian => write!(f, "West Lothian"),
        }
    }
}

// Senegal Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SenegalSubregions {
    Senegal,
    Dakar,
    Diourbel,
    Fatick,
    Kaffrine,
    Kaolack,
    Kedougou,
    Kolda,
    Louga,
    Matam,
    SaintLouis,
    Sedhiou,
    Tambacounda,
    Thies,
    Ziguinchor,
}

impl TryFrom<u8> for SenegalSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Senegal),
            0x02 => Ok(Self::Dakar),
            0x03 => Ok(Self::Diourbel),
            0x04 => Ok(Self::Fatick),
            0x05 => Ok(Self::Kaffrine),
            0x06 => Ok(Self::Kaolack),
            0x07 => Ok(Self::Kedougou),
            0x08 => Ok(Self::Kolda),
            0x09 => Ok(Self::Louga),
            0x0A => Ok(Self::Matam),
            0x0B => Ok(Self::SaintLouis),
            0x0C => Ok(Self::Sedhiou),
            0x0D => Ok(Self::Tambacounda),
            0x0E => Ok(Self::Thies),
            0x0F => Ok(Self::Ziguinchor),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SenegalSubregions> for u8 {
    fn from(value: SenegalSubregions) -> u8 {
        match value {
            SenegalSubregions::Senegal => 0x01,
            SenegalSubregions::Dakar => 0x02,
            SenegalSubregions::Diourbel => 0x03,
            SenegalSubregions::Fatick => 0x04,
            SenegalSubregions::Kaffrine => 0x05,
            SenegalSubregions::Kaolack => 0x06,
            SenegalSubregions::Kedougou => 0x07,
            SenegalSubregions::Kolda => 0x08,
            SenegalSubregions::Louga => 0x09,
            SenegalSubregions::Matam => 0x0A,
            SenegalSubregions::SaintLouis => 0x0B,
            SenegalSubregions::Sedhiou => 0x0C,
            SenegalSubregions::Tambacounda => 0x0D,
            SenegalSubregions::Thies => 0x0E,
            SenegalSubregions::Ziguinchor => 0x0F,
        }
    }
}

impl Display for SenegalSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Senegal => write!(f, "Senegal"),
            Self::Dakar => write!(f, "Dakar"),
            Self::Diourbel => write!(f, "Diourbel"),
            Self::Fatick => write!(f, "Fatick"),
            Self::Kaffrine => write!(f, "Kaffrine"),
            Self::Kaolack => write!(f, "Kaolack"),
            Self::Kedougou => write!(f, "Kédougou"),
            Self::Kolda => write!(f, "Kolda"),
            Self::Louga => write!(f, "Louga"),
            Self::Matam => write!(f, "Matam"),
            Self::SaintLouis => write!(f, "Saint-Louis"),
            Self::Sedhiou => write!(f, "Sédhiou"),
            Self::Tambacounda => write!(f, "Tambacounda"),
            Self::Thies => write!(f, "Thiès"),
            Self::Ziguinchor => write!(f, "Ziguinchor"),
        }
    }
}

// Serbia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SerbiaSubregions {
    Serbia,
    CentralSerbia,
    Vojvodina,
    KosovoAndMetohija,
}

impl TryFrom<u8> for SerbiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Serbia),
            0x02 => Ok(Self::CentralSerbia),
            0x03 => Ok(Self::Vojvodina),
            0x04 => Ok(Self::KosovoAndMetohija),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SerbiaSubregions> for u8 {
    fn from(value: SerbiaSubregions) -> u8 {
        match value {
            SerbiaSubregions::Serbia => 0x01,
            SerbiaSubregions::CentralSerbia => 0x02,
            SerbiaSubregions::Vojvodina => 0x03,
            SerbiaSubregions::KosovoAndMetohija => 0x04,
        }
    }
}

impl Display for SerbiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serbia => write!(f, "Serbia"),
            Self::CentralSerbia => write!(f, "Central Serbia"),
            Self::Vojvodina => write!(f, "Vojvodina"),
            Self::KosovoAndMetohija => write!(f, "Kosovo & Metohija"),
        }
    }
}

// Seychelles Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeychellesSubregions {
    Seychelles,
}

impl TryFrom<u8> for SeychellesSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Seychelles),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SeychellesSubregions> for u8 {
    fn from(value: SeychellesSubregions) -> u8 {
        match value {
            SeychellesSubregions::Seychelles => 0x01,
        }
    }
}

impl Display for SeychellesSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seychelles => write!(f, "Seychelles"),
        }
    }
}

// Sierra Leone Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SierraLeoneSubregions {
    SierraLeone,
    WesternArea,
    EasternProvince,
    NorthernProvince,
    NorthWestProvince,
    SouthernProvince,
}

impl TryFrom<u8> for SierraLeoneSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SierraLeone),
            0x02 => Ok(Self::WesternArea),
            0x03 => Ok(Self::EasternProvince),
            0x04 => Ok(Self::NorthernProvince),
            0x05 => Ok(Self::NorthWestProvince),
            0x06 => Ok(Self::SouthernProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SierraLeoneSubregions> for u8 {
    fn from(value: SierraLeoneSubregions) -> u8 {
        match value {
            SierraLeoneSubregions::SierraLeone => 0x01,
            SierraLeoneSubregions::WesternArea => 0x02,
            SierraLeoneSubregions::EasternProvince => 0x03,
            SierraLeoneSubregions::NorthernProvince => 0x04,
            SierraLeoneSubregions::NorthWestProvince => 0x05,
            SierraLeoneSubregions::SouthernProvince => 0x06,
        }
    }
}

impl Display for SierraLeoneSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SierraLeone => write!(f, "Sierra Leone"),
            Self::WesternArea => write!(f, "Western Area"),
            Self::EasternProvince => write!(f, "Eastern Province"),
            Self::NorthernProvince => write!(f, "Northern Province"),
            Self::NorthWestProvince => write!(f, "North West Province"),
            Self::SouthernProvince => write!(f, "Southern Province"),
        }
    }
}

// Singapore Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SingaporeSubregions {
    Singapore,
}

impl TryFrom<u8> for SingaporeSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Singapore),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SingaporeSubregions> for u8 {
    fn from(value: SingaporeSubregions) -> u8 {
        match value {
            SingaporeSubregions::Singapore => 0x01,
        }
    }
}

impl Display for SingaporeSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Singapore => write!(f, "Singapore"),
        }
    }
}

// Sint Maarten Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SintMaartenSubregions {
    SintMaarten,
}

impl TryFrom<u8> for SintMaartenSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SintMaarten),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SintMaartenSubregions> for u8 {
    fn from(value: SintMaartenSubregions) -> u8 {
        match value {
            SintMaartenSubregions::SintMaarten => 0x01,
        }
    }
}

impl Display for SintMaartenSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SintMaarten => write!(f, "Sint Maarten"),
        }
    }
}

// Slovakia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlovakiaSubregions {
    Slovakia,
    Bratislava,
    BanskaBystrica,
    Košice,
    Nitra,
    Prešov,
    Trencin,
    Trnava,
    Žilina,
}

impl TryFrom<u8> for SlovakiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Slovakia),
            0x02 => Ok(Self::Bratislava),
            0x03 => Ok(Self::BanskaBystrica),
            0x04 => Ok(Self::Košice),
            0x05 => Ok(Self::Nitra),
            0x06 => Ok(Self::Prešov),
            0x07 => Ok(Self::Trencin),
            0x08 => Ok(Self::Trnava),
            0x09 => Ok(Self::Žilina),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SlovakiaSubregions> for u8 {
    fn from(value: SlovakiaSubregions) -> u8 {
        match value {
            SlovakiaSubregions::Slovakia => 0x01,
            SlovakiaSubregions::Bratislava => 0x02,
            SlovakiaSubregions::BanskaBystrica => 0x03,
            SlovakiaSubregions::Košice => 0x04,
            SlovakiaSubregions::Nitra => 0x05,
            SlovakiaSubregions::Prešov => 0x06,
            SlovakiaSubregions::Trencin => 0x07,
            SlovakiaSubregions::Trnava => 0x08,
            SlovakiaSubregions::Žilina => 0x09,
        }
    }
}

impl Display for SlovakiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slovakia => write!(f, "Slovakia"),
            Self::Bratislava => write!(f, "Bratislava"),
            Self::BanskaBystrica => write!(f, "Banská Bystrica"),
            Self::Košice => write!(f, "Košice"),
            Self::Nitra => write!(f, "Nitra"),
            Self::Prešov => write!(f, "Prešov"),
            Self::Trencin => write!(f, "Trencín"),
            Self::Trnava => write!(f, "Trnava"),
            Self::Žilina => write!(f, "Žilina"),
        }
    }
}

// Slovenia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SloveniaSubregions {
    Slovenia,
    Carniola,
    LowerStyria,
    Prekmurje,
    SlovenianLittoral,
    Carinthia,
}

impl TryFrom<u8> for SloveniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Slovenia),
            0x02 => Ok(Self::Carniola),
            0x03 => Ok(Self::LowerStyria),
            0x04 => Ok(Self::Prekmurje),
            0x05 => Ok(Self::SlovenianLittoral),
            0x06 => Ok(Self::Carinthia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SloveniaSubregions> for u8 {
    fn from(value: SloveniaSubregions) -> u8 {
        match value {
            SloveniaSubregions::Slovenia => 0x01,
            SloveniaSubregions::Carniola => 0x02,
            SloveniaSubregions::LowerStyria => 0x03,
            SloveniaSubregions::Prekmurje => 0x04,
            SloveniaSubregions::SlovenianLittoral => 0x05,
            SloveniaSubregions::Carinthia => 0x06,
        }
    }
}

impl Display for SloveniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slovenia => write!(f, "Slovenia"),
            Self::Carniola => write!(f, "Carniola"),
            Self::LowerStyria => write!(f, "Lower Styria"),
            Self::Prekmurje => write!(f, "Prekmurje"),
            Self::SlovenianLittoral => write!(f, "Slovenian Littoral"),
            Self::Carinthia => write!(f, "Carinthia"),
        }
    }
}

// Solomon Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SolomonIslandsSubregions {
    SolomonIslands,
    GuadalcanalProvince,
    CentralProvince,
    ChoiseulProvince,
    IsabelProvince,
    MakiraUlawaProvince,
    MalaitaProvince,
    RennellAndBellonaProvince,
    TemotuProvince,
    WesternProvince,
}

impl TryFrom<u8> for SolomonIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SolomonIslands),
            0x02 => Ok(Self::GuadalcanalProvince),
            0x03 => Ok(Self::CentralProvince),
            0x04 => Ok(Self::ChoiseulProvince),
            0x05 => Ok(Self::IsabelProvince),
            0x06 => Ok(Self::MakiraUlawaProvince),
            0x07 => Ok(Self::MalaitaProvince),
            0x08 => Ok(Self::RennellAndBellonaProvince),
            0x09 => Ok(Self::TemotuProvince),
            0x0A => Ok(Self::WesternProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SolomonIslandsSubregions> for u8 {
    fn from(value: SolomonIslandsSubregions) -> u8 {
        match value {
            SolomonIslandsSubregions::SolomonIslands => 0x01,
            SolomonIslandsSubregions::GuadalcanalProvince => 0x02,
            SolomonIslandsSubregions::CentralProvince => 0x03,
            SolomonIslandsSubregions::ChoiseulProvince => 0x04,
            SolomonIslandsSubregions::IsabelProvince => 0x05,
            SolomonIslandsSubregions::MakiraUlawaProvince => 0x06,
            SolomonIslandsSubregions::MalaitaProvince => 0x07,
            SolomonIslandsSubregions::RennellAndBellonaProvince => 0x08,
            SolomonIslandsSubregions::TemotuProvince => 0x09,
            SolomonIslandsSubregions::WesternProvince => 0x0A,
        }
    }
}

impl Display for SolomonIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SolomonIslands => write!(f, "Solomon Islands"),
            Self::GuadalcanalProvince => write!(f, "Guadalcanal Province"),
            Self::CentralProvince => write!(f, "Central Province"),
            Self::ChoiseulProvince => write!(f, "Choiseul Province"),
            Self::IsabelProvince => write!(f, "Isabel Province"),
            Self::MakiraUlawaProvince => write!(f, "Makira-Ulawa Province"),
            Self::MalaitaProvince => write!(f, "Malaita Province"),
            Self::RennellAndBellonaProvince => write!(f, "Rennell and Bellona Province"),
            Self::TemotuProvince => write!(f, "Temotu Province"),
            Self::WesternProvince => write!(f, "Western Province"),
        }
    }
}

// Somalia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SomaliaSubregions {
    Somalia,
    BanaadirRegion,
    AwdalRegion,
    BakoolRegion,
    BariRegion,
    BayRegion,
    GalguduudRegion,
    GedoRegion,
    HiranRegion,
    LowerJubaRegion,
    LowerShebelleRegion,
    MiddleJubaRegion,
    MiddleShebelleRegion,
    MudugRegion,
    NugalRegion,
    SanaagRegion,
    SoolRegion,
    TogdheerRegion,
    WoqooyiGalbeedRegion,
}

impl TryFrom<u8> for SomaliaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Somalia),
            0x02 => Ok(Self::BanaadirRegion),
            0x03 => Ok(Self::AwdalRegion),
            0x04 => Ok(Self::BakoolRegion),
            0x05 => Ok(Self::BariRegion),
            0x06 => Ok(Self::BayRegion),
            0x07 => Ok(Self::GalguduudRegion),
            0x08 => Ok(Self::GedoRegion),
            0x09 => Ok(Self::HiranRegion),
            0x0A => Ok(Self::LowerJubaRegion),
            0x0B => Ok(Self::LowerShebelleRegion),
            0x0C => Ok(Self::MiddleJubaRegion),
            0x0D => Ok(Self::MiddleShebelleRegion),
            0x0E => Ok(Self::MudugRegion),
            0x0F => Ok(Self::NugalRegion),
            0x10 => Ok(Self::SanaagRegion),
            0x11 => Ok(Self::SoolRegion),
            0x12 => Ok(Self::TogdheerRegion),
            0x13 => Ok(Self::WoqooyiGalbeedRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SomaliaSubregions> for u8 {
    fn from(value: SomaliaSubregions) -> u8 {
        match value {
            SomaliaSubregions::Somalia => 0x01,
            SomaliaSubregions::BanaadirRegion => 0x02,
            SomaliaSubregions::AwdalRegion => 0x03,
            SomaliaSubregions::BakoolRegion => 0x04,
            SomaliaSubregions::BariRegion => 0x05,
            SomaliaSubregions::BayRegion => 0x06,
            SomaliaSubregions::GalguduudRegion => 0x07,
            SomaliaSubregions::GedoRegion => 0x08,
            SomaliaSubregions::HiranRegion => 0x09,
            SomaliaSubregions::LowerJubaRegion => 0x0A,
            SomaliaSubregions::LowerShebelleRegion => 0x0B,
            SomaliaSubregions::MiddleJubaRegion => 0x0C,
            SomaliaSubregions::MiddleShebelleRegion => 0x0D,
            SomaliaSubregions::MudugRegion => 0x0E,
            SomaliaSubregions::NugalRegion => 0x0F,
            SomaliaSubregions::SanaagRegion => 0x10,
            SomaliaSubregions::SoolRegion => 0x11,
            SomaliaSubregions::TogdheerRegion => 0x12,
            SomaliaSubregions::WoqooyiGalbeedRegion => 0x13,
        }
    }
}

impl Display for SomaliaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Somalia => write!(f, "Somalia"),
            Self::BanaadirRegion => write!(f, "Banaadir Region"),
            Self::AwdalRegion => write!(f, "Awdal Region"),
            Self::BakoolRegion => write!(f, "Bakool Region"),
            Self::BariRegion => write!(f, "Bari Region"),
            Self::BayRegion => write!(f, "Bay Region"),
            Self::GalguduudRegion => write!(f, "Galguduud Region"),
            Self::GedoRegion => write!(f, "Gedo Region"),
            Self::HiranRegion => write!(f, "Hiran Region"),
            Self::LowerJubaRegion => write!(f, "Lower Juba Region"),
            Self::LowerShebelleRegion => write!(f, "Lower Shebelle Region"),
            Self::MiddleJubaRegion => write!(f, "Middle Juba Region"),
            Self::MiddleShebelleRegion => write!(f, "Middle Shebelle Region"),
            Self::MudugRegion => write!(f, "Mudug Region"),
            Self::NugalRegion => write!(f, "Nugal Region"),
            Self::SanaagRegion => write!(f, "Sanaag Region"),
            Self::SoolRegion => write!(f, "Sool Region"),
            Self::TogdheerRegion => write!(f, "Togdheer Region"),
            Self::WoqooyiGalbeedRegion => write!(f, "Woqooyi Galbeed Region"),
        }
    }
}

// South Africa Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthAfricaSubregions {
    SouthAfrica,
    Gauteng,
    WesternCape,
    NorthernCape,
    EasternCape,
    KwazuluNatal,
    FreeState,
    NorthWest,
    Mpumalanga,
    Limpopo,
}

impl TryFrom<u8> for SouthAfricaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SouthAfrica),
            0x02 => Ok(Self::Gauteng),
            0x03 => Ok(Self::WesternCape),
            0x04 => Ok(Self::NorthernCape),
            0x05 => Ok(Self::EasternCape),
            0x06 => Ok(Self::KwazuluNatal),
            0x07 => Ok(Self::FreeState),
            0x08 => Ok(Self::NorthWest),
            0x09 => Ok(Self::Mpumalanga),
            0x0A => Ok(Self::Limpopo),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SouthAfricaSubregions> for u8 {
    fn from(value: SouthAfricaSubregions) -> u8 {
        match value {
            SouthAfricaSubregions::SouthAfrica => 0x01,
            SouthAfricaSubregions::Gauteng => 0x02,
            SouthAfricaSubregions::WesternCape => 0x03,
            SouthAfricaSubregions::NorthernCape => 0x04,
            SouthAfricaSubregions::EasternCape => 0x05,
            SouthAfricaSubregions::KwazuluNatal => 0x06,
            SouthAfricaSubregions::FreeState => 0x07,
            SouthAfricaSubregions::NorthWest => 0x08,
            SouthAfricaSubregions::Mpumalanga => 0x09,
            SouthAfricaSubregions::Limpopo => 0x0A,
        }
    }
}

impl Display for SouthAfricaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SouthAfrica => write!(f, "South Africa"),
            Self::Gauteng => write!(f, "Gauteng"),
            Self::WesternCape => write!(f, "Western Cape"),
            Self::NorthernCape => write!(f, "Northern Cape"),
            Self::EasternCape => write!(f, "Eastern Cape"),
            Self::KwazuluNatal => write!(f, "KwaZulu-Natal"),
            Self::FreeState => write!(f, "Free State"),
            Self::NorthWest => write!(f, "North West"),
            Self::Mpumalanga => write!(f, "Mpumalanga"),
            Self::Limpopo => write!(f, "Limpopo"),
        }
    }
}

// South Georgia and the South Sandwich Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthGeorgiaAndTheSouthSandwichIslandsSubregions {
    SouthGeorgiaAndTheSouthSandwichIslands,
}

impl TryFrom<u8> for SouthGeorgiaAndTheSouthSandwichIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SouthGeorgiaAndTheSouthSandwichIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SouthGeorgiaAndTheSouthSandwichIslandsSubregions> for u8 {
    fn from(value: SouthGeorgiaAndTheSouthSandwichIslandsSubregions) -> u8 {
        match value {
            SouthGeorgiaAndTheSouthSandwichIslandsSubregions::SouthGeorgiaAndTheSouthSandwichIslands => 0x01,
        }
    }
}

impl Display for SouthGeorgiaAndTheSouthSandwichIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SouthGeorgiaAndTheSouthSandwichIslands => {
                write!(f, "South Georgia and the South Sandwich Islands")
            }
        }
    }
}

// South Korea Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthKoreaSubregions {
    SouthKorea,
    SeoulTeukbyeolsi,
    BusanGwangyeoksi,
    DaeguGwangyeoksi,
    IncheonGwangyeoksi,
    GwangjuGwangyeoksi,
    DaejeonGwangyeoksi,
    UlsanGwangyeoksi,
    GyeonggiDo,
    GangwonDo,
    ChungcheongbukDo,
    ChungcheongnamDo,
    JeollabukDo,
    JeollanamDo,
    GyeongsangbukDo,
    GyeongsangnamDo,
    JejuTeukbyeoljachido,
}

impl TryFrom<u8> for SouthKoreaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SouthKorea),
            0x02 => Ok(Self::SeoulTeukbyeolsi),
            0x03 => Ok(Self::BusanGwangyeoksi),
            0x04 => Ok(Self::DaeguGwangyeoksi),
            0x05 => Ok(Self::IncheonGwangyeoksi),
            0x06 => Ok(Self::GwangjuGwangyeoksi),
            0x07 => Ok(Self::DaejeonGwangyeoksi),
            0x08 => Ok(Self::UlsanGwangyeoksi),
            0x09 => Ok(Self::GyeonggiDo),
            0x0A => Ok(Self::GangwonDo),
            0x0B => Ok(Self::ChungcheongbukDo),
            0x0C => Ok(Self::ChungcheongnamDo),
            0x0D => Ok(Self::JeollabukDo),
            0x0E => Ok(Self::JeollanamDo),
            0x0F => Ok(Self::GyeongsangbukDo),
            0x10 => Ok(Self::GyeongsangnamDo),
            0x11 => Ok(Self::JejuTeukbyeoljachido),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SouthKoreaSubregions> for u8 {
    fn from(value: SouthKoreaSubregions) -> u8 {
        match value {
            SouthKoreaSubregions::SouthKorea => 0x01,
            SouthKoreaSubregions::SeoulTeukbyeolsi => 0x02,
            SouthKoreaSubregions::BusanGwangyeoksi => 0x03,
            SouthKoreaSubregions::DaeguGwangyeoksi => 0x04,
            SouthKoreaSubregions::IncheonGwangyeoksi => 0x05,
            SouthKoreaSubregions::GwangjuGwangyeoksi => 0x06,
            SouthKoreaSubregions::DaejeonGwangyeoksi => 0x07,
            SouthKoreaSubregions::UlsanGwangyeoksi => 0x08,
            SouthKoreaSubregions::GyeonggiDo => 0x09,
            SouthKoreaSubregions::GangwonDo => 0x0A,
            SouthKoreaSubregions::ChungcheongbukDo => 0x0B,
            SouthKoreaSubregions::ChungcheongnamDo => 0x0C,
            SouthKoreaSubregions::JeollabukDo => 0x0D,
            SouthKoreaSubregions::JeollanamDo => 0x0E,
            SouthKoreaSubregions::GyeongsangbukDo => 0x0F,
            SouthKoreaSubregions::GyeongsangnamDo => 0x10,
            SouthKoreaSubregions::JejuTeukbyeoljachido => 0x11,
        }
    }
}

impl Display for SouthKoreaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SouthKorea => write!(f, "South Korea"),
            Self::SeoulTeukbyeolsi => write!(f, "Seoul-teukbyeolsi"),
            Self::BusanGwangyeoksi => write!(f, "Busan-gwangyeoksi"),
            Self::DaeguGwangyeoksi => write!(f, "Daegu-gwangyeoksi"),
            Self::IncheonGwangyeoksi => write!(f, "Incheon-gwangyeoksi"),
            Self::GwangjuGwangyeoksi => write!(f, "Gwangju-gwangyeoksi"),
            Self::DaejeonGwangyeoksi => write!(f, "Daejeon-gwangyeoksi"),
            Self::UlsanGwangyeoksi => write!(f, "Ulsan-gwangyeoksi"),
            Self::GyeonggiDo => write!(f, "Gyeonggi-do"),
            Self::GangwonDo => write!(f, "Gangwon-do"),
            Self::ChungcheongbukDo => write!(f, "Chungcheongbuk-do"),
            Self::ChungcheongnamDo => write!(f, "Chungcheongnam-do"),
            Self::JeollabukDo => write!(f, "Jeollabuk-do"),
            Self::JeollanamDo => write!(f, "Jeollanam-do"),
            Self::GyeongsangbukDo => write!(f, "Gyeongsangbuk-do"),
            Self::GyeongsangnamDo => write!(f, "Gyeongsangnam-do"),
            Self::JejuTeukbyeoljachido => write!(f, "Jeju-teukbyeoljachido"),
        }
    }
}

// South Ossetia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthOssetiaSubregions {
    SouthOssetia,
    TskhinvaliDistrict,
    DzauDistrict,
    LeningorDistrict,
    ZnaurDistrict,
}

impl TryFrom<u8> for SouthOssetiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SouthOssetia),
            0x02 => Ok(Self::TskhinvaliDistrict),
            0x03 => Ok(Self::DzauDistrict),
            0x04 => Ok(Self::LeningorDistrict),
            0x05 => Ok(Self::ZnaurDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SouthOssetiaSubregions> for u8 {
    fn from(value: SouthOssetiaSubregions) -> u8 {
        match value {
            SouthOssetiaSubregions::SouthOssetia => 0x01,
            SouthOssetiaSubregions::TskhinvaliDistrict => 0x02,
            SouthOssetiaSubregions::DzauDistrict => 0x03,
            SouthOssetiaSubregions::LeningorDistrict => 0x04,
            SouthOssetiaSubregions::ZnaurDistrict => 0x05,
        }
    }
}

impl Display for SouthOssetiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SouthOssetia => write!(f, "South Ossetia"),
            Self::TskhinvaliDistrict => write!(f, "Tskhinvali District"),
            Self::DzauDistrict => write!(f, "Dzau District"),
            Self::LeningorDistrict => write!(f, "Leningor District"),
            Self::ZnaurDistrict => write!(f, "Znaur District"),
        }
    }
}

// South Sudan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SouthSudanSubregions {
    SouthSudan,
    Equatoria,
    BahrElGhazal,
    GreaterUpperNile,
}

impl TryFrom<u8> for SouthSudanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SouthSudan),
            0x02 => Ok(Self::Equatoria),
            0x03 => Ok(Self::BahrElGhazal),
            0x04 => Ok(Self::GreaterUpperNile),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SouthSudanSubregions> for u8 {
    fn from(value: SouthSudanSubregions) -> u8 {
        match value {
            SouthSudanSubregions::SouthSudan => 0x01,
            SouthSudanSubregions::Equatoria => 0x02,
            SouthSudanSubregions::BahrElGhazal => 0x03,
            SouthSudanSubregions::GreaterUpperNile => 0x04,
        }
    }
}

impl Display for SouthSudanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SouthSudan => write!(f, "South Sudan"),
            Self::Equatoria => write!(f, "Equatoria"),
            Self::BahrElGhazal => write!(f, "Bahr el Ghazal"),
            Self::GreaterUpperNile => write!(f, "Greater Upper Nile"),
        }
    }
}

// Spain Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpainSubregions {
    Spain,
    Madrid,
    Andalusia,
    Aragon,
    PrincipalityOfAsturias,
    BalearicIslands,
    CanaryIslands,
    Cantabria,
    CastileLaMancha,
    CastillaYLeon,
    Catalonia,
    Valencia,
    Extremadura,
    Galicia,
    Murcia,
    Navarre,
    BasqueCountry,
    LaRioja,
    Ceuta,
    Melilla,
}

impl TryFrom<u8> for SpainSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Spain),
            0x02 => Ok(Self::Madrid),
            0x03 => Ok(Self::Andalusia),
            0x04 => Ok(Self::Aragon),
            0x05 => Ok(Self::PrincipalityOfAsturias),
            0x06 => Ok(Self::BalearicIslands),
            0x07 => Ok(Self::CanaryIslands),
            0x08 => Ok(Self::Cantabria),
            0x09 => Ok(Self::CastileLaMancha),
            0x0A => Ok(Self::CastillaYLeon),
            0x0B => Ok(Self::Catalonia),
            0x0C => Ok(Self::Valencia),
            0x0D => Ok(Self::Extremadura),
            0x0E => Ok(Self::Galicia),
            0x0F => Ok(Self::Murcia),
            0x10 => Ok(Self::Navarre),
            0x11 => Ok(Self::BasqueCountry),
            0x12 => Ok(Self::LaRioja),
            0x13 => Ok(Self::Ceuta),
            0x14 => Ok(Self::Melilla),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SpainSubregions> for u8 {
    fn from(value: SpainSubregions) -> u8 {
        match value {
            SpainSubregions::Spain => 0x01,
            SpainSubregions::Madrid => 0x02,
            SpainSubregions::Andalusia => 0x03,
            SpainSubregions::Aragon => 0x04,
            SpainSubregions::PrincipalityOfAsturias => 0x05,
            SpainSubregions::BalearicIslands => 0x06,
            SpainSubregions::CanaryIslands => 0x07,
            SpainSubregions::Cantabria => 0x08,
            SpainSubregions::CastileLaMancha => 0x09,
            SpainSubregions::CastillaYLeon => 0x0A,
            SpainSubregions::Catalonia => 0x0B,
            SpainSubregions::Valencia => 0x0C,
            SpainSubregions::Extremadura => 0x0D,
            SpainSubregions::Galicia => 0x0E,
            SpainSubregions::Murcia => 0x0F,
            SpainSubregions::Navarre => 0x10,
            SpainSubregions::BasqueCountry => 0x11,
            SpainSubregions::LaRioja => 0x12,
            SpainSubregions::Ceuta => 0x13,
            SpainSubregions::Melilla => 0x14,
        }
    }
}

impl Display for SpainSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spain => write!(f, "Spain"),
            Self::Madrid => write!(f, "Madrid"),
            Self::Andalusia => write!(f, "Andalusia"),
            Self::Aragon => write!(f, "Aragon"),
            Self::PrincipalityOfAsturias => write!(f, "Principality of Asturias"),
            Self::BalearicIslands => write!(f, "Balearic Islands"),
            Self::CanaryIslands => write!(f, "Canary Islands"),
            Self::Cantabria => write!(f, "Cantabria"),
            Self::CastileLaMancha => write!(f, "Castile-La Mancha"),
            Self::CastillaYLeon => write!(f, "Castilla y León"),
            Self::Catalonia => write!(f, "Catalonia"),
            Self::Valencia => write!(f, "Valencia"),
            Self::Extremadura => write!(f, "Extremadura"),
            Self::Galicia => write!(f, "Galicia"),
            Self::Murcia => write!(f, "Murcia"),
            Self::Navarre => write!(f, "Navarre"),
            Self::BasqueCountry => write!(f, "Basque Country"),
            Self::LaRioja => write!(f, "La Rioja"),
            Self::Ceuta => write!(f, "Ceuta"),
            Self::Melilla => write!(f, "Melilla"),
        }
    }
}

// Sri Lanka Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SriLankaSubregions {
    SriLanka,
    WesternProvince,
    CentralProvince,
    EasternProvince,
    NorthCentralProvince,
    NorthernProvince,
    NorthWesternProvince,
    SabaragamuwaProvince,
    SouthernProvince,
    UvaProvince,
}

impl TryFrom<u8> for SriLankaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SriLanka),
            0x02 => Ok(Self::WesternProvince),
            0x03 => Ok(Self::CentralProvince),
            0x04 => Ok(Self::EasternProvince),
            0x05 => Ok(Self::NorthCentralProvince),
            0x06 => Ok(Self::NorthernProvince),
            0x07 => Ok(Self::NorthWesternProvince),
            0x08 => Ok(Self::SabaragamuwaProvince),
            0x09 => Ok(Self::SouthernProvince),
            0x0A => Ok(Self::UvaProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SriLankaSubregions> for u8 {
    fn from(value: SriLankaSubregions) -> u8 {
        match value {
            SriLankaSubregions::SriLanka => 0x01,
            SriLankaSubregions::WesternProvince => 0x02,
            SriLankaSubregions::CentralProvince => 0x03,
            SriLankaSubregions::EasternProvince => 0x04,
            SriLankaSubregions::NorthCentralProvince => 0x05,
            SriLankaSubregions::NorthernProvince => 0x06,
            SriLankaSubregions::NorthWesternProvince => 0x07,
            SriLankaSubregions::SabaragamuwaProvince => 0x08,
            SriLankaSubregions::SouthernProvince => 0x09,
            SriLankaSubregions::UvaProvince => 0x0A,
        }
    }
}

impl Display for SriLankaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SriLanka => write!(f, "Sri Lanka"),
            Self::WesternProvince => write!(f, "Western Province"),
            Self::CentralProvince => write!(f, "Central Province"),
            Self::EasternProvince => write!(f, "Eastern Province"),
            Self::NorthCentralProvince => write!(f, "North Central Province"),
            Self::NorthernProvince => write!(f, "Northern Province"),
            Self::NorthWesternProvince => write!(f, "North Western Province"),
            Self::SabaragamuwaProvince => write!(f, "Sabaragamuwa Province"),
            Self::SouthernProvince => write!(f, "Southern Province"),
            Self::UvaProvince => write!(f, "Uva Province"),
        }
    }
}

// St. Kitts and Nevis Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StKittsAndNevisSubregions {
    StKittsAndNevis,
    SaintGeorgeBasseterre,
    ChristChurchNicholaTown,
    SaintAnneSandyPoint,
    SaintGeorgeGingerland,
    SaintJamesWindward,
    SaintJohnCapesterre,
    SaintJohnFigtree,
    SaintMaryCayon,
    SaintPaulCapesterre,
    SaintPaulCharlestown,
    SaintPeterBasseterre,
    SaintThomasLowland,
    SaintThomasMiddleIsland,
    TrinityPalmettoPoint,
}

impl TryFrom<u8> for StKittsAndNevisSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::StKittsAndNevis),
            0x02 => Ok(Self::SaintGeorgeBasseterre),
            0x03 => Ok(Self::ChristChurchNicholaTown),
            0x04 => Ok(Self::SaintAnneSandyPoint),
            0x05 => Ok(Self::SaintGeorgeGingerland),
            0x06 => Ok(Self::SaintJamesWindward),
            0x07 => Ok(Self::SaintJohnCapesterre),
            0x08 => Ok(Self::SaintJohnFigtree),
            0x09 => Ok(Self::SaintMaryCayon),
            0x0A => Ok(Self::SaintPaulCapesterre),
            0x0B => Ok(Self::SaintPaulCharlestown),
            0x0C => Ok(Self::SaintPeterBasseterre),
            0x0D => Ok(Self::SaintThomasLowland),
            0x0E => Ok(Self::SaintThomasMiddleIsland),
            0x0F => Ok(Self::TrinityPalmettoPoint),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<StKittsAndNevisSubregions> for u8 {
    fn from(value: StKittsAndNevisSubregions) -> u8 {
        match value {
            StKittsAndNevisSubregions::StKittsAndNevis => 0x01,
            StKittsAndNevisSubregions::SaintGeorgeBasseterre => 0x02,
            StKittsAndNevisSubregions::ChristChurchNicholaTown => 0x03,
            StKittsAndNevisSubregions::SaintAnneSandyPoint => 0x04,
            StKittsAndNevisSubregions::SaintGeorgeGingerland => 0x05,
            StKittsAndNevisSubregions::SaintJamesWindward => 0x06,
            StKittsAndNevisSubregions::SaintJohnCapesterre => 0x07,
            StKittsAndNevisSubregions::SaintJohnFigtree => 0x08,
            StKittsAndNevisSubregions::SaintMaryCayon => 0x09,
            StKittsAndNevisSubregions::SaintPaulCapesterre => 0x0A,
            StKittsAndNevisSubregions::SaintPaulCharlestown => 0x0B,
            StKittsAndNevisSubregions::SaintPeterBasseterre => 0x0C,
            StKittsAndNevisSubregions::SaintThomasLowland => 0x0D,
            StKittsAndNevisSubregions::SaintThomasMiddleIsland => 0x0E,
            StKittsAndNevisSubregions::TrinityPalmettoPoint => 0x0F,
        }
    }
}

impl Display for StKittsAndNevisSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StKittsAndNevis => write!(f, "St. Kitts and Nevis"),
            Self::SaintGeorgeBasseterre => write!(f, "Saint George Basseterre"),
            Self::ChristChurchNicholaTown => write!(f, "Christ Church Nichola Town"),
            Self::SaintAnneSandyPoint => write!(f, "Saint Anne Sandy Point"),
            Self::SaintGeorgeGingerland => write!(f, "Saint George Gingerland"),
            Self::SaintJamesWindward => write!(f, "Saint James Windward"),
            Self::SaintJohnCapesterre => write!(f, "Saint John Capesterre"),
            Self::SaintJohnFigtree => write!(f, "Saint John Figtree"),
            Self::SaintMaryCayon => write!(f, "Saint Mary Cayon"),
            Self::SaintPaulCapesterre => write!(f, "Saint Paul Capesterre"),
            Self::SaintPaulCharlestown => write!(f, "Saint Paul Charlestown"),
            Self::SaintPeterBasseterre => write!(f, "Saint Peter Basseterre"),
            Self::SaintThomasLowland => write!(f, "Saint Thomas Lowland"),
            Self::SaintThomasMiddleIsland => write!(f, "Saint Thomas Middle Island"),
            Self::TrinityPalmettoPoint => write!(f, "Trinity Palmetto Point"),
        }
    }
}

// St. Lucia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StLuciaSubregions {
    StLucia,
}

impl TryFrom<u8> for StLuciaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::StLucia),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<StLuciaSubregions> for u8 {
    fn from(value: StLuciaSubregions) -> u8 {
        match value {
            StLuciaSubregions::StLucia => 0x01,
        }
    }
}

impl Display for StLuciaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StLucia => write!(f, "St. Lucia"),
        }
    }
}

// St. Vincent and the Grenadines Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StVincentAndTheGrenadinesSubregions {
    StVincentAndTheGrenadines,
    SaintGeorgeParish,
    CharlotteParish,
    GrenadinesParish,
    SaintAndrewParish,
    SaintDavidParish,
    SaintPatrickParish,
}

impl TryFrom<u8> for StVincentAndTheGrenadinesSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::StVincentAndTheGrenadines),
            0x02 => Ok(Self::SaintGeorgeParish),
            0x03 => Ok(Self::CharlotteParish),
            0x04 => Ok(Self::GrenadinesParish),
            0x05 => Ok(Self::SaintAndrewParish),
            0x06 => Ok(Self::SaintDavidParish),
            0x07 => Ok(Self::SaintPatrickParish),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<StVincentAndTheGrenadinesSubregions> for u8 {
    fn from(value: StVincentAndTheGrenadinesSubregions) -> u8 {
        match value {
            StVincentAndTheGrenadinesSubregions::StVincentAndTheGrenadines => 0x01,
            StVincentAndTheGrenadinesSubregions::SaintGeorgeParish => 0x02,
            StVincentAndTheGrenadinesSubregions::CharlotteParish => 0x03,
            StVincentAndTheGrenadinesSubregions::GrenadinesParish => 0x04,
            StVincentAndTheGrenadinesSubregions::SaintAndrewParish => 0x05,
            StVincentAndTheGrenadinesSubregions::SaintDavidParish => 0x06,
            StVincentAndTheGrenadinesSubregions::SaintPatrickParish => 0x07,
        }
    }
}

impl Display for StVincentAndTheGrenadinesSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StVincentAndTheGrenadines => write!(f, "St. Vincent and the Grenadines"),
            Self::SaintGeorgeParish => write!(f, "Saint George Parish"),
            Self::CharlotteParish => write!(f, "Charlotte Parish"),
            Self::GrenadinesParish => write!(f, "Grenadines Parish"),
            Self::SaintAndrewParish => write!(f, "Saint Andrew Parish"),
            Self::SaintDavidParish => write!(f, "Saint David Parish"),
            Self::SaintPatrickParish => write!(f, "Saint Patrick Parish"),
        }
    }
}

// Sudan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SudanSubregions {
    Sudan,
    Khartoum,
    AlQadarif,
    BlueNile,
    CentralDarfur,
    EastDarfur,
    Gezira,
    Kassala,
    NorthDarfur,
    NorthKordofan,
    Northern,
    RedSea,
    RiverNile,
    Sennar,
    SouthDarfur,
    SouthKordofan,
    WestDarfur,
    WestKordofan,
    WhiteNile,
}

impl TryFrom<u8> for SudanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Sudan),
            0x02 => Ok(Self::Khartoum),
            0x03 => Ok(Self::AlQadarif),
            0x04 => Ok(Self::BlueNile),
            0x05 => Ok(Self::CentralDarfur),
            0x06 => Ok(Self::EastDarfur),
            0x07 => Ok(Self::Gezira),
            0x08 => Ok(Self::Kassala),
            0x09 => Ok(Self::NorthDarfur),
            0x0A => Ok(Self::NorthKordofan),
            0x0B => Ok(Self::Northern),
            0x0C => Ok(Self::RedSea),
            0x0D => Ok(Self::RiverNile),
            0x0E => Ok(Self::Sennar),
            0x0F => Ok(Self::SouthDarfur),
            0x10 => Ok(Self::SouthKordofan),
            0x11 => Ok(Self::WestDarfur),
            0x12 => Ok(Self::WestKordofan),
            0x13 => Ok(Self::WhiteNile),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SudanSubregions> for u8 {
    fn from(value: SudanSubregions) -> u8 {
        match value {
            SudanSubregions::Sudan => 0x01,
            SudanSubregions::Khartoum => 0x02,
            SudanSubregions::AlQadarif => 0x03,
            SudanSubregions::BlueNile => 0x04,
            SudanSubregions::CentralDarfur => 0x05,
            SudanSubregions::EastDarfur => 0x06,
            SudanSubregions::Gezira => 0x07,
            SudanSubregions::Kassala => 0x08,
            SudanSubregions::NorthDarfur => 0x09,
            SudanSubregions::NorthKordofan => 0x0A,
            SudanSubregions::Northern => 0x0B,
            SudanSubregions::RedSea => 0x0C,
            SudanSubregions::RiverNile => 0x0D,
            SudanSubregions::Sennar => 0x0E,
            SudanSubregions::SouthDarfur => 0x0F,
            SudanSubregions::SouthKordofan => 0x10,
            SudanSubregions::WestDarfur => 0x11,
            SudanSubregions::WestKordofan => 0x12,
            SudanSubregions::WhiteNile => 0x13,
        }
    }
}

impl Display for SudanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sudan => write!(f, "Sudan"),
            Self::Khartoum => write!(f, "Khartoum"),
            Self::AlQadarif => write!(f, "Al Qadarif"),
            Self::BlueNile => write!(f, "Blue Nile"),
            Self::CentralDarfur => write!(f, "Central Darfur"),
            Self::EastDarfur => write!(f, "East Darfur"),
            Self::Gezira => write!(f, "Gezira"),
            Self::Kassala => write!(f, "Kassala"),
            Self::NorthDarfur => write!(f, "North Darfur"),
            Self::NorthKordofan => write!(f, "North Kordofan"),
            Self::Northern => write!(f, "Northern"),
            Self::RedSea => write!(f, "Red Sea"),
            Self::RiverNile => write!(f, "River Nile"),
            Self::Sennar => write!(f, "Sennar"),
            Self::SouthDarfur => write!(f, "South Darfur"),
            Self::SouthKordofan => write!(f, "South Kordofan"),
            Self::WestDarfur => write!(f, "West Darfur"),
            Self::WestKordofan => write!(f, "West Kordofan"),
            Self::WhiteNile => write!(f, "White Nile"),
        }
    }
}

// Suriname Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurinameSubregions {
    Suriname,
    Paramaribo,
    Brokopondo,
    Commewijne,
    Coronie,
    Marowijne,
    Nickerie,
    Para,
    Saramacca,
    Sipaliwini,
    Wanica,
}

impl TryFrom<u8> for SurinameSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Suriname),
            0x02 => Ok(Self::Paramaribo),
            0x03 => Ok(Self::Brokopondo),
            0x04 => Ok(Self::Commewijne),
            0x05 => Ok(Self::Coronie),
            0x06 => Ok(Self::Marowijne),
            0x07 => Ok(Self::Nickerie),
            0x08 => Ok(Self::Para),
            0x09 => Ok(Self::Saramacca),
            0x0A => Ok(Self::Sipaliwini),
            0x0B => Ok(Self::Wanica),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SurinameSubregions> for u8 {
    fn from(value: SurinameSubregions) -> u8 {
        match value {
            SurinameSubregions::Suriname => 0x01,
            SurinameSubregions::Paramaribo => 0x02,
            SurinameSubregions::Brokopondo => 0x03,
            SurinameSubregions::Commewijne => 0x04,
            SurinameSubregions::Coronie => 0x05,
            SurinameSubregions::Marowijne => 0x06,
            SurinameSubregions::Nickerie => 0x07,
            SurinameSubregions::Para => 0x08,
            SurinameSubregions::Saramacca => 0x09,
            SurinameSubregions::Sipaliwini => 0x0A,
            SurinameSubregions::Wanica => 0x0B,
        }
    }
}

impl Display for SurinameSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Suriname => write!(f, "Suriname"),
            Self::Paramaribo => write!(f, "Paramaribo"),
            Self::Brokopondo => write!(f, "Brokopondo"),
            Self::Commewijne => write!(f, "Commewijne"),
            Self::Coronie => write!(f, "Coronie"),
            Self::Marowijne => write!(f, "Marowijne"),
            Self::Nickerie => write!(f, "Nickerie"),
            Self::Para => write!(f, "Para"),
            Self::Saramacca => write!(f, "Saramacca"),
            Self::Sipaliwini => write!(f, "Sipaliwini"),
            Self::Wanica => write!(f, "Wanica"),
        }
    }
}

// Sweden Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwedenSubregions {
    Sweden,
    StockholmCounty,
    SkåneCounty,
    VastraGotalandCounty,
    OstergotlandCounty,
    SodermanlandCounty,
    VarmlandCounty,
    UppsalaCounty,
    GavleborgCounty,
    VasterbottenCounty,
    NorrbottenCounty,
    GotlandIsland,
    JamtlandCounty,
    DalarnaCounty,
    BlekingeCounty,
    OrebroCounty,
    VasternorrlandCounty,
    JonkopingCounty,
    KronobergCounty,
    KalmarCounty,
    VastmanlandCounty,
    HallandCounty,
}

impl TryFrom<u8> for SwedenSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Sweden),
            0x02 => Ok(Self::StockholmCounty),
            0x03 => Ok(Self::SkåneCounty),
            0x04 => Ok(Self::VastraGotalandCounty),
            0x05 => Ok(Self::OstergotlandCounty),
            0x06 => Ok(Self::SodermanlandCounty),
            0x07 => Ok(Self::VarmlandCounty),
            0x08 => Ok(Self::UppsalaCounty),
            0x09 => Ok(Self::GavleborgCounty),
            0x0A => Ok(Self::VasterbottenCounty),
            0x0B => Ok(Self::NorrbottenCounty),
            0x0C => Ok(Self::GotlandIsland),
            0x0D => Ok(Self::JamtlandCounty),
            0x0E => Ok(Self::DalarnaCounty),
            0x0F => Ok(Self::BlekingeCounty),
            0x10 => Ok(Self::OrebroCounty),
            0x11 => Ok(Self::VasternorrlandCounty),
            0x12 => Ok(Self::JonkopingCounty),
            0x13 => Ok(Self::KronobergCounty),
            0x14 => Ok(Self::KalmarCounty),
            0x15 => Ok(Self::VastmanlandCounty),
            0x16 => Ok(Self::HallandCounty),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SwedenSubregions> for u8 {
    fn from(value: SwedenSubregions) -> u8 {
        match value {
            SwedenSubregions::Sweden => 0x01,
            SwedenSubregions::StockholmCounty => 0x02,
            SwedenSubregions::SkåneCounty => 0x03,
            SwedenSubregions::VastraGotalandCounty => 0x04,
            SwedenSubregions::OstergotlandCounty => 0x05,
            SwedenSubregions::SodermanlandCounty => 0x06,
            SwedenSubregions::VarmlandCounty => 0x07,
            SwedenSubregions::UppsalaCounty => 0x08,
            SwedenSubregions::GavleborgCounty => 0x09,
            SwedenSubregions::VasterbottenCounty => 0x0A,
            SwedenSubregions::NorrbottenCounty => 0x0B,
            SwedenSubregions::GotlandIsland => 0x0C,
            SwedenSubregions::JamtlandCounty => 0x0D,
            SwedenSubregions::DalarnaCounty => 0x0E,
            SwedenSubregions::BlekingeCounty => 0x0F,
            SwedenSubregions::OrebroCounty => 0x10,
            SwedenSubregions::VasternorrlandCounty => 0x11,
            SwedenSubregions::JonkopingCounty => 0x12,
            SwedenSubregions::KronobergCounty => 0x13,
            SwedenSubregions::KalmarCounty => 0x14,
            SwedenSubregions::VastmanlandCounty => 0x15,
            SwedenSubregions::HallandCounty => 0x16,
        }
    }
}

impl Display for SwedenSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sweden => write!(f, "Sweden"),
            Self::StockholmCounty => write!(f, "Stockholm County"),
            Self::SkåneCounty => write!(f, "Skåne County"),
            Self::VastraGotalandCounty => write!(f, "Västra Götaland County"),
            Self::OstergotlandCounty => write!(f, "Östergötland County"),
            Self::SodermanlandCounty => write!(f, "Södermanland County"),
            Self::VarmlandCounty => write!(f, "Värmland County"),
            Self::UppsalaCounty => write!(f, "Uppsala County"),
            Self::GavleborgCounty => write!(f, "Gävleborg County"),
            Self::VasterbottenCounty => write!(f, "Västerbotten County"),
            Self::NorrbottenCounty => write!(f, "Norrbotten County"),
            Self::GotlandIsland => write!(f, "Gotland Island"),
            Self::JamtlandCounty => write!(f, "Jämtland County"),
            Self::DalarnaCounty => write!(f, "Dalarna County"),
            Self::BlekingeCounty => write!(f, "Blekinge County"),
            Self::OrebroCounty => write!(f, "Örebro County"),
            Self::VasternorrlandCounty => write!(f, "Västernorrland County"),
            Self::JonkopingCounty => write!(f, "Jönköping County"),
            Self::KronobergCounty => write!(f, "Kronoberg County"),
            Self::KalmarCounty => write!(f, "Kalmar County"),
            Self::VastmanlandCounty => write!(f, "Västmanland County"),
            Self::HallandCounty => write!(f, "Halland County"),
        }
    }
}

// Switzerland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitzerlandSubregions {
    Switzerland,
    Bern,
    Appenzell,
    Aargau,
    BaselCity,
    Fribourg,
    Geneva,
    Glarus,
    Graubunden,
    Jura,
    Luzern,
    Neuchatel,
    Obwalden,
    StGallen,
    Schaffhausen,
    Schwyz,
    Solothurn,
    Thurgau,
    Ticino,
    Uri,
    Valais,
    Vaud,
    Zug,
    Zurich,
}

impl TryFrom<u8> for SwitzerlandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Switzerland),
            0x02 => Ok(Self::Bern),
            0x03 => Ok(Self::Appenzell),
            0x04 => Ok(Self::Aargau),
            0x05 => Ok(Self::BaselCity),
            0x06 => Ok(Self::Fribourg),
            0x07 => Ok(Self::Geneva),
            0x08 => Ok(Self::Glarus),
            0x09 => Ok(Self::Graubunden),
            0x0A => Ok(Self::Jura),
            0x0B => Ok(Self::Luzern),
            0x0C => Ok(Self::Neuchatel),
            0x0D => Ok(Self::Obwalden),
            0x0E => Ok(Self::StGallen),
            0x0F => Ok(Self::Schaffhausen),
            0x10 => Ok(Self::Schwyz),
            0x11 => Ok(Self::Solothurn),
            0x12 => Ok(Self::Thurgau),
            0x13 => Ok(Self::Ticino),
            0x14 => Ok(Self::Uri),
            0x15 => Ok(Self::Valais),
            0x16 => Ok(Self::Vaud),
            0x17 => Ok(Self::Zug),
            0x18 => Ok(Self::Zurich),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SwitzerlandSubregions> for u8 {
    fn from(value: SwitzerlandSubregions) -> u8 {
        match value {
            SwitzerlandSubregions::Switzerland => 0x01,
            SwitzerlandSubregions::Bern => 0x02,
            SwitzerlandSubregions::Appenzell => 0x03,
            SwitzerlandSubregions::Aargau => 0x04,
            SwitzerlandSubregions::BaselCity => 0x05,
            SwitzerlandSubregions::Fribourg => 0x06,
            SwitzerlandSubregions::Geneva => 0x07,
            SwitzerlandSubregions::Glarus => 0x08,
            SwitzerlandSubregions::Graubunden => 0x09,
            SwitzerlandSubregions::Jura => 0x0A,
            SwitzerlandSubregions::Luzern => 0x0B,
            SwitzerlandSubregions::Neuchatel => 0x0C,
            SwitzerlandSubregions::Obwalden => 0x0D,
            SwitzerlandSubregions::StGallen => 0x0E,
            SwitzerlandSubregions::Schaffhausen => 0x0F,
            SwitzerlandSubregions::Schwyz => 0x10,
            SwitzerlandSubregions::Solothurn => 0x11,
            SwitzerlandSubregions::Thurgau => 0x12,
            SwitzerlandSubregions::Ticino => 0x13,
            SwitzerlandSubregions::Uri => 0x14,
            SwitzerlandSubregions::Valais => 0x15,
            SwitzerlandSubregions::Vaud => 0x16,
            SwitzerlandSubregions::Zug => 0x17,
            SwitzerlandSubregions::Zurich => 0x18,
        }
    }
}

impl Display for SwitzerlandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Switzerland => write!(f, "Switzerland"),
            Self::Bern => write!(f, "Bern"),
            Self::Appenzell => write!(f, "Appenzell"),
            Self::Aargau => write!(f, "Aargau"),
            Self::BaselCity => write!(f, "Basel-City"),
            Self::Fribourg => write!(f, "Fribourg"),
            Self::Geneva => write!(f, "Geneva"),
            Self::Glarus => write!(f, "Glarus"),
            Self::Graubunden => write!(f, "Graubünden"),
            Self::Jura => write!(f, "Jura"),
            Self::Luzern => write!(f, "Luzern"),
            Self::Neuchatel => write!(f, "Neuchâtel"),
            Self::Obwalden => write!(f, "Obwalden"),
            Self::StGallen => write!(f, "St. Gallen"),
            Self::Schaffhausen => write!(f, "Schaffhausen"),
            Self::Schwyz => write!(f, "Schwyz"),
            Self::Solothurn => write!(f, "Solothurn"),
            Self::Thurgau => write!(f, "Thurgau"),
            Self::Ticino => write!(f, "Ticino"),
            Self::Uri => write!(f, "Uri"),
            Self::Valais => write!(f, "Valais"),
            Self::Vaud => write!(f, "Vaud"),
            Self::Zug => write!(f, "Zug"),
            Self::Zurich => write!(f, "Zürich"),
        }
    }
}

// Syria Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyriaSubregions {
    Syria,
    RifDimashq,
    AlHasakah,
    AlLadhiqiyah,
    AlQunaytirah,
    ArRaqqah,
    AsSuwayda,
    Dara,
    DayrAzZawr,
    Halab,
    Hamah,
    Hims,
    Idlib,
    Tartus,
}

impl TryFrom<u8> for SyriaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Syria),
            0x02 => Ok(Self::RifDimashq),
            0x03 => Ok(Self::AlHasakah),
            0x04 => Ok(Self::AlLadhiqiyah),
            0x05 => Ok(Self::AlQunaytirah),
            0x06 => Ok(Self::ArRaqqah),
            0x07 => Ok(Self::AsSuwayda),
            0x08 => Ok(Self::Dara),
            0x09 => Ok(Self::DayrAzZawr),
            0x0A => Ok(Self::RifDimashq),
            0x0B => Ok(Self::Halab),
            0x0C => Ok(Self::Hamah),
            0x0D => Ok(Self::Hims),
            0x0E => Ok(Self::Idlib),
            0x0F => Ok(Self::Tartus),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SyriaSubregions> for u8 {
    fn from(value: SyriaSubregions) -> u8 {
        match value {
            SyriaSubregions::Syria => 0x01,
            SyriaSubregions::RifDimashq => 0x02,
            SyriaSubregions::AlHasakah => 0x03,
            SyriaSubregions::AlLadhiqiyah => 0x04,
            SyriaSubregions::AlQunaytirah => 0x05,
            SyriaSubregions::ArRaqqah => 0x06,
            SyriaSubregions::AsSuwayda => 0x07,
            SyriaSubregions::Dara => 0x08,
            SyriaSubregions::DayrAzZawr => 0x09,
            SyriaSubregions::Halab => 0x0B,
            SyriaSubregions::Hamah => 0x0C,
            SyriaSubregions::Hims => 0x0D,
            SyriaSubregions::Idlib => 0x0E,
            SyriaSubregions::Tartus => 0x0F,
        }
    }
}

impl Display for SyriaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syria => write!(f, "Syria"),
            Self::RifDimashq => write!(f, "Rif Dimashq"),
            Self::AlHasakah => write!(f, "Al Hasakah"),
            Self::AlLadhiqiyah => write!(f, "Al Ladhiqiyah"),
            Self::AlQunaytirah => write!(f, "Al Qunaytirah"),
            Self::ArRaqqah => write!(f, "Ar Raqqah"),
            Self::AsSuwayda => write!(f, "As Suwayda'"),
            Self::Dara => write!(f, "Dar‘a"),
            Self::DayrAzZawr => write!(f, "Dayr az Zawr"),
            Self::Halab => write!(f, "Halab"),
            Self::Hamah => write!(f, "Hamah"),
            Self::Hims => write!(f, "Hims"),
            Self::Idlib => write!(f, "Idlib"),
            Self::Tartus => write!(f, "Tartus"),
        }
    }
}

// São Tomé and Príncipe Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaoTomeAndPrincipeSubregions {
    SaoTomeAndPrincipe,
    SaoTome,
    Principe,
}

impl TryFrom<u8> for SaoTomeAndPrincipeSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::SaoTomeAndPrincipe),
            0x02 => Ok(Self::SaoTome),
            0x03 => Ok(Self::Principe),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<SaoTomeAndPrincipeSubregions> for u8 {
    fn from(value: SaoTomeAndPrincipeSubregions) -> u8 {
        match value {
            SaoTomeAndPrincipeSubregions::SaoTomeAndPrincipe => 0x01,
            SaoTomeAndPrincipeSubregions::SaoTome => 0x02,
            SaoTomeAndPrincipeSubregions::Principe => 0x03,
        }
    }
}

impl Display for SaoTomeAndPrincipeSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaoTomeAndPrincipe => write!(f, "São Tomé and Príncipe"),
            Self::SaoTome => write!(f, "São Tomé"),
            Self::Principe => write!(f, "Príncipe"),
        }
    }
}

// Taiwan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaiwanSubregions {
    Taiwan,
    TaipeiCity,
    KaohsiungCity,
    KeelungCity,
    HsinchuCity,
    TaichungCity,
    ChiayiCity,
    TainanCity,
    TaipeiCounty,
    TaoyuanCounty,
    HsinchuCounty,
    MiaoliCounty,
    TaichungCounty,
    ChanghuaCounty,
    NantouCounty,
    YunlinCounty,
    ChiayiCounty,
    TainanCounty,
    KaohsiungCounty,
    PingtungCounty,
    YilanCounty,
    HualienCounty,
    TaitungCounty,
    PenghuCounty,
    KinmenCounty,
    LienchiangCounty,
}

impl TryFrom<u8> for TaiwanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Taiwan),
            0x02 => Ok(Self::TaipeiCity),
            0x03 => Ok(Self::KaohsiungCity),
            0x04 => Ok(Self::KeelungCity),
            0x05 => Ok(Self::HsinchuCity),
            0x06 => Ok(Self::TaichungCity),
            0x07 => Ok(Self::ChiayiCity),
            0x08 => Ok(Self::TainanCity),
            0x09 => Ok(Self::TaipeiCounty),
            0x0A => Ok(Self::TaoyuanCounty),
            0x0B => Ok(Self::HsinchuCounty),
            0x0C => Ok(Self::MiaoliCounty),
            0x0D => Ok(Self::TaichungCounty),
            0x0E => Ok(Self::ChanghuaCounty),
            0x0F => Ok(Self::NantouCounty),
            0x10 => Ok(Self::YunlinCounty),
            0x11 => Ok(Self::ChiayiCounty),
            0x12 => Ok(Self::TainanCounty),
            0x13 => Ok(Self::KaohsiungCounty),
            0x14 => Ok(Self::PingtungCounty),
            0x15 => Ok(Self::YilanCounty),
            0x16 => Ok(Self::HualienCounty),
            0x17 => Ok(Self::TaitungCounty),
            0x18 => Ok(Self::PenghuCounty),
            0x19 => Ok(Self::KinmenCounty),
            0x1A => Ok(Self::LienchiangCounty),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TaiwanSubregions> for u8 {
    fn from(value: TaiwanSubregions) -> u8 {
        match value {
            TaiwanSubregions::Taiwan => 0x01,
            TaiwanSubregions::TaipeiCity => 0x02,
            TaiwanSubregions::KaohsiungCity => 0x03,
            TaiwanSubregions::KeelungCity => 0x04,
            TaiwanSubregions::HsinchuCity => 0x05,
            TaiwanSubregions::TaichungCity => 0x06,
            TaiwanSubregions::ChiayiCity => 0x07,
            TaiwanSubregions::TainanCity => 0x08,
            TaiwanSubregions::TaipeiCounty => 0x09,
            TaiwanSubregions::TaoyuanCounty => 0x0A,
            TaiwanSubregions::HsinchuCounty => 0x0B,
            TaiwanSubregions::MiaoliCounty => 0x0C,
            TaiwanSubregions::TaichungCounty => 0x0D,
            TaiwanSubregions::ChanghuaCounty => 0x0E,
            TaiwanSubregions::NantouCounty => 0x0F,
            TaiwanSubregions::YunlinCounty => 0x10,
            TaiwanSubregions::ChiayiCounty => 0x11,
            TaiwanSubregions::TainanCounty => 0x12,
            TaiwanSubregions::KaohsiungCounty => 0x13,
            TaiwanSubregions::PingtungCounty => 0x14,
            TaiwanSubregions::YilanCounty => 0x15,
            TaiwanSubregions::HualienCounty => 0x16,
            TaiwanSubregions::TaitungCounty => 0x17,
            TaiwanSubregions::PenghuCounty => 0x18,
            TaiwanSubregions::KinmenCounty => 0x19,
            TaiwanSubregions::LienchiangCounty => 0x1A,
        }
    }
}

impl Display for TaiwanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Taiwan => write!(f, "Taiwan"),
            Self::TaipeiCity => write!(f, "Taipei City"),
            Self::KaohsiungCity => write!(f, "Kaohsiung City"),
            Self::KeelungCity => write!(f, "Keelung City"),
            Self::HsinchuCity => write!(f, "Hsinchu City"),
            Self::TaichungCity => write!(f, "Taichung City"),
            Self::ChiayiCity => write!(f, "Chiayi City"),
            Self::TainanCity => write!(f, "Tainan City"),
            Self::TaipeiCounty => write!(f, "Taipei County"),
            Self::TaoyuanCounty => write!(f, "Taoyuan County"),
            Self::HsinchuCounty => write!(f, "HsinChu County"),
            Self::MiaoliCounty => write!(f, "Miaoli County"),
            Self::TaichungCounty => write!(f, "Taichung County"),
            Self::ChanghuaCounty => write!(f, "Changhua County"),
            Self::NantouCounty => write!(f, "Nantou County"),
            Self::YunlinCounty => write!(f, "Yunlin County"),
            Self::ChiayiCounty => write!(f, "Chiayi County"),
            Self::TainanCounty => write!(f, "Tainan County"),
            Self::KaohsiungCounty => write!(f, "Kaohsiung County"),
            Self::PingtungCounty => write!(f, "Pingtung County"),
            Self::YilanCounty => write!(f, "Yilan County"),
            Self::HualienCounty => write!(f, "Hualien County"),
            Self::TaitungCounty => write!(f, "Taitung County"),
            Self::PenghuCounty => write!(f, "Penghu County"),
            Self::KinmenCounty => write!(f, "Kinmen County"),
            Self::LienchiangCounty => write!(f, "Lienchiang County"),
        }
    }
}

// Tajikistan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TajikistanSubregions {
    Tajikistan,
    DistrictsOfRepublicanSubordination,
    GornoBadakhshanAutonomousRegion,
    KhatlonRegion,
    SughdRegion,
}

impl TryFrom<u8> for TajikistanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Tajikistan),
            0x02 => Ok(Self::DistrictsOfRepublicanSubordination),
            0x03 => Ok(Self::GornoBadakhshanAutonomousRegion),
            0x04 => Ok(Self::KhatlonRegion),
            0x05 => Ok(Self::SughdRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TajikistanSubregions> for u8 {
    fn from(value: TajikistanSubregions) -> u8 {
        match value {
            TajikistanSubregions::Tajikistan => 0x01,
            TajikistanSubregions::DistrictsOfRepublicanSubordination => 0x02,
            TajikistanSubregions::GornoBadakhshanAutonomousRegion => 0x03,
            TajikistanSubregions::KhatlonRegion => 0x04,
            TajikistanSubregions::SughdRegion => 0x05,
        }
    }
}

impl Display for TajikistanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tajikistan => write!(f, "Tajikistan"),
            Self::DistrictsOfRepublicanSubordination => {
                write!(f, "Districts of Republican Subordination")
            }
            Self::GornoBadakhshanAutonomousRegion => {
                write!(f, "Gorno-Badakhshan Autonomous Region")
            }
            Self::KhatlonRegion => write!(f, "Khatlon Region"),
            Self::SughdRegion => write!(f, "Sughd Region"),
        }
    }
}

// Tanzania Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TanzaniaSubregions {
    Tanzania,
    DodomaRegion,
    ArushaRegion,
    DarEsSalaamRegion,
    GeitaRegion,
    IringaRegion,
    KageraRegion,
    KataviRegion,
    KigomaRegion,
    KilimanjaroRegion,
    LindiRegion,
    ManyaraRegion,
    MaraRegion,
    MbeyaRegion,
    MjiniMagharibiRegion,
    MorogoroRegion,
    MtwaraRegion,
    MwanzaRegion,
    NjombeRegion,
    PembaNorthRegion,
    PembaSouthRegion,
    PwaniRegion,
    RukwaRegion,
    RuvumaRegion,
    ShinyangaRegion,
    SimiyuRegion,
    SingidaRegion,
    SongweRegion,
    TaboraRegion,
    TangaRegion,
    UngujaNorthRegion,
    UngujaSouthRegion,
}

impl TryFrom<u8> for TanzaniaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Tanzania),
            0x02 => Ok(Self::DodomaRegion),
            0x03 => Ok(Self::ArushaRegion),
            0x04 => Ok(Self::DarEsSalaamRegion),
            0x05 => Ok(Self::GeitaRegion),
            0x06 => Ok(Self::IringaRegion),
            0x07 => Ok(Self::KageraRegion),
            0x08 => Ok(Self::KataviRegion),
            0x09 => Ok(Self::KigomaRegion),
            0x0A => Ok(Self::KilimanjaroRegion),
            0x0B => Ok(Self::LindiRegion),
            0x0C => Ok(Self::ManyaraRegion),
            0x0D => Ok(Self::MaraRegion),
            0x0E => Ok(Self::MbeyaRegion),
            0x0F => Ok(Self::MjiniMagharibiRegion),
            0x10 => Ok(Self::MorogoroRegion),
            0x11 => Ok(Self::MtwaraRegion),
            0x12 => Ok(Self::MwanzaRegion),
            0x13 => Ok(Self::NjombeRegion),
            0x14 => Ok(Self::PembaNorthRegion),
            0x15 => Ok(Self::PembaSouthRegion),
            0x16 => Ok(Self::PwaniRegion),
            0x17 => Ok(Self::RukwaRegion),
            0x18 => Ok(Self::RuvumaRegion),
            0x19 => Ok(Self::ShinyangaRegion),
            0x1A => Ok(Self::SimiyuRegion),
            0x1B => Ok(Self::SingidaRegion),
            0x1C => Ok(Self::SongweRegion),
            0x1D => Ok(Self::TaboraRegion),
            0x1E => Ok(Self::TangaRegion),
            0x1F => Ok(Self::UngujaNorthRegion),
            0x20 => Ok(Self::UngujaSouthRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TanzaniaSubregions> for u8 {
    fn from(value: TanzaniaSubregions) -> u8 {
        match value {
            TanzaniaSubregions::Tanzania => 0x01,
            TanzaniaSubregions::DodomaRegion => 0x02,
            TanzaniaSubregions::ArushaRegion => 0x03,
            TanzaniaSubregions::DarEsSalaamRegion => 0x04,
            TanzaniaSubregions::GeitaRegion => 0x05,
            TanzaniaSubregions::IringaRegion => 0x06,
            TanzaniaSubregions::KageraRegion => 0x07,
            TanzaniaSubregions::KataviRegion => 0x08,
            TanzaniaSubregions::KigomaRegion => 0x09,
            TanzaniaSubregions::KilimanjaroRegion => 0x0A,
            TanzaniaSubregions::LindiRegion => 0x0B,
            TanzaniaSubregions::ManyaraRegion => 0x0C,
            TanzaniaSubregions::MaraRegion => 0x0D,
            TanzaniaSubregions::MbeyaRegion => 0x0E,
            TanzaniaSubregions::MjiniMagharibiRegion => 0x0F,
            TanzaniaSubregions::MorogoroRegion => 0x10,
            TanzaniaSubregions::MtwaraRegion => 0x11,
            TanzaniaSubregions::MwanzaRegion => 0x12,
            TanzaniaSubregions::NjombeRegion => 0x13,
            TanzaniaSubregions::PembaNorthRegion => 0x14,
            TanzaniaSubregions::PembaSouthRegion => 0x15,
            TanzaniaSubregions::PwaniRegion => 0x16,
            TanzaniaSubregions::RukwaRegion => 0x17,
            TanzaniaSubregions::RuvumaRegion => 0x18,
            TanzaniaSubregions::ShinyangaRegion => 0x19,
            TanzaniaSubregions::SimiyuRegion => 0x1A,
            TanzaniaSubregions::SingidaRegion => 0x1B,
            TanzaniaSubregions::SongweRegion => 0x1C,
            TanzaniaSubregions::TaboraRegion => 0x1D,
            TanzaniaSubregions::TangaRegion => 0x1E,
            TanzaniaSubregions::UngujaNorthRegion => 0x1F,
            TanzaniaSubregions::UngujaSouthRegion => 0x20,
        }
    }
}

impl Display for TanzaniaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tanzania => write!(f, "Tanzania"),
            Self::DodomaRegion => write!(f, "Dodoma Region"),
            Self::ArushaRegion => write!(f, "Arusha Region"),
            Self::DarEsSalaamRegion => write!(f, "Dar es Salaam Region"),
            Self::GeitaRegion => write!(f, "Geita Region"),
            Self::IringaRegion => write!(f, "Iringa Region"),
            Self::KageraRegion => write!(f, "Kagera Region"),
            Self::KataviRegion => write!(f, "Katavi Region"),
            Self::KigomaRegion => write!(f, "Kigoma Region"),
            Self::KilimanjaroRegion => write!(f, "Kilimanjaro Region"),
            Self::LindiRegion => write!(f, "Lindi Region"),
            Self::ManyaraRegion => write!(f, "Manyara Region"),
            Self::MaraRegion => write!(f, "Mara Region"),
            Self::MbeyaRegion => write!(f, "Mbeya Region"),
            Self::MjiniMagharibiRegion => write!(f, "Mjini Magharibi Region"),
            Self::MorogoroRegion => write!(f, "Morogoro Region"),
            Self::MtwaraRegion => write!(f, "Mtwara Region"),
            Self::MwanzaRegion => write!(f, "Mwanza Region"),
            Self::NjombeRegion => write!(f, "Njombe Region"),
            Self::PembaNorthRegion => write!(f, "Pemba North Region"),
            Self::PembaSouthRegion => write!(f, "Pemba South Region"),
            Self::PwaniRegion => write!(f, "Pwani Region"),
            Self::RukwaRegion => write!(f, "Rukwa Region"),
            Self::RuvumaRegion => write!(f, "Ruvuma Region"),
            Self::ShinyangaRegion => write!(f, "Shinyanga Region"),
            Self::SimiyuRegion => write!(f, "Simiyu Region"),
            Self::SingidaRegion => write!(f, "Singida Region"),
            Self::SongweRegion => write!(f, "Songwe Region"),
            Self::TaboraRegion => write!(f, "Tabora Region"),
            Self::TangaRegion => write!(f, "Tanga Region"),
            Self::UngujaNorthRegion => write!(f, "Unguja North Region"),
            Self::UngujaSouthRegion => write!(f, "Unguja South Region"),
        }
    }
}

// Thailand Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThailandSubregions {
    Thailand,
    KrungThepMahanakhon,
    MaeHongSon,
    ChiangMai,
    ChiangRai,
    Nan,
    Lamphun,
    Lampang,
    Phrae,
    Tak,
    Sukhothai,
    Uttaradit,
    KamphaengPhet,
    Phitsanulok,
    Phichit,
    Phetchabun,
    UthaiThani,
    NakhonSawan,
    NongKhai,
    Loei,
    SakonNakhon,
    KhonKaen,
    Kalasin,
    MahaSarakham,
    RoiEt,
    Chaiyaphum,
    NakhonRatchasima,
    Buriram,
    Surin,
    Sisaket,
    Narathiwat,
    ChaiNat,
    SingBuri,
    LopBuri,
    AngThong,
    PhraNakhonSiAyutthaya,
    SaraBuri,
    Nonthaburi,
    PathumThani,
    Phayao,
    SamutPrakan,
    NakhonNayok,
    Chachoengsao,
    ChonBuri,
    Rayong,
    Chanthaburi,
    Trat,
    Kanchanaburi,
    SuphanBuri,
    Ratchaburi,
    NakhonPathom,
    SamutSongkhram,
    SamutSakhon,
    Phetchaburi,
    PrachuapKhiriKhan,
    Chumphon,
    Ranong,
    SuratThani,
    Phangnga,
    Phuket,
    Krabi,
    NakhonSiThammarat,
    Trang,
    Phatthalung,
    Satun,
    Songkhla,
    Pattani,
    Yala,
    Yasothon,
    NakhonPhanom,
    PrachinBuri,
    UbonRatchathani,
    UdonThani,
    AmnatCharoen,
    Mukdahan,
    NongBuaLamphu,
    SaKaeo,
}

impl TryFrom<u8> for ThailandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Thailand),
            0x02 => Ok(Self::KrungThepMahanakhon),
            0x03 => Ok(Self::MaeHongSon),
            0x04 => Ok(Self::ChiangMai),
            0x05 => Ok(Self::ChiangRai),
            0x06 => Ok(Self::Nan),
            0x07 => Ok(Self::Lamphun),
            0x08 => Ok(Self::Lampang),
            0x09 => Ok(Self::Phrae),
            0x0A => Ok(Self::Tak),
            0x0B => Ok(Self::Sukhothai),
            0x0C => Ok(Self::Uttaradit),
            0x0D => Ok(Self::KamphaengPhet),
            0x0E => Ok(Self::Phitsanulok),
            0x0F => Ok(Self::Phichit),
            0x10 => Ok(Self::Phetchabun),
            0x11 => Ok(Self::UthaiThani),
            0x12 => Ok(Self::NakhonSawan),
            0x13 => Ok(Self::NongKhai),
            0x14 => Ok(Self::Loei),
            0x15 => Ok(Self::SakonNakhon),
            0x16 => Ok(Self::KhonKaen),
            0x17 => Ok(Self::Kalasin),
            0x18 => Ok(Self::MahaSarakham),
            0x19 => Ok(Self::RoiEt),
            0x1A => Ok(Self::Chaiyaphum),
            0x1B => Ok(Self::NakhonRatchasima),
            0x1C => Ok(Self::Buriram),
            0x1D => Ok(Self::Surin),
            0x1E => Ok(Self::Sisaket),
            0x1F => Ok(Self::Narathiwat),
            0x20 => Ok(Self::ChaiNat),
            0x21 => Ok(Self::SingBuri),
            0x22 => Ok(Self::LopBuri),
            0x23 => Ok(Self::AngThong),
            0x24 => Ok(Self::PhraNakhonSiAyutthaya),
            0x25 => Ok(Self::SaraBuri),
            0x26 => Ok(Self::Nonthaburi),
            0x27 => Ok(Self::PathumThani),
            0x28 => Ok(Self::Phayao),
            0x29 => Ok(Self::SamutPrakan),
            0x2A => Ok(Self::NakhonNayok),
            0x2B => Ok(Self::Chachoengsao),
            0x2C => Ok(Self::ChonBuri),
            0x2D => Ok(Self::Rayong),
            0x2E => Ok(Self::Chanthaburi),
            0x2F => Ok(Self::Trat),
            0x30 => Ok(Self::Kanchanaburi),
            0x31 => Ok(Self::SuphanBuri),
            0x32 => Ok(Self::Ratchaburi),
            0x33 => Ok(Self::NakhonPathom),
            0x34 => Ok(Self::SamutSongkhram),
            0x35 => Ok(Self::SamutSakhon),
            0x36 => Ok(Self::Phetchaburi),
            0x37 => Ok(Self::PrachuapKhiriKhan),
            0x38 => Ok(Self::Chumphon),
            0x39 => Ok(Self::Ranong),
            0x3A => Ok(Self::SuratThani),
            0x3B => Ok(Self::Phangnga),
            0x3C => Ok(Self::Phuket),
            0x3D => Ok(Self::Krabi),
            0x3E => Ok(Self::NakhonSiThammarat),
            0x3F => Ok(Self::Trang),
            0x40 => Ok(Self::Phatthalung),
            0x41 => Ok(Self::Satun),
            0x42 => Ok(Self::Songkhla),
            0x43 => Ok(Self::Pattani),
            0x44 => Ok(Self::Yala),
            0x45 => Ok(Self::Yasothon),
            0x46 => Ok(Self::NakhonPhanom),
            0x47 => Ok(Self::PrachinBuri),
            0x48 => Ok(Self::UbonRatchathani),
            0x49 => Ok(Self::UdonThani),
            0x4A => Ok(Self::AmnatCharoen),
            0x4B => Ok(Self::Mukdahan),
            0x4C => Ok(Self::NongBuaLamphu),
            0x4D => Ok(Self::SaKaeo),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ThailandSubregions> for u8 {
    fn from(value: ThailandSubregions) -> u8 {
        match value {
            ThailandSubregions::Thailand => 0x01,
            ThailandSubregions::KrungThepMahanakhon => 0x02,
            ThailandSubregions::MaeHongSon => 0x03,
            ThailandSubregions::ChiangMai => 0x04,
            ThailandSubregions::ChiangRai => 0x05,
            ThailandSubregions::Nan => 0x06,
            ThailandSubregions::Lamphun => 0x07,
            ThailandSubregions::Lampang => 0x08,
            ThailandSubregions::Phrae => 0x09,
            ThailandSubregions::Tak => 0x0A,
            ThailandSubregions::Sukhothai => 0x0B,
            ThailandSubregions::Uttaradit => 0x0C,
            ThailandSubregions::KamphaengPhet => 0x0D,
            ThailandSubregions::Phitsanulok => 0x0E,
            ThailandSubregions::Phichit => 0x0F,
            ThailandSubregions::Phetchabun => 0x10,
            ThailandSubregions::UthaiThani => 0x11,
            ThailandSubregions::NakhonSawan => 0x12,
            ThailandSubregions::NongKhai => 0x13,
            ThailandSubregions::Loei => 0x14,
            ThailandSubregions::SakonNakhon => 0x15,
            ThailandSubregions::KhonKaen => 0x16,
            ThailandSubregions::Kalasin => 0x17,
            ThailandSubregions::MahaSarakham => 0x18,
            ThailandSubregions::RoiEt => 0x19,
            ThailandSubregions::Chaiyaphum => 0x1A,
            ThailandSubregions::NakhonRatchasima => 0x1B,
            ThailandSubregions::Buriram => 0x1C,
            ThailandSubregions::Surin => 0x1D,
            ThailandSubregions::Sisaket => 0x1E,
            ThailandSubregions::Narathiwat => 0x1F,
            ThailandSubregions::ChaiNat => 0x20,
            ThailandSubregions::SingBuri => 0x21,
            ThailandSubregions::LopBuri => 0x22,
            ThailandSubregions::AngThong => 0x23,
            ThailandSubregions::PhraNakhonSiAyutthaya => 0x24,
            ThailandSubregions::SaraBuri => 0x25,
            ThailandSubregions::Nonthaburi => 0x26,
            ThailandSubregions::PathumThani => 0x27,
            ThailandSubregions::Phayao => 0x28,
            ThailandSubregions::SamutPrakan => 0x29,
            ThailandSubregions::NakhonNayok => 0x2A,
            ThailandSubregions::Chachoengsao => 0x2B,
            ThailandSubregions::ChonBuri => 0x2C,
            ThailandSubregions::Rayong => 0x2D,
            ThailandSubregions::Chanthaburi => 0x2E,
            ThailandSubregions::Trat => 0x2F,
            ThailandSubregions::Kanchanaburi => 0x30,
            ThailandSubregions::SuphanBuri => 0x31,
            ThailandSubregions::Ratchaburi => 0x32,
            ThailandSubregions::NakhonPathom => 0x33,
            ThailandSubregions::SamutSongkhram => 0x34,
            ThailandSubregions::SamutSakhon => 0x35,
            ThailandSubregions::Phetchaburi => 0x36,
            ThailandSubregions::PrachuapKhiriKhan => 0x37,
            ThailandSubregions::Chumphon => 0x38,
            ThailandSubregions::Ranong => 0x39,
            ThailandSubregions::SuratThani => 0x3A,
            ThailandSubregions::Phangnga => 0x3B,
            ThailandSubregions::Phuket => 0x3C,
            ThailandSubregions::Krabi => 0x3D,
            ThailandSubregions::NakhonSiThammarat => 0x3E,
            ThailandSubregions::Trang => 0x3F,
            ThailandSubregions::Phatthalung => 0x40,
            ThailandSubregions::Satun => 0x41,
            ThailandSubregions::Songkhla => 0x42,
            ThailandSubregions::Pattani => 0x43,
            ThailandSubregions::Yala => 0x44,
            ThailandSubregions::Yasothon => 0x45,
            ThailandSubregions::NakhonPhanom => 0x46,
            ThailandSubregions::PrachinBuri => 0x47,
            ThailandSubregions::UbonRatchathani => 0x48,
            ThailandSubregions::UdonThani => 0x49,
            ThailandSubregions::AmnatCharoen => 0x4A,
            ThailandSubregions::Mukdahan => 0x4B,
            ThailandSubregions::NongBuaLamphu => 0x4C,
            ThailandSubregions::SaKaeo => 0x4D,
        }
    }
}

impl Display for ThailandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thailand => write!(f, "Thailand"),
            Self::KrungThepMahanakhon => write!(f, "Krung Thep Mahanakhon"),
            Self::MaeHongSon => write!(f, "Mae Hong Son"),
            Self::ChiangMai => write!(f, "Chiang Mai"),
            Self::ChiangRai => write!(f, "Chiang Rai"),
            Self::Nan => write!(f, "Nan"),
            Self::Lamphun => write!(f, "Lamphun"),
            Self::Lampang => write!(f, "Lampang"),
            Self::Phrae => write!(f, "Phrae"),
            Self::Tak => write!(f, "Tak"),
            Self::Sukhothai => write!(f, "Sukhothai"),
            Self::Uttaradit => write!(f, "Uttaradit"),
            Self::KamphaengPhet => write!(f, "Kamphaeng Phet"),
            Self::Phitsanulok => write!(f, "Phitsanulok"),
            Self::Phichit => write!(f, "Phichit"),
            Self::Phetchabun => write!(f, "Phetchabun"),
            Self::UthaiThani => write!(f, "Uthai Thani"),
            Self::NakhonSawan => write!(f, "Nakhon Sawan"),
            Self::NongKhai => write!(f, "Nong Khai"),
            Self::Loei => write!(f, "Loei"),
            Self::SakonNakhon => write!(f, "Sakon Nakhon"),
            Self::KhonKaen => write!(f, "Khon Kaen"),
            Self::Kalasin => write!(f, "Kalasin"),
            Self::MahaSarakham => write!(f, "Maha Sarakham"),
            Self::RoiEt => write!(f, "Roi Et"),
            Self::Chaiyaphum => write!(f, "Chaiyaphum"),
            Self::NakhonRatchasima => write!(f, "Nakhon Ratchasima"),
            Self::Buriram => write!(f, "Buriram"),
            Self::Surin => write!(f, "Surin"),
            Self::Sisaket => write!(f, "Sisaket"),
            Self::Narathiwat => write!(f, "Narathiwat"),
            Self::ChaiNat => write!(f, "Chai Nat"),
            Self::SingBuri => write!(f, "Sing Buri"),
            Self::LopBuri => write!(f, "Lop Buri"),
            Self::AngThong => write!(f, "Ang Thong"),
            Self::PhraNakhonSiAyutthaya => write!(f, "Phra Nakhon Si Ayutthaya"),
            Self::SaraBuri => write!(f, "Sara Buri"),
            Self::Nonthaburi => write!(f, "Nonthaburi"),
            Self::PathumThani => write!(f, "Pathum Thani"),
            Self::Phayao => write!(f, "Phayao"),
            Self::SamutPrakan => write!(f, "Samut Prakan"),
            Self::NakhonNayok => write!(f, "Nakhon Nayok"),
            Self::Chachoengsao => write!(f, "Chachoengsao"),
            Self::ChonBuri => write!(f, "Chon Buri"),
            Self::Rayong => write!(f, "Rayong"),
            Self::Chanthaburi => write!(f, "Chanthaburi"),
            Self::Trat => write!(f, "Trat"),
            Self::Kanchanaburi => write!(f, "Kanchanaburi"),
            Self::SuphanBuri => write!(f, "Suphan Buri"),
            Self::Ratchaburi => write!(f, "Ratchaburi"),
            Self::NakhonPathom => write!(f, "Nakhon Pathom"),
            Self::SamutSongkhram => write!(f, "Samut Songkhram"),
            Self::SamutSakhon => write!(f, "Samut Sakhon"),
            Self::Phetchaburi => write!(f, "Phetchaburi"),
            Self::PrachuapKhiriKhan => write!(f, "Prachuap Khiri Khan"),
            Self::Chumphon => write!(f, "Chumphon"),
            Self::Ranong => write!(f, "Ranong"),
            Self::SuratThani => write!(f, "Surat Thani"),
            Self::Phangnga => write!(f, "Phangnga"),
            Self::Phuket => write!(f, "Phuket"),
            Self::Krabi => write!(f, "Krabi"),
            Self::NakhonSiThammarat => write!(f, "Nakhon Si Thammarat"),
            Self::Trang => write!(f, "Trang"),
            Self::Phatthalung => write!(f, "Phatthalung"),
            Self::Satun => write!(f, "Satun"),
            Self::Songkhla => write!(f, "Songkhla"),
            Self::Pattani => write!(f, "Pattani"),
            Self::Yala => write!(f, "Yala"),
            Self::Yasothon => write!(f, "Yasothon"),
            Self::NakhonPhanom => write!(f, "Nakhon Phanom"),
            Self::PrachinBuri => write!(f, "Prachin Buri"),
            Self::UbonRatchathani => write!(f, "Ubon Ratchathani"),
            Self::UdonThani => write!(f, "Udon Thani"),
            Self::AmnatCharoen => write!(f, "Amnat Charoen"),
            Self::Mukdahan => write!(f, "Mukdahan"),
            Self::NongBuaLamphu => write!(f, "Nong Bua Lamphu"),
            Self::SaKaeo => write!(f, "Sa Kaeo"),
        }
    }
}

// The Gambia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TheGambiaSubregions {
    TheGambia,
    Banjul,
    CentralRiver,
    LowerRiver,
    NorthBank,
    UpperRiver,
    Western,
}

impl TryFrom<u8> for TheGambiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::TheGambia),
            0x02 => Ok(Self::Banjul),
            0x03 => Ok(Self::CentralRiver),
            0x04 => Ok(Self::LowerRiver),
            0x05 => Ok(Self::NorthBank),
            0x06 => Ok(Self::UpperRiver),
            0x07 => Ok(Self::Western),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TheGambiaSubregions> for u8 {
    fn from(value: TheGambiaSubregions) -> u8 {
        match value {
            TheGambiaSubregions::TheGambia => 0x01,
            TheGambiaSubregions::Banjul => 0x02,
            TheGambiaSubregions::CentralRiver => 0x03,
            TheGambiaSubregions::LowerRiver => 0x04,
            TheGambiaSubregions::NorthBank => 0x05,
            TheGambiaSubregions::UpperRiver => 0x06,
            TheGambiaSubregions::Western => 0x07,
        }
    }
}

impl Display for TheGambiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TheGambia => write!(f, "The Gambia"),
            Self::Banjul => write!(f, "Banjul"),
            Self::CentralRiver => write!(f, "Central River"),
            Self::LowerRiver => write!(f, "Lower River"),
            Self::NorthBank => write!(f, "North Bank"),
            Self::UpperRiver => write!(f, "Upper River"),
            Self::Western => write!(f, "Western"),
        }
    }
}

// Timor-Leste Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimorLesteSubregions {
    TimorLeste,
    DiliMunicipality,
    AileuMunicipality,
    AinaroMunicipality,
    BaucauMunicipality,
    BobonaroMunicipality,
    CovaLimaMunicipality,
    ErmeraMunicipality,
    LautemMunicipality,
    LiquicaMunicipality,
    ManatutoMunicipality,
    ManufahiMunicipality,
    Oecusse,
    ViquequeMunicipality,
}

impl TryFrom<u8> for TimorLesteSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::TimorLeste),
            0x02 => Ok(Self::DiliMunicipality),
            0x03 => Ok(Self::AileuMunicipality),
            0x04 => Ok(Self::AinaroMunicipality),
            0x05 => Ok(Self::BaucauMunicipality),
            0x06 => Ok(Self::BobonaroMunicipality),
            0x07 => Ok(Self::CovaLimaMunicipality),
            0x08 => Ok(Self::ErmeraMunicipality),
            0x09 => Ok(Self::LautemMunicipality),
            0x0A => Ok(Self::LiquicaMunicipality),
            0x0B => Ok(Self::ManatutoMunicipality),
            0x0C => Ok(Self::ManufahiMunicipality),
            0x0D => Ok(Self::Oecusse),
            0x0E => Ok(Self::ViquequeMunicipality),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TimorLesteSubregions> for u8 {
    fn from(value: TimorLesteSubregions) -> u8 {
        match value {
            TimorLesteSubregions::TimorLeste => 0x01,
            TimorLesteSubregions::DiliMunicipality => 0x02,
            TimorLesteSubregions::AileuMunicipality => 0x03,
            TimorLesteSubregions::AinaroMunicipality => 0x04,
            TimorLesteSubregions::BaucauMunicipality => 0x05,
            TimorLesteSubregions::BobonaroMunicipality => 0x06,
            TimorLesteSubregions::CovaLimaMunicipality => 0x07,
            TimorLesteSubregions::ErmeraMunicipality => 0x08,
            TimorLesteSubregions::LautemMunicipality => 0x09,
            TimorLesteSubregions::LiquicaMunicipality => 0x0A,
            TimorLesteSubregions::ManatutoMunicipality => 0x0B,
            TimorLesteSubregions::ManufahiMunicipality => 0x0C,
            TimorLesteSubregions::Oecusse => 0x0D,
            TimorLesteSubregions::ViquequeMunicipality => 0x0E,
        }
    }
}

impl Display for TimorLesteSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimorLeste => write!(f, "Timor-Leste"),
            Self::DiliMunicipality => write!(f, "Dili Municipality"),
            Self::AileuMunicipality => write!(f, "Aileu Municipality"),
            Self::AinaroMunicipality => write!(f, "Ainaro Municipality"),
            Self::BaucauMunicipality => write!(f, "Baucau Municipality"),
            Self::BobonaroMunicipality => write!(f, "Bobonaro Municipality"),
            Self::CovaLimaMunicipality => write!(f, "Cova Lima Municipality"),
            Self::ErmeraMunicipality => write!(f, "Ermera Municipality"),
            Self::LautemMunicipality => write!(f, "Lautém Municipality"),
            Self::LiquicaMunicipality => write!(f, "Liquiçá Municipality"),
            Self::ManatutoMunicipality => write!(f, "Manatuto Municipality"),
            Self::ManufahiMunicipality => write!(f, "Manufahi Municipality"),
            Self::Oecusse => write!(f, "Oecusse"),
            Self::ViquequeMunicipality => write!(f, "Viqueque Municipality"),
        }
    }
}

// Togo Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TogoSubregions {
    Togo,
    Maritime,
    Centrale,
    Kara,
    Plateaux,
    Savanes,
}

impl TryFrom<u8> for TogoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Togo),
            0x02 => Ok(Self::Maritime),
            0x03 => Ok(Self::Centrale),
            0x04 => Ok(Self::Kara),
            0x05 => Ok(Self::Plateaux),
            0x06 => Ok(Self::Savanes),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TogoSubregions> for u8 {
    fn from(value: TogoSubregions) -> u8 {
        match value {
            TogoSubregions::Togo => 0x01,
            TogoSubregions::Maritime => 0x02,
            TogoSubregions::Centrale => 0x03,
            TogoSubregions::Kara => 0x04,
            TogoSubregions::Plateaux => 0x05,
            TogoSubregions::Savanes => 0x06,
        }
    }
}

impl Display for TogoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Togo => write!(f, "Togo"),
            Self::Maritime => write!(f, "Maritime"),
            Self::Centrale => write!(f, "Centrale"),
            Self::Kara => write!(f, "Kara"),
            Self::Plateaux => write!(f, "Plateaux"),
            Self::Savanes => write!(f, "Savanes"),
        }
    }
}

// Tokelau Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokelauSubregions {
    Tokelau,
    Atafu,
    Fakaofo,
    Nukunonu,
}

impl TryFrom<u8> for TokelauSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Tokelau),
            0x02 => Ok(Self::Atafu),
            0x03 => Ok(Self::Fakaofo),
            0x04 => Ok(Self::Nukunonu),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TokelauSubregions> for u8 {
    fn from(value: TokelauSubregions) -> u8 {
        match value {
            TokelauSubregions::Tokelau => 0x01,
            TokelauSubregions::Atafu => 0x02,
            TokelauSubregions::Fakaofo => 0x03,
            TokelauSubregions::Nukunonu => 0x04,
        }
    }
}

impl Display for TokelauSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tokelau => write!(f, "Tokelau"),
            Self::Atafu => write!(f, "Atafu"),
            Self::Fakaofo => write!(f, "Fakaofo"),
            Self::Nukunonu => write!(f, "Nukunonu"),
        }
    }
}

// Tonga Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TongaSubregions {
    Tonga,
    Tongatapu,
    Eua,
    Haapai,
    OngoNiua,
    Vavau,
}

impl TryFrom<u8> for TongaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Tonga),
            0x02 => Ok(Self::Tongatapu),
            0x03 => Ok(Self::Eua),
            0x04 => Ok(Self::Haapai),
            0x05 => Ok(Self::OngoNiua),
            0x06 => Ok(Self::Vavau),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TongaSubregions> for u8 {
    fn from(value: TongaSubregions) -> u8 {
        match value {
            TongaSubregions::Tonga => 0x01,
            TongaSubregions::Tongatapu => 0x02,
            TongaSubregions::Eua => 0x03,
            TongaSubregions::Haapai => 0x04,
            TongaSubregions::OngoNiua => 0x05,
            TongaSubregions::Vavau => 0x06,
        }
    }
}

impl Display for TongaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tonga => write!(f, "Tonga"),
            Self::Tongatapu => write!(f, "Tongatapu"),
            Self::Eua => write!(f, "'Eua"),
            Self::Haapai => write!(f, "Ha'apai"),
            Self::OngoNiua => write!(f, "Ongo Niua"),
            Self::Vavau => write!(f, "Vava'u"),
        }
    }
}

// Transnistria Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransnistriaSubregions {
    Transnistria,
    Tiraspol,
    Bender,
    CamencaDistrict,
    DubasariDistrict,
    GrigoriopolDistrict,
    RibnizaDistrict,
    SloboziaDistrict,
}

impl TryFrom<u8> for TransnistriaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Transnistria),
            0x02 => Ok(Self::Tiraspol),
            0x03 => Ok(Self::Bender),
            0x04 => Ok(Self::CamencaDistrict),
            0x05 => Ok(Self::DubasariDistrict),
            0x06 => Ok(Self::GrigoriopolDistrict),
            0x07 => Ok(Self::RibnizaDistrict),
            0x08 => Ok(Self::SloboziaDistrict),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TransnistriaSubregions> for u8 {
    fn from(value: TransnistriaSubregions) -> u8 {
        match value {
            TransnistriaSubregions::Transnistria => 0x01,
            TransnistriaSubregions::Tiraspol => 0x02,
            TransnistriaSubregions::Bender => 0x03,
            TransnistriaSubregions::CamencaDistrict => 0x04,
            TransnistriaSubregions::DubasariDistrict => 0x05,
            TransnistriaSubregions::GrigoriopolDistrict => 0x06,
            TransnistriaSubregions::RibnizaDistrict => 0x07,
            TransnistriaSubregions::SloboziaDistrict => 0x08,
        }
    }
}

impl Display for TransnistriaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transnistria => write!(f, "Transnistria"),
            Self::Tiraspol => write!(f, "Tiraspol"),
            Self::Bender => write!(f, "Bender"),
            Self::CamencaDistrict => write!(f, "Camenca District"),
            Self::DubasariDistrict => write!(f, "Dubasari District"),
            Self::GrigoriopolDistrict => write!(f, "Grigoriopol District"),
            Self::RibnizaDistrict => write!(f, "Rîbniza District"),
            Self::SloboziaDistrict => write!(f, "Slobozia District"),
        }
    }
}

// Trinidad and Tobago Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrinidadAndTobagoSubregions {
    TrinidadAndTobago,
    PortOfSpain,
    Arima,
    Caroni,
    Mayaro,
    Nariva,
    SaintAndrew,
    SaintDavid,
    SaintGeorge,
    SaintPatrick,
    SanFernando,
    Tobago,
    Victoria,
}

impl TryFrom<u8> for TrinidadAndTobagoSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::TrinidadAndTobago),
            0x02 => Ok(Self::PortOfSpain),
            0x03 => Ok(Self::Arima),
            0x04 => Ok(Self::Caroni),
            0x05 => Ok(Self::Mayaro),
            0x06 => Ok(Self::Nariva),
            0x07 => Ok(Self::SaintAndrew),
            0x08 => Ok(Self::SaintDavid),
            0x09 => Ok(Self::SaintGeorge),
            0x0A => Ok(Self::SaintPatrick),
            0x0B => Ok(Self::SanFernando),
            0x0C => Ok(Self::Tobago),
            0x0D => Ok(Self::Victoria),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TrinidadAndTobagoSubregions> for u8 {
    fn from(value: TrinidadAndTobagoSubregions) -> u8 {
        match value {
            TrinidadAndTobagoSubregions::TrinidadAndTobago => 0x01,
            TrinidadAndTobagoSubregions::PortOfSpain => 0x02,
            TrinidadAndTobagoSubregions::Arima => 0x03,
            TrinidadAndTobagoSubregions::Caroni => 0x04,
            TrinidadAndTobagoSubregions::Mayaro => 0x05,
            TrinidadAndTobagoSubregions::Nariva => 0x06,
            TrinidadAndTobagoSubregions::SaintAndrew => 0x07,
            TrinidadAndTobagoSubregions::SaintDavid => 0x08,
            TrinidadAndTobagoSubregions::SaintGeorge => 0x09,
            TrinidadAndTobagoSubregions::SaintPatrick => 0x0A,
            TrinidadAndTobagoSubregions::SanFernando => 0x0B,
            TrinidadAndTobagoSubregions::Tobago => 0x0C,
            TrinidadAndTobagoSubregions::Victoria => 0x0D,
        }
    }
}

impl Display for TrinidadAndTobagoSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TrinidadAndTobago => write!(f, "Trinidad and Tobago"),
            Self::PortOfSpain => write!(f, "Port-of-Spain"),
            Self::Arima => write!(f, "Arima"),
            Self::Caroni => write!(f, "Caroni"),
            Self::Mayaro => write!(f, "Mayaro"),
            Self::Nariva => write!(f, "Nariva"),
            Self::SaintAndrew => write!(f, "Saint Andrew"),
            Self::SaintDavid => write!(f, "Saint David"),
            Self::SaintGeorge => write!(f, "Saint George"),
            Self::SaintPatrick => write!(f, "Saint Patrick"),
            Self::SanFernando => write!(f, "San Fernando"),
            Self::Tobago => write!(f, "Tobago"),
            Self::Victoria => write!(f, "Victoria"),
        }
    }
}

// Tunisia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TunisiaSubregions {
    Tunisia,
    TunisGovernorate,
    ArianaGovernorate,
    BejaGovernorate,
    BenArousGovernorate,
    BizerteGovernorate,
    GabesGovernorate,
    GafsaGovernorate,
    JendoubaGovernorate,
    KairouanGovernorate,
    KasserineGovernorate,
    KebiliGovernorate,
    KefGovernorate,
    MahdiaGovernorate,
    ManoubaGovernorate,
    MedenineGovernorate,
    MonastirGovernorate,
    NabeulGovernorate,
    SfaxGovernorate,
    SidiBouzidGovernorate,
    SilianaGovernorate,
    SousseGovernorate,
    TataouineGovernorate,
    TozeurGovernorate,
    ZaghouanGovernorate,
}

impl TryFrom<u8> for TunisiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Tunisia),
            0x02 => Ok(Self::TunisGovernorate),
            0x03 => Ok(Self::ArianaGovernorate),
            0x04 => Ok(Self::BejaGovernorate),
            0x05 => Ok(Self::BenArousGovernorate),
            0x06 => Ok(Self::BizerteGovernorate),
            0x07 => Ok(Self::GabesGovernorate),
            0x08 => Ok(Self::GafsaGovernorate),
            0x09 => Ok(Self::JendoubaGovernorate),
            0x0A => Ok(Self::KairouanGovernorate),
            0x0B => Ok(Self::KasserineGovernorate),
            0x0C => Ok(Self::KebiliGovernorate),
            0x0D => Ok(Self::KefGovernorate),
            0x0E => Ok(Self::MahdiaGovernorate),
            0x0F => Ok(Self::ManoubaGovernorate),
            0x10 => Ok(Self::MedenineGovernorate),
            0x11 => Ok(Self::MonastirGovernorate),
            0x12 => Ok(Self::NabeulGovernorate),
            0x13 => Ok(Self::SfaxGovernorate),
            0x14 => Ok(Self::SidiBouzidGovernorate),
            0x15 => Ok(Self::SilianaGovernorate),
            0x16 => Ok(Self::SousseGovernorate),
            0x17 => Ok(Self::TataouineGovernorate),
            0x18 => Ok(Self::TozeurGovernorate),
            0x19 => Ok(Self::ZaghouanGovernorate),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TunisiaSubregions> for u8 {
    fn from(value: TunisiaSubregions) -> u8 {
        match value {
            TunisiaSubregions::Tunisia => 0x01,
            TunisiaSubregions::TunisGovernorate => 0x02,
            TunisiaSubregions::ArianaGovernorate => 0x03,
            TunisiaSubregions::BejaGovernorate => 0x04,
            TunisiaSubregions::BenArousGovernorate => 0x05,
            TunisiaSubregions::BizerteGovernorate => 0x06,
            TunisiaSubregions::GabesGovernorate => 0x07,
            TunisiaSubregions::GafsaGovernorate => 0x08,
            TunisiaSubregions::JendoubaGovernorate => 0x09,
            TunisiaSubregions::KairouanGovernorate => 0x0A,
            TunisiaSubregions::KasserineGovernorate => 0x0B,
            TunisiaSubregions::KebiliGovernorate => 0x0C,
            TunisiaSubregions::KefGovernorate => 0x0D,
            TunisiaSubregions::MahdiaGovernorate => 0x0E,
            TunisiaSubregions::ManoubaGovernorate => 0x0F,
            TunisiaSubregions::MedenineGovernorate => 0x10,
            TunisiaSubregions::MonastirGovernorate => 0x11,
            TunisiaSubregions::NabeulGovernorate => 0x12,
            TunisiaSubregions::SfaxGovernorate => 0x13,
            TunisiaSubregions::SidiBouzidGovernorate => 0x14,
            TunisiaSubregions::SilianaGovernorate => 0x15,
            TunisiaSubregions::SousseGovernorate => 0x16,
            TunisiaSubregions::TataouineGovernorate => 0x17,
            TunisiaSubregions::TozeurGovernorate => 0x18,
            TunisiaSubregions::ZaghouanGovernorate => 0x19,
        }
    }
}

impl Display for TunisiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tunisia => write!(f, "Tunisia"),
            Self::TunisGovernorate => write!(f, "Tunis Governorate"),
            Self::ArianaGovernorate => write!(f, "Ariana Governorate"),
            Self::BejaGovernorate => write!(f, "Béja Governorate"),
            Self::BenArousGovernorate => write!(f, "Ben Arous Governorate"),
            Self::BizerteGovernorate => write!(f, "Bizerte Governorate"),
            Self::GabesGovernorate => write!(f, "Gabès Governorate"),
            Self::GafsaGovernorate => write!(f, "Gafsa Governorate"),
            Self::JendoubaGovernorate => write!(f, "Jendouba Governorate"),
            Self::KairouanGovernorate => write!(f, "Kairouan Governorate"),
            Self::KasserineGovernorate => write!(f, "Kasserine Governorate"),
            Self::KebiliGovernorate => write!(f, "Kebili Governorate"),
            Self::KefGovernorate => write!(f, "Kef Governorate"),
            Self::MahdiaGovernorate => write!(f, "Mahdia Governorate"),
            Self::ManoubaGovernorate => write!(f, "Manouba Governorate"),
            Self::MedenineGovernorate => write!(f, "Medenine Governorate"),
            Self::MonastirGovernorate => write!(f, "Monastir Governorate"),
            Self::NabeulGovernorate => write!(f, "Nabeul Governorate"),
            Self::SfaxGovernorate => write!(f, "Sfax Governorate"),
            Self::SidiBouzidGovernorate => write!(f, "Sidi Bouzid Governorate"),
            Self::SilianaGovernorate => write!(f, "Siliana Governorate"),
            Self::SousseGovernorate => write!(f, "Sousse Governorate"),
            Self::TataouineGovernorate => write!(f, "Tataouine Governorate"),
            Self::TozeurGovernorate => write!(f, "Tozeur Governorate"),
            Self::ZaghouanGovernorate => write!(f, "Zaghouan Governorate"),
        }
    }
}

// Turkey Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurkeySubregions {
    Turkey,
    Ankara,
    Istanbul,
    Izmir,
    Bursa,
    Adana,
    Gaziantep,
    Konya,
    Antalya,
    Diyarbakir,
    Mersin,
    Kayseri,
    Eskisehir,
    Sanliurfa,
    Malatya,
    Erzurum,
    Samsun,
    Van,
    Kahramanmaras,
    Denizli,
    Batman,
    Elazig,
    Sakarya,
    Kocaeli,
    Sivas,
    Manisa,
    Trabzon,
    Balikesir,
    Adiyaman,
    Tekirdag,
    Kirikkale,
    Osmaniye,
    Kutahya,
    Corum,
    Isparta,
    Aydin,
    Hatay,
    Mardin,
    Aksaray,
    Afyonkarahisar,
    Tokat,
    Edirne,
    Karaman,
    Ordu,
    Siirt,
    Erzincan,
    Cankiri,
    Zonguldak,
    Yozgat,
    USAk,
}

impl TryFrom<u8> for TurkeySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Turkey),
            0x02 => Ok(Self::Ankara),
            0x03 => Ok(Self::Istanbul),
            0x04 => Ok(Self::Izmir),
            0x05 => Ok(Self::Bursa),
            0x06 => Ok(Self::Adana),
            0x07 => Ok(Self::Gaziantep),
            0x08 => Ok(Self::Konya),
            0x09 => Ok(Self::Antalya),
            0x0A => Ok(Self::Diyarbakir),
            0x0B => Ok(Self::Mersin),
            0x0C => Ok(Self::Kayseri),
            0x0D => Ok(Self::Eskisehir),
            0x0E => Ok(Self::Sanliurfa),
            0x0F => Ok(Self::Malatya),
            0x10 => Ok(Self::Erzurum),
            0x11 => Ok(Self::Samsun),
            0x12 => Ok(Self::Van),
            0x13 => Ok(Self::Kahramanmaras),
            0x14 => Ok(Self::Denizli),
            0x15 => Ok(Self::Batman),
            0x16 => Ok(Self::Elazig),
            0x17 => Ok(Self::Sakarya),
            0x18 => Ok(Self::Kocaeli),
            0x19 => Ok(Self::Sivas),
            0x1A => Ok(Self::Manisa),
            0x1B => Ok(Self::Trabzon),
            0x1C => Ok(Self::Balikesir),
            0x1D => Ok(Self::Adiyaman),
            0x1E => Ok(Self::Tekirdag),
            0x1F => Ok(Self::Kirikkale),
            0x20 => Ok(Self::Osmaniye),
            0x21 => Ok(Self::Kutahya),
            0x22 => Ok(Self::Corum),
            0x23 => Ok(Self::Isparta),
            0x24 => Ok(Self::Aydin),
            0x25 => Ok(Self::Hatay),
            0x26 => Ok(Self::Mardin),
            0x27 => Ok(Self::Aksaray),
            0x28 => Ok(Self::Afyonkarahisar),
            0x29 => Ok(Self::Tokat),
            0x2A => Ok(Self::Edirne),
            0x2B => Ok(Self::Karaman),
            0x2C => Ok(Self::Ordu),
            0x2D => Ok(Self::Siirt),
            0x2E => Ok(Self::Erzincan),
            0x2F => Ok(Self::Cankiri),
            0x30 => Ok(Self::Zonguldak),
            0x31 => Ok(Self::Yozgat),
            0x32 => Ok(Self::USAk),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TurkeySubregions> for u8 {
    fn from(value: TurkeySubregions) -> u8 {
        match value {
            TurkeySubregions::Turkey => 0x01,
            TurkeySubregions::Ankara => 0x02,
            TurkeySubregions::Istanbul => 0x03,
            TurkeySubregions::Izmir => 0x04,
            TurkeySubregions::Bursa => 0x05,
            TurkeySubregions::Adana => 0x06,
            TurkeySubregions::Gaziantep => 0x07,
            TurkeySubregions::Konya => 0x08,
            TurkeySubregions::Antalya => 0x09,
            TurkeySubregions::Diyarbakir => 0x0A,
            TurkeySubregions::Mersin => 0x0B,
            TurkeySubregions::Kayseri => 0x0C,
            TurkeySubregions::Eskisehir => 0x0D,
            TurkeySubregions::Sanliurfa => 0x0E,
            TurkeySubregions::Malatya => 0x0F,
            TurkeySubregions::Erzurum => 0x10,
            TurkeySubregions::Samsun => 0x11,
            TurkeySubregions::Van => 0x12,
            TurkeySubregions::Kahramanmaras => 0x13,
            TurkeySubregions::Denizli => 0x14,
            TurkeySubregions::Batman => 0x15,
            TurkeySubregions::Elazig => 0x16,
            TurkeySubregions::Sakarya => 0x17,
            TurkeySubregions::Kocaeli => 0x18,
            TurkeySubregions::Sivas => 0x19,
            TurkeySubregions::Manisa => 0x1A,
            TurkeySubregions::Trabzon => 0x1B,
            TurkeySubregions::Balikesir => 0x1C,
            TurkeySubregions::Adiyaman => 0x1D,
            TurkeySubregions::Tekirdag => 0x1E,
            TurkeySubregions::Kirikkale => 0x1F,
            TurkeySubregions::Osmaniye => 0x20,
            TurkeySubregions::Kutahya => 0x21,
            TurkeySubregions::Corum => 0x22,
            TurkeySubregions::Isparta => 0x23,
            TurkeySubregions::Aydin => 0x24,
            TurkeySubregions::Hatay => 0x25,
            TurkeySubregions::Mardin => 0x26,
            TurkeySubregions::Aksaray => 0x27,
            TurkeySubregions::Afyonkarahisar => 0x28,
            TurkeySubregions::Tokat => 0x29,
            TurkeySubregions::Edirne => 0x2A,
            TurkeySubregions::Karaman => 0x2B,
            TurkeySubregions::Ordu => 0x2C,
            TurkeySubregions::Siirt => 0x2D,
            TurkeySubregions::Erzincan => 0x2E,
            TurkeySubregions::Cankiri => 0x2F,
            TurkeySubregions::Zonguldak => 0x30,
            TurkeySubregions::Yozgat => 0x31,
            TurkeySubregions::USAk => 0x32,
        }
    }
}

impl Display for TurkeySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Turkey => write!(f, "Turkey"),
            Self::Ankara => write!(f, "Ankara"),
            Self::Istanbul => write!(f, "Istanbul"),
            Self::Izmir => write!(f, "Izmir"),
            Self::Bursa => write!(f, "Bursa"),
            Self::Adana => write!(f, "Adana"),
            Self::Gaziantep => write!(f, "Gaziantep"),
            Self::Konya => write!(f, "Konya"),
            Self::Antalya => write!(f, "Antalya"),
            Self::Diyarbakir => write!(f, "Diyarbakir"),
            Self::Mersin => write!(f, "Mersin"),
            Self::Kayseri => write!(f, "Kayseri"),
            Self::Eskisehir => write!(f, "Eskisehir"),
            Self::Sanliurfa => write!(f, "Sanliurfa"),
            Self::Malatya => write!(f, "Malatya"),
            Self::Erzurum => write!(f, "Erzurum"),
            Self::Samsun => write!(f, "Samsun"),
            Self::Van => write!(f, "Van"),
            Self::Kahramanmaras => write!(f, "Kahramanmaras"),
            Self::Denizli => write!(f, "Denizli"),
            Self::Batman => write!(f, "Batman"),
            Self::Elazig => write!(f, "Elazig"),
            Self::Sakarya => write!(f, "Sakarya"),
            Self::Kocaeli => write!(f, "Kocaeli"),
            Self::Sivas => write!(f, "Sivas"),
            Self::Manisa => write!(f, "Manisa"),
            Self::Trabzon => write!(f, "Trabzon"),
            Self::Balikesir => write!(f, "Balikesir"),
            Self::Adiyaman => write!(f, "Adiyaman"),
            Self::Tekirdag => write!(f, "Tekirdag"),
            Self::Kirikkale => write!(f, "Kirikkale"),
            Self::Osmaniye => write!(f, "Osmaniye"),
            Self::Kutahya => write!(f, "Kütahya"),
            Self::Corum => write!(f, "Corum"),
            Self::Isparta => write!(f, "Isparta"),
            Self::Aydin => write!(f, "Aydin"),
            Self::Hatay => write!(f, "Hatay"),
            Self::Mardin => write!(f, "Mardin"),
            Self::Aksaray => write!(f, "Aksaray"),
            Self::Afyonkarahisar => write!(f, "Afyonkarahisar"),
            Self::Tokat => write!(f, "Tokat"),
            Self::Edirne => write!(f, "Edirne"),
            Self::Karaman => write!(f, "Karaman"),
            Self::Ordu => write!(f, "Ordu"),
            Self::Siirt => write!(f, "Siirt"),
            Self::Erzincan => write!(f, "Erzincan"),
            Self::Cankiri => write!(f, "Cankiri"),
            Self::Zonguldak => write!(f, "Zonguldak"),
            Self::Yozgat => write!(f, "Yozgat"),
            Self::USAk => write!(f, "Usak"),
        }
    }
}

// Turkmenistan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurkmenistanSubregions {
    Turkmenistan,
    Ashgabat,
    AhalRegion,
    BalkanRegion,
    DashoguzRegion,
    LebapRegion,
    MaryRegion,
}

impl TryFrom<u8> for TurkmenistanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Turkmenistan),
            0x02 => Ok(Self::Ashgabat),
            0x03 => Ok(Self::AhalRegion),
            0x04 => Ok(Self::BalkanRegion),
            0x05 => Ok(Self::DashoguzRegion),
            0x06 => Ok(Self::LebapRegion),
            0x07 => Ok(Self::MaryRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TurkmenistanSubregions> for u8 {
    fn from(value: TurkmenistanSubregions) -> u8 {
        match value {
            TurkmenistanSubregions::Turkmenistan => 0x01,
            TurkmenistanSubregions::Ashgabat => 0x02,
            TurkmenistanSubregions::AhalRegion => 0x03,
            TurkmenistanSubregions::BalkanRegion => 0x04,
            TurkmenistanSubregions::DashoguzRegion => 0x05,
            TurkmenistanSubregions::LebapRegion => 0x06,
            TurkmenistanSubregions::MaryRegion => 0x07,
        }
    }
}

impl Display for TurkmenistanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Turkmenistan => write!(f, "Turkmenistan"),
            Self::Ashgabat => write!(f, "Ashgabat"),
            Self::AhalRegion => write!(f, "Ahal Region"),
            Self::BalkanRegion => write!(f, "Balkan Region"),
            Self::DashoguzRegion => write!(f, "Dashoguz Region"),
            Self::LebapRegion => write!(f, "Lebap Region"),
            Self::MaryRegion => write!(f, "Mary Region"),
        }
    }
}

// Turks and Caicos Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurksAndCaicosIslandsSubregions {
    TurksAndCaicosIslands,
    TurksIslands,
    CaicosIslands,
}

impl TryFrom<u8> for TurksAndCaicosIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::TurksAndCaicosIslands),
            0x02 => Ok(Self::TurksIslands),
            0x03 => Ok(Self::CaicosIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TurksAndCaicosIslandsSubregions> for u8 {
    fn from(value: TurksAndCaicosIslandsSubregions) -> u8 {
        match value {
            TurksAndCaicosIslandsSubregions::TurksAndCaicosIslands => 0x01,
            TurksAndCaicosIslandsSubregions::TurksIslands => 0x02,
            TurksAndCaicosIslandsSubregions::CaicosIslands => 0x03,
        }
    }
}

impl Display for TurksAndCaicosIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TurksAndCaicosIslands => write!(f, "Turks and Caicos Islands"),
            Self::TurksIslands => write!(f, "Turks Islands"),
            Self::CaicosIslands => write!(f, "Caicos Islands"),
        }
    }
}

// Tuvalu Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TuvaluSubregions {
    Tuvalu,
    Funafuti,
    Nanumanga,
    Nanumea,
    Niulakita,
    Niutao,
    Nui,
    Nukufetau,
    Nukulaelae,
    Vaitupu,
}

impl TryFrom<u8> for TuvaluSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Tuvalu),
            0x02 => Ok(Self::Funafuti),
            0x03 => Ok(Self::Nanumanga),
            0x04 => Ok(Self::Nanumea),
            0x05 => Ok(Self::Niulakita),
            0x06 => Ok(Self::Niutao),
            0x07 => Ok(Self::Nui),
            0x08 => Ok(Self::Nukufetau),
            0x09 => Ok(Self::Nukulaelae),
            0x0A => Ok(Self::Vaitupu),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<TuvaluSubregions> for u8 {
    fn from(value: TuvaluSubregions) -> u8 {
        match value {
            TuvaluSubregions::Tuvalu => 0x01,
            TuvaluSubregions::Funafuti => 0x02,
            TuvaluSubregions::Nanumanga => 0x03,
            TuvaluSubregions::Nanumea => 0x04,
            TuvaluSubregions::Niulakita => 0x05,
            TuvaluSubregions::Niutao => 0x06,
            TuvaluSubregions::Nui => 0x07,
            TuvaluSubregions::Nukufetau => 0x08,
            TuvaluSubregions::Nukulaelae => 0x09,
            TuvaluSubregions::Vaitupu => 0x0A,
        }
    }
}

impl Display for TuvaluSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tuvalu => write!(f, "Tuvalu"),
            Self::Funafuti => write!(f, "Funafuti"),
            Self::Nanumanga => write!(f, "Nanumanga"),
            Self::Nanumea => write!(f, "Nanumea"),
            Self::Niulakita => write!(f, "Niulakita"),
            Self::Niutao => write!(f, "Niutao"),
            Self::Nui => write!(f, "Nui"),
            Self::Nukufetau => write!(f, "Nukufetau"),
            Self::Nukulaelae => write!(f, "Nukulaelae"),
            Self::Vaitupu => write!(f, "Vaitupu"),
        }
    }
}

// United Arab Emirates Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitedArabEmiratesSubregions {
    UnitedArabEmirates,
    AbuDhabi,
    Ajman,
    AshShariqah,
    RasAlKhaimah,
    Dubai,
    AlFujayrah,
    UmmAlQaywayn,
}

impl TryFrom<u8> for UnitedArabEmiratesSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::UnitedArabEmirates),
            0x02 => Ok(Self::AbuDhabi),
            0x03 => Ok(Self::Ajman),
            0x04 => Ok(Self::AshShariqah),
            0x05 => Ok(Self::RasAlKhaimah),
            0x06 => Ok(Self::Dubai),
            0x07 => Ok(Self::AlFujayrah),
            0x08 => Ok(Self::UmmAlQaywayn),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<UnitedArabEmiratesSubregions> for u8 {
    fn from(value: UnitedArabEmiratesSubregions) -> u8 {
        match value {
            UnitedArabEmiratesSubregions::UnitedArabEmirates => 0x01,
            UnitedArabEmiratesSubregions::AbuDhabi => 0x02,
            UnitedArabEmiratesSubregions::Ajman => 0x03,
            UnitedArabEmiratesSubregions::AshShariqah => 0x04,
            UnitedArabEmiratesSubregions::RasAlKhaimah => 0x05,
            UnitedArabEmiratesSubregions::Dubai => 0x06,
            UnitedArabEmiratesSubregions::AlFujayrah => 0x07,
            UnitedArabEmiratesSubregions::UmmAlQaywayn => 0x08,
        }
    }
}

impl Display for UnitedArabEmiratesSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnitedArabEmirates => write!(f, "United Arab Emirates"),
            Self::AbuDhabi => write!(f, "Abu Dhabi"),
            Self::Ajman => write!(f, "‘Ajman"),
            Self::AshShariqah => write!(f, "Ash Shariqah"),
            Self::RasAlKhaimah => write!(f, "Ras al-Khaimah"),
            Self::Dubai => write!(f, "Dubai"),
            Self::AlFujayrah => write!(f, "Al Fujayrah"),
            Self::UmmAlQaywayn => write!(f, "Umm al Qaywayn"),
        }
    }
}

// US Virgin Islands Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum USVirginIslandsSubregions {
    USVirginIslands,
}

impl TryFrom<u8> for USVirginIslandsSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::USVirginIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<USVirginIslandsSubregions> for u8 {
    fn from(value: USVirginIslandsSubregions) -> u8 {
        match value {
            USVirginIslandsSubregions::USVirginIslands => 0x01,
        }
    }
}

impl Display for USVirginIslandsSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::USVirginIslands => write!(f, "US Virgin Islands"),
        }
    }
}

// Uganda Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UgandaSubregions {
    Uganda,
    Central,
    Eastern,
    Northern,
    Western,
}

impl TryFrom<u8> for UgandaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Uganda),
            0x02 => Ok(Self::Central),
            0x03 => Ok(Self::Eastern),
            0x04 => Ok(Self::Northern),
            0x05 => Ok(Self::Western),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<UgandaSubregions> for u8 {
    fn from(value: UgandaSubregions) -> u8 {
        match value {
            UgandaSubregions::Uganda => 0x01,
            UgandaSubregions::Central => 0x02,
            UgandaSubregions::Eastern => 0x03,
            UgandaSubregions::Northern => 0x04,
            UgandaSubregions::Western => 0x05,
        }
    }
}

impl Display for UgandaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uganda => write!(f, "Uganda"),
            Self::Central => write!(f, "Central"),
            Self::Eastern => write!(f, "Eastern"),
            Self::Northern => write!(f, "Northern"),
            Self::Western => write!(f, "Western"),
        }
    }
}

// Ukraine Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UkraineSubregions {
    Ukraine,
    KievOblast,
    AutonomousRepublicOfCrimea,
    CherkasyOblast,
    ChernihivOblast,
    ChernivtsiOblast,
    DnipropetrovskOblast,
    DonetskOblast,
    IvanoFrankivskOblast,
    KharkivOblast,
    KhersonOblast,
    KhmelnytskyiOblast,
    KirovohradOblast,
    LuhanskOblast,
    LvivOblast,
    MykolaivOblast,
    OdessaOblast,
    PoltavaOblast,
    RivneOblast,
    Sevastopol,
    SumyOblast,
    TernopilOblast,
    VinnytsiaOblast,
    VolynOblast,
    ZakarpattiaOblast,
    ZaporizhiaOblast,
    ZhytomyrOblast,
}

impl TryFrom<u8> for UkraineSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Ukraine),
            0x02 => Ok(Self::KievOblast),
            0x03 => Ok(Self::AutonomousRepublicOfCrimea),
            0x04 => Ok(Self::CherkasyOblast),
            0x05 => Ok(Self::ChernihivOblast),
            0x06 => Ok(Self::ChernivtsiOblast),
            0x07 => Ok(Self::DnipropetrovskOblast),
            0x08 => Ok(Self::DonetskOblast),
            0x09 => Ok(Self::IvanoFrankivskOblast),
            0x0A => Ok(Self::KharkivOblast),
            0x0B => Ok(Self::KhersonOblast),
            0x0C => Ok(Self::KhmelnytskyiOblast),
            0x0D => Ok(Self::KirovohradOblast),
            0x0E => Ok(Self::LuhanskOblast),
            0x0F => Ok(Self::LvivOblast),
            0x10 => Ok(Self::MykolaivOblast),
            0x11 => Ok(Self::OdessaOblast),
            0x12 => Ok(Self::PoltavaOblast),
            0x13 => Ok(Self::RivneOblast),
            0x14 => Ok(Self::Sevastopol),
            0x15 => Ok(Self::SumyOblast),
            0x16 => Ok(Self::TernopilOblast),
            0x17 => Ok(Self::VinnytsiaOblast),
            0x18 => Ok(Self::VolynOblast),
            0x19 => Ok(Self::ZakarpattiaOblast),
            0x1A => Ok(Self::ZaporizhiaOblast),
            0x1B => Ok(Self::ZhytomyrOblast),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<UkraineSubregions> for u8 {
    fn from(value: UkraineSubregions) -> u8 {
        match value {
            UkraineSubregions::Ukraine => 0x01,
            UkraineSubregions::KievOblast => 0x02,
            UkraineSubregions::AutonomousRepublicOfCrimea => 0x03,
            UkraineSubregions::CherkasyOblast => 0x04,
            UkraineSubregions::ChernihivOblast => 0x05,
            UkraineSubregions::ChernivtsiOblast => 0x06,
            UkraineSubregions::DnipropetrovskOblast => 0x07,
            UkraineSubregions::DonetskOblast => 0x08,
            UkraineSubregions::IvanoFrankivskOblast => 0x09,
            UkraineSubregions::KharkivOblast => 0x0A,
            UkraineSubregions::KhersonOblast => 0x0B,
            UkraineSubregions::KhmelnytskyiOblast => 0x0C,
            UkraineSubregions::KirovohradOblast => 0x0D,
            UkraineSubregions::LuhanskOblast => 0x0E,
            UkraineSubregions::LvivOblast => 0x0F,
            UkraineSubregions::MykolaivOblast => 0x10,
            UkraineSubregions::OdessaOblast => 0x11,
            UkraineSubregions::PoltavaOblast => 0x12,
            UkraineSubregions::RivneOblast => 0x13,
            UkraineSubregions::Sevastopol => 0x14,
            UkraineSubregions::SumyOblast => 0x15,
            UkraineSubregions::TernopilOblast => 0x16,
            UkraineSubregions::VinnytsiaOblast => 0x17,
            UkraineSubregions::VolynOblast => 0x18,
            UkraineSubregions::ZakarpattiaOblast => 0x19,
            UkraineSubregions::ZaporizhiaOblast => 0x1A,
            UkraineSubregions::ZhytomyrOblast => 0x1B,
        }
    }
}

impl Display for UkraineSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ukraine => write!(f, "Ukraine"),
            Self::KievOblast => write!(f, "Kiev Oblast"),
            Self::AutonomousRepublicOfCrimea => write!(f, "Autonomous Republic of Crimea"),
            Self::CherkasyOblast => write!(f, "Cherkasy Oblast"),
            Self::ChernihivOblast => write!(f, "Chernihiv Oblast"),
            Self::ChernivtsiOblast => write!(f, "Chernivtsi Oblast"),
            Self::DnipropetrovskOblast => write!(f, "Dnipropetrovsk Oblast"),
            Self::DonetskOblast => write!(f, "Donetsk Oblast"),
            Self::IvanoFrankivskOblast => write!(f, "Ivano-Frankivsk Oblast"),
            Self::KharkivOblast => write!(f, "Kharkiv Oblast"),
            Self::KhersonOblast => write!(f, "Kherson Oblast"),
            Self::KhmelnytskyiOblast => write!(f, "Khmelnytskyi Oblast"),
            Self::KirovohradOblast => write!(f, "Kirovohrad Oblast"),
            Self::LuhanskOblast => write!(f, "Luhansk Oblast"),
            Self::LvivOblast => write!(f, "Lviv Oblast"),
            Self::MykolaivOblast => write!(f, "Mykolaiv Oblast"),
            Self::OdessaOblast => write!(f, "Odessa Oblast"),
            Self::PoltavaOblast => write!(f, "Poltava Oblast"),
            Self::RivneOblast => write!(f, "Rivne Oblast"),
            Self::Sevastopol => write!(f, "Sevastopol"),
            Self::SumyOblast => write!(f, "Sumy Oblast"),
            Self::TernopilOblast => write!(f, "Ternopil Oblast"),
            Self::VinnytsiaOblast => write!(f, "Vinnytsia Oblast"),
            Self::VolynOblast => write!(f, "Volyn Oblast"),
            Self::ZakarpattiaOblast => write!(f, "Zakarpattia Oblast"),
            Self::ZaporizhiaOblast => write!(f, "Zaporizhia Oblast"),
            Self::ZhytomyrOblast => write!(f, "Zhytomyr Oblast"),
        }
    }
}

// United Kingdom Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitedKingdomSubregions {
    UnitedKingdom,
    England,
    IsleOfMan,
    Scotland,
    Wales,
    NorthernIreland,
    GreaterLondon,
    EastMidlands,
    EastOfEngland,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    WestMidlands,
    YorkshireAndTheHumber,
    AkrotiriAndDhekelia,
    Anguilla,
    BailiwickOfGuernsey,
    BailiwickOfJersey,
    Bermuda,
    BritishAntarcticTerritory,
    BritishIndianOceanTerritory,
    BritishVirginIslands,
    CaymanIslands,
    FalklandIslands,
    Gibraltar,
    Montserrat,
    PitcairnIslands,
    SaintHelenaAscensionAndTristanDaCunha,
    SouthGeorgiaAndTheSouthSandwichIslands,
    TurksAndCaicosIslands,
}

impl TryFrom<u8> for UnitedKingdomSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::UnitedKingdom),
            0x02 => Ok(Self::England),
            0x03 => Ok(Self::IsleOfMan),
            0x04 => Ok(Self::Scotland),
            0x05 => Ok(Self::Wales),
            0x06 => Ok(Self::NorthernIreland),
            0x07 => Ok(Self::GreaterLondon),
            0x08 => Ok(Self::EastMidlands),
            0x09 => Ok(Self::EastOfEngland),
            0x0A => Ok(Self::NorthEast),
            0x0B => Ok(Self::NorthWest),
            0x0C => Ok(Self::SouthEast),
            0x0D => Ok(Self::SouthWest),
            0x0E => Ok(Self::WestMidlands),
            0x0F => Ok(Self::YorkshireAndTheHumber),
            0x10 => Ok(Self::AkrotiriAndDhekelia),
            0x11 => Ok(Self::Anguilla),
            0x12 => Ok(Self::BailiwickOfGuernsey),
            0x13 => Ok(Self::BailiwickOfJersey),
            0x14 => Ok(Self::Bermuda),
            0x15 => Ok(Self::BritishAntarcticTerritory),
            0x16 => Ok(Self::BritishIndianOceanTerritory),
            0x17 => Ok(Self::BritishVirginIslands),
            0x18 => Ok(Self::CaymanIslands),
            0x19 => Ok(Self::FalklandIslands),
            0x1A => Ok(Self::Gibraltar),
            0x1B => Ok(Self::Montserrat),
            0x1C => Ok(Self::PitcairnIslands),
            0x1D => Ok(Self::SaintHelenaAscensionAndTristanDaCunha),
            0x1E => Ok(Self::SouthGeorgiaAndTheSouthSandwichIslands),
            0x1F => Ok(Self::TurksAndCaicosIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<UnitedKingdomSubregions> for u8 {
    fn from(value: UnitedKingdomSubregions) -> u8 {
        match value {
            UnitedKingdomSubregions::UnitedKingdom => 0x01,
            UnitedKingdomSubregions::England => 0x02,
            UnitedKingdomSubregions::IsleOfMan => 0x03,
            UnitedKingdomSubregions::Scotland => 0x04,
            UnitedKingdomSubregions::Wales => 0x05,
            UnitedKingdomSubregions::NorthernIreland => 0x06,
            UnitedKingdomSubregions::GreaterLondon => 0x07,
            UnitedKingdomSubregions::EastMidlands => 0x08,
            UnitedKingdomSubregions::EastOfEngland => 0x09,
            UnitedKingdomSubregions::NorthEast => 0x0A,
            UnitedKingdomSubregions::NorthWest => 0x0B,
            UnitedKingdomSubregions::SouthEast => 0x0C,
            UnitedKingdomSubregions::SouthWest => 0x0D,
            UnitedKingdomSubregions::WestMidlands => 0x0E,
            UnitedKingdomSubregions::YorkshireAndTheHumber => 0x0F,
            UnitedKingdomSubregions::AkrotiriAndDhekelia => 0x10,
            UnitedKingdomSubregions::Anguilla => 0x11,
            UnitedKingdomSubregions::BailiwickOfGuernsey => 0x12,
            UnitedKingdomSubregions::BailiwickOfJersey => 0x13,
            UnitedKingdomSubregions::Bermuda => 0x14,
            UnitedKingdomSubregions::BritishAntarcticTerritory => 0x15,
            UnitedKingdomSubregions::BritishIndianOceanTerritory => 0x16,
            UnitedKingdomSubregions::BritishVirginIslands => 0x17,
            UnitedKingdomSubregions::CaymanIslands => 0x18,
            UnitedKingdomSubregions::FalklandIslands => 0x19,
            UnitedKingdomSubregions::Gibraltar => 0x1A,
            UnitedKingdomSubregions::Montserrat => 0x1B,
            UnitedKingdomSubregions::PitcairnIslands => 0x1C,
            UnitedKingdomSubregions::SaintHelenaAscensionAndTristanDaCunha => 0x1D,
            UnitedKingdomSubregions::SouthGeorgiaAndTheSouthSandwichIslands => 0x1E,
            UnitedKingdomSubregions::TurksAndCaicosIslands => 0x1F,
        }
    }
}

impl Display for UnitedKingdomSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnitedKingdom => write!(f, "United Kingdom"),
            Self::England => write!(f, "England"),
            Self::IsleOfMan => write!(f, "Isle of Man"),
            Self::Scotland => write!(f, "Scotland"),
            Self::Wales => write!(f, "Wales"),
            Self::NorthernIreland => write!(f, "Northern Ireland"),
            Self::GreaterLondon => write!(f, "Greater London"),
            Self::EastMidlands => write!(f, "East Midlands"),
            Self::EastOfEngland => write!(f, "East of England"),
            Self::NorthEast => write!(f, "North East"),
            Self::NorthWest => write!(f, "North West"),
            Self::SouthEast => write!(f, "South East"),
            Self::SouthWest => write!(f, "South West"),
            Self::WestMidlands => write!(f, "West Midlands"),
            Self::YorkshireAndTheHumber => write!(f, "Yorkshire and the Humber"),
            Self::AkrotiriAndDhekelia => write!(f, "Akrotiri and Dhekelia"),
            Self::Anguilla => write!(f, "Anguilla"),
            Self::BailiwickOfGuernsey => write!(f, "Bailiwick of Guernsey"),
            Self::BailiwickOfJersey => write!(f, "Bailiwick of Jersey"),
            Self::Bermuda => write!(f, "Bermuda"),
            Self::BritishAntarcticTerritory => write!(f, "British Antarctic Territory"),
            Self::BritishIndianOceanTerritory => write!(f, "British Indian Ocean Territory"),
            Self::BritishVirginIslands => write!(f, "British Virgin Islands"),
            Self::CaymanIslands => write!(f, "Cayman Islands"),
            Self::FalklandIslands => write!(f, "Falkland Islands"),
            Self::Gibraltar => write!(f, "Gibraltar"),
            Self::Montserrat => write!(f, "Montserrat"),
            Self::PitcairnIslands => write!(f, "Pitcairn Islands"),
            Self::SaintHelenaAscensionAndTristanDaCunha => {
                write!(f, "Saint Helena, Ascension and Tristan da Cunha")
            }
            Self::SouthGeorgiaAndTheSouthSandwichIslands => {
                write!(f, "South Georgia and the South Sandwich Islands")
            }
            Self::TurksAndCaicosIslands => write!(f, "Turks and Caicos Islands"),
        }
    }
}

// United States Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitedStatesSubregions {
    UnitedStates,
    DistrictOfColumbia,
    Alaska,
    Alabama,
    Arkansas,
    Arizona,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Iowa,
    Idaho,
    Illinois,
    Indiana,
    Kansas,
    Kentucky,
    Louisiana,
    Massachusetts,
    Maryland,
    Maine,
    Michigan,
    Minnesota,
    Missouri,
    Mississippi,
    Montana,
    NorthCarolina,
    NorthDakota,
    Nebraska,
    NewHampshire,
    NewJersey,
    NewMexico,
    Nevada,
    NewYork,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Virginia,
    Vermont,
    Washington,
    Wisconsin,
    WestVirginia,
    Wyoming,
    PuertoRico,
    AmericanSamoa,
    Guam,
    NorthernMarianaIslands,
    UnitedStatesMinorOutlyingIslands,
    USVirginIslands,
}

impl TryFrom<u8> for UnitedStatesSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::UnitedStates),
            0x02 => Ok(Self::DistrictOfColumbia),
            0x03 => Ok(Self::Alaska),
            0x04 => Ok(Self::Alabama),
            0x05 => Ok(Self::Arkansas),
            0x06 => Ok(Self::Arizona),
            0x07 => Ok(Self::California),
            0x08 => Ok(Self::Colorado),
            0x09 => Ok(Self::Connecticut),
            0x0A => Ok(Self::Delaware),
            0x0B => Ok(Self::Florida),
            0x0C => Ok(Self::Georgia),
            0x0D => Ok(Self::Hawaii),
            0x0E => Ok(Self::Iowa),
            0x0F => Ok(Self::Idaho),
            0x10 => Ok(Self::Illinois),
            0x11 => Ok(Self::Indiana),
            0x12 => Ok(Self::Kansas),
            0x13 => Ok(Self::Kentucky),
            0x14 => Ok(Self::Louisiana),
            0x15 => Ok(Self::Massachusetts),
            0x16 => Ok(Self::Maryland),
            0x17 => Ok(Self::Maine),
            0x18 => Ok(Self::Michigan),
            0x19 => Ok(Self::Minnesota),
            0x1A => Ok(Self::Missouri),
            0x1B => Ok(Self::Mississippi),
            0x1C => Ok(Self::Montana),
            0x1D => Ok(Self::NorthCarolina),
            0x1E => Ok(Self::NorthDakota),
            0x1F => Ok(Self::Nebraska),
            0x20 => Ok(Self::NewHampshire),
            0x21 => Ok(Self::NewJersey),
            0x22 => Ok(Self::NewMexico),
            0x23 => Ok(Self::Nevada),
            0x24 => Ok(Self::NewYork),
            0x25 => Ok(Self::Ohio),
            0x26 => Ok(Self::Oklahoma),
            0x27 => Ok(Self::Oregon),
            0x28 => Ok(Self::Pennsylvania),
            0x29 => Ok(Self::RhodeIsland),
            0x2A => Ok(Self::SouthCarolina),
            0x2B => Ok(Self::SouthDakota),
            0x2C => Ok(Self::Tennessee),
            0x2D => Ok(Self::Texas),
            0x2E => Ok(Self::Utah),
            0x2F => Ok(Self::Virginia),
            0x30 => Ok(Self::Vermont),
            0x31 => Ok(Self::Washington),
            0x32 => Ok(Self::Wisconsin),
            0x33 => Ok(Self::WestVirginia),
            0x34 => Ok(Self::Wyoming),
            0x35 => Ok(Self::PuertoRico),
            0x36 => Ok(Self::AmericanSamoa),
            0x37 => Ok(Self::Guam),
            0x38 => Ok(Self::NorthernMarianaIslands),
            0x39 => Ok(Self::UnitedStatesMinorOutlyingIslands),
            0x3A => Ok(Self::USVirginIslands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<UnitedStatesSubregions> for u8 {
    fn from(value: UnitedStatesSubregions) -> u8 {
        match value {
            UnitedStatesSubregions::UnitedStates => 0x01,
            UnitedStatesSubregions::DistrictOfColumbia => 0x02,
            UnitedStatesSubregions::Alaska => 0x03,
            UnitedStatesSubregions::Alabama => 0x04,
            UnitedStatesSubregions::Arkansas => 0x05,
            UnitedStatesSubregions::Arizona => 0x06,
            UnitedStatesSubregions::California => 0x07,
            UnitedStatesSubregions::Colorado => 0x08,
            UnitedStatesSubregions::Connecticut => 0x09,
            UnitedStatesSubregions::Delaware => 0x0A,
            UnitedStatesSubregions::Florida => 0x0B,
            UnitedStatesSubregions::Georgia => 0x0C,
            UnitedStatesSubregions::Hawaii => 0x0D,
            UnitedStatesSubregions::Iowa => 0x0E,
            UnitedStatesSubregions::Idaho => 0x0F,
            UnitedStatesSubregions::Illinois => 0x10,
            UnitedStatesSubregions::Indiana => 0x11,
            UnitedStatesSubregions::Kansas => 0x12,
            UnitedStatesSubregions::Kentucky => 0x13,
            UnitedStatesSubregions::Louisiana => 0x14,
            UnitedStatesSubregions::Massachusetts => 0x15,
            UnitedStatesSubregions::Maryland => 0x16,
            UnitedStatesSubregions::Maine => 0x17,
            UnitedStatesSubregions::Michigan => 0x18,
            UnitedStatesSubregions::Minnesota => 0x19,
            UnitedStatesSubregions::Missouri => 0x1A,
            UnitedStatesSubregions::Mississippi => 0x1B,
            UnitedStatesSubregions::Montana => 0x1C,
            UnitedStatesSubregions::NorthCarolina => 0x1D,
            UnitedStatesSubregions::NorthDakota => 0x1E,
            UnitedStatesSubregions::Nebraska => 0x1F,
            UnitedStatesSubregions::NewHampshire => 0x20,
            UnitedStatesSubregions::NewJersey => 0x21,
            UnitedStatesSubregions::NewMexico => 0x22,
            UnitedStatesSubregions::Nevada => 0x23,
            UnitedStatesSubregions::NewYork => 0x24,
            UnitedStatesSubregions::Ohio => 0x25,
            UnitedStatesSubregions::Oklahoma => 0x26,
            UnitedStatesSubregions::Oregon => 0x27,
            UnitedStatesSubregions::Pennsylvania => 0x28,
            UnitedStatesSubregions::RhodeIsland => 0x29,
            UnitedStatesSubregions::SouthCarolina => 0x2A,
            UnitedStatesSubregions::SouthDakota => 0x2B,
            UnitedStatesSubregions::Tennessee => 0x2C,
            UnitedStatesSubregions::Texas => 0x2D,
            UnitedStatesSubregions::Utah => 0x2E,
            UnitedStatesSubregions::Virginia => 0x2F,
            UnitedStatesSubregions::Vermont => 0x30,
            UnitedStatesSubregions::Washington => 0x31,
            UnitedStatesSubregions::Wisconsin => 0x32,
            UnitedStatesSubregions::WestVirginia => 0x33,
            UnitedStatesSubregions::Wyoming => 0x34,
            UnitedStatesSubregions::PuertoRico => 0x35,
            UnitedStatesSubregions::AmericanSamoa => 0x36,
            UnitedStatesSubregions::Guam => 0x37,
            UnitedStatesSubregions::NorthernMarianaIslands => 0x38,
            UnitedStatesSubregions::UnitedStatesMinorOutlyingIslands => 0x39,
            UnitedStatesSubregions::USVirginIslands => 0x3A,
        }
    }
}

impl Display for UnitedStatesSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnitedStates => write!(f, "United States"),
            Self::DistrictOfColumbia => write!(f, "District of Columbia"),
            Self::Alaska => write!(f, "Alaska"),
            Self::Alabama => write!(f, "Alabama"),
            Self::Arkansas => write!(f, "Arkansas"),
            Self::Arizona => write!(f, "Arizona"),
            Self::California => write!(f, "California"),
            Self::Colorado => write!(f, "Colorado"),
            Self::Connecticut => write!(f, "Connecticut"),
            Self::Delaware => write!(f, "Delaware"),
            Self::Florida => write!(f, "Florida"),
            Self::Georgia => write!(f, "Georgia"),
            Self::Hawaii => write!(f, "Hawaii"),
            Self::Iowa => write!(f, "Iowa"),
            Self::Idaho => write!(f, "Idaho"),
            Self::Illinois => write!(f, "Illinois"),
            Self::Indiana => write!(f, "Indiana"),
            Self::Kansas => write!(f, "Kansas"),
            Self::Kentucky => write!(f, "Kentucky"),
            Self::Louisiana => write!(f, "Louisiana"),
            Self::Massachusetts => write!(f, "Massachusetts"),
            Self::Maryland => write!(f, "Maryland"),
            Self::Maine => write!(f, "Maine"),
            Self::Michigan => write!(f, "Michigan"),
            Self::Minnesota => write!(f, "Minnesota"),
            Self::Missouri => write!(f, "Missouri"),
            Self::Mississippi => write!(f, "Mississippi"),
            Self::Montana => write!(f, "Montana"),
            Self::NorthCarolina => write!(f, "North Carolina"),
            Self::NorthDakota => write!(f, "North Dakota"),
            Self::Nebraska => write!(f, "Nebraska"),
            Self::NewHampshire => write!(f, "New Hampshire"),
            Self::NewJersey => write!(f, "New Jersey"),
            Self::NewMexico => write!(f, "New Mexico"),
            Self::Nevada => write!(f, "Nevada"),
            Self::NewYork => write!(f, "New York"),
            Self::Ohio => write!(f, "Ohio"),
            Self::Oklahoma => write!(f, "Oklahoma"),
            Self::Oregon => write!(f, "Oregon"),
            Self::Pennsylvania => write!(f, "Pennsylvania"),
            Self::RhodeIsland => write!(f, "Rhode Island"),
            Self::SouthCarolina => write!(f, "South Carolina"),
            Self::SouthDakota => write!(f, "South Dakota"),
            Self::Tennessee => write!(f, "Tennessee"),
            Self::Texas => write!(f, "Texas"),
            Self::Utah => write!(f, "Utah"),
            Self::Virginia => write!(f, "Virginia"),
            Self::Vermont => write!(f, "Vermont"),
            Self::Washington => write!(f, "Washington"),
            Self::Wisconsin => write!(f, "Wisconsin"),
            Self::WestVirginia => write!(f, "West Virginia"),
            Self::Wyoming => write!(f, "Wyoming"),
            Self::PuertoRico => write!(f, "Puerto Rico"),
            Self::AmericanSamoa => write!(f, "American Samoa"),
            Self::Guam => write!(f, "Guam"),
            Self::NorthernMarianaIslands => write!(f, "Northern Mariana Islands"),
            Self::UnitedStatesMinorOutlyingIslands => {
                write!(f, "United States Minor Outlying Islands")
            }
            Self::USVirginIslands => write!(f, "US Virgin Islands"),
        }
    }
}

// Uruguay Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UruguaySubregions {
    Uruguay,
    Montevideo,
    Artigas,
    Canelones,
    CerroLargo,
    Colonia,
    Durazno,
    Flores,
    Florida,
    Lavalleja,
    Maldonado,
    Paysandu,
    RioNegro,
    Rivera,
    Rocha,
    Salto,
    SanJose,
    Soriano,
    Tacuarembo,
    TreintaYTres,
}

impl TryFrom<u8> for UruguaySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Uruguay),
            0x02 => Ok(Self::Montevideo),
            0x03 => Ok(Self::Artigas),
            0x04 => Ok(Self::Canelones),
            0x05 => Ok(Self::CerroLargo),
            0x06 => Ok(Self::Colonia),
            0x07 => Ok(Self::Durazno),
            0x08 => Ok(Self::Flores),
            0x09 => Ok(Self::Florida),
            0x0A => Ok(Self::Lavalleja),
            0x0B => Ok(Self::Maldonado),
            0x0C => Ok(Self::Paysandu),
            0x0D => Ok(Self::RioNegro),
            0x0E => Ok(Self::Rivera),
            0x0F => Ok(Self::Rocha),
            0x10 => Ok(Self::Salto),
            0x11 => Ok(Self::SanJose),
            0x12 => Ok(Self::Soriano),
            0x13 => Ok(Self::Tacuarembo),
            0x14 => Ok(Self::TreintaYTres),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<UruguaySubregions> for u8 {
    fn from(value: UruguaySubregions) -> u8 {
        match value {
            UruguaySubregions::Uruguay => 0x01,
            UruguaySubregions::Montevideo => 0x02,
            UruguaySubregions::Artigas => 0x03,
            UruguaySubregions::Canelones => 0x04,
            UruguaySubregions::CerroLargo => 0x05,
            UruguaySubregions::Colonia => 0x06,
            UruguaySubregions::Durazno => 0x07,
            UruguaySubregions::Flores => 0x08,
            UruguaySubregions::Florida => 0x09,
            UruguaySubregions::Lavalleja => 0x0A,
            UruguaySubregions::Maldonado => 0x0B,
            UruguaySubregions::Paysandu => 0x0C,
            UruguaySubregions::RioNegro => 0x0D,
            UruguaySubregions::Rivera => 0x0E,
            UruguaySubregions::Rocha => 0x0F,
            UruguaySubregions::Salto => 0x10,
            UruguaySubregions::SanJose => 0x11,
            UruguaySubregions::Soriano => 0x12,
            UruguaySubregions::Tacuarembo => 0x13,
            UruguaySubregions::TreintaYTres => 0x14,
        }
    }
}

impl Display for UruguaySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uruguay => write!(f, "Uruguay"),
            Self::Montevideo => write!(f, "Montevideo"),
            Self::Artigas => write!(f, "Artigas"),
            Self::Canelones => write!(f, "Canelones"),
            Self::CerroLargo => write!(f, "Cerro Largo"),
            Self::Colonia => write!(f, "Colonia"),
            Self::Durazno => write!(f, "Durazno"),
            Self::Flores => write!(f, "Flores"),
            Self::Florida => write!(f, "Florida"),
            Self::Lavalleja => write!(f, "Lavalleja"),
            Self::Maldonado => write!(f, "Maldonado"),
            Self::Paysandu => write!(f, "Paysandú"),
            Self::RioNegro => write!(f, "Río Negro"),
            Self::Rivera => write!(f, "Rivera"),
            Self::Rocha => write!(f, "Rocha"),
            Self::Salto => write!(f, "Salto"),
            Self::SanJose => write!(f, "San José"),
            Self::Soriano => write!(f, "Soriano"),
            Self::Tacuarembo => write!(f, "Tacuarembó"),
            Self::TreintaYTres => write!(f, "Treinta y Tres"),
        }
    }
}

// Uzbekistan Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UzbekistanSubregions {
    Uzbekistan,
    TashkentRegion,
    AndijanRegion,
    BukharaRegion,
    FerganaRegion,
    JizzakhRegion,
    Karakalpakstan,
    NamanganRegion,
    NavoiyRegion,
    QashqadaryoRegion,
    SamarqandRegion,
    SirdaryoRegion,
    SurxondaryoRegion,
    XorazmRegion,
}

impl TryFrom<u8> for UzbekistanSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Uzbekistan),
            0x02 => Ok(Self::TashkentRegion),
            0x03 => Ok(Self::AndijanRegion),
            0x04 => Ok(Self::BukharaRegion),
            0x05 => Ok(Self::FerganaRegion),
            0x06 => Ok(Self::JizzakhRegion),
            0x07 => Ok(Self::Karakalpakstan),
            0x08 => Ok(Self::NamanganRegion),
            0x09 => Ok(Self::NavoiyRegion),
            0x0A => Ok(Self::QashqadaryoRegion),
            0x0B => Ok(Self::SamarqandRegion),
            0x0C => Ok(Self::SirdaryoRegion),
            0x0D => Ok(Self::SurxondaryoRegion),
            0x0E => Ok(Self::XorazmRegion),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<UzbekistanSubregions> for u8 {
    fn from(value: UzbekistanSubregions) -> u8 {
        match value {
            UzbekistanSubregions::Uzbekistan => 0x01,
            UzbekistanSubregions::TashkentRegion => 0x02,
            UzbekistanSubregions::AndijanRegion => 0x03,
            UzbekistanSubregions::BukharaRegion => 0x04,
            UzbekistanSubregions::FerganaRegion => 0x05,
            UzbekistanSubregions::JizzakhRegion => 0x06,
            UzbekistanSubregions::Karakalpakstan => 0x07,
            UzbekistanSubregions::NamanganRegion => 0x08,
            UzbekistanSubregions::NavoiyRegion => 0x09,
            UzbekistanSubregions::QashqadaryoRegion => 0x0A,
            UzbekistanSubregions::SamarqandRegion => 0x0B,
            UzbekistanSubregions::SirdaryoRegion => 0x0C,
            UzbekistanSubregions::SurxondaryoRegion => 0x0D,
            UzbekistanSubregions::XorazmRegion => 0x0E,
        }
    }
}

impl Display for UzbekistanSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uzbekistan => write!(f, "Uzbekistan"),
            Self::TashkentRegion => write!(f, "Tashkent Region"),
            Self::AndijanRegion => write!(f, "Andijan Region"),
            Self::BukharaRegion => write!(f, "Bukhara Region"),
            Self::FerganaRegion => write!(f, "Fergana Region"),
            Self::JizzakhRegion => write!(f, "Jizzakh Region"),
            Self::Karakalpakstan => write!(f, "Karakalpakstan"),
            Self::NamanganRegion => write!(f, "Namangan Region"),
            Self::NavoiyRegion => write!(f, "Navoiy Region"),
            Self::QashqadaryoRegion => write!(f, "Qashqadaryo Region"),
            Self::SamarqandRegion => write!(f, "Samarqand Region"),
            Self::SirdaryoRegion => write!(f, "Sirdaryo Region"),
            Self::SurxondaryoRegion => write!(f, "Surxondaryo Region"),
            Self::XorazmRegion => write!(f, "Xorazm Region"),
        }
    }
}

// Vanuatu Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VanuatuSubregions {
    Vanuatu,
    Shefa,
    Malampa,
    Penama,
    Sanma,
    Tafea,
    Torba,
}

impl TryFrom<u8> for VanuatuSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Vanuatu),
            0x02 => Ok(Self::Shefa),
            0x03 => Ok(Self::Malampa),
            0x04 => Ok(Self::Penama),
            0x05 => Ok(Self::Sanma),
            0x06 => Ok(Self::Tafea),
            0x07 => Ok(Self::Torba),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<VanuatuSubregions> for u8 {
    fn from(value: VanuatuSubregions) -> u8 {
        match value {
            VanuatuSubregions::Vanuatu => 0x01,
            VanuatuSubregions::Shefa => 0x02,
            VanuatuSubregions::Malampa => 0x03,
            VanuatuSubregions::Penama => 0x04,
            VanuatuSubregions::Sanma => 0x05,
            VanuatuSubregions::Tafea => 0x06,
            VanuatuSubregions::Torba => 0x07,
        }
    }
}

impl Display for VanuatuSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vanuatu => write!(f, "Vanuatu"),
            Self::Shefa => write!(f, "Shefa"),
            Self::Malampa => write!(f, "Malampa"),
            Self::Penama => write!(f, "Penama"),
            Self::Sanma => write!(f, "Sanma"),
            Self::Tafea => write!(f, "Tafea"),
            Self::Torba => write!(f, "Torba"),
        }
    }
}

// Vatican City Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VaticanCitySubregions {
    VaticanCity,
}

impl TryFrom<u8> for VaticanCitySubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::VaticanCity),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<VaticanCitySubregions> for u8 {
    fn from(value: VaticanCitySubregions) -> u8 {
        match value {
            VaticanCitySubregions::VaticanCity => 0x01,
        }
    }
}

impl Display for VaticanCitySubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VaticanCity => write!(f, "Vatican City"),
        }
    }
}

// Venezuela Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VenezuelaSubregions {
    Venezuela,
    DistritoFederal,
    Amazonas,
    Anzoategui,
    Apure,
    Aragua,
    Barinas,
    Bolivar,
    Carabobo,
    Cojedes,
    DeltaAmacuro,
    Falcon,
    Guarico,
    Lara,
    Merida,
    Miranda,
    Monagas,
    NuevaEsparta,
    Portuguesa,
    Sucre,
    Tachira,
    Trujillo,
    Yaracuy,
    Zulia,
    DependenciasFederales,
    Vargas,
}

impl TryFrom<u8> for VenezuelaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Venezuela),
            0x02 => Ok(Self::DistritoFederal),
            0x03 => Ok(Self::Amazonas),
            0x04 => Ok(Self::Anzoategui),
            0x05 => Ok(Self::Apure),
            0x06 => Ok(Self::Aragua),
            0x07 => Ok(Self::Barinas),
            0x08 => Ok(Self::Bolivar),
            0x09 => Ok(Self::Carabobo),
            0x0A => Ok(Self::Cojedes),
            0x0B => Ok(Self::DeltaAmacuro),
            0x0C => Ok(Self::Falcon),
            0x0D => Ok(Self::Guarico),
            0x0E => Ok(Self::Lara),
            0x0F => Ok(Self::Merida),
            0x10 => Ok(Self::Miranda),
            0x11 => Ok(Self::Monagas),
            0x12 => Ok(Self::NuevaEsparta),
            0x13 => Ok(Self::Portuguesa),
            0x14 => Ok(Self::Sucre),
            0x15 => Ok(Self::Tachira),
            0x16 => Ok(Self::Trujillo),
            0x17 => Ok(Self::Yaracuy),
            0x18 => Ok(Self::Zulia),
            0x19 => Ok(Self::DependenciasFederales),
            0x1A => Ok(Self::Vargas),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<VenezuelaSubregions> for u8 {
    fn from(value: VenezuelaSubregions) -> u8 {
        match value {
            VenezuelaSubregions::Venezuela => 0x01,
            VenezuelaSubregions::DistritoFederal => 0x02,
            VenezuelaSubregions::Amazonas => 0x03,
            VenezuelaSubregions::Anzoategui => 0x04,
            VenezuelaSubregions::Apure => 0x05,
            VenezuelaSubregions::Aragua => 0x06,
            VenezuelaSubregions::Barinas => 0x07,
            VenezuelaSubregions::Bolivar => 0x08,
            VenezuelaSubregions::Carabobo => 0x09,
            VenezuelaSubregions::Cojedes => 0x0A,
            VenezuelaSubregions::DeltaAmacuro => 0x0B,
            VenezuelaSubregions::Falcon => 0x0C,
            VenezuelaSubregions::Guarico => 0x0D,
            VenezuelaSubregions::Lara => 0x0E,
            VenezuelaSubregions::Merida => 0x0F,
            VenezuelaSubregions::Miranda => 0x10,
            VenezuelaSubregions::Monagas => 0x11,
            VenezuelaSubregions::NuevaEsparta => 0x12,
            VenezuelaSubregions::Portuguesa => 0x13,
            VenezuelaSubregions::Sucre => 0x14,
            VenezuelaSubregions::Tachira => 0x15,
            VenezuelaSubregions::Trujillo => 0x16,
            VenezuelaSubregions::Yaracuy => 0x17,
            VenezuelaSubregions::Zulia => 0x18,
            VenezuelaSubregions::DependenciasFederales => 0x19,
            VenezuelaSubregions::Vargas => 0x1A,
        }
    }
}

impl Display for VenezuelaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Venezuela => write!(f, "Venezuela"),
            Self::DistritoFederal => write!(f, "Distrito Federal"),
            Self::Amazonas => write!(f, "Amazonas"),
            Self::Anzoategui => write!(f, "Anzoátegui"),
            Self::Apure => write!(f, "Apure"),
            Self::Aragua => write!(f, "Aragua"),
            Self::Barinas => write!(f, "Barinas"),
            Self::Bolivar => write!(f, "Bolívar"),
            Self::Carabobo => write!(f, "Carabobo"),
            Self::Cojedes => write!(f, "Cojedes"),
            Self::DeltaAmacuro => write!(f, "Delta Amacuro"),
            Self::Falcon => write!(f, "Falcón"),
            Self::Guarico => write!(f, "Guárico"),
            Self::Lara => write!(f, "Lara"),
            Self::Merida => write!(f, "Mérida"),
            Self::Miranda => write!(f, "Miranda"),
            Self::Monagas => write!(f, "Monagas"),
            Self::NuevaEsparta => write!(f, "Nueva Esparta"),
            Self::Portuguesa => write!(f, "Portuguesa"),
            Self::Sucre => write!(f, "Sucre"),
            Self::Tachira => write!(f, "Táchira"),
            Self::Trujillo => write!(f, "Trujillo"),
            Self::Yaracuy => write!(f, "Yaracuy"),
            Self::Zulia => write!(f, "Zulia"),
            Self::DependenciasFederales => write!(f, "Dependencias Federales"),
            Self::Vargas => write!(f, "Vargas"),
        }
    }
}

// Vietnam Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VietnamSubregions {
    Vietnam,
    RedRiverDelta,
    CentralHighlands,
    MekongDelta,
    NorthCentralCoast,
    Northeast,
    Northwest,
    SouthCentralCoast,
    Southeast,
}

impl TryFrom<u8> for VietnamSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Vietnam),
            0x02 => Ok(Self::RedRiverDelta),
            0x03 => Ok(Self::CentralHighlands),
            0x04 => Ok(Self::MekongDelta),
            0x05 => Ok(Self::NorthCentralCoast),
            0x06 => Ok(Self::Northeast),
            0x07 => Ok(Self::Northwest),
            0x08 => Ok(Self::SouthCentralCoast),
            0x09 => Ok(Self::Southeast),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<VietnamSubregions> for u8 {
    fn from(value: VietnamSubregions) -> u8 {
        match value {
            VietnamSubregions::Vietnam => 0x01,
            VietnamSubregions::RedRiverDelta => 0x02,
            VietnamSubregions::CentralHighlands => 0x03,
            VietnamSubregions::MekongDelta => 0x04,
            VietnamSubregions::NorthCentralCoast => 0x05,
            VietnamSubregions::Northeast => 0x06,
            VietnamSubregions::Northwest => 0x07,
            VietnamSubregions::SouthCentralCoast => 0x08,
            VietnamSubregions::Southeast => 0x09,
        }
    }
}

impl Display for VietnamSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vietnam => write!(f, "Vietnam"),
            Self::RedRiverDelta => write!(f, "Red River Delta"),
            Self::CentralHighlands => write!(f, "Central Highlands"),
            Self::MekongDelta => write!(f, "Mekong Delta"),
            Self::NorthCentralCoast => write!(f, "North Central Coast"),
            Self::Northeast => write!(f, "Northeast"),
            Self::Northwest => write!(f, "Northwest"),
            Self::SouthCentralCoast => write!(f, "South Central Coast"),
            Self::Southeast => write!(f, "Southeast"),
        }
    }
}

// Wales Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WalesSubregions {
    Wales,
    Cardiff,
    Anglesey,
    BlaenauGwent,
    Bridgend,
    Caerphilly,
    Carmarthenshire,
    Ceredigion,
    Conwy,
    Denbighshire,
    Flintshire,
    Gwynedd,
    MerthyrTydfil,
    Monmouthshire,
    NeathPortTalbot,
    Newport,
    Pembrokeshire,
    Powys,
    RhonddaCynonTaf,
    Swansea,
    Torfaen,
    ValeOfGlamorgan,
    Wrexham,
}

impl TryFrom<u8> for WalesSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Wales),
            0x02 => Ok(Self::Cardiff),
            0x03 => Ok(Self::Anglesey),
            0x04 => Ok(Self::BlaenauGwent),
            0x05 => Ok(Self::Bridgend),
            0x06 => Ok(Self::Caerphilly),
            0x07 => Ok(Self::Carmarthenshire),
            0x08 => Ok(Self::Ceredigion),
            0x09 => Ok(Self::Conwy),
            0x0A => Ok(Self::Denbighshire),
            0x0B => Ok(Self::Flintshire),
            0x0C => Ok(Self::Gwynedd),
            0x0D => Ok(Self::MerthyrTydfil),
            0x0E => Ok(Self::Monmouthshire),
            0x0F => Ok(Self::NeathPortTalbot),
            0x10 => Ok(Self::Newport),
            0x11 => Ok(Self::Pembrokeshire),
            0x12 => Ok(Self::Powys),
            0x13 => Ok(Self::RhonddaCynonTaf),
            0x14 => Ok(Self::Swansea),
            0x15 => Ok(Self::Torfaen),
            0x16 => Ok(Self::ValeOfGlamorgan),
            0x17 => Ok(Self::Wrexham),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<WalesSubregions> for u8 {
    fn from(value: WalesSubregions) -> u8 {
        match value {
            WalesSubregions::Wales => 0x01,
            WalesSubregions::Cardiff => 0x02,
            WalesSubregions::Anglesey => 0x03,
            WalesSubregions::BlaenauGwent => 0x04,
            WalesSubregions::Bridgend => 0x05,
            WalesSubregions::Caerphilly => 0x06,
            WalesSubregions::Carmarthenshire => 0x07,
            WalesSubregions::Ceredigion => 0x08,
            WalesSubregions::Conwy => 0x09,
            WalesSubregions::Denbighshire => 0x0A,
            WalesSubregions::Flintshire => 0x0B,
            WalesSubregions::Gwynedd => 0x0C,
            WalesSubregions::MerthyrTydfil => 0x0D,
            WalesSubregions::Monmouthshire => 0x0E,
            WalesSubregions::NeathPortTalbot => 0x0F,
            WalesSubregions::Newport => 0x10,
            WalesSubregions::Pembrokeshire => 0x11,
            WalesSubregions::Powys => 0x12,
            WalesSubregions::RhonddaCynonTaf => 0x13,
            WalesSubregions::Swansea => 0x14,
            WalesSubregions::Torfaen => 0x15,
            WalesSubregions::ValeOfGlamorgan => 0x16,
            WalesSubregions::Wrexham => 0x17,
        }
    }
}

impl Display for WalesSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wales => write!(f, "Wales"),
            Self::Cardiff => write!(f, "Cardiff"),
            Self::Anglesey => write!(f, "Anglesey"),
            Self::BlaenauGwent => write!(f, "Blaenau Gwent"),
            Self::Bridgend => write!(f, "Bridgend"),
            Self::Caerphilly => write!(f, "Caerphilly"),
            Self::Carmarthenshire => write!(f, "Carmarthenshire"),
            Self::Ceredigion => write!(f, "Ceredigion"),
            Self::Conwy => write!(f, "Conwy"),
            Self::Denbighshire => write!(f, "Denbighshire"),
            Self::Flintshire => write!(f, "Flintshire"),
            Self::Gwynedd => write!(f, "Gwynedd"),
            Self::MerthyrTydfil => write!(f, "Merthyr Tydfil"),
            Self::Monmouthshire => write!(f, "Monmouthshire"),
            Self::NeathPortTalbot => write!(f, "Neath Port Talbot"),
            Self::Newport => write!(f, "Newport"),
            Self::Pembrokeshire => write!(f, "Pembrokeshire"),
            Self::Powys => write!(f, "Powys"),
            Self::RhonddaCynonTaf => write!(f, "Rhondda Cynon Taf"),
            Self::Swansea => write!(f, "Swansea"),
            Self::Torfaen => write!(f, "Torfaen"),
            Self::ValeOfGlamorgan => write!(f, "Vale of Glamorgan"),
            Self::Wrexham => write!(f, "Wrexham"),
        }
    }
}

// Wallis and Futuna Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WallisAndFutunaSubregions {
    WallisAndFutuna,
    Uvea,
    Alo,
    Sigave,
}

impl TryFrom<u8> for WallisAndFutunaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::WallisAndFutuna),
            0x02 => Ok(Self::Uvea),
            0x03 => Ok(Self::Alo),
            0x04 => Ok(Self::Sigave),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<WallisAndFutunaSubregions> for u8 {
    fn from(value: WallisAndFutunaSubregions) -> u8 {
        match value {
            WallisAndFutunaSubregions::WallisAndFutuna => 0x01,
            WallisAndFutunaSubregions::Uvea => 0x02,
            WallisAndFutunaSubregions::Alo => 0x03,
            WallisAndFutunaSubregions::Sigave => 0x04,
        }
    }
}

impl Display for WallisAndFutunaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WallisAndFutuna => write!(f, "Wallis and Futuna"),
            Self::Uvea => write!(f, "Uvea"),
            Self::Alo => write!(f, "Alo"),
            Self::Sigave => write!(f, "Sigave"),
        }
    }
}

// Yemen Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YemenSubregions {
    Yemen,
    SanaaGovernorate,
    AmranGovernorate,
    AbyanGovernorate,
    AdenGovernorate,
    AlBaydaGovernorate,
    AlHudaydahGovernorate,
    AlJawfGovernorate,
    AlMahrahGovernorate,
    AlMahwitGovernorate,
    DhaleGovernorate,
    DhamarGovernorate,
    HadhramautGovernorate,
    HajjahGovernorate,
    IbbGovernorate,
    LahijGovernorate,
    MaribGovernorate,
    RaymahGovernorate,
    SaadaGovernorate,
    ShabwahGovernorate,
    SocotraGovernorate,
    TaizGovernorate,
}

impl TryFrom<u8> for YemenSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Yemen),
            0x02 => Ok(Self::SanaaGovernorate),
            0x03 => Ok(Self::AmranGovernorate),
            0x04 => Ok(Self::AbyanGovernorate),
            0x05 => Ok(Self::AdenGovernorate),
            0x06 => Ok(Self::AlBaydaGovernorate),
            0x07 => Ok(Self::AlHudaydahGovernorate),
            0x08 => Ok(Self::AlJawfGovernorate),
            0x09 => Ok(Self::AlMahrahGovernorate),
            0x0A => Ok(Self::AlMahwitGovernorate),
            0x0B => Ok(Self::DhaleGovernorate),
            0x0C => Ok(Self::DhamarGovernorate),
            0x0D => Ok(Self::HadhramautGovernorate),
            0x0E => Ok(Self::HajjahGovernorate),
            0x0F => Ok(Self::IbbGovernorate),
            0x10 => Ok(Self::LahijGovernorate),
            0x11 => Ok(Self::MaribGovernorate),
            0x12 => Ok(Self::RaymahGovernorate),
            0x13 => Ok(Self::SaadaGovernorate),
            0x14 => Ok(Self::ShabwahGovernorate),
            0x15 => Ok(Self::SocotraGovernorate),
            0x16 => Ok(Self::TaizGovernorate),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<YemenSubregions> for u8 {
    fn from(value: YemenSubregions) -> u8 {
        match value {
            YemenSubregions::Yemen => 0x01,
            YemenSubregions::SanaaGovernorate => 0x02,
            YemenSubregions::AmranGovernorate => 0x03,
            YemenSubregions::AbyanGovernorate => 0x04,
            YemenSubregions::AdenGovernorate => 0x05,
            YemenSubregions::AlBaydaGovernorate => 0x06,
            YemenSubregions::AlHudaydahGovernorate => 0x07,
            YemenSubregions::AlJawfGovernorate => 0x08,
            YemenSubregions::AlMahrahGovernorate => 0x09,
            YemenSubregions::AlMahwitGovernorate => 0x0A,
            YemenSubregions::DhaleGovernorate => 0x0B,
            YemenSubregions::DhamarGovernorate => 0x0C,
            YemenSubregions::HadhramautGovernorate => 0x0D,
            YemenSubregions::HajjahGovernorate => 0x0E,
            YemenSubregions::IbbGovernorate => 0x0F,
            YemenSubregions::LahijGovernorate => 0x10,
            YemenSubregions::MaribGovernorate => 0x11,
            YemenSubregions::RaymahGovernorate => 0x12,
            YemenSubregions::SaadaGovernorate => 0x13,
            YemenSubregions::ShabwahGovernorate => 0x14,
            YemenSubregions::SocotraGovernorate => 0x15,
            YemenSubregions::TaizGovernorate => 0x16,
        }
    }
}

impl Display for YemenSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yemen => write!(f, "Yemen"),
            Self::SanaaGovernorate => write!(f, "Sanaa Governorate"),
            Self::AmranGovernorate => write!(f, "'Amran Governorate"),
            Self::AbyanGovernorate => write!(f, "Abyan Governorate"),
            Self::AdenGovernorate => write!(f, "Aden Governorate"),
            Self::AlBaydaGovernorate => write!(f, "Al Bayda Governorate"),
            Self::AlHudaydahGovernorate => write!(f, "Al Hudaydah Governorate"),
            Self::AlJawfGovernorate => write!(f, "Al Jawf Governorate"),
            Self::AlMahrahGovernorate => write!(f, "Al Mahrah Governorate"),
            Self::AlMahwitGovernorate => write!(f, "Al Mahwit Governorate"),
            Self::DhaleGovernorate => write!(f, "Dhale Governorate"),
            Self::DhamarGovernorate => write!(f, "Dhamar Governorate"),
            Self::HadhramautGovernorate => write!(f, "Hadhramaut Governorate"),
            Self::HajjahGovernorate => write!(f, "Hajjah Governorate"),
            Self::IbbGovernorate => write!(f, "Ibb Governorate"),
            Self::LahijGovernorate => write!(f, "Lahij Governorate"),
            Self::MaribGovernorate => write!(f, "Ma'rib Governorate"),
            Self::RaymahGovernorate => write!(f, "Raymah Governorate"),
            Self::SaadaGovernorate => write!(f, "Saada Governorate"),
            Self::ShabwahGovernorate => write!(f, "Shabwah Governorate"),
            Self::SocotraGovernorate => write!(f, "Socotra Governorate"),
            Self::TaizGovernorate => write!(f, "Taiz Governorate"),
        }
    }
}

// Zambia Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZambiaSubregions {
    Zambia,
    LusakaProvince,
    CentralProvince,
    CopperbeltProvince,
    EasternProvince,
    LuapulaProvince,
    NorthernProvince,
    NorthWesternProvince,
    SouthernProvince,
    WesternProvince,
}

impl TryFrom<u8> for ZambiaSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Zambia),
            0x02 => Ok(Self::LusakaProvince),
            0x03 => Ok(Self::CentralProvince),
            0x04 => Ok(Self::CopperbeltProvince),
            0x05 => Ok(Self::EasternProvince),
            0x06 => Ok(Self::LuapulaProvince),
            0x07 => Ok(Self::NorthernProvince),
            0x08 => Ok(Self::NorthWesternProvince),
            0x09 => Ok(Self::SouthernProvince),
            0x0A => Ok(Self::WesternProvince),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ZambiaSubregions> for u8 {
    fn from(value: ZambiaSubregions) -> u8 {
        match value {
            ZambiaSubregions::Zambia => 0x01,
            ZambiaSubregions::LusakaProvince => 0x02,
            ZambiaSubregions::CentralProvince => 0x03,
            ZambiaSubregions::CopperbeltProvince => 0x04,
            ZambiaSubregions::EasternProvince => 0x05,
            ZambiaSubregions::LuapulaProvince => 0x06,
            ZambiaSubregions::NorthernProvince => 0x07,
            ZambiaSubregions::NorthWesternProvince => 0x08,
            ZambiaSubregions::SouthernProvince => 0x09,
            ZambiaSubregions::WesternProvince => 0x0A,
        }
    }
}

impl Display for ZambiaSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zambia => write!(f, "Zambia"),
            Self::LusakaProvince => write!(f, "Lusaka Province"),
            Self::CentralProvince => write!(f, "Central Province"),
            Self::CopperbeltProvince => write!(f, "Copperbelt Province"),
            Self::EasternProvince => write!(f, "Eastern Province"),
            Self::LuapulaProvince => write!(f, "Luapula Province"),
            Self::NorthernProvince => write!(f, "Northern Province"),
            Self::NorthWesternProvince => write!(f, "North-Western Province"),
            Self::SouthernProvince => write!(f, "Southern Province"),
            Self::WesternProvince => write!(f, "Western Province"),
        }
    }
}

// Zimbabwe Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZimbabweSubregions {
    Zimbabwe,
    Harare,
    Bulawayo,
    Manicaland,
    MashonalandCentral,
    MashonalandEast,
    MashonalandWest,
    Masvingo,
    MatabelelandNorth,
    MatabelelandSouth,
    Midlands,
}

impl TryFrom<u8> for ZimbabweSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Zimbabwe),
            0x02 => Ok(Self::Harare),
            0x03 => Ok(Self::Bulawayo),
            0x04 => Ok(Self::Manicaland),
            0x05 => Ok(Self::MashonalandCentral),
            0x06 => Ok(Self::MashonalandEast),
            0x07 => Ok(Self::MashonalandWest),
            0x08 => Ok(Self::Masvingo),
            0x09 => Ok(Self::MatabelelandNorth),
            0x0A => Ok(Self::MatabelelandSouth),
            0x0B => Ok(Self::Midlands),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<ZimbabweSubregions> for u8 {
    fn from(value: ZimbabweSubregions) -> u8 {
        match value {
            ZimbabweSubregions::Zimbabwe => 0x01,
            ZimbabweSubregions::Harare => 0x02,
            ZimbabweSubregions::Bulawayo => 0x03,
            ZimbabweSubregions::Manicaland => 0x04,
            ZimbabweSubregions::MashonalandCentral => 0x05,
            ZimbabweSubregions::MashonalandEast => 0x06,
            ZimbabweSubregions::MashonalandWest => 0x07,
            ZimbabweSubregions::Masvingo => 0x08,
            ZimbabweSubregions::MatabelelandNorth => 0x09,
            ZimbabweSubregions::MatabelelandSouth => 0x0A,
            ZimbabweSubregions::Midlands => 0x0B,
        }
    }
}

impl Display for ZimbabweSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zimbabwe => write!(f, "Zimbabwe"),
            Self::Harare => write!(f, "Harare"),
            Self::Bulawayo => write!(f, "Bulawayo"),
            Self::Manicaland => write!(f, "Manicaland"),
            Self::MashonalandCentral => write!(f, "Mashonaland Central"),
            Self::MashonalandEast => write!(f, "Mashonaland East"),
            Self::MashonalandWest => write!(f, "Mashonaland West"),
            Self::Masvingo => write!(f, "Masvingo"),
            Self::MatabelelandNorth => write!(f, "Matabeleland North"),
            Self::MatabelelandSouth => write!(f, "Matabeleland South"),
            Self::Midlands => write!(f, "Midlands"),
        }
    }
}

// Åland Subregions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlandSubregions {
    Aland,
}

impl TryFrom<u8> for AlandSubregions {
    type Error = SubregionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Aland),
            _ => Err(SubregionError::NonexistentSubregion),
        }
    }
}

impl From<AlandSubregions> for u8 {
    fn from(value: AlandSubregions) -> u8 {
        match value {
            AlandSubregions::Aland => 0x01,
        }
    }
}

impl Display for AlandSubregions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aland => write!(f, "Åland"),
        }
    }
}
