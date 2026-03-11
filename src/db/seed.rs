use rusqlite::Connection;

pub fn seed_if_empty(conn: &Connection) -> Result<(), rusqlite::Error> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0))?;
    if count > 0 {
        return Ok(());
    }
    seed_all(conn)
}

fn seed_all(conn: &Connection) -> Result<(), rusqlite::Error> {
    seed_subjects(conn)?;
    seed_topics(conn)?;
    seed_lessons(conn)?;
    seed_explanations(conn)?;
    seed_quiz_questions(conn)?;
    seed_learning_paths(conn)?;
    Ok(())
}

fn seed_subjects(conn: &Connection) -> Result<(), rusqlite::Error> {
    let subjects = [
        ("Mathematics", "Numbers, shapes, and logical thinking — the language of the universe."),
        ("Science", "Understanding the natural world through observation and experiment."),
        ("Language", "Reading, writing, and communicating effectively."),
        ("History", "Key events and ideas that shaped our world."),
        ("Health", "Taking care of your body and mind."),
    ];
    for (name, desc) in &subjects {
        conn.execute("INSERT INTO subjects (name, description) VALUES (?1, ?2)", [name, desc])?;
    }
    Ok(())
}

fn seed_topics(conn: &Connection) -> Result<(), rusqlite::Error> {
    let topics = [
        // Mathematics (subject_id=1)
        (1, "Arithmetic", "beginner", 1),
        (1, "Fractions", "beginner", 2),
        (1, "Percentages", "intermediate", 3),
        (1, "Algebra Basics", "intermediate", 4),
        (1, "Geometry", "beginner", 5),
        // Science (subject_id=2)
        (2, "Photosynthesis", "beginner", 1),
        (2, "Cell Division", "intermediate", 2),
        (2, "Gravity", "beginner", 3),
        (2, "States of Matter", "beginner", 4),
        // Language (subject_id=3)
        (3, "Grammar Basics", "beginner", 1),
        (3, "Reading Comprehension", "intermediate", 2),
        // History (subject_id=4)
        (4, "Ancient Civilizations", "beginner", 1),
        (4, "World Wars", "intermediate", 2),
        (4, "Industrial Revolution", "intermediate", 3),
        // Health (subject_id=5)
        (5, "Hygiene", "beginner", 1),
        (5, "Nutrition", "beginner", 2),
        (5, "First Aid Basics", "intermediate", 3),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }
    Ok(())
}

