use granola::{homemade::*, prelude::*, recipes::*};

pub fn a() -> HtmlA {
    HtmlA::new().content("docs").href("https://askama.rs")
}

pub fn abbr() -> HtmlAbbr {
    HtmlAbbr::new()
        .content("TMNT")
        .title("Teenage Mutant Ninja Turtles")
}

pub fn address() -> HtmlAddress {
    let mail = HtmlA::new()
        .content("contact@holmes.co.uk")
        .href("mailto:contact@holmes.co.uk");

    let content = bake_block!["221B Baker St, London NW1 6XE ·", mail];

    HtmlAddress::new().content(content)
}

pub fn area() -> HtmlArea {
    HtmlArea::new()
        .href("https://w.wiki/LTnF")
        .alt("Red triangle")
        .shape("poly")
        .coords("300,63,470,357,130,357")
}

pub fn article() -> HtmlArticle {
    let h2 = HtmlH2::new().content("New Café");

    let content = bake_block![
        "Oats &amp; Ends opened last week on Oak Street,
    at the corner of Elm Avenue, bringing new aromas to the block.",
        "Its cozy atmosphere draws in passersby looking to treat themselves to
    a cup or two of good, hot black coffee."
    ];

    let p = HtmlP::new().content(content);

    HtmlArticle::new().content(bake_block![h2, p])
}

pub fn aside() -> HtmlAside {
    let tip = HtmlStrong::new().content("Tip:");
    let content = HtmlP::new().content(bake![tip, " trust your senses more than the timer."]);

    HtmlAside::new().content(content).role("note")
}

pub fn audio() -> HtmlAudio {
    HtmlAudio::new().src("toms-dinner.mp3").autoplay(true)
}

pub fn b() -> String {
    let flour = HtmlB::new().content("flour");
    let water = HtmlB::new().content("water");
    let salt = HtmlB::new().content("salt");

    bake!["Mix ", flour, ", ", water, ", and ", salt, "."]
}

pub fn base() -> HtmlBase {
    HtmlBase::new().href("https://www.example.com")
}

pub fn bdi() -> String {
    let gal = HtmlBdi::new().content("גל גדות");

    bake![gal, " liked your post"]
}

pub fn bdo() -> HtmlBdo {
    HtmlBdo::new().content("looking-glass").dir("rtl")
}

pub fn blockquote() -> HtmlBlockquote {
    let content = bake_block![
        "The &lt;blockquote&gt; element indicates that the enclosed text is an extended quotation.",
        "Usually, this is rendered visually by indentation.",
        "A URL for the source of the quotation may be given using the cite attribute,",
        "while a text representation of the source can be given using the &lt;cite&gt; element.",
    ];

    let paragraph = HtmlP::new().content(content);

    HtmlBlockquote::new()
        .content(paragraph)
        .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote")
}

pub fn body() -> HtmlBody {
    HtmlBody::new().content("flow content")
}

pub fn br() -> HtmlP {
    let br = HtmlBr::new();

    let roses = bake!["Roses are red,", br];
    let violets = "Violets are blue.";

    HtmlP::new().content(bake_block![roses, violets])
}

pub fn button() -> HtmlButton {
    HtmlButton::new()
        .content("Add to favorites")
        .button_type(ButtonType::Button)
        .name("favorite")
}

pub fn canvas() -> String {
    let content = r#"
    const ctx = document.getElementById("canvas").getContext("2d");
    ctx.font = "64px sans";
    ctx.fillText(":-)", 10, 62);"#;

    let script = HtmlScript::new().content(content);

    let canvas = HtmlCanvas::new()
        .content("ASCII smiley")
        .id("canvas")
        .width(160)
        .height(80);

    bake_block![script, canvas]
}

pub fn caption() -> HtmlCaption {
    HtmlCaption::new().content("Our favorites, yours to try.")
}

pub fn cite() -> HtmlCite {
    HtmlCite::new().content("Act Without Words I")
}

pub fn code() -> HtmlCode {
    HtmlCode::new().content("todo!()")
}

