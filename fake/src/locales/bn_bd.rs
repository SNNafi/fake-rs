use crate::{faker::impls::address::CityNameGenFn, locales::Data};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct BN_BD;

impl Data for BN_BD {
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        "মোহাম্মদ",
        "আবদুল",
        "ফাতিমা",
        "আয়েশা",
        "নাসরিন",
        "শর্মিন",
        "তানভীর",
        "সোহেল",
        "রহিম",
        "করিম",
        "জাহিদ",
        "সাবিনা",
        "ইমরান",
        "নাজমুল",
        "শাহানা",
        "রিয়াদ",
        "তানিয়া",
        "রাজিব",
        "সুমাইয়া",
        "আরিফ",
        "নিলুফার",
        "সাকিব",
        "মাহমুদ",
        "রুমানা",
        "জুনায়েদ",
        "শাহনাজ",
        "ফারুক",
        "নাজনীন",
        "রাশেদ",
        "সালমা",
        "কামরুল",
        "লতিফা",
        "হাসান",
        "রুবিনা",
        "জাহাঙ্গীর",
        "শাহানা",
        "মনির",
        "নিলু",
        "রিয়াজ",
        "সাবিন",
    ];

    const NAME_LAST_NAME: &'static [&'static str] = &[
        "ইসলাম",
        "হোসেন",
        "আহমেদ",
        "খান",
        "হাসান",
        "রহমান",
        "আক্তার",
        "খাতুন",
        "বেগম",
        "উদ্দিন",
        "আলী",
        "আলম",
        "মিয়া",
        "সুলতানা",
        "চৌধুরী",
        "মণ্ডল",
        "সরদার",
        "শেখ",
        "পাটোয়ারী",
        "মল্লিক",
        "দাস",
        "রায়",
        "সিকদার",
        "মজুমদার",
        "কর",
    ];

    const NAME_TITLE: &'static [&'static str] = &["জনাব", "ডাঃ", "প্রফেসর", "ইঞ্জিনিয়ার", "অ্যাডভোকেট"];

    const NAME_SUFFIX: &'static [&'static str] = &["জুনিয়র", "সিনিয়র"];

    const NAME_TPL: &'static str = "{FirstName} {LastName}";
    const NAME_WITH_TITLE_TPL: &'static str = "{Title} {FirstName} {LastName}";

    // Number (Bengali digits ০-৯)
    const NUMBER_DIGIT: &'static [&'static str] = &[
        "০", "১", "২", "৩", "৪", "৫", "৬", "৭", "৮", "৯",
    ];

    // Job (Bengali)
    const JOB_SENIORITY: &'static [&'static str] = &[
        "সিনিয়র", "জুনিয়র", "লিড", "প্রিন্সিপাল", "চিফ", "রিজিওনাল", "ন্যাশনাল", "গ্লোবাল",
    ];
    const JOB_FIELD: &'static [&'static str] = &[
        "মার্কেটিং", "আইটি", "অ্যাকাউন্টিং", "প্রশাসন", "ব্যাংকিং", "শিক্ষা", "স্বাস্থ্য",
        "আইন", "উৎপাদন", "বিক্রয়", "প্রযুক্তি", "পরামর্শদান",
    ];
    const JOB_POSITION: &'static [&'static str] = &[
        "সুপারভাইজার", "ম্যানেজার", "ইঞ্জিনিয়ার", "বিশেষজ্ঞ", "পরিচালক", "সমন্বয়ক",
        "বিশ্লেষক", "ডেভেলপার", "পরামর্শদাতা", "প্রতিনিধি",
    ];
    const JOB_TITLE_TPL: &'static str = "{Seniority} {Field} {Position}";

    const LOREM_WORD: &'static [&'static str] = &[
        "এক", "দুই", "তিন", "চার", "পাঁচ", "ছয়", "সাত", "আট", "নয়", "দশ",
        "মানুষ", "জল", "আগুন", "মাটি", "আকাশ", "সূর্য", "চাঁদ", "নদী", "বন", "ঘর",
        "ভালো", "খারাপ", "বড়", "ছোট", "নতুন", "পুরানো", "সুন্দর", "কাজ", "দিন", "রাত",
        "বাংলা", "দেশ", "শহর", "গ্রাম", "পথ", "বই", "কথা", "মন", "প্রাণ", "আলো",
    ];

    const ADDRESS_CITY_PREFIX: &'static [&'static str] = &["উত্তর", "দক্ষিণ", "পূর্ব", "পশ্চিম", "নতুন", "পুরান"];

    const ADDRESS_CITY_SUFFIX: &'static [&'static str] = &["পুর", "নগর", "বাজার", "উপজেলা", "থানা"];

    const ADDRESS_CITY_TPL: &'static str = "{CityName} {CitySuffix}";
    const ADDRESS_CITY_WITH_PREFIX_TPL: &'static str = "{CityPrefix} {CityName} {CitySuffix}";

    const ADDRESS_COUNTRY: &'static [&'static str] = &["বাংলাদেশ"];

    const ADDRESS_COUNTRY_CODE: &'static [&'static str] = &["BD"];

    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = &[
        "রোড", "সড়ক", "লেন", "গলি", "এভিনিউ", "মহাসড়ক", "ব্রিজ", "চত্বর",
    ];

    const ADDRESS_STREET_TPL: &'static str = "{StreetName} {StreetSuffix}";

    const ADDRESS_SECONDARY_ADDR_TYPE: &'static [&'static str] = &["ফ্ল্যাট", "সুইট"];

    const ADDRESS_TIME_ZONE: &'static [&'static str] = &["Asia/Dhaka"];

    const ADDRESS_STATE: &'static [&'static str] = &[
        "ঢাকা",
        "চট্টগ্রাম",
        "রাজশাহী",
        "খুলনা",
        "বরিশাল",
        "সিলেট",
        "রংপুর",
        "ময়মনসিংহ",
    ];

    const ADDRESS_STATE_ABBR: &'static [&'static str] = &["A", "B", "C", "D", "E", "F", "G", "H"];

    const ADDRESS_BUILDING_NUMBER_FORMATS: &'static [&'static str] = &["##", "###", "#"];

    const ADDRESS_ZIP_FORMATS: &'static [&'static str] = &["####"];

    const ADDRESS_POSTCODE_FORMATS: &'static [&'static str] = &["####"];

    const PHONE_NUMBER_FORMATS: &'static [&'static str] = &[
        "+880 2 #### ####",
        "+880 2# #### ####",
    ];

    const PHONE_CELL_NUMBER_FORMATS: &'static [&'static str] = &[
        "+880 1### #### ##",
        "+880 1### ### ###",
    ];

    const CHRONO_DEFAULT_TIME_FORMAT: &'static str = "%H:%M:%S";
    const CHRONO_DEFAULT_DATE_FORMAT: &'static str = "%d-%m-%Y";
    const CHRONO_DEFAULT_DATETIME_FORMAT: &'static str = "%d-%m-%Y %H:%M:%S";

    const TIME_DEFAULT_DATE_FORMAT: &'static str = "[day]-[month]-[year]";
    const TIME_DEFAULT_DATETIME_FORMAT: &'static str =
        "[day]-[month]-[year] [hour]:[minute]:[second]";

    // Commerce (Bengali)
    const COMMERCE_COLOR: &'static [&'static str] = &[
        "কালো", "সাদা", "লাল", "সবুজ", "নীল", "হলুদ", "বেগুনি", "গোলাপি", "বাদামি", "ধূসর",
        "রূপালি", "কমলা", "টিল", "চুনি", "সোনালি", "ক্রিম", "আবলুস",
    ];

    const COMMERCE_DEPARTMENT: &'static [&'static str] = &[
        "বই", "সিনেমা", "গান", "গেম", "ইলেকট্রনিক্স", "কম্পিউটার", "ঘর", "বাগান", "গৃহসজ্জা",
        "কম্পিউটার অ্যাকসেসরিজ", "পোশাক", "জুতা", "খেলনা", "স্বাস্থ্য", "সৌন্দর্য", "অফিস",
    ];

    const COMMERCE_PRODUCT_ADJECTIVE: &'static [&'static str] = &[
        "ছোট", "আরামদায়ক", "টেকসই", "আধুনিক", "সুন্দর", "চমৎকার", "ব্যবহারিক", "মজবুত",
        "হালকা", "কম্প্যাক্ট", "প্রিমিয়াম", "নতুন", "বেশি বিক্রিত",
    ];

    const COMMERCE_PRODUCT_MATERIAL: &'static [&'static str] = &[
        "স্টিল", "কাঠ", "প্লাস্টিক", "কাপড়", "চামড়া", "কাচ", "অ্যালুমিনিয়াম", "তামা",
        "বাঁশ", "সিরামিক", "রাবার", "কাগজ",
    ];

    const COMMERCE_PRODUCT_TYPE: &'static [&'static str] = &[
        "চেয়ার", "টেবিল", "কম্পিউটার", "কীবোর্ড", "মাউস", "ফোন", "বই", "সোফা", "বেড",
        "ল্যাপটপ", "শার্ট", "জ্যাকেট", "জিন্স", "জুতা", "ঘড়ি", "ব্যাগ", "ল্যাম্প", "কাপ",
        "প্লেট", "তোয়ালে", "সাবান", "শ্যাম্পু", "খেলনা", "ক্যামেরা", "হেডফোন",
    ];

    const COMMERCE_PRODUCT: &'static str = "{Adjective} {Material} {Product}";
    const COMMERCE_PRODUCT_PRICE: &'static str = "####.##";

    const COMMERCE_PROMOTION_CODE_PREFIX: &'static [&'static str] = &[
        "####", "SAVE", "OFFER", "PROMO", "SALE", "DISCOUNT", "NEW", "SPECIAL",
    ];

    const COMMERCE_PROMOTION_CODE_SUFFIX: &'static [&'static str] = &[
        "####", "##", "2024", "2025", "10", "20", "50",
    ];

    const COMMERCE_PROMOTION_CODE: &'static [&'static str] = &[
        "{Prefix}-{Suffix}",
        "{Prefix}{Suffix}",
    ];

    const COMMERCE_PRODUCT_DESCRIPTION: &'static [&'static str] = &[
        "এই {Adjective} {Material} {Product} অত্যন্ত {Adjective}।",
        "এই {Product} {Adjective} {Material} দিয়ে তৈরি।",
        "{Department} বিভাগের জন্য {Adjective} {Product}।",
    ];

    // Company (Bengali)
    const COMPANY_SUFFIX: &'static [&'static str] = &["প্রাইভেট লিমিটেড", "লিমিটেড", "গ্রুপ", "এন্ড সন্স"];
    const COMPANY_NAME_TPLS: &'static [&'static str] = &["{Name_1} {Suffix}", "{Name_1} অ্যান্ড {Name_2} {Suffix}"];
    const COMPANY_BUZZWORD_HEAD: &'static [&'static str] = &[
        "অ্যাডাপ্টিভ", "অ্যাডভান্সড", "অটোমেটেড", "ইন্টিগ্রেটেড", "ইনোভেটিভ", "অপ্টিমাইজড",
        "রোবাস্ট", "স্কেলেবল", "মডার্ন", "ডিজিটাল",
    ];
    const COMPANY_BUZZWORD_MIDDLE: &'static [&'static str] = &[
        "২৪/৭", "রিয়েল-টাইম", "ক্লাউড-বেসড", "ক্লায়েন্ট-ফোকাসড", "ডেটা-ড্রিভেন",
        "স্কেলেবল", "সিকিউর", "ইন্টেলিজেন্ট",
    ];
    const COMPANY_BUZZWORD_TAIL: &'static [&'static str] = &[
        "সলিউশন", "প্ল্যাটফর্ম", "সিস্টেম", "সার্ভিস", "টেকনোলজি", "অ্যাপ্লিকেশন",
        "ইনফ্রাস্ট্রাকচার", "নেটওয়ার্ক",
    ];
    const COMPANY_CATCH_PHASE_TPL: &'static str = "{Head} {Middle} {Tail}";
    const COMPANY_BS_VERBS: &'static [&'static str] = &[
        "ইমপ্লিমেন্ট", "অপ্টিমাইজ", "ইন্টিগ্রেট", "ট্রান্সফর্ম", "এনাবল", "এনহ্যান্স",
        "ডেলিভার", "ডেভেলপ", "ম্যানেজ", "স্কেল",
    ];
    const COMPANY_BS_ADJ: &'static [&'static str] = &[
        "ভ্যালু-অ্যাডেড", "কাস্টমার-ফোকাসড", "ক্লাউড-নেটিভ", "সিকিউর", "স্কেলেবল",
        "ইনোভেটিভ", "ইন্টিগ্রেটেড", "মডার্ন",
    ];
    const COMPANY_BS_NOUNS: &'static [&'static str] = &[
        "সলিউশন", "প্ল্যাটফর্ম", "সিস্টেম", "সার্ভিস", "টেকনোলজি", "ইনফ্রাস্ট্রাকচার",
        "অ্যাপ্লিকেশন", "নেটওয়ার্ক", "ডেটাবেস",
    ];
    const COMPANY_BS_TPL: &'static str = "{Verb} {Adj} {Noun}";
    const COMPANY_INDUSTRY: &'static [&'static str] = &[
        "আইটি সেবা", "ব্যাংকিং", "শিক্ষা", "স্বাস্থ্য", "খুচরা বিক্রয়", "উৎপাদন",
        "কৃষি", "নির্মাণ", "পরিবহন", "টেলিকম", "মিডিয়া", "বিপণন",
    ];
    const COMPANY_PROFESSION: &'static [&'static str] = &[
        "শিক্ষক", "চিকিৎসক", "ইঞ্জিনিয়ার", "অ্যাকাউন্ট্যান্ট", "উকিল", "আর্কিটেক্ট",
        "প্রোগ্রামার", "ডিজাইনার", "নার্স", "সাংবাদিক", "বিক্রয় প্রতিনিধি",
    ];

    // Currency (Bangladesh focus)
    const CURRENCY_CODE: &'static [&'static str] = &["BDT", "USD", "EUR", "GBP", "INR"];
    const CURRENCY_NAME: &'static [&'static str] = &["টাকা", "US Dollar", "Euro", "Pound Sterling", "Indian Rupee"];
    const CURRENCY_SYMBOL: &'static [&'static str] = &["৳", "৳", "$", "€", "£"];

    // Internet (common in BD)
    const INTERNET_FREE_EMAIL_PROVIDER: &'static [&'static str] = &[
        "gmail.com", "yahoo.com", "hotmail.com", "outlook.com", "mail.com",
    ];
    const INTERNET_DOMAIN_SUFFIX: &'static [&'static str] = &["com", "com.bd", "org", "net", "edu.bd"];
}

impl CityNameGenFn for BN_BD {}
