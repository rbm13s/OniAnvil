use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct LangMeta {
    pub code: &'static str,
    pub file_stem: &'static str,
    pub flag: &'static str,
    pub name: &'static str,
    pub english_name: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
}

pub const ALL_LANGS: &[LangMeta] = &[
    LangMeta { code: "en", file_stem: "index", flag: "🇬🇧", name: "English", english_name: "English", title: "Pixel Sprite & Onion-Skin Animation Studio Built for Game-Ready Atlases", desc: "High-performance cross-platform pixel art workstation written in Rust. Multi-track layer timeline, configurable onion skinning, sketch tracing underlay, palette skin swapper, and game engine export." },
    LangMeta { code: "ru", file_stem: "ru", flag: "🇷🇺", name: "Русский", english_name: "Russian", title: "Студия пиксельной графики и анимации на базе игровых текстурных атласов", desc: "Высокопроизводительная кроссплатформенная рабочая станция для пиксель-арта на Rust. Профессиональный многодорожечный таймлайн, гибкая калька (Onion Skinning), слой эскизов с пипеткой, скины палитр и экспорт для игровых движков." },
    LangMeta { code: "uk", file_stem: "uk", flag: "🇺🇦", name: "Українська", english_name: "Ukrainian", title: "Студія піксельної графіки та покадрової анімації на базі ігрових атласів", desc: "Високопродуктивна кросплатформна робоча станція для піксель-арту на Rust. Багатодоріжковий таймлайн, гнучка калька (Onion Skinning), шар ескізів, скіни палітр та експорт для ігрових рушіїв." },
    LangMeta { code: "be", file_stem: "be", flag: "🇧🇾", name: "Беларуская", english_name: "Belarusian", title: "Студыя піксельнай графікі і пакадравай анімацыі на базе гульнявых атласаў", desc: "Высокапрадукцыйная працоўная станцыя для піксель-арту на Rust. Шматдарожкавы тайнлайн, гнуткая калька (Onion Skinning), пласт эскізаў, скіны палітраў і экспарт для гульнявых рухавічкоў." },
    LangMeta { code: "kk", file_stem: "kk", flag: "🇰🇿", name: "Қазақша", english_name: "Kazakh", title: "Ойын атластарына негізделген пиксельдік графика және кадрлық анимация студиясы", desc: "Rust тіліндегі пиксель-артқа арналған жоғары өнімді кроссплатформалық жұмыс станциясы. Көпжолды таймлайн, калька (Onion Skinning), эскиз қабаты және ойын қозғалтқыштарына экспорттау." },
    LangMeta { code: "ka", file_stem: "ka", flag: "🇬🇪", name: "ქართული", english_name: "Georgian", title: "პიქსელ-არტისა და ანიმაციის სტუდია თამაშის ატლასების ბაზაზე", desc: "მაღალი წარმადობის კროსპლატფორმული პიქსელ-არტის სამუშაო სადგური Rust-ზე. მრავალტრეკიანი ტაიმლაინი, მოქნილი კალკა (Onion Skinning), ესკიზის ფენა და ექსპორტი თამაშის ძრავებისთვის." },
    LangMeta { code: "hy", file_stem: "hy", flag: "🇦🇲", name: "Հայերեն", english_name: "Armenian", title: "Փիքսելային արվեստի և անիմացիայի ստուդիա՝ խաղային ատլասների հիման վրա", desc: "Բարձր արտադրողականությամբ պիքսել-արտի աշխատանքային կայան Rust-ով: Բազմաշերտ թայմլայն, ճկուն կալկա (Onion Skinning), էսքիզների շերտ և արտահանում խաղային շարժիչների համար:" },
    LangMeta { code: "az", file_stem: "az", flag: "🇦🇿", name: "Azərbaycan", english_name: "Azerbaijani", title: "Oyun atlasları bazasında piksel qrafikası və kadrba-kadr animasiya studiyası", desc: "Rust-da yazılmış yüksək məhsuldarlıqlı piksel-art iş stansiyası. Çoxcığırlı taymlayn, çevik kalka (Onion Skinning), eskiz qatı və oyun mühərrikləri üçün ixrac." },
    LangMeta { code: "uz", file_stem: "uz", flag: "🇺🇿", name: "Oʻzbekcha", english_name: "Uzbek", title: "Oʻyin atlaslari asosidagi piksel grafikasi va animatsiya studiyasi", desc: "Rust-da yaratilgan yuqori mahsuldorlikka ega piksel-art ish stansiyasi. Koʻp yoʻlli taymlayn, kalka (Onion Skinning), eskiz qatlami va oʻyin dvigatellari uchun eksport." },
    LangMeta { code: "de", file_stem: "de", flag: "🇩🇪", name: "Deutsch", english_name: "German", title: "Pixel-Art- & Onion-Skin-Animationsstudio für spielfertige Texturatlas-Dateien", desc: "Hochperformante plattformübergreifende Pixel-Art-Workstation in Rust. Mehrspurige Ebenen-Timeline, konfigurierbares Onion Skinning, Skizzen-Pauspause, Farbpaletten-Skins und Spiel-Engine-Export." },
    LangMeta { code: "fr", file_stem: "fr", flag: "🇫🇷", name: "Français", english_name: "French", title: "Studio d'animation pixel art et pelure d'oignon (Onion-Skin) pour atlas de jeux", desc: "Station de travail pixel art multiplateforme haute performance écrite en Rust. Chronologie multipiste, pelure d'oignon configurable, calque de croquis, variantes de palettes et export pour moteurs de jeu." },
    LangMeta { code: "es", file_stem: "es", flag: "🇪🇸", name: "Español", english_name: "Spanish", title: "Estudio de animación pixel art y papel cebolla (Onion-Skin) para atlas de juegos", desc: "Estación de trabajo de pixel art multiplataforma y de alto rendimiento escrita en Rust. Línea de tiempo multipista, papel cebolla configurable, capa de boceto, variantes de paletas y exportación para motores de juego." },
    LangMeta { code: "it", file_stem: "it", flag: "🇮🇹", name: "Italiano", english_name: "Italian", title: "Studio di animazione pixel art e onion skinning per atlanti di gioco", desc: "Workstation pixel art multipiattaforma ad alte prestazioni scritta in Rust. Timeline multilivello, onion skinning configurabile, livello di schizzo, varianti di tavolozza ed esportazione per motori di gioco." },
    LangMeta { code: "pt", file_stem: "pt", flag: "🇵🇹", name: "Português", english_name: "Portuguese", title: "Estúdio de pixel art e animação papel vegetal (Onion-Skin) para atlas de jogos", desc: "Estação de trabalho pixel art multiplataforma de alto desempenho escrita em Rust. Linha do tempo multipista, papel vegetal configurável, camada de esboço, variantes de paleta e exportação para motores de jogo." },
    LangMeta { code: "pl", file_stem: "pl", flag: "🇵🇱", name: "Polski", english_name: "Polish", title: "Studio grafiki pikselowej i animacji klatkowej (Onion Skin) dla atlasów gier", desc: "Wysokowydajna wieloplatformowa stacja robocza pixel art napisana w Rust. Wielościeżkowa oś czasu, konfigurowalna kalka (Onion Skinning), warstwa szkicu, warianty palet i eksport dla silników gier." },
    LangMeta { code: "cs", file_stem: "cs", flag: "🇨🇿", name: "Čeština", english_name: "Czech", title: "Studio pro pixel art a animaci s cibulovým pauzováním pro herní atlasy", desc: "Vysoce výkonná multiplatformní pracovní stanice pro pixel art v jazyce Rust. Vícestopá časová osa, cibulové pauzování (Onion Skinning), vrstva skici, výměna palet a export pro herní enginy." },
    LangMeta { code: "sk", file_stem: "sk", flag: "🇸🇰", name: "Slovenčina", english_name: "Slovak", title: "Štúdio pixel artu a animácie s cibuľovým pauzovaním pre herné atlasy", desc: "Vysoko výkonná multiplatformová pracovná stanica pre pixel art v jazyku Rust. Viacstopová časová os, cibuľové pauzovanie (Onion Skinning), vrstva náčrtu, výmena paliet a export pre herné enginy." },
    LangMeta { code: "sl", file_stem: "sl", flag: "🇸🇮", name: "Slovenščina", english_name: "Slovenian", title: "Studio za slikovno pikčasto risanje in animacijo čebulne lupine za igralne atlase", desc: "Visokozmogljiva večplatformna delovna postaja za slikovno pikčasto umetnost v Rustu. Večtirna časovnica, animacija čebulne lupine, skicirna plast in izvoz za igralne pogone." },
    LangMeta { code: "bg", file_stem: "bg", flag: "🇧🇬", name: "Български", english_name: "Bulgarian", title: "Студио за пикселно изкуство и анимация с лукова люспа за игрални атласи", desc: "Високопроизводителна кросплатформена работна станция за пикселно изкуство на Rust. Многопистова времева линия, лукова люспа (Onion Skinning), слой за скици и експорт за игрови двигатели." },
    LangMeta { code: "sr", file_stem: "sr", flag: "🇷🇸", name: "Српски", english_name: "Serbian", title: "Студио за пикселну уметност и анимацију са луковом кожом за атласе игара", desc: "Високоперформантна вишеплатформска радна станица за пиксел арт у Rust-у. Вишетрачна временска линија, лукова кожа (Onion Skinning), слој скица и извоз за погоне игара." },
    LangMeta { code: "hr", file_stem: "hr", flag: "🇭🇷", name: "Hrvatski", english_name: "Croatian", title: "Studio za pikselnu grafiku i animaciju luka (Onion Skin) za igrane atlase", desc: "Visokoučinkovita multiplatformska radna stanica za pixel art u Rustu. Višestazna vremenska traka, lukova koža (Onion Skinning), sloj skica i izvoz za pokretače igara." },
    LangMeta { code: "nl", file_stem: "nl", flag: "🇳🇱", name: "Nederlands", english_name: "Dutch", title: "Pixelart- en uienhuid-animatiestudio voor game-klare spritesheets", desc: "Krachtig platformonafhankelijk pixelart-werkstation geschreven in Rust. Meersporen tijdlijn, configureerbare uienhuid (Onion Skinning), schetslaag, paletvarianten en export voor game-engines." },
    LangMeta { code: "da", file_stem: "da", flag: "🇩🇰", name: "Dansk", english_name: "Danish", title: "Pixel art- og løgskinds-animationsstudie bygget til spilklare teksturatlasser", desc: "Højtydende platformsudvidende pixel art-arbejdsstation skrevet i Rust. Flersporet tidslinje, konfigurerbart løgskind (Onion Skinning), skitselag, paletvarianter og eksport til spilmotorer." },
    LangMeta { code: "no", file_stem: "no", flag: "🇳🇴", name: "Norsk", english_name: "Norwegian", title: "Pikselkunst- og løkskinn-animasjonsstudio bygget for spillklare teksturatlaser", desc: "Høyytelses plattformuavhengig pikselkunst-arbeidsstasjon skrevet i Rust. Fler-spors tidslinje, konfigurerbart løkskinn (Onion Skinning), skisselag, palettvarianter og eksport til spillmotorer." },
    LangMeta { code: "sv", file_stem: "sv", flag: "🇸🇪", name: "Svenska", english_name: "Swedish", title: "Pixelkonst- och lökbladsanimationsstudio byggd för spelklara texturatlaser", desc: "Högpresterande plattformsoberoende pixelkonst-arbetsstation skriven i Rust. Flerspårig tidslinje, konfigurerbart lökblad (Onion Skinning), skisslager, palettvarianter och export till spelmotorer." },
    LangMeta { code: "fi", file_stem: "fi", flag: "🇫🇮", name: "Suomi", english_name: "Finnish", title: "Pikselitaide- ja sipulinkuorianimaatiostudio pelivalmiille tekstuuriatlaksille", desc: "Huippusuorituskykyinen monialustainen pikselitaidetyöasema Rustilla. Moniraitainen aikajana, säädettävä sipulinkuori (Onion Skinning), luonnoskerros ja vienti pelimoottoreille." },
    LangMeta { code: "et", file_stem: "et", flag: "🇪🇪", name: "Eesti", english_name: "Estonian", title: "Pikslikunsti ja sibulakoore animatsioonistuudio mänguvalmis sprite-lehtedele", desc: "Kõrge jõudlusega platvormideülene pikslikunsti tööjaam Rustis. Mitmerajaline ajaskaala, konfigureeritav sibulakook (Onion Skinning), visandikiht ja eksport mängumootoritele." },
    LangMeta { code: "lv", file_stem: "lv", flag: "🇱🇻", name: "Latviešu", english_name: "Latvian", title: "Pikseļu mākslas un sīpolu mizas animācijas studija spēļu atlantiem", desc: "Augstas veiktspējas daudzplatformu pikseļu mākslas darbstacija programmēta Rust. Daudzceliņu laika skala, sīpolu miza (Onion Skinning), skiču slānis un eksports spēļu dzinējiem." },
    LangMeta { code: "lt", file_stem: "lt", flag: "🇱🇹", name: "Lietuvių", english_name: "Lithuanian", title: "Pikselių meno ir svogūno lukšto animacijos studija žaidimų atlasams", desc: "Didelio našumo kelių platformų pikselių meno darbo stotis Rust kalba. Kelių takelių laiko juosta, svogūno lukštas (Onion Skinning), eskizų sluoksnis ir eksportas žaidimų varikliams." },
    LangMeta { code: "hu", file_stem: "hu", flag: "🇭🇺", name: "Magyar", english_name: "Hungarian", title: "Pixel art és hagymázott animációs stúdió játékra kész atlaszokhoz", desc: "Nagy teljesítményű többplatformos pixel art munkaállomás Rust nyelven. Többsávos idővonal, konfigurálható hagymázás (Onion Skinning), vázlatréteg és export játék motorokhoz." },
    LangMeta { code: "ro", file_stem: "ro", flag: "🇷🇴", name: "Română", english_name: "Romanian", title: "Studio de animație pixel art și foi de ceapă (Onion Skin) pentru atlase de jocuri", desc: "Stație de lucru pixel art multiplatformă de înaltă performanță scrisă în Rust. Cronologie multipistă, foi de ceapă configurabile (Onion Skinning), strat de schiță și export pentru motoare de joc." },
    LangMeta { code: "tr", file_stem: "tr", flag: "🇹🇷", name: "Türkçe", english_name: "Turkish", title: "Oyun atlasları için piksel sanatı ve soğan zarı (Onion Skin) animasyon stüdyosu", desc: "Rust ile yazılmış yüksek performanslı çapraz platform piksel sanatı iş istasyonu. Çok kanallı zaman çizelgesi, yapılandırılabilir soğan zarı, taslak katmanı ve oyun motoru dışa aktarımı." },
    LangMeta { code: "el", file_stem: "el", flag: "🇬🇷", name: "Ελληνικά", english_name: "Greek", title: "Στούντιο pixel art και διαδοχικής κινούμενης εικόνας για άτλαντες παιχνιδιών", desc: "Υψηλής απόδοσης διαπλατφορμικός σταθμός εργασίας pixel art γραμμένος σε Rust. Χρονοδιάγραμμα πολλαπλών κομματιών, διαφάνεια onion skinning, στρώμα σχεδίου και εξαγωγή για μηχανές παιχνιδιών." },
    LangMeta { code: "zh-cn", file_stem: "zh-cn", flag: "🇨🇳", name: "简体中文", english_name: "Chinese Simplified", title: "专为游戏图集打造的像素画与洋葱皮定格动画工作室", desc: "使用 Rust 编写的高性能跨平台像素艺术工作站。多轨道时间轴、可配置洋葱皮透光、参考草图底图、调色板皮肤切换以及游戏引擎导出。" },
    LangMeta { code: "zh-tw", file_stem: "zh-tw", flag: "🇹🇼", name: "繁體中文", english_name: "Chinese Traditional", title: "專為遊戲圖集打造的像素畫與洋蔥皮逐格動畫工作室", desc: "使用 Rust 編寫的高效能跨平台像素藝術工作站。多軌道時間軸、可配置洋葱皮透光、參考草圖底圖、調色板皮膚切換以及遊戲引擎匯出。" },
    LangMeta { code: "ja", file_stem: "ja", flag: "🇯🇵", name: "日本語", english_name: "Japanese", title: "ゲーム用スプライトアトラスに最適化されたピクセルアート＆オニオンスキンアニメーションスタジオ", desc: "Rustで書かれた高性能クロスプラットフォームピクセルアートワークステーション。マルチトラックレイヤータイムライン、オニオンスキン、下絵トレース、パレットスワップ、ゲームエンジン書き出しに対応。" },
    LangMeta { code: "ko", file_stem: "ko", flag: "🇰🇷", name: "한국어", english_name: "Korean", title: "게임용 스프라이트 아틀라스를 위한 픽셀 아트 및 어니언 스킨 애니메이션 스튜디오", desc: "Rust로 작성된 고성능 크로스 플랫폼 픽셀 아트 워크스테이션. 멀티 트랙 레이어 타임라인, 구성 가능한 어니언 스킨, 스케치 트레이싱, 팔레트 스왑 및 게임 엔진 내보내기 지원." },
];

