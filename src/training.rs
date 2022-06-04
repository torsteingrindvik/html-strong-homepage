use axum::response::Html;
use html_strong::document_tree::Node;
use html_strong::{science_lab::NodeExt, tags::*};
use reqwest::StatusCode;

use crate::base::html_doc;
use crate::common::render;

#[derive(Debug)]
struct Section {
    title: String,
    paragraphs: Vec<String>,
}

impl Section {
    fn new(title: &'static str) -> Self {
        Self {
            title: title.into(),
            paragraphs: vec![],
        }
    }

    fn push(&mut self, paragraph: &'static str) {
        self.paragraphs.push(paragraph.into());
    }
}

#[derive(Debug)]
struct Article {
    sections: Vec<Section>,
}

impl Article {
    fn new() -> Self {
        Self { sections: vec![] }
    }

    fn p(mut self, text: &'static str) -> Self {
        let mut section = self.sections.pop().unwrap();
        section.push(text.into());
        self.sections.push(section);
        self
    }

    fn header(mut self, text: &'static str) -> Self {
        self.sections.push(Section::new(text));
        self
    }
}

impl NodeExt for Article {
    fn into_node(self) -> Node {
        let mut output = Div.id("training");

        for h in self.sections {
            let classes = "training-section breather-y";
            let mut section = Div.class(classes).kid(H2.text(h.title));

            for p in h.paragraphs {
                section.push_kid(P.text(p).class("lil-room"))
            }

            output.push_kid(section);
        }
        output
    }
}

#[derive(Debug, Clone)]
struct Source {
    title: String,
    url: String,
    contents: Node,
}

impl Source {
    fn new(title: &str, url: &str, contents: impl NodeExt) -> Self {
        Self {
            title: title.to_string(),
            url: url.to_string(),
            contents: contents.into_node(),
        }
    }
}

impl NodeExt for Source {
    fn into_node(self) -> Node {
        Div.kid(H1.text(self.title))
            .kid(
                P.text("Information found ")
                    .kid(A::href(&self.url).text("here"))
                    .text("."),
            )
            .kid(self.contents)
    }
}