pub fn col() -> String {
    let item = HtmlCol::new().class("item");
    let description = HtmlCol::new().class("description");

    bake_block![item, description]
}

pub fn colgroup() -> HtmlColgroup {
    let item = HtmlCol::new().class("item");
    let description = HtmlCol::new().class("description");

    HtmlColgroup::new().content([item, description])
}

pub fn data() -> HtmlData {
    HtmlData::new().content("$13.37").value("1337")
}

pub fn datalist() -> HtmlDatalist {
    let options = [
        HtmlOption::new().value("Chocolate"),
        HtmlOption::new().value("Strawberry"),
        HtmlOption::new().value("Vanilla"),
    ];

    HtmlDatalist::new().content(options).id("ice-cream-flavors")
}

pub fn dd() -> String {
    let dt = HtmlDt::new().content("Hiraeth");
    let dd =
        HtmlDd::new().content("A longing for a home that no longer exists, or perhaps never did.");

    bake_block![dt, dd]
}

pub fn del() -> HtmlDel {
    HtmlDel::new()
        .content("try!")
        .datetime("2019-11-07")
        .cite("https://github.com/rust-lang/rust/pull/62672")
}

pub fn details() -> HtmlDetails {
    let summary = HtmlSummary::new().content("Pandora's box");

    HtmlDetails::new().content(bake_block![summary, "Hope"])
}

pub fn dfn() -> HtmlP {
    let corro = HtmlDfn::new().content("Corro");

    let about = bake![corro, " the Unsafe Rusturchin"];

    HtmlP::new().content(about)
}

pub fn dialog() -> String {
    let open_button = HtmlButton::new()
        .content("open dialog")
        .popovertarget("modal_popover");
    let close_button = HtmlButton::new()
        .content("Close")
        .popovertarget("modal_popover")
        .popovertargetaction("hide");

    let dialog = HtmlDialog::new()
        .content(bake_block!["Hello, there!", close_button])
        .id("modal_popover")
        .popover("auto");

    bake_block![open_button, dialog]
}

pub fn div() -> HtmlDiv {
    let save = HtmlButton::new().content("Save");
    let cancel = HtmlButton::new()
        .content("Cancel")
        .button_type(ButtonType::Button);

    let content = bake_block![save, cancel];

    HtmlDiv::new()
        .content(content)
        .class("flex justify-end gap-2")
}

pub fn dl() -> HtmlDl {
    let dt_1 = HtmlDt::new().content("Hiraeth");
    let dd_1 =
        HtmlDd::new().content("A longing for a home that no longer exists, or perhaps never did.");

    let group_1 = bake_block![dt_1, dd_1];

    let dt_2 = HtmlDt::new().content("Pålegg");
    let dd_2 = HtmlDd::new().content("Anything and everything you might put on a slice of bread.");

    let group_2 = bake_block![dt_2, dd_2];

    let list = bake_block![group_1, "", group_2];

    HtmlDl::new().content(list)
}

pub fn doctype() -> HtmlDoctype {
    HtmlDoctype::new()
}

pub fn dt() -> String {
    let dt = HtmlDt::new().content("Pålegg");
    let dd = HtmlDd::new().content("Anything and everything you might put on a slice of bread.");

    bake_block![dt, dd]
}

pub fn em() -> String {
    let owned = HtmlEm::new().content("owned");

    bake!["I never said he ", owned, " it."]
}

pub fn embed() -> HtmlEmbed {
    HtmlEmbed::new()
        .src("flower.png")
        .mime_type(MimeType::Png)
        .width(420)
        .height(420)
}

pub fn fieldset() -> HtmlFieldset {
    let legend = HtmlLegend::new().content("To be, or not to be?");
    let input = HtmlInput::new()
        .input_type(InputType::Checkbox)
        .id("chbx")
        .name("to-be")
        .value("dunno");
    let label = HtmlLabel::new()
        .content("That is the question")
        .for_id("chbx");

    HtmlFieldset::new().content(bake_block![legend, input, label])
}