fn seed_lessons(conn: &Connection) -> Result<(), rusqlite::Error> {
    let lessons = [
        // Arithmetic (topic_id=1)
        (1, "Addition & Subtraction", "Addition combines quantities: 3 + 5 = 8. Subtraction finds the difference: 8 - 3 = 5.\n\nThink of addition as putting groups together and subtraction as taking away.\n\nKey properties:\n- Commutative: a + b = b + a\n- Associative: (a + b) + c = a + (b + c)\n- Identity: a + 0 = a", 1),
        (1, "Multiplication & Division", "Multiplication is repeated addition: 4 × 3 = 4 + 4 + 4 = 12.\nDivision splits into equal groups: 12 ÷ 3 = 4.\n\nKey facts:\n- a × 1 = a (identity)\n- a × 0 = 0\n- Division by zero is undefined!", 2),
        // Fractions (topic_id=2)
        (2, "What Are Fractions?", "A fraction represents a part of a whole. Written as numerator/denominator.\n\nExample: 3/4 means 3 parts out of 4 equal parts.\n\nTypes:\n- Proper: numerator < denominator (3/4)\n- Improper: numerator ≥ denominator (5/3)\n- Mixed: whole number + fraction (1 2/3)", 1),
        (2, "Adding & Subtracting Fractions", "Same denominator: add/subtract numerators. 2/5 + 1/5 = 3/5\n\nDifferent denominators: find common denominator first.\n1/3 + 1/4 = 4/12 + 3/12 = 7/12\n\nAlways simplify your answer!", 2),
        // Percentages (topic_id=3)
        (3, "Understanding Percentages", "Percent means 'per hundred'. 50% = 50/100 = 0.5\n\nConverting:\n- Fraction → %: multiply by 100. (3/4 = 75%)\n- % → Decimal: divide by 100. (25% = 0.25)\n\nCommon: 50% = half, 25% = quarter, 10% = tenth", 1),
        // Algebra (topic_id=4)
        (4, "Variables and Expressions", "A variable is a letter representing an unknown number.\n\nx + 3 = 7 → x = 4\n\nExpressions combine variables and numbers: 2x + 5\n\nKey idea: whatever you do to one side of an equation, do to the other.", 1),
        // Geometry (topic_id=5)
        (5, "Shapes and Angles", "Basic shapes:\n- Triangle: 3 sides, angles sum to 180°\n- Square: 4 equal sides, 4 right angles (90°)\n- Circle: all points equidistant from center\n\nAngle types:\n- Acute: < 90°\n- Right: = 90°\n- Obtuse: > 90°", 1),
        // Photosynthesis (topic_id=6)
        (6, "How Plants Make Food", "Photosynthesis: plants convert sunlight into energy.\n\nFormula: 6CO₂ + 6H₂O + light → C₆H₁₂O₆ + 6O₂\n\nIngredients: carbon dioxide, water, sunlight\nProducts: glucose (sugar), oxygen\n\nHappens in chloroplasts, using chlorophyll (green pigment).", 1),
        // Cell Division (topic_id=7)
        (7, "Mitosis", "Mitosis: one cell divides into two identical cells.\n\nPhases:\n1. Prophase: chromosomes condense\n2. Metaphase: chromosomes line up\n3. Anaphase: chromosomes pull apart\n4. Telophase: two nuclei form\n5. Cytokinesis: cell splits\n\nResult: 2 identical daughter cells.", 1),
        // Gravity (topic_id=8)
        (8, "What is Gravity?", "Gravity: the force that pulls objects toward each other.\n\nKey facts:\n- Earth's gravity = 9.8 m/s²\n- More mass = stronger gravity\n- Keeps planets orbiting the sun\n- Newton's apple story (1687)\n- Weight = mass × gravity", 1),
        // States of Matter (topic_id=9)
        (9, "Solids, Liquids, and Gases", "Three main states:\n\nSolid: fixed shape, fixed volume. Particles packed tightly.\nLiquid: takes shape of container, fixed volume. Particles slide.\nGas: fills any container. Particles move freely.\n\nChanges: melting, freezing, evaporation, condensation, sublimation.", 1),
        // Grammar (topic_id=10)
        (10, "Parts of Speech", "8 parts of speech:\n1. Noun: person, place, thing (dog, Paris)\n2. Verb: action or state (run, is)\n3. Adjective: describes noun (big, red)\n4. Adverb: describes verb (quickly)\n5. Pronoun: replaces noun (he, she)\n6. Preposition: shows relation (in, on)\n7. Conjunction: connects (and, but)\n8. Interjection: emotion (wow!)", 1),
        // Reading Comprehension (topic_id=11)
        (11, "Active Reading Strategies", "Before reading: preview title, headings, images.\n\nDuring reading:\n- Ask questions: Who? What? Why?\n- Visualize the scene\n- Connect to what you know\n- Note unfamiliar words\n\nAfter reading:\n- Summarize in your own words\n- Identify the main idea\n- Discuss or write about it", 1),
        // Ancient Civilizations (topic_id=12)
        (12, "Early Civilizations", "Major early civilizations:\n\nMesopotamia (3500 BCE): First writing (cuneiform), between Tigris & Euphrates.\nEgypt (3100 BCE): Pyramids, pharaohs, hieroglyphics, Nile River.\nIndus Valley (2600 BCE): Planned cities, advanced drainage.\nChina (1600 BCE): Shang dynasty, oracle bones, bronze work.\n\nCommon thread: all arose near rivers.", 1),
        // World Wars (topic_id=13)
        (13, "World War I & II Overview", "WWI (1914-1918):\n- Triggered by assassination of Archduke Franz Ferdinand\n- Trench warfare, new weapons (tanks, gas)\n- ~17 million deaths\n\nWWII (1939-1945):\n- Rise of fascism (Hitler, Mussolini)\n- Holocaust: 6 million Jews murdered\n- Atomic bombs on Hiroshima & Nagasaki\n- ~70-85 million deaths\n- Led to United Nations founding", 1),
        // Industrial Revolution (topic_id=14)
        (14, "The Age of Machines", "Industrial Revolution (1760-1840):\n\nKey inventions:\n- Steam engine (James Watt)\n- Spinning jenny (textiles)\n- Railways\n\nEffects:\n- Rural → urban migration\n- Factory system replaced cottage industry\n- Child labor was common\n- Eventually led to labor laws and unions\n\nStarted in Britain, spread worldwide.", 1),
        // Hygiene (topic_id=15)
        (15, "Personal Hygiene", "Why hygiene matters: prevents disease, infections.\n\nEssentials:\n- Wash hands: before eating, after bathroom (20 seconds with soap)\n- Brush teeth: twice daily, 2 minutes\n- Bathe regularly\n- Clean clothes\n- Cover coughs and sneezes\n\nHand washing alone prevents ~30% of diarrheal diseases.", 1),
        // Nutrition (topic_id=16)
        (16, "Healthy Eating", "Food groups:\n1. Fruits & Vegetables: vitamins, fiber\n2. Grains: energy (bread, rice)\n3. Protein: growth (meat, beans, eggs)\n4. Dairy: calcium (milk, cheese)\n5. Fats: small amounts needed\n\nTips:\n- Drink water, not sugary drinks\n- Eat the rainbow (variety)\n- Breakfast matters!", 1),
        // First Aid (topic_id=17)
        (17, "Basic First Aid", "Key skills:\n\nCuts: Clean with water, apply pressure, bandage.\nBurns: Cool under running water 10+ minutes. No ice!\nNosebleed: Lean forward, pinch soft part of nose.\nChoking: Back blows, then abdominal thrusts (Heimlich).\n\nEmergency: Call local emergency number.\nStay calm. Assess the situation. Keep the person comfortable.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }
    Ok(())
}

fn seed_explanations(conn: &Connection) -> Result<(), rusqlite::Error> {
    let explanations = [
        (1, "addition", "Addition means putting numbers together to get a total.", Some("Think of it like putting apples in a basket — 3 apples + 2 apples = 5 apples."), Some("Can you think of a time you used addition today?")),
        (2, "fractions", "A fraction is a way to show parts of a whole.", Some("Imagine cutting a pizza into 4 slices and eating 1 — you ate 1/4 of the pizza!"), Some("If you eat 2 slices of an 8-slice pizza, what fraction did you eat?")),
        (3, "percentages", "A percentage is a number out of 100.", Some("Think of a test with 100 questions — your score IS your percentage!"), Some("If you got 8 out of 10 right, what percentage is that?")),
        (4, "algebra", "Algebra uses letters to represent unknown numbers we want to find.", Some("It's like a mystery: x + 3 = 7. What number is hiding behind x?"), Some("If x + 5 = 12, what is x?")),
        (5, "geometry", "Geometry is the study of shapes, sizes, and space.", Some("Look around you — rectangles (doors), circles (clocks), triangles (roofs). Geometry is everywhere!"), Some("How many right angles does a rectangle have?")),
        (6, "photosynthesis", "Photosynthesis is how plants make their own food using sunlight.", Some("Plants are like tiny solar-powered kitchens — they use sunlight to cook sugar from water and air!"), Some("What gas do plants release during photosynthesis?")),
        (7, "cell division", "Cell division is how one cell becomes two new cells.", Some("Imagine making a photocopy of yourself — that's what cells do to grow and repair your body!"), Some("What is the name of the process where a cell divides into two identical cells?")),
        (8, "gravity", "Gravity is the invisible force that pulls things toward each other.", Some("It's why you come back down when you jump — Earth is pulling you like a giant magnet!"), Some("Does a heavier object fall faster than a lighter one in a vacuum?")),
        (9, "states of matter", "Matter exists in three main states: solid, liquid, and gas.", Some("Think of water: ice cube (solid), water (liquid), steam (gas). Same stuff, different states!"), Some("What happens to ice when you heat it?")),
        (10, "grammar", "Grammar is the set of rules for how we put words together.", Some("Grammar is like the recipe for a sentence — without it, the ingredients don't make sense!"), Some("In the sentence 'The cat sat on the mat', which word is the verb?")),
        (15, "hygiene", "Hygiene means keeping yourself and your surroundings clean to stay healthy.", Some("Think of your hands as sponges — they pick up tiny germs everywhere. Washing squeezes them away!"), Some("How long should you wash your hands with soap?")),
        (16, "nutrition", "Nutrition is about eating the right foods to keep your body healthy and strong.", Some("Your body is like a car — it needs the right fuel. Junk food is like putting soda in the gas tank!"), Some("Can you name three foods from different food groups?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }
    Ok(())
}

fn seed_quiz_questions(conn: &Connection) -> Result<(), rusqlite::Error> {
    let questions = [
        // Arithmetic
        (1, "What is 7 + 8?", "multiple_choice", "15", Some("13"), Some("14"), Some("15"), Some("16"), Some("Count up from 7: 8 more"), "7 + 8 = 15. You can verify: 15 - 8 = 7."),
        (1, "What is 12 × 4?", "multiple_choice", "48", Some("36"), Some("44"), Some("48"), Some("52"), Some("12 × 4 = 12 + 12 + 12 + 12"), "12 × 4 = 48. Think of it as 4 groups of 12."),
        (1, "What is 100 ÷ 5?", "multiple_choice", "20", Some("15"), Some("20"), Some("25"), Some("50"), Some("How many 5s fit in 100?"), "100 ÷ 5 = 20. Five times twenty equals one hundred."),
        // Fractions
        (2, "What is 1/2 + 1/4?", "multiple_choice", "3/4", Some("2/4"), Some("2/6"), Some("3/4"), Some("1/4"), Some("Convert 1/2 to 2/4 first"), "1/2 = 2/4, so 2/4 + 1/4 = 3/4."),
        (2, "Simplify 4/8.", "multiple_choice", "1/2", Some("1/2"), Some("2/4"), Some("1/4"), Some("4/8"), Some("Find the GCD of 4 and 8"), "4/8 = 1/2. Divide both by 4."),
        // Percentages
        (3, "What is 25% of 200?", "multiple_choice", "50", Some("25"), Some("40"), Some("50"), Some("75"), Some("25% = 1/4"), "25% of 200 = 200 × 0.25 = 50."),
        (3, "Convert 3/5 to a percentage.", "multiple_choice", "60%", Some("30%"), Some("50%"), Some("60%"), Some("75%"), Some("Divide 3 by 5, multiply by 100"), "3 ÷ 5 = 0.6 = 60%."),
        // Algebra
        (4, "Solve: x + 7 = 15", "multiple_choice", "8", Some("7"), Some("8"), Some("9"), Some("22"), Some("Subtract 7 from both sides"), "x = 15 - 7 = 8."),
        (4, "Solve: 3x = 21", "multiple_choice", "7", Some("3"), Some("7"), Some("18"), Some("63"), Some("Divide both sides by 3"), "x = 21 ÷ 3 = 7."),
        // Geometry
        (5, "How many degrees in a triangle?", "multiple_choice", "180", Some("90"), Some("180"), Some("270"), Some("360"), Some("It's less than a full circle"), "The angles of any triangle always add up to 180°."),
        (5, "What is the area of a rectangle 5 × 3?", "multiple_choice", "15", Some("8"), Some("15"), Some("16"), Some("30"), Some("Area = length × width"), "Area = 5 × 3 = 15 square units."),
        // Photosynthesis
        (6, "What gas do plants absorb during photosynthesis?", "multiple_choice", "Carbon dioxide", Some("Oxygen"), Some("Nitrogen"), Some("Carbon dioxide"), Some("Hydrogen"), Some("It's what we breathe out"), "Plants absorb CO₂ (carbon dioxide) and release O₂ (oxygen)."),
        (6, "Where does photosynthesis happen in a cell?", "multiple_choice", "Chloroplast", Some("Nucleus"), Some("Mitochondria"), Some("Chloroplast"), Some("Cell wall"), Some("It contains chlorophyll"), "Photosynthesis occurs in chloroplasts, which contain chlorophyll."),
        // Cell Division
        (7, "How many cells result from mitosis?", "multiple_choice", "2", Some("1"), Some("2"), Some("4"), Some("8"), Some("One becomes..."), "Mitosis produces 2 identical daughter cells."),
        // Gravity
        (8, "What is Earth's gravitational acceleration?", "multiple_choice", "9.8 m/s²", Some("5.5 m/s²"), Some("9.8 m/s²"), Some("10.5 m/s²"), Some("15 m/s²"), Some("It's close to 10"), "Earth's gravitational acceleration is approximately 9.8 m/s²."),
        // States of Matter
        (9, "What is it called when a liquid becomes a gas?", "multiple_choice", "Evaporation", Some("Melting"), Some("Condensation"), Some("Evaporation"), Some("Freezing"), Some("Think of boiling water"), "Evaporation (or boiling/vaporization) turns liquid into gas."),
        (9, "What state of matter has a fixed shape?", "multiple_choice", "Solid", Some("Solid"), Some("Liquid"), Some("Gas"), Some("Plasma"), Some("Think of ice"), "Solids have a fixed shape because their particles are tightly packed."),
        // Grammar
        (10, "Which is a noun in: 'The dog runs fast'?", "multiple_choice", "dog", Some("the"), Some("dog"), Some("runs"), Some("fast"), Some("A noun is a person, place, or thing"), "'Dog' is a noun — it names a thing."),
        (10, "What type of word is 'quickly'?", "multiple_choice", "Adverb", Some("Noun"), Some("Adjective"), Some("Adverb"), Some("Verb"), Some("It describes how something is done"), "'Quickly' is an adverb — it describes how an action is performed."),
        // Hygiene
        (15, "How long should you wash your hands?", "multiple_choice", "20 seconds", Some("5 seconds"), Some("10 seconds"), Some("20 seconds"), Some("1 minute"), Some("Sing 'Happy Birthday' twice"), "Wash hands for at least 20 seconds with soap and water."),
        // Nutrition
        (16, "Which food group provides calcium?", "multiple_choice", "Dairy", Some("Grains"), Some("Dairy"), Some("Fruits"), Some("Fats"), Some("Think of milk and cheese"), "Dairy products like milk, cheese, and yogurt are rich in calcium."),
        // First Aid
        (17, "How should you cool a minor burn?", "multiple_choice", "Running cool water for 10+ minutes", Some("Apply ice directly"), Some("Running cool water for 10+ minutes"), Some("Apply butter"), Some("Blow on it"), Some("Gentle cooling, not freezing"), "Cool burns under running cool (not cold) water for at least 10 minutes. Never use ice or butter."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }
    Ok(())
}

fn seed_learning_paths(conn: &Connection) -> Result<(), rusqlite::Error> {
    let paths = [
        ("algebra", 1, 1, "Master basic arithmetic — addition, subtraction, multiplication, division"),
        ("algebra", 2, 2, "Understand fractions — parts of a whole"),
        ("algebra", 3, 3, "Learn percentages — fractions of 100"),
        ("algebra", 4, 4, "Begin algebra — variables and equations"),
        ("cells", 1, 6, "Understand photosynthesis — how cells produce energy"),
        ("cells", 2, 9, "Learn states of matter — foundation for chemistry"),
        ("cells", 3, 7, "Study cell division — how organisms grow"),
        ("healthy living", 1, 15, "Start with hygiene — disease prevention"),
        ("healthy living", 2, 16, "Learn nutrition — fuel your body right"),
        ("healthy living", 3, 17, "Basic first aid — be prepared for emergencies"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema;
    use rusqlite::Connection;

    #[test]
    fn test_seed_populates_data() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_seed_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        seed_if_empty(&conn).unwrap(); // should not duplicate
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_all_topics_have_lessons() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let orphans: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t WHERE NOT EXISTS (SELECT 1 FROM lessons WHERE topic_id = t.id)",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(orphans, 0, "All topics should have at least one lesson");
    }

    #[test]
    fn test_quiz_questions_exist() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM quiz_questions", [], |r| r.get(0)).unwrap();
        assert!(count >= 20, "Should have at least 20 quiz questions, got {}", count);
    }
}