fn huberman_podcast_with_andy_galpin() -> impl NodeExt {
    Article::new()
        .header("Adaptations of Exercise, Progressive Overload")
        .p("Needs progressive overload, stress. More weight, more reps, more often etc.")
        .header("Modifiable Variables, One-Rep Max, Muscle Soreness")
        .p("Exercise choice != adaptation. Sets, reps, rest time etc. affects adaptation.")
        .p(
            "One-rep max is a way to have a % of intensity for sets. E.g. 70% of one-rep-max for \
             8 reps.",
        )
        .p("Volume. Reps x sets.")
        .p(
            "Soreness: Bad indicator. Anyway on a soreness scale we want to be feeling like a 3 \
             out of 10.",
        )
        .p("Frequency is an important factor in adaptation.")
        .header("Modifiable Variables of Strength Training, Supersets")
        .p(
            "Larger range of motion is in general linked to more strength, hypertrophy. But of \
             course caveats, technique and safety is important.",
        )
        .p(
            "SAID: Specificity adaptation imposed demands. Or something like that. Some principle \
             about needing to go for _strength_ as an imposed demand to actually gain strength. \
             So more weight instead of more reps! That means high load.",
        )
        .p(
            "Warmup: 50% intensity high rep, work towards 75% intensity lower reps. Then go on to \
             e.g. 85% for work sets.",
        )
        .p(
            "Rest: 2-4 min for a set of 85% intensity. Can super set other exercises in the mean \
             time.",
        )
        .header("How to Select Training Frequency: Strength vs. Hypertrophy")
        .p(
            "Hypertrophy: 48-72 hours of new protein synthesis -> recovery time. Hypertrophy \
             needs 72 hours e.g. Monday -> Thursday same muscle to get optimal rest.",
        )
        .p("Hypertrophy: Not _intensity_ the main driver!")
        .p(
            "Hypertrophy: Total volume. 10 working sets per muscle group per week. Minimum \
             threshold. (He typically uses 10 reps in his examples here). 15, 20, 25 (trained \
             folks) sets is better!",
        )
        .header("Hypertrophy Training, Repetition Ranges, Blood Flow Restriction")
        .p("8-30 reps per set! Literature shows. So have some fun by varying range for yourself.")
        .p("Muscular failure! You need it. But not _extreme_ failure.")
        .p(
            "1. Feel contraction? 2. Feel a pump after/during? 3. No soreness day after? If none \
             of these, probably little muscle growth. So we want to hit all of these/at least \
             some of these for each muscle group.",
        )
        .header("Tools: Protocols for Strength Training, the 3 by 5 Concept")
        .p(
            "3 to 5 exercises, 3 to 5 reps, 3 to 5 sets, 3 to 5 minutes rest, 3 to 5 times per \
             week. Modify the number by how you are feeling.",
        )
        .header("Mind-Muscle Connection")
        .p(
            "Intention to do things is important, e.g. for strength folks it can build more \
             strength.",
        )
        .p(
            "Hypertrophy: Mind body connection creates more growth! Look at muscles contracting, \
             feel it more etc., it helps!",
        )
        .header("Mental Awareness")
        .p("Touching a muscle to prepare for working that can help.")
        .p(
            "Make L shape (thumbs out) palms facing out front. Place just above hips. Try moving \
             entire hand outwards. This is core muscles working. TODO: Learn to separate this \
             breath! If we need to hold breath for brace, we're not there yet.",
        )
        .p("20% brace activation can be enough.")
        .p(
            "Eccentric (moving weight away from body, the second half of the rep) movements can \
             help getting activation where its hard.",
        )
        .header("Breathing Tools for Resistance Training & Post-Training")
        .p("If can brace when breathing, don't care.")
        .p("Try doing double exhale time to inhale for calming down between stuff. Productive!")
        .p("Really do this even while showering. Take a few mins to \
            do this breathing to tell the body it's safe.")
        .header("Endurance Training & Combining with Strength")
        .p("Now it's more about endurance, so let's not write more notes.")
}

fn eating_for_hypertrophy() -> impl NodeExt {
    Article::new()
        .header("Hypertrophy eating")
        .p("Need caloric surplus. 10-15% caloric surplus is the research.")
        .p("2-4g/kg proteins. 2g considered low end for hypertrophy.")
        .p("Need high quality proteins. 700-3000mg leucine? Total proteins?")
        .p("Number of meals? Who knows. >=3 min is probably ok.")
        .p("Probably doesn't matter that meals are _directly_ after a workout")
}

fn new_science_of_muscle_hypertrophy_1() -> impl NodeExt {
    Article::new()
        .header("How much muscle grows (9:05)")
        .p("5-20% muscle volume mass in first 8-16 weeks")
        .header("Is the growth uniform across the muscle? (9:50)")
        .p("Probably not")
        .header("Show you ACTUAL elite athlete muscle fibers (15:00)")
        .p("Cool, actual fibers like small threads out of a red chunk of muscle/myo fiber/cell")
        .p("So fibers are made of myofibril, which again contains actin, myosin")
        .p("Sarcomeres run along the length of a fiber. Can make more of these in series or more in parallel to get more muscle volume")
        .header("Myofiber lengthening")
        .p("Pennation angle (angle of muscle fibers) can change due to hypertrophy (7-14%)")
        .p("Fascicle length too. But insertion point of muscle is not changed")
        .header("Myofiber splitting & hyperplasia")
        .p("Diameter of fibers increase with hypertrophy")
        .p("This means to increase number of muscle fibers. Hard to know, increases in cats though..")
        .p("Maybe more realistic that we split fibers and they grow independent. Likely possible but very uncommon (unless steroids)")
        .header("Myofiber growth (47:30)")
        .p("Resistance exercise is very well documented to increase muscle cell cross sectional area")
        .header("Bodybuilder vs. powerlifter muscle growth (sarcoplasmic hypertrophy)")
        .p("Powerlifter more functional hypertrophy, more contractile stuff == myofibrils. Bodybuilders more non-functional growth. Non-contractile sarcoplasmic proteins. Disproportionately more fluids!")
}