pub fn figcaption() -> String {
    let code = HtmlCode::new().content(r#"function greet() print("hi!") end"#);

    let figcaption = HtmlFigcaption::new().content("Defining a function in Lua");

    bake_block![code, figcaption]
}

pub fn figure() -> HtmlFigure {
    let code = HtmlCode::new().content(r#"function greet() print("hi!") end"#);

    let figcaption = HtmlFigcaption::new().content("Defining a function in Lua");

    let content = bake_block![code, figcaption];

    HtmlFigure::new().content(content)
}

pub fn footer() -> HtmlFooter {
    let content = HtmlSmall::new().content("&copy; 2026 Oats &amp; Ends Café");
    let paragraph = HtmlP::new().content(content);

    HtmlFooter::new().content(paragraph)
}

pub fn form() -> HtmlForm {
    let input = HtmlInput::new().name("cast-wish");
    let label = HtmlLabel::new().content(bake_block!["Wish:", input]);
    let button = HtmlButton::new().content("Cast");

    HtmlForm::new()
        .content(bake_block![label, button])
        .method(FormMethod::Get)
}

pub fn h1() -> HtmlH1 {
    HtmlH1::new().content("The Rust Programming Language")
}

pub fn h2() -> HtmlH2 {
    HtmlH2::new().content("Error Handling")
}

pub fn h3() -> HtmlH3 {
    let panic = HtmlCode::new().content("panic!");

    let content = bake!["Unrecoverable Errors with ", panic];

    HtmlH3::new().content(content)
}

pub fn h4() -> HtmlH4 {
    HtmlH4::new().content("In fable and literature")
}

pub fn head() -> HtmlHead {
    let charset = HtmlMeta::new().charset();
    let viewport = HtmlMeta::new()
        .name("viewport")
        .content("width=device-width");
    let title = HtmlTitle::new().content("Document title");

    HtmlHead::new().content(bake_block![charset, viewport, title])
}

pub fn header() -> HtmlHeader {
    let logo = HtmlA::new().content("Oats &amp; Ends").href("/");

    HtmlHeader::new().content(logo)
}

pub fn hgroup() -> HtmlHgroup {
    let heading = HtmlH2::new().content("Capítulo VIII.");

    let subtitle = "Del buen suceso que el valeroso don Quijote tuvo en la espantable y
    jamás imaginada aventura de los molinos de viento, con otros sucesos
    dignos de felice recordación";

    let paragraph = HtmlP::new().content(subtitle);

    HtmlHgroup::new().content(bake_block![heading, paragraph])
}

pub fn hr() -> String {
    let p1 = HtmlP::new().content("She blew out the candle. The room went dark.");
    let p2 = HtmlP::new()
        .content("Morning came with birds and the smell of bread from somewhere below.");

    let hr = HtmlHr::new();

    bake_block![p1, "", hr, "", p2]
}

pub fn i() -> String {
    let voila = HtmlI::new().content("voilà");

    bake!["and ", voila, "!"]
}

pub fn iframe() -> HtmlIframe {
    HtmlIframe::new()
        .src("https://w.wiki/LJK7")
        .title("Pedestrians crossing an intersection.")
}

pub fn img() -> HtmlImg {
    HtmlImg::new()
        .src("profile.png")
        .alt("A mustachioed in a red cap and blue overalls")
}

pub fn input() -> HtmlInput {
    HtmlInput::new()
        .name("nickname")
        .minlength(4)
        .required(true)
}

pub fn ins() -> HtmlIns {
    HtmlIns::new()
        .content("?")
        .datetime("2016-11-10")
        .cite("https://blog.rust-lang.org/2016/11/10/Rust-1.13")
}

pub fn kbd() -> HtmlKbd {
    HtmlKbd::new().content("Enter")
}

pub fn label() -> HtmlLabel {
    let input = HtmlInput::new()
        .input_type(InputType::Checkbox)
        .name("reality-check")
        .disabled(true);

    HtmlLabel::new().content(bake_block!["We're so back", input])
}

pub fn legend() -> HtmlLegend {
    HtmlLegend::new().content("Choose your favorite spoon")
}

pub fn li() -> String {
    let sugar = HtmlLi::new().content("sugar");
    let spice = HtmlLi::new().content("spice");

    bake_block![sugar, spice]
}

pub fn link() -> HtmlLink {
    HtmlLink::new().href("fancy.css").rel("stylesheet")
}

pub fn main() -> HtmlMain {
    HtmlMain::new().content("hello, world!")
}

pub fn map() -> String {
    let img = HtmlImg::new()
        .src("mg_flag.png")
        .alt("MG flag")
        .width(600)
        .height(420)
        .usemap("#minas-gerais");

    let area = HtmlArea::new()
        .href("https://w.wiki/LTnF")
        .alt("Red triangle")
        .shape("poly")
        .coords("300,63,470,357,130,357");

    let map = HtmlMap::new().content(area).name("minas-gerais");

    bake_block![img, map]
}

pub fn mark() -> String {
    let but_the_clouds = HtmlMark::new().content("but the clouds");

    let br = HtmlBr::new();

    bake_block![
        bake!["Seem ", but_the_clouds, " of the sky"],
        br,
        "When the horizon fades;",
        br,
        "Or a bird's sleepy cry",
        br,
        "Among the deepening shades."
    ]
}

pub fn menu() -> HtmlMenu {
    let items = [
        HtmlLi::new().content("Buy"),
        HtmlLi::new().content("Use"),
        HtmlLi::new().content("Break"),
        HtmlLi::new().content("Fix"),
    ];

    HtmlMenu::new().content(items)
}

pub fn meta() -> HtmlMeta {
    HtmlMeta::new().content("noindex, nofollow").name("robots")
}

pub fn meter() -> HtmlMeter {
    HtmlMeter::new()
        .content("12%")
        .value(12.)
        .min(0.)
        .max(100.)
        .low(20.)
        .high(60.)
        .optimum(80.)
}

pub fn nav() -> HtmlNav {
    let location = HtmlA::new()
        .content("Oak Street, corner of Elm Avenue")
        .href("/location");
    let menu = HtmlA::new().content("the menu").href("/menu");
    let note = HtmlA::new().content("note").href("/contact");

    let content = bake_block![
        bake!["You can find us at ", location, "."],
        bake!["Everything we make and love is on ", menu, "."],
        bake!["Have a thought? Send us a ", note, "."],
    ];

    let p = HtmlP::new().content(content);

    HtmlNav::new().content(p).aria_label("Site navigation")
}

pub fn noscript() -> HtmlNoscript {
    HtmlNoscript::new().content("It's javascript all the way down")
}

pub fn object() -> HtmlObject {
    HtmlObject::new()
        .mime_type(MimeType::Mp4)
        .data("/videos/flower.mp4")
        .width(420)
        .height(420)
}

pub fn ol() -> HtmlOl {
    let items = [
        HtmlLi::new().content("Add the sugar"),
        HtmlLi::new().content("Coat with spice"),
        HtmlLi::new().content("Fold in everything nice"),
    ];

    HtmlOl::new().content(items)
}

pub fn optgroup() -> HtmlOptgroup {
    let options = [
        HtmlOption::new().content("Grape"),
        HtmlOption::new().content("Mango"),
        HtmlOption::new().content("Strawberry"),
    ];

    HtmlOptgroup::new().content(options).label("Fruits")
}

pub fn option() -> HtmlOption {
    HtmlOption::new().content("Chocolate").value("chocolate")
}

pub fn output() -> HtmlOutput {
    HtmlOutput::new()
        .content("42")
        .name("answer")
        .for_id("ultimate-question")
}

pub fn p() -> HtmlP {
    HtmlP::new().content("Lorem ipsum dolor sit amet...🙄")
}

pub fn picture() -> HtmlPicture {
    let source = HtmlSource::new()
        .srcset("logo-wide.png")
        .media("(width >= 600px)");
    let img = HtmlImg::new().src("logo-narrow.png").alt("logo");

    HtmlPicture::new().content(bake_block![source, img])
}

pub fn pre() -> String {
    let ferris_ascii = r#"
 __________________________
&lt; Hello fellow Rustaceans! &gt;
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;

    let pre = HtmlPre::new()
        .content(ferris_ascii)
        .role("img")
        .aria_label("ASCII ferris");

    let url = HtmlA::new()
        .content("ferris-says")
        .href(r#"https://crates.io/crates/ferris-says"#);
    let cite = HtmlCite::new().content(url);

    bake_block![pre, cite]
}

pub fn progress() -> HtmlProgress {
    HtmlProgress::new()
        .content("10/300")
        .id("experience")
        .max(300.)
        .value(10.)
}

pub fn q() -> HtmlQ {
    HtmlQ::new()
        .content("This element is intended for short quotations")
        .cite("https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q")
}

pub fn root() -> HtmlRoot {
    let body = HtmlBody::new().content("flow content");

    HtmlRoot::new().content(body).lang("en")
}

pub fn rp() -> String {
    let opening_rp = HtmlRp::new().content("(");
    let rt = HtmlRt::new().content("tori");
    let closing_rp = HtmlRp::new().content(")");

    bake![opening_rp, rt, closing_rp]
}

pub fn rt() -> HtmlRt {
    HtmlRt::new().content("tori")
}

pub fn ruby() -> HtmlRuby {
    let opening_rp = HtmlRp::new().content("(");
    let rt = HtmlRt::new().content("とり");
    let closing_rp = HtmlRp::new().content(")");

    let tori = bake!["鳥", opening_rp, rt, closing_rp];

    HtmlRuby::new().content(tori)
}

pub fn s() -> String {
    let nine = HtmlS::new().content("nine");

    bake!["Our solar system has ", nine, " eight planets"]
}

pub fn samp() -> HtmlSamp {
    HtmlSamp::new().content("No such file or directory")
}

pub fn script() -> HtmlScript {
    HtmlScript::new().content(r#"alert("Hello, world!");"#)
}

pub fn search() -> HtmlSearch {
    let label = HtmlLabel::new()
        .content("Search the haystack")
        .for_id("query");
    let input = HtmlInput::new()
        .input_type(InputType::Search)
        .id("query")
        .name("q")
        .placeholder("needle");
    let button = HtmlButton::new().content("Search");

    let form = HtmlForm::new()
        .content(bake_block![label, input, button])
        .action("/search");

    HtmlSearch::new().content(form)
}

pub fn section() -> HtmlSection {
    let h2 = HtmlH2::new().content("Latest news");
    let ul = HtmlUl::new().content(HtmlLi::new().content("New café on Oak Street"));

    HtmlSection::new().content(bake_block![h2, ul])
}

pub fn select() -> HtmlSelect {
    let opt_1 = HtmlOption::new().content("Salmon").value("salmon");
    let opt_2 = HtmlOption::new().content("Turbot").value("turbot");

    HtmlSelect::new()
        .content(bake_block![opt_1, opt_2])
        .name("fishes")
}

pub fn slot() -> HtmlSlot {
    HtmlSlot::new().name("who")
}

pub fn small() -> HtmlSmall {
    let unlicense = "This is free and unencumbered software released into the public domain.";

    HtmlSmall::new().content(unlicense)
}

pub fn source() -> HtmlSource {
    HtmlSource::new()
        .src("/videos/flower.mp4")
        .mime_type(MimeType::Mp4)
}

pub fn span() -> HtmlSpan {
    HtmlSpan::new()
        .content("aesthetic")
        .class("tracking-widest")
}

pub fn strong() -> HtmlStrong {
    HtmlStrong::new().content("Do not feed the trolls.")
}

pub fn style() -> HtmlStyle {
    let css_rule = CssRule::new()
        .selectors_list("p")
        .declarations_block([("color", "violet"), ("font-weight", "lighter")]);

    HtmlStyle::new().content(css_rule)
}

pub fn sub() -> String {
    let sub = HtmlSub::new().content("2");

    bake!["H", sub, "O"]
}

pub fn summary() -> HtmlSummary {
    HtmlSummary::new().content("Don't forget")
}

pub fn sup() -> String {
    let sup = HtmlSup::new().content("e");

    bake!["100", sup, " anniversaire"]
}

pub fn table() -> HtmlTable {
    let caption = HtmlCaption::new().content("Our favorites, yours to try.");

    let col_1 = HtmlCol::new().class("item");
    let col_2 = HtmlCol::new().class("description");

    let colgroup = HtmlColgroup::new().content([col_1, col_2]);

    let head_1 = HtmlTh::new().content("Item").scope("col");
    let head_2 = HtmlTh::new().content("Description").scope("col");

    let tr_head = HtmlTr::new().content(bake_block![head_1, head_2]);

    let thead = HtmlThead::new().content(tr_head);

    let th_1 = HtmlTh::new().content("Black coffee").scope("row");
    let td_1 = HtmlTd::new().content("A good, hot, black coffee");

    let black_coffee = HtmlTr::new().content(bake_block![th_1, td_1]);

    let th_2 = HtmlTh::new().content("Hot chocolate").scope("row");
    let td_2 = HtmlTd::new().content("Melted dark chocolate with milk");

    let hot_chocolate = HtmlTr::new().content(bake_block![th_2, td_2]);

    let tbody = HtmlTbody::new().content([black_coffee, hot_chocolate]);

    let td_foot = HtmlTd::new()
        .content("Don't see what you're after? We'll do our best.")
        .colspan(2);
    let tr_foot = HtmlTr::new().content(td_foot);

    let tfoot = HtmlTfoot::new().content(tr_foot);

    let content = bake_block![caption, colgroup, thead, tbody, tfoot,];

    HtmlTable::new().content(content)
}

pub fn tbody() -> HtmlTbody {
    let th_1 = HtmlTh::new().content("Black coffee").scope("row");
    let td_1 = HtmlTd::new().content("A good, hot, black coffee");

    let black_coffee = HtmlTr::new().content(bake_block![th_1, td_1]);

    let th_2 = HtmlTh::new().content("Hot chocolate").scope("row");
    let td_2 = HtmlTd::new().content("Melted dark chocolate with milk");

    let hot_chocolate = HtmlTr::new().content(bake_block![th_2, td_2]);

    HtmlTbody::new().content([black_coffee, hot_chocolate])
}

pub fn td() -> HtmlTd {
    HtmlTd::new().content("Melted dark chocolate with milk")
}

pub fn template() -> HtmlTemplate {
    let knock_knock = HtmlP::new().content("Knock knock.");
    let who_s_there = HtmlP::new().content("Who's there?");

    let name_slot = HtmlSlot::new().name("setup");
    let name_p1 = HtmlP::new().content(bake![name_slot, "."]);
    let name_p2 = HtmlP::new().content(bake![name_slot, " who?"]);

    let punchline_slot = HtmlSlot::new().name("punchline");
    let punchline = HtmlP::new().content(punchline_slot);

    let content = bake_block![knock_knock, who_s_there, name_p1, name_p2, punchline];

    HtmlTemplate::new().content(content).id("tmpl")
}

pub fn textarea() -> HtmlTextarea {
    HtmlTextarea::new()
        .content("Carpe diem")
        .name("reminder")
        .readonly(true)
}

pub fn tfoot() -> HtmlTfoot {
    let td = HtmlTd::new()
        .content("Don't see what you're after? We'll do our best.")
        .colspan(2);
    let tr = HtmlTr::new().content(td);

    HtmlTfoot::new().content(tr)
}

pub fn th() -> HtmlTh {
    HtmlTh::new().content("Hot chocolate").scope("row")
}

pub fn thead() -> HtmlThead {
    let item = HtmlTh::new().content("Item").scope("col");
    let description = HtmlTh::new().content("Description").scope("col");

    let tr = HtmlTr::new().content(bake_block![item, description]);

    HtmlThead::new().content(tr)
}

pub fn time() -> HtmlTime {
    HtmlTime::new()
        .content("Unix epoch")
        .datetime("1970-01-01T00:00:00Z")
}

pub fn title() -> HtmlTitle {
    HtmlTitle::new().content("On the unabashed art of self-referential examples")
}

pub fn tr() -> HtmlTr {
    let th = HtmlTh::new().content("Hot chocolate").scope("row");
    let td = HtmlTd::new().content("Melted dark chocolate with milk");

    HtmlTr::new().content(bake_block![th, td])
}

pub fn track() -> HtmlTrack {
    HtmlTrack::new()
        .src("der_himmel_uber_berlin.vtt")
        .kind("captions")
        .enabled(true)
}

pub fn u() -> String {
    let wowwd = HtmlU::new().content("world");

    bake!["hewwo, ", wowwd, "!"]
}

pub fn ul() -> HtmlUl {
    let items = [
        HtmlLi::new().content("sugar"),
        HtmlLi::new().content("spice"),
        HtmlLi::new().content("everything nice"),
    ];

    HtmlUl::new().content(items)
}

pub fn var() -> String {
    let var = HtmlVar::new().content("a");

    bake!["An equilateral triangle with side ", var]
}

pub fn video() -> HtmlVideo {
    HtmlVideo::new()
        .src("Never_Gonna_Give_You_Up.mp4")
        .width(800)
        .height(600)
        .autoplay(true)
}

pub fn wbr() -> HtmlWbr {
    HtmlWbr::new().id("line_break_opportunity")
}

pub fn at_rule() -> CssAtRule {
    let block = bake_block![
        "from { transform: translateX(0%); }",
        "to { transform: translateX(100%); }",
    ];

    CssAtRule::new()
        .identifier("keyframes")
        .rule("slide-in")
        .block(block)
}

pub fn declaration() -> CssDeclaration {
    CssDeclaration::new("color", "rebeccapurple")
}

pub fn declarations_block() -> CssDeclarationsBlock {
    CssDeclarationsBlock::new()
        .push(("color", "violet"))
        .push(("font-weight", "lighter"))
}

pub fn rule() -> CssRule {
    let css_selector = CssSimpleSelector::new().selector("p");
    let css_selector_list = CssSelectorsList::new().push(css_selector);

    let css_declaration = CssDeclaration::new("color", "rgb(102, 51, 153)");
    let css_declarations_block = CssDeclarationsBlock::new().push(css_declaration);

    CssRule::new()
        .selectors_list(css_selector_list)
        .declarations_block(css_declarations_block)
}

pub fn stylesheet() -> CssStylesheet {
    let at_rule = CssAtRule::new()
        .identifier("import")
        .rule(r#"url("layout.css")"#);

    let rule = CssRule::new()
        .selectors_list("p")
        .declarations_block(("color", "rebeccapurple"));

    CssStylesheet::new().push(at_rule).push(rule)
}

pub fn simple_selector() -> CssSimpleSelector {
    CssSimpleSelector::new().selector("p")
}

pub fn type_selector() -> CssTypeSelector {
    CssTypeSelector::new().selector("a").namespace("svg")
}

pub fn compound_selector() -> CssCompoundSelector {
    CssCompoundSelector::new()
        .type_selector("col")
        .push(".highlighted")
}

pub fn complex_selector() -> CssComplexSelector {
    CssComplexSelector::new().first("form").child("input")
}

pub fn selectors_list() -> CssSelectorsList {
    let selector = CssSimpleSelector::new().selector("p");

    CssSelectorsList::new().push(selector)
}

pub fn stylesheet_preflight() -> CssStylesheet<Preflight> {
    CssStylesheet::from(Preflight)
}

pub fn document_homemade() -> HtmlDocument<Homemade> {
    let meta = HtmlMeta::from(NameRobots).content("noindex, nofollow");
    let title = HtmlTitle::new().content("Home");

    let css_rule = CssRule::new()
        .selectors_list("body")
        .declarations_block([("height", "100vh"), ("margin", "0")]);
    let style = HtmlStyle::new().content(css_rule);

    let body = HtmlBody::new().content("Hello, world!");

    HtmlDocument::from(Homemade)
        .lang("en")
        .push_meta(meta)
        .push_title(title)
        .push_style(style)
        .body(body)
}