pub fn get_ui_nav_labels(code: &str) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
    // (Features, Tools, Showcase, Shortcuts, Downloads)
    match code {
        "ru" => ("Возможности", "Инструменты", "Скриншоты", "Горячие клавиши", "Скачать"),
        "uk" => ("Можливості", "Інструменти", "Скріншоти", "Гарячі клавіші", "Завантажити"),
        "be" => ("Магчымасці", "Інструменты", "Скрыншоты", "Гарачыя клавішы", "Спампаваць"),
        "kk" => ("Мүмкіндіктер", "Құралдар", "Скриншоттар", "Ыстық пернелер", "Жүктеп алу"),
        "ka" => ("შესაძლებლობები", "ინსტრუმენტები", "სკრინშოტები", "ცხელი კლავიშები", "ჩამოტვირთვა"),
        "hy" => ("Հնարավորություններ", "Գործիքներ", "Սքրինշոթեր", "Ստեղնաշարային դյուրանցումներ", "Ներբեռնել"),
        "az" => ("Xüsusiyyətlər", "Alətlər", "Skrinşotlar", "Qaynar düymələr", "Yüklə"),
        "uz" => ("Imkoniyatlar", "Asboblar", "Skrinshotlar", "Tezkor tugmalar", "Yuklab olish"),
        "de" => ("Funktionen", "Werkzeuge", "Galerie", "Tastenkürzel", "Downloads"),
        "fr" => ("Fonctionnalités", "Outils", "Captures", "Raccourcis", "Téléchargements"),
        "es" => ("Características", "Herramientas", "Capturas", "Atajos", "Descargas"),
        "it" => ("Caratteristiche", "Strumenti", "Galleria", "Scorciatoie", "Download"),
        "pt" => ("Recursos", "Ferramentas", "Galeria", "Atalhos", "Downloads"),
        "pl" => ("Funkcje", "Narzędzia", "Zrzuty ekranu", "Skróty klawiszowe", "Pobierz"),
        "cs" => ("Funkce", "Nástroje", "Snímky obrazovky", "Zkratky", "Stáhnout"),
        "sk" => ("Funkcie", "Nástroje", "Snímky obrazovky", "Skratky", "Stiahnuť"),
        "sl" => ("Zmožnosti", "Orodja", "Posnetki", "Bližnjice", "Prenosi"),
        "bg" => ("Възможности", "Инструменти", "Екранни снимки", "Бързи клавиши", "Изтегляне"),
        "sr" => ("Могућности", "Алати", "Снимци екрана", "Пречице", "Преузми"),
        "hr" => ("Značajke", "Alati", "Snimke zaslona", "Prečaci", "Preuzimanja"),
        "nl" => ("Functies", "Gereedschappen", "Schermafbeeldingen", "Sneltoetsen", "Downloads"),
        "da" => ("Funktioner", "Værktøjer", "Skærmbilleder", "Genveje", "Downloads"),
        "no" => ("Funksjoner", "Verktøy", "Skjermbilder", "Snarveier", "Nedlastinger"),
        "sv" => ("Funktioner", "Verktyg", "Skärmdumpar", "Genvägar", "Nedladdningar"),
        "fi" => ("Ominaisuudet", "Työkalut", "Kuvakaappaukset", "Pikanäppäimet", "Lataukset"),
        "et" => ("Funktsioonid", "Tööriistad", "Kuvatõmmised", "Kiirklahvid", "Allalaadimised"),
        "lv" => ("Iespējas", "Rīki", "Ekrānuzņēmumi", "Īsinājumtaustiņi", "Lejupielādes"),
        "lt" => ("Ypatybės", "Įrankiai", "Ekrano kopijos", "Spartieji klavišai", "Atsisiuntimai"),
        "hu" => ("Funkciók", "Eszközök", "Képernyőképek", "Gyorsbillentyűk", "Letöltések"),
        "ro" => ("Funcționalități", "Instrumente", "Capturi", "Comenzi rapide", "Descărcări"),
        "tr" => ("Özellikler", "Araçlar", "Ekran Görüntüleri", "Kısayollar", "İndir"),
        "el" => ("Δυνατότητες", "Εργαλεία", "Στιγμιότυπα", "Συντομεύσεις", "Λήψεις"),
        "zh-cn" => ("功能特性", "工具指南", "视觉展示", "快捷键", "下载客户端"),
        "zh-tw" => ("功能特性", "工具指南", "視覺展示", "快捷鍵", "下載用戶端"),
        "ja" => ("機能紹介", "ツール一覧", "スクリーンショット", "ショートカット", "ダウンロード"),
        "ko" => ("주요 기능", "도구 모음", "스크린샷", "단축키", "다운로드"),
        _ => ("Features", "Tools", "Showcase", "Shortcuts", "Downloads"),
    }
}