fn new_science_of_muscle_hypertrophy_2() -> impl NodeExt {
    Article::new()
        .header("5 Steps To Activating Muscle Growth")
        .p("1: Stimulus. Of the muscle cell membrane. Mechanical tension, by lifting 'heavy' (30-60%+ of 1RM")
        .p("Amount of soreness is not really a good indicator. Muscle damage not a goal in itself (DOMS).")
        .p("The 'pump' can be an indicator (of metabolic stress, good)")
        .p("2: Signaling")
        .p("3: Gene expression")
        .p("4: Protein synthesis")
        .header("TESTOSTERONE?")
        .p("Natural testosterone levels do not matter much.")
        .header("Signaling (Cytoplasm)")
        .p("So the cell wall (sarcoma)? has protein receptors.\
        If we can create some stimulus, this creates an anabolic signal inside the cell, for signaling proteins.\
        Called an anabolic cascade.")
        .header("Cell Signaling")
        .p("Anabolic is growing. Catabolic is breaking down. Inhibitors (e.g. myostatin) stop the anabolic process (else cancer).")
        .header("Protein Synthesis (Ribosomes)")
        .p("We can have lots of protein synthesis, but we have protein breakdown too. We need a positive balance.")
        .header("Overall Protein Balance")
        .p("Sitting still has you in a negative balance. Resistance exercise will move you towards hypertrophy, but\
        can't get you positive by itself. A high protein meal moves you up into hypertrophy.\
        Both resistance exercise and a high protein meal takes you far up. Carbs too are necessary.")
        .p("The above holds given essential amino acids. Needs the leucine threshold.")
        .header("If You Want To Add Muscle, You Have To..")
        .p("* Train. Stress, tension, damage. Need to signal the cells.")
        .p("* Eat. Fuel the training (carbs) and build back up (proteins). Protein synthesis is faster with carbs (woah keto spooky).")
        .header("Short summary")
        .p("Hit tension/stress/damage 1-3x per week per muscle group, eat >=10% Kcal surplus and at at least 1g/kg protein.")
}

