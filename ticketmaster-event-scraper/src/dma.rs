use std::collections::HashMap;

pub fn get_hashmap() -> HashMap<&'static str, &'static str> {
    let mut city_to_dma = HashMap::new();
    city_to_dma.insert("All of US", "200");
    city_to_dma.insert("Abilene - Sweetwater", "212");
    city_to_dma.insert("Albany - Schenectady - Troy", "213");
    city_to_dma.insert("Albany, GA", "214");
    city_to_dma.insert("Albuquerque - Santa Fe", "215");
    city_to_dma.insert("Alexandria, LA", "216");
    city_to_dma.insert("Alpena", "217");
    city_to_dma.insert("Amarillo", "218");
    city_to_dma.insert("Anchorage", "219");
    city_to_dma.insert("Atlanta", "220");
    city_to_dma.insert("Augusta", "221");
    city_to_dma.insert("Austin", "222");
    city_to_dma.insert("Bakersfield", "223");
    city_to_dma.insert("Baltimore", "224");
    city_to_dma.insert("Bangor", "225");
    city_to_dma.insert("Baton Rouge", "226");
    city_to_dma.insert("Beaumont - Port Arthur", "227");
    city_to_dma.insert("Bend, OR", "228");
    city_to_dma.insert("Billings", "229");
    city_to_dma.insert("Biloxi - Gulfport", "230");
    city_to_dma.insert("Binghamton", "231");
    city_to_dma.insert("Birmingham (Anniston and Tuscaloosa)", "232");
    city_to_dma.insert("Bluefield - Beckley - Oak Hill", "233");
    city_to_dma.insert("Boise", "234");
    city_to_dma.insert("Boston (Manchester)", "235");
    city_to_dma.insert("Bowling Green", "236");
    city_to_dma.insert("Buffalo", "237");
    city_to_dma.insert("Burlington - Plattsburgh", "238");
    city_to_dma.insert("Butte - Bozeman", "239");
    city_to_dma.insert("Casper - Riverton", "240");
    city_to_dma.insert("Cedar Rapids - Waterloo & Dubuque", "241");
    city_to_dma.insert("Champaign & Springfield - Decatur", "242");
    city_to_dma.insert("Charleston, SC", "243");
    city_to_dma.insert("Charleston-Huntington", "244");
    city_to_dma.insert("Charlotte", "245");
    city_to_dma.insert("Charlottesville", "246");
    city_to_dma.insert("Chattanooga", "247");
    city_to_dma.insert("Cheyenne - Scottsbluff", "248");
    city_to_dma.insert("Chicago", "249");
    city_to_dma.insert("Chico - Redding", "250");
    city_to_dma.insert("Cincinnati", "251");
    city_to_dma.insert("Clarksburg - Weston", "252");
    city_to_dma.insert("Cleveland", "253");
    city_to_dma.insert("Colorado Springs - Pueblo", "254");
    city_to_dma.insert("Columbia - Jefferson City", "255");
    city_to_dma.insert("Columbia, SC", "256");
    city_to_dma.insert("Columbus - Tupelo - West Point", "257");
    city_to_dma.insert("Columbus, GA", "258");
    city_to_dma.insert("Columbus, OH", "259");
    city_to_dma.insert("Corpus Christi", "260");
    city_to_dma.insert("Dallas - Fort Worth", "261");
    city_to_dma.insert("Davenport - Rock Island - Moline", "262");
    city_to_dma.insert("Dayton", "263");
    city_to_dma.insert("Denver", "264");
    city_to_dma.insert("Des Moines - Ames", "265");
    city_to_dma.insert("Detroit", "266");
    city_to_dma.insert("Dothan", "267");
    city_to_dma.insert("Duluth - Superior", "268");
    city_to_dma.insert("El Paso", "269");
    city_to_dma.insert("Elmira", "270");
    city_to_dma.insert("Erie", "271");
    city_to_dma.insert("Eugene", "272");
    city_to_dma.insert("Eureka", "273");
    city_to_dma.insert("Evansville", "274");
    city_to_dma.insert("Fairbanks", "275");
    city_to_dma.insert("Fargo - Valley City", "276");
    city_to_dma.insert("Flint - Saginaw - Bay City", "277");
    city_to_dma.insert("Florence - Myrtle Beach", "278");
    city_to_dma.insert("Fort Myers - Naples", "279");
    city_to_dma.insert("Fort Smith - Fayetteville - Springdale - Rogers", "280");
    city_to_dma.insert("Fort Wayne", "281");
    city_to_dma.insert("Fresno - Visalia", "282");
    city_to_dma.insert("Gainesville", "283");
    city_to_dma.insert("Glendive", "284");
    city_to_dma.insert("Grand Junction - Montrose", "285");
    city_to_dma.insert("Grand Rapids - Kalamazoo - Battle Creek", "286");
    city_to_dma.insert("Great Falls", "287");
    city_to_dma.insert("Green Bay - Appleton", "288");
    city_to_dma.insert("Greensboro - High Point - Winston-Salem", "289");
    city_to_dma.insert("Greenville - New Bern - Washington", "290");
    city_to_dma.insert("Greenville - Spartansburg - Asheville - Anderson", "291");
    city_to_dma.insert("Greenwood - Greenville", "292");
    city_to_dma.insert("Harlingen - Weslaco - Brownsville - McAllen", "293");
    city_to_dma.insert("Harrisburg - Lancaster - Lebanon - York", "294");
    city_to_dma.insert("Harrisonburg", "295");
    city_to_dma.insert("Hartford & New Haven", "296");
    city_to_dma.insert("Hattiesburg - Laurel", "297");
    city_to_dma.insert("Helena", "298");
    city_to_dma.insert("Honolulu", "299");
    city_to_dma.insert("Houston", "300");
    city_to_dma.insert("Huntsville - Decatur (Florence)", "301");
    city_to_dma.insert("Idaho Falls - Pocatello", "302");
    city_to_dma.insert("Indianapolis", "303");
    city_to_dma.insert("Jackson, MS", "304");
    city_to_dma.insert("Jackson, TN", "305");
    city_to_dma.insert("Jacksonville", "306");
    city_to_dma.insert("Johnstown - Altoona", "307");
    city_to_dma.insert("Jonesboro", "308");
    city_to_dma.insert("Joplin - Pittsburg", "309");
    city_to_dma.insert("Juneau", "310");
    city_to_dma.insert("Kansas City", "311");
    city_to_dma.insert("Knoxville", "312");
    city_to_dma.insert("La Crosse - Eau Claire", "313");
    city_to_dma.insert("Lafayette, IN", "314");
    city_to_dma.insert("Lafayette, LA", "315");
    city_to_dma.insert("Lake Charles", "316");
    city_to_dma.insert("Lansing", "317");
    city_to_dma.insert("Laredo", "318");
    city_to_dma.insert("Las Vegas", "319");
    city_to_dma.insert("Lexington", "320");
    city_to_dma.insert("Lima", "321");
    city_to_dma.insert("Lincoln & Hastings - Kearney", "322");
    city_to_dma.insert("Little Rock - Pine Bluff", "323");
    city_to_dma.insert("Los Angeles", "324");
    city_to_dma.insert("Louisville", "325");
    city_to_dma.insert("Lubbock", "326");
    city_to_dma.insert("Macon", "327");
    city_to_dma.insert("Madison", "328");
    city_to_dma.insert("Mankato", "329");
    city_to_dma.insert("Marquette", "330");
    city_to_dma.insert("Medford - Klamath Falls", "331");
    city_to_dma.insert("Memphis", "332");
    city_to_dma.insert("Meridian", "333");
    city_to_dma.insert("Miami - Fort Lauderdale", "334");
    city_to_dma.insert("Milwaukee", "335");
    city_to_dma.insert("Minneapolis - Saint Paul", "336");
    city_to_dma.insert("Minot - Bismarck - Dickinson", "337");
    city_to_dma.insert("Missoula", "338");
    city_to_dma.insert("Mobile - Pensacola (Fort Walton Beach)", "339");
    city_to_dma.insert("Monroe - El Dorado", "340");
    city_to_dma.insert("Monterey - Salinas", "341");
    city_to_dma.insert("Montgomery (Selma)", "342");
    city_to_dma.insert("Nashville", "343");
    city_to_dma.insert("New Orleans", "344");
    city_to_dma.insert("New York", "345");
    city_to_dma.insert("Norfolk - Portsmouth - Newport News", "346");
    city_to_dma.insert("North Platte", "347");
    city_to_dma.insert("Odessa - Midland", "348");
    city_to_dma.insert("Oklahoma City", "349");
    city_to_dma.insert("Omaha", "350");
    city_to_dma.insert("Orlando - Daytona Beach - Melbourne", "351");
    city_to_dma.insert("Ottumwa - Kirksville", "352");
    city_to_dma.insert("Paducah - Cape Girardeau - Harrisburg - Mt Vernon", "353");
    city_to_dma.insert("Palm Springs", "354");
    city_to_dma.insert("Panama City", "355");
    city_to_dma.insert("Parkersburg", "356");
    city_to_dma.insert("Peoria - Bloomington", "357");
    city_to_dma.insert("Philadelphia", "358");
    city_to_dma.insert("Phoenix", "359");
    city_to_dma.insert("Pittsburgh", "360");
    city_to_dma.insert("Portland - Auburn", "361");
    city_to_dma.insert("Portland, OR", "362");
    city_to_dma.insert("Presque Isle", "363");
    city_to_dma.insert("Providence - New Bedford", "364");
    city_to_dma.insert("Quincy - Hannibal - Keokuk", "365");
    city_to_dma.insert("Raleigh - Durham (Fayetteville)", "366");
    city_to_dma.insert("Rapid City", "367");
    city_to_dma.insert("Reno", "368");
    city_to_dma.insert("Richmond - Petersburg", "369");
    city_to_dma.insert("Roanoke - Lynchburg", "370");
    city_to_dma.insert("Rochester - Mason City - Austin", "371");
    city_to_dma.insert("Rochester, NY", "372");
    city_to_dma.insert("Rockford", "373");
    city_to_dma.insert("Sacramento - Stockton - Modesto", "374");
    city_to_dma.insert("Saint Joseph", "375");
    city_to_dma.insert("Saint Louis", "376");
    city_to_dma.insert("Salisbury", "377");
    city_to_dma.insert("Salt Lake City", "378");
    city_to_dma.insert("San Angelo", "379");
    city_to_dma.insert("San Antonio", "380");
    city_to_dma.insert("San Diego", "381");
    city_to_dma.insert("San Francisco - Oakland - San Jose", "382");
    city_to_dma.insert("Santa Barbara - Santa Maria - San Luis Obispo", "383");
    city_to_dma.insert("Savannah", "384");
    city_to_dma.insert("Seattle - Tacoma", "385");
    city_to_dma.insert("Sherman - Ada", "386");
    city_to_dma.insert("Shreveport", "387");
    city_to_dma.insert("Sioux City", "388");
    city_to_dma.insert("Sioux Falls (Mitchell)", "389");
    city_to_dma.insert("South Bend - Elkhart", "390");
    city_to_dma.insert("Spokane", "391");
    city_to_dma.insert("Springfield - Holyoke", "392");
    city_to_dma.insert("Springfield, MO", "393");
    city_to_dma.insert("Syracuse", "394");
    city_to_dma.insert("Tallahassee - Thomasville", "395");
    city_to_dma.insert("Tampa - Saint Petersburg (Sarasota)", "396");
    city_to_dma.insert("Terre Haute", "397");
    city_to_dma.insert("Toledo", "398");
    city_to_dma.insert("Topeka", "399");
    city_to_dma.insert("Traverse City - Cadillac", "400");
    city_to_dma.insert("Tri-Cities, TN-VA", "401");
    city_to_dma.insert("Tucson (Sierra Vista)", "402");
    city_to_dma.insert("Tulsa", "403");
    city_to_dma.insert("Twin Falls", "404");
    city_to_dma.insert("Tyler - Longview (Lufkin & Nacogdoches)", "405");
    city_to_dma.insert("Utica", "406");
    city_to_dma.insert("Victoria", "407");
    city_to_dma.insert("Waco - Temple - Bryan", "408");
    city_to_dma.insert("Washington DC (Hagerstown)", "409");
    city_to_dma.insert("Watertown", "410");
    city_to_dma.insert("Wausau - Rhinelander", "411");
    city_to_dma.insert("West Palm Beach - Fort Pierce", "412");
    city_to_dma.insert("Wheeling - Steubenville", "413");
    city_to_dma.insert("Wichita - Hutchinson", "414");
    city_to_dma.insert("Wichita Falls & Lawton", "415");
    city_to_dma.insert("Wilkes Barre - Scranton", "416");
    city_to_dma.insert("Wilmington", "417");
    city_to_dma.insert("Yakima - Pasco - Richland - Kennewick", "418");
    city_to_dma.insert("Youngstown", "419");
    city_to_dma.insert("Yuma - El Centro", "420");
    city_to_dma.insert("Zanesville", "421");
    city_to_dma.insert("Northern New Jersey", "422");
    city_to_dma.insert("All of Canada", "500");
    
    return city_to_dma;
}