pub fn get_showcase_tabs(code: &str) -> [&'static str; 12] {
    match code {
        "ru" => [
            "1. Рабочее пространство", "2. Панели инструментов", "3. Таймлайн и слои", "4. Живое превью",
            "5. Палитры и замена цвета", "6. Точки привязки и слоты", "7. Эскиз и калька (Onion Skin)", "8. Нарезка листов и 9-Slice UI",
            "9. Живые фильтры и матрица", "10. 35 ретро-тем", "11. Сетки и фоны холста", "12. Меню и создание проекта"
        ],
        "uk" => [
            "1. Робочий простір", "2. Панелі інструментів", "3. Таймлайн і шари", "4. Живе прев'ю",
            "5. Палітри та заміна кольорів", "6. Опорні точки та слоти", "7. Ескіз і калька (Onion Skin)", "8. Нарізка аркушів і 9-Slice UI",
            "9. Живі фільтри та матриця", "10. 35 ретро-тем", "11. Сітки та фони полотна", "12. Меню та створення проекту"
        ],
        "be" => [
            "1. Працоўная прастора", "2. Панэлі інструментаў", "3. Таймлайн і пласты", "4. Жывы перадпрагляд",
            "5. Палітры і замена колеру", "6. Апорныя кропкі і слоты", "7. Эскіз і калька (Onion Skin)", "8. Нарэзка аркушаў і 9-Slice UI",
            "9. Жывыя фільтры і матрыца", "10. 35 рэтра-тэм", "11. Сеткі і фоны палатна", "12. Меню і стварэнне праекта"
        ],
        "de" => [
            "1. Arbeitsbereich", "2. Werkzeugleisten", "3. Timeline & Ebenen", "4. Live-Vorschau",
            "5. Paletten & Farbtausch", "6. Drehpunkte & Slots", "7. Skizze & Onion Skin", "8. Sheet-Slicer & 9-Slice UI",
            "9. Live-Filter & Matrizen", "10. 35 Retro-Themes", "11. Raster & Hintergründe", "12. Menüs & Projekterstellung"
        ],
        "fr" => [
            "1. Espace de travail", "2. Barres d'outils", "3. Chronologie & Calques", "4. Aperçu en direct",
            "5. Palettes & Remplacement", "6. Pivots & Points d'action", "7. Croquis & Pelure d'oignon", "8. Découpeur & UI 9-Slice",
            "9. Filtres & Matrice 3x3", "10. 35 Thèmes rétro", "11. Grilles & Arrière-plans", "12. Menus & Création de projet"
        ],
        "es" => [
            "1. Espacio de trabajo", "2. Barras de herramientas", "3. Línea de tiempo y capas", "4. Vista previa en vivo",
            "5. Paletas y reemplazo de color", "6. Pivotes y puntos de acción", "7. Boceto y papel cebolla", "8. Cortador de hojas y 9-Slice",
            "9. Filtros en vivo y matriz", "10. 35 Temas retro", "11. Cuadrículas y fondos", "12. Menús y nuevo proyecto"
        ],
        "zh-cn" => [
            "1. 主工作区", "2. 工具与栏目", "3. 时间轴与图层", "4. 实时动画预览",
            "5. 调色板与色彩置换", "6. 旋转原点与动作槽", "7. 草图底图与洋葱皮", "8. 切片器与九宫格UI",
            "9. 动态滤镜与卷积矩阵", "10. 35套复古主题", "11. 像素网格与棋盘背景", "12. 顶部菜单与新建项目"
        ],
        "ja" => [
            "1. ワークスペース", "2. ツールバーとドック", "3. タイムライン＆レイヤー", "4. リアルタイムプレビュー",
            "5. パレット＆色置換", "6. 回転ピボット＆ソケット", "7. 下絵トレース＆オニオンスキン", "8. シートスライサー＆9-Slice",
            "9. ライブFX＆3x3行列フィルター", "10. 35種のレトロテーマ", "11. グリッド＆市松模様背景", "12. メニュー＆新規プロジェクト"
        ],
        _ => [
            "1. Workspace", "2. Tools & Toolbar", "3. Timeline & Layers", "4. Live Preview",
            "5. Palette & Swaps", "6. Pivot & Action Points", "7. Sketch & Onion Skin", "8. Slicer & 9-Slice UI",
            "9. Live FX & Filters", "10. 35 Retro Themes", "11. Viewports & Backgrounds", "12. Menus & Dialogs"
        ],
    }
}

fn build_language_dropdown(current_code: &str) -> String {
    let mut s = String::from("          <select aria-label=\"Language\" onchange=\"window.location.href=this.value\">\n");
    for l in ALL_LANGS {
        let filename = if l.code == "en" { "index.html".to_string() } else { format!("{}.html", l.code) };
        let selected = if l.code == current_code { " selected" } else { "" };
        s.push_str(&format!("            <option value=\"{}\"{}>{} {}</option>\n", filename, selected, l.flag, l.name));
    }
    s.push_str("          </select>");
    s
}

fn build_markdown_language_bar(current_code: &str, in_docs_dir: bool) -> String {
    let mut s = String::from("**Languages / Языки / Мови / Sprachen / 语言:**<br/>\n");
    let mut links = Vec::new();
    for l in ALL_LANGS {
        let is_current = l.code == current_code;
        let link_target = if l.code == "en" {
            if in_docs_dir { "../README.md".to_string() } else { "README.md".to_string() }
        } else if in_docs_dir {
            format!("README-{}.md", l.code)
        } else {
            format!("docs/README-{}.md", l.code)
        };

        if is_current {
            links.push(format!("**{}**", l.name));
        } else {
            links.push(format!("[{}]({})", l.name, link_target));
        }
    }
    s.push_str(&links.join(" •\n"));
    s
}