fn new_science_of_muscle_hypertrophy_3() -> impl NodeExt {
    Article::new()
        .header("How many calories for muscle growth")
        .p("10-15% surplus at least")
        .p("Carbohydrates: 4-7g/kg")
        .p("Fat: 1g/kg")
        .p("Protein: 2-4g/kg. Most important. Move fat/carbs around with proteins pinned.")
        .p("Want high quality proteins (essential amino acids).")
        .p("3g of leucine per feeding would be good. High quality proteins will bring this.")
        .header("Anabolic Window")
        .p("I.e. when to eat. It does exist. Look at separate video (TODO).")
        .header("Volume")
        .p("Volume is king! Accumulate work. 10 sets per week up to 20 sets per week per muscle.\
        After some years, maybe even 20 to even 30 sets per week.")
        .p("5 sets a week enough for maintenance.")
        .p("Have to balance volume against fatigue.")
        .header("Intensity (31:30)")
        .p("~30-85% 1RM. Kinda idiot proof as long as you get the volume. Just gotta hit tension/stress/damage.")
        .p("Do some stuff all around the spectrum! But note that higher strength is best gained at higher %s")
        .p("Implicit: Gotta do to failure or close (especially at lower %s)")
        .header("Training to failure (35:30)")
        .p("Stopping 1-3 reps short of failure seems to be OK, when volume is equated.")
        .p("Stopping short a few can be an advantage for recovery.")
        .header("Rest intervals")
        .p("Science varies. Switch it up.")
        .header("Frequency (times per week per muscle group)")
        .p("When volume is equated, does not matter too much!")
        .p("Say we do squats, leg press, leg extensions, each 3 sets. That's 9 sets for legs. If we want to get 15-25 sets per week\
        that's a lot for 1 day. So makes more sense to spread it out.")
        .p("Seems 3x a week is a nice sweet spot.")
        .header("Which exercises work best")
        .p("Application of exercises is what matters, and volume. Not the exercise itself.")
        .p("Exercise considerations: Movement plane, muscles/joints, contraction type, technical proficiency")
        .p("For hypertrophy you can choose by movement (vertical press, horizontal row, etc.) or by muscle.\
        We just gotta find the thing that hits the muscle")
        .p("Order vs chaos: Order is doing the same exercise the same way every time. But then we might not\
        get well rounded. But if we do chaos (variations in movement, incline, bars etc.) we might not hit the same spot enough\
        for growth. Gotta balance!")
        .header("Renaissance Periodization")
        .p("Simple system, 0-2 points for little-much for each of these: Pump, next day soreness, feel in muscle during exercise.\
        As a rule we want to get 4-5 points. 5-6 over time can be fatiguing, 0-3 can be suboptimal.")
        .header("Partial vs. Full range of motion")
        .p("Default: Try getting joints through all ranges of motion (in good form).")
        .p("Full ROM most of the time. Do some partials once in a while.")
        .header("Concentric vs. Eccentric")
        .p("Do some eccentrics (negative reps), can be a nice boost for hypertrophy!")
        .header("Repetition tempo and time under tension")
        .p("Probably not very important.")
        .header("What time of day to lift")
        .p("Whatever. What fits your lifestyle.")
        .header("Potpourri")
        .p("For hypertrophy: It's all about the quality of the contractions and the volume!")
        .p("Can't finish the reps/sets? What do? Keep weight do less reps, or less weight more reps?\
        Does not matter too much. But volume is important, so cut some weight and get the volume in.")
        .header("Summary")
        .p("Both complex and single join exercises (big/specific)")
        .p("15-25 sets pr week pr muscle group")
        .p("Mostly in 60-80% 1REM range")
        .p("30s-240s rest per set")
        .p("QUALITY of contraction is key (speed of rep not so much)")
        .p("Do some negatives")
        .p("Hypertrophy: Olympic snatch type stuff isn't really recommended")
        .p("Pyramids/drop sets/super sets etc. all good!")
        .p("Slow progressive overload")
        .p("Be consistent, build habits")
        .p("Over long time, multi-join controlled speed full ROM stuff is nice")
}

pub async fn training() -> Result<Html<String>, (StatusCode, String)> {
    let mut contents = Div.kid(Style.text(".lil-room { margin-bottom: 1.0rem }"));

    let spacer = Hr.class("lil-room");

    contents.push_kid(Source::new(
        "Dr. Andy Galpin: How to Build Strength, Muscle Size & Endurance | Huberman Lab Podcast #65",
        "https://www.youtube.com/watch?v=IAnhFUUCq6c",
        huberman_podcast_with_andy_galpin(),
    ));
    contents.push_kid(spacer.clone());

    contents.push_kid(Source::new(
        "Eating for Hypertrophy : 5 Min Phys",
        "https://www.youtube.com/watch?v=0fCtyTChU_U",
        eating_for_hypertrophy(),
    ));
    contents.push_kid(spacer.clone());

    contents.push_kid(Source::new(
        "New Science of Muscle Hypertrophy - Part 1, Physiology",
        "https://www.youtube.com/watch?v=MyKrc-fheBw",
        new_science_of_muscle_hypertrophy_1(),
    ));
    contents.push_kid(spacer.clone());

    contents.push_kid(Source::new(
        "New Science of Muscle Hypertrophy - Part 2, Stimuli",
        "https://www.youtube.com/watch?v=-FR5CQhsDg4",
        new_science_of_muscle_hypertrophy_2(),
    ));
    contents.push_kid(spacer.clone());

    contents.push_kid(Source::new(
        "New Science of Muscle Hypertrophy - Part 3, Eating & Training",
        "https://www.youtube.com/watch?v=cw6XPWaEK20",
        new_science_of_muscle_hypertrophy_3(),
    ));

    let html = html_doc::<String>(None, None, None, contents);

    render(html).await
}