fn main() {
    let en_path = Path::new("assets/locales/en.json");
    let ru_path = Path::new("assets/locales/ru.json");

    let en_text = fs::read_to_string(en_path).expect("Read en.json");
    let ru_text = fs::read_to_string(ru_path).expect("Read ru.json");

    let en_map: BTreeMap<String, String> = serde_json::from_str(&en_text).expect("Parse en.json");
    let ru_map: BTreeMap<String, String> = serde_json::from_str(&ru_text).expect("Parse ru.json");

    println!("Base keys loaded: {} (en), {} (ru)", en_map.len(), ru_map.len());

    // Generate 35 JSON locale files
    for lang in ALL_LANGS {
        if lang.code == "en" || lang.code == "ru" {
            continue;
        }

        let out_json_path = format!("assets/locales/{}.json", lang.code);
        let mut target_map = BTreeMap::new();

        // Use appropriate translation base: Slavic languages use Russian base where closer, others use English base
        let use_ru_base = matches!(lang.code, "uk" | "be" | "kk" | "bg" | "sr");

        for (k, en_val) in &en_map {
            let base_val = if use_ru_base {
                ru_map.get(k).unwrap_or(en_val)
            } else {
                en_val
            };

            // Custom specific overrides for common keys
            let val = match k.as_str() {
                "menu.file" => match lang.code {
                    "uk" => "Файл", "be" => "Файл", "de" => "Datei", "fr" => "Fichier", "es" => "Archivo", "it" => "File", "pt" => "Ficheiro", "pl" => "Plik", "cs" => "Soubor", "zh-cn" => "文件", "zh-tw" => "檔案", "ja" => "ファイル", "ko" => "파일", _ => base_val,
                },
                "menu.edit" => match lang.code {
                    "uk" => "Правка", "be" => "Праўка", "de" => "Bearbeiten", "fr" => "Édition", "es" => "Editar", "it" => "Modifica", "pt" => "Editar", "pl" => "Edycja", "cs" => "Úpravy", "zh-cn" => "编辑", "zh-tw" => "編輯", "ja" => "編集", "ko" => "편집", _ => base_val,
                },
                "menu.view" => match lang.code {
                    "uk" => "Вигляд", "be" => "Выгляд", "de" => "Ansicht", "fr" => "Affichage", "es" => "Ver", "it" => "Visualizza", "pt" => "Ver", "pl" => "Widok", "cs" => "Zobrazení", "zh-cn" => "视图", "zh-tw" => "檢視", "ja" => "表示", "ko" => "보기", _ => base_val,
                },
                "menu.help" => match lang.code {
                    "uk" => "Довідка", "be" => "Даведка", "de" => "Hilfe", "fr" => "Aide", "es" => "Ayuda", "it" => "Aiuto", "pt" => "Ajuda", "pl" => "Pomoc", "cs" => "Nápověda", "zh-cn" => "帮助", "zh-tw" => "說明", "ja" => "ヘルプ", "ko" => "도움말", _ => base_val,
                },
                "tools.pencil" => match lang.code {
                    "uk" => "Олівець", "be" => "Аловак", "de" => "Bleistift", "fr" => "Crayon", "es" => "Lápiz", "it" => "Matita", "pt" => "Lápis", "pl" => "Ołówek", "cs" => "Tužka", "zh-cn" => "铅笔", "zh-tw" => "鉛筆", "ja" => "鉛筆", "ko" => "연필", _ => base_val,
                },
                "tools.eraser" => match lang.code {
                    "uk" => "Гумка", "be" => "Гумка", "de" => "Radierer", "fr" => "Gomme", "es" => "Borrador", "it" => "Gomma", "pt" => "Borracha", "pl" => "Gumka", "cs" => "Guma", "zh-cn" => "橡皮擦", "zh-tw" => "橡皮擦", "ja" => "消しゴム", "ko" => "지우개", _ => base_val,
                },
                "tools.eyedropper" => match lang.code {
                    "uk" => "Піпетка", "be" => "Піпетка", "de" => "Pipette", "fr" => "Pipette", "es" => "Cuentagotas", "it" => "Contagocce", "pt" => "Conta-gotas", "pl" => "Pipeta", "cs" => "Kapátko", "zh-cn" => "吸管", "zh-tw" => "滴管", "ja" => "スポイト", "ko" => "스포이트", _ => base_val,
                },
                "tools.fill" => match lang.code {
                    "uk" => "Заливка", "be" => "Заліванне", "de" => "Füllung", "fr" => "Remplissage", "es" => "Bote de pintura", "it" => "Secchiello", "pt" => "Balde de tinta", "pl" => "Wypełnienie", "cs" => "Plechovka", "zh-cn" => "油漆桶", "zh-tw" => "油漆桶", "ja" => "塗りつぶし", "ko" => "페인트통", _ => base_val,
                },
                _ => base_val,
            };

            target_map.insert(k.clone(), val.to_string());
        }

        let json_output = serde_json::to_string_pretty(&target_map).expect("Serialize locale JSON");
        fs::write(&out_json_path, json_output).expect("Write locale json");
        println!("Generated {}", out_json_path);
    }

    // Now update README.md and docs/README-ru.md with the 37-language bar
    let en_bar = build_markdown_language_bar("en", false);
    let ru_bar = build_markdown_language_bar("ru", true);

    let readme_en = fs::read_to_string("README.md").expect("Read README.md");
    if let Some(pos_start) = readme_en.find("**Languages /") {
        if let Some(pos_end) = readme_en[pos_start..].find("</div>") {
            let before = &readme_en[..pos_start];
            let after = &readme_en[pos_start + pos_end..];
            let new_content = format!("{}{}\n\n{}", before, en_bar, after);
            fs::write("README.md", new_content).expect("Write README.md");
            println!("Updated README.md language bar.");
        }
    }

    let readme_ru = fs::read_to_string("docs/README-ru.md").expect("Read README-ru.md");
    if let Some(pos_start) = readme_ru.find("**Языки /") {
        if let Some(pos_end) = readme_ru[pos_start..].find("</div>") {
            let before = &readme_ru[..pos_start];
            let after = &readme_ru[pos_start + pos_end..];
            let new_content = format!("{}{}\n\n{}", before, ru_bar, after);
            fs::write("docs/README-ru.md", new_content).expect("Write README-ru.md");
            println!("Updated docs/README-ru.md language bar.");
        }
    }

    // Update docs/index.html and docs/ru.html with the 37-language select dropdown
    let index_html = fs::read_to_string("docs/index.html").expect("Read index.html");
    if let Some(s_pos) = index_html.find("<div class=\"lang-selector\">") {
        if let Some(e_pos) = index_html[s_pos..].find("</div>") {
            let before = &index_html[..s_pos + 27];
            let after = &index_html[s_pos + e_pos..];
            let dropdown = build_language_dropdown("en");
            let new_html = format!("{}\n{}\n        {}", before, dropdown, after);
            fs::write("docs/index.html", new_html).expect("Write index.html");
            println!("Updated docs/index.html language dropdown.");
        }
    }

    let ru_html = fs::read_to_string("docs/ru.html").expect("Read ru.html");
    if let Some(s_pos) = ru_html.find("<div class=\"lang-selector\">") {
        if let Some(e_pos) = ru_html[s_pos..].find("</div>") {
            let before = &ru_html[..s_pos + 27];
            let after = &ru_html[s_pos + e_pos..];
            let dropdown = build_language_dropdown("ru");
            let new_html = format!("{}\n{}\n        {}", before, dropdown, after);
            fs::write("docs/ru.html", new_html).expect("Write ru.html");
            println!("Updated docs/ru.html language dropdown.");
        }
    }

    // Generate docs/README-<lang>.md and docs/<lang>.html for all other 35 languages
    for lang in ALL_LANGS {
        if lang.code == "en" || lang.code == "ru" {
            continue;
        }

        let (nav_feat, nav_tools, nav_show, nav_keys, nav_dl) = get_ui_nav_labels(lang.code);
        let tabs = get_showcase_tabs(lang.code);
        let lang_bar = build_markdown_language_bar(lang.code, true);
        let dropdown = build_language_dropdown(lang.code);

        // Generate docs/README-<lang>.md
        // Based on README-ru.md structure
        let md_template = format!(r###"<div align="center">

<img src="assets/icon.jpg" alt="OniAnvil Logo" width="128" height="128" style="border-radius: 24px;" />

# OniAnvil

**{title}**

[![Build & Release](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml/badge.svg)](https://github.com/rbm13s/OniAnvil/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/github/downloads/rbm13s/OniAnvil/total?logo=github&color=blue)](https://github.com/rbm13s/OniAnvil/releases)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui_%2F_eframe_0.31-blue)](https://github.com/emilk/egui)
[![Renderer](https://img.shields.io/badge/Renderer-wgpu-green)](https://wgpu.rs/)
[![Languages](https://img.shields.io/badge/Languages-37_Locales-purple)](#localization)
[![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Linux_%7C_macOS_%7C_Web-0078D6)](#getting-started)
[![Web Demo](https://img.shields.io/badge/Online_App-Try_in_Browser_(Experimental)-yellow?logo=webassembly)](https://rbm13s.github.io/OniAnvil/app/?lang={code})
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)

<br/>

👉 **[🚀 Try OniAnvil in Browser (WebAssembly)](https://rbm13s.github.io/OniAnvil/app/?lang={code})** *(⚠️ Experimental build: may contain WebGL/WASM limitations and bugs)* • **[🌐 Official Documentation Website](https://rbm13s.github.io/OniAnvil/{code}.html)**

<br/>

{lang_bar}

</div>

---

## {title}

{desc}

### Visual Tour

| Workspace | Timeline & Layers | Palette & Color Swaps |
|:---:|:---:|:---:|
| [![Workspace](screenshots/main.png)](screenshots/main.png) | [![Timeline](screenshots/timeline.png)](screenshots/timeline.png) | [![Palette](screenshots/palette_and_swaps.png)](screenshots/palette_and_swaps.png) |
| **Workspace & Canvas** | **Multi-Track Studio Timeline** | **Palette Editor & Swaps** |

| Sheet Slicer | 3x3 UI Margins | Rotation Pivot & Sockets |
|:---:|:---:|:---:|
| [![Slicer](screenshots/slicer.png)](screenshots/slicer.png) | [![3x3](screenshots/3x3.png)](screenshots/3x3.png) | [![Pivot](screenshots/pivot_and_action_points.png)](screenshots/pivot_and_action_points.png) |
| **Automatic Sheet Slicer** | **9-Slice Scalable UI** | **Origin Pivot & Custom Sockets** |

---

## Comprehensive Tools Reference

| Icon | Tool | Shortcut | Options | Description |
|:---:|:---|:---:|:---|:---|
| `\u{{f303}}` | **Pencil** | <kbd>P</kbd> | 1–64 px, Pixel-Perfect | 1px precision raster drawing. Pixel-Perfect algorithm eliminates unwanted staircase corners. |
| `\u{{f12d}}` | **Eraser** | <kbd>E</kbd> | 1–64 px | Removes pixel color data back to pure alpha transparency on active layer. |
| `\u{{f1fb}}` | **Eyedropper** | <kbd>I</kbd> / <kbd>Alt</kbd> | Canvas / Sketch | Real-time pixel color sampling. Left-click sets Primary color; Right-click sets Secondary color. |
| `\u{{f576}}` | **Paint Bucket** | <kbd>G</kbd> | Tolerance 0–255, 4/8-way | Flood fills connected color regions or all matching pixels on the canvas. |
| `\u{{f5c3}}` | **Gradient** | <kbd>U</kbd> | Linear / Radial, Dither | Interactive two-point gradient interpolation with optional retro Bayer pattern dithering. |
| `\u{{f715}}` | **Line** | <kbd>V</kbd> | 1–64 px, Shift-Snap | Bresenham straight line rasterizer with live rubber-banding preview. |
| `\u{{f55b}}` | **Bézier Curve** | <kbd>C</kbd> | 1–64 px, 3-point | Three-point quadratic Bézier curve: start point, end point, and middle control node. |
| `\u{{f0c8}}` | **Rectangle** | <kbd>R</kbd> | Outline / Solid / Both | Geometric box tool. Holding <kbd>Shift</kbd> constrains to a 1:1 square. |
| `\u{{f111}}` | **Ellipse** | <kbd>O</kbd> | Outline / Solid / Both | Midpoint ellipse and circle rasterizer. Holding <kbd>Shift</kbd> forces an equidistant circle. |
| `\u{{f43c}}` | **Dither Brush** | <kbd>D</kbd> | Bayer 2×2 / 4×4 / 8×8 | Authentic retro hardware shading alternating primary and secondary pixels. |
| `\u{{f042}}` | **Shading** | <kbd>H</kbd> | Lighten / Darken | Steps pixel colors up or down within color ramps defined in the current palette. |
| `\u{{f5cb}}` | **Rect Select** | <kbd>M</kbd> | Marching Ants, Clipboard | Defines rectangular pixel bounding boxes with animated marching ants boundary. |
| `\u{{f5ee}}` | **Lasso Select** | <kbd>L</kbd> | Freehand Polygon | Freeform polygonal selection tool to isolate complex silhouettes. |
| `\u{{f0d0}}` | **Magic Wand** | <kbd>W</kbd> | Tolerance 0–255 | Selects color-contiguous or canvas-wide pixel clusters matching clicked pixel color. |
| `\u{{f05b}}` | **Pivot & Sockets** | <kbd>Y</kbd> | Center, Named Sockets | Sets frame rotation pivot center and assigns custom named socket points (`muzzle`, `grip`). |

---

## Key Features

1. **Dual Atlas-Centric Architecture**: The game sprite atlas texture is the single source of truth.
2. **Multi-Track Layer Timeline**: Independent tracks (`V1, V2, V3...`), duration multipliers, linked frames, and loop tags.
3. **Real-Time Live Preview**: Floating preview window with FPS control, playback loop, and zoom.
4. **Configurable Onion Skinning**: Multi-frame ghosting with independent red/blue tinting and opacity falloff.
5. **Reference Sketch Underlay**: Load PNG/JPEG/WebP concept art as an underlay with free pan, zoom, and eyedropper sampling.
6. **Live FX & RotSprite**: HSV adjustment, automatic pixel outline generator, 3x3 convolution filters, and RotSprite rotation.
7. **Palette Management & Skins**: Day/Night variants, team colors, and GPL/HEX palette import/export.
8. **Sheet Slicer & 9-Slice UI**: Auto-detect sprite boundaries and configure 9-slice scalable UI margins.
9. **Game Engine Ready Export**: Scaled PNGs with extruded borders, animated GIFs, and JSON metadata (Godot, Bevy, Raylib).
10. **35 Retro & Modern Themes**: CRT Amber, CRT Green, Famicom, Mega Drive, Game Boy, PSX, and sleek studio dark/light themes.

---

## Keyboard Shortcuts

| Shortcut | Tool / Action | Description |
|:---:|:---|:---|
| <kbd>P</kbd> | Pencil | 1px precision raster drawing |
| <kbd>E</kbd> | Eraser | Clear pixels back to transparency |
| <kbd>I</kbd> / <kbd>Alt</kbd> | Eyedropper | Sample color from canvas or sketch |
| <kbd>G</kbd> | Paint Bucket | Flood fill contiguous color region |
| <kbd>U</kbd> | Gradient | Linear and radial gradients |
| <kbd>V</kbd> | Line | Straight lines with angle snapping |
| <kbd>C</kbd> | Bézier Curve | Smooth 3-point quadratic spline |
| <kbd>R</kbd> | Rectangle | Outlined or solid rectangular shapes |
| <kbd>O</kbd> | Ellipse | Outlined or solid circles and ellipses |
| <kbd>D</kbd> | Dither Brush | Pattern dither drawing |
| <kbd>H</kbd> | Shading | Step lighter or darker through palette ramps |
| <kbd>M</kbd> | Rectangular Select | Select region with marching ants |
| <kbd>L</kbd> | Lasso Select | Freehand polygon selection |
| <kbd>W</kbd> | Magic Wand | Select color-contiguous pixels |
| <kbd>Y</kbd> | Pivot & Sockets | Rotation center and socket points |
| <kbd>X</kbd> | Swap Swatches | Swap Primary and Secondary color swatches |
| <kbd>Ctrl+C / X / V</kbd> | Clipboard | Copy, cut, and paste pixel clusters |
| <kbd>Space</kbd> | Play / Pause | Toggle animation playback |
| <kbd>A / D</kbd> | Step Frames | Step to Previous / Next frame |
| <kbd>Shift+R</kbd> | Replace Color | Interactive color replacement dialog |
| <kbd>Ctrl+U</kbd> | Color Adjust | HSV hue shift and contrast adjustment |
| <kbd>Ctrl+R</kbd> | Free Rotate | Arbitrary angle rotation via RotSprite |
| <kbd>Space+Drag</kbd> | Pan Canvas | Smooth infinite panning |
| <kbd>Ctrl+Wheel</kbd> | Zoom | Pixel-crisp cursor-centered zoom |

---

## Getting Started

### Standalone Binaries
- **Windows**: Download `OniAnvil-windows-x86_64.zip`, extract, and launch `onianvil.exe`.
- **Linux**: Download `OniAnvil-linux-x86_64.tar.gz` (portable) or install `onianvil_*_amd64.deb`.
- **macOS**: Download `OniAnvil-macos-universal.zip`, extract, and drag `OniAnvil.app` to Applications.
- **Web**: Run directly in your browser without installing anything: **[Launch WebAssembly App](https://rbm13s.github.io/OniAnvil/app/?lang={code})**.
  > [!WARNING]
  > The WebAssembly (browser) build has an **Experimental** status. Due to browser sandbox constraints, WebGL/WebGPU differences, and clipboard limitations, you may encounter minor graphical glitches or performance bottlenecks. For production work, native desktop builds are strongly recommended.

---

## License & Disclaimer

This project is licensed under the **[MIT License](../LICENSE)**.

### Disclaimer of Warranty & Limitation of Liability
> [!IMPORTANT]
> **OniAnvil** is distributed **free of charge** on an **"AS IS"** and **"AS AVAILABLE"** basis, without warranties or conditions of any kind, either express or implied.
> 
> Under no circumstances shall the author(s), copyright holders, or contributors be held liable for any direct, indirect, incidental, special, exemplary, or consequential damages (including, but not limited to, loss of data, file corruption, hardware failure, lost profits, or business interruption) arising in any way out of the use of or inability to use this software. You assume all responsibility and risk for using this program.
"###,
            code = lang.code,
            title = lang.title,
            desc = lang.desc,
            lang_bar = lang_bar
        );

        let out_md_path = format!("docs/README-{}.md", lang.code);
        fs::write(&out_md_path, md_template).expect("Write localized README");
        println!("Generated {}", out_md_path);

        // Generate docs/<lang>.html
        let out_html_path = format!("docs/{}.html", lang.code);
        let html_template = format!(r###"<!DOCTYPE html>
<html lang="{code}">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>OniAnvil — {title}</title>
  <meta name="description" content="{desc}">
  <link rel="icon" type="image/x-icon" href="assets/icon.ico">
  <link rel="stylesheet" href="style.css">
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.2/css/all.min.css">
  <script>
    function switchTab(idx) {{
      var btns = document.querySelectorAll('.tab-btn');
      var contents = document.querySelectorAll('.tab-content');
      for (var i = 0; i < btns.length; i++) {{
        btns[i].classList.remove('active');
        contents[i].classList.remove('active');
      }}
      btns[idx].classList.add('active');
      contents[idx].classList.add('active');
    }}
  </script>
</head>
<body>

  <!-- Header -->
  <header class="site-header">
    <div class="container nav-wrap">
      <a href="{code}.html" class="brand">
        <img src="assets/icon.jpg" alt="OniAnvil Logo">
        <span>OniAnvil</span>
        <span class="badge-ver">v1.0.0</span>
      </a>

      <nav class="main-nav">
        <a href="#features">{nav_feat}</a>
        <a href="#tools">{nav_tools}</a>
        <a href="#showcase">{nav_show}</a>
        <a href="#shortcuts">{nav_keys}</a>
        <a href="#downloads">{nav_dl}</a>
        <a href="https://github.com/rbm13s/OniAnvil" target="_blank" rel="noopener">GitHub</a>
      </nav>

      <div class="header-actions">
        <div class="lang-selector">
{dropdown}
        </div>
        <a href="app/?lang={code}" class="btn btn-primary btn-sm">Online</a>
      </div>
    </div>
  </header>

  <!-- Hero -->
  <section class="hero">
    <div class="container">
      <div class="hero-pill">
        <span class="dot"></span>
        <span>v1.0.0 • Open Source MIT • Native Desktop &amp; WebAssembly</span>
      </div>

      <h1 class="hero-title">
        {title}
      </h1>

      <p class="hero-subtitle">
        {desc}
      </p>

      <div class="hero-cta">
        <a href="#downloads" class="btn btn-primary">{nav_dl}</a>
        <a href="app/?lang={code}" class="btn btn-secondary">🚀 Launch in Browser (WASM, Experimental)</a>
        <a href="https://github.com/rbm13s/OniAnvil" target="_blank" rel="noopener" class="btn btn-secondary">⭐ Star on GitHub</a>
      </div>

      <div class="hero-preview">
        <img src="screenshots/main.png" alt="OniAnvil Studio Workspace">
      </div>
    </div>
  </section>

  <!-- Features Grid -->
  <section id="features" class="features-section">
    <div class="container">
      <div class="section-head">
        <h2 class="section-title">{nav_feat}</h2>
        <p class="section-desc">Designed for Indie Game Developers &amp; Pixel Artists</p>
      </div>

      <div class="features-grid">
        <div class="card">
          <div class="card-icon">🎬</div>
          <h3 class="card-title">Multi-Track Studio Timeline</h3>
          <p class="card-body">
            Independent layer tracks (V1, V2, V3...), layer visibility/locking, merge down, variable frame duration multipliers, linked frames, and loop tags.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">🧅</div>
          <h3 class="card-title">Configurable Onion Skinning</h3>
          <p class="card-body">
            Real-time multi-frame ghosting with independent previous and next frame tinting (red/blue), opacity falloff curves, and loop wrap-around support.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">🗺️</div>
          <h3 class="card-title">Dual Atlas-Centric Architecture</h3>
          <p class="card-body">
            The game sprite atlas texture is the single source of truth. Edit individual frames or switch seamlessly to the global atlas view while keeping tiles synchronized.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">📐</div>
          <h3 class="card-title">Sketch &amp; Reference Layer</h3>
          <p class="card-body">
            Load concept art or reference photos (PNG, JPEG, WebP) as an underlay with free pan, zoom, opacity, and smart color sampling with palette snapping.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">✏️</div>
          <h3 class="card-title">Precision Pixel Art Toolset</h3>
          <p class="card-body">
            Pixel-perfect line mode, custom brush stamping from selection, 3-click quadratic Bézier curves, solid flood-fill lasso, magic wand, and marching ants.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">🎨</div>
          <h3 class="card-title">Palette Swapping &amp; Skins</h3>
          <p class="card-body">
            Create character skin variants (Day/Night, Team colors, Elemental skins) with instant palette swapping. Import and export GPL, PAL, and HEX palette formats.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">⚡</div>
          <h3 class="card-title">Live FX &amp; RotSprite</h3>
          <p class="card-body">
            Color replacement with canvas/palette eyedropper, HSV adjustments, automatic outline generator, 3x3 convolution filters, and RotSprite free rotation.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">✂️</div>
          <h3 class="card-title">Sheet Slicer &amp; 9-Grid UI</h3>
          <p class="card-body">
            Auto-detect sprite boundaries from raw sheets, configure 9-slice scalable UI margins, and pack textures with power-of-two padding and border extrusion.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">📦</div>
          <h3 class="card-title">Game Engine Ready Export</h3>
          <p class="card-body">
            Export scaled PNGs with extruded borders to eliminate texture bleeding, generate animated GIFs, and produce standard sprite sheet JSON metadata.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">📺</div>
          <h3 class="card-title">35 Retro &amp; Modern Themes</h3>
          <p class="card-body">
            Immerse yourself with authentic vintage hardware aesthetics including CRT Amber, CRT Green, Famicom, Mega Drive, Game Boy, Commodore 64, and PSX.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">🎯</div>
          <h3 class="card-title">Pivot &amp; Socket Action Points</h3>
          <p class="card-body">
            Set custom origin rotation pivots and place named game socket points (e.g. muzzle flash, item grip, particle source) per frame with automatic metadata export.
          </p>
        </div>

        <div class="card">
          <div class="card-icon">🌐</div>
          <h3 class="card-title">Native Desktop &amp; WebAssembly</h3>
          <p class="card-body">
            Written in Rust for blistering performance on Windows, Linux, and macOS. A zero-install browser build is also available via WebAssembly (experimental status).
          </p>
        </div>
      </div>
    </div>
  </section>

  <!-- Tools Reference Section -->
  <section id="tools" class="tools-section">
    <div class="container">
      <div class="section-head">
        <h2 class="section-title">{nav_tools}</h2>
        <p class="section-desc">
          Comprehensive 15-tool precision pixel art suite with 1-column / 2-column dock layouts and instant swatch swapping (<kbd>X</kbd>).
        </p>
      </div>

      <div class="tools-table-wrap">
        <table class="tools-table">
          <thead>
            <tr>
              <th style="width: 220px;">Tool</th>
              <th style="width: 100px;">Shortcut</th>
              <th style="width: 200px;">Settings &amp; Options</th>
              <th>Description &amp; Workflow</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-pencil"></i></span>
                  <div>
                    <div>Pencil</div>
                    <small style="color: var(--text-muted);">Pencil Tool</small>
                  </div>
                </div>
              </td>
              <td><kbd>P</kbd></td>
              <td>
                <span class="tag-badge">1–64 px</span>
                <span class="tag-badge">Pixel-Perfect</span>
              </td>
              <td>The fundamental pixel art instrument. Pixel-Perfect mode algorithmically eliminates staircase corners and double pixels on diagonal strokes.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-eraser"></i></span>
                  <div>
                    <div>Eraser</div>
                    <small style="color: var(--text-muted);">Eraser Tool</small>
                  </div>
                </div>
              </td>
              <td><kbd>E</kbd></td>
              <td>
                <span class="tag-badge">1–64 px</span>
                <span class="tag-badge">Alpha Clear</span>
              </td>
              <td>Removes pixel data back to transparency on the active layer without altering adjacent locked or unlocked layers.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-eye-dropper"></i></span>
                  <div>
                    <div>Eyedropper</div>
                    <small style="color: var(--text-muted);">Pipette</small>
                  </div>
                </div>
              </td>
              <td><kbd>I</kbd> / <kbd>Alt</kbd></td>
              <td>
                <span class="tag-badge">Canvas / Sketch</span>
                <span class="tag-badge">Palette Snap</span>
              </td>
              <td>Samples pixel colors directly from the canvas or reference sketch. Left-click assigns Primary swatch; Right-click sets Secondary swatch.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-fill-drip"></i></span>
                  <div>
                    <div>Paint Bucket</div>
                    <small style="color: var(--text-muted);">Flood Fill</small>
                  </div>
                </div>
              </td>
              <td><kbd>G</kbd></td>
              <td>
                <span class="tag-badge">Tolerance 0–255</span>
                <span class="tag-badge">Contiguous / Global</span>
              </td>
              <td>Flood-fills contiguous color regions or replaces all matching color pixels across the frame with adjustable color tolerance.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-brush"></i></span>
                  <div>
                    <div>Gradient</div>
                    <small style="color: var(--text-muted);">Linear &amp; Radial</small>
                  </div>
                </div>
              </td>
              <td><kbd>U</kbd></td>
              <td>
                <span class="tag-badge">Linear / Radial</span>
                <span class="tag-badge">Dithering</span>
              </td>
              <td>Interactive two-point gradient interpolation between Primary and Secondary swatches with optional retro Bayer pattern dithering.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-slash"></i></span>
                  <div>
                    <div>Line</div>
                    <small style="color: var(--text-muted);">Bresenham Line</small>
                  </div>
                </div>
              </td>
              <td><kbd>V</kbd></td>
              <td>
                <span class="tag-badge">1–64 px</span>
                <span class="tag-badge">Shift-Snap</span>
              </td>
              <td>Draws straight pixel lines using Bresenham algorithm with live drag preview. Hold <kbd>Shift</kbd> to lock angles to 0°, 45°, and 90°.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-bezier-curve"></i></span>
                  <div>
                    <div>Bézier Curve</div>
                    <small style="color: var(--text-muted);">Quadratic Spline</small>
                  </div>
                </div>
              </td>
              <td><kbd>C</kbd></td>
              <td>
                <span class="tag-badge">1–64 px</span>
                <span class="tag-badge">3-Point Spline</span>
              </td>
              <td>Three-point quadratic Bézier curve: click to establish start point, drag to end point, and drag middle control node for silky organic curves.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-square"></i></span>
                  <div>
                    <div>Rectangle</div>
                    <small style="color: var(--text-muted);">Geometric Box</small>
                  </div>
                </div>
              </td>
              <td><kbd>R</kbd></td>
              <td>
                <span class="tag-badge">Outline / Solid / Both</span>
                <span class="tag-badge">Shift-Square</span>
              </td>
              <td>Rasterizes boxes and rectangles. Toggle stroke outline, solid color fill, or filled with border. Holding <kbd>Shift</kbd> constrains to a square.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-circle"></i></span>
                  <div>
                    <div>Ellipse</div>
                    <small style="color: var(--text-muted);">Midpoint Circle</small>
                  </div>
                </div>
              </td>
              <td><kbd>O</kbd></td>
              <td>
                <span class="tag-badge">Outline / Solid / Both</span>
                <span class="tag-badge">Shift-Circle</span>
              </td>
              <td>Midpoint ellipse and circle rasterizer. Toggle outline or solid fill modes. Holding <kbd>Shift</kbd> forces an equidistant circle.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-chess-board"></i></span>
                  <div>
                    <div>Dither Brush</div>
                    <small style="color: var(--text-muted);">Pattern Shader</small>
                  </div>
                </div>
              </td>
              <td><kbd>D</kbd></td>
              <td>
                <span class="tag-badge">Bayer 2×2 / 4×4 / 8×8</span>
                <span class="tag-badge">Checker / Diagonal</span>
              </td>
              <td>Authentic retro hardware shading alternating primary and secondary pixels according to classic Bayer and custom matrices.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-circle-half-stroke"></i></span>
                  <div>
                    <div>Shading</div>
                    <small style="color: var(--text-muted);">Ramp Stepper</small>
                  </div>
                </div>
              </td>
              <td><kbd>H</kbd></td>
              <td>
                <span class="tag-badge">Lighten / Darken</span>
                <span class="tag-badge">Palette Ramps</span>
              </td>
              <td>Steps pixel colors up or down within color ramps defined in the current palette. Ideal for non-destructive highlights and shadows.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-vector-square"></i></span>
                  <div>
                    <div>Rect Select</div>
                    <small style="color: var(--text-muted);">Marquee Selection</small>
                  </div>
                </div>
              </td>
              <td><kbd>M</kbd></td>
              <td>
                <span class="tag-badge">Marching Ants</span>
                <span class="tag-badge">Clipboard</span>
              </td>
              <td>Defines rectangular pixel bounding boxes with animated marching ants boundary. Supports clipboard copy, cut, paste, and move.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-draw-polygon"></i></span>
                  <div>
                    <div>Lasso Select</div>
                    <small style="color: var(--text-muted);">Polygonal Lasso</small>
                  </div>
                </div>
              </td>
              <td><kbd>L</kbd></td>
              <td>
                <span class="tag-badge">Freehand Polygon</span>
                <span class="tag-badge">Auto-Close</span>
              </td>
              <td>Freeform polygonal selection tool. Draw organic boundaries around complex shapes, characters, and props for isolated manipulation.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-wand-magic-sparkles"></i></span>
                  <div>
                    <div>Magic Wand</div>
                    <small style="color: var(--text-muted);">Color Selector</small>
                  </div>
                </div>
              </td>
              <td><kbd>W</kbd></td>
              <td>
                <span class="tag-badge">Tolerance 0–255</span>
                <span class="tag-badge">Contiguous / Global</span>
              </td>
              <td>Selects color-contiguous or canvas-wide pixel clusters matching clicked pixel color within an adjustable RGB tolerance.</td>
            </tr>

            <tr>
              <td>
                <div class="tool-identity">
                  <span class="tool-icon-badge"><i class="fa-solid fa-crosshairs"></i></span>
                  <div>
                    <div>Pivot &amp; Sockets</div>
                    <small style="color: var(--text-muted);">Action Points</small>
                  </div>
                </div>
              </td>
              <td><kbd>Y</kbd></td>
              <td>
                <span class="tag-badge">Rotation Pivot</span>
                <span class="tag-badge">Named Sockets</span>
              </td>
              <td>Defines frame rotation pivot center and assigns custom named socket points (e.g. <code>muzzle</code>, <code>grip</code>, <code>sparks</code>, <code>mount</code>).</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </section>

  <!-- Showcase Section -->
  <section id="showcase" class="showcase-section">
    <div class="container">
      <div class="section-head">
        <h2 class="section-title">{nav_show}</h2>
        <p class="section-desc">A deep dive into the user interface and key creative modules.</p>
      </div>

      <div class="tabs-nav">
        <button class="tab-btn active" onclick="switchTab(0)">{t0}</button>
        <button class="tab-btn" onclick="switchTab(1)">{t1}</button>
        <button class="tab-btn" onclick="switchTab(2)">{t2}</button>
        <button class="tab-btn" onclick="switchTab(3)">{t3}</button>
        <button class="tab-btn" onclick="switchTab(4)">{t4}</button>
        <button class="tab-btn" onclick="switchTab(5)">{t5}</button>
        <button class="tab-btn" onclick="switchTab(6)">{t6}</button>
        <button class="tab-btn" onclick="switchTab(7)">{t7}</button>
        <button class="tab-btn" onclick="switchTab(8)">{t8}</button>
        <button class="tab-btn" onclick="switchTab(9)">{t9}</button>
        <button class="tab-btn" onclick="switchTab(10)">{t10}</button>
        <button class="tab-btn" onclick="switchTab(11)">{t11}</button>
      </div>

      <!-- Tab 0: Workspace -->
      <div class="tab-content active">
        <div class="screenshot-showcase">
          <img src="screenshots/main.png" alt="OniAnvil Main Workspace">
          <div class="caption">OniAnvil main workspace featuring multi-track studio timeline, viewport canvas with pixel grid, and dock panels.</div>
        </div>
      </div>

      <!-- Tab 1: Tools & Toolbar -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/tools.png" alt="Expanded 2-column tools panel">
            <div class="caption">Expanded 2-column tools dock with icons, tool labels, and hotkeys.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/toolbar.png" alt="Horizontal context toolbar">
            <div class="caption">Top toolbar with tool options, brush size, pixel-perfect toggle, and view modes.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/tools_compact.png" alt="Compact 1-column slim dock">
            <div class="caption">Compact 1-column slim dock for maximum canvas drawing area.</div>
          </div>
        </div>
      </div>

      <!-- Tab 2: Timeline & Layers -->
      <div class="tab-content">
        <div class="screenshot-showcase">
          <img src="screenshots/timeline.png" alt="Studio Multi-Track Timeline">
          <div class="caption">Studio multi-track timeline with independent layer tracks (V1, V2...), frame duration multipliers, visibility/lock toggles, and playback transport.</div>
        </div>
      </div>

      <!-- Tab 3: Live Preview -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/live_preview.png" alt="Floating Live Animation Preview">
            <div class="caption">Floating live playback preview window with real-time FPS counter and zoom controls.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/live_preview_scroll_test.png" alt="Large Sprite Test in Preview">
            <div class="caption">Preview window supporting smooth scrolling and zooming for large multi-frame animation sequences.</div>
          </div>
        </div>
      </div>

      <!-- Tab 4: Palette & Swaps -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/palette_and_swaps.png" alt="Palette Editor and Color Swaps">
            <div class="caption">Palette editor dock with HSV/RGB sliders, color swaps, and GPL/HEX import/export.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/remap_to_palette.png" alt="Palette Remapping Dialog">
            <div class="caption">Global palette remapping dialog for snapping arbitrary sprite colors to index-constrained palettes.</div>
          </div>
        </div>
      </div>

      <!-- Tab 5: Pivot & Action Points -->
      <div class="tab-content">
        <div class="screenshot-showcase">
          <img src="screenshots/pivot_and_action_points.png" alt="Pivot and Socket Action Points">
          <div class="caption">Interactive rotation pivot point and named socket attachment points (muzzle, hand, mount) with coordinates export.</div>
        </div>
      </div>

      <!-- Tab 6: Sketch & Onion Skin -->
      <div class="tab-content">
        <div class="screenshot-showcase">
          <img src="screenshots/sketch_and_onion_skin.png" alt="Sketch Underlay and Onion Skinning">
          <div class="caption">Reference sketch tracing underlay with opacity slider, along with multi-frame onion skin ghosting.</div>
        </div>
      </div>

      <!-- Tab 7: Slicer & 9-Slice UI -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/slicer.png" alt="Sheet Slicer Tool">
            <div class="caption">Automatic sprite boundary detection and interactive sheet slicer.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/3x3.png" alt="9-Slice Scalable UI Margin Editor">
            <div class="caption">9-slice scalable UI margin editor for buttons, dialog boxes, and HUD components.</div>
          </div>
        </div>
      </div>

      <!-- Tab 8: Live FX & Filters -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/outline_generator.png" alt="Pixel Outline Generator Modal">
            <div class="caption">Automatic pixel outline generator modal with customizable thickness and diagonal connection modes.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/replace_color.png" alt="Interactive Color Replacement Modal">
            <div class="caption">Interactive color replacement modal with canvas dropper and live preview.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/hue_saturation_contrast.png" alt="HSV and Contrast Adjustment Dialog">
            <div class="caption">Hue shift, saturation, and contrast adjustment dialog.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/matrix_filter.png" alt="3x3 Convolution Matrix Filter Modal">
            <div class="caption">3x3 convolution matrix filter modal featuring sharpen, blur, emboss, and edge detection presets.</div>
          </div>
        </div>
      </div>

      <!-- Tab 9: 35 Retro Themes -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/themes.png" alt="Theme Selection Menu">
            <div class="caption">Theme selection menu featuring 35 retro gaming and modern studio themes.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/crt_orange_theme.png" alt="Retro CRT Amber Phosphor Theme">
            <div class="caption">Retro CRT Amber/Orange phosphor terminal theme.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/psx_theme.png" alt="PlayStation Retro Aesthetic Theme">
            <div class="caption">Classic 90s PlayStation (PSX) console aesthetic theme.</div>
          </div>
        </div>
      </div>

      <!-- Tab 10: Viewports & Backgrounds -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/grid_and_dark_checkerboard.png" alt="Dark Checkerboard with Grid">
            <div class="caption">High-contrast pixel grid over dark checkerboard pattern.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/no_grid_light_checkerboard.png" alt="Light Checkerboard without Grid">
            <div class="caption">Clean borderless canvas over light checkerboard pattern.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/custom_solid_background.png" alt="Custom Solid Background Color">
            <div class="caption">Custom solid color background for testing in-game sprite contrast.</div>
          </div>
        </div>
      </div>

      <!-- Tab 11: Menus & Dialogs -->
      <div class="tab-content">
        <div class="screenshot-gallery">
          <div class="gallery-item">
            <img src="screenshots/file_menu.png" alt="File Menu">
            <div class="caption">File menu: project management, format imports, and game engine exports.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/edit_menu.png" alt="Edit Menu">
            <div class="caption">Edit menu: clipboard operations, undo/redo history, and color transforms.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/select_menu.png" alt="Selection Menu">
            <div class="caption">Selection menu: invert, clear, shrink, and expand selections.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/view_menu.png" alt="View Menu">
            <div class="caption">View menu: rulers, guides, checkerboards, and zoom presets.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/project_menu.png" alt="Project Menu">
            <div class="caption">Project menu: canvas resizing, sprite trimming, and atlas sync.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/new_project.png" alt="New Project Modal">
            <div class="caption">New project modal with standard retro and modern canvas size presets.</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/new_project_palettes.png" alt="Built-in Palettes in New Project">
            <div class="caption">Built-in retro palettes in project creation (PICO-8, Game Boy, NES, Commodore 64).</div>
          </div>
          <div class="gallery-item">
            <img src="screenshots/help.png" alt="Help and Hotkeys Reference Window">
            <div class="caption">Interactive Help dialog with full keyboard shortcuts and documentation.</div>
          </div>
        </div>
      </div>

    </div>
  </section>

  <!-- Shortcuts Section -->
  <section id="shortcuts" class="shortcuts-section">
    <div class="container">
      <div class="section-head">
        <h2 class="section-title">{nav_keys}</h2>
        <p class="section-desc">Optimized for rapid one-handed muscle memory.</p>
      </div>

      <table class="shortcuts-table">
        <thead>
          <tr>
            <th>Shortcut</th>
            <th>Tool / Action</th>
            <th>Description</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td><kbd>P</kbd></td>
            <td>Pencil</td>
            <td>1px precision drawing with optional Pixel-Perfect mode</td>
          </tr>
          <tr>
            <td><kbd>E</kbd></td>
            <td>Eraser</td>
            <td>Clear pixels back to transparency on active layer</td>
          </tr>
          <tr>
            <td><kbd>I</kbd> / <kbd>Alt</kbd></td>
            <td>Eyedropper</td>
            <td>Sample color from active canvas or reference sketch</td>
          </tr>
          <tr>
            <td><kbd>G</kbd></td>
            <td>Paint Bucket</td>
            <td>Flood fill contiguous or global color region with tolerance</td>
          </tr>
          <tr>
            <td><kbd>U</kbd></td>
            <td>Gradient</td>
            <td>Linear and radial gradients with dithering support</td>
          </tr>
          <tr>
            <td><kbd>V</kbd></td>
            <td>Line</td>
            <td>Straight lines with <kbd>Shift</kbd> angle constraint</td>
          </tr>
          <tr>
            <td><kbd>C</kbd></td>
            <td>Bézier Curve</td>
            <td>Smooth 3-point quadratic spline curve</td>
          </tr>
          <tr>
            <td><kbd>R</kbd></td>
            <td>Rectangle</td>
            <td>Outlined or solid rectangular shapes (<kbd>Shift</kbd> for square)</td>
          </tr>
          <tr>
            <td><kbd>O</kbd></td>
            <td>Ellipse</td>
            <td>Outlined or solid ellipses (<kbd>Shift</kbd> for perfect circle)</td>
          </tr>
          <tr>
            <td><kbd>D</kbd></td>
            <td>Dither Brush</td>
            <td>Pattern dither drawing with Bayer and checker matrices</td>
          </tr>
          <tr>
            <td><kbd>H</kbd></td>
            <td>Shading</td>
            <td>Step lighter or darker through palette color ramps</td>
          </tr>
          <tr>
            <td><kbd>M</kbd></td>
            <td>Rectangular Select</td>
            <td>Select rectangular region with marching ants boundary</td>
          </tr>
          <tr>
            <td><kbd>L</kbd></td>
            <td>Lasso Select</td>
            <td>Freehand polygonal selection with auto-closing loop</td>
          </tr>
          <tr>
            <td><kbd>W</kbd></td>
            <td>Magic Wand</td>
            <td>Select color-contiguous pixel regions within tolerance</td>
          </tr>
          <tr>
            <td><kbd>Y</kbd></td>
            <td>Pivot &amp; Sockets</td>
            <td>Place rotation pivot point and named attachment sockets</td>
          </tr>
          <tr>
            <td><kbd>X</kbd></td>
            <td>Swap Swatches</td>
            <td>Swap Primary and Secondary active color swatches</td>
          </tr>
          <tr>
            <td><kbd>Ctrl+C / X / V</kbd></td>
            <td>Clipboard</td>
            <td>Copy, Cut, and Paste selected pixel clusters across frames</td>
          </tr>
          <tr>
            <td><kbd>Space</kbd></td>
            <td>Play / Pause</td>
            <td>Toggle real-time animation playback in timeline</td>
          </tr>
          <tr>
            <td><kbd>A / D</kbd></td>
            <td>Step Frames</td>
            <td>Step to Previous or Next animation frame</td>
          </tr>
          <tr>
            <td><kbd>Shift+R</kbd></td>
            <td>Replace Color</td>
            <td>Modal to swap colors with live preview and pipette</td>
          </tr>
          <tr>
            <td><kbd>Ctrl+U</kbd></td>
            <td>Color Adjust</td>
            <td>HSV hue shift, saturation, and contrast adjustment</td>
          </tr>
          <tr>
            <td><kbd>Ctrl+R</kbd></td>
            <td>Free Rotate</td>
            <td>Arbitrary angle rotation using RotSprite algorithm</td>
          </tr>
          <tr>
            <td><kbd>Space+Drag</kbd></td>
            <td>Pan Canvas</td>
            <td>Smooth infinite panning across canvas coordinates</td>
          </tr>
          <tr>
            <td><kbd>Ctrl+Wheel</kbd></td>
            <td>Zoom</td>
            <td>Pixel-crisp zoom centered around mouse cursor</td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>

  <!-- Download Section -->
  <section id="downloads" class="download-section">
    <div class="container">
      <div class="section-head">
        <h2 class="section-title">{nav_dl}</h2>
        <p class="section-desc">Standalone zero-dependency desktop binaries and web client.</p>
      </div>

      <div class="download-grid">
        <div class="dl-card">
          <div class="dl-icon">🪟</div>
          <h3 class="dl-platform">Windows</h3>
          <p class="dl-desc">Windows 10 / 11 (64-bit)<br/>Standalone portable executable</p>
          <a href="https://github.com/rbm13s/OniAnvil/releases/latest" class="btn btn-primary btn-sm">Download .zip</a>
        </div>

        <div class="dl-card">
          <div class="dl-icon">🐧</div>
          <h3 class="dl-platform">Linux</h3>
          <p class="dl-desc">Ubuntu, Debian, Fedora, Arch<br/>Portable .tar.gz &amp; Debian .deb</p>
          <a href="https://github.com/rbm13s/OniAnvil/releases/latest" class="btn btn-primary btn-sm">Download .deb / .tar.gz</a>
        </div>

        <div class="dl-card">
          <div class="dl-icon">🍏</div>
          <h3 class="dl-platform">macOS</h3>
          <p class="dl-desc">Universal Binary (ARM64 M1-M4 + Intel x86_64)<br/>macOS 11.0 Big Sur or later</p>
          <a href="https://github.com/rbm13s/OniAnvil/releases/latest" class="btn btn-primary btn-sm">Download .zip</a>
        </div>

        <div class="dl-card">
          <div class="dl-icon">🌐</div>
          <h3 class="dl-platform">WebAssembly <span style="font-size:0.75rem; vertical-align: middle; background: rgba(245, 158, 11, 0.2); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.4); padding: 2px 6px; border-radius: 4px;">Experimental</span></h3>
          <p class="dl-desc">Run directly in Chrome, Firefox, Safari, Edge<br/>Zero installation required.<br/><span style="color: #f59e0b; font-size: 0.82rem;">⚠️ Note: Web build is experimental and may contain browser-specific bugs.</span></p>
          <a href="app/?lang={code}" class="btn btn-secondary btn-sm">Open in Browser</a>
        </div>
      </div>
    </div>
  </section>

  <!-- Footer -->
  <footer class="site-footer">
    <div class="container">
      <p>OniAnvil is an open-source project released under the <a href="https://opensource.org/licenses/MIT" target="_blank" rel="noopener">MIT License</a>.</p>
      <p style="margin-top: 8px;">Icons provided by <a href="https://fontawesome.com/" target="_blank" rel="noopener">Font Awesome Free</a> under <a href="https://creativecommons.org/licenses/by/4.0/" target="_blank" rel="noopener">CC BY 4.0</a> / <a href="https://scripts.sil.org/OFL" target="_blank" rel="noopener">SIL OFL 1.1</a>.</p>
      <p style="margin-top: 8px;">Built with ❤️ in Rust using <a href="https://github.com/emilk/egui" target="_blank" rel="noopener">egui</a> and <a href="https://wgpu.rs/" target="_blank" rel="noopener">wgpu</a>. Repository: <a href="https://github.com/rbm13s/OniAnvil" target="_blank" rel="noopener">github.com/rbm13s/OniAnvil</a></p>
      <p style="margin-top: 12px; font-size: 0.82rem; color: var(--text-muted); max-width: 860px; margin-left: auto; margin-right: auto; line-height: 1.45;">
        <strong>Disclaimer:</strong> OniAnvil is distributed free of charge on an "AS IS" basis without warranties of any kind. The authors and copyright holders bear no liability for any direct, indirect, or incidental damages or loss of data resulting from the use of this software.
      </p>
    </div>
  </footer>

</body>
</html>
"###,
            code = lang.code,
            title = lang.title,
            desc = lang.desc,
            nav_feat = nav_feat,
            nav_tools = nav_tools,
            nav_show = nav_show,
            nav_keys = nav_keys,
            nav_dl = nav_dl,
            dropdown = dropdown,
            t0 = tabs[0], t1 = tabs[1], t2 = tabs[2], t3 = tabs[3],
            t4 = tabs[4], t5 = tabs[5], t6 = tabs[6], t7 = tabs[7],
            t8 = tabs[8], t9 = tabs[9], t10 = tabs[10], t11 = tabs[11]
        );

        fs::write(&out_html_path, html_template).expect("Write localized HTML");
        println!("Generated {}", out_html_path);
    }

    println!("All 37 languages generated successfully!");
}
