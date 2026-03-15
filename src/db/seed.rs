use rusqlite::Connection;

type LessonRow<'a> = (i64, &'a str, &'a str, i64);
type ExplanationRow<'a> = (i64, &'a str, &'a str, Option<&'a str>, Option<&'a str>);
type QuizRow<'a> = (i64, &'a str, &'a str, &'a str, Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>, &'a str);
type QuizRowHint<'a> = (i64, &'a str, &'a str, &'a str, Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>, &'a str, &'a str);
type QuizRowNoTopic<'a> = (&'a str, &'a str, &'a str, Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>, &'a str, &'a str);

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
    seed_chemistry(conn)?;
    seed_biology(conn)?;
    seed_sociology(conn)?;
    seed_linguistics(conn)?;
    seed_probability(conn)?;
    seed_statistics(conn)?;
    seed_ethics(conn)?;
    seed_world_literature(conn)?;
    seed_trigonometry(conn)?;
    seed_political_science(conn)?;
    seed_renaissance(conn)?;
    seed_extra_history_quizzes(conn)?;
    seed_formal_logic_and_health(conn)?;
    seed_anthropology(conn)?;
    seed_nutrition_science(conn)?;
    seed_expanded_language_health(conn)?;
    seed_astronomy_physics_expanded(conn)?;
    seed_calculus(conn)?;
    seed_programming_basics(conn)?;
    seed_extra_math_quizzes(conn)?;
    seed_creative_writing(conn)?;
    seed_earth_science(conn)?;
    seed_data_science(conn)?;
    seed_music_theory(conn)?;
    seed_civics_and_media(conn)?;
    seed_world_languages(conn)?;
    seed_geography_expanded(conn)?;
    seed_psychology_expanded(conn)?;
    seed_game_theory(conn)?;
    seed_architecture(conn)?;
    seed_extra_quizzes_round2(conn)?;
    seed_cybersecurity(conn)?;
    seed_discrete_mathematics(conn)?;
    seed_linear_algebra(conn)?;
    seed_electrical_engineering(conn)?;
    seed_robotics_ai(conn)?;
    seed_number_theory(conn)?;
    seed_formal_languages(conn)?;
    seed_philosophy_of_mind(conn)?;
    seed_organic_chemistry(conn)?;
    seed_graph_theory(conn)?;
    seed_thermodynamics(conn)?;
    seed_cognitive_science(conn)?;
    seed_cloze_questions(conn)?;
    seed_ecology(conn)?;
    seed_abstract_algebra(conn)?;
    seed_molecular_biology(conn)?;
    seed_set_theory(conn)?;
    seed_analogy_questions(conn)?;
    seed_paleontology(conn)?;
    seed_marine_biology(conn)?;
    seed_astrophysics(conn)?;
    seed_neuroscience(conn)?;
    seed_cryptography(conn)?;
    seed_information_theory(conn)?;
    seed_extra_core_quizzes(conn)?;
    assign_quiz_difficulties(conn)?;
    Ok(())
}

fn seed_subjects(conn: &Connection) -> Result<(), rusqlite::Error> {
    let subjects = [
        ("Mathematics", "Numbers, shapes, and logical thinking — the language of the universe."),
        ("Science", "Understanding the natural world through observation and experiment."),
        ("Language", "Reading, writing, and communicating effectively."),
        ("History", "Key events and ideas that shaped our world."),
        ("Health", "Taking care of your body and mind."),
        ("Computer Science", "How computers work, from logic gates to algorithms."),
        ("Geography", "Understanding our planet — landscapes, climates, and cultures."),
        ("Music", "The art of sound — rhythm, melody, harmony, and expression."),
        ("Art", "Visual creativity — color, form, composition, and art history."),
        ("Philosophy", "Thinking about thinking — logic, ethics, and the big questions of life."),
        ("Economics", "How societies produce, distribute, and consume goods and services."),
        ("Psychology", "Understanding the mind — how we think, feel, behave, and interact with others."),
        ("Environmental Science", "Exploring ecosystems, climate, pollution, and sustainability."),
        ("Creative Writing", "Crafting stories, poems, and essays — finding your voice through words."),
        ("Astronomy", "Exploring space — stars, planets, galaxies, and the vast universe beyond Earth."),
        ("Physics", "The fundamental science of matter, energy, forces, and motion — understanding how the universe works."),
        ("Linguistics", "The scientific study of language — its structure, meaning, sounds, and evolution across cultures."),
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
        // Computer Science (subject_id=6)
        (6, "Binary & Data", "beginner", 1),
        (6, "Algorithms", "beginner", 2),
        (6, "Programming Concepts", "intermediate", 3),
        (6, "Computer Hardware", "beginner", 4),
        // Geography (subject_id=7)
        (7, "Continents & Oceans", "beginner", 1),
        (7, "Weather & Climate", "beginner", 2),
        (7, "Maps & Navigation", "beginner", 3),
        (7, "Natural Resources", "intermediate", 4),
        // Music (subject_id=8)
        (8, "Musical Notes & Scales", "beginner", 1),
        (8, "Rhythm & Time Signatures", "beginner", 2),
        (8, "Musical Instruments", "beginner", 3),
        (8, "Music History", "intermediate", 4),
        // Art (subject_id=9)
        (9, "Color Theory", "beginner", 1),
        (9, "Elements of Art", "beginner", 2),
        (9, "Art History", "intermediate", 3),
        (9, "Composition & Design", "intermediate", 4),
        // Philosophy (subject_id=10)
        (10, "Logic & Reasoning", "beginner", 1),
        (10, "Ethics & Morality", "beginner", 2),
        (10, "Famous Philosophers", "intermediate", 3),
        (10, "Thought Experiments", "intermediate", 4),
        // Economics (subject_id=11)
        (11, "Supply & Demand", "beginner", 1),
        (11, "Money & Banking", "beginner", 2),
        (11, "Trade & Globalization", "intermediate", 3),
        (11, "Economic Systems", "intermediate", 4),
        // Psychology (subject_id=12)
        (12, "Introduction to Psychology", "beginner", 1),
        (12, "Memory & Learning", "beginner", 2),
        (12, "Emotions & Motivation", "beginner", 3),
        (12, "Social Psychology", "intermediate", 4),
        // Environmental Science (subject_id=13)
        (13, "Ecosystems & Biomes", "beginner", 1),
        (13, "Climate Change", "beginner", 2),
        (13, "Pollution & Waste", "intermediate", 3),
        (13, "Conservation & Sustainability", "intermediate", 4),
        // Creative Writing (subject_id=14)
        (14, "Story Structure", "beginner", 1),
        (14, "Character Development", "beginner", 2),
        (14, "Dialogue & Voice", "intermediate", 3),
        (14, "Poetry Fundamentals", "intermediate", 4),
        // Astronomy (subject_id=15)
        (15, "The Solar System", "beginner", 1),
        (15, "Stars & Stellar Evolution", "beginner", 2),
        (15, "Galaxies & the Universe", "intermediate", 3),
        (15, "Space Exploration", "intermediate", 4),
        // Physics (subject_id=16)
        (16, "Electricity & Magnetism", "beginner", 1),
        (16, "Thermodynamics", "intermediate", 2),
        (16, "Waves & Sound", "beginner", 3),
        (16, "Nuclear Physics", "intermediate", 4),
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
        (1, "Order of Operations", "When an expression has multiple operations, follow PEMDAS/BODMAS:\n\n1. Parentheses / Brackets\n2. Exponents / Orders\n3. Multiplication & Division (left to right)\n4. Addition & Subtraction (left to right)\n\nExample: 3 + 4 × 2 = 3 + 8 = 11 (not 14!)\nExample: (3 + 4) × 2 = 7 × 2 = 14", 3),
        (1, "Negative Numbers", "Negative numbers are less than zero, written with a minus sign.\n\nNumber line: ... -3, -2, -1, 0, 1, 2, 3 ...\n\nRules:\n- Adding a negative = subtracting: 5 + (-3) = 2\n- Subtracting a negative = adding: 5 - (-3) = 8\n- Negative × negative = positive: (-2) × (-3) = 6\n- Negative × positive = negative: (-2) × 3 = -6\n\nReal life: temperature below zero, debt, elevators below ground.", 4),
        // Fractions (topic_id=2)
        (2, "What Are Fractions?", "A fraction represents a part of a whole. Written as numerator/denominator.\n\nExample: 3/4 means 3 parts out of 4 equal parts.\n\nTypes:\n- Proper: numerator < denominator (3/4)\n- Improper: numerator ≥ denominator (5/3)\n- Mixed: whole number + fraction (1 2/3)", 1),
        (2, "Adding & Subtracting Fractions", "Same denominator: add/subtract numerators. 2/5 + 1/5 = 3/5\n\nDifferent denominators: find common denominator first.\n1/3 + 1/4 = 4/12 + 3/12 = 7/12\n\nAlways simplify your answer!", 2),
        (2, "Multiplying & Dividing Fractions", "Multiplying: multiply numerators, multiply denominators.\n2/3 × 4/5 = 8/15\n\nDividing: flip the second fraction and multiply.\n2/3 ÷ 4/5 = 2/3 × 5/4 = 10/12 = 5/6\n\nTip: simplify before multiplying to keep numbers small.\nExample: 3/4 × 2/9 → cancel 3: 1/4 × 2/3 = 2/12 = 1/6", 3),
        // Percentages (topic_id=3)
        (3, "Understanding Percentages", "Percent means 'per hundred'. 50% = 50/100 = 0.5\n\nConverting:\n- Fraction → %: multiply by 100. (3/4 = 75%)\n- % → Decimal: divide by 100. (25% = 0.25)\n\nCommon: 50% = half, 25% = quarter, 10% = tenth", 1),
        (3, "Percentage Calculations", "Finding a percentage of a number:\n20% of 150 = 0.20 × 150 = 30\n\nFinding what percentage one number is of another:\n30 is what % of 150? → 30/150 × 100 = 20%\n\nPercentage increase/decrease:\nIncrease 80 by 25%: 80 × 1.25 = 100\nDecrease 80 by 25%: 80 × 0.75 = 60\n\nTip: 'of' means multiply, 'is' means equals.", 2),
        // Algebra (topic_id=4)
        (4, "Variables and Expressions", "A variable is a letter representing an unknown number.\n\nx + 3 = 7 → x = 4\n\nExpressions combine variables and numbers: 2x + 5\n\nKey idea: whatever you do to one side of an equation, do to the other.", 1),
        (4, "Solving Equations", "Steps to solve an equation:\n\n1. Simplify each side (combine like terms)\n2. Get variables on one side\n3. Isolate the variable\n\nExample: 2x + 5 = 13\n  2x = 13 - 5 = 8\n  x = 8/2 = 4\n\nCheck: 2(4) + 5 = 13 ✓\n\nTwo-step equations:\n  3x - 7 = 14\n  3x = 21\n  x = 7", 2),
        // Geometry (topic_id=5)
        (5, "Shapes and Angles", "Basic shapes:\n- Triangle: 3 sides, angles sum to 180°\n- Square: 4 equal sides, 4 right angles (90°)\n- Circle: all points equidistant from center\n\nAngle types:\n- Acute: < 90°\n- Right: = 90°\n- Obtuse: > 90°", 1),
        (5, "Perimeter and Area", "Perimeter is the distance around a shape.\nArea is the space inside.\n\nRectangle: P = 2(l + w), A = l × w\nTriangle: A = 1/2 × base × height\nCircle: C = 2πr, A = πr²\n\nπ ≈ 3.14159\n\nExample: Circle with r=5: A = π × 25 ≈ 78.5 square units", 2),
        (5, "3D Shapes", "Three-dimensional shapes have length, width, and height.\n\nCube: 6 square faces. Volume = s³, Surface Area = 6s²\nRectangular prism: V = l × w × h\nCylinder: V = πr²h\nSphere: V = 4/3 × πr³\nCone: V = 1/3 × πr²h\n\nReal life: dice (cube), cans (cylinder), balls (sphere), ice cream cones (cone).", 3),
        // Photosynthesis (topic_id=6)
        (6, "How Plants Make Food", "Photosynthesis: plants convert sunlight into energy.\n\nFormula: 6CO₂ + 6H₂O + light → C₆H₁₂O₆ + 6O₂\n\nIngredients: carbon dioxide, water, sunlight\nProducts: glucose (sugar), oxygen\n\nHappens in chloroplasts, using chlorophyll (green pigment).", 1),
        // Cell Division (topic_id=7)
        (7, "Mitosis", "Mitosis: one cell divides into two identical cells.\n\nPhases:\n1. Prophase: chromosomes condense\n2. Metaphase: chromosomes line up\n3. Anaphase: chromosomes pull apart\n4. Telophase: two nuclei form\n5. Cytokinesis: cell splits\n\nResult: 2 identical daughter cells.", 1),
        (7, "Meiosis", "Meiosis: cell division that produces sex cells (gametes).\n\nKey differences from mitosis:\n- Two rounds of division\n- Produces 4 cells (not 2)\n- Daughter cells have HALF the chromosomes\n- Creates genetic diversity through crossing over\n\nHumans: 46 chromosomes → meiosis → 23 in each egg/sperm\nFertilization: 23 + 23 = 46 again!", 2),
        // Gravity (topic_id=8)
        (8, "What is Gravity?", "Gravity: the force that pulls objects toward each other.\n\nKey facts:\n- Earth's gravity = 9.8 m/s²\n- More mass = stronger gravity\n- Keeps planets orbiting the sun\n- Newton's apple story (1687)\n- Weight = mass × gravity", 1),
        (8, "Newton's Laws of Motion", "First Law (Inertia): An object stays at rest or in motion unless a force acts on it.\n\nSecond Law: Force = mass × acceleration (F = ma)\n\nThird Law: Every action has an equal and opposite reaction.\n\nExamples:\n- Seatbelts (1st law)\n- Pushing a heavy vs light box (2nd law)\n- Rocket engines (3rd law)", 2),
        // States of Matter (topic_id=9)
        (9, "Solids, Liquids, and Gases", "Three main states:\n\nSolid: fixed shape, fixed volume. Particles packed tightly.\nLiquid: takes shape of container, fixed volume. Particles slide.\nGas: fills any container. Particles move freely.\n\nChanges: melting, freezing, evaporation, condensation, sublimation.", 1),
        (9, "Plasma and Beyond", "Beyond the three common states:\n\nPlasma: super-heated gas with charged particles.\nExamples: lightning, the Sun, neon signs.\n\nBose-Einstein Condensate: super-cooled atoms near absolute zero.\n\nPhase diagram: shows which state exists at different temperatures and pressures.\n\nTriple point: where solid, liquid, and gas coexist.", 2),
        // Grammar (topic_id=10)
        (10, "Parts of Speech", "8 parts of speech:\n1. Noun: person, place, thing (dog, Paris)\n2. Verb: action or state (run, is)\n3. Adjective: describes noun (big, red)\n4. Adverb: describes verb (quickly)\n5. Pronoun: replaces noun (he, she)\n6. Preposition: shows relation (in, on)\n7. Conjunction: connects (and, but)\n8. Interjection: emotion (wow!)", 1),
        (10, "Sentence Structure", "A complete sentence needs a subject and a predicate (verb).\n\nSimple: The cat sat. (one clause)\nCompound: The cat sat, and the dog ran. (two independent clauses)\nComplex: When it rained, the cat sat inside. (independent + dependent clause)\n\nCommon errors:\n- Run-on: two sentences joined without punctuation\n- Fragment: missing subject or verb\n- Comma splice: two sentences joined only by a comma", 2),
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
        // Computer Science (topic_id=18-21)
        (18, "Understanding Binary", "Computers use binary (base 2) — only 0s and 1s.\n\nDecimal vs Binary:\n0 = 0, 1 = 1, 2 = 10, 3 = 11, 4 = 100, 5 = 101\n\nEach position is a power of 2:\n1011 = 8 + 0 + 2 + 1 = 11\n\nBits and Bytes:\n- 1 bit = 0 or 1\n- 8 bits = 1 byte\n- 1 byte can store 256 values (0-255)\n\nASCII: A=65, B=66 — text is just numbers!", 1),
        (18, "Data Representation", "How computers store different types of data:\n\nText: ASCII (128 chars) or Unicode (143,000+ chars)\nImages: grids of pixels, each with RGB values (0-255)\nSound: samples of air pressure at regular intervals\nVideo: sequences of images + audio\n\nFile sizes: KB (1,000 bytes), MB (1,000,000), GB (1 billion)\nA page of text ≈ 2 KB, a photo ≈ 3 MB, a movie ≈ 4 GB", 2),
        (19, "What is an Algorithm?", "An algorithm is a step-by-step set of instructions to solve a problem.\n\nEveryday example — making a sandwich:\n1. Get bread\n2. Spread butter\n3. Add filling\n4. Close sandwich\n\nKey properties:\n- Clear steps (no ambiguity)\n- Finite (must eventually end)\n- Produces a result\n\nSearching: linear search (check one by one) vs binary search (divide in half)\nSorting: bubble sort, merge sort", 1),
        (19, "Big O Notation", "Big O describes how an algorithm's speed grows with input size.\n\nCommon complexities:\n- O(1): constant — same speed regardless of size\n- O(log n): logarithmic — binary search\n- O(n): linear — checking each item once\n- O(n log n): merge sort, quicksort\n- O(n²): bubble sort, nested loops\n\nWhy it matters: O(n²) with 1 million items = 1 trillion operations!\nO(n log n) with 1 million = ~20 million operations.", 2),
        (20, "Variables and Loops", "Programming builds on two key ideas:\n\nVariables: named containers for data.\n  name = \"Alice\"\n  age = 12\n  score = 95.5\n\nLoops: repeat actions.\n  FOR i = 1 to 10:\n    print(i)\n\nTypes of loops:\n- FOR: repeat a known number of times\n- WHILE: repeat until a condition is false\n\nConditionals:\n  IF score >= 90: print(\"A\")\n  ELSE IF score >= 80: print(\"B\")", 1),
        (20, "Functions and Debugging", "Functions: reusable blocks of code.\n  function greet(name):\n    return \"Hello, \" + name\n\nBenefits:\n- Avoid repeating code (DRY principle)\n- Break big problems into small pieces\n- Easier to test and fix\n\nDebugging: finding and fixing errors.\n- Syntax error: typo in code (missing bracket)\n- Logic error: code runs but gives wrong answer\n- Runtime error: crash during execution (divide by zero)\n\nTip: print values at each step to find bugs!", 2),
        (21, "Inside a Computer", "Main components:\n\nCPU (Central Processing Unit): the brain — executes instructions.\n  Speed measured in GHz (billions of cycles/second).\n\nRAM (Random Access Memory): short-term memory.\n  Fast but loses data when power off. Typically 8-32 GB.\n\nStorage (SSD/HDD): long-term memory.\n  Slower but keeps data without power. 256 GB to several TB.\n\nInput: keyboard, mouse, microphone, camera.\nOutput: screen, speakers, printer.\n\nMotherboard: connects everything together.", 1),
        // Geography (topic_id=22-25)
        (22, "The Seven Continents", "From largest to smallest:\n\n1. Asia: largest by area and population. Home to Himalayas, China, India.\n2. Africa: 54 countries, Sahara Desert, Nile River.\n3. North America: USA, Canada, Mexico, Caribbean.\n4. South America: Amazon rainforest, Andes mountains.\n5. Antarctica: ice-covered, no permanent residents.\n6. Europe: 44 countries, diverse cultures, EU.\n7. Australia/Oceania: island continent + Pacific islands.\n\nFive oceans: Pacific (largest), Atlantic, Indian, Southern, Arctic.", 1),
        (22, "Major Rivers and Mountains", "Longest rivers:\n1. Nile (6,650 km) — Africa\n2. Amazon (6,400 km) — South America\n3. Yangtze (6,300 km) — Asia\n\nHighest mountains:\n1. Everest (8,849 m) — Asia/Himalayas\n2. K2 (8,611 m) — Asia/Karakoram\n3. Kangchenjunga (8,586 m) — Asia/Himalayas\n\nOther notable features:\n- Grand Canyon (USA): 1.6 km deep\n- Great Barrier Reef: 2,300 km long\n- Mariana Trench: deepest point on Earth (11 km)", 2),
        (23, "Weather vs Climate", "Weather: short-term conditions (today's temperature, rain).\nClimate: long-term patterns over 30+ years.\n\nClimate zones:\n- Tropical: hot and wet year-round (near equator)\n- Arid/Desert: very dry, extreme temperatures\n- Temperate: moderate, four seasons\n- Continental: hot summers, cold winters\n- Polar: extremely cold year-round\n\nWater cycle: evaporation → condensation → precipitation → collection\n\nClimate change: Earth's average temperature rising due to greenhouse gases.", 1),
        (24, "Reading Maps", "Key map elements:\n\n- Title: what the map shows\n- Legend/Key: explains symbols and colors\n- Scale: shows real-world distances\n- Compass rose: shows N, S, E, W\n- Grid/coordinates: locate specific places\n\nTypes of maps:\n- Physical: shows landforms, elevation\n- Political: shows borders, cities\n- Thematic: shows specific data (population, climate)\n\nLatitude: horizontal lines (0° = equator)\nLongitude: vertical lines (0° = Prime Meridian, Greenwich)", 1),
        (25, "Earth's Resources", "Renewable resources: replenish naturally.\n- Solar energy, wind, water (hydroelectric)\n- Timber (if replanted), geothermal\n\nNon-renewable: finite supply.\n- Fossil fuels: coal, oil, natural gas\n- Minerals: iron, copper, gold\n- Uranium (nuclear energy)\n\nWhy it matters:\n- Fossil fuels release CO₂ → climate change\n- Many resources are unevenly distributed globally\n- Recycling conserves resources\n- Sustainability: meeting today's needs without compromising the future", 1),
        // Music (topic_id=26-29)
        (26, "Notes and the Staff", "Music uses 7 note names: A B C D E F G, then repeats.\n\nThe staff has 5 lines and 4 spaces.\nTreble clef (G clef): for higher notes. Lines: E G B D F. Spaces: F A C E.\nBass clef (F clef): for lower notes. Lines: G B D F A. Spaces: A C E G.\n\nSharps (#) raise a note by a half step.\nFlats (♭) lower a note by a half step.\n\nAn octave: 8 notes from one letter to the same letter higher (e.g. C to C).", 1),
        (26, "Major and Minor Scales", "A scale is a sequence of notes in order.\n\nMajor scale pattern (whole/half steps): W W H W W W H\nC major: C D E F G A B C (all white keys on piano)\n\nMinor scale pattern: W H W W H W W\nA minor: A B C D E F G A (also all white keys!)\n\nMajor scales sound bright and happy.\nMinor scales sound dark or sad.\n\nEvery major key has a relative minor (same notes, different starting point).\nC major ↔ A minor.", 2),
        (27, "Understanding Rhythm", "Rhythm is the pattern of sounds and silences in time.\n\nNote values (in 4/4 time):\n- Whole note: 4 beats\n- Half note: 2 beats\n- Quarter note: 1 beat\n- Eighth note: 1/2 beat\n- Sixteenth note: 1/4 beat\n\nRests: silences with the same durations.\nA dot after a note adds half its value: dotted half = 3 beats.", 1),
        (27, "Time Signatures", "Time signature: two numbers at the start of a piece.\n\nTop number: how many beats per measure.\nBottom number: which note gets one beat.\n\n4/4 (common time): 4 quarter-note beats per measure. Most popular.\n3/4 (waltz time): 3 quarter-note beats. Used in waltzes.\n6/8: 6 eighth-note beats, grouped in 2 sets of 3.\n2/4: 2 quarter-note beats. Marches.\n\nTempo: speed of the beat (BPM — beats per minute).\nAllegro ≈ 120-156 BPM, Adagio ≈ 66-76 BPM.", 2),
        (28, "Instrument Families", "Orchestra instruments in four families:\n\nStrings: violin, viola, cello, double bass, harp.\n  Sound from vibrating strings (bowed or plucked).\n\nWoodwinds: flute, clarinet, oboe, bassoon, saxophone.\n  Sound from air vibrating in a tube.\n\nBrass: trumpet, trombone, French horn, tuba.\n  Sound from buzzing lips into a metal mouthpiece.\n\nPercussion: drums, xylophone, timpani, cymbals, triangle.\n  Sound from striking, shaking, or scraping.", 1),
        (28, "Keyboard and Electronic Instruments", "Piano: 88 keys (52 white, 36 black). Covers over 7 octaves.\n  Acoustic piano uses hammers hitting strings.\n  Digital piano uses electronic samples.\n\nOrgan: sustained notes, multiple keyboards (manuals) and pedals.\n  Pipe organ: air through pipes. Electric organ: electronic circuits.\n\nSynthesizer: creates and manipulates electronic sounds.\n  Can imitate any instrument or create entirely new sounds.\n  Key concepts: oscillators, filters, envelopes.\n\nMIDI: digital protocol that sends note data between instruments and computers.", 2),
        (29, "Classical Music Periods", "Western classical music eras:\n\nBaroque (1600-1750): Bach, Vivaldi, Handel.\n  Ornate, complex. Harpsichord era.\n\nClassical (1750-1820): Mozart, Haydn, early Beethoven.\n  Balance, clarity. Sonata form.\n\nRomantic (1820-1900): Chopin, Tchaikovsky, Brahms.\n  Emotion, expression, larger orchestras.\n\n20th Century: Debussy, Stravinsky, Shostakovich.\n  Experimentation, dissonance, new scales.\n\nEach era built on the previous one while rebelling against its rules.", 1),
        (29, "Popular Music Genres", "Major genres and their origins:\n\nBlues (1870s): African American spirituals. 12-bar form. Mississippi Delta.\nJazz (1900s): Improvisation. New Orleans. Louis Armstrong, Miles Davis.\nRock & Roll (1950s): Chuck Berry, Elvis. Electric guitar driven.\nSoul/R&B (1960s): Ray Charles, Aretha Franklin. Gospel + blues.\nHip Hop (1970s): DJing, MCing, breakdancing. The Bronx, NYC.\nElectronic (1980s+): Synthesizers, drum machines. Kraftwerk, house, techno.\nPop: Catchy melodies, broad appeal. Constantly evolving.\n\nGenres blend and influence each other continuously.", 2),
        // Art (topic_id=30-33)
        (30, "The Color Wheel", "Primary colors: Red, Blue, Yellow — cannot be mixed from others.\nSecondary colors: made by mixing two primaries.\n  Red + Blue = Purple\n  Blue + Yellow = Green\n  Red + Yellow = Orange\n\nTertiary colors: mix of primary + adjacent secondary (e.g. red-orange).\n\nWarm colors: reds, oranges, yellows — energy, excitement.\nCool colors: blues, greens, purples — calm, distance.\n\nComplementary colors: opposite on the wheel (red/green, blue/orange). High contrast.", 1),
        (30, "Color Properties", "Every color has three properties:\n\nHue: the name of the color (red, blue, green).\nValue: how light or dark (add white = tint, add black = shade).\nSaturation: how vivid or muted (pure vs. grayish).\n\nColor schemes:\n- Monochromatic: one hue, different values.\n- Analogous: neighboring colors (blue, blue-green, green).\n- Complementary: opposite colors for contrast.\n- Triadic: three equally spaced colors.\n\nColor psychology: red = passion/danger, blue = trust/calm, green = nature/growth.", 2),
        (31, "The Seven Elements", "The building blocks of all visual art:\n\n1. Line: a moving point. Straight, curved, thick, thin.\n2. Shape: 2D enclosed area. Geometric (circle, square) or organic (cloud, leaf).\n3. Form: 3D shape with volume. Sphere, cube, cylinder.\n4. Color: hue, value, saturation.\n5. Value: lightness and darkness. Creates contrast and depth.\n6. Texture: how something feels or looks like it feels. Smooth, rough, bumpy.\n7. Space: the area around, between, and within objects. Positive vs. negative space.", 1),
        (31, "Principles of Design", "How elements are organized in art:\n\n1. Balance: visual weight distributed evenly. Symmetrical or asymmetrical.\n2. Contrast: differences that create interest (light vs. dark, big vs. small).\n3. Emphasis: focal point — what grabs your eye first.\n4. Movement: how the eye travels across the artwork.\n5. Pattern: repeating elements (stripes, dots, motifs).\n6. Rhythm: pattern with variation — creates visual tempo.\n7. Unity: everything works together as a whole.", 2),
        (32, "Ancient and Renaissance Art", "Ancient art:\n- Cave paintings (40,000+ years ago) — Lascaux, Altamira.\n- Egyptian: stylized figures, hieroglyphs, tomb art.\n- Greek: idealized human form, pottery, sculpture (Venus de Milo).\n- Roman: mosaics, frescoes, realistic portraits.\n\nRenaissance (1400-1600):\n- Rebirth of classical ideals in Italy.\n- Perspective, anatomy, realism.\n- Key artists: Leonardo da Vinci (Mona Lisa), Michelangelo (Sistine Chapel), Raphael.\n- Oil painting became dominant medium.", 1),
        (32, "Modern Art Movements", "Impressionism (1870s): light, color, fleeting moments. Monet, Renoir, Degas.\nPost-Impressionism: bold color, structure. Van Gogh, Cézanne, Gauguin.\nCubism (1907): fragmented forms, multiple viewpoints. Picasso, Braque.\nSurrealism (1920s): dreams, the unconscious. Dalí, Magritte.\nAbstract Expressionism (1940s): gesture, emotion. Pollock, Rothko.\nPop Art (1960s): mass culture, consumerism. Warhol, Lichtenstein.\nStreet Art: Banksy, Keith Haring — art outside galleries.\n\nContemporary art: anything goes — video, installation, digital, performance.", 2),
        (33, "Composing a Picture", "Composition: how elements are arranged in an artwork.\n\nRule of Thirds: divide the frame into a 3×3 grid. Place key elements at intersections.\n\nLeading lines: guide the viewer's eye into the image.\nFraming: use elements to frame the subject (doorways, branches).\nSymmetry: creates calm, formal feel.\nAsymmetry: dynamic, interesting.\n\nForeground/middleground/background: creates depth.\nNegative space: empty areas that give the subject room to breathe.", 1),
        (33, "Visual Hierarchy", "Visual hierarchy controls what the viewer sees first, second, third.\n\nTools:\n- Size: bigger = more important.\n- Color: bright/contrasting colors attract attention.\n- Position: top and center get noticed first.\n- Contrast: high contrast stands out.\n- Isolation: an element alone draws the eye.\n- Detail: detailed areas attract attention over plain areas.\n\nUsed in art, design, advertising, and web design.\nAsk: 'Where does my eye go first?' — that reveals the hierarchy.", 2),
        // Philosophy — Logic & Reasoning (topic_id=34)
        (34, "What Is Logic?", "Logic is the study of correct reasoning.\n\nArguments have:\n- Premises: statements assumed to be true\n- Conclusion: what follows from the premises\n\nValid argument: if premises are true, conclusion MUST be true.\nSound argument: valid AND premises are actually true.\n\nExample:\n  Premise 1: All cats are animals.\n  Premise 2: Whiskers is a cat.\n  Conclusion: Whiskers is an animal. ✓ (valid and sound)", 1),
        (34, "Common Logical Fallacies", "Fallacies are errors in reasoning that seem persuasive but are flawed.\n\nAd Hominem: attacking the person, not the argument.\nStraw Man: misrepresenting someone's argument to make it easier to attack.\nAppeal to Authority: 'A famous person said it, so it must be true.'\nFalse Dilemma: presenting only two options when more exist.\nSlippery Slope: claiming one event will inevitably lead to extreme consequences.\nCircular Reasoning: using the conclusion as a premise.\n\nSpotting fallacies helps you think critically about arguments you encounter daily.", 2),
        // Philosophy — Ethics & Morality (topic_id=35)
        (35, "Introduction to Ethics", "Ethics asks: What is right and wrong? How should we live?\n\nThree major frameworks:\n\n1. Consequentialism: judge actions by their outcomes.\n   Utilitarianism: the right action produces the most happiness for the most people.\n\n2. Deontology: some actions are right or wrong regardless of consequences.\n   Kant's rule: act only by rules you'd want everyone to follow.\n\n3. Virtue Ethics: focus on character, not rules.\n   Aristotle: cultivate virtues like courage, honesty, and wisdom.", 1),
        (35, "Moral Dilemmas", "The Trolley Problem: A runaway trolley will kill 5 people. You can pull a lever to divert it to a side track where it will kill 1 person. Should you?\n\nUtilitarian answer: Pull the lever (saves more lives).\nDeontological answer: Don't pull (you shouldn't actively cause someone's death).\n\nThe Heinz Dilemma: A man's wife is dying. The only medicine costs $10,000. He can only afford $1,000. Should he steal it?\n\nThese dilemmas have no 'right' answer — they reveal how we reason about morality.", 2),
        // Philosophy — Famous Philosophers (topic_id=36)
        (36, "Ancient Philosophers", "Socrates (469-399 BCE): 'I know that I know nothing.'\n  Method: ask questions to expose contradictions (Socratic method).\n  Executed for 'corrupting the youth' — chose death over silence.\n\nPlato (428-348 BCE): Student of Socrates.\n  Theory of Forms: the physical world is a shadow of perfect ideals.\n  Founded the Academy — arguably the first university.\n\nAristotle (384-322 BCE): Student of Plato.\n  Logic, biology, ethics, politics — father of many fields.\n  Golden Mean: virtue is the balance between extremes.\n  Tutored Alexander the Great.", 1),
        (36, "Modern Philosophers", "René Descartes (1596-1650): 'I think, therefore I am.'\n  Radical doubt: question everything until you find certainty.\n\nImmanuel Kant (1724-1804): Categorical Imperative.\n  Act only by rules you'd want to be universal laws.\n\nJohn Stuart Mill (1806-1873): Utilitarianism.\n  The greatest good for the greatest number.\n\nFriedrich Nietzsche (1844-1900): 'God is dead.'\n  Questioned traditional morality. Übermensch: create your own values.\n\nSimone de Beauvoir (1908-1986): 'One is not born, but rather becomes, a woman.'\n  Existentialism and feminism.", 2),
        // Philosophy — Thought Experiments (topic_id=37)
        (37, "Classic Thought Experiments", "Thought experiments test ideas with imagination instead of equipment.\n\nPlato's Cave: Prisoners chained in a cave see only shadows on a wall. They believe shadows are reality. One escapes and sees the sun — the true source of light. Lesson: our perceptions may be limited. Seek deeper truth.\n\nShip of Theseus: If you replace every plank of a ship one by one, is it still the same ship? What if you rebuild the old planks into a second ship? Which is the 'real' one? Explores: identity and change.\n\nBrain in a Vat: How do you know you're not a brain in a jar, being fed fake sensory experiences? (Inspired The Matrix!) Explores: the limits of knowledge.", 1),
        (37, "Modern Thought Experiments", "The Chinese Room (John Searle, 1980): A person in a room follows rules to respond to Chinese messages, producing perfect Chinese output — but understands nothing. Does the room 'understand' Chinese? Challenges: can computers truly think?\n\nThe Experience Machine (Robert Nozick, 1974): Would you plug into a machine that gives you any experience you want — but none of it is real? Most people say no. Shows: we value authenticity, not just pleasure.\n\nThe Original Position (John Rawls, 1971): Design a society without knowing what position you'd occupy (rich/poor, healthy/sick). What rules would you choose? Most choose fairness. Foundation of modern theories of justice.", 2),
        // Economics — Supply & Demand (topic_id=38)
        (38, "The Law of Supply and Demand", "Supply: how much producers are willing to sell at each price.\nDemand: how much consumers want to buy at each price.\n\nLaw of Demand: as price rises, demand falls (and vice versa).\nLaw of Supply: as price rises, supply increases.\n\nEquilibrium: where supply meets demand.\n  At this price, everything produced gets bought.\n\nShortage: demand > supply (price too low).\nSurplus: supply > demand (price too high).\n\nExample: Concert tickets at $50 sell out (shortage). At $500, seats are empty (surplus). Equilibrium is somewhere between.", 1),
        (38, "Price Elasticity", "Elasticity measures how sensitive demand is to price changes.\n\nElastic demand: small price change → big demand change.\n  Examples: luxury goods, entertainment, restaurants.\n\nInelastic demand: price changes don't affect demand much.\n  Examples: medicine, gasoline, electricity.\n\nFormula: % change in quantity demanded ÷ % change in price.\n  >1 = elastic, <1 = inelastic, =1 = unit elastic.\n\nWhy it matters: companies set prices based on elasticity.\nA hospital can charge more for life-saving drugs (inelastic).\nA movie theater must lower prices to fill seats (elastic).", 2),
        // Economics — Money & Banking (topic_id=39)
        (39, "What Is Money?", "Money is anything widely accepted in exchange for goods and services.\n\nFunctions of money:\n1. Medium of exchange: buy and sell without bartering.\n2. Store of value: save purchasing power for later.\n3. Unit of account: measure and compare value.\n\nHistory: shells → coins → paper → digital.\n\nFiat money: value from government decree, not gold.\n  All modern currencies are fiat.\n\nInflation: when money buys less over time.\n  $1 in 1950 ≈ $12 today.\n  Caused by: more money in circulation, higher costs, strong demand.", 1),
        (39, "Banks and Interest", "Banks serve two main functions:\n1. Accept deposits (savings accounts).\n2. Make loans (mortgages, business loans).\n\nInterest: the cost of borrowing money (or the reward for saving).\n  Simple interest: calculated on the original amount.\n  Compound interest: calculated on amount + accumulated interest.\n  $1,000 at 5% compound interest → $1,629 after 10 years!\n\nCentral banks (e.g. Federal Reserve, ECB):\n- Set base interest rates.\n- Control money supply.\n- Goal: stable prices and full employment.\n\nThe money multiplier: banks lend most of what they receive, which gets deposited again and lent again.", 2),
        // Economics — Trade & Globalization (topic_id=40)
        (40, "International Trade", "Why countries trade: comparative advantage.\n  Even if Country A makes everything cheaper, both countries benefit\n  if each specializes in what they're relatively best at.\n\nExports: goods/services sold to other countries.\nImports: goods/services bought from other countries.\nTrade balance: exports - imports.\n  Surplus: exports > imports.\n  Deficit: imports > exports.\n\nTariffs: taxes on imports (protect domestic industry but raise prices).\nFree trade agreements: reduce barriers (EU single market, USMCA).\n\nGlobalization: the increasing interconnection of world economies.\nBenefits: lower prices, more variety, economic growth.\nChallenges: job displacement, inequality, environmental concerns.", 1),
        (40, "Exchange Rates", "Exchange rate: the price of one currency in terms of another.\n  1 EUR = 1.10 USD means 1 euro buys 1.10 dollars.\n\nStrong currency: buys more foreign goods (imports cheaper).\nWeak currency: exports become cheaper for foreigners.\n\nWhat affects rates:\n- Interest rates (higher → stronger currency).\n- Inflation (lower → stronger currency).\n- Political stability.\n- Trade balances.\n\nFloating rates: determined by market supply and demand (most currencies).\nFixed/pegged rates: government sets the rate (e.g. Hong Kong dollar).", 2),
        // Economics — Economic Systems (topic_id=41)
        (41, "Types of Economic Systems", "How societies organize production and distribution:\n\nMarket Economy (Capitalism):\n  Private ownership. Prices set by supply/demand.\n  Pros: innovation, efficiency, freedom.\n  Cons: inequality, market failures.\n  Examples: USA, Switzerland, Singapore.\n\nCommand Economy (Planned):\n  Government controls production and prices.\n  Pros: can ensure equality, direct resources.\n  Cons: inefficiency, lack of innovation, reduced freedom.\n  Historical: Soviet Union, Maoist China.\n\nMixed Economy: combines market and government involvement.\n  Most modern economies are mixed.\n  Examples: Germany, Sweden, Japan.\n\nTraditional Economy: based on customs and traditions.\n  Found in some indigenous communities.", 1),
        (41, "GDP and Economic Indicators", "GDP (Gross Domestic Product): total value of all goods and services produced in a country in a year.\n  Nominal GDP: measured in current prices.\n  Real GDP: adjusted for inflation.\n  GDP per capita: GDP ÷ population (standard of living).\n\nOther key indicators:\n  Unemployment rate: % of workforce without jobs.\n  Inflation rate: % increase in prices per year.\n  CPI (Consumer Price Index): tracks price of a basket of goods.\n\nBusiness cycle: expansion → peak → recession → trough → recovery.\n  Recession: two consecutive quarters of declining GDP.\n\nHDI (Human Development Index): combines income, education, and life expectancy.", 2),
        // Creative Writing — Story Structure (topic_id=50)
        (50, "The Three-Act Structure", "Most stories follow three acts:\n\nAct 1 — Setup (25%): introduce characters, setting, and the inciting incident.\n  The inciting incident disrupts the character's normal world.\n\nAct 2 — Confrontation (50%): rising action, obstacles, and complications.\n  The midpoint raises the stakes. The character faces their greatest challenge.\n\nAct 3 — Resolution (25%): climax and denouement.\n  The climax is the turning point. The denouement ties up loose ends.\n\nExample: In The Wizard of Oz:\n  Act 1: Dorothy arrives in Oz (inciting incident: tornado).\n  Act 2: She travels to Emerald City, faces the Witch.\n  Act 3: She defeats the Witch and returns home.", 1),
        (50, "Narrative Arcs and Conflict", "Every story needs conflict — a problem that drives the plot.\n\nTypes of conflict:\n- Person vs. Person: hero vs. villain.\n- Person vs. Self: internal struggle (fear, doubt).\n- Person vs. Nature: survival against the elements.\n- Person vs. Society: challenging norms or injustice.\n- Person vs. Technology: AI, machines, progress.\n\nKurt Vonnegut's story shapes:\n- Rags to Riches: steady rise (Cinderella).\n- Riches to Rags: steady fall (tragedy).\n- Man in a Hole: fall then rise (most adventure stories).\n- Icarus: rise then fall (cautionary tales).\n\nThe best stories combine external conflict with internal growth.", 2),
        // Creative Writing — Character Development (topic_id=51)
        (51, "Building Memorable Characters", "Great characters feel real because they have:\n\n1. Desire: what they want (external goal).\n2. Need: what they actually need (often different from desire).\n3. Flaw: a weakness that creates obstacles.\n4. Backstory: past experiences that shape them.\n\nShow, don't tell:\n  BAD: 'Sarah was brave.'\n  GOOD: 'Sarah stepped between the snarling dog and the trembling child.'\n\nCharacter arc: how a character changes over the story.\n  Positive arc: flawed → grows (most protagonists).\n  Negative arc: good → corrupted (tragedy).\n  Flat arc: stays the same, changes the world around them (mentor figures).", 1),
        (51, "Character Motivation and Backstory", "Motivation answers: Why does this character do what they do?\n\nMaslow's hierarchy applies to characters too:\n- Survival stories: physiological/safety needs.\n- Romance: belonging needs.\n- Coming-of-age: esteem and self-actualization.\n\nIceberg technique (Hemingway): know 10× more about your character than appears on the page. The depth shows through.\n\nCharacter questionnaire starters:\n- What is their greatest fear?\n- What would they die for?\n- What do they lie about?\n- What habit do they have when nervous?\n- What's in their pockets right now?\n\nContradiction makes characters interesting:\n  A tough soldier who writes poetry. A shy librarian who's a fierce negotiator.", 2),
        // Creative Writing — Dialogue & Voice (topic_id=52)
        (52, "Writing Natural Dialogue", "Good dialogue does double duty — it reveals character AND advances the plot.\n\nRules of thumb:\n1. Each character should sound distinct. Remove the name tags — can you tell who's speaking?\n2. People rarely say exactly what they mean. Subtext is key.\n3. Read dialogue aloud. If it sounds stiff, rewrite it.\n4. Use contractions (don't, won't, can't) — people speak casually.\n5. Avoid 'said bookisms': he exclaimed, she articulated. 'Said' is invisible.\n\nSubtext example:\n  She asked: 'How's the new job?'\n  What she meant: 'Are you going to be able to pay me back?'\n\nDialogue tags: 'said' and 'asked' are enough 90% of the time.\nAction beats replace tags: 'He slammed the door. \"We're done.\"'", 1),
        (52, "Finding Your Voice", "Voice is your unique writing fingerprint — the rhythm, word choice, and personality.\n\nNarrative voice types:\n- First person (I/me): intimate, limited perspective.\n- Third person limited (she/he): follows one character's thoughts.\n- Third person omniscient: knows everything, all characters.\n- Second person (you): rare, immersive (used in Choose Your Own Adventure).\n\nDeveloping voice:\n- Write a lot. Voice emerges through practice.\n- Read widely. Notice what you're drawn to.\n- Write like you talk, then refine.\n- Break grammar rules deliberately (fragments. For effect.)\n\nTone vs. voice:\n  Voice is WHO is speaking. Tone is HOW they feel about the subject.\n  Same voice can have different tones (humorous, serious, nostalgic).", 2),
        // Creative Writing — Poetry Fundamentals (topic_id=53)
        (53, "Poetry Forms and Devices", "Poetry compresses language for maximum emotional impact.\n\nCommon forms:\n- Free verse: no fixed meter or rhyme. Most modern poetry.\n- Sonnet: 14 lines, iambic pentameter. Shakespeare, Petrarch.\n- Haiku: 3 lines (5-7-5 syllables). Captures a moment.\n- Limerick: 5 lines (AABBA). Humorous.\n- Villanelle: 19 lines with repeating lines. ('Do Not Go Gentle' — Dylan Thomas).\n\nSound devices:\n- Alliteration: same starting sound (Peter Piper picked).\n- Assonance: repeated vowel sounds (the rain in Spain).\n- Onomatopoeia: words that sound like their meaning (buzz, crash, whisper).\n- Rhyme: end rhyme, internal rhyme, slant rhyme.", 1),
        (53, "Imagery and Figurative Language", "Imagery engages the five senses:\n- Visual: 'The crimson leaves carpeted the path.'\n- Auditory: 'The creek whispered over smooth stones.'\n- Tactile: 'Sandpaper rough under her fingertips.'\n- Olfactory: 'The kitchen smelled of cinnamon and burnt toast.'\n- Gustatory: 'The lemon was sharp enough to make her wince.'\n\nFigurative language:\n- Simile: comparison using like/as. 'Life is like a box of chocolates.'\n- Metaphor: direct comparison. 'Time is a thief.'\n- Personification: giving human traits to non-human things. 'The wind howled.'\n- Hyperbole: exaggeration. 'I've told you a million times.'\n- Symbolism: an object represents something larger. A rose = love.\n\nRule: use figurative language to clarify, not to show off.", 2),
        // Astronomy — The Solar System (topic_id=54)
        (54, "Our Solar System", "The Solar System formed ~4.6 billion years ago from a cloud of gas and dust.\n\nThe Sun: a medium-sized star containing 99.86% of the system's mass.\n  Surface temperature: ~5,500°C. Core: ~15 million°C.\n\nPlanets (from Sun outward):\n  Inner (rocky): Mercury, Venus, Earth, Mars.\n  Outer (gas/ice giants): Jupiter, Saturn, Uranus, Neptune.\n\nOther objects:\n- Dwarf planets: Pluto, Eris, Ceres.\n- Asteroid belt: between Mars and Jupiter.\n- Kuiper belt: beyond Neptune.\n- Oort cloud: outermost boundary.\n\nScale: light from the Sun reaches Earth in ~8 minutes. It takes ~4 hours to reach Neptune.", 1),
        (54, "Planets Up Close", "Mercury: smallest, no atmosphere, extreme temps (-180°C to 430°C). Orbits Sun in 88 days.\nVenus: hottest planet (465°C), thick CO₂ atmosphere, rotates backward.\nEarth: the Goldilocks planet — liquid water, atmosphere, magnetic field.\nMars: the Red Planet, thin atmosphere, Olympus Mons (tallest volcano in solar system).\n\nJupiter: largest planet, Great Red Spot (storm bigger than Earth). 79+ moons including Europa (subsurface ocean).\nSaturn: famous rings (ice and rock), density less than water. Moon Titan has lakes of methane.\nUranus: tilted 98° on its side, ice giant.\nNeptune: windiest planet (2,100 km/h), deep blue color, moon Triton orbits backward.", 2),
        // Astronomy — Stars & Stellar Evolution (topic_id=55)
        (55, "The Life of a Star", "Stars are born in nebulae — vast clouds of gas and dust.\n\nStellar lifecycle:\n1. Nebula: gravity pulls gas together.\n2. Protostar: heats up as it collapses.\n3. Main sequence: hydrogen fusion begins. Our Sun is here.\n   Lasts billions of years (longer for smaller stars).\n4. Red giant: hydrogen runs out, star expands.\n5. Death depends on mass:\n   - Low/medium mass → planetary nebula → white dwarf.\n   - High mass → supernova → neutron star or black hole.\n\nHertzsprung-Russell diagram: plots stars by temperature vs. luminosity.\nMost stars lie on the 'main sequence' diagonal.", 1),
        (55, "Types of Stars", "Stars are classified by temperature and color (spectral type: O B A F G K M).\nMnemonic: Oh Be A Fine Girl/Guy, Kiss Me.\n\nO-type: blue, hottest (>30,000 K), short-lived (millions of years).\nG-type: yellow, like our Sun (~5,800 K), lives ~10 billion years.\nM-type: red, coolest (<3,500 K), most common, lives trillions of years.\n\nSpecial objects:\n- Red dwarfs: most stars in the universe. Small, dim, extremely long-lived.\n- White dwarfs: dead cores of medium stars. Earth-sized but Sun's mass.\n- Neutron stars: incredibly dense. A teaspoon weighs ~6 billion tons.\n- Black holes: gravity so strong even light can't escape. Stellar or supermassive.\n\nNearest star (besides Sun): Proxima Centauri, 4.24 light-years away.", 2),
        // Astronomy — Galaxies & the Universe (topic_id=56)
        (56, "Galaxies", "A galaxy is a gravitationally bound system of stars, gas, dust, and dark matter.\n\nTypes of galaxies:\n- Spiral: flat disk with arms (Milky Way, Andromeda). ~77% of observed galaxies.\n- Elliptical: round/oval, older stars, little gas. From small to giant.\n- Irregular: no defined shape (Magellanic Clouds).\n\nThe Milky Way:\n- ~200-400 billion stars.\n- ~100,000 light-years across.\n- Our Sun is ~26,000 light-years from the center.\n- A supermassive black hole (Sagittarius A*) sits at the center — 4 million solar masses.\n\nGalaxy groups and clusters: galaxies aren't alone.\n  The Milky Way is part of the Local Group (~54 galaxies).\n  The observable universe contains ~2 trillion galaxies.", 1),
        (56, "The Expanding Universe", "Edwin Hubble (1929) discovered that galaxies are moving apart — the universe is expanding.\n\nThe Big Bang theory:\n- ~13.8 billion years ago, all matter and energy was in an incredibly hot, dense point.\n- It expanded rapidly (not an explosion 'in' space — space itself expanded).\n- As it cooled: quarks → protons/neutrons → atoms → stars → galaxies.\n\nEvidence for the Big Bang:\n1. Cosmic Microwave Background (CMB): radiation left over from 380,000 years after the Big Bang.\n2. Hubble's Law: farther galaxies move away faster.\n3. Abundance of hydrogen and helium matches predictions.\n\nDark matter: ~27% of the universe. Invisible but detectable by gravity.\nDark energy: ~68% of the universe. Accelerating the expansion.\nOrdinary matter: only ~5% of everything!", 2),
        // Astronomy — Space Exploration (topic_id=57)
        (57, "History of Space Exploration", "Key milestones:\n\n1957: Sputnik 1 — first artificial satellite (USSR).\n1961: Yuri Gagarin — first human in space (USSR).\n1969: Apollo 11 — Neil Armstrong and Buzz Aldrin walk on the Moon (USA).\n1971: First space station (Salyut 1, USSR).\n1977: Voyager 1 & 2 launched — still sending data from interstellar space.\n1990: Hubble Space Telescope launched.\n1998: International Space Station (ISS) construction begins.\n2012: Curiosity rover lands on Mars.\n2019: First image of a black hole (M87*).\n2021: James Webb Space Telescope launched.\n2022: DART mission successfully redirects an asteroid.\n\nThe Space Race (1957-1975) between USA and USSR drove rapid advancement.", 1),
        (57, "The Future of Space", "Current and future missions:\n\nMars exploration:\n- NASA's Perseverance rover searching for ancient life.\n- SpaceX Starship: aims for crewed Mars missions in the 2030s.\n- Challenges: 6-9 month journey, radiation, thin atmosphere, no return plan yet.\n\nArtemis program: returning humans to the Moon. Gateway lunar station planned.\n\nPrivate spaceflight:\n- SpaceX: reusable rockets, Starlink satellite internet.\n- Blue Origin: space tourism, lunar lander.\n- Virgin Galactic: suborbital flights.\n\nBig questions:\n- Is there life elsewhere? (Europa, Enceladus, Mars, exoplanets)\n- Can we become a multi-planetary species?\n- Will we detect signals from intelligent civilizations?\n\nFermi Paradox: if the universe is so big, where is everyone?", 2),
        // Physics — Electricity & Magnetism (topic_id=58)
        (58, "Electric Charge and Current", "Electric charge is a fundamental property of matter. Protons are positive (+), electrons are negative (-).\n\nLike charges repel, opposite charges attract.\n\nElectric current is the flow of charge (electrons) through a conductor.\n- Measured in amperes (A)\n- 1 ampere = 1 coulomb of charge per second\n\nVoltage (V): the 'push' that drives current. Like water pressure in a pipe.\nResistance (Ω): opposition to current flow. Like a narrow pipe.\n\nOhm's Law: V = I × R\n  Voltage = Current × Resistance\n  12V = 2A × 6Ω\n\nDirect Current (DC): flows one direction (batteries).\nAlternating Current (AC): reverses direction (wall outlets, 50/60 Hz).", 1),
        (58, "Magnetism and Electromagnetism", "Magnets have north and south poles. Like poles repel, opposite attract.\n\nMagnetic fields: invisible lines of force from north to south pole.\n\nKey discovery (Ørsted, 1820): electric current creates a magnetic field!\n\nElectromagnets: coil of wire + current = controllable magnet.\n  Used in: motors, speakers, MRI machines, maglev trains.\n\nFaraday's Law: a changing magnetic field induces electric current.\n  This is how generators work — spin a magnet near a coil → electricity.\n\nThe relationship between electricity and magnetism (electromagnetism) is one of physics' greatest unifications. Maxwell's equations describe it all in four elegant formulas.", 2),
        // Physics — Thermodynamics (topic_id=59)
        (59, "Heat and Temperature", "Temperature measures average kinetic energy of particles.\n  Celsius: 0°C = water freezes, 100°C = water boils\n  Kelvin: absolute scale. 0 K = absolute zero (-273.15°C)\n  Fahrenheit: 32°F = freezing, 212°F = boiling\n\nHeat is energy transferred between objects of different temperatures.\n  Flows from hot → cold (never the reverse spontaneously)\n\nHeat transfer mechanisms:\n1. Conduction: through direct contact (metal spoon in hot soup)\n2. Convection: through fluid movement (hot air rising)\n3. Radiation: through electromagnetic waves (sunlight)\n\nSpecific heat capacity: energy needed to raise 1 kg by 1°C.\n  Water has very high specific heat (4,186 J/kg·°C) — why oceans moderate climate.", 1),
        (59, "Laws of Thermodynamics", "Four laws that govern energy and entropy:\n\nZeroth Law: If A is in thermal equilibrium with B, and B with C, then A is with C.\n  (This is why thermometers work!)\n\nFirst Law: Energy cannot be created or destroyed, only transformed.\n  ΔU = Q - W (internal energy change = heat added - work done)\n  This is conservation of energy.\n\nSecond Law: Entropy (disorder) of an isolated system always increases.\n  Heat flows hot → cold. You can't unscramble an egg.\n  Efficiency of any heat engine < 100%.\n\nThird Law: As temperature approaches absolute zero, entropy approaches a minimum.\n  You can never reach exactly 0 K.\n\nEntropy explains the arrow of time — why we remember the past but not the future.", 2),
        // Physics — Waves & Sound (topic_id=60)
        (60, "Properties of Waves", "A wave transfers energy without transferring matter.\n\nTwo main types:\n- Transverse: oscillation perpendicular to direction (light, water surface)\n- Longitudinal: oscillation parallel to direction (sound, compression springs)\n\nWave properties:\n- Wavelength (λ): distance between two crests\n- Frequency (f): waves per second (Hz)\n- Amplitude: height of wave (relates to energy/loudness)\n- Speed: v = f × λ\n\nWave behaviors:\n- Reflection: bouncing off a surface (echo, mirror)\n- Refraction: bending when entering a new medium (straw in water looks bent)\n- Diffraction: spreading around obstacles\n- Interference: waves combining (constructive = louder, destructive = quieter)", 1),
        (60, "Sound", "Sound is a longitudinal wave — compressions and rarefactions in a medium.\n\nSpeed of sound:\n- In air (~20°C): 343 m/s\n- In water: ~1,480 m/s\n- In steel: ~5,960 m/s\n- Cannot travel through vacuum (no medium)\n\nPitch: determined by frequency.\n  Human hearing: 20 Hz to 20,000 Hz\n  Middle C on piano: 262 Hz\n  Infrasound: < 20 Hz (elephants communicate with it)\n  Ultrasound: > 20,000 Hz (bats, medical imaging)\n\nLoudness: determined by amplitude. Measured in decibels (dB).\n  Whisper: ~30 dB, Conversation: ~60 dB, Rock concert: ~110 dB\n  >85 dB can cause hearing damage over time.\n\nDoppler effect: pitch changes when source moves relative to listener.\n  Ambulance siren sounds higher approaching, lower receding.", 2),
        // Physics — Nuclear Physics (topic_id=61)
        (61, "Atomic Structure and Radioactivity", "The atom: nucleus (protons + neutrons) surrounded by electron cloud.\n  Protons: positive charge, define the element (atomic number)\n  Neutrons: neutral, add mass. Isotopes = same element, different neutrons\n  Electrons: negative charge, determine chemical behavior\n\nNucleus is tiny: if atom were a stadium, nucleus would be a marble at center.\n  But contains 99.95% of the atom's mass!\n\nRadioactivity: unstable nuclei emit radiation to become stable.\n  Alpha (α): 2 protons + 2 neutrons (helium nucleus). Stopped by paper.\n  Beta (β): electron or positron. Stopped by aluminum.\n  Gamma (γ): high-energy photon. Needs lead or thick concrete.\n\nHalf-life: time for half the radioactive atoms to decay.\n  Carbon-14: 5,730 years (used for dating ancient objects)\n  Uranium-238: 4.5 billion years (dating rocks)", 1),
        (61, "Fission, Fusion, and E=mc²", "Einstein's famous equation: E = mc²\n  Energy = mass × speed of light squared\n  A tiny amount of mass converts to enormous energy.\n  1 kg of matter = ~90 petajoules (≈ 21 megatons of TNT)\n\nNuclear Fission: splitting heavy atoms (uranium, plutonium).\n  Used in: nuclear power plants, atomic bombs.\n  Chain reaction: one split releases neutrons that split more atoms.\n  Nuclear power provides ~10% of world's electricity. No CO₂ during operation.\n  Challenge: radioactive waste, meltdown risk.\n\nNuclear Fusion: combining light atoms (hydrogen → helium).\n  Powers the Sun and all stars.\n  Potential for virtually unlimited clean energy on Earth.\n  Challenge: requires 100+ million °C. Containment is incredibly difficult.\n  Active research: ITER (France), NIF (USA), private ventures.\n  'Fusion is always 30 years away' — but real progress is being made.", 2),
        // Additional lessons for previously under-served topics
        // Reading Comprehension (topic_id=11) — add a second lesson
        (11, "Inference and Context Clues", "Inference means drawing conclusions from evidence, not just what's directly stated.\n\nTypes of context clues:\n- Definition: the word is defined in the sentence.\n- Synonym: a similar word appears nearby.\n- Antonym: an opposite word provides contrast.\n- Example: examples help clarify meaning.\n\nPractice:\n'The arid desert received less than 10cm of rain per year.'\n  Clue: desert, little rain → arid means very dry.\n\nMaking inferences:\n- What does the author imply but not say?\n- What evidence supports your conclusion?\n- Could there be another interpretation?", 2),
        // Ancient Civilizations (topic_id=12) — add a second lesson
        (12, "Contributions of Ancient Civilizations", "Lasting gifts from the ancient world:\n\nMesopotamia: writing, the wheel, the 60-minute hour, legal codes (Hammurabi).\nEgypt: calendar, papyrus, medicine, engineering (pyramids built to millimeter precision).\nGreece: democracy, philosophy, theater, the Olympic Games, geometry.\nRome: roads, aqueducts, legal systems, concrete, republican government.\nChina: paper, printing, gunpowder, compass.\nIndia: zero and the decimal system, chess, cotton textiles.\n\nPattern: innovation often arose where trade routes met, ideas crossed, and diverse peoples exchanged knowledge.", 2),
        // World Wars (topic_id=13) — add a second lesson
        (13, "The Aftermath and Lessons", "After WWI:\n- Treaty of Versailles: harsh penalties on Germany → economic devastation.\n- League of Nations formed (but USA didn't join).\n- Rise of fascism in Italy and Germany.\n\nAfter WWII:\n- United Nations founded (1945) — 193 member states today.\n- Marshall Plan: USA rebuilt Europe.\n- Cold War begins: USA vs. Soviet Union.\n- Universal Declaration of Human Rights (1948).\n- Decolonization: European empires dissolved.\n\nLessons:\n- Harsh peace terms can breed future conflict.\n- International cooperation prevents war.\n- Human rights need active protection.\n- Nuclear weapons changed warfare forever.", 2),
        // Industrial Revolution (topic_id=14) — add a second lesson
        (14, "The Second Industrial Revolution", "The Second Industrial Revolution (1870-1914):\n\nKey innovations:\n- Electricity: Thomas Edison (light bulb), Nikola Tesla (AC power).\n- Internal combustion engine: Karl Benz (automobile).\n- Telephone: Alexander Graham Bell.\n- Assembly line: Henry Ford made cars affordable.\n\nImpact:\n- Mass production lowered prices.\n- Cities grew rapidly (urbanization).\n- Middle class expanded.\n- New social challenges: pollution, urban poverty.\n\nDigital Revolution (1970s-now): sometimes called the Third Industrial Revolution.\n  Computers → Internet → smartphones → AI.\n  Each revolution transformed how people live, work, and communicate.", 2),
        // Hygiene (topic_id=15) — add a second lesson
        (15, "Germ Theory and Disease Prevention", "Before germ theory, people blamed illness on bad air ('miasma').\n\nKey figures:\n- Louis Pasteur (1860s): proved microorganisms cause disease.\n- Joseph Lister: introduced antiseptic surgery.\n- Robert Koch: identified bacteria causing tuberculosis and cholera.\n\nHow diseases spread:\n- Airborne: coughing, sneezing (flu, COVID).\n- Contact: touching contaminated surfaces.\n- Waterborne: contaminated water (cholera).\n- Vector-borne: carried by insects (malaria via mosquitoes).\n\nPrevention: vaccination, sanitation, clean water, hygiene.\nVaccines have saved more lives than any other medical intervention.", 2),
        // Nutrition (topic_id=16) — add a second lesson
        (16, "Reading Food Labels", "Understanding nutrition labels empowers healthy choices.\n\nKey information:\n- Serving size: everything is based on this — check it first!\n- Calories: energy per serving.\n- Macronutrients: carbohydrates, protein, fat.\n- Micronutrients: vitamins and minerals.\n- Daily Value (%DV): how much one serving contributes to daily needs.\n  5% DV or less = low. 20% DV or more = high.\n\nWatch out for:\n- Added sugars (not the same as natural sugars in fruit).\n- Sodium: <2,300 mg/day recommended.\n- Trans fats: avoid entirely.\n\nIngredient list: items listed by weight (most to least).\nIf sugar is in the first 3 ingredients, reconsider.", 2),
        // Psychology — Introduction (topic_id=42)
        (42, "What Is Psychology?", "Psychology is the scientific study of mind and behavior.\n\nMajor branches:\n- Clinical: diagnosing and treating mental disorders.\n- Cognitive: how we think, remember, and solve problems.\n- Developmental: how people change across the lifespan.\n- Social: how others influence our behavior.\n- Biological: the brain and nervous system.\n\nHistory:\n- Wilhelm Wundt (1879): opened the first psychology lab.\n- Sigmund Freud: the unconscious mind.\n- B.F. Skinner: behaviorism — observable actions.\n- Carl Rogers: humanistic psychology — personal growth.\n\nPsychology uses the scientific method: observe, hypothesize, test, conclude.", 1),
        (42, "Nature vs. Nurture", "One of psychology's oldest debates: are we shaped by genes (nature) or environment (nurture)?\n\nNature (genetics):\n- Eye color, height, temperament tendencies.\n- Twin studies: identical twins raised apart still share traits.\n\nNurture (environment):\n- Language, values, skills, fears.\n- Culture shapes what we consider 'normal.'\n\nModern view: it's BOTH. Genes provide potential; environment shapes expression.\n- Epigenetics: environment can turn genes on or off.\n- Example: a child may inherit musical talent (nature) but needs practice to develop it (nurture).\n\nNeither alone explains human behavior.", 2),
        // Psychology — Memory & Learning (topic_id=43)
        (43, "How Memory Works", "Memory has three stages:\n\n1. Encoding: taking in information.\n   - Visual (images), acoustic (sounds), semantic (meaning).\n   - Deeper processing = better memory.\n\n2. Storage:\n   - Sensory memory: <1 second (sights, sounds).\n   - Short-term/Working memory: ~20 seconds, 7±2 items.\n   - Long-term memory: unlimited capacity, can last a lifetime.\n\n3. Retrieval: accessing stored information.\n   - Recall: producing information (essay question).\n   - Recognition: identifying information (multiple choice).\n\nForgetting curve (Ebbinghaus): we forget ~50% within an hour without review.\nSpaced repetition fights this by reviewing at optimal intervals.", 1),
        (43, "Learning Theories", "How do we learn? Major theories:\n\nClassical Conditioning (Pavlov):\n  Pair a stimulus with a response. Dog hears bell → gets food → salivates.\n  Eventually: bell alone → salivation.\n\nOperant Conditioning (Skinner):\n  Behavior shaped by consequences.\n  - Positive reinforcement: reward increases behavior (treat for good grades).\n  - Negative reinforcement: removing something unpleasant increases behavior.\n  - Punishment: decreases behavior.\n\nObservational Learning (Bandura):\n  Learn by watching others. Children imitate adults (Bobo doll experiment).\n\nConstructivism (Piaget):\n  Learners actively build knowledge from experience.\n  Stages: sensorimotor → preoperational → concrete → formal.", 2),
        // Psychology — Emotions & Motivation (topic_id=44)
        (44, "Understanding Emotions", "Emotions are complex reactions involving:\n- Physiological responses (heart rate, sweating).\n- Subjective feelings (happiness, fear).\n- Behavioral expressions (smiling, fleeing).\n\nBasic emotions (Paul Ekman): happiness, sadness, fear, anger, surprise, disgust.\nThese are universal across cultures.\n\nTheories:\n- James-Lange: body reacts first, then we feel emotion. (We feel sad because we cry.)\n- Cannon-Bard: body and feeling happen simultaneously.\n- Schachter-Singer: arousal + cognitive label = emotion.\n\nEmotional intelligence (Goleman): ability to recognize, understand, and manage emotions — in yourself and others. Predicts success as well as IQ.", 1),
        (44, "Motivation", "Motivation is what drives us to act.\n\nIntrinsic motivation: doing something for its own sake (curiosity, enjoyment).\nExtrinsic motivation: doing something for external rewards (money, grades).\n\nMaslow's Hierarchy of Needs (bottom to top):\n1. Physiological: food, water, sleep.\n2. Safety: shelter, security.\n3. Belonging: love, friendship.\n4. Esteem: achievement, respect.\n5. Self-actualization: reaching your full potential.\n\nSelf-Determination Theory (Deci & Ryan):\nThree core needs: autonomy, competence, relatedness.\nWhen met, intrinsic motivation flourishes.\n\nGrowth mindset (Carol Dweck): believing abilities can improve through effort leads to more persistence and success.", 2),
        // Psychology — Social Psychology (topic_id=45)
        (45, "How Others Influence Us", "Social psychology studies how people affect each other's thoughts and behaviors.\n\nConformity (Asch, 1951):\n  People agree with a group even when the group is clearly wrong.\n  ~75% conformed at least once. Why? Social pressure.\n\nObedience (Milgram, 1963):\n  65% of participants gave what they believed were dangerous electric shocks\n  when ordered by an authority figure.\n\nBystander Effect:\n  The more people present, the less likely anyone helps.\n  Diffusion of responsibility: 'someone else will do it.'\n\nGroupthink: groups make worse decisions when they prioritize harmony over critical thinking.", 1),
        (45, "Cognitive Biases", "Our brains take mental shortcuts (heuristics) that can lead to errors.\n\nConfirmation bias: we seek information that confirms what we already believe.\nAnchoring: the first number we see influences our judgment.\nAvailability heuristic: we judge likelihood by how easily examples come to mind.\n  Shark attacks feel common (dramatic news) but are extremely rare.\nDunning-Kruger effect: beginners overestimate their ability; experts underestimate theirs.\nHalo effect: one positive trait (attractiveness) makes us assume others (intelligence).\nSunk cost fallacy: continuing something because of past investment, not future value.\n\nAwareness of biases is the first step to better thinking.\nSlowing down and thinking deliberately (System 2) helps counter them.", 2),
        // Environmental Science — Ecosystems & Biomes (topic_id=46)
        (46, "What Is an Ecosystem?", "An ecosystem is a community of living organisms interacting with their non-living environment.\n\nComponents:\n- Biotic: plants, animals, fungi, bacteria.\n- Abiotic: water, sunlight, temperature, soil, minerals.\n\nEnergy flow:\n  Sun → producers (plants) → primary consumers (herbivores) → secondary consumers (carnivores) → decomposers.\n  Only ~10% of energy transfers between levels (10% rule).\n\nFood web: interconnected food chains in an ecosystem.\n\nBiodiversity: the variety of life in an ecosystem.\n  Higher biodiversity = more resilient ecosystem.\n  A forest with 500 species recovers better from disturbance than one with 50.", 1),
        (46, "Major Biomes", "Biomes are large regions with similar climate, plants, and animals.\n\nTerrestrial biomes:\n- Tropical rainforest: hot, wet. Greatest biodiversity. Amazon, Congo.\n- Desert: very dry (<25 cm rain/year). Sahara, Gobi.\n- Grassland/Savanna: grasses dominate. Serengeti, Great Plains.\n- Temperate forest: four seasons. Deciduous trees. Europe, eastern USA.\n- Boreal forest (Taiga): cold, coniferous trees. Canada, Russia.\n- Tundra: freezing, treeless, permafrost. Arctic.\n\nAquatic biomes:\n- Freshwater: rivers, lakes, wetlands.\n- Marine: oceans, coral reefs, deep sea.\n- Estuaries: where rivers meet the sea. Extremely productive.", 2),
        // Environmental Science — Climate Change (topic_id=47)
        (47, "The Greenhouse Effect", "The greenhouse effect is natural and essential — without it, Earth would be -18°C!\n\nHow it works:\n1. Sunlight passes through the atmosphere and warms Earth's surface.\n2. Earth radiates heat (infrared radiation) back toward space.\n3. Greenhouse gases (CO₂, methane, water vapor, N₂O) trap some of this heat.\n4. This keeps Earth warm enough for life (~15°C average).\n\nThe problem: human activities have increased greenhouse gas concentrations.\n- CO₂: burning fossil fuels, deforestation. Up 50% since 1750.\n- Methane: livestock, rice paddies, landfills. 80× more potent than CO₂ (short-term).\n- N₂O: agriculture, fertilizers.\n\nResult: enhanced greenhouse effect → global warming.", 1),
        (47, "Impacts of Climate Change", "Observed and projected effects:\n\nTemperature: Earth has warmed ~1.1°C since pre-industrial times.\n  Goal: limit to 1.5°C (Paris Agreement).\n\nSea level rise: thermal expansion + ice melt.\n  ~20 cm rise since 1900. Could reach 1 m by 2100.\n\nExtreme weather: more intense heatwaves, droughts, hurricanes, flooding.\n\nEcosystems: coral bleaching, species migration, extinction risk.\n  ~1 million species at risk.\n\nHuman impact: food security, water scarcity, climate refugees.\n\nSolutions:\n- Mitigation: reduce emissions (renewables, efficiency, carbon capture).\n- Adaptation: prepare for changes (sea walls, drought-resistant crops).\n- Individual actions: reduce, reuse, recycle; eat less meat; use public transport.", 2),
        // Environmental Science — Pollution & Waste (topic_id=48)
        (48, "Types of Pollution", "Pollution: harmful substances introduced into the environment.\n\nAir pollution:\n- Sources: vehicles, factories, power plants, wildfires.\n- Effects: respiratory disease, smog, acid rain.\n- Key pollutants: particulate matter (PM2.5), ozone, CO, SO₂, NOx.\n\nWater pollution:\n- Sources: industrial waste, agricultural runoff, sewage, plastics.\n- Effects: unsafe drinking water, dead zones, ecosystem damage.\n- 2 billion people lack safe drinking water.\n\nSoil pollution:\n- Sources: pesticides, heavy metals, industrial waste, landfills.\n- Effects: reduced crop yields, contaminated food chain.\n\nNoise and light pollution: often overlooked but affect health and wildlife.", 1),
        (48, "The Plastic Problem", "Plastic production: 400+ million tonnes per year. Only 9% is recycled.\n\nWhere it goes:\n- Landfills: takes 400-1000 years to decompose.\n- Oceans: 8 million tonnes enter oceans annually.\n  Great Pacific Garbage Patch: 1.6 million km² (3× the size of France).\n\nMicroplastics: tiny fragments (<5 mm) found everywhere.\n  In water, soil, air, food, and even human blood.\n\nImpact on wildlife:\n- Entanglement: sea turtles, seabirds, marine mammals.\n- Ingestion: animals mistake plastic for food.\n- 100,000+ marine animals die from plastic annually.\n\nSolutions:\n- Reduce single-use plastics.\n- Improve recycling infrastructure.\n- Biodegradable alternatives.\n- Extended producer responsibility.", 2),
        // Environmental Science — Conservation & Sustainability (topic_id=49)
        (49, "Biodiversity and Conservation", "Biodiversity: the variety of life at genetic, species, and ecosystem levels.\n\nWhy it matters:\n- Ecosystem services: pollination, water purification, carbon storage.\n- Medicine: 25% of drugs derived from plants.\n- Food security: genetic diversity protects crops from disease.\n- Resilience: diverse ecosystems recover faster from disturbance.\n\nThreats (HIPPO):\n- Habitat loss: #1 threat. Deforestation, urbanization.\n- Invasive species: outcompete native species.\n- Pollution: poisons ecosystems.\n- Population growth: more resources needed.\n- Overexploitation: overfishing, poaching.\n\nConservation strategies:\n- Protected areas (national parks, marine reserves).\n- Habitat restoration.\n- Species breeding programs.\n- Wildlife corridors connecting habitats.", 1),
        (49, "Sustainability", "Sustainability: meeting present needs without compromising future generations.\n\nThree pillars:\n1. Environmental: protect natural resources and ecosystems.\n2. Social: equity, health, education, community.\n3. Economic: viable economy without depleting resources.\n\nCircular economy: design out waste. Products are reused, repaired, recycled.\n  Linear (take-make-dispose) → Circular (reduce-reuse-recycle).\n\nUN Sustainable Development Goals (SDGs): 17 goals for 2030.\n  No poverty, zero hunger, clean energy, climate action, life below water, etc.\n\nIndividual actions:\n- Reduce consumption and waste.\n- Choose renewable energy.\n- Support sustainable businesses.\n- Eat locally and seasonally.\n- Educate others — knowledge multiplies impact.", 2),
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
        (18, "binary", "Binary is a number system that uses only two digits: 0 and 1.", Some("It's like a light switch — it can only be ON (1) or OFF (0). Computers chain millions of switches together!"), Some("What is the number 5 in binary?")),
        (19, "algorithms", "An algorithm is a set of step-by-step instructions to solve a problem.", Some("A recipe is an algorithm for cooking! Follow the steps in order, and you get a cake."), Some("Can you write step-by-step instructions for brushing your teeth?")),
        (20, "programming", "Programming means writing instructions that a computer can follow.", Some("You're like a director giving commands to actors — except your actors are incredibly fast but incredibly literal!"), Some("What would happen if you told a computer to repeat something forever?")),
        (22, "continents", "Continents are Earth's seven large landmasses.", Some("Think of the Earth as a giant jigsaw puzzle — the continents are the biggest pieces!"), Some("Can you name all seven continents?")),
        (23, "climate", "Climate is the average weather pattern in a place over many years.", Some("Weather is what you wear today; climate is what's in your wardrobe!"), Some("What climate zone do you live in?")),
        (26, "musical notes", "Musical notes are symbols that represent sounds of specific pitch and duration.", Some("Notes are like letters in a language — combine them and you get melodies, just like letters make words!"), Some("How many note names are there before they repeat?")),
        (27, "rhythm", "Rhythm is the pattern of long and short sounds and silences in music.", Some("Think of your heartbeat — that steady ba-DUM, ba-DUM is a rhythm!"), Some("How many beats does a whole note get in 4/4 time?")),
        (28, "instruments", "Musical instruments are devices designed to produce musical sounds.", Some("Every instrument is really just a clever way to make air vibrate — strings do it one way, drums another!"), Some("Can you name one instrument from each family?")),
        (30, "color theory", "Color theory explains how colors relate to each other and how they can be combined.", Some("Colors are like a team — some work together (analogous), some challenge each other (complementary), and that tension makes art exciting!"), Some("What two primary colors mix to make orange?")),
        (31, "elements of art", "The elements of art are the basic building blocks used to create any artwork.", Some("Think of them like ingredients in cooking — line, shape, color, form, value, texture, space. Every artwork uses some combination!"), Some("What's the difference between a shape and a form?")),
        (32, "art history", "Art history traces how visual art evolved through different periods and cultures.", Some("Art is like a conversation across centuries — each generation responds to what came before, sometimes agreeing, sometimes rebelling!"), Some("Which art movement focused on capturing light and fleeting moments?")),
        // Philosophy
        (34, "logic", "Logic is the study of correct reasoning — how to build arguments that hold up.", Some("Logic is like the grammar of thinking — just as bad grammar makes sentences confusing, bad logic makes arguments fall apart!"), Some("Can you spot what's wrong with this: 'All dogs are animals. My cat is an animal. Therefore my cat is a dog.'?")),
        (35, "ethics", "Ethics is the branch of philosophy that asks how we should live and what makes actions right or wrong.", Some("Ethics is like a compass — it doesn't tell you the exact path, but it helps you find your direction when you're lost!"), Some("If you found a wallet with $100, what would you do and why?")),
        (36, "philosophers", "Philosophers are thinkers who ask the deepest questions about reality, knowledge, and how to live.", Some("Philosophers are like explorers of the mind — while others mapped continents, they mapped the landscape of ideas!"), Some("What question would you most want a philosopher to answer?")),
        (37, "thought experiments", "Thought experiments are imaginary scenarios used to test ideas and challenge assumptions.", Some("Thought experiments are like video game levels for your brain — you can explore dangerous scenarios without any real risk!"), Some("If you could live in a perfect virtual world or an imperfect real one, which would you choose?")),
        // Economics
        (38, "supply and demand", "Supply and demand is the core mechanism that determines prices in a market economy.", Some("Supply and demand is like a tug-of-war between buyers and sellers — the price settles where the rope stops moving!"), Some("What happens to the price of umbrellas when it starts raining?")),
        (39, "money", "Money is anything widely accepted as payment — it solves the problem of bartering.", Some("Imagine trading a piano for groceries — money is the middleman that makes exchange possible!"), Some("Why can't a country just print more money to make everyone rich?")),
        (40, "trade", "International trade allows countries to specialize in what they do best and exchange with others.", Some("Trade between countries is like trading lunch items at school — everyone ends up happier when they can swap for what they really want!"), Some("Can you think of something you use daily that was made in another country?")),
        (41, "economic systems", "An economic system is how a society organizes the production and distribution of goods and services.", Some("Economic systems are like different recipes for running a country — same ingredients (land, labor, capital) but very different results!"), Some("What do you think is the government's role in the economy?")),
        // Psychology
        (42, "psychology", "Psychology is the scientific study of the mind and behavior.", Some("Psychology is like being a detective of the mind — instead of crime scenes, you investigate thoughts, feelings, and behaviors to understand why people do what they do!"), Some("Can you think of a time your brain tricked you into believing something that wasn't true?")),
        (43, "memory", "Memory is the process of encoding, storing, and retrieving information.", Some("Your memory is like a library — encoding is writing the book, storage is putting it on the shelf, and retrieval is finding it again. The problem? Our librarian is sometimes lazy!"), Some("Why do you think you can remember your 5th birthday party but not what you had for lunch last Tuesday?")),
        (44, "emotions", "Emotions are complex psychological and physical responses that influence how we think and behave.", Some("Emotions are like the weather inside your mind — sometimes sunny, sometimes stormy, but always changing. And just like weather, you can't always control them, but you can prepare for them!"), Some("Do you think animals experience emotions the same way humans do?")),
        (45, "social psychology", "Social psychology studies how people's thoughts, feelings, and behaviors are influenced by others.", Some("Social psychology reveals that we're all actors on a stage — changing our performance depending on the audience. Sometimes we don't even realize we're doing it!"), Some("Have you ever changed your opinion just because everyone around you disagreed?")),
        // Environmental Science
        (46, "ecosystems", "An ecosystem is a community of living and non-living things that work together.", Some("An ecosystem is like a web where everything is connected — pull one thread and the whole thing shifts. Remove bees and flowers can't reproduce; remove wolves and deer overpopulate!"), Some("Can you name three living and three non-living things in an ecosystem near you?")),
        (47, "climate change", "Climate change is the long-term shift in global temperatures and weather patterns, largely driven by human activities.", Some("Imagine Earth wearing a blanket (the atmosphere). Greenhouse gases are making that blanket thicker — great for staying warm, but we're now overheating under it!"), Some("What's one thing you could do this week to reduce your carbon footprint?")),
        (48, "pollution", "Pollution is the introduction of harmful substances into the environment.", Some("Pollution is like putting the wrong fuel in an engine — the machine still runs for a while, but eventually things start breaking down. Earth is that engine!"), Some("Where does your garbage go after it leaves your house?")),
        (49, "sustainability", "Sustainability means using resources in a way that meets current needs without preventing future generations from meeting theirs.", Some("Sustainability is like eating from a fruit tree — if you pick all the fruit AND cut down the tree, there's nothing for next year. Smart harvesting keeps the tree alive forever!"), Some("If you could redesign one everyday product to be more sustainable, what would it be?")),
        // Astronomy
        (54, "solar system", "The solar system is the Sun and everything that orbits it — planets, moons, asteroids, and comets.", Some("Think of the solar system like a giant spinning record player — the Sun is the center, and the planets are grooves spiraling outward, each at their own speed!"), Some("Can you name the eight planets in order from the Sun?")),
        (55, "stars", "Stars are massive balls of hot gas that produce energy through nuclear fusion — converting hydrogen into helium.", Some("A star is like a giant nuclear campfire — except instead of burning wood, it crushes hydrogen atoms together so hard they become helium and release incredible energy!"), Some("Why do you think some stars are blue and others are red?")),
        (56, "galaxies", "A galaxy is a vast collection of stars, gas, dust, and dark matter bound together by gravity.", Some("Imagine a galaxy as a cosmic city — each star is a house, and there are hundreds of billions of houses, all orbiting a central downtown (usually a supermassive black hole)!"), Some("If the Milky Way is 100,000 light-years across, what does that tell you about how long light takes to cross it?")),
        (57, "space exploration", "Space exploration is humanity's effort to understand and travel beyond Earth — using telescopes, satellites, rovers, and crewed missions.", Some("Space exploration is like being an ant on a beach ball trying to understand the ocean — we've barely stepped off our tiny planet, but what we've already found is mind-blowing!"), Some("If you could send one message to an alien civilization, what would you say?")),
        // Creative Writing
        (50, "story structure", "Story structure is the framework that organizes a narrative — beginning, middle, and end.", Some("Think of story structure like a roller coaster: the slow climb builds tension (setup), the big drop is the climax, and the gentle return to the station is the resolution. No one rides a flat roller coaster!"), Some("What's the inciting incident in your favorite movie?")),
        (51, "characters", "Characters are the people (or creatures) whose experiences drive a story forward.", Some("A character without a flaw is like a diamond without facets — technically perfect but not very interesting to look at. Flaws catch the light!"), Some("Think of a character you love — what's their biggest flaw, and why do you love them anyway?")),
        (52, "dialogue", "Dialogue is the spoken words of characters in a story — it reveals personality, advances plot, and creates tension.", Some("Good dialogue is like an iceberg — what characters say is the tip above water, but what they really mean is the massive shape hiding underneath."), Some("Try rewriting 'I'm angry at you' without using the word 'angry' — how would a character SHOW it?")),
        (53, "poetry", "Poetry is the art of compressing language to its most powerful, musical, and evocative form.", Some("Poetry is like a photograph of language — it freezes a single moment or feeling and lets you study every detail. Prose is the movie; poetry is the still frame that takes your breath away."), Some("Can you write a haiku (5-7-5 syllables) about something you see right now?")),
        // Physics
        (58, "electromagnetism", "Electromagnetism is the unified force combining electricity and magnetism — a changing electric field creates a magnetic field, and vice versa.", Some("Think of electricity and magnetism as dance partners — when one moves, the other follows. Together they create light, radio waves, and everything in between!"), Some("If you spin a magnet near a coil of wire, what happens?")),
        (59, "entropy", "Entropy is a measure of disorder in a system — the Second Law of Thermodynamics says it always increases in isolated systems.", Some("Think of your bedroom — it naturally gets messy (high entropy) and takes effort to clean up (decrease entropy). The universe is the same, just on a cosmic scale!"), Some("Can you think of a process that seems to decrease entropy? What energy input makes it possible?")),
        (60, "doppler effect", "The Doppler effect is the change in frequency of a wave when the source or observer is moving.", Some("An ambulance siren sounds higher as it approaches and lower as it drives away — the sound waves are getting squished or stretched!"), Some("Could the Doppler effect work with light too? (Hint: redshift!)")),
        (61, "nuclear fusion", "Nuclear fusion combines light atomic nuclei into heavier ones, releasing enormous energy — it's how stars shine.", Some("Fusion is like smashing two Lego pieces together so hard they become one new piece AND release a burst of energy. The Sun does this with hydrogen atoms 600 million tons per second!"), Some("Why do you think we haven't built a working fusion power plant yet?")),
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
        (1, "True or false: 7 × 0 = 7", "true_false", "false", Some("true"), Some("false"), None, None, Some("Any number times zero is..."), "False. Any number multiplied by zero equals zero."),
        (1, "In the expression 3 + 4 × 2, the correct answer is ___.", "fill_in_blank", "11", None, None, None, None, Some("Multiplication comes before addition"), "Following order of operations: 4 × 2 = 8, then 3 + 8 = 11."),
        (1, "What is (-3) × (-4)?", "multiple_choice", "12", Some("-12"), Some("-7"), Some("12"), Some("7"), Some("Negative times negative is..."), "(-3) × (-4) = 12. A negative times a negative gives a positive."),
        // Fractions
        (2, "What is 1/2 + 1/4?", "multiple_choice", "3/4", Some("2/4"), Some("2/6"), Some("3/4"), Some("1/4"), Some("Convert 1/2 to 2/4 first"), "1/2 = 2/4, so 2/4 + 1/4 = 3/4."),
        (2, "Simplify 4/8.", "multiple_choice", "1/2", Some("1/2"), Some("2/4"), Some("1/4"), Some("4/8"), Some("Find the GCD of 4 and 8"), "4/8 = 1/2. Divide both by 4."),
        (2, "What is 2/3 × 3/4?", "multiple_choice", "1/2", Some("1/2"), Some("6/7"), Some("5/12"), Some("2/4"), Some("Multiply across, then simplify"), "2/3 × 3/4 = 6/12 = 1/2."),
        // Percentages
        (3, "What is 25% of 200?", "multiple_choice", "50", Some("25"), Some("40"), Some("50"), Some("75"), Some("25% = 1/4"), "25% of 200 = 200 × 0.25 = 50."),
        (3, "Convert 3/5 to a percentage.", "multiple_choice", "60%", Some("30%"), Some("50%"), Some("60%"), Some("75%"), Some("Divide 3 by 5, multiply by 100"), "3 ÷ 5 = 0.6 = 60%."),
        (3, "If a shirt costs $80 and is 30% off, the discount is $___.", "fill_in_blank", "24", None, None, None, None, Some("80 × 0.30 = ?"), "30% of $80 = $80 × 0.30 = $24."),
        // Algebra
        (4, "Solve: x + 7 = 15", "multiple_choice", "8", Some("7"), Some("8"), Some("9"), Some("22"), Some("Subtract 7 from both sides"), "x = 15 - 7 = 8."),
        (4, "Solve: 3x = 21", "multiple_choice", "7", Some("3"), Some("7"), Some("18"), Some("63"), Some("Divide both sides by 3"), "x = 21 ÷ 3 = 7."),
        (4, "Solve: 2x + 5 = 13. x = ___", "fill_in_blank", "4", None, None, None, None, Some("Subtract 5, then divide by 2"), "2x = 13 - 5 = 8, so x = 8/2 = 4."),
        // Geometry
        (5, "How many degrees in a triangle?", "multiple_choice", "180", Some("90"), Some("180"), Some("270"), Some("360"), Some("It's less than a full circle"), "The angles of any triangle always add up to 180°."),
        (5, "What is the area of a rectangle 5 × 3?", "multiple_choice", "15", Some("8"), Some("15"), Some("16"), Some("30"), Some("Area = length × width"), "Area = 5 × 3 = 15 square units."),
        (5, "True or false: A triangle can have two right angles.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Angles in a triangle sum to 180°"), "False. Two right angles = 180°, leaving 0° for the third angle, which is impossible."),
        (5, "The volume of a cube with side 3 is ___.", "fill_in_blank", "27", None, None, None, None, Some("Volume = s³"), "Volume = 3³ = 3 × 3 × 3 = 27 cubic units."),
        // Photosynthesis
        (6, "What gas do plants absorb during photosynthesis?", "multiple_choice", "Carbon dioxide", Some("Oxygen"), Some("Nitrogen"), Some("Carbon dioxide"), Some("Hydrogen"), Some("It's what we breathe out"), "Plants absorb CO₂ (carbon dioxide) and release O₂ (oxygen)."),
        (6, "Where does photosynthesis happen in a cell?", "multiple_choice", "Chloroplast", Some("Nucleus"), Some("Mitochondria"), Some("Chloroplast"), Some("Cell wall"), Some("It contains chlorophyll"), "Photosynthesis occurs in chloroplasts, which contain chlorophyll."),
        // Cell Division
        (7, "How many cells result from mitosis?", "multiple_choice", "2", Some("1"), Some("2"), Some("4"), Some("8"), Some("One becomes..."), "Mitosis produces 2 identical daughter cells."),
        (7, "How many cells result from meiosis?", "multiple_choice", "4", Some("1"), Some("2"), Some("4"), Some("8"), Some("It has two rounds of division"), "Meiosis produces 4 cells, each with half the chromosomes."),
        // Gravity
        (8, "What is Earth's gravitational acceleration?", "multiple_choice", "9.8 m/s²", Some("5.5 m/s²"), Some("9.8 m/s²"), Some("10.5 m/s²"), Some("15 m/s²"), Some("It's close to 10"), "Earth's gravitational acceleration is approximately 9.8 m/s²."),
        (8, "True or false: Objects fall at the same rate in a vacuum regardless of mass.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Think about Galileo's experiment"), "True. In a vacuum (no air resistance), all objects accelerate at the same rate due to gravity."),
        (8, "Newton's Second Law: F = m × ___", "fill_in_blank", "a", None, None, None, None, Some("Force = mass × ?"), "F = ma. Force equals mass times acceleration."),
        // States of Matter
        (9, "What is it called when a liquid becomes a gas?", "multiple_choice", "Evaporation", Some("Melting"), Some("Condensation"), Some("Evaporation"), Some("Freezing"), Some("Think of boiling water"), "Evaporation (or boiling/vaporization) turns liquid into gas."),
        (9, "What state of matter has a fixed shape?", "multiple_choice", "Solid", Some("Solid"), Some("Liquid"), Some("Gas"), Some("Plasma"), Some("Think of ice"), "Solids have a fixed shape because their particles are tightly packed."),
        // Grammar
        (10, "Which is a noun in: 'The dog runs fast'?", "multiple_choice", "dog", Some("the"), Some("dog"), Some("runs"), Some("fast"), Some("A noun is a person, place, or thing"), "'Dog' is a noun — it names a thing."),
        (10, "What type of word is 'quickly'?", "multiple_choice", "Adverb", Some("Noun"), Some("Adjective"), Some("Adverb"), Some("Verb"), Some("It describes how something is done"), "'Quickly' is an adverb — it describes how an action is performed."),
        (10, "Which sentence is correct?", "multiple_choice", "She and I went to the store.", Some("Her and me went to the store."), Some("She and I went to the store."), Some("Her and I went to the store."), Some("She and me went to the store."), Some("Try each pronoun alone: 'I went' or 'me went'?"), "Use subject pronouns (she, I) for subjects. 'She went' and 'I went' are both correct."),
        // Reading Comprehension
        (11, "What is the main idea of a passage?", "multiple_choice", "The central point the author is making", Some("The first sentence"), Some("The central point the author is making"), Some("The longest paragraph"), Some("Any interesting detail"), Some("It's what the whole text is about"), "The main idea is the central point or message the author wants to convey."),
        // History
        (12, "Which civilization developed cuneiform writing?", "multiple_choice", "Mesopotamia", Some("Egypt"), Some("Mesopotamia"), Some("China"), Some("Indus Valley"), Some("Between two famous rivers"), "Mesopotamia (modern Iraq) developed cuneiform, one of the earliest writing systems."),
        (13, "What event triggered World War I?", "multiple_choice", "Assassination of Archduke Franz Ferdinand", Some("Sinking of the Lusitania"), Some("Assassination of Archduke Franz Ferdinand"), Some("Invasion of Poland"), Some("Treaty of Versailles"), Some("It happened in Sarajevo in 1914"), "The assassination of Archduke Franz Ferdinand of Austria-Hungary in Sarajevo (1914) triggered WWI."),
        (14, "Where did the Industrial Revolution begin?", "multiple_choice", "Britain", Some("France"), Some("Germany"), Some("Britain"), Some("United States"), Some("An island nation"), "The Industrial Revolution began in Britain in the late 18th century."),
        // Health
        (15, "How long should you wash your hands?", "multiple_choice", "20 seconds", Some("5 seconds"), Some("10 seconds"), Some("20 seconds"), Some("1 minute"), Some("Sing 'Happy Birthday' twice"), "Wash hands for at least 20 seconds with soap and water."),
        (16, "Which food group provides calcium?", "multiple_choice", "Dairy", Some("Grains"), Some("Dairy"), Some("Fruits"), Some("Fats"), Some("Think of milk and cheese"), "Dairy products like milk, cheese, and yogurt are rich in calcium."),
        (17, "How should you cool a minor burn?", "multiple_choice", "Running cool water for 10+ minutes", Some("Apply ice directly"), Some("Running cool water for 10+ minutes"), Some("Apply butter"), Some("Blow on it"), Some("Gentle cooling, not freezing"), "Cool burns under running cool (not cold) water for at least 10 minutes. Never use ice or butter."),
        // Computer Science
        (18, "What is the binary representation of the number 10?", "multiple_choice", "1010", Some("1001"), Some("1010"), Some("1100"), Some("1110"), Some("10 = 8 + 2"), "10 in binary = 1010 (8 + 0 + 2 + 0)."),
        (18, "How many values can 1 byte store?", "multiple_choice", "256", Some("8"), Some("128"), Some("256"), Some("512"), Some("2 to the power of 8"), "1 byte = 8 bits. 2^8 = 256 possible values (0-255)."),
        (19, "Which sorting algorithm divides the list in half repeatedly?", "multiple_choice", "Merge sort", Some("Bubble sort"), Some("Merge sort"), Some("Selection sort"), Some("Insertion sort"), Some("Divide and conquer"), "Merge sort divides the list in half, sorts each half, then merges them."),
        (19, "True or false: Binary search requires a sorted list.", "true_false", "true", Some("true"), Some("false"), None, None, Some("How does it know which half to pick?"), "True. Binary search only works on sorted data — it needs order to eliminate half the options."),
        (20, "What does DRY stand for in programming?", "multiple_choice", "Don't Repeat Yourself", Some("Do Run Yesterday"), Some("Don't Repeat Yourself"), Some("Data Reads Yield"), Some("Debug Runtime Yearly"), Some("It's about avoiding duplication"), "DRY = Don't Repeat Yourself. Avoid writing the same code in multiple places."),
        (20, "True or false: A syntax error means the code logic is wrong.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Syntax is about grammar, not logic"), "False. A syntax error is a grammar mistake (like a missing bracket). A logic error means the code runs but produces wrong results."),
        (21, "What does CPU stand for?", "multiple_choice", "Central Processing Unit", Some("Central Processing Unit"), Some("Computer Personal Utility"), Some("Core Program Unit"), Some("Central Power Unit"), Some("It's the 'brain' of the computer"), "CPU = Central Processing Unit. It executes instructions."),
        // Geography
        (22, "What is the largest continent by area?", "multiple_choice", "Asia", Some("Africa"), Some("Asia"), Some("North America"), Some("Europe"), Some("It has the most people too"), "Asia is the largest continent, covering about 44.6 million km²."),
        (22, "Which is the deepest ocean?", "multiple_choice", "Pacific", Some("Atlantic"), Some("Indian"), Some("Pacific"), Some("Arctic"), Some("It's also the largest"), "The Pacific Ocean is both the largest and deepest, containing the Mariana Trench."),
        (23, "True or false: Climate and weather are the same thing.", "true_false", "false", Some("true"), Some("false"), None, None, Some("One is short-term, one is long-term"), "False. Weather is short-term conditions; climate is the average pattern over 30+ years."),
        (24, "What do latitude lines measure?", "multiple_choice", "Distance north or south of the equator", Some("Distance east or west of Greenwich"), Some("Distance north or south of the equator"), Some("Elevation above sea level"), Some("Distance between cities"), Some("Think horizontal lines"), "Latitude lines run horizontally, measuring distance north or south of the equator (0°)."),
        (25, "Which of these is a renewable resource?", "multiple_choice", "Solar energy", Some("Coal"), Some("Natural gas"), Some("Solar energy"), Some("Oil"), Some("It won't run out"), "Solar energy is renewable — the sun continuously provides energy."),
        // Music questions
        (26, "How many note names are there before they repeat?", "multiple_choice", "7", Some("5"), Some("7"), Some("8"), Some("12"), Some("A through G"), "There are 7 note names: A, B, C, D, E, F, G, then they repeat an octave higher."),
        (26, "What does a sharp (#) do to a note?", "multiple_choice", "Raises it by a half step", Some("Lowers it by a half step"), Some("Raises it by a half step"), Some("Raises it by a whole step"), Some("Doubles its duration"), Some("Sharp = higher"), "A sharp raises a note by one half step (semitone)."),
        (26, "The C major scale uses ___ sharps and flats.", "fill_in_blank", "0", None, None, None, None, Some("All white keys on a piano"), "C major has no sharps or flats — it uses only the white keys: C D E F G A B C."),
        (27, "How many beats does a whole note get in 4/4 time?", "multiple_choice", "4", Some("1"), Some("2"), Some("4"), Some("8"), Some("It fills the whole measure"), "A whole note gets 4 beats — it fills an entire measure of 4/4 time."),
        (27, "In a 3/4 time signature, there are ___ beats per measure.", "fill_in_blank", "3", None, None, None, None, Some("The top number tells you"), "The top number of a time signature tells the beats per measure. 3/4 = 3 beats."),
        (27, "Which time signature is used for waltzes?", "multiple_choice", "3/4", Some("2/4"), Some("3/4"), Some("4/4"), Some("6/8"), Some("ONE-two-three, ONE-two-three"), "Waltzes use 3/4 time: three quarter-note beats per measure."),
        (28, "Which instrument family uses a bow?", "multiple_choice", "Strings", Some("Strings"), Some("Woodwinds"), Some("Brass"), Some("Percussion"), Some("Violin, viola, cello..."), "String instruments like violin, viola, and cello are typically played with a bow."),
        (28, "True or false: A saxophone is a brass instrument.", "true_false", "false", Some("true"), Some("false"), None, None, Some("It looks like brass but uses a reed"), "False. Despite being made of brass, the saxophone is a woodwind because it uses a reed to produce sound."),
        (29, "Which era came first: Baroque or Romantic?", "multiple_choice", "Baroque", Some("Baroque"), Some("Classical"), Some("Romantic"), Some("Modern"), Some("Bach was early"), "Baroque (1600-1750) came before Romantic (1820-1900). The Classical period was in between."),
        (29, "Which genre originated in New Orleans?", "multiple_choice", "Jazz", Some("Blues"), Some("Jazz"), Some("Rock"), Some("Hip Hop"), Some("Improvisation is key"), "Jazz originated in New Orleans in the early 1900s, blending African American musical traditions."),
        // Art questions
        (30, "What are the three primary colors?", "multiple_choice", "Red, Blue, Yellow", Some("Red, Blue, Yellow"), Some("Red, Green, Blue"), Some("Cyan, Magenta, Yellow"), Some("Red, Orange, Yellow"), Some("In traditional color theory"), "In traditional (subtractive) color theory, the primaries are Red, Blue, and Yellow."),
        (30, "What do you get when you mix blue and yellow?", "multiple_choice", "Green", Some("Purple"), Some("Green"), Some("Orange"), Some("Brown"), Some("Think of a forest"), "Blue + Yellow = Green. Green is a secondary color."),
        (30, "Colors opposite each other on the color wheel are called ___.", "fill_in_blank", "complementary", None, None, None, None, Some("They 'complete' each other"), "Complementary colors are opposite on the wheel (e.g. red/green). They create maximum contrast."),
        (31, "How many elements of art are there?", "multiple_choice", "7", Some("5"), Some("6"), Some("7"), Some("8"), Some("Line, shape, form, color, value, texture, space"), "There are 7 elements of art: line, shape, form, color, value, texture, and space."),
        (31, "True or false: A circle is a form.", "true_false", "false", Some("true"), Some("false"), None, None, Some("2D = shape, 3D = form"), "False. A circle is a shape (2D). A sphere is a form (3D)."),
        (32, "Who painted the Mona Lisa?", "multiple_choice", "Leonardo da Vinci", Some("Michelangelo"), Some("Leonardo da Vinci"), Some("Raphael"), Some("Botticelli"), Some("A Renaissance polymath"), "Leonardo da Vinci painted the Mona Lisa, completed around 1517."),
        (32, "Which art movement featured melting clocks?", "multiple_choice", "Surrealism", Some("Impressionism"), Some("Cubism"), Some("Surrealism"), Some("Pop Art"), Some("Salvador Dalí"), "Salvador Dalí's 'The Persistence of Memory' (1931) is a famous Surrealist work with melting clocks."),
        (33, "The Rule of Thirds divides the frame into a ___ grid.", "fill_in_blank", "3x3", None, None, None, None, Some("Three by three"), "The Rule of Thirds divides the frame into a 3×3 grid (9 equal sections)."),
        (33, "True or false: Negative space is wasted space.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Empty space has a purpose"), "False. Negative space is intentional — it gives subjects room to breathe and improves readability."),
        // Philosophy — Logic
        (34, "What makes an argument 'valid'?", "multiple_choice", "If premises are true, conclusion must be true", Some("It sounds convincing"), Some("If premises are true, conclusion must be true"), Some("The conclusion is actually true"), Some("Most people agree with it"), Some("Validity is about structure, not truth"), "A valid argument means IF the premises are true, the conclusion MUST follow. The premises don't have to actually be true."),
        (34, "Which fallacy attacks the person instead of the argument?", "multiple_choice", "Ad Hominem", Some("Straw Man"), Some("Ad Hominem"), Some("False Dilemma"), Some("Slippery Slope"), Some("It's Latin for 'to the person'"), "Ad Hominem attacks the person making the argument rather than the argument itself."),
        (34, "True or false: A sound argument can have a false conclusion.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Sound = valid + true premises"), "False. A sound argument is valid with true premises, so its conclusion must be true."),
        // Philosophy — Ethics
        (35, "Which ethical framework judges actions by their outcomes?", "multiple_choice", "Consequentialism", Some("Deontology"), Some("Virtue Ethics"), Some("Consequentialism"), Some("Nihilism"), Some("Consequences = outcomes"), "Consequentialism (including utilitarianism) judges the morality of actions based on their results."),
        (35, "Who said 'I know that I know nothing'?", "multiple_choice", "Socrates", Some("Plato"), Some("Socrates"), Some("Aristotle"), Some("Descartes"), Some("He was famous for asking questions"), "Socrates used this phrase to express intellectual humility and the importance of questioning assumptions."),
        (35, "Kant's Categorical Imperative says you should act only by rules you'd want to be ___.", "fill_in_blank", "universal", None, None, None, None, Some("Rules for everyone"), "Kant's test: could your rule work if everyone followed it? If not, it's wrong."),
        // Philosophy — Famous Philosophers
        (36, "Which philosopher said 'I think, therefore I am'?", "multiple_choice", "Descartes", Some("Socrates"), Some("Kant"), Some("Descartes"), Some("Nietzsche"), Some("He doubted everything"), "René Descartes used radical doubt to find one thing he couldn't doubt: that he was thinking."),
        (36, "Who founded the Academy in Athens?", "multiple_choice", "Plato", Some("Socrates"), Some("Plato"), Some("Aristotle"), Some("Pythagoras"), Some("Student of Socrates, teacher of Aristotle"), "Plato founded the Academy around 387 BCE — often considered the first institution of higher learning in the Western world."),
        // Philosophy — Thought Experiments
        (37, "What does Plato's Cave allegory illustrate?", "multiple_choice", "Our perceptions may be limited", Some("Caves are dangerous"), Some("Our perceptions may be limited"), Some("Shadows are real"), Some("Knowledge comes from experience alone"), Some("The prisoners only saw shadows"), "Plato's Cave shows that what we perceive might be only shadows of a deeper reality."),
        (37, "The Ship of Theseus asks about ___.", "fill_in_blank", "identity", None, None, None, None, Some("Is it still the same ship?"), "The Ship of Theseus explores questions of identity: if every part is replaced, is it still the same thing?"),
        (37, "True or false: The Chinese Room argument suggests computers can truly understand language.", "true_false", "false", Some("true"), Some("false"), None, None, Some("The person in the room doesn't understand Chinese"), "False. Searle's Chinese Room argues that following rules to manipulate symbols is not the same as understanding."),
        // Economics — Supply & Demand
        (38, "When price rises, what happens to demand (normally)?", "multiple_choice", "Demand falls", Some("Demand rises"), Some("Demand stays the same"), Some("Demand falls"), Some("Demand disappears"), Some("Think about expensive concert tickets"), "The Law of Demand: as price increases, quantity demanded decreases (all else equal)."),
        (38, "What is it called when supply exceeds demand?", "multiple_choice", "Surplus", Some("Shortage"), Some("Surplus"), Some("Equilibrium"), Some("Inflation"), Some("Too much supply, not enough buyers"), "A surplus occurs when quantity supplied exceeds quantity demanded — usually because price is too high."),
        (38, "True or false: Demand for medicine is typically elastic.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Do people stop buying medicine when prices rise?"), "False. Medicine is inelastic — people need it regardless of price changes."),
        // Economics — Money & Banking
        (39, "Which is NOT a function of money?", "multiple_choice", "Source of energy", Some("Medium of exchange"), Some("Store of value"), Some("Source of energy"), Some("Unit of account"), Some("Money has three main functions"), "Money serves as a medium of exchange, store of value, and unit of account — not as a source of energy."),
        (39, "What is inflation?", "multiple_choice", "When money buys less over time", Some("When prices fall"), Some("When money buys less over time"), Some("When banks close"), Some("When wages rise faster than prices"), Some("Your dollar buys less each year"), "Inflation means the general price level rises, so each unit of currency buys fewer goods and services."),
        (39, "Interest earned on interest is called ___ interest.", "fill_in_blank", "compound", None, None, None, None, Some("It builds on itself"), "Compound interest is calculated on both the initial principal and the accumulated interest from previous periods."),
        // Economics — Trade
        (40, "What is a tariff?", "multiple_choice", "A tax on imports", Some("A trade agreement"), Some("A tax on imports"), Some("A type of currency"), Some("A shipping route"), Some("It makes foreign goods more expensive"), "A tariff is a tax imposed on imported goods, making them more expensive to protect domestic industries."),
        (40, "True or false: A trade deficit means a country exports more than it imports.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Deficit means lacking"), "False. A trade deficit means imports exceed exports. A trade surplus means exports exceed imports."),
        // Economics — Economic Systems
        (41, "Which economic system relies on private ownership and market prices?", "multiple_choice", "Market economy", Some("Command economy"), Some("Market economy"), Some("Traditional economy"), Some("Mixed economy"), Some("Also called capitalism"), "A market economy (capitalism) relies on private ownership and supply/demand to set prices."),
        (41, "GDP stands for Gross ___ Product.", "fill_in_blank", "Domestic", None, None, None, None, Some("It measures a country's output"), "GDP = Gross Domestic Product — the total value of goods and services produced within a country."),
        (41, "A recession is defined as ___ consecutive quarters of declining GDP.", "fill_in_blank", "2", None, None, None, None, Some("A common rule of thumb"), "A recession is commonly defined as two consecutive quarters of negative GDP growth."),
        // Psychology — Introduction
        (42, "Who opened the first psychology laboratory?", "multiple_choice", "Wilhelm Wundt", Some("Sigmund Freud"), Some("Wilhelm Wundt"), Some("B.F. Skinner"), Some("Carl Rogers"), Some("It was in Leipzig, Germany, in 1879"), "Wilhelm Wundt opened the first psychology lab in Leipzig, Germany, in 1879."),
        (42, "True or false: The nature vs. nurture debate concludes that genes alone determine behavior.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Modern psychology says it's both"), "False. Modern psychology recognizes both nature (genetics) and nurture (environment) interact."),
        (42, "The study of how the brain and nervous system affect behavior is called ___ psychology.", "fill_in_blank", "biological", None, None, None, None, Some("Related to biology"), "Biological psychology examines how physical processes in the brain influence behavior and mental states."),
        // Psychology — Memory & Learning
        (43, "How many items can short-term memory typically hold?", "multiple_choice", "7 plus or minus 2", Some("3 plus or minus 1"), Some("7 plus or minus 2"), Some("12 plus or minus 3"), Some("Unlimited"), Some("Miller's magic number"), "George Miller found short-term memory holds about 7±2 items."),
        (43, "Which scientist is famous for classical conditioning with dogs?", "multiple_choice", "Pavlov", Some("Skinner"), Some("Pavlov"), Some("Bandura"), Some("Piaget"), Some("Dogs, bells, and salivation"), "Ivan Pavlov demonstrated classical conditioning — dogs learned to salivate at the sound of a bell."),
        (43, "True or false: We forget approximately 50% of new information within one hour.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Ebbinghaus discovered this"), "True. Ebbinghaus's forgetting curve shows rapid forgetting without review — about 50% within an hour."),
        (43, "Operant conditioning was developed by ___.", "fill_in_blank", "Skinner", None, None, None, None, Some("Associated with reinforcement and punishment"), "B.F. Skinner developed operant conditioning — behavior shaped by consequences (reinforcement and punishment)."),
        // Psychology — Emotions & Motivation
        (44, "How many basic universal emotions did Paul Ekman identify?", "multiple_choice", "6", Some("4"), Some("6"), Some("8"), Some("10"), Some("Happiness, sadness, fear, anger..."), "Ekman identified 6 basic emotions: happiness, sadness, fear, anger, surprise, and disgust."),
        (44, "What is at the top of Maslow's hierarchy of needs?", "multiple_choice", "Self-actualization", Some("Safety"), Some("Belonging"), Some("Esteem"), Some("Self-actualization"), Some("Reaching your full potential"), "Self-actualization — reaching your full potential — sits at the top of Maslow's hierarchy."),
        (44, "True or false: Intrinsic motivation comes from external rewards.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Intrinsic means from within"), "False. Intrinsic motivation comes from within (enjoyment, curiosity). Extrinsic comes from external rewards."),
        // Psychology — Social Psychology
        (45, "In Milgram's obedience study, what percentage gave maximum shocks?", "multiple_choice", "65%", Some("10%"), Some("35%"), Some("65%"), Some("90%"), Some("A surprisingly high number"), "65% of participants administered what they believed were maximum shocks when instructed by an authority figure."),
        (45, "Which bias makes us seek information that confirms what we already believe?", "multiple_choice", "Confirmation bias", Some("Anchoring bias"), Some("Confirmation bias"), Some("Halo effect"), Some("Sunk cost fallacy"), Some("We look for confirming evidence"), "Confirmation bias leads us to seek, interpret, and remember information that confirms our existing beliefs."),
        (45, "The tendency for individuals to be less likely to help when others are present is called the ___ effect.", "fill_in_blank", "bystander", None, None, None, None, Some("Bystanders watch but don't act"), "The bystander effect: the more people present, the less likely any individual is to help."),
        // Environmental Science — Ecosystems
        (46, "Approximately what percentage of energy transfers between trophic levels?", "multiple_choice", "10%", Some("1%"), Some("10%"), Some("50%"), Some("90%"), Some("It's called the 10% rule"), "Only about 10% of energy transfers from one trophic level to the next. The rest is lost as heat."),
        (46, "Which biome has the greatest biodiversity?", "multiple_choice", "Tropical rainforest", Some("Desert"), Some("Tropical rainforest"), Some("Tundra"), Some("Grassland"), Some("Hot and wet year-round"), "Tropical rainforests contain over 50% of Earth's species despite covering only ~6% of land."),
        (46, "True or false: Decomposers are an essential part of every ecosystem.", "true_false", "true", Some("true"), Some("false"), None, None, Some("What would happen to dead organisms without them?"), "True. Decomposers break down dead matter and recycle nutrients back into the ecosystem."),
        // Environmental Science — Climate Change
        (47, "By how much has Earth's temperature risen since pre-industrial times?", "multiple_choice", "About 1.1°C", Some("About 0.3°C"), Some("About 1.1°C"), Some("About 2.5°C"), Some("About 5°C"), Some("We're trying to stay below 1.5°C"), "Earth has warmed approximately 1.1°C since pre-industrial times (late 1800s)."),
        (47, "Which greenhouse gas is most abundant from human activities?", "multiple_choice", "Carbon dioxide (CO₂)", Some("Methane"), Some("Carbon dioxide (CO₂)"), Some("Nitrous oxide"), Some("Water vapor"), Some("We produce it by burning fossil fuels"), "CO₂ is the most abundant human-produced greenhouse gas, mainly from burning fossil fuels."),
        (47, "The Paris Agreement aims to limit warming to ___ °C above pre-industrial levels.", "fill_in_blank", "1.5", None, None, None, None, Some("Less than two degrees"), "The Paris Agreement's ambitious target is 1.5°C, with a fallback limit of 2°C."),
        // Environmental Science — Pollution
        (48, "How much plastic enters the oceans annually?", "multiple_choice", "About 8 million tonnes", Some("About 1 million tonnes"), Some("About 8 million tonnes"), Some("About 50 million tonnes"), Some("About 100 million tonnes"), Some("Millions, not billions"), "Approximately 8 million tonnes of plastic enter the world's oceans every year."),
        (48, "True or false: Only 9% of all plastic ever produced has been recycled.", "true_false", "true", Some("true"), Some("false"), None, None, Some("The recycling rate is shockingly low"), "True. Of all plastic ever produced, only about 9% has been recycled. 12% incinerated, 79% in landfills or environment."),
        (48, "Tiny plastic fragments smaller than 5mm are called ___.", "fill_in_blank", "microplastics", None, None, None, None, Some("Micro means very small"), "Microplastics are plastic particles smaller than 5mm, found in water, soil, air, and even human blood."),
        // Environmental Science — Conservation
        (49, "What does the acronym HIPPO stand for in conservation?", "multiple_choice", "Habitat loss, Invasive species, Pollution, Population, Overexploitation", Some("Heat, Ice, Pollution, People, Oceans"), Some("Habitat loss, Invasive species, Pollution, Population, Overexploitation"), Some("Hunting, Industry, Plastic, Poverty, Oil"), Some("Hurricanes, Ice, Pollution, Population, Ozone"), Some("The main threats to biodiversity"), "HIPPO: Habitat loss, Invasive species, Pollution, Population growth, Overexploitation — the five main threats to biodiversity."),
        (49, "How many UN Sustainable Development Goals are there?", "multiple_choice", "17", Some("10"), Some("15"), Some("17"), Some("20"), Some("Targets for 2030"), "There are 17 UN Sustainable Development Goals (SDGs) set for achievement by 2030."),
        (49, "True or false: The number one threat to biodiversity is climate change.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Think about what directly destroys species' homes"), "False. Habitat loss is the #1 threat to biodiversity. Climate change is a growing but secondary factor."),
        // Creative Writing — Story Structure
        (50, "In the three-act structure, the inciting incident occurs in which act?", "multiple_choice", "Act 1", Some("Act 1"), Some("Act 2"), Some("Act 3"), Some("The climax"), Some("It disrupts the character's normal world"), "The inciting incident occurs in Act 1 (Setup) — it's the event that kicks the story into motion."),
        (50, "Which type of conflict involves a character struggling against their own fears?", "multiple_choice", "Person vs. Self", Some("Person vs. Person"), Some("Person vs. Self"), Some("Person vs. Nature"), Some("Person vs. Society"), Some("It's internal, not external"), "Person vs. Self is an internal conflict — the character battles their own doubts, fears, or desires."),
        (50, "True or false: The climax occurs in Act 2 of the three-act structure.", "true_false", "false", Some("true"), Some("false"), None, None, Some("The climax is the turning point near the end"), "False. The climax occurs in Act 3 (Resolution). Act 2 contains rising action and the midpoint."),
        // Creative Writing — Character Development
        (51, "The 'iceberg technique' in character writing was associated with which author?", "multiple_choice", "Hemingway", Some("Dickens"), Some("Hemingway"), Some("Tolkien"), Some("Austen"), Some("A famously sparse writer"), "Ernest Hemingway advocated knowing far more about your characters than you reveal — the depth shows through."),
        (51, "A character who starts flawed and grows over the story has a ___ arc.", "fill_in_blank", "positive", None, None, None, None, Some("They improve and overcome"), "A positive character arc shows a flawed character who grows, learns, and overcomes their weakness."),
        (51, "Which is an example of 'show, don't tell'?", "multiple_choice", "Her hands trembled as she opened the letter", Some("She was very nervous"), Some("Her hands trembled as she opened the letter"), Some("She felt scared and anxious"), Some("She was worried about the news"), Some("Actions reveal emotions better than labels"), "'Her hands trembled' shows nervousness through physical action rather than telling with an emotion label."),
        // Creative Writing — Dialogue & Voice
        (52, "Which word is recommended for most dialogue tags?", "multiple_choice", "said", Some("exclaimed"), Some("said"), Some("articulated"), Some("declared"), Some("It's invisible to the reader"), "'Said' is nearly invisible to readers, keeping focus on the dialogue itself rather than the tag."),
        (52, "True or false: Good dialogue should sound exactly like real speech, including all the 'ums' and 'ahs'.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Fiction dialogue is polished"), "False. Good dialogue captures the feel of natural speech but is more focused and purposeful than real conversation."),
        (52, "First-person narration uses which pronoun?", "multiple_choice", "I", Some("He/She"), Some("I"), Some("You"), Some("They"), Some("The narrator is telling their own story"), "First-person uses 'I/me' — the narrator is a character in the story telling it from their perspective."),
        // Creative Writing — Poetry
        (53, "A haiku has how many syllables in total?", "multiple_choice", "17", Some("14"), Some("17"), Some("20"), Some("10"), Some("5 + 7 + 5"), "A haiku has 17 syllables: 5 in the first line, 7 in the second, 5 in the third."),
        (53, "Which device gives human qualities to non-human things?", "multiple_choice", "Personification", Some("Simile"), Some("Metaphor"), Some("Personification"), Some("Alliteration"), Some("The wind 'howled' — can wind really howl?"), "Personification attributes human characteristics to non-human things (the wind howled, time marches on)."),
        (53, "'Life is like a box of chocolates' is an example of a ___.", "fill_in_blank", "simile", None, None, None, None, Some("It uses 'like' or 'as'"), "A simile compares two things using 'like' or 'as'. A metaphor would say 'Life IS a box of chocolates.'"),
        // Astronomy — The Solar System (topic_id=54)
        (54, "Which planet is closest to the Sun?", "multiple_choice", "Mercury", Some("Venus"), Some("Mercury"), Some("Mars"), Some("Earth"), Some("The smallest planet"), "Mercury is the closest planet to the Sun, orbiting at an average distance of 58 million km."),
        (54, "Which planet is the hottest in our solar system?", "multiple_choice", "Venus", Some("Mercury"), Some("Venus"), Some("Mars"), Some("Jupiter"), Some("Hint: it's not the closest to the Sun"), "Venus is the hottest (465°C) due to its thick CO₂ atmosphere creating a runaway greenhouse effect."),
        (54, "How many planets are in our solar system?", "multiple_choice", "8", Some("7"), Some("8"), Some("9"), Some("10"), Some("Pluto was reclassified"), "There are 8 planets. Pluto was reclassified as a dwarf planet in 2006."),
        (54, "Which planet has the Great Red Spot?", "multiple_choice", "Jupiter", Some("Mars"), Some("Saturn"), Some("Jupiter"), Some("Neptune"), Some("The largest planet"), "Jupiter's Great Red Spot is a storm larger than Earth that has raged for at least 350 years."),
        (54, "True or false: Saturn is the only planet with rings.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Other gas giants have them too"), "False. Jupiter, Uranus, and Neptune also have ring systems, though Saturn's are by far the most prominent."),
        (54, "Light from the Sun reaches Earth in about ___ minutes.", "fill_in_blank", "8", None, None, None, None, Some("The Sun is about 150 million km away"), "Light travels at ~300,000 km/s. The Sun is ~150 million km away: 150,000,000 ÷ 300,000 ≈ 500 seconds ≈ 8.3 minutes."),
        // Astronomy — Stars & Stellar Evolution (topic_id=55)
        (55, "What process powers stars?", "multiple_choice", "Nuclear fusion", Some("Nuclear fission"), Some("Nuclear fusion"), Some("Chemical combustion"), Some("Gravitational collapse"), Some("Hydrogen atoms combine"), "Stars are powered by nuclear fusion — hydrogen atoms fuse into helium, releasing enormous energy (E=mc²)."),
        (55, "What will our Sun eventually become?", "multiple_choice", "White dwarf", Some("Black hole"), Some("Neutron star"), Some("White dwarf"), Some("Red dwarf"), Some("It's not massive enough for a dramatic death"), "Our Sun will expand into a red giant, shed its outer layers as a planetary nebula, and leave behind a white dwarf."),
        (55, "Which spectral type is the hottest?", "multiple_choice", "O", Some("M"), Some("G"), Some("O"), Some("A"), Some("Oh Be A Fine Girl/Guy Kiss Me"), "O-type stars are the hottest (>30,000 K), appearing blue-white. M-type are the coolest."),
        (55, "True or false: Red dwarfs are the most common type of star.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Small and dim but everywhere"), "True. Red dwarfs (M-type) make up about 70-80% of all stars in the Milky Way."),
        (55, "The nearest star to Earth (besides the Sun) is ___.", "fill_in_blank", "Proxima Centauri", None, None, None, None, Some("It's part of the Alpha Centauri system"), "Proxima Centauri is 4.24 light-years from Earth, part of the Alpha Centauri triple star system."),
        // Astronomy — Galaxies & the Universe (topic_id=56)
        (56, "What type of galaxy is the Milky Way?", "multiple_choice", "Spiral", Some("Elliptical"), Some("Spiral"), Some("Irregular"), Some("Lenticular"), Some("It has arms"), "The Milky Way is a barred spiral galaxy, about 100,000 light-years in diameter."),
        (56, "What sits at the center of the Milky Way?", "multiple_choice", "A supermassive black hole", Some("A giant star"), Some("A supermassive black hole"), Some("A neutron star"), Some("Nothing"), Some("Named Sagittarius A*"), "Sagittarius A* is a supermassive black hole at the Milky Way's center, with ~4 million solar masses."),
        (56, "How old is the universe?", "multiple_choice", "About 13.8 billion years", Some("About 4.6 billion years"), Some("About 13.8 billion years"), Some("About 100 billion years"), Some("About 1 billion years"), Some("4.6 billion is our solar system"), "The universe is approximately 13.8 billion years old, determined from the cosmic microwave background."),
        (56, "True or false: The universe is expanding.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Hubble discovered this in 1929"), "True. Edwin Hubble discovered that galaxies are moving apart, and the expansion is accelerating due to dark energy."),
        (56, "Ordinary matter makes up about ___% of the universe.", "fill_in_blank", "5", None, None, None, None, Some("Dark matter and dark energy dominate"), "Only ~5% of the universe is ordinary matter. ~27% is dark matter and ~68% is dark energy."),
        // Astronomy — Space Exploration (topic_id=57)
        (57, "Who was the first human in space?", "multiple_choice", "Yuri Gagarin", Some("Neil Armstrong"), Some("Yuri Gagarin"), Some("John Glenn"), Some("Buzz Aldrin"), Some("Soviet cosmonaut, 1961"), "Yuri Gagarin orbited Earth on April 12, 1961, aboard Vostok 1."),
        (57, "In what year did humans first walk on the Moon?", "multiple_choice", "1969", Some("1965"), Some("1969"), Some("1972"), Some("1959"), Some("Apollo 11"), "Neil Armstrong and Buzz Aldrin walked on the Moon on July 20, 1969, during the Apollo 11 mission."),
        (57, "Which space telescope launched in 2021?", "multiple_choice", "James Webb Space Telescope", Some("Hubble"), Some("James Webb Space Telescope"), Some("Spitzer"), Some("Kepler"), Some("Named after a NASA administrator"), "The James Webb Space Telescope (JWST) launched on December 25, 2021, as Hubble's successor for infrared astronomy."),
        (57, "True or false: Voyager 1 has left the solar system.", "true_false", "true", Some("true"), Some("false"), None, None, Some("It entered interstellar space in 2012"), "True. Voyager 1 crossed into interstellar space in August 2012, becoming the first human-made object to do so."),
        (57, "The Fermi Paradox asks: if the universe is so big, where is ___?", "fill_in_blank", "everyone", None, None, None, None, Some("Where are all the aliens?"), "The Fermi Paradox highlights the contradiction between the high probability of alien civilizations and the lack of evidence for them."),
        // Physics — Electricity & Magnetism (topic_id=58)
        (58, "What does Ohm's Law state?", "multiple_choice", "V = I × R", Some("V = I + R"), Some("V = I × R"), Some("V = I / R"), Some("V = I² × R"), Some("Voltage, current, resistance"), "Ohm's Law states that voltage equals current times resistance: V = I × R."),
        (58, "What type of current do batteries produce?", "multiple_choice", "Direct Current (DC)", Some("Alternating Current (AC)"), Some("Direct Current (DC)"), Some("Pulsed Current"), Some("Static Current"), Some("Flows in one direction"), "Batteries produce Direct Current (DC), which flows in one constant direction."),
        (58, "True or false: A changing magnetic field can induce an electric current.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Faraday's discovery"), "True. This is Faraday's Law of electromagnetic induction, the principle behind generators."),
        (58, "Electric current is measured in ___.", "fill_in_blank", "amperes", None, None, None, None, Some("Named after André-Marie Ampère"), "Electric current is measured in amperes (A), where 1 ampere = 1 coulomb per second."),
        (58, "Who discovered that electric current creates a magnetic field?", "multiple_choice", "Ørsted", Some("Faraday"), Some("Ørsted"), Some("Maxwell"), Some("Tesla"), Some("Danish physicist, 1820"), "Hans Christian Ørsted discovered in 1820 that electric current creates a magnetic field."),
        // Physics — Thermodynamics (topic_id=59)
        (59, "What is absolute zero?", "multiple_choice", "-273.15°C (0 K)", Some("-100°C"), Some("-273.15°C (0 K)"), Some("-459°F"), Some("0°C"), Some("Lowest possible temperature"), "Absolute zero is 0 Kelvin (-273.15°C), the lowest theoretically possible temperature."),
        (59, "The First Law of Thermodynamics is essentially the law of ___.", "fill_in_blank", "conservation of energy", None, None, None, None, Some("Energy cannot be created or destroyed"), "The First Law states that energy is conserved — it can change form but cannot be created or destroyed."),
        (59, "Which heat transfer mechanism does NOT require a medium?", "multiple_choice", "Radiation", Some("Conduction"), Some("Convection"), Some("Radiation"), Some("All require a medium"), Some("How the sun heats Earth through space"), "Radiation transfers heat via electromagnetic waves and can travel through the vacuum of space."),
        (59, "True or false: The entropy of an isolated system always decreases.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Second Law of Thermodynamics"), "False. The Second Law states that entropy (disorder) of an isolated system always increases or stays the same."),
        (59, "Water has a notably ___ specific heat capacity.", "fill_in_blank", "high", None, None, None, None, Some("This is why oceans moderate climate"), "Water has a very high specific heat capacity (4,186 J/kg·°C), which is why it moderates climate."),
        // Physics — Waves & Sound (topic_id=60)
        (60, "What is the speed of sound in air at 20°C?", "multiple_choice", "343 m/s", Some("300 m/s"), Some("343 m/s"), Some("500 m/s"), Some("1,000 m/s"), Some("About 1,235 km/h"), "Sound travels at approximately 343 meters per second in air at 20°C."),
        (60, "True or false: Sound can travel through a vacuum.", "true_false", "false", Some("true"), Some("false"), None, None, Some("In space, no one can hear you scream"), "False. Sound requires a medium (solid, liquid, or gas) — it cannot travel through a vacuum."),
        (60, "The Doppler effect causes a change in perceived ___.", "fill_in_blank", "pitch", None, None, None, None, Some("Think of an ambulance siren passing by"), "The Doppler effect changes the perceived pitch (frequency) when the source or observer is moving."),
        (60, "What type of wave is sound?", "multiple_choice", "Longitudinal", Some("Transverse"), Some("Longitudinal"), Some("Surface"), Some("Electromagnetic"), Some("Compressions and rarefactions"), "Sound is a longitudinal wave — particles oscillate parallel to the wave's direction of travel."),
        (60, "Human hearing range is approximately ___ Hz to 20,000 Hz.", "fill_in_blank", "20", None, None, None, None, Some("Below this is infrasound"), "Humans can typically hear frequencies from about 20 Hz to 20,000 Hz."),
        // Physics — Nuclear Physics (topic_id=61)
        (61, "What does E = mc² mean?", "multiple_choice", "Energy equals mass times the speed of light squared", Some("Energy equals mass times velocity"), Some("Energy equals mass times the speed of light squared"), Some("Entropy equals mass times constant"), Some("Energy equals momentum times charge"), Some("Einstein's most famous equation"), "E = mc² shows that mass and energy are interchangeable; a small mass yields enormous energy."),
        (61, "What is nuclear fission?", "multiple_choice", "Splitting heavy atoms", Some("Combining light atoms"), Some("Splitting heavy atoms"), Some("Electron capture"), Some("Neutron decay"), Some("Used in nuclear power plants"), "Fission splits heavy nuclei (like uranium) into lighter ones, releasing large amounts of energy."),
        (61, "True or false: Nuclear fusion powers the Sun.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Hydrogen → Helium"), "True. The Sun fuses hydrogen nuclei into helium, releasing enormous energy via E = mc²."),
        (61, "The half-life of Carbon-14 is approximately ___ years.", "fill_in_blank", "5730", None, None, None, None, Some("Used for radiocarbon dating"), "Carbon-14 has a half-life of about 5,730 years, making it useful for dating organic materials up to ~50,000 years old."),
        (61, "Which type of radiation is stopped by a sheet of paper?", "multiple_choice", "Alpha particles", Some("Alpha particles"), Some("Beta particles"), Some("Gamma rays"), Some("Neutron radiation"), Some("Heaviest type of radiation"), "Alpha particles (2 protons + 2 neutrons) are large and can be stopped by paper or even skin."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
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
        ("programming", 1, 18, "Understand binary — how computers represent data"),
        ("programming", 2, 19, "Learn algorithms — step-by-step problem solving"),
        ("programming", 3, 20, "Programming concepts — variables, loops, functions"),
        ("programming", 4, 21, "Computer hardware — understand what runs your code"),
        ("world geography", 1, 22, "Continents & oceans — the big picture"),
        ("world geography", 2, 23, "Weather & climate — understanding our atmosphere"),
        ("world geography", 3, 24, "Maps & navigation — finding your way"),
        ("world geography", 4, 25, "Natural resources — what Earth provides"),
        ("music fundamentals", 1, 26, "Notes & scales — the alphabet of music"),
        ("music fundamentals", 2, 27, "Rhythm — the heartbeat of music"),
        ("music fundamentals", 3, 28, "Instruments — the voices of music"),
        ("music fundamentals", 4, 29, "Music history — where it all came from"),
        ("visual arts", 1, 30, "Color theory — understanding how colors work"),
        ("visual arts", 2, 31, "Elements of art — the building blocks"),
        ("visual arts", 3, 33, "Composition — arranging elements effectively"),
        ("visual arts", 4, 32, "Art history — learning from the masters"),
        ("critical thinking", 1, 34, "Logic & reasoning — the foundation of clear thinking"),
        ("critical thinking", 2, 35, "Ethics & morality — right, wrong, and everything between"),
        ("critical thinking", 3, 36, "Famous philosophers — standing on the shoulders of giants"),
        ("critical thinking", 4, 37, "Thought experiments — stretch your mind"),
        ("economics basics", 1, 38, "Supply & demand — how prices work"),
        ("economics basics", 2, 39, "Money & banking — the financial system"),
        ("economics basics", 3, 40, "Trade & globalization — the connected world"),
        ("economics basics", 4, 41, "Economic systems — how societies organize"),
        ("understanding people", 1, 42, "Introduction to psychology — the science of mind"),
        ("understanding people", 2, 43, "Memory & learning — how we acquire knowledge"),
        ("understanding people", 3, 44, "Emotions & motivation — what drives us"),
        ("understanding people", 4, 45, "Social psychology — how others shape us"),
        ("planet earth", 1, 46, "Ecosystems & biomes — the web of life"),
        ("planet earth", 2, 47, "Climate change — our warming world"),
        ("planet earth", 3, 48, "Pollution & waste — the cost of progress"),
        ("planet earth", 4, 49, "Conservation & sustainability — protecting our future"),
        ("creative writing", 1, 50, "Story structure — the skeleton of every narrative"),
        ("creative writing", 2, 51, "Character development — breathing life into people on the page"),
        ("creative writing", 3, 52, "Dialogue & voice — making characters speak and finding your style"),
        ("creative writing", 4, 53, "Poetry fundamentals — the art of compressed language"),
        ("space and astronomy", 1, 54, "The solar system — our cosmic neighborhood"),
        ("space and astronomy", 2, 55, "Stars — the life and death of suns"),
        ("space and astronomy", 3, 56, "Galaxies — island universes of stars"),
        ("space and astronomy", 4, 57, "Space exploration — humanity's journey beyond Earth"),
        ("physics fundamentals", 1, 58, "Electricity & magnetism — charges, currents, and fields"),
        ("physics fundamentals", 2, 60, "Waves & sound — how energy travels through space and matter"),
        ("physics fundamentals", 3, 59, "Thermodynamics — heat, energy, and entropy"),
        ("physics fundamentals", 4, 61, "Nuclear physics — atoms, radioactivity, and E=mc²"),
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
        assert_eq!(count, 58); // 56 previous + Cryptography + Information Theory
    }

    #[test]
    fn test_seed_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 58);
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
    fn test_all_topics_have_quiz_questions() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let orphans: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t WHERE NOT EXISTS (SELECT 1 FROM quiz_questions WHERE topic_id = t.id)",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(orphans, 0, "All topics should have at least one quiz question");
    }

    #[test]
    fn test_quiz_questions_exist() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM quiz_questions", [], |r| r.get(0)).unwrap();
        assert!(count >= 40, "Should have at least 40 quiz questions, got {}", count);
    }

    #[test]
    fn test_fill_in_blank_questions_exist() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM quiz_questions WHERE question_type = 'fill_in_blank'",
            [], |r| r.get(0)
        ).unwrap();
        assert!(count >= 5, "Should have at least 5 fill-in-the-blank questions, got {}", count);
    }

    #[test]
    fn test_music_and_art_subjects_exist() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let music: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Music'", [], |r| r.get(0)
        ).unwrap();
        let art: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Art'", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(music, 1);
        assert_eq!(art, 1);
    }

    #[test]
    fn test_philosophy_and_economics_subjects_exist() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let philosophy: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Philosophy'", [], |r| r.get(0)
        ).unwrap();
        let economics: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Economics'", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(philosophy, 1);
        assert_eq!(economics, 1);
    }

    #[test]
    fn test_new_subjects_have_topics() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let phil_topics: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Philosophy'",
            [], |r| r.get(0)
        ).unwrap();
        let econ_topics: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Economics'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(phil_topics, 4);
        assert_eq!(econ_topics, 4);
    }

    #[test]
    fn test_learning_paths_include_new_subjects() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let ct: i64 = conn.query_row(
            "SELECT COUNT(*) FROM learning_paths WHERE goal = 'critical thinking'",
            [], |r| r.get(0)
        ).unwrap();
        let econ: i64 = conn.query_row(
            "SELECT COUNT(*) FROM learning_paths WHERE goal = 'economics basics'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(ct, 4);
        assert_eq!(econ, 4);
    }

    #[test]
    fn test_psychology_and_envscience_subjects_exist() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let psych: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Psychology'", [], |r| r.get(0)
        ).unwrap();
        let env: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Environmental Science'", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(psych, 1);
        assert_eq!(env, 1);
    }

    #[test]
    fn test_new_subjects_have_complete_content() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        for (name, min_topics, min_lessons) in &[("Psychology", 7, 8), ("Environmental Science", 4, 8)] {
            let topic_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = ?1",
                [name], |r| r.get(0)
            ).unwrap();
            assert!(topic_count >= *min_topics, "{} should have at least {} topics, got {}", name, min_topics, topic_count);

            let lesson_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM lessons l JOIN topics t ON t.id = l.topic_id JOIN subjects s ON s.id = t.subject_id WHERE s.name = ?1",
                [name], |r| r.get(0)
            ).unwrap();
            assert!(lesson_count >= *min_lessons, "{} should have at least {} lessons, got {}", name, min_lessons, lesson_count);
        }
    }

    #[test]
    fn test_creative_writing_subject_exists() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let cw: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Creative Writing'", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(cw, 1);
        let topic_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Creative Writing'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(topic_count, 7);
    }

    #[test]
    fn test_learning_paths_include_psychology_and_envscience() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let psych_paths: i64 = conn.query_row(
            "SELECT COUNT(*) FROM learning_paths WHERE goal = 'understanding people'",
            [], |r| r.get(0)
        ).unwrap();
        let env_paths: i64 = conn.query_row(
            "SELECT COUNT(*) FROM learning_paths WHERE goal = 'planet earth'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(psych_paths, 4);
        assert_eq!(env_paths, 4);
    }

    #[test]
    fn test_chemistry_subject_exists() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let chem: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Chemistry'", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(chem, 1);
        let topic_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Chemistry'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(topic_count, 4);
        // Verify learning paths
        let path_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM learning_paths WHERE goal = 'chemistry basics'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(path_count, 4);
    }

    #[test]
    fn test_civics_subject_exists() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let civics: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Civics & Government'", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(civics, 1);
        let topic_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Civics & Government'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(topic_count, 4);
        let path_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM learning_paths WHERE goal = 'Civics Foundations'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(path_count, 4);
    }

    #[test]
    fn test_media_literacy_subject_exists() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let media: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subjects WHERE name = 'Media Literacy'", [], |r| r.get(0)
        ).unwrap();
        assert_eq!(media, 1);
        let topic_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Media Literacy'",
            [], |r| r.get(0)
        ).unwrap();
        assert_eq!(topic_count, 4);
        let quiz_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM quiz_questions q JOIN topics t ON t.id = q.topic_id JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Media Literacy'",
            [], |r| r.get(0)
        ).unwrap();
        assert!(quiz_count >= 10, "Media Literacy should have at least 10 quiz questions, got {}", quiz_count);
    }

    #[test]
    fn test_extra_geography_content() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let geo_quiz_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM quiz_questions q JOIN topics t ON t.id = q.topic_id JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'Geography'",
            [], |r| r.get(0)
        ).unwrap();
        assert!(geo_quiz_count >= 10, "Geography should have at least 10 quiz questions after expansion, got {}", geo_quiz_count);
    }
}

/// Assign varied difficulty levels to quiz questions that still have the default 'medium'.
/// True/false → easy, fill_in_blank → hard, multiple_choice → distributed by id.
fn assign_quiz_difficulties(conn: &Connection) -> Result<(), rusqlite::Error> {
    // True/false questions are generally easier
    conn.execute(
        "UPDATE quiz_questions SET difficulty = 'easy' WHERE question_type = 'true_false' AND difficulty = 'medium'",
        [],
    )?;
    // Fill-in-the-blank questions are harder (no options to choose from)
    conn.execute(
        "UPDATE quiz_questions SET difficulty = 'hard' WHERE question_type = 'fill_in_blank' AND difficulty = 'medium'",
        [],
    )?;
    // For multiple choice, assign roughly 40% easy, 30% medium, 30% hard based on id
    conn.execute(
        "UPDATE quiz_questions SET difficulty = 'easy' WHERE question_type = 'multiple_choice' AND difficulty = 'medium' AND id % 10 < 4",
        [],
    )?;
    conn.execute(
        "UPDATE quiz_questions SET difficulty = 'hard' WHERE question_type = 'multiple_choice' AND difficulty = 'medium' AND id % 10 >= 7",
        [],
    )?;
    Ok(())
}

// Additional seed function for Chemistry subject (added in improvement pass)
#[allow(clippy::type_complexity)]
pub fn seed_chemistry(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Check if Chemistry already exists
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Chemistry'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Chemistry', 'The science of matter — atoms, molecules, reactions, and the elements that make up everything around us.')",
        [],
    )?;
    let chem_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Chemistry'",
        [],
        |r| r.get(0),
    )?;

    // Topics
    let topics = [
        (chem_id, "Atoms & Elements", "beginner", 1),
        (chem_id, "Chemical Bonds", "beginner", 2),
        (chem_id, "Chemical Reactions", "intermediate", 3),
        (chem_id, "Acids, Bases & pH", "intermediate", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    // Get topic IDs
    let atoms_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Atoms & Elements'",
        [chem_id], |r| r.get(0),
    )?;
    let bonds_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Chemical Bonds'",
        [chem_id], |r| r.get(0),
    )?;
    let reactions_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Chemical Reactions'",
        [chem_id], |r| r.get(0),
    )?;
    let ph_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Acids, Bases & pH'",
        [chem_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (atoms_id, "The Building Blocks of Matter", "Everything around you is made of atoms — the smallest unit of an element that retains its chemical identity.\n\nAtomic structure:\n- Protons: positive charge, in the nucleus. Define the element (atomic number).\n- Neutrons: neutral, in the nucleus. Add mass; different counts create isotopes.\n- Electrons: negative charge, orbit the nucleus in shells/energy levels.\n\nThe Periodic Table organizes all 118 known elements by atomic number.\n- Rows (periods): number of electron shells.\n- Columns (groups): similar chemical properties.\n- Metals (left), nonmetals (right), metalloids (staircase line).\n\nKey elements:\n- Hydrogen (H, 1): simplest atom, most abundant in universe.\n- Carbon (C, 6): basis of organic chemistry and all life.\n- Oxygen (O, 8): essential for respiration and combustion.\n- Iron (Fe, 26): core of Earth, essential in blood (hemoglobin).", 1),
        (atoms_id, "Electron Configuration & the Periodic Table", "Electrons fill shells from lowest energy to highest:\n- 1st shell: max 2 electrons\n- 2nd shell: max 8 electrons\n- 3rd shell: max 18 electrons (but fills 8 first)\n\nValence electrons: electrons in the outermost shell.\n- Determine how an atom bonds with others.\n- Group 1 (alkali metals): 1 valence electron → very reactive.\n- Group 17 (halogens): 7 valence electrons → very reactive.\n- Group 18 (noble gases): 8 valence electrons (full shell) → stable, unreactive.\n\nOctet Rule: atoms tend to gain, lose, or share electrons to have 8 in their outer shell.\n\nTrends across the periodic table:\n- Atomic radius: decreases left→right, increases top→bottom.\n- Electronegativity: increases left→right (fluorine is highest).\n- Ionization energy: increases left→right.", 2),
        (bonds_id, "How Atoms Connect", "Chemical bonds form when atoms share or transfer electrons to become more stable.\n\nIonic bonds:\n- One atom gives electrons, another takes them.\n- Creates charged ions: cation (+) and anion (-).\n- Opposite charges attract → ionic compound.\n- Example: NaCl (table salt). Na gives 1e⁻ to Cl.\n- Properties: crystalline, high melting point, conduct electricity when dissolved.\n\nCovalent bonds:\n- Atoms share electrons.\n- Single bond: 1 shared pair. Double: 2 pairs. Triple: 3 pairs.\n- Example: H₂O — oxygen shares electrons with two hydrogens.\n- Properties: lower melting points, poor conductors.\n\nMetallic bonds:\n- Metal atoms share a 'sea' of delocalized electrons.\n- Explains conductivity, malleability, and luster.", 1),
        (bonds_id, "Molecular Shapes & Polarity", "VSEPR Theory: electron pairs around a central atom repel each other, determining molecular shape.\n\nCommon shapes:\n- Linear: 2 bonding pairs, 180° (CO₂)\n- Trigonal planar: 3 bonding pairs, 120° (BF₃)\n- Tetrahedral: 4 bonding pairs, 109.5° (CH₄)\n- Bent: 2 bonding + 1-2 lone pairs (H₂O, 104.5°)\n\nPolarity:\n- Nonpolar: electrons shared equally (O₂, CH₄).\n- Polar: unequal sharing, partial charges (H₂O, HCl).\n- Electronegativity difference determines polarity.\n\nWhy it matters:\n- Water is polar → dissolves salts, enables life.\n- Oil is nonpolar → doesn't mix with water.\n- 'Like dissolves like' — polar solvents dissolve polar solutes.", 2),
        (reactions_id, "Types of Chemical Reactions", "A chemical reaction rearranges atoms to form new substances.\n\nBalancing equations: atoms in = atoms out (conservation of mass).\n  2H₂ + O₂ → 2H₂O\n\nFive main types:\n1. Synthesis: A + B → AB (iron + sulfur → iron sulfide)\n2. Decomposition: AB → A + B (water → hydrogen + oxygen via electrolysis)\n3. Single replacement: A + BC → AC + B (zinc + hydrochloric acid → zinc chloride + hydrogen)\n4. Double replacement: AB + CD → AD + CB (silver nitrate + sodium chloride → silver chloride + sodium nitrate)\n5. Combustion: fuel + O₂ → CO₂ + H₂O (burning methane)\n\nExothermic: releases heat (combustion, rusting).\nEndothermic: absorbs heat (photosynthesis, melting ice).", 1),
        (reactions_id, "Reaction Rates & Equilibrium", "Reaction rate: how fast reactants become products.\n\nFactors affecting rate:\n- Temperature: higher → faster (particles move more, collide harder).\n- Concentration: more reactant molecules → more collisions.\n- Surface area: smaller pieces → more exposed surface → faster.\n- Catalysts: speed up reactions without being consumed. Enzymes are biological catalysts.\n\nActivation energy: minimum energy needed to start a reaction.\n  Catalysts lower the activation energy.\n\nChemical equilibrium: forward and reverse reactions occur at equal rates.\n  Le Chatelier's Principle: if you disturb equilibrium, the system shifts to counteract the change.\n  - Add more reactant → shifts toward products.\n  - Increase temperature → shifts toward endothermic direction.\n  - Increase pressure → shifts toward fewer gas molecules.", 2),
        (ph_id, "The pH Scale", "pH measures how acidic or basic a solution is.\n\nScale: 0 (most acidic) → 7 (neutral) → 14 (most basic/alkaline).\n\nAcids:\n- Taste sour, react with metals, pH < 7.\n- Release H⁺ ions in water.\n- Examples: lemon juice (pH 2), vinegar (pH 3), stomach acid (pH 1.5).\n\nBases (alkalis):\n- Taste bitter, feel slippery, pH > 7.\n- Release OH⁻ ions in water.\n- Examples: baking soda (pH 9), soap (pH 10), bleach (pH 13).\n\nNeutral: pure water (pH 7).\n\nThe pH scale is logarithmic: each step = 10× difference.\n  pH 3 is 10× more acidic than pH 4, and 100× more than pH 5.", 1),
        (ph_id, "Neutralization & Buffers", "Neutralization: acid + base → salt + water.\n  HCl + NaOH → NaCl + H₂O\n\nApplications:\n- Antacids neutralize stomach acid (Mg(OH)₂ + HCl).\n- Lime (CaO) neutralizes acidic soil for farming.\n- Treating acid rain damage in lakes.\n\nIndicators: substances that change color with pH.\n- Litmus paper: red in acid, blue in base.\n- Universal indicator: rainbow of colors across pH range.\n- Phenolphthalein: colorless in acid, pink in base.\n\nBuffers: solutions that resist pH changes.\n- Made of weak acid + its conjugate base.\n- Blood is buffered at pH 7.35-7.45 — critical for survival.\n- Even small deviations (acidosis/alkalosis) can be life-threatening.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (atoms_id, "atoms", "Atoms are the smallest unit of an element — the building blocks of all matter.", Some("Think of atoms like LEGO bricks — just as every LEGO creation is built from simple bricks, everything in the universe is built from atoms. There are about 118 types of 'bricks' (elements), and by combining them differently, you get water, air, gold, and even you!"), Some("If you could zoom in far enough to see a single atom, what do you think it would look like?")),
        (bonds_id, "chemical bonds", "Chemical bonds are the forces that hold atoms together to form molecules and compounds.", Some("Chemical bonds are like handshakes between atoms — some atoms share equally (covalent, like a cooperative handshake), some take from the other (ionic, like one person grabbing the other's hand), and metals are like a group high-five where everyone shares!"), Some("Why do you think salt (NaCl) dissolves in water but oil doesn't?")),
        (reactions_id, "chemical reactions", "A chemical reaction transforms one set of substances into another by rearranging atoms.", Some("A chemical reaction is like cooking — you start with raw ingredients (reactants), apply energy (heat), and end up with something completely new (products). You can't un-bake a cake, just like you can't easily reverse most reactions!"), Some("What happens when you mix baking soda and vinegar? What type of reaction is it?")),
        (ph_id, "pH", "pH is a scale from 0 to 14 that measures how acidic or basic a solution is.", Some("Think of pH like a temperature scale for sourness vs. slipperiness — lemon juice is very 'sour' (acidic, low pH), soap is very 'slippery' (basic, high pH), and pure water is right in the middle (neutral, pH 7)."), Some("What do you think happens to the pH of water when you add lemon juice?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let questions: Vec<QuizRow> = vec![
        (atoms_id, "What particle defines which element an atom is?", "multiple_choice", "Proton", Some("Electron"), Some("Proton"), Some("Neutron"), Some("Photon"), Some("It determines the atomic number"), "The number of protons (atomic number) defines the element. Change the protons and you change the element."),
        (atoms_id, "How many electrons can the first shell hold?", "multiple_choice", "2", Some("2"), Some("4"), Some("8"), Some("16"), Some("The simplest shell"), "The first electron shell can hold a maximum of 2 electrons."),
        (atoms_id, "Which group of elements is the most unreactive?", "multiple_choice", "Noble gases", Some("Alkali metals"), Some("Halogens"), Some("Noble gases"), Some("Transition metals"), Some("They have full outer shells"), "Noble gases (Group 18) are the most stable and unreactive because they have full valence shells."),
        (atoms_id, "True or false: Isotopes of an element have different numbers of protons.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Isotopes differ in which subatomic particle?"), "False. Isotopes have the same number of protons but different numbers of neutrons."),
        (atoms_id, "The element with atomic number 6, essential for all life, is ___.", "fill_in_blank", "Carbon", None, None, None, None, Some("Organic chemistry is based on this element"), "Carbon (C) has 6 protons and is the basis of all known life and organic chemistry."),
        (bonds_id, "In an ionic bond, atoms ___ electrons.", "multiple_choice", "Transfer", Some("Share"), Some("Transfer"), Some("Destroy"), Some("Absorb"), Some("One gives, one takes"), "In ionic bonds, one atom transfers electrons to another, creating oppositely charged ions."),
        (bonds_id, "What type of bond does water (H₂O) have?", "multiple_choice", "Covalent", Some("Ionic"), Some("Covalent"), Some("Metallic"), Some("Hydrogen"), Some("The atoms share electrons"), "Water has covalent bonds — oxygen shares electron pairs with two hydrogen atoms."),
        (bonds_id, "True or false: Metallic bonds explain why metals conduct electricity.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Delocalized electrons are free to move"), "True. The 'sea' of delocalized electrons in metallic bonds can flow, carrying electric current."),
        (bonds_id, "A molecule where electrons are shared unequally is called ___.", "fill_in_blank", "polar", None, None, None, None, Some("Water is an example"), "A polar molecule has unequal electron sharing, creating partial positive and negative charges."),
        (reactions_id, "What must be conserved in a balanced chemical equation?", "multiple_choice", "Mass (atoms)", Some("Energy"), Some("Mass (atoms)"), Some("Volume"), Some("Color"), Some("Atoms in = atoms out"), "Conservation of mass: the same number of each type of atom must appear on both sides of the equation."),
        (reactions_id, "Which reaction type combines two substances into one?", "multiple_choice", "Synthesis", Some("Decomposition"), Some("Synthesis"), Some("Combustion"), Some("Single replacement"), Some("A + B → AB"), "Synthesis (combination) reactions join two or more substances into a single product."),
        (reactions_id, "A catalyst speeds up a reaction by lowering the ___.", "fill_in_blank", "activation energy", None, None, None, None, Some("The energy barrier to start the reaction"), "Catalysts lower the activation energy, allowing more molecules to react at a given temperature."),
        (reactions_id, "True or false: Combustion reactions release heat.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Think about fire"), "True. Combustion is exothermic — it releases heat and light energy."),
        (ph_id, "What is the pH of pure water?", "multiple_choice", "7", Some("0"), Some("5"), Some("7"), Some("14"), Some("Right in the middle"), "Pure water has a pH of 7, which is neutral — neither acidic nor basic."),
        (ph_id, "Which substance is most acidic?", "multiple_choice", "Stomach acid (pH 1.5)", Some("Stomach acid (pH 1.5)"), Some("Lemon juice (pH 2)"), Some("Coffee (pH 5)"), Some("Milk (pH 6.5)"), Some("Lower pH = more acidic"), "Stomach acid has the lowest pH (1.5) and is therefore the most acidic."),
        (ph_id, "Acid + base → ___ + water", "fill_in_blank", "salt", None, None, None, None, Some("Neutralization reaction"), "Neutralization: acid + base → salt + water. Example: HCl + NaOH → NaCl + H₂O."),
        (ph_id, "True or false: The pH scale is linear.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Each step represents a 10× change"), "False. The pH scale is logarithmic — each unit represents a 10-fold change in H⁺ concentration."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    // Learning paths
    let paths = [
        ("chemistry basics", 1, atoms_id, "Atoms & elements — the building blocks of all matter"),
        ("chemistry basics", 2, bonds_id, "Chemical bonds — how atoms connect"),
        ("chemistry basics", 3, reactions_id, "Chemical reactions — transforming substances"),
        ("chemistry basics", 4, ph_id, "Acids, bases & pH — the chemistry of solutions"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_biology(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row("SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Biology'", [], |r| r.get(0))
        .unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Biology', 'The science of life — cells, genetics, evolution, and the diversity of living organisms.')",
        [],
    )?;
    let bio_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Biology'", [], |r| r.get(0))?;

    let topics = [
        (bio_id, "Cell Biology", "beginner", 1),
        (bio_id, "Genetics & DNA", "intermediate", 2),
        (bio_id, "Evolution & Natural Selection", "intermediate", 3),
        (bio_id, "Human Body Systems", "beginner", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let cell_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Cell Biology'", [bio_id], |r| r.get(0))?;
    let gen_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Genetics & DNA'", [bio_id], |r| r.get(0))?;
    let evo_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Evolution & Natural Selection'", [bio_id], |r| r.get(0))?;
    let body_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Human Body Systems'", [bio_id], |r| r.get(0))?;

    // Lessons
    let lessons: &[LessonRow] = &[
        (cell_id, "The Cell — Unit of Life", "All living things are made of cells — the smallest unit that can carry out life processes.\n\nTwo main types:\n- Prokaryotic: no nucleus, simple (bacteria, archaea). DNA floats in cytoplasm.\n- Eukaryotic: have a nucleus and membrane-bound organelles (plants, animals, fungi).\n\nKey organelles in eukaryotic cells:\n- Nucleus: contains DNA, controls cell activities.\n- Mitochondria: 'powerhouses' — produce ATP energy via cellular respiration.\n- Ribosomes: build proteins from mRNA instructions.\n- Endoplasmic reticulum (ER): smooth (lipid synthesis), rough (has ribosomes, protein processing).\n- Golgi apparatus: packages and ships proteins.\n- Cell membrane: phospholipid bilayer, controls what enters and exits.\n\nPlant cells also have:\n- Cell wall: rigid support (cellulose).\n- Chloroplasts: photosynthesis (convert sunlight to glucose).\n- Large central vacuole: stores water, maintains pressure.", 1),
        (cell_id, "Cell Division: Mitosis & Meiosis", "Cells reproduce by dividing:\n\nMitosis: produces 2 identical daughter cells.\n- For growth, repair, and asexual reproduction.\n- Phases: Prophase → Metaphase → Anaphase → Telophase.\n- Result: 2 diploid cells (2n), genetically identical to parent.\n\nMeiosis: produces 4 genetically unique cells.\n- For sexual reproduction (making gametes: sperm/eggs).\n- Two rounds of division: Meiosis I and Meiosis II.\n- Result: 4 haploid cells (n), each with half the chromosomes.\n- Crossing over in Prophase I creates genetic diversity.\n\nWhy it matters:\n- Mitosis: a cut heals, a child grows.\n- Meiosis: you are genetically unique because of it.\n- Cancer: uncontrolled mitosis.", 2),
        (gen_id, "DNA — The Blueprint of Life", "DNA (deoxyribonucleic acid) carries genetic instructions for all living organisms.\n\nStructure (Watson & Crick, 1953):\n- Double helix — two strands twisted like a spiral staircase.\n- Sugar-phosphate backbone with nitrogenous base pairs as 'rungs'.\n- Base pairing: A-T (adenine-thymine), C-G (cytosine-guanine).\n\nGene: a segment of DNA that codes for a specific protein.\nChromosome: a long, coiled DNA molecule. Humans have 46 (23 pairs).\nGenome: all of an organism's DNA.\n\nFrom DNA to protein (Central Dogma):\n1. Transcription: DNA → mRNA (in nucleus).\n2. Translation: mRNA → protein (at ribosomes).\n\nMutations: changes in DNA sequence.\n- Can be neutral, harmful (genetic diseases), or beneficial (evolution).\n- Types: substitution, insertion, deletion.", 1),
        (gen_id, "Heredity & Mendelian Genetics", "Gregor Mendel (1860s) discovered the rules of inheritance using pea plants.\n\nKey concepts:\n- Alleles: different versions of a gene (e.g., B = brown eyes, b = blue eyes).\n- Dominant (B): expressed even with one copy.\n- Recessive (b): only expressed with two copies (bb).\n- Genotype: genetic makeup (BB, Bb, bb).\n- Phenotype: observable trait (brown eyes, blue eyes).\n\nPunnett Squares: predict offspring ratios.\n  Bb × Bb → 1 BB : 2 Bb : 1 bb (3:1 phenotype ratio).\n\nBeyond Mendel:\n- Incomplete dominance: blend (red + white → pink flower).\n- Codominance: both expressed (AB blood type).\n- Polygenic traits: many genes → continuous variation (height, skin color).\n- Sex-linked traits: genes on X chromosome (color blindness, hemophilia).", 2),
        (evo_id, "Darwin & Natural Selection", "Charles Darwin proposed the theory of evolution by natural selection (1859).\n\nFour conditions for natural selection:\n1. Variation: individuals differ in traits.\n2. Inheritance: traits are passed to offspring.\n3. Overproduction: more offspring than can survive.\n4. Differential survival: traits that help survival and reproduction are passed on more.\n\nExamples:\n- Peppered moths: darker moths survived better on soot-covered trees during industrialization.\n- Darwin's finches: beak shapes adapted to different food sources on Galapagos Islands.\n- Antibiotic resistance: bacteria with resistant mutations survive and multiply.\n\nEvidence for evolution:\n- Fossil record: transitional forms (Archaeopteryx: dinosaur → bird).\n- Homologous structures: same bones, different functions (human arm, whale flipper, bat wing).\n- DNA comparisons: more similar DNA = more closely related.\n- Biogeography: island species resemble nearby mainland species.", 1),
        (evo_id, "Speciation & the Tree of Life", "Speciation: how one species becomes two or more.\n\nAllopatric speciation: geographic barrier separates populations.\n  Example: Grand Canyon split squirrel populations → two species.\n\nSympatric speciation: new species emerge without physical separation.\n  Example: polyploidy in plants (chromosome doubling).\n\nClassification (taxonomy):\n  Domain → Kingdom → Phylum → Class → Order → Family → Genus → Species\n  Mnemonic: Dear King Philip Came Over For Good Spaghetti.\n\nThree domains of life:\n- Bacteria: prokaryotes, most diverse.\n- Archaea: prokaryotes, extremophiles.\n- Eukarya: eukaryotes (protists, fungi, plants, animals).\n\nPhylogenetic trees: diagrams showing evolutionary relationships.\n  Branches = lineage splits. Closer branches = more closely related.", 2),
        (body_id, "Major Body Systems", "The human body has 11 organ systems working together:\n\n1. Circulatory: heart pumps blood (oxygen, nutrients) through vessels.\n   - Arteries (away from heart), veins (to heart), capillaries (exchange).\n2. Respiratory: lungs exchange O₂ and CO₂. Diaphragm drives breathing.\n3. Digestive: breaks down food → nutrients. Mouth → esophagus → stomach → intestines.\n4. Nervous: brain + spinal cord + nerves. Processes information, controls responses.\n5. Skeletal: 206 bones provide structure, protect organs, produce blood cells.\n6. Muscular: 600+ muscles enable movement. Skeletal, smooth, and cardiac types.", 1),
        (body_id, "Immunity & Homeostasis", "The immune system defends against pathogens (bacteria, viruses, fungi).\n\nInnate immunity (non-specific):\n- Skin: physical barrier.\n- Mucus, tears, stomach acid: chemical barriers.\n- White blood cells (phagocytes): engulf invaders.\n- Inflammation: increases blood flow to infected area.\n\nAdaptive immunity (specific):\n- B cells: produce antibodies that target specific pathogens.\n- T cells: kill infected cells directly (killer T) or coordinate response (helper T).\n- Memory cells: remember pathogens for faster future response → basis of vaccination.\n\nHomeostasis: maintaining stable internal conditions.\n- Body temperature: ~37°C. Sweat to cool, shiver to warm.\n- Blood sugar: insulin (lowers) and glucagon (raises).\n- Water balance: kidneys filter blood, adjust urine concentration.\n- Feedback loops: negative (most, stabilizing) and positive (rare, amplifying — e.g., childbirth contractions).", 2),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (cell_id, "cells", "Cells are the basic building blocks of all living things.", Some("A cell is like a tiny factory — the nucleus is the manager's office (holds the plans), mitochondria are the generators (make energy), ribosomes are the assembly lines (build products), and the cell membrane is the security gate (controls what comes in and out)."), Some("Why do you think plant cells need both chloroplasts AND mitochondria?")),
        (gen_id, "DNA", "DNA is the molecule that carries genetic instructions in all living organisms.", Some("DNA is like a recipe book written in a 4-letter alphabet (A, T, C, G). Each gene is one recipe. Your cells read these recipes to build the proteins that make you — your eye color, height, and how your body works."), Some("If both your parents have brown eyes, is it possible for you to have blue eyes?")),
        (evo_id, "natural selection", "Natural selection is the process where organisms with favorable traits survive and reproduce more.", Some("Imagine a bowl of M&Ms on a red tablecloth. A bird eats the ones it can see easily. The red M&Ms 'survive' because they blend in. Over many generations of this, you'd end up with mostly red M&Ms — that's natural selection."), Some("Can you think of an animal whose coloring helps it survive in its environment?")),
        (body_id, "immune system", "The immune system is your body's defense network against disease-causing organisms.", Some("Your immune system is like a castle's defense. The skin is the outer wall, mucus is the moat, white blood cells are the soldiers, and memory cells are the scouts who remember past invaders so the army is ready next time."), Some("Why do you only get chickenpox once, but can catch a cold many times?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let questions: &[QuizRow] = &[
        (cell_id, "Which organelle is known as the 'powerhouse of the cell'?", "multiple_choice", "Mitochondria", Some("Nucleus"), Some("Mitochondria"), Some("Ribosome"), Some("Golgi apparatus"), Some("It produces ATP energy"), "Mitochondria generate most of the cell's ATP through cellular respiration."),
        (cell_id, "True or false: Prokaryotic cells have a nucleus.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Bacteria are prokaryotes"), "False. Prokaryotic cells lack a membrane-bound nucleus; their DNA floats freely in the cytoplasm."),
        (cell_id, "Plant cells have a rigid outer layer made of cellulose called the cell ___.", "fill_in_blank", "wall", None, None, None, None, Some("It provides structural support"), "The cell wall gives plant cells their rigid shape and structural support."),
        (cell_id, "Mitosis produces how many daughter cells?", "multiple_choice", "2", Some("1"), Some("2"), Some("4"), Some("8"), Some("Identical copies"), "Mitosis produces 2 genetically identical daughter cells."),
        (cell_id, "Which type of cell division produces gametes?", "multiple_choice", "Meiosis", Some("Mitosis"), Some("Meiosis"), Some("Binary fission"), Some("Budding"), Some("Sperm and eggs"), "Meiosis produces 4 haploid gametes (sex cells) with genetic diversity."),
        (gen_id, "What does DNA stand for?", "multiple_choice", "Deoxyribonucleic acid", Some("Deoxyribonucleic acid"), Some("Dinitrogen acid"), Some("Dynamic nucleic assembly"), Some("Dual nitrogen acid"), Some("It's a nucleic acid"), "DNA = Deoxyribonucleic acid, the molecule that carries genetic instructions."),
        (gen_id, "In DNA, adenine (A) always pairs with ___.", "fill_in_blank", "thymine", None, None, None, None, Some("A-T and C-G"), "Adenine pairs with thymine (A-T) via two hydrogen bonds. Cytosine pairs with guanine (C-G)."),
        (gen_id, "A Bb genotype is described as:", "multiple_choice", "Heterozygous", Some("Homozygous dominant"), Some("Homozygous recessive"), Some("Heterozygous"), Some("Codominant"), Some("Two different alleles"), "Heterozygous means having two different alleles (Bb) for a gene."),
        (gen_id, "True or false: All mutations are harmful.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Think about evolution"), "False. Mutations can be neutral, harmful, or beneficial. Beneficial mutations drive evolution."),
        (gen_id, "Humans have ___ pairs of chromosomes.", "fill_in_blank", "23", None, None, None, None, Some("46 total"), "Humans have 23 pairs (46 total) of chromosomes in each body cell."),
        (evo_id, "Who proposed the theory of evolution by natural selection?", "multiple_choice", "Charles Darwin", Some("Gregor Mendel"), Some("Charles Darwin"), Some("Louis Pasteur"), Some("James Watson"), Some("Published in 1859"), "Charles Darwin published 'On the Origin of Species' in 1859."),
        (evo_id, "True or false: Evolution means 'survival of the fittest individual'.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Fitness means reproductive success, not physical strength"), "False. In biology, 'fitness' means reproductive success — passing genes to the next generation, not physical strength."),
        (evo_id, "Structures with the same origin but different functions (like a human arm and whale flipper) are called ___ structures.", "fill_in_blank", "homologous", None, None, None, None, Some("Same bones, different uses"), "Homologous structures share a common evolutionary origin but may serve different functions."),
        (evo_id, "What type of speciation occurs when a geographic barrier separates populations?", "multiple_choice", "Allopatric", Some("Sympatric"), Some("Allopatric"), Some("Parapatric"), Some("Peripatric"), Some("Think 'allo' = other place"), "Allopatric speciation occurs when populations are geographically isolated from each other."),
        (body_id, "Which system pumps blood through the body?", "multiple_choice", "Circulatory", Some("Respiratory"), Some("Circulatory"), Some("Digestive"), Some("Nervous"), Some("The heart is the central organ"), "The circulatory system (heart + blood vessels) pumps blood carrying oxygen and nutrients."),
        (body_id, "True or false: Antibodies are produced by T cells.", "true_false", "false", Some("true"), Some("false"), None, None, Some("B cells or T cells?"), "False. Antibodies are produced by B cells. T cells kill infected cells directly or coordinate immune responses."),
        (body_id, "The process of maintaining stable internal conditions is called ___.", "fill_in_blank", "homeostasis", None, None, None, None, Some("Body temperature, blood sugar, water balance"), "Homeostasis is the maintenance of a stable internal environment despite external changes."),
        (body_id, "How many bones are in the adult human skeleton?", "multiple_choice", "206", Some("106"), Some("206"), Some("306"), Some("406"), Some("Just over 200"), "The adult human skeleton has 206 bones."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    // Learning paths
    let paths = [
        ("biology fundamentals", 1, cell_id, "Cell biology — the building blocks of life"),
        ("biology fundamentals", 2, gen_id, "Genetics & DNA — the blueprint of inheritance"),
        ("biology fundamentals", 3, evo_id, "Evolution — how life changes over time"),
        ("biology fundamentals", 4, body_id, "Human body systems — how your body works"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_sociology(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row("SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Sociology'", [], |r| r.get(0))
        .unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Sociology', 'The study of society — social structures, institutions, inequality, and how groups shape human behavior.')",
        [],
    )?;
    let soc_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Sociology'", [], |r| r.get(0))?;

    let topics = [
        (soc_id, "Social Structures & Institutions", "beginner", 1),
        (soc_id, "Culture & Socialization", "beginner", 2),
        (soc_id, "Social Inequality", "intermediate", 3),
        (soc_id, "Collective Behavior & Movements", "intermediate", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let struct_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Social Structures & Institutions'", [soc_id], |r| r.get(0))?;
    let culture_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Culture & Socialization'", [soc_id], |r| r.get(0))?;
    let ineq_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Social Inequality'", [soc_id], |r| r.get(0))?;
    let move_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Collective Behavior & Movements'", [soc_id], |r| r.get(0))?;

    // Lessons
    let lessons: &[LessonRow] = &[
        (struct_id, "What is Social Structure?", "Social structure is the organized pattern of relationships and institutions that together form society.\n\nKey concepts:\n- Status: a position in society (student, parent, CEO). Can be ascribed (born into) or achieved (earned).\n- Role: the expected behavior attached to a status. Role conflict: when roles clash (student vs. employee).\n- Groups: primary (close, personal — family, friends) vs. secondary (formal, goal-oriented — coworkers, classmates).\n\nSocial institutions: organized systems that meet society's basic needs.\n- Family: socialization, emotional support, reproduction.\n- Education: knowledge transfer, social integration, sorting.\n- Economy: production and distribution of goods/services.\n- Government: order, laws, collective decision-making.\n- Religion: meaning, moral framework, community.\n- Healthcare: maintaining population health.\n\nBureaucracy (Max Weber): hierarchical organization with clear rules, specialization, and impersonal relationships. Efficient but can become rigid ('iron cage').", 1),
        (struct_id, "Sociological Perspectives", "Three major theoretical frameworks:\n\n1. Functionalism (Durkheim, Parsons):\n   - Society is a system of interconnected parts working together.\n   - Each institution serves a function for stability.\n   - Dysfunction: when a part fails (e.g., failing schools).\n   - Criticism: overlooks inequality and conflict.\n\n2. Conflict Theory (Marx, Weber):\n   - Society is shaped by struggles between groups for power and resources.\n   - Institutions benefit the powerful at the expense of others.\n   - Focus: class, race, gender inequality.\n   - Criticism: overemphasizes conflict, underestimates cooperation.\n\n3. Symbolic Interactionism (Mead, Goffman):\n   - Society is created through everyday interactions and shared meanings.\n   - People act based on the meanings they assign to things.\n   - Goffman's dramaturgical approach: social life as a stage performance.\n   - Criticism: ignores larger structural forces.", 2),
        (culture_id, "Culture — The Fabric of Society", "Culture: shared beliefs, values, norms, and material objects of a group.\n\nComponents:\n- Values: abstract ideals (freedom, equality, hard work).\n- Norms: specific rules of behavior.\n  - Folkways: informal customs (saying 'please').\n  - Mores: serious norms with moral significance (honesty).\n  - Taboos: strongly forbidden (incest, cannibalism).\n  - Laws: formally enacted norms with penalties.\n- Symbols: things that carry meaning (flags, words, gestures).\n- Language: Sapir-Whorf hypothesis — language shapes thought.\n\nCultural concepts:\n- Ethnocentrism: judging other cultures by your own standards.\n- Cultural relativism: understanding a culture on its own terms.\n- Subculture: a group within a larger culture with distinct values (gamers, surfers).\n- Counterculture: actively opposes dominant culture (hippies, punks).\n- Culture shock: disorientation when encountering an unfamiliar culture.", 1),
        (culture_id, "Socialization — Becoming a Person", "Socialization: the lifelong process of learning norms, values, and roles.\n\nAgents of socialization:\n- Family: primary agent; language, values, identity.\n- Peers: conformity, independence, social skills.\n- School: formal knowledge, hidden curriculum (punctuality, obedience).\n- Media: shapes worldview, norms, desires.\n- Religion: moral framework, community belonging.\n- Workplace: professional norms, adult identity.\n\nKey theories:\n- Cooley's 'Looking-Glass Self': we see ourselves through others' reactions.\n  1. We imagine how we appear to others.\n  2. We imagine their judgment.\n  3. We develop feelings about ourselves based on that.\n- Mead's stages: imitation → play (taking one role) → game (understanding multiple roles) → generalized other.\n- Erikson's 8 stages of psychosocial development: trust vs. mistrust through integrity vs. despair.\n\nResocialization: radical change in identity (military boot camp, prison, religious conversion).", 2),
        (ineq_id, "Social Stratification", "Social stratification: the hierarchical ranking of people into layers based on wealth, power, and prestige.\n\nSystems of stratification:\n- Caste: closed system, status assigned at birth (historical India).\n- Class: open system, based on economic position. Mobility possible.\n- Meritocracy: ideal where position is earned by ability and effort.\n\nSocial class (typical model):\n- Upper class: great wealth, power, influence (~1-5%).\n- Middle class: professionals, managers, moderate comfort.\n- Working class: manual and service labor, less security.\n- Lower class / poverty: limited resources, systemic barriers.\n\nTheories of inequality:\n- Marx: bourgeoisie (owners) vs. proletariat (workers). Class conflict drives history.\n- Weber: class (wealth) + status (prestige) + party (power) — three dimensions.\n- Davis-Moore: inequality motivates people to fill important positions (controversial).\n\nSocial mobility:\n- Intragenerational: within one's lifetime.\n- Intergenerational: compared to parents.\n- Structural: due to economic changes (industrialization creates new middle class).", 1),
        (ineq_id, "Race, Gender & Intersectionality", "Social inequality operates along multiple axes:\n\nRace & ethnicity:\n- Race: socially constructed categories based on perceived physical differences.\n- Ethnicity: shared cultural heritage (language, religion, customs).\n- Racism: prejudice + power → systemic advantage/disadvantage.\n- Institutional racism: discrimination embedded in laws, policies, practices.\n\nGender:\n- Sex: biological (chromosomes, anatomy).\n- Gender: socially constructed roles, behaviors, expectations.\n- Gender socialization: begins at birth (colors, toys, expectations).\n- Patriarchy: system where men hold primary power.\n- Gender pay gap: women earn less on average for similar work.\n\nIntersectionality (Kimberle Crenshaw, 1989):\n- Multiple identities (race, gender, class, sexuality) intersect.\n- Creates unique experiences of privilege and oppression.\n- A Black woman's experience differs from both a white woman's and a Black man's.\n- Cannot understand inequality by looking at one dimension alone.", 2),
        (move_id, "Social Movements & Change", "Social movements: organized efforts by groups to promote or resist change.\n\nTypes:\n- Reform: change within existing system (civil rights, labor rights).\n- Revolutionary: overthrow existing system (French Revolution).\n- Resistance/reactionary: oppose change, return to previous state.\n- New social movements: identity and quality of life (environmentalism, LGBTQ+ rights).\n\nStages of a social movement:\n1. Emergence: widespread dissatisfaction, problem recognized.\n2. Coalescence: organized leadership, strategy, collective action.\n3. Bureaucratization: formal organization, professional staff.\n4. Decline: success (institutionalization), failure, co-optation, or repression.\n\nKey concepts:\n- Collective action problem: why join if others will do it? (free rider problem).\n- Resource mobilization theory: movements need resources (money, media, networks).\n- Framing theory: how issues are presented shapes public support.\n- Political opportunity theory: movements succeed when political conditions are favorable.", 1),
        (move_id, "Deviance & Social Control", "Deviance: behavior that violates social norms (not necessarily illegal).\n\nTheories of deviance:\n- Durkheim: deviance is normal and functional — reinforces norms, promotes social change.\n- Merton's Strain Theory: deviance results from a gap between cultural goals and legitimate means.\n  - Conformity, Innovation, Ritualism, Retreatism, Rebellion.\n- Labeling Theory (Becker): deviance is not inherent — it's a label applied by society.\n  - Primary deviance: initial act. Secondary: identity built around the label.\n- Differential Association (Sutherland): deviance is learned through social interaction.\n\nSocial control: mechanisms society uses to enforce conformity.\n- Informal: gossip, ridicule, praise, ostracism.\n- Formal: laws, police, courts, prisons.\n\nThe criminal justice system:\n- Retribution: punishment proportional to the offense.\n- Deterrence: discouraging future crime.\n- Rehabilitation: reforming offenders.\n- Restorative justice: repairing harm, reconciling offender and victim.", 2),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (struct_id, "social structure", "Social structure is the organized pattern of statuses, roles, groups, and institutions that shape society.", Some("Social structure is like the skeleton of a building — you can't see it from outside, but it determines the shape of everything. The walls (institutions), floors (social classes), and rooms (groups) are all held in place by this invisible framework."), Some("Can you think of a role you play that sometimes conflicts with another role in your life?")),
        (culture_id, "culture", "Culture is the shared set of beliefs, values, norms, and symbols that define a group's way of life.", Some("Culture is like the operating system of a society — just as Windows or macOS determines how your computer behaves, culture determines how people in a society think, act, and interact. Different societies run different 'operating systems', which is why customs vary so much around the world."), Some("Can you think of something that's completely normal in your culture but might be strange in another?")),
        (ineq_id, "social inequality", "Social inequality is the unequal distribution of resources, opportunities, and privileges among members of a society.", Some("Social inequality is like a board game where different players start with different amounts of money, different rules apply to different players, and the winners get to make the rules for the next round. Understanding this helps explain why 'just work harder' isn't always sufficient advice."), Some("Why might two equally talented people end up in very different economic situations?")),
        (move_id, "social movements", "Social movements are organized collective efforts to promote or resist social change.", Some("A social movement is like a river formed from many small streams — individual frustrations and ideas flow together, gaining force until they can reshape the landscape of society. Some rivers carve new paths; others get dammed up."), Some("What social movement in history do you think had the biggest impact on your daily life?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let questions: &[QuizRow] = &[
        (struct_id, "Which sociological perspective views society as a system of interconnected parts?", "multiple_choice", "Functionalism", Some("Conflict theory"), Some("Functionalism"), Some("Symbolic interactionism"), Some("Postmodernism"), Some("Think Durkheim"), "Functionalism sees society as a system where each part contributes to overall stability."),
        (struct_id, "True or false: An achieved status is one you are born into.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Ascribed vs. achieved"), "False. An achieved status is earned through effort (doctor, graduate). Ascribed statuses are assigned at birth (race, royal title)."),
        (struct_id, "Max Weber described bureaucracy as an 'iron ___'.", "fill_in_blank", "cage", None, None, None, None, Some("Efficient but constraining"), "Weber warned that bureaucracy, while efficient, could trap people in an 'iron cage' of rationality and rigid rules."),
        (struct_id, "Which perspective focuses on how people create meaning through everyday interactions?", "multiple_choice", "Symbolic interactionism", Some("Functionalism"), Some("Conflict theory"), Some("Symbolic interactionism"), Some("Structural functionalism"), Some("Mead and Goffman"), "Symbolic interactionism studies how people construct meaning through their daily social interactions."),
        (culture_id, "Judging another culture by the standards of your own culture is called:", "multiple_choice", "Ethnocentrism", Some("Cultural relativism"), Some("Ethnocentrism"), Some("Multiculturalism"), Some("Subculture"), Some("A common bias"), "Ethnocentrism is evaluating other cultures according to the norms and values of your own."),
        (culture_id, "Strongly forbidden norms (like cannibalism) are called ___.", "fill_in_blank", "taboos", None, None, None, None, Some("The strongest type of norm"), "Taboos are the most strongly prohibited norms in any society."),
        (culture_id, "True or false: The family is the primary agent of socialization.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Where do children first learn norms?"), "True. Family is typically the first and most influential agent of socialization."),
        (culture_id, "Cooley's theory that we see ourselves through others' reactions is called the:", "multiple_choice", "Looking-glass self", Some("Generalized other"), Some("Looking-glass self"), Some("Dramaturgical approach"), Some("Social identity"), Some("Like a mirror"), "The looking-glass self (Cooley): we imagine how others see us, imagine their judgment, and feel accordingly."),
        (ineq_id, "Which system of stratification assigns status at birth with no mobility?", "multiple_choice", "Caste", Some("Class"), Some("Meritocracy"), Some("Caste"), Some("Estate"), Some("Historical India"), "Caste systems are closed — status is determined at birth and cannot be changed."),
        (ineq_id, "The concept of intersectionality was introduced by:", "multiple_choice", "Kimberle Crenshaw", Some("Karl Marx"), Some("Kimberle Crenshaw"), Some("Max Weber"), Some("Emile Durkheim"), Some("1989, focused on race and gender"), "Kimberle Crenshaw coined 'intersectionality' in 1989 to describe how race, gender, and class overlap."),
        (ineq_id, "True or false: According to Marx, history is driven by class conflict.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Bourgeoisie vs. proletariat"), "True. Marx argued that the struggle between those who own the means of production and workers drives historical change."),
        (ineq_id, "___ is the socially constructed set of roles and expectations associated with being male or female.", "fill_in_blank", "gender", None, None, None, None, Some("Different from biological sex"), "Gender refers to socially constructed roles and behaviors, as opposed to biological sex."),
        (move_id, "Which theory says social movements need money, media, and networks to succeed?", "multiple_choice", "Resource mobilization", Some("Framing theory"), Some("Strain theory"), Some("Resource mobilization"), Some("Labeling theory"), Some("Resources matter"), "Resource mobilization theory emphasizes that movements need tangible resources to organize and sustain action."),
        (move_id, "True or false: Deviance is always illegal.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Norms vs. laws"), "False. Deviance means violating social norms, which may or may not be laws. Burping loudly in public is deviant but not illegal."),
        (move_id, "Merton's theory that deviance results from a gap between goals and means is called ___ theory.", "fill_in_blank", "strain", None, None, None, None, Some("The pressure to succeed"), "Merton's Strain Theory: when people lack legitimate means to achieve cultural goals, they may turn to deviant behavior."),
        (move_id, "Which type of social movement seeks to completely overthrow the existing system?", "multiple_choice", "Revolutionary", Some("Reform"), Some("Revolutionary"), Some("Resistance"), Some("Redemptive"), Some("Think French Revolution"), "Revolutionary movements aim to completely replace the existing social or political order."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    // Learning paths
    let paths = [
        ("sociology fundamentals", 1, struct_id, "Social structures & institutions — how society is organized"),
        ("sociology fundamentals", 2, culture_id, "Culture & socialization — how we learn to be social"),
        ("sociology fundamentals", 3, ineq_id, "Social inequality — why resources are unevenly distributed"),
        ("sociology fundamentals", 4, move_id, "Social movements — how societies change"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_linguistics(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Check for topics, not subject — subject may already exist from seed_subjects
    let has_topics: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM topics t JOIN subjects s ON t.subject_id = s.id WHERE s.name = 'Linguistics'",
        [], |r| r.get(0),
    ).unwrap_or(false);
    if has_topics { return Ok(()); }

    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('Linguistics', 'The scientific study of language — its structure, meaning, sounds, and evolution across cultures.')",
        [],
    )?;
    let ling_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Linguistics'", [], |r| r.get(0))?;

    let topics = [
        (ling_id, "Phonetics & Phonology", "beginner", 1),
        (ling_id, "Morphology & Syntax", "intermediate", 2),
        (ling_id, "Semantics & Pragmatics", "intermediate", 3),
        (ling_id, "Language Families & Change", "advanced", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let phon_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Phonetics & Phonology'", [ling_id], |r| r.get(0))?;
    let morph_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Morphology & Syntax'", [ling_id], |r| r.get(0))?;
    let sem_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Semantics & Pragmatics'", [ling_id], |r| r.get(0))?;
    let fam_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Language Families & Change'", [ling_id], |r| r.get(0))?;

    // Lessons
    let lessons: &[LessonRow] = &[
        (phon_id, "Sounds of Language", "Phonetics studies the physical sounds humans produce; phonology studies how languages organize those sounds.\n\nKey concepts:\n- Phone: any speech sound (written in square brackets, e.g. [p]).\n- Phoneme: a sound that distinguishes meaning in a language (written in slashes, e.g. /p/).\n- Minimal pair: two words that differ by exactly one phoneme (e.g. 'bat' vs 'pat').\n\nPlaces of articulation:\n- Bilabial: both lips (p, b, m).\n- Alveolar: tongue on the ridge behind teeth (t, d, n, s).\n- Velar: back of tongue on soft palate (k, g, ŋ).\n- Glottal: vocal folds (h, glottal stop ʔ).\n\nManners of articulation:\n- Stop/Plosive: complete closure then release (p, t, k).\n- Fricative: narrow constriction creating turbulence (f, s, ʃ).\n- Nasal: air through the nose (m, n, ŋ).\n- Approximant: narrowing without friction (w, r, l, j).\n\nVoicing: vocal folds vibrate (voiced: b, d, g) or not (voiceless: p, t, k).\n\nVowels are classified by height (high/mid/low), backness (front/central/back), and rounding.", 1),
        (phon_id, "Phonological Rules & Processes", "Languages have rules that modify sounds in context:\n\nAssimilation: a sound becomes more like a neighboring sound.\n- 'input' is often pronounced [ɪmpʊt] — /n/ becomes [m] before /p/.\n- Voicing assimilation: 'dogs' [dɔɡz] vs 'cats' [kæts].\n\nDissimilation: sounds become less alike to avoid repetition.\n- Latin 'peregrinus' → 'pilgrim' (r...r → l...r).\n\nDeletion: sounds are dropped.\n- 'comfortable' → [kʌmftəbl] — vowel deletion.\n\nEpenthesis: sounds are inserted.\n- 'something' → [sʌmpθɪŋ] — [p] inserted between [m] and [θ].\n\nMetathesis: sounds swap positions.\n- Old English 'brid' → modern 'bird'.\n\nSuprasegmentals:\n- Stress: which syllable is emphasized ('REcord' noun vs 'reCORD' verb).\n- Tone: pitch distinguishes meaning (Mandarin: mā/má/mǎ/mà — four different words).\n- Intonation: pitch pattern over a sentence (rising = question in English).", 2),
        (morph_id, "Building Words — Morphology", "Morphology studies the internal structure of words.\n\nMorpheme: the smallest unit of meaning.\n- Free morphemes: can stand alone ('book', 'run', 'happy').\n- Bound morphemes: must attach to another ('un-', '-ness', '-ed').\n\nTypes of bound morphemes:\n- Prefix: before the root ('un-happy', 're-write').\n- Suffix: after the root ('happi-ness', 'teach-er').\n- Infix: inside the root (Tagalog: 'sulat' → 's-um-ulat').\n\nInflection vs. Derivation:\n- Inflection: changes grammatical function without changing category.\n  - 'walk' → 'walks' / 'walked' / 'walking' (still a verb).\n- Derivation: creates a new word, often changing category.\n  - 'happy' (adj) → 'happiness' (noun) → 'unhappiness' (noun).\n\nWord formation processes:\n- Compounding: combining free morphemes ('blackboard', 'sunflower').\n- Blending: merging parts of words ('brunch' = breakfast + lunch).\n- Clipping: shortening ('exam' from 'examination').\n- Acronyms: initials as words ('NASA', 'scuba').\n- Back-formation: removing a supposed affix ('edit' from 'editor').", 1),
        (morph_id, "Sentence Structure — Syntax", "Syntax studies how words combine into phrases and sentences.\n\nConstituency: words group into phrases.\n- Noun phrase (NP): 'the big red ball'\n- Verb phrase (VP): 'kicked the ball quickly'\n- Prepositional phrase (PP): 'in the park'\n\nPhrase structure rules (simplified):\n- S → NP VP (a sentence is a noun phrase + verb phrase)\n- NP → (Det) (Adj*) N (PP*)\n- VP → V (NP) (PP*) (AdvP*)\n\nTree diagrams: visual representations of hierarchical structure.\n\nRecursion: phrases can contain phrases of the same type.\n- 'The cat [that chased the mouse [that ate the cheese]]'\n- This is what makes human language infinite from finite rules.\n\nUniversal Grammar (Chomsky): all humans are born with an innate language faculty.\n- Explains why children acquire language so quickly.\n- Languages differ on the surface but share deep structural principles.\n\nWord order typology:\n- SVO: English, Spanish, Chinese ('She reads books').\n- SOV: Japanese, Korean, Hindi ('She books reads').\n- VSO: Irish, Arabic, Welsh ('Reads she books').", 2),
        (sem_id, "Meaning in Language — Semantics", "Semantics studies meaning at the word and sentence level.\n\nLexical relations:\n- Synonymy: similar meaning ('big' / 'large').\n- Antonymy: opposite meaning ('hot' / 'cold', 'alive' / 'dead').\n  - Gradable: 'hot' / 'cold' (degrees exist).\n  - Complementary: 'alive' / 'dead' (no middle ground).\n- Hyponymy: 'rose' is a hyponym of 'flower' (IS-A relation).\n- Meronymy: 'wheel' is a meronym of 'car' (PART-OF relation).\n- Polysemy: one word, multiple related meanings ('bank' of a river / financial bank).\n- Homophony: same sound, different meanings ('bare' / 'bear').\n\nCompositional semantics: meaning of a sentence comes from its parts + structure.\n- 'The dog chased the cat' ≠ 'The cat chased the dog' (same words, different meaning).\n\nEntailment: if A is true, B must be true.\n- 'She murdered him' entails 'He is dead'.\n\nPresupposition: background assumption.\n- 'Have you stopped cheating?' presupposes you were cheating.", 1),
        (sem_id, "Language in Context — Pragmatics", "Pragmatics studies how context shapes meaning beyond the literal.\n\nSpeech acts (Austin & Searle): utterances that perform actions.\n- Locutionary: the literal meaning ('It's cold in here').\n- Illocutionary: the intended meaning (request to close the window).\n- Perlocutionary: the actual effect (someone closes the window).\n\nGrice's Cooperative Principle: conversations follow implicit rules.\nMaxims:\n- Quantity: say enough, but not too much.\n- Quality: be truthful.\n- Relation: be relevant.\n- Manner: be clear and orderly.\n\nImplicature: what is implied but not literally said.\n- 'Some students passed' → implicates 'not all students passed'.\n- 'Nice weather, isn't it?' (during a storm) → sarcasm via maxim violation.\n\nDeixis: words whose meaning depends on context.\n- Person deixis: 'I', 'you' (who is speaking?).\n- Spatial deixis: 'here', 'there', 'this', 'that'.\n- Temporal deixis: 'now', 'then', 'yesterday'.\n\nPoliteness theory (Brown & Levinson): strategies for face-saving.\n- Positive face: desire to be liked and approved of.\n- Negative face: desire not to be imposed upon.", 2),
        (fam_id, "Language Families of the World", "Languages are grouped into families descended from common ancestors.\n\nMajor families:\n- Indo-European (~3.2 billion speakers): English, Spanish, Hindi, Russian, Greek, Persian.\n  - Sub-branches: Germanic, Romance, Slavic, Indo-Iranian, Celtic.\n- Sino-Tibetan (~1.3 billion): Mandarin, Cantonese, Burmese, Tibetan.\n- Afro-Asiatic (~500 million): Arabic, Hebrew, Amharic, Hausa, Somali.\n- Niger-Congo (~500 million): Swahili, Yoruba, Zulu, Igbo.\n- Austronesian (~400 million): Malay, Tagalog, Maori, Hawaiian.\n- Dravidian (~250 million): Tamil, Telugu, Kannada, Malayalam.\n- Turkic (~200 million): Turkish, Uzbek, Kazakh, Azerbaijani.\n\nLanguage isolates: no known relatives.\n- Basque (Spain/France), Korean (debated), Ainu (Japan).\n\nThe comparative method: reconstruct proto-languages by finding systematic sound correspondences.\n- Latin 'pater', Sanskrit 'pitar', English 'father' → Proto-Indo-European *ph₂tér.\n\nThere are ~7,000 languages alive today, but ~40% are endangered.", 1),
        (fam_id, "How Languages Change", "All living languages change constantly.\n\nSound change:\n- Great Vowel Shift (1400-1700): English long vowels systematically raised.\n  - 'bite' was once pronounced like modern 'beet'.\n- Grimm's Law: Proto-Indo-European stops shifted in Germanic languages.\n  - PIE *p → Germanic f (Latin 'pater' → English 'father').\n  - PIE *t → Germanic θ (Latin 'tres' → English 'three').\n\nSemantic change:\n- Broadening: 'dog' once meant a specific breed, now all dogs.\n- Narrowing: 'meat' once meant any food, now only animal flesh.\n- Amelioration: 'knight' meant servant → noble warrior.\n- Pejoration: 'villain' meant peasant → evil person.\n\nGrammaticalization: content words become function words.\n- 'going to' (motion) → 'gonna' (future tense marker).\n\nLanguage contact:\n- Borrowing: English borrowed 'piano' (Italian), 'kindergarten' (German), 'tsunami' (Japanese).\n- Pidgins: simplified contact languages for trade.\n- Creoles: pidgins that become native languages with full grammar.\n\nLanguage death: when the last speaker dies. ~1 language dies every 2 weeks.", 2),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (phon_id, "phoneme", "A phoneme is the smallest unit of sound that can change meaning in a language.", Some("Think of phonemes like letters in a combination lock — each position matters. Changing just one 'click' (sound) gives you a completely different result. 'Bat' vs 'pat' differ by one phoneme, just like changing one digit in a lock opens a different safe."), Some("Can you find a minimal pair — two words in your language that differ by only one sound?")),
        (morph_id, "morpheme", "A morpheme is the smallest meaningful unit in language — it carries meaning but cannot be broken down further.", Some("Morphemes are like LEGO bricks. Each brick (morpheme) has its own shape and purpose. You can snap them together to build complex structures (words). 'Un-break-able' is three bricks: 'un' (not) + 'break' (smash) + 'able' (capable of)."), Some("How many morphemes can you find in the word 'unhappiness'?")),
        (sem_id, "implicature", "An implicature is something implied but not literally said — meaning that goes beyond the words.", Some("Implicature is like reading between the lines of a text message. When your friend texts 'I'm fine.' with a period, they're literally saying they're okay — but you know from context they're probably not fine at all."), Some("If someone says 'I have two children', do they necessarily have exactly two?")),
        (fam_id, "proto-language", "A proto-language is the reconstructed common ancestor of a language family — it was never written down but is inferred from its descendant languages.", Some("A proto-language is like a family tree's root ancestor. You might never meet your great-great-great-grandmother, but by looking at family photos and shared features of living relatives, you can deduce what she probably looked like."), Some("Why do you think English 'father', Latin 'pater', and Sanskrit 'pitar' all look similar?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let questions: &[QuizRow] = &[
        (phon_id, "What is a minimal pair?", "multiple_choice", "Two words differing by one phoneme", Some("Two words with the same meaning"), Some("Two words differing by one phoneme"), Some("Two sounds that are always interchangeable"), Some("A pair of consonants"), Some("Think 'bat' vs 'pat'"), "A minimal pair consists of two words that differ in exactly one phoneme, proving those sounds are distinct in the language."),
        (phon_id, "True or false: Voicing refers to whether the vocal folds vibrate during a sound.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Compare 'b' and 'p'"), "True. Voiced sounds (b, d, g) involve vocal fold vibration; voiceless sounds (p, t, k) do not."),
        (phon_id, "The sounds [p], [t], and [k] are classified as ___.", "fill_in_blank", "stops", None, None, None, None, Some("Complete closure then release"), "Stops (or plosives) are produced by completely blocking airflow and then releasing it."),
        (phon_id, "Which phonological process occurs when a sound becomes more similar to a neighboring sound?", "multiple_choice", "Assimilation", Some("Dissimilation"), Some("Assimilation"), Some("Metathesis"), Some("Epenthesis"), Some("'input' → [ɪmpʊt]"), "Assimilation: a sound changes to become more like an adjacent sound, e.g. /n/ → [m] before /p/."),
        (morph_id, "The word 'unhappiness' contains how many morphemes?", "multiple_choice", "3", Some("1"), Some("2"), Some("3"), Some("4"), Some("Break it into meaningful parts"), "'Un-' (not) + 'happy' (root) + '-ness' (state of) = 3 morphemes."),
        (morph_id, "True or false: An infix is a morpheme inserted inside a root word.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Tagalog uses these"), "True. Infixes are placed inside a root, unlike prefixes (before) or suffixes (after)."),
        (morph_id, "Creating 'brunch' from 'breakfast' and 'lunch' is an example of ___.", "fill_in_blank", "blending", None, None, None, None, Some("Merging parts of two words"), "Blending combines parts of two words into a new word: breakfast + lunch = brunch."),
        (morph_id, "Which word order does English primarily use?", "multiple_choice", "SVO", Some("SOV"), Some("VSO"), Some("SVO"), Some("OVS"), Some("She reads books"), "English uses Subject-Verb-Object order: 'She (S) reads (V) books (O)'."),
        (sem_id, "The relationship between 'rose' and 'flower' is called:", "multiple_choice", "Hyponymy", Some("Synonymy"), Some("Antonymy"), Some("Hyponymy"), Some("Meronymy"), Some("IS-A vs PART-OF"), "'Rose' IS-A type of 'flower' — this IS-A relationship is called hyponymy."),
        (sem_id, "True or false: Pragmatics studies meaning independent of context.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Context-dependent or not?"), "False. Pragmatics specifically studies how context contributes to meaning beyond the literal."),
        (sem_id, "Grice's maxim of ___ says speakers should be truthful.", "fill_in_blank", "quality", None, None, None, None, Some("One of four cooperative maxims"), "The maxim of Quality: do not say what you believe to be false or lack evidence for."),
        (sem_id, "Words like 'here', 'now', and 'I' whose meaning depends on context are called:", "multiple_choice", "Deictic expressions", Some("Presuppositions"), Some("Deictic expressions"), Some("Entailments"), Some("Implicatures"), Some("They point to context"), "Deictic expressions (deixis) are words whose reference depends on the context of utterance."),
        (fam_id, "Which is the largest language family by number of speakers?", "multiple_choice", "Indo-European", Some("Sino-Tibetan"), Some("Indo-European"), Some("Afro-Asiatic"), Some("Niger-Congo"), Some("~3.2 billion speakers"), "Indo-European has ~3.2 billion speakers, including English, Spanish, Hindi, and Russian."),
        (fam_id, "True or false: Basque is classified as a language isolate.", "true_false", "true", Some("true"), Some("false"), None, None, Some("No known relatives"), "True. Basque has no proven genetic relationship to any other language — it's a language isolate."),
        (fam_id, "The systematic shift where PIE *p became Germanic f is known as ___ Law.", "fill_in_blank", "Grimm's", None, None, None, None, Some("Named after a famous fairy tale collector"), "Grimm's Law describes the systematic consonant shift from Proto-Indo-European to Germanic languages."),
        (fam_id, "When 'meat' changed meaning from 'any food' to 'animal flesh', this is an example of:", "multiple_choice", "Narrowing", Some("Broadening"), Some("Narrowing"), Some("Amelioration"), Some("Pejoration"), Some("The meaning got more specific"), "Semantic narrowing: the meaning becomes more specific over time."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    // Ordering questions
    conn.execute(
        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
         VALUES (?1, 'Order from smallest to largest unit of language:', 'ordering', 'Phoneme,Morpheme,Word,Phrase', 'Word', 'Phoneme', 'Phrase', 'Morpheme', 'Sound → meaning → standalone → combination', 'Phoneme (single sound) → Morpheme (smallest meaningful unit) → Word (free-standing) → Phrase (group of words).')",
        [phon_id],
    )?;

    // Learning paths
    let paths = [
        ("linguistics fundamentals", 1, phon_id, "Phonetics & phonology — the sounds of language"),
        ("linguistics fundamentals", 2, morph_id, "Morphology & syntax — word and sentence structure"),
        ("linguistics fundamentals", 3, sem_id, "Semantics & pragmatics — meaning and context"),
        ("linguistics fundamentals", 4, fam_id, "Language families & change — how languages evolve"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_probability(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add a Probability topic to Mathematics
    let math_id: i64 = match conn.query_row("SELECT id FROM subjects WHERE name = 'Mathematics'", [], |r| r.get(0)) {
        Ok(id) => id,
        Err(_) => return Ok(()),
    };

    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM topics WHERE subject_id = ?1 AND name = 'Probability'",
        [math_id], |r| r.get(0),
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Probability', 'intermediate', 6)",
        [math_id],
    )?;
    let prob_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Probability'", [math_id], |r| r.get(0))?;

    let lessons: &[LessonRow] = &[
        (prob_id, "Introduction to Probability", "Probability measures how likely an event is to occur, on a scale from 0 (impossible) to 1 (certain).\n\nBasic formula: P(event) = favorable outcomes / total outcomes\n\nExample: Rolling a 6 on a fair die → P = 1/6 ≈ 0.167\n\nKey terminology:\n- Experiment: a process with uncertain outcomes (flipping a coin).\n- Sample space (S): all possible outcomes. For a coin: S = {Heads, Tails}.\n- Event: a subset of the sample space (rolling an even number: {2, 4, 6}).\n\nTypes of probability:\n- Theoretical: based on reasoning (fair coin = 50/50).\n- Experimental: based on observed data (flipping a coin 1000 times).\n- Subjective: based on personal judgment ('I think there's a 70% chance of rain').\n\nComplement rule: P(not A) = 1 - P(A)\n- If P(rain) = 0.3, then P(no rain) = 0.7.\n\nProbability of 0 = impossible. Probability of 1 = certain. Most events are somewhere in between.", 1),
        (prob_id, "Combining Probabilities", "When dealing with multiple events, we use addition and multiplication rules.\n\nAddition rule (OR):\n- Mutually exclusive events: P(A or B) = P(A) + P(B)\n  - P(rolling 1 or 6) = 1/6 + 1/6 = 2/6 = 1/3\n- Not mutually exclusive: P(A or B) = P(A) + P(B) - P(A and B)\n  - P(red card or king) = 26/52 + 4/52 - 2/52 = 28/52\n\nMultiplication rule (AND):\n- Independent events: P(A and B) = P(A) × P(B)\n  - P(heads AND heads) = 1/2 × 1/2 = 1/4\n- Dependent events: P(A and B) = P(A) × P(B|A)\n  - Drawing 2 aces without replacement: P = 4/52 × 3/51 = 12/2652\n\nConditional probability: P(B|A) = P(A and B) / P(A)\n- 'Probability of B given that A has occurred'\n\nBayes' Theorem: P(A|B) = P(B|A) × P(A) / P(B)\n- Lets you update beliefs based on new evidence.\n- Used in medical testing, spam filters, machine learning.", 2),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: &[ExplanationRow] = &[
        (prob_id, "probability", "Probability is a number between 0 and 1 that measures how likely something is to happen.", Some("Probability is like a weather forecast for events. Just as '30% chance of rain' tells you to maybe bring an umbrella, a probability of 0.3 tells you an event happens about 3 out of 10 times. Zero means 'pack sunscreen', one means 'bring a boat'."), Some("If you flip two coins, what is the probability of getting at least one head?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let questions: &[QuizRow] = &[
        (prob_id, "What is the probability of rolling a 3 on a fair six-sided die?", "multiple_choice", "1/6", Some("1/2"), Some("1/6"), Some("1/3"), Some("1/12"), Some("How many favorable outcomes out of total?"), "There is 1 favorable outcome (rolling 3) out of 6 total outcomes, so P = 1/6."),
        (prob_id, "True or false: The probability of any event is between 0 and 1 inclusive.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Can probability be negative?"), "True. Probability ranges from 0 (impossible) to 1 (certain)."),
        (prob_id, "P(not A) = 1 - P(A) is called the ___ rule.", "fill_in_blank", "complement", None, None, None, None, Some("What's left over"), "The complement rule: the probability of an event NOT happening equals 1 minus the probability of it happening."),
        (prob_id, "For independent events A and B, P(A and B) equals:", "multiple_choice", "P(A) × P(B)", Some("P(A) + P(B)"), Some("P(A) × P(B)"), Some("P(A) - P(B)"), Some("P(A) / P(B)"), Some("Multiply for AND"), "For independent events, the probability of both occurring is the product of their individual probabilities."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    // Ordering questions
    conn.execute(
        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
         VALUES (?1, 'Order these probabilities from least to most likely:', 'ordering', 'Rolling a 12 on two dice,Rolling a 6 on one die,Flipping heads,Drawing a red card', 'Flipping heads', 'Rolling a 6 on one die', 'Drawing a red card', 'Rolling a 12 on two dice', 'Think about the number of favorable outcomes', 'P(12 on 2 dice)=1/36, P(6)=1/6, P(heads)=1/2, P(red card)=26/52=1/2. Ordered: 1/36, 1/6, 1/2, 1/2.')",
        [prob_id],
    )?;

    // Learning path
    conn.execute(
        "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('probability mastery', 1, ?1, 'Probability basics — understanding chance and likelihood')",
        [prob_id],
    )?;

    Ok(())
}

fn seed_statistics(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Statistics & Data'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Statistics & Data', 'Collecting, analyzing, and interpreting data — the foundation of evidence-based reasoning.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Statistics & Data'",
        [],
        |r| r.get(0),
    )?;

    // Topics
    let topics = [
        ("Mean, Median & Mode", "beginner", 1),
        ("Data Visualization", "beginner", 2),
        ("Standard Deviation", "intermediate", 3),
        ("Correlation & Causation", "intermediate", 4),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let mean_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Mean, Median & Mode'",
        [subj_id], |r| r.get(0),
    )?;
    let viz_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Data Visualization'",
        [subj_id], |r| r.get(0),
    )?;
    let stddev_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Standard Deviation'",
        [subj_id], |r| r.get(0),
    )?;
    let corr_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Correlation & Causation'",
        [subj_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons = [
        (mean_id, "Measures of Central Tendency", "The **mean** (average) is the sum of all values divided by the count. The **median** is the middle value when data is sorted. The **mode** is the most frequent value.\n\nExample: For {2, 3, 3, 5, 7}:\n- Mean = (2+3+3+5+7)/5 = 4.0\n- Median = 3 (middle value)\n- Mode = 3 (appears twice)\n\nThe mean is sensitive to outliers, while the median is robust. Use the median when data is skewed (e.g., income distributions)."),
        (viz_id, "Choosing the Right Chart", "Different data types need different visualizations:\n\n- **Bar chart**: Comparing categories (e.g., favorite colors)\n- **Line graph**: Showing trends over time (e.g., temperature)\n- **Pie chart**: Parts of a whole (e.g., budget breakdown)\n- **Histogram**: Distribution of continuous data (e.g., test scores)\n- **Scatter plot**: Relationship between two variables (e.g., height vs. weight)\n\nA misleading chart can distort data — always start axes at zero for bar charts!"),
        (stddev_id, "Understanding Spread", "**Standard deviation** measures how spread out data is from the mean.\n\n- Low SD → data clustered near the mean\n- High SD → data spread out widely\n\nFormula: σ = √(Σ(x - μ)² / N)\n\nExample: Scores {90, 91, 89, 90, 90} have a tiny SD (~0.6). Scores {50, 70, 90, 100, 40} have a large SD (~22.4).\n\nThe **68-95-99.7 rule**: In a normal distribution, ~68% of data falls within 1 SD of the mean, ~95% within 2 SD, and ~99.7% within 3 SD."),
        (corr_id, "Correlation Is Not Causation", "**Correlation** measures how two variables move together. A correlation coefficient (r) ranges from -1 to +1:\n\n- r = +1: Perfect positive correlation (both increase together)\n- r = 0: No linear relationship\n- r = -1: Perfect negative correlation (one increases, other decreases)\n\n**Causation** means one variable directly causes the other. Correlation alone cannot prove causation!\n\nFamous example: Ice cream sales correlate with drowning deaths. Does ice cream cause drowning? No — both increase in summer (confounding variable)."),
    ];
    for (tid, title, content) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, 1)",
            rusqlite::params![tid, title, content],
        )?;
    }

    // Explanations
    let explanations = [
        (mean_id, "mean", "The mean is the arithmetic average — add up all values and divide by how many there are.", Some("Think of it like splitting a pizza equally: if 3 people eat 2, 4, and 6 slices, the mean is 4 slices each."), Some("When would the mean be misleading?")),
        (viz_id, "histogram", "A histogram groups continuous data into bins and shows frequency. Unlike a bar chart, the bars touch because the data is continuous.", Some("Imagine sorting all test scores into buckets of 10 points each (0-10, 10-20, ...) and stacking blocks for each score."), Some("What is the difference between a histogram and a bar chart?")),
        (stddev_id, "standard deviation", "Standard deviation tells you how far typical values are from the average. A small SD means values are clustered; a large SD means they are spread out.", Some("Think of darts on a dartboard: a small SD means all darts are near the bullseye; a large SD means they are scattered everywhere."), Some("What happens to the standard deviation if every value is the same?")),
        (corr_id, "confounding variable", "A confounding variable is a hidden factor that influences both variables being studied, creating a false impression of a direct relationship.", Some("It is like blaming the rooster for the sunrise — the rooster crows and the sun rises, but neither causes the other."), Some("Can you think of another confounding variable example?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    // Quiz questions
    #[allow(clippy::type_complexity)]
    let questions: Vec<QuizRowHint> = vec![
        (mean_id, "What is the mean of {4, 8, 6, 2, 10}?", "multiple_choice", "6", Some("4"), Some("6"), Some("8"), Some("5"), "Add them up and divide by 5.", "Sum = 30, Count = 5, Mean = 30/5 = 6."),
        (mean_id, "What is the median of {1, 3, 7, 9, 11}?", "multiple_choice", "7", Some("3"), Some("7"), Some("9"), Some("5"), "Sort the values and find the middle one.", "The sorted list is {1, 3, 7, 9, 11}. The middle (3rd) value is 7."),
        (mean_id, "The mode is the value that appears most often in a dataset.", "true_false", "true", Some("true"), Some("false"), None, None, "Think about the word 'mode' — like fashion, it is what is most popular.", "The mode is defined as the most frequently occurring value."),
        (mean_id, "The mean of {10, 20, 30} is ___.", "fill_in_blank", "20", None, None, None, None, "Add them up and divide by the count.", "(10+20+30)/3 = 60/3 = 20."),
        (viz_id, "Which chart type is best for showing change over time?", "multiple_choice", "Line graph", Some("Pie chart"), Some("Line graph"), Some("Bar chart"), Some("Scatter plot"), "Think about what connects points across a timeline.", "Line graphs connect data points in time order, making trends visible."),
        (viz_id, "A pie chart shows parts of a whole.", "true_false", "true", Some("true"), Some("false"), None, None, "Think about slicing a pizza.", "Pie charts display proportions of categories within a total."),
        (viz_id, "Which chart uses dots to show the relationship between two variables?", "multiple_choice", "Scatter plot", Some("Bar chart"), Some("Histogram"), Some("Line graph"), Some("Scatter plot"), "Each data point is a pair of values.", "Scatter plots plot individual (x, y) data points to reveal correlations."),
        (stddev_id, "If all values in a dataset are the same, the standard deviation is ___.", "fill_in_blank", "0", None, None, None, None, "If nothing deviates from the mean, the deviation is...", "When all values equal the mean, every squared difference is 0, so SD = 0."),
        (stddev_id, "In a normal distribution, approximately what percentage of data falls within 1 standard deviation of the mean?", "multiple_choice", "68%", Some("50%"), Some("68%"), Some("95%"), Some("99.7%"), "Think of the 68-95-99.7 rule.", "The empirical rule: ~68% within 1 SD, ~95% within 2 SD, ~99.7% within 3 SD."),
        (stddev_id, "A larger standard deviation means data is more spread out.", "true_false", "true", Some("true"), Some("false"), None, None, "Deviation means distance from the center.", "Higher SD = more variability around the mean."),
        (corr_id, "A correlation coefficient of -0.9 indicates:", "multiple_choice", "A strong negative relationship", Some("No relationship"), Some("A weak positive relationship"), Some("A strong negative relationship"), Some("A perfect positive relationship"), "Negative means inverse; closer to -1 means stronger.", "r = -0.9 is close to -1, indicating a strong inverse linear relationship."),
        (corr_id, "Correlation implies causation.", "true_false", "false", Some("true"), Some("false"), None, None, "This is one of the most important lessons in statistics!", "Correlation shows association, not causation. Confounding variables may explain the link."),
        (corr_id, "What is a confounding variable?", "multiple_choice", "A hidden variable affecting both measured variables", Some("A variable that stays constant"), Some("A hidden variable affecting both measured variables"), Some("The dependent variable"), Some("A variable you can control"), "Think about what could secretly influence the results.", "A confounding variable is an unmeasured third factor that distorts the apparent relationship."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    // Ordering question
    conn.execute(
        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
         VALUES (?1, 'Order these measures from most to least affected by outliers:', 'ordering', 'Mean,Mode,Median', 'Median', 'Mean', 'Mode', NULL, 'Which measure uses all values in its calculation?', 'The mean uses every value so outliers pull it; the mode only counts frequency; the median is the middle value and is fairly robust.')",
        [mean_id],
    )?;

    // Learning path
    let path_topics = [
        (mean_id, "Learn measures of central tendency — mean, median, and mode"),
        (viz_id, "Master data visualization — choosing the right chart"),
        (stddev_id, "Understand variability with standard deviation"),
        (corr_id, "Distinguish correlation from causation"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('statistics mastery', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_ethics(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Ethics'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Ethics', 'Exploring right and wrong — moral reasoning, dilemmas, and frameworks for making good decisions.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Ethics'",
        [],
        |r| r.get(0),
    )?;

    let topics = [
        ("Moral Foundations", "beginner", 1),
        ("Ethical Frameworks", "intermediate", 2),
        ("Applied Ethics", "intermediate", 3),
        ("Digital Ethics", "advanced", 4),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let moral_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Moral Foundations'",
        [subj_id], |r| r.get(0),
    )?;
    let frameworks_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Ethical Frameworks'",
        [subj_id], |r| r.get(0),
    )?;
    let applied_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Applied Ethics'",
        [subj_id], |r| r.get(0),
    )?;
    let digital_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Digital Ethics'",
        [subj_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (moral_id, "What Is Ethics?", "**Ethics** is the branch of philosophy that asks: What is right? What is wrong? And how should we decide?\n\nEthics is not just about following rules — it is about *reasoning* through difficult choices. Every day, you make ethical decisions: Should I share? Is it okay to lie to protect someone? Is it fair?\n\n**Morality** refers to your personal sense of right and wrong. **Ethics** is the systematic study of those beliefs.\n\nThree key questions in ethics:\n1. What makes an action right or wrong?\n2. What kind of person should I be?\n3. What do I owe to others?", 1),
        (frameworks_id, "Consequentialism vs Deontology", "The two most influential ethical frameworks:\n\n**Consequentialism** (especially Utilitarianism): An action is right if it produces the best overall outcome. 'The ends justify the means.'\n- Example: Lying to save someone's life might be justified if the outcome is good.\n- Key thinker: Jeremy Bentham, John Stuart Mill\n\n**Deontology**: An action is right or wrong based on rules and duties, regardless of consequences. 'Do the right thing because it IS right.'\n- Example: Lying is always wrong, even if it might save someone, because honesty is a duty.\n- Key thinker: Immanuel Kant\n\nNeither framework is perfect — real-world dilemmas often pull us in both directions.", 1),
        (applied_id, "The Trolley Problem", "The **Trolley Problem** is the most famous thought experiment in ethics:\n\nA runaway trolley is heading toward 5 people on the track. You can pull a lever to divert it to a side track, where it will kill 1 person instead. Do you pull the lever?\n\n- A **consequentialist** says: Yes, saving 5 lives is better than saving 1.\n- A **deontologist** might say: No, pulling the lever makes YOU the cause of someone's death.\n\nVariations make it harder: What if instead of pulling a lever, you had to push a large person off a bridge to stop the trolley? Most people say the lever is okay but pushing is wrong — even though the math is the same. This reveals our moral intuitions are not always consistent.", 1),
        (digital_id, "AI and Privacy Ethics", "As technology advances, new ethical questions emerge:\n\n**AI Ethics**: Should self-driving cars prioritize passengers or pedestrians? Who is responsible when an AI makes a mistake? Should AI be used in hiring decisions?\n\n**Privacy**: How much data collection is acceptable? Do companies have a duty to protect your information? Is surveillance justified for security?\n\n**Digital Rights**: Is internet access a human right? Should algorithms be transparent? Who owns your online data?\n\nThese questions do not have easy answers — but thinking through them systematically using ethical frameworks helps us make better decisions in an increasingly digital world.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (moral_id, "morality vs ethics", "Morality is your personal sense of right and wrong. Ethics is the philosophical study of moral principles — it asks WHY something is right or wrong.", Some("Morality is like knowing how to cook by instinct. Ethics is like studying culinary science — understanding the principles behind what makes food taste good."), Some("Can someone be moral without studying ethics?")),
        (frameworks_id, "utilitarianism", "Utilitarianism says the right action is the one that produces the greatest good for the greatest number of people.", Some("Imagine choosing a restaurant for a group — you pick the one that makes the most people happy, even if it is not your personal favorite."), Some("What are the limits of always choosing the greatest good?")),
        (applied_id, "trolley problem", "The trolley problem tests whether we judge actions by their outcomes (5 saved vs. 1 lost) or by whether we directly caused harm (pulling the lever makes you responsible).", Some("It is like the difference between not donating to charity (letting harm happen) and stealing from someone (causing harm) — most people feel the second is worse."), Some("Does it matter whether you cause harm directly or indirectly?")),
        (digital_id, "algorithmic bias", "Algorithmic bias occurs when AI systems reflect or amplify existing prejudices in their training data, leading to unfair outcomes for certain groups.", Some("If you teach a parrot only rude words, it will be rude — not because parrots are rude, but because of what it was taught. AI is similar."), Some("Whose responsibility is it to fix algorithmic bias?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    // Quiz questions
    #[allow(clippy::type_complexity)]
    let questions: Vec<QuizRow> = vec![
        (moral_id, "Ethics is the systematic study of:", "multiple_choice", "Right and wrong", Some("Science"), Some("Right and wrong"), Some("Mathematics"), Some("History"), Some("Think about moral reasoning."), "Ethics is the philosophical study of morality — what is right and wrong."),
        (moral_id, "Morality and ethics mean exactly the same thing.", "true_false", "false", Some("true"), Some("false"), None, None, Some("One is personal, the other is systematic."), "Morality is your personal sense of right/wrong; ethics is the philosophical study of those principles."),
        (frameworks_id, "Which framework judges actions by their outcomes?", "multiple_choice", "Consequentialism", Some("Deontology"), Some("Consequentialism"), Some("Virtue ethics"), Some("Nihilism"), Some("Think about consequences."), "Consequentialism evaluates actions based on the results they produce."),
        (frameworks_id, "Kant is associated with ___ ethics.", "fill_in_blank", "deontological", None, None, None, None, Some("It is about duties and rules."), "Immanuel Kant developed deontological ethics based on moral duties."),
        (frameworks_id, "A deontologist believes the ends always justify the means.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Deontology is about rules, not outcomes."), "Deontologists believe some actions are inherently right or wrong regardless of outcomes."),
        (applied_id, "In the classic trolley problem, how many people are on the main track?", "multiple_choice", "5", Some("1"), Some("3"), Some("5"), Some("10"), Some("It is the most common version."), "The classic trolley problem has 5 people on the main track and 1 on the side track."),
        (applied_id, "The trolley problem reveals that our moral intuitions are always consistent.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Think about the lever vs. the bridge variant."), "Most people approve pulling the lever but not pushing someone — showing inconsistency in our intuitions."),
        (digital_id, "Algorithmic bias occurs when AI systems:", "multiple_choice", "Reflect prejudices from training data", Some("Run too slowly"), Some("Reflect prejudices from training data"), Some("Use too much memory"), Some("Have syntax errors"), Some("Think about where AI learns its patterns."), "AI systems can inherit and amplify biases present in the data they were trained on."),
        (digital_id, "Who coined the term 'utilitarianism'?", "multiple_choice", "Jeremy Bentham", Some("Aristotle"), Some("Immanuel Kant"), Some("Jeremy Bentham"), Some("John Rawls"), Some("He wanted to maximize happiness."), "Jeremy Bentham developed utilitarianism, later refined by John Stuart Mill."),
        (digital_id, "Internet access is universally recognized as a human right by all countries.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Think about international consensus."), "While the UN has passed resolutions on internet access, it is not universally recognized as a human right by all countries."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path
    let path_topics = [
        (moral_id, "Understand the foundations of moral reasoning"),
        (frameworks_id, "Compare consequentialism, deontology, and virtue ethics"),
        (applied_id, "Apply ethical reasoning to classic dilemmas"),
        (digital_id, "Explore modern ethical challenges in technology"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('ethics mastery', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_world_literature(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'World Literature'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('World Literature', 'Great stories from around the globe — exploring humanity through fiction, poetry, and drama across cultures and centuries.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'World Literature'",
        [],
        |r| r.get(0),
    )?;

    let topics = [
        ("Mythology & Epic Poetry", "beginner", 1),
        ("Shakespeare & Drama", "intermediate", 2),
        ("The Novel", "intermediate", 3),
        ("Poetry & Verse", "beginner", 4),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let myth_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Mythology & Epic Poetry'",
        [subj_id], |r| r.get(0),
    )?;
    let shakespeare_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Shakespeare & Drama'",
        [subj_id], |r| r.get(0),
    )?;
    let novel_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'The Novel'",
        [subj_id], |r| r.get(0),
    )?;
    let poetry_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Poetry & Verse'",
        [subj_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (myth_id, "Epic Tales from the Ancient World", "**Epic poetry** is among the oldest forms of literature. These long narrative poems tell stories of heroes, gods, and the origins of civilizations.\n\n**The Iliad & The Odyssey** (Homer, ~8th century BCE): The Iliad tells of the Trojan War; The Odyssey follows Odysseus on his 10-year journey home.\n\n**The Epic of Gilgamesh** (~2100 BCE, Mesopotamia): The oldest known work of literature. King Gilgamesh seeks immortality after his friend Enkidu dies.\n\n**The Mahabharata** (India, ~400 BCE): One of the longest poems ever written, containing the famous Bhagavad Gita.\n\n**The Aeneid** (Virgil, 19 BCE): Rome's founding myth, following Aeneas from Troy to Italy.\n\nThese stories explore universal themes: mortality, honor, love, duty, and the meaning of being human.", 1),
        (shakespeare_id, "The Bard of Avon", "**William Shakespeare** (1564–1616) is widely regarded as the greatest writer in the English language. He wrote ~37 plays, 154 sonnets, and several longer poems.\n\nHis works fall into three categories:\n- **Comedies**: A Midsummer Night's Dream, Much Ado About Nothing, Twelfth Night\n- **Tragedies**: Hamlet, Macbeth, Othello, King Lear, Romeo and Juliet\n- **Histories**: Henry V, Richard III\n\nShakespeare invented over 1,700 words we still use today: 'lonely', 'generous', 'assassination', 'eyeball'.\n\nHis plays explore jealousy, ambition, love, betrayal, and power — themes as relevant today as in the 1600s. He wrote for everyone: groundlings (standing audience) and royalty alike.", 1),
        (novel_id, "The Rise of the Novel", "The **novel** as a literary form emerged in the 17th-18th centuries. Unlike epic poetry, novels are written in prose and typically focus on individual experience.\n\n**Key milestones**:\n- **Don Quixote** (Cervantes, 1605) — Often called the first modern novel\n- **Robinson Crusoe** (Defoe, 1719) — Pioneered realistic fiction\n- **Pride and Prejudice** (Austen, 1813) — Master of social observation\n- **Crime and Punishment** (Dostoevsky, 1866) — Psychological depth\n- **One Hundred Years of Solitude** (Márquez, 1967) — Magical realism\n- **Things Fall Apart** (Achebe, 1958) — African literature on the world stage\n\nThe novel gave voice to ordinary people and interior life in ways poetry and drama could not.", 1),
        (poetry_id, "The Power of Verse", "**Poetry** compresses language to its most powerful form. Every word matters.\n\n**Key concepts**:\n- **Meter**: The rhythmic pattern (iambic pentameter: da-DUM da-DUM da-DUM da-DUM da-DUM)\n- **Rhyme scheme**: The pattern of end rhymes (ABAB, AABB, etc.)\n- **Free verse**: Poetry without fixed meter or rhyme\n- **Imagery**: Vivid sensory language that creates pictures in the mind\n\n**Famous poets across cultures**:\n- **Rumi** (13th c. Persia): Mystical love poetry\n- **Emily Dickinson** (19th c. USA): Compact, enigmatic verses\n- **Pablo Neruda** (20th c. Chile): Passionate, political poetry\n- **Matsuo Bashō** (17th c. Japan): Master of haiku\n\nPoetry is meant to be read aloud — rhythm and sound are as important as meaning.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (myth_id, "epic poetry", "Epic poetry is a long narrative poem that tells the story of heroic deeds, often involving gods, monsters, and journeys. It uses elevated language and often begins 'in medias res' (in the middle of things).", Some("An epic poem is like a blockbuster movie franchise — grand scale, a hero's journey, special effects (supernatural elements), and a story that shapes a whole culture's identity."), Some("Why do you think so many ancient cultures independently created epic poems?")),
        (shakespeare_id, "tragedy", "In Shakespeare's tragedies, a noble character with a fatal flaw (hamartia) makes choices that lead to their downfall and death. The audience feels pity and fear (catharsis).", Some("A Shakespearean tragedy is like watching someone build an incredible tower of blocks, knowing one wrong move will topple everything — and you cannot look away."), Some("What is Hamlet's fatal flaw?")),
        (novel_id, "magical realism", "Magical realism blends realistic narrative with magical elements that characters treat as normal. It originated in Latin American literature.", Some("Imagine if your grandmother casually told you she floated up to the ceiling yesterday while making dinner, and everyone just nodded. That is magical realism."), Some("How does magical realism differ from fantasy?")),
        (poetry_id, "haiku", "Haiku is a Japanese poetry form with three lines of 5, 7, and 5 syllables. It traditionally captures a moment in nature and evokes a season.", Some("A haiku is like a photograph in words — it freezes one perfect moment. No backstory, no explanation, just the image."), Some("Can you write a haiku about the current season?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    // Quiz questions
    #[allow(clippy::type_complexity)]
    let questions: Vec<QuizRow> = vec![
        (myth_id, "What is the oldest known work of literature?", "multiple_choice", "The Epic of Gilgamesh", Some("The Iliad"), Some("The Odyssey"), Some("The Epic of Gilgamesh"), Some("The Aeneid"), Some("It comes from Mesopotamia."), "The Epic of Gilgamesh (~2100 BCE) from ancient Mesopotamia is the oldest known literary work."),
        (myth_id, "Homer wrote The Iliad and The Odyssey.", "true_false", "true", Some("true"), Some("false"), None, None, Some("He is the most famous ancient Greek poet."), "Homer is traditionally credited with both The Iliad and The Odyssey."),
        (myth_id, "The Odyssey follows ___ on his journey home.", "fill_in_blank", "Odysseus", None, None, None, None, Some("The poem is named after him."), "The Odyssey follows the hero Odysseus on his 10-year journey home from Troy."),
        (shakespeare_id, "How many plays did Shakespeare write approximately?", "multiple_choice", "37", Some("12"), Some("25"), Some("37"), Some("52"), Some("More than 30, fewer than 40."), "Shakespeare is credited with approximately 37 plays."),
        (shakespeare_id, "Which of these is a Shakespeare comedy?", "multiple_choice", "A Midsummer Night's Dream", Some("Hamlet"), Some("A Midsummer Night's Dream"), Some("Macbeth"), Some("King Lear"), Some("Think fairies and love potions."), "A Midsummer Night's Dream is one of Shakespeare's most beloved comedies."),
        (shakespeare_id, "Shakespeare invented the word 'eyeball'.", "true_false", "true", Some("true"), Some("false"), None, None, Some("He coined over 1,700 words."), "Shakespeare first used the word 'eyeball' in A Midsummer Night's Dream."),
        (novel_id, "Which book is often called the first modern novel?", "multiple_choice", "Don Quixote", Some("Robinson Crusoe"), Some("Don Quixote"), Some("Pride and Prejudice"), Some("Gulliver's Travels"), Some("It was written by Cervantes in 1605."), "Don Quixote by Miguel de Cervantes (1605) is widely considered the first modern novel."),
        (novel_id, "Who wrote Things Fall Apart?", "multiple_choice", "Chinua Achebe", Some("Wole Soyinka"), Some("Chinua Achebe"), Some("Ngũgĩ wa Thiong'o"), Some("Chimamanda Adichie"), Some("A Nigerian author, published in 1958."), "Chinua Achebe wrote Things Fall Apart, a landmark of African literature."),
        (novel_id, "Magical realism originated in ___ American literature.", "fill_in_blank", "Latin", None, None, None, None, Some("Think Márquez and Borges."), "Magical realism is most associated with Latin American literature."),
        (poetry_id, "How many syllables are in the middle line of a haiku?", "multiple_choice", "7", Some("5"), Some("7"), Some("9"), Some("3"), Some("The pattern is 5-?-5."), "A haiku has three lines with 5, 7, and 5 syllables."),
        (poetry_id, "Iambic pentameter has ___ metrical feet per line.", "fill_in_blank", "5", None, None, None, None, Some("Penta- means five."), "Iambic pentameter has 5 iambs (da-DUM) per line, totaling 10 syllables."),
        (poetry_id, "Free verse poetry has no fixed meter or rhyme.", "true_false", "true", Some("true"), Some("false"), None, None, Some("The name says it all."), "Free verse abandons traditional rules of meter and rhyme for more natural expression."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    // Ordering question
    conn.execute(
        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
         VALUES (?1, 'Order these literary works from oldest to newest:', 'ordering', 'The Epic of Gilgamesh,The Iliad,The Aeneid,Don Quixote', 'The Aeneid', 'Don Quixote', 'The Iliad', 'The Epic of Gilgamesh', 'The oldest is from Mesopotamia.', 'Gilgamesh (~2100 BCE), Iliad (~8th c. BCE), Aeneid (19 BCE), Don Quixote (1605 CE).')",
        [myth_id],
    )?;

    // Learning path
    let path_topics = [
        (myth_id, "Explore the origins of storytelling through epic poetry"),
        (poetry_id, "Understand the craft of poetry — meter, rhyme, and imagery"),
        (shakespeare_id, "Study Shakespeare's plays and dramatic techniques"),
        (novel_id, "Trace the rise of the novel across cultures"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('world literature mastery', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_trigonometry(conn: &Connection) -> Result<(), rusqlite::Error> {
    let math_id: i64 = match conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Mathematics'",
        [],
        |r| r.get(0),
    ) {
        Ok(id) => id,
        Err(_) => return Ok(()),
    };

    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM topics WHERE subject_id = ?1 AND name = 'Trigonometry'",
            [math_id],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Trigonometry', 'advanced', 6)",
        [math_id],
    )?;
    let trig_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Trigonometry'",
        [math_id], |r| r.get(0),
    )?;

    conn.execute(
        "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, 'Introduction to Trigonometry', \
        'Trigonometry studies the relationships between the sides and angles of triangles.\n\n\
        The three primary trig functions for a right triangle:\n\
        - **sin(θ)** = opposite / hypotenuse\n\
        - **cos(θ)** = adjacent / hypotenuse\n\
        - **tan(θ)** = opposite / adjacent = sin(θ)/cos(θ)\n\n\
        **SOH-CAH-TOA** is the classic mnemonic.\n\n\
        Key angles to memorize:\n\
        | Angle | sin | cos | tan |\n\
        |-------|-----|-----|-----|\n\
        | 0°    | 0   | 1   | 0   |\n\
        | 30°   | 1/2 | √3/2 | 1/√3 |\n\
        | 45°   | √2/2 | √2/2 | 1  |\n\
        | 60°   | √3/2 | 1/2 | √3  |\n\
        | 90°   | 1   | 0   | undefined |\n\n\
        The **unit circle** extends trig to all angles, not just those in right triangles.', 1)",
        [trig_id],
    )?;

    conn.execute(
        "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, \
        'SOH-CAH-TOA', \
        'A mnemonic for the three basic trig ratios: Sine = Opposite/Hypotenuse, Cosine = Adjacent/Hypotenuse, Tangent = Opposite/Adjacent.', \
        'Think of SOH-CAH-TOA as three recipes: each trig function is a different ratio of the same triangle ingredients.', \
        'If sin(θ) = 0.5, what angle is θ?')",
        [trig_id],
    )?;

    #[allow(clippy::type_complexity)]
    let questions: Vec<QuizRowHint> = vec![
        (trig_id, "What is sin(30°)?", "multiple_choice", "0.5", Some("0"), Some("0.5"), Some("1"), Some("√3/2"), "It is one of the key angles to memorize.", "sin(30°) = 1/2 = 0.5"),
        (trig_id, "SOH-CAH-TOA is a mnemonic for:", "multiple_choice", "Trig ratios", Some("Algebraic identities"), Some("Trig ratios"), Some("Calculus rules"), Some("Geometry theorems"), "It helps you remember sine, cosine, and tangent.", "SOH-CAH-TOA stands for Sin=Opp/Hyp, Cos=Adj/Hyp, Tan=Opp/Adj."),
        (trig_id, "tan(θ) = sin(θ) / cos(θ)", "true_false", "true", Some("true"), Some("false"), None, None, "Think about the definitions.", "By definition, tan(θ) = opposite/adjacent = (opp/hyp)/(adj/hyp) = sin(θ)/cos(θ)."),
        (trig_id, "cos(0°) = ___", "fill_in_blank", "1", None, None, None, None, "At 0° the adjacent side equals the hypotenuse.", "cos(0°) = adjacent/hypotenuse = 1/1 = 1."),
        (trig_id, "What is tan(45°)?", "multiple_choice", "1", Some("0"), Some("0.5"), Some("1"), Some("undefined"), "At 45° the opposite and adjacent sides are equal.", "tan(45°) = opposite/adjacent = 1/1 = 1."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    Ok(())
}

pub fn seed_political_science(conn: &Connection) -> Result<(), rusqlite::Error> {
    let sub_id: i64 = conn.query_row(
        "INSERT INTO subjects (name, description) VALUES ('Political Science', 'The study of governments, political processes, and power structures — how societies organize and govern themselves.') RETURNING id",
        [],
        |r| r.get(0),
    )?;

    // Topics
    let gov_sys: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Government Systems', 'beginner', 1) RETURNING id",
        [sub_id], |r| r.get(0),
    )?;
    let intl_rel: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'International Relations', 'intermediate', 2) RETURNING id",
        [sub_id], |r| r.get(0),
    )?;
    let pol_phil: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Political Philosophy', 'intermediate', 3) RETURNING id",
        [sub_id], |r| r.get(0),
    )?;
    let human_r: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Human Rights', 'beginner', 4) RETURNING id",
        [sub_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (gov_sys, "Types of Government", "Government systems organize political power.\n\nDemocracy: citizens vote to elect leaders or decide policy directly.\n  - Direct democracy: citizens vote on laws themselves (e.g., Swiss cantons).\n  - Representative democracy: citizens elect representatives (e.g., USA, Germany).\n  - Parliamentary: legislature picks the head of government (e.g., UK, Canada).\n  - Presidential: separate executive elected by the people (e.g., USA, Brazil).\n\nMonarchy: power held by a king or queen.\n  - Absolute: monarch rules with total authority (historical France, Saudi Arabia).\n  - Constitutional: monarch is head of state but power rests with parliament (UK, Japan).\n\nAuthoritarian: power concentrated in a small group; limited political freedom.\n  Examples: military juntas, single-party states.\n\nTheocracy: religious leaders govern; laws based on religious doctrine.\n  Examples: Vatican City, historical Tibet.\n\nFederalism: power shared between central and regional governments.\n  Examples: USA, Switzerland, Germany.\n\nKey insight: most modern states blend elements of multiple systems.", 1),
        (gov_sys, "Separation of Powers", "Most democracies divide government into three branches:\n\n1. Legislative: makes laws (parliament, congress).\n2. Executive: enforces laws (president, prime minister, cabinet).\n3. Judicial: interprets laws (courts, supreme court).\n\nChecks and balances prevent any branch from becoming too powerful.\n  - The legislature can impeach the executive.\n  - The judiciary can strike down unconstitutional laws.\n  - The executive can veto legislation.\n\nMontesquieu (1748) first articulated this idea in 'The Spirit of the Laws.'\n\nNot all democracies separate powers equally:\n  - USA: strong separation between branches.\n  - UK: parliament is supreme; PM comes from the legislature.\n  - Switzerland: collective executive (Federal Council) elected by parliament.", 2),
        (intl_rel, "International Organizations", "Major international organizations:\n\nUnited Nations (UN): founded 1945; 193 member states.\n  - General Assembly: all members, one vote each.\n  - Security Council: 5 permanent members with veto power (USA, UK, France, Russia, China).\n  - Agencies: WHO, UNESCO, UNICEF, UNHCR.\n\nEuropean Union (EU): 27 member states; common market, shared currency (euro).\n  - Parliament, Commission, Council.\n  - Free movement of people, goods, services, capital.\n\nNATO: military alliance (1949); collective defense — attack on one is attack on all.\n\nWorld Trade Organization (WTO): regulates international trade; resolves disputes.\n\nInternational Criminal Court (ICC): prosecutes genocide, war crimes, crimes against humanity.\n\nKey tension: national sovereignty vs. international cooperation.", 1),
        (intl_rel, "Diplomacy and Conflict", "Diplomacy: the art of managing relations between nations.\n\nKey concepts:\n- Bilateral: between two countries.\n- Multilateral: among many countries.\n- Soft power: influence through culture, values, and institutions (vs. military force).\n- Hard power: coercion through military or economic force.\n- Smart power: combining soft and hard power strategically.\n\nConflict resolution tools:\n1. Negotiation: direct talks between parties.\n2. Mediation: neutral third party facilitates agreement.\n3. Arbitration: third party makes a binding decision.\n4. Sanctions: economic penalties to pressure behavior change.\n5. Peacekeeping: international forces monitor ceasefires.\n\nJust War Theory: war is justifiable only if:\n  - Last resort (all peaceful options exhausted).\n  - Just cause (e.g., self-defense).\n  - Proportional response.\n  - Reasonable chance of success.", 2),
        (pol_phil, "Social Contract Theory", "Social contract theory asks: why do people accept government authority?\n\nThomas Hobbes (1651, Leviathan):\n  - Without government, life is 'solitary, poor, nasty, brutish, and short.'\n  - People surrender freedom to a strong sovereign for security.\n  - Favored absolute authority.\n\nJohn Locke (1689, Two Treatises):\n  - People have natural rights: life, liberty, property.\n  - Government exists to protect these rights.\n  - If it fails, people may revolt.\n  - Foundation of liberal democracy.\n\nJean-Jacques Rousseau (1762, The Social Contract):\n  - 'Man is born free, and everywhere he is in chains.'\n  - Legitimate authority comes from the 'general will' of the people.\n  - Emphasized popular sovereignty and equality.\n\nModern impact: these ideas shaped the American and French Revolutions,\nand remain central to debates about government legitimacy.", 1),
        (pol_phil, "Justice and Equality", "Political philosophy grapples with what a just society looks like.\n\nJohn Rawls (1971, A Theory of Justice):\n  - The 'veil of ignorance': design society without knowing your place in it.\n  - Two principles: (1) equal basic liberties for all, (2) inequalities only if they benefit the least advantaged.\n  - Influenced modern welfare state thinking.\n\nRobert Nozick (1974, Anarchy, State, and Utopia):\n  - Minimal state: government should only protect against force and fraud.\n  - Holdings are just if acquired fairly (liberty-based).\n  - Redistribution violates individual rights.\n\nAmartya Sen (1999, Development as Freedom):\n  - Justice = expanding real freedoms (capabilities).\n  - Poverty is unfreedom — lacking capability to live well.\n  - Focus on what people can actually do and be.\n\nMartha Nussbaum:\n  - Capabilities approach: 10 central capabilities every government should guarantee.\n  - Includes life, health, education, political participation, emotional well-being.", 2),
        (human_r, "Universal Declaration of Human Rights", "The UDHR was adopted by the UN General Assembly on December 10, 1948.\n\n30 articles covering fundamental rights:\n\nCivil & Political Rights:\n  - Right to life, liberty, security (Art. 3).\n  - Freedom from slavery (Art. 4) and torture (Art. 5).\n  - Right to a fair trial (Art. 10).\n  - Freedom of thought, conscience, religion (Art. 18).\n  - Freedom of opinion and expression (Art. 19).\n  - Right to peaceful assembly (Art. 20).\n\nEconomic, Social & Cultural Rights:\n  - Right to work and fair wages (Art. 23).\n  - Right to education (Art. 26).\n  - Right to participate in cultural life (Art. 27).\n\nKey principle: rights are universal, inalienable, and indivisible.\n  Universal: apply to every person regardless of nationality.\n  Inalienable: cannot be taken away.\n  Indivisible: civil rights and economic rights are equally important.\n\nThe UDHR is not legally binding, but it inspired:\n  - International Covenant on Civil and Political Rights (ICCPR).\n  - International Covenant on Economic, Social and Cultural Rights (ICESCR).\n  - Many national constitutions.", 1),
        (human_r, "Human Rights Challenges", "Despite legal frameworks, human rights face ongoing challenges:\n\nCurrent issues:\n  - Freedom of expression vs. hate speech regulation.\n  - Digital privacy and surveillance.\n  - Refugee rights and asylum.\n  - Labor rights in global supply chains.\n  - Indigenous peoples' rights to land and self-determination.\n  - Gender equality and LGBTQ+ rights.\n\nEnforcement gaps:\n  - Sovereignty: states resist external scrutiny.\n  - Power dynamics: powerful nations rarely face consequences.\n  - Cultural relativism: are rights truly universal or culturally shaped?\n\nKey institutions:\n  - UN Human Rights Council: reviews member states.\n  - International Criminal Court: prosecutes worst violations.\n  - NGOs (Amnesty International, Human Rights Watch): document and advocate.\n\nDebate: positive vs. negative rights.\n  - Negative rights: freedom FROM interference (e.g., no torture).\n  - Positive rights: entitlement TO something (e.g., education, healthcare).\n  - Which should governments prioritize?", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1,?2,?3,?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    #[allow(clippy::type_complexity)]
    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (gov_sys, "Democracy", "Democracy means 'rule by the people.' Citizens participate in decision-making, either directly (voting on laws) or by electing representatives. Key features include free elections, rule of law, and protection of minority rights.", Some("Think of democracy like a group project where everyone gets a vote on what to do — majority wins, but good groups also listen to dissenting voices."), Some("Why might a majority-rule democracy still need protections for minorities?")),
        (gov_sys, "Separation of Powers", "Dividing government into legislative, executive, and judicial branches prevents any one group from gaining too much power. Each branch can check the others.", Some("Like a three-legged stool — remove one leg and the whole thing topples. Each branch keeps the others balanced."), Some("What happens when one branch becomes much stronger than the others?")),
        (intl_rel, "Sovereignty", "Sovereignty means a state has supreme authority within its borders and is independent from external control. It's the foundation of the international system since the Peace of Westphalia (1648).", Some("Sovereignty is like a fence around your property — what you do inside is your business, but your neighbors might complain if your actions affect them."), Some("When, if ever, should the international community override a nation's sovereignty?")),
        (pol_phil, "Social Contract", "The idea that people agree (implicitly) to give up some freedoms in exchange for social order and protection. Different philosophers (Hobbes, Locke, Rousseau) envisioned different versions.", Some("Imagine you and friends are stranded on an island — you'd naturally agree on some rules to keep everyone safe. That's the social contract."), Some("Did you ever explicitly agree to your government's rules? Does that matter?")),
        (human_r, "Universal Human Rights", "The idea that every person has inherent rights simply by being human — regardless of nationality, ethnicity, gender, or status. Codified in the 1948 Universal Declaration of Human Rights.", Some("Human rights are like a minimum specification for dignity — the baseline that no government or person should violate."), Some("Are human rights truly universal, or do they reflect specific cultural values?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    // Quiz questions
    #[allow(clippy::type_complexity)]
    let questions: Vec<QuizRowHint> = vec![
        (gov_sys, "In which system does the legislature choose the head of government?", "multiple_choice", "Parliamentary", Some("Presidential"), Some("Parliamentary"), Some("Theocratic"), Some("Monarchical"), "Think about the UK and Canada.", "In a parliamentary system, the prime minister is chosen by (and accountable to) the legislature."),
        (gov_sys, "True or false: In a constitutional monarchy, the monarch holds absolute power.", "true_false", "false", Some("true"), Some("false"), None, None, "Think about the UK's queen/king.", "False. In a constitutional monarchy, the monarch's powers are limited by law; real power lies with elected officials."),
        (gov_sys, "Montesquieu proposed the separation of powers in which work?", "fill_in_blank", "The Spirit of the Laws", None, None, None, None, "Published in 1748.", "Montesquieu outlined the separation of powers in 'The Spirit of the Laws' (1748)."),
        (gov_sys, "Which country is an example of a federal system?", "multiple_choice", "Switzerland", Some("France"), Some("Switzerland"), Some("Japan"), Some("United Kingdom"), "This country has 26 cantons.", "Switzerland is a federal state with 26 cantons sharing power with the central government."),
        (gov_sys, "The three branches of government are legislative, executive, and ___.", "fill_in_blank", "judicial", None, None, None, None, "This branch interprets laws.", "The judicial branch interprets laws and ensures they are applied fairly."),
        (intl_rel, "How many permanent members does the UN Security Council have?", "multiple_choice", "5", Some("3"), Some("5"), Some("10"), Some("15"), "They have veto power.", "The Security Council has 5 permanent members: USA, UK, France, Russia, and China."),
        (intl_rel, "True or false: NATO's principle of collective defense means an attack on one member is considered an attack on all.", "true_false", "true", Some("true"), Some("false"), None, None, "This is Article 5.", "True. NATO's Article 5 establishes collective defense — an attack on one ally is an attack on all."),
        (intl_rel, "What is 'soft power'?", "multiple_choice", "Influence through culture, values, and institutions", Some("Military force"), Some("Economic sanctions"), Some("Influence through culture, values, and institutions"), Some("Espionage"), "Think of cultural exports, education, and diplomacy.", "Soft power is the ability to influence others through attraction (culture, values, policies) rather than coercion."),
        (intl_rel, "The WTO primarily regulates:", "multiple_choice", "International trade", Some("Military alliances"), Some("International trade"), Some("Human rights"), Some("Space exploration"), "Think about tariffs and trade disputes.", "The World Trade Organization (WTO) regulates international trade and resolves trade disputes."),
        (pol_phil, "Who wrote 'Leviathan' (1651)?", "multiple_choice", "Thomas Hobbes", Some("John Locke"), Some("Thomas Hobbes"), Some("Jean-Jacques Rousseau"), Some("Niccolò Machiavelli"), "He described life without government as 'nasty, brutish, and short.'", "Thomas Hobbes wrote Leviathan, arguing for a strong sovereign to prevent the chaos of the 'state of nature.'"),
        (pol_phil, "Rawls's 'veil of ignorance' asks you to design society without knowing:", "multiple_choice", "Your place in it", Some("The laws"), Some("Your place in it"), Some("The geography"), Some("The technology"), "Imagine not knowing if you'll be rich or poor.", "Behind the veil of ignorance, you don't know your race, wealth, gender, or talents — leading to fairer principles."),
        (pol_phil, "True or false: Nozick supported a strong welfare state.", "true_false", "false", Some("true"), Some("false"), None, None, "He favored a minimal state.", "False. Nozick argued for a minimal state that only protects against force and fraud; redistribution violates rights."),
        (pol_phil, "Which philosopher said 'Man is born free, and everywhere he is in chains'?", "multiple_choice", "Rousseau", Some("Hobbes"), Some("Locke"), Some("Rousseau"), Some("Marx"), "He wrote 'The Social Contract.'", "Jean-Jacques Rousseau opened The Social Contract (1762) with this famous line about natural freedom vs. social constraints."),
        (human_r, "When was the Universal Declaration of Human Rights adopted?", "multiple_choice", "1948", Some("1919"), Some("1945"), Some("1948"), Some("1966"), "It was after World War II.", "The UDHR was adopted by the UN General Assembly on December 10, 1948."),
        (human_r, "Freedom from torture is found in Article ___ of the UDHR.", "fill_in_blank", "5", None, None, None, None, "It's one of the first articles.", "Article 5 of the UDHR states: 'No one shall be subjected to torture or to cruel, inhuman or degrading treatment.'"),
        (human_r, "True or false: The UDHR is legally binding on all UN member states.", "true_false", "false", Some("true"), Some("false"), None, None, "It's a declaration, not a treaty.", "False. The UDHR is a declaration, not a treaty — it's not directly legally binding, though it has inspired binding covenants."),
        (human_r, "Negative rights protect freedom FROM interference; positive rights provide entitlement TO ___.", "fill_in_blank", "something", None, None, None, None, "Think of education or healthcare.", "Positive rights entitle people to something (e.g., education, healthcare) that the state must actively provide."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    // Learning paths
    let paths: Vec<(&str, i64, i64, &str)> = vec![
        ("political science", 1, gov_sys, "Learn the major types of government and how power is organized"),
        ("political science", 2, human_r, "Understand fundamental human rights and their legal framework"),
        ("political science", 3, intl_rel, "Explore how nations interact through diplomacy and organizations"),
        ("political science", 4, pol_phil, "Dive into the philosophical foundations of politics and justice"),
    ];
    for (goal, step, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, step, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_renaissance(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add a new History topic: Renaissance & Reformation
    let history_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'History'", [], |r| r.get(0),
    )?;
    let ren_id: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Renaissance & Reformation', 'intermediate', 4) RETURNING id",
        [history_id], |r| r.get(0),
    )?;

    // Lessons
    conn.execute(
        "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, 'The Renaissance', ?2, 1)",
        rusqlite::params![ren_id, "The Renaissance (14th–17th century) was a cultural rebirth centered in Italy.\n\nKey ideas:\n  - Humanism: focus on human potential, reason, and classical learning.\n  - Individualism: celebrating personal achievement.\n  - Secularism: growing interest in the material world alongside faith.\n\nMajor figures:\n  - Leonardo da Vinci: painter, inventor, scientist (Mona Lisa, Vitruvian Man).\n  - Michelangelo: sculptor and painter (David, Sistine Chapel ceiling).\n  - Raphael: painter (School of Athens).\n  - Niccolò Machiavelli: political philosopher (The Prince).\n  - Galileo Galilei: astronomer, championed heliocentrism.\n\nFlorence was the epicenter, funded by the Medici banking family.\n\nThe printing press (Gutenberg, ~1440) accelerated the spread of ideas.\n\nThe Renaissance spread north to the Netherlands, Germany, France, and England:\n  - Erasmus (Netherlands): Christian humanism.\n  - Albrecht Dürer (Germany): Northern Renaissance art.\n  - William Shakespeare (England): drama and poetry."],
    )?;
    conn.execute(
        "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, 'The Reformation', ?2, 2)",
        rusqlite::params![ren_id, "The Reformation (16th century) transformed Christianity in Europe.\n\nMartin Luther (1517):\n  - Posted 95 Theses criticizing the Catholic Church, especially the sale of indulgences.\n  - Key ideas: salvation by faith alone (sola fide), scripture as sole authority (sola scriptura).\n  - Excommunicated in 1521; protected by German princes.\n  - His ideas spread rapidly thanks to the printing press.\n\nJohn Calvin (Geneva):\n  - Predestination: God has already chosen who is saved.\n  - Strict moral code; Geneva became a 'Protestant Rome.'\n  - Calvinism spread to France (Huguenots), Netherlands, Scotland (Presbyterianism).\n\nHenry VIII (England):\n  - Broke from Rome over his divorce request.\n  - Created the Church of England (Anglican Church).\n  - Political as much as theological.\n\nCounter-Reformation:\n  - The Catholic Church responded with the Council of Trent (1545–1563).\n  - Clarified doctrine, reformed abuses, established seminaries.\n  - Jesuits (Society of Jesus) led education and missionary work.\n\nConsequences:\n  - Religious wars across Europe (Thirty Years' War, 1618–1648).\n  - Peace of Westphalia (1648): established state sovereignty.\n  - Permanent split: Catholic south, Protestant north in Europe."],
    )?;

    // Explanations
    conn.execute(
        "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, 'Renaissance Humanism', 'Humanism was the intellectual movement at the heart of the Renaissance. It emphasized the study of classical Greek and Roman texts, human potential, and the value of reason. Unlike medieval thinking that focused on the afterlife, humanists celebrated life in the present.', 'If medieval thinking was like staring at the sky waiting for heaven, humanism was like looking around and saying — the world right here is pretty amazing too, let''s study it.', 'How did the rediscovery of classical texts change European thought?')",
        [ren_id],
    )?;
    conn.execute(
        "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, 'The Reformation', 'The Reformation was a religious revolution that challenged the Catholic Church''s authority. Starting with Martin Luther''s 95 Theses in 1517, it questioned practices like selling indulgences and argued that individuals could connect with God directly through scripture.', 'Imagine a giant corporation (the Church) controlled all access to a product (salvation). The Reformation was like open-source — anyone could read the source code (Bible) themselves.', 'Why was the printing press so crucial to the Reformation''s success?')",
        [ren_id],
    )?;

    // Quiz questions
    #[allow(clippy::type_complexity)]
    let questions: Vec<(&str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
        ("In which city did the Renaissance begin?", "multiple_choice", "Florence", Some("Rome"), Some("Florence"), Some("Venice"), Some("Paris"), "Think about the Medici family.", "The Renaissance began in Florence, Italy, supported by wealthy patrons like the Medici family."),
        ("Who painted the Sistine Chapel ceiling?", "multiple_choice", "Michelangelo", Some("Leonardo da Vinci"), Some("Raphael"), Some("Michelangelo"), Some("Botticelli"), "He also sculpted David.", "Michelangelo painted the Sistine Chapel ceiling (1508–1512) for Pope Julius II."),
        ("Martin Luther posted his 95 Theses in which year?", "multiple_choice", "1517", Some("1492"), Some("1517"), Some("1534"), Some("1648"), "It was in Wittenberg, Germany.", "Martin Luther posted his 95 Theses on October 31, 1517, criticizing the sale of indulgences."),
        ("True or false: The printing press helped spread Reformation ideas.", "true_false", "true", Some("true"), Some("false"), None, None, "Gutenberg's invention changed everything.", "True. The printing press allowed Luther's writings and translated Bibles to spread rapidly across Europe."),
        ("Gutenberg invented the movable-type printing press around ___.", "fill_in_blank", "1440", None, None, None, None, "It was in the mid-15th century.", "Johannes Gutenberg developed his printing press around 1440, revolutionizing the spread of knowledge."),
        ("The Peace of Westphalia (1648) established the principle of:", "multiple_choice", "State sovereignty", Some("Papal supremacy"), Some("State sovereignty"), Some("Universal monarchy"), Some("Free trade"), "It ended the Thirty Years' War.", "The Peace of Westphalia established state sovereignty as the foundation of the international order."),
        ("Who proposed the idea of predestination?", "multiple_choice", "John Calvin", Some("Martin Luther"), Some("John Calvin"), Some("Henry VIII"), Some("Erasmus"), "He led the reformation in Geneva.", "John Calvin taught that God has predetermined who will be saved (predestination)."),
        ("The Council of Trent was part of the:", "multiple_choice", "Counter-Reformation", Some("Renaissance"), Some("Reformation"), Some("Counter-Reformation"), Some("Enlightenment"), "It was the Catholic Church's response.", "The Council of Trent (1545–1563) was central to the Counter-Reformation, reforming Catholic practices."),
    ];
    for (q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![ren_id, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path
    conn.execute(
        "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('renaissance', 1, ?1, 'Explore the cultural rebirth and religious upheaval that reshaped Europe')",
        [ren_id],
    )?;

    Ok(())
}

pub fn seed_extra_history_quizzes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add more quiz questions for existing History topics (Ancient Civilizations, World Wars, Industrial Revolution)

    // Ancient Civilizations (topic_id=12 based on seed order)
    let anc_id: i64 = conn.query_row(
        "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'History' AND t.name = 'Ancient Civilizations'",
        [], |r| r.get(0),
    )?;
    #[allow(clippy::type_complexity)]
    let anc_qs: Vec<(&str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
        ("Which ancient civilization built the pyramids at Giza?", "multiple_choice", "Egypt", Some("Mesopotamia"), Some("Egypt"), Some("Greece"), Some("Rome"), "They're on the Nile.", "The ancient Egyptians built the pyramids at Giza around 2560 BCE as tombs for pharaohs."),
        ("The Roman Republic became the Roman Empire in which century?", "multiple_choice", "1st century BCE", Some("3rd century BCE"), Some("1st century BCE"), Some("1st century CE"), Some("3rd century CE"), "Think about Julius Caesar and Augustus.", "The Roman Republic transitioned to the Roman Empire in 27 BCE when Octavian became Augustus."),
        ("True or false: Ancient Athens practiced direct democracy.", "true_false", "true", Some("true"), Some("false"), None, None, "Citizens voted on laws themselves.", "True. In Athens, eligible citizens (free adult males) voted directly on laws and policies in the Assembly."),
        ("The Indus Valley civilization was located in modern-day ___ and India.", "fill_in_blank", "Pakistan", None, None, None, None, "It's India's western neighbor.", "The Indus Valley civilization (c. 3300–1300 BCE) was centered in modern Pakistan and northwestern India."),
    ];
    for (q, qtype, correct, a, b, c, d, hint, expl) in anc_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![anc_id, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    // World Wars (topic_id=13)
    let ww_id: i64 = conn.query_row(
        "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'History' AND t.name = 'World Wars'",
        [], |r| r.get(0),
    )?;
    #[allow(clippy::type_complexity)]
    let ww_qs: Vec<(&str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
        ("Which event marked the start of World War II in Europe?", "multiple_choice", "Germany's invasion of Poland", Some("Attack on Pearl Harbor"), Some("Germany's invasion of Poland"), Some("Battle of Britain"), Some("Treaty of Versailles"), "It happened on September 1, 1939.", "Germany invaded Poland on September 1, 1939, prompting Britain and France to declare war."),
        ("D-Day (June 6, 1944) was the Allied invasion of:", "multiple_choice", "Normandy, France", Some("Berlin, Germany"), Some("Normandy, France"), Some("Sicily, Italy"), Some("London, England"), "It was a beach landing in northern France.", "D-Day was the Allied amphibious invasion of Normandy, France — the largest seaborne invasion in history."),
        ("True or false: The United States entered World War I in 1917.", "true_false", "true", Some("true"), Some("false"), None, None, "The war started in 1914 but the US joined later.", "True. The US entered WWI in April 1917, partly due to unrestricted submarine warfare and the Zimmermann Telegram."),
        ("The Treaty of ___ ended World War I.", "fill_in_blank", "Versailles", None, None, None, None, "It was signed at a French palace in 1919.", "The Treaty of Versailles (1919) officially ended WWI, imposing harsh terms on Germany."),
    ];
    for (q, qtype, correct, a, b, c, d, hint, expl) in ww_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![ww_id, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    // Industrial Revolution (topic_id=14)
    let ind_id: i64 = conn.query_row(
        "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = 'History' AND t.name = 'Industrial Revolution'",
        [], |r| r.get(0),
    )?;
    #[allow(clippy::type_complexity)]
    let ind_qs: Vec<(&str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
        ("James Watt improved which invention?", "multiple_choice", "The steam engine", Some("The spinning jenny"), Some("The steam engine"), Some("The telegraph"), Some("The locomotive"), "It powered factories and transport.", "James Watt dramatically improved the steam engine in the 1760s–70s, making it practical for industry."),
        ("True or false: The Industrial Revolution led to mass urbanization.", "true_false", "true", Some("true"), Some("false"), None, None, "People moved to cities for factory work.", "True. Millions moved from rural areas to cities for factory jobs, transforming society."),
        ("Child labor in British factories was first regulated by the Factory Act of ___.", "fill_in_blank", "1833", None, None, None, None, "It was in the early 19th century.", "The Factory Act of 1833 limited working hours for children and required factory inspections."),
    ];
    for (q, qtype, correct, a, b, c, d, hint, expl) in ind_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![ind_id, q, qtype, correct, a, b, c, d, hint, expl],
        )?;
    }

    Ok(())
}

/// Seed additional content: formal logic, first aid quiz, nutrition quiz.
pub fn seed_formal_logic_and_health(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Check if already seeded
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM lessons WHERE title = 'Propositional Logic'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    // ── Logic & Reasoning (topic that already exists, id looked up dynamically) ──
    let logic_id: i64 = conn.query_row(
        "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id \
         WHERE s.name = 'Philosophy' AND t.name = 'Logic & Reasoning'",
        [],
        |r| r.get(0),
    )?;

    // Additional lessons for Logic & Reasoning
    let logic_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (logic_id, "Propositional Logic",
         "Propositional logic deals with propositions (statements that are true or false) \
and logical connectives.\n\n\
Connectives:\n\
- AND (∧): Both must be true. \"It's raining AND cold\" → only true if both.\n\
- OR (∨): At least one must be true. \"Tea OR coffee\" → true if either.\n\
- NOT (¬): Negation. If P is true, ¬P is false.\n\
- IF...THEN (→): Implication. \"If it rains, the ground is wet.\"\n\
  Only false when premise is true but conclusion is false.\n\
- IF AND ONLY IF (↔): Biconditional. True when both sides have the same truth value.\n\n\
Truth tables: list all possible combinations of truth values.\n\
  P | Q | P ∧ Q | P ∨ Q | P → Q\n\
  T | T |   T   |   T   |   T\n\
  T | F |   F   |   T   |   F\n\
  F | T |   F   |   T   |   T\n\
  F | F |   F   |   F   |   T\n\n\
Tautology: always true regardless of values (e.g., P ∨ ¬P).\n\
Contradiction: always false (e.g., P ∧ ¬P).", 3),
        (logic_id, "Common Logical Fallacies",
         "A fallacy is an error in reasoning that makes an argument invalid.\n\n\
Formal fallacies (structural errors):\n\
- Affirming the consequent: \"If it rains, ground is wet. Ground is wet, so it rained.\" (Wrong — sprinkler!)\n\
- Denying the antecedent: \"If it rains, ground is wet. It didn't rain, so ground isn't wet.\" (Wrong.)\n\n\
Informal fallacies (content errors):\n\
- Ad hominem: attacking the person instead of the argument.\n\
- Straw man: misrepresenting someone's argument to make it easier to attack.\n\
- Appeal to authority: \"Einstein said X\" — authority doesn't guarantee truth.\n\
- False dilemma: presenting only two options when more exist.\n\
- Slippery slope: claiming one event will lead to extreme consequences without evidence.\n\
- Circular reasoning: using the conclusion as a premise (\"The Bible is true because it says so\").\n\
- Red herring: introducing an irrelevant topic to divert attention.\n\
- Tu quoque: \"You do it too\" — doesn't address the argument.\n\n\
Recognizing fallacies is essential for critical thinking and evaluating arguments.", 4),
    ];
    for (tid, title, content, order) in &logic_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Logic quiz questions
    #[allow(clippy::type_complexity)]
    let logic_qs: Vec<QuizRow> = vec![
        (logic_id, "In propositional logic, P → Q is false only when:", "multiple_choice", "P is true and Q is false",
         Some("P is true and Q is false"), Some("P is false and Q is true"), Some("Both are false"), Some("Both are true"),
         Some("The conclusion fails despite the premise holding"),
         "An implication is false only when the premise (P) is true but the conclusion (Q) is false."),
        (logic_id, "P ∨ ¬P is an example of a:", "multiple_choice", "Tautology",
         Some("Contradiction"), Some("Tautology"), Some("Contingency"), Some("Fallacy"),
         Some("It's always true regardless of P's value"),
         "P ∨ ¬P (P or not P) is always true — this is the Law of Excluded Middle, a tautology."),
        (logic_id, "Attacking the person making an argument instead of the argument itself is called:", "multiple_choice", "Ad hominem",
         Some("Straw man"), Some("Ad hominem"), Some("Red herring"), Some("Tu quoque"),
         Some("Latin for 'to the person'"),
         "Ad hominem means 'to the person' — it's a fallacy that attacks the arguer rather than the argument."),
        (logic_id, "True or false: 'If it rains, the ground is wet' is logically equivalent to 'If the ground is not wet, it did not rain'.", "true_false", "true",
         Some("true"), Some("false"), None, None,
         Some("This is called the contrapositive"),
         "True. The contrapositive (¬Q → ¬P) is always logically equivalent to the original (P → Q)."),
        (logic_id, "Presenting only two options when more exist is the ___ fallacy.", "fill_in_blank", "false dilemma",
         None, None, None, None,
         Some("Also called a false dichotomy"),
         "A false dilemma (or false dichotomy) artificially limits choices to two options when others exist."),
        (logic_id, "What logical connective does 'AND' represent?", "multiple_choice", "∧",
         Some("∨"), Some("∧"), Some("→"), Some("↔"),
         Some("Conjunction requires both to be true"),
         "The conjunction operator ∧ (AND) is true only when both propositions are true."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &logic_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, *hint, expl],
        )?;
    }

    // Logic explanations
    #[allow(clippy::type_complexity)]
    let logic_explanations: Vec<ExplanationRow> = vec![
        (logic_id, "propositional logic",
         "Propositional logic is the branch of logic that studies how to combine simple true/false statements using connectives like AND, OR, NOT, and IF-THEN.",
         Some("Think of propositional logic like electrical circuits: AND is two switches in series (both must be on), OR is two switches in parallel (either works), and NOT is an inverter that flips the signal."),
         Some("Can you build a truth table for (P ∧ Q) → R?")),
        (logic_id, "logical fallacies",
         "Logical fallacies are errors in reasoning that undermine the logic of an argument, even when the conclusion might happen to be true.",
         Some("Fallacies are like optical illusions for your brain — they look convincing at first glance, but once you learn to spot them, you see them everywhere: in politics, advertising, and everyday arguments."),
         Some("Can you identify the fallacy in: 'Everyone is buying this product, so it must be good'?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &logic_explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // ── Additional quiz questions for Health topics ──

    // First Aid (topic 17)
    let first_aid_id: i64 = conn.query_row(
        "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id \
         WHERE s.name = 'Health' AND t.name = 'First Aid Basics'",
        [],
        |r| r.get(0),
    )?;

    #[allow(clippy::type_complexity)]
    let fa_qs: Vec<QuizRow> = vec![
        (first_aid_id, "What should you do first when you find someone unconscious?", "multiple_choice", "Check for responsiveness and call for help",
         Some("Start CPR immediately"), Some("Check for responsiveness and call for help"), Some("Give them water"), Some("Move them to a bed"),
         Some("Safety first, then assess"),
         "Always check responsiveness (tap and shout), ensure the scene is safe, and call emergency services before starting any intervention."),
        (first_aid_id, "How many chest compressions per minute during CPR?", "multiple_choice", "100-120",
         Some("60-80"), Some("100-120"), Some("140-160"), Some("80-100"),
         Some("Think of the beat of 'Stayin' Alive'"),
         "The AHA recommends 100-120 compressions per minute during CPR. The song 'Stayin' Alive' has the right tempo."),
        (first_aid_id, "True or false: You should tilt a person's head back to open the airway.", "true_false", "true",
         Some("true"), Some("false"), None, None,
         Some("Head-tilt, chin-lift maneuver"),
         "True. The head-tilt chin-lift maneuver opens the airway by lifting the tongue away from the back of the throat."),
        (first_aid_id, "For a severe burn, you should apply ___ water for at least 10 minutes.", "fill_in_blank", "cool",
         None, None, None, None,
         Some("Not ice-cold, not warm"),
         "Cool (not ice-cold) running water for at least 10-20 minutes reduces burn depth and pain. Never use ice directly."),
        (first_aid_id, "The recovery position is used for someone who is:", "multiple_choice", "Unconscious but breathing",
         Some("Having a heart attack"), Some("Unconscious but breathing"), Some("Choking"), Some("Bleeding severely"),
         Some("They need to stay on their side"),
         "The recovery position keeps the airway clear for unconscious but breathing people, preventing choking on vomit or fluids."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &fa_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, *hint, expl],
        )?;
    }

    // Nutrition (topic 16)
    let nutrition_id: i64 = conn.query_row(
        "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id \
         WHERE s.name = 'Health' AND t.name = 'Nutrition'",
        [],
        |r| r.get(0),
    )?;

    #[allow(clippy::type_complexity)]
    let nut_qs: Vec<QuizRow> = vec![
        (nutrition_id, "Which vitamin is produced when skin is exposed to sunlight?", "multiple_choice", "Vitamin D",
         Some("Vitamin A"), Some("Vitamin D"), Some("Vitamin C"), Some("Vitamin K"),
         Some("The sunshine vitamin"),
         "Vitamin D is synthesized in the skin upon UVB radiation exposure. It's essential for bone health and immune function."),
        (nutrition_id, "True or false: Carbohydrates are the body's primary source of energy.", "true_false", "true",
         Some("true"), Some("false"), None, None,
         Some("Think glucose"),
         "True. Carbohydrates are broken down into glucose, the body's preferred fuel source, especially for the brain."),
        (nutrition_id, "Iron deficiency can lead to ___.", "fill_in_blank", "anemia",
         None, None, None, None,
         Some("A condition where blood can't carry enough oxygen"),
         "Iron is essential for hemoglobin in red blood cells. Deficiency causes anemia: fatigue, weakness, and pale skin."),
        (nutrition_id, "Which mineral is essential for strong bones and teeth?", "multiple_choice", "Calcium",
         Some("Iron"), Some("Calcium"), Some("Potassium"), Some("Zinc"),
         Some("Found abundantly in dairy products"),
         "Calcium is the most abundant mineral in the body and is critical for bone structure, muscle function, and nerve signaling."),
        (nutrition_id, "How many liters of water should an average adult drink daily?", "multiple_choice", "About 2-3 liters",
         Some("About 0.5 liters"), Some("About 2-3 liters"), Some("About 5-6 liters"), Some("About 1 liter"),
         Some("8 glasses is a common guideline"),
         "The general recommendation is about 2-3 liters (8-12 cups) per day, though needs vary by activity level and climate."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &nut_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, *hint, expl],
        )?;
    }

    // Hygiene (topic 15)
    let hygiene_id: i64 = conn.query_row(
        "SELECT t.id FROM topics t JOIN subjects s ON s.id = t.subject_id \
         WHERE s.name = 'Health' AND t.name = 'Hygiene'",
        [],
        |r| r.get(0),
    )?;

    #[allow(clippy::type_complexity)]
    let hyg_qs: Vec<QuizRow> = vec![
        (hygiene_id, "How long should you wash your hands with soap?", "multiple_choice", "At least 20 seconds",
         Some("5 seconds"), Some("At least 20 seconds"), Some("1 minute"), Some("10 seconds"),
         Some("Sing 'Happy Birthday' twice"),
         "The CDC recommends scrubbing hands with soap for at least 20 seconds — about the time to sing 'Happy Birthday' twice."),
        (hygiene_id, "True or false: Antibacterial soap is significantly more effective than regular soap.", "true_false", "false",
         Some("true"), Some("false"), None, None,
         Some("FDA findings on consumer antibacterial soaps"),
         "False. Studies show regular soap and water are just as effective. The FDA found no evidence antibacterial soaps are superior."),
        (hygiene_id, "You should replace your toothbrush every ___ months.", "fill_in_blank", "3",
         None, None, None, None,
         Some("Or when bristles are frayed"),
         "Dentists recommend replacing your toothbrush every 3-4 months, or sooner if the bristles are frayed."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &hyg_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, *hint, expl],
        )?;
    }

    Ok(())
}

/// Seed Anthropology subject with full content.
#[allow(clippy::type_complexity)]
pub fn seed_anthropology(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row("SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Anthropology'", [], |r| r.get(0))
        .unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Anthropology', 'The study of humanity — cultures, societies, biological evolution, and what makes us human across time and place.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Anthropology'", [], |r| r.get(0))?;

    let topics = [
        (subj_id, "Cultural Anthropology", "beginner", 1),
        (subj_id, "Biological Anthropology", "beginner", 2),
        (subj_id, "Archaeology", "intermediate", 3),
        (subj_id, "Linguistic Anthropology", "intermediate", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let cult_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Cultural Anthropology'", [subj_id], |r| r.get(0))?;
    let bio_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Biological Anthropology'", [subj_id], |r| r.get(0))?;
    let arch_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Archaeology'", [subj_id], |r| r.get(0))?;
    let ling_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Linguistic Anthropology'", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (cult_id, "What Is Culture?", "Culture is the learned set of beliefs, values, norms, and practices shared by a group of people. It includes language, art, religion, food, music, and social habits. Culture is not genetic — it is transmitted through socialization. Anthropologists study culture to understand human diversity and the many ways societies organize life.", 1),
        (cult_id, "Kinship and Social Organization", "Kinship systems define how people relate to one another through blood, marriage, and adoption. Different societies have matrilineal (traced through the mother) or patrilineal (traced through the father) descent systems. Understanding kinship helps explain inheritance, authority, and social obligations across cultures.", 2),
        (cult_id, "Rituals and Rites of Passage", "Rituals are symbolic actions performed in a prescribed order, often marking transitions in life: birth, adulthood, marriage, death. Arnold van Gennep identified three phases of rites of passage: separation, liminality (the in-between), and incorporation. These rituals reinforce social bonds and cultural identity.", 3),
        (bio_id, "Human Evolution", "Biological anthropology traces the evolution of Homo sapiens from early primates. Key milestones include bipedalism (~4 million years ago), tool use (~2.6 million years ago), and the development of language. Fossils like Lucy (Australopithecus afarensis) help reconstruct our evolutionary history.", 1),
        (bio_id, "Primatology", "Primatology studies non-human primates — monkeys, apes, and prosimians — to understand our closest relatives. Jane Goodall's work with chimpanzees revealed tool use and complex social behaviors previously thought unique to humans. Comparing primates helps us understand what traits are shared and what is uniquely human.", 2),
        (bio_id, "Human Biological Diversity", "Humans show remarkable biological variation in skin color, body proportions, and disease resistance, shaped by adaptation to different environments. For example, darker skin near the equator protects against UV radiation, while lighter skin at higher latitudes aids vitamin D production. Race is a social construct — genetic variation within so-called 'races' is greater than variation between them.", 3),
        (arch_id, "What Is Archaeology?", "Archaeology reconstructs past human societies through their material remains — tools, pottery, buildings, and trash. Unlike historians who rely on written records, archaeologists study physical evidence. Stratigraphy (layering of soil) and carbon-14 dating help establish chronologies of past cultures.", 1),
        (arch_id, "The Neolithic Revolution", "Around 10,000 BCE, humans transitioned from hunting and gathering to farming. This Neolithic Revolution began independently in several regions (Fertile Crescent, China, Mesoamerica). Agriculture led to permanent settlements, population growth, social stratification, and eventually the first cities and states.", 2),
        (ling_id, "Language and Culture", "Linguistic anthropology examines how language shapes thought and social life. The Sapir-Whorf hypothesis suggests that the structure of a language influences how its speakers perceive the world. For example, some languages have dozens of words for snow or family relationships, reflecting cultural priorities.", 1),
        (ling_id, "Language Endangerment", "Of roughly 7,000 languages spoken today, nearly half are endangered — spoken by fewer than 1,000 people. When a language dies, its unique worldview, stories, and ecological knowledge vanish. Linguists and communities work on documentation and revitalization to preserve endangered languages.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (cult_id, "Ethnocentrism", "Ethnocentrism is judging another culture solely by the standards of your own. Anthropologists practice cultural relativism instead — understanding a culture on its own terms.", Some("Imagine visiting another planet and insisting they eat with forks. Their spoons work perfectly for their food!"), Some("Can you think of a custom from another culture you initially found strange but now understand?")),
        (bio_id, "Natural Selection", "Natural selection is the process where traits that improve survival and reproduction become more common over generations. It is a key mechanism of evolution, first described by Charles Darwin.", Some("Think of it like a job interview: the environment is the employer, and beneficial traits are the qualifications."), Some("How might natural selection explain why some populations are taller than others?")),
        (arch_id, "Stratigraphy", "Stratigraphy is the study of rock and soil layers (strata). In archaeology, deeper layers are generally older. This principle lets archaeologists date finds relative to each other.", Some("It's like a stack of pancakes — the one on the bottom was made first."), Some("Why might archaeological layers sometimes be disturbed or mixed?")),
        (ling_id, "Sapir-Whorf Hypothesis", "The Sapir-Whorf hypothesis proposes that language influences thought and perception. The strong version says language determines thought; the weaker version says it merely influences it.", Some("If your language has no word for 'blue', do you still see blue? Research with the Himba people suggests color perception is indeed influenced by vocabulary."), Some("How might bilingual speakers experience the world differently?")),
    ];
    for (tid, concept, expl, analogy, followup) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // Quiz questions
    #[allow(clippy::type_complexity)]
    let quizzes: Vec<QuizRow> = vec![
        (cult_id, "What does 'culture' mean in anthropology?", "multiple_choice", "Learned beliefs, values, and practices shared by a group", Some("Only art and music"), Some("Learned beliefs, values, and practices shared by a group"), Some("Genetic traits passed down"), Some("Government laws and rules"), None, "Culture encompasses all learned and shared behaviors, not just arts or genetics."),
        (cult_id, "In a matrilineal society, descent is traced through:", "multiple_choice", "The mother's line", Some("The father's line"), Some("The mother's line"), Some("Both parents equally"), Some("The eldest sibling"), None, "Matrilineal means tracing lineage through the maternal side."),
        (cult_id, "Arnold van Gennep identified three phases of rites of passage: separation, ___, and incorporation.", "fill_in_blank", "liminality", None, None, None, None, Some("This is the 'in-between' phase"), "Liminality is the transitional phase where the person is between their old and new social status."),
        (bio_id, "Approximately when did bipedalism evolve in the human lineage?", "multiple_choice", "About 4 million years ago", Some("About 100,000 years ago"), Some("About 4 million years ago"), Some("About 50 million years ago"), Some("About 500,000 years ago"), None, "Bipedalism appeared in early hominins like Australopithecus around 4 million years ago."),
        (bio_id, "Jane Goodall is famous for studying which primates?", "multiple_choice", "Chimpanzees", Some("Gorillas"), Some("Orangutans"), Some("Chimpanzees"), Some("Bonobos"), None, "Jane Goodall spent decades studying chimpanzees in Gombe, Tanzania."),
        (bio_id, "Race is primarily a biological category.", "true_false", "false", None, None, None, None, None, "Genetic variation within racial groups exceeds variation between them; race is a social construct."),
        (arch_id, "What dating method uses the decay of carbon-14 isotopes?", "multiple_choice", "Radiocarbon dating", Some("Radiocarbon dating"), Some("Stratigraphy"), Some("Dendrochronology"), Some("Thermoluminescence"), None, "Radiocarbon (C-14) dating measures the decay of carbon-14 to estimate age up to ~50,000 years."),
        (arch_id, "The Neolithic Revolution refers to the transition from hunting-gathering to:", "fill_in_blank", "farming", None, None, None, None, Some("Think about growing your own food"), "The Neolithic Revolution (~10,000 BCE) was the shift to agriculture and settled life."),
        (ling_id, "The Sapir-Whorf hypothesis suggests that language influences:", "multiple_choice", "Thought and perception", Some("Physical health"), Some("Thought and perception"), Some("Musical ability"), Some("Athletic performance"), None, "The Sapir-Whorf hypothesis links language structure to cognitive patterns and perception."),
        (ling_id, "Approximately how many languages are spoken in the world today?", "multiple_choice", "About 7,000", Some("About 200"), Some("About 1,500"), Some("About 7,000"), Some("About 50,000"), None, "There are roughly 7,000 languages spoken worldwide, but nearly half are endangered."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Learning paths
    let paths = [
        ("anthropology", 1, cult_id, "Cultural anthropology — understand how societies organize life"),
        ("anthropology", 2, bio_id, "Biological anthropology — human evolution and diversity"),
        ("anthropology", 3, arch_id, "Archaeology — reconstructing the past through material evidence"),
        ("anthropology", 4, ling_id, "Linguistic anthropology — how language shapes human experience"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

/// Seed Nutrition Science subject with full content.
#[allow(clippy::type_complexity)]
pub fn seed_nutrition_science(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row("SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Nutrition Science'", [], |r| r.get(0))
        .unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Nutrition Science', 'The science of food and nutrients — how what we eat affects growth, health, disease prevention, and well-being.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Nutrition Science'", [], |r| r.get(0))?;

    let topics = [
        (subj_id, "Macronutrients", "beginner", 1),
        (subj_id, "Micronutrients", "beginner", 2),
        (subj_id, "Digestion & Metabolism", "intermediate", 3),
        (subj_id, "Dietary Patterns & Health", "intermediate", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let macro_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Macronutrients'", [subj_id], |r| r.get(0))?;
    let micro_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Micronutrients'", [subj_id], |r| r.get(0))?;
    let digest_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Digestion & Metabolism'", [subj_id], |r| r.get(0))?;
    let diet_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Dietary Patterns & Health'", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (macro_id, "Carbohydrates", "Carbohydrates are the body's primary energy source. They break down into glucose, which fuels cells. Simple carbs (sugars) provide quick energy, while complex carbs (starches, fiber) provide sustained energy. Fiber, found in whole grains, fruits, and vegetables, aids digestion and feeds beneficial gut bacteria. Recommended intake: 45–65% of total calories.", 1),
        (macro_id, "Proteins", "Proteins are chains of amino acids essential for building and repairing tissues, making enzymes and hormones, and supporting immune function. There are 20 amino acids; 9 are 'essential' (must come from food). Complete proteins (meat, eggs, soy) contain all essential amino acids; incomplete proteins (beans, grains) should be combined. Recommended intake: 10–35% of total calories.", 2),
        (macro_id, "Fats", "Dietary fats are essential for absorbing fat-soluble vitamins (A, D, E, K), insulating organs, and producing hormones. Unsaturated fats (olive oil, nuts, fish) promote heart health. Saturated fats (butter, red meat) should be limited. Trans fats (partially hydrogenated oils) are harmful and should be avoided. Recommended intake: 20–35% of total calories.", 3),
        (micro_id, "Vitamins", "Vitamins are organic compounds needed in small amounts for metabolism. Water-soluble vitamins (B-complex, C) must be consumed regularly since the body cannot store them. Fat-soluble vitamins (A, D, E, K) are stored in body fat. Deficiencies cause specific diseases: scurvy (vitamin C), rickets (vitamin D), night blindness (vitamin A).", 1),
        (micro_id, "Minerals", "Minerals are inorganic elements that serve structural and regulatory functions. Calcium builds bones and teeth. Iron carries oxygen in hemoglobin. Sodium and potassium regulate fluid balance and nerve signaling. Zinc supports immune function. Most minerals come from a varied diet; deficiencies can cause anemia (iron), osteoporosis (calcium), or goiter (iodine).", 2),
        (digest_id, "The Digestive Process", "Digestion begins in the mouth (mechanical chewing + salivary amylase). The stomach uses acid and pepsin to break down proteins. The small intestine is where most nutrient absorption occurs, aided by bile (from the liver) and pancreatic enzymes. The large intestine absorbs water and houses trillions of gut bacteria that produce vitamins and short-chain fatty acids.", 1),
        (digest_id, "Metabolism and Energy Balance", "Metabolism is the sum of chemical reactions that convert food into energy. Basal Metabolic Rate (BMR) — the calories burned at rest — accounts for 60-70% of daily energy use. Energy balance determines weight: calories in = calories out maintains weight; surplus leads to gain, deficit to loss. Metabolic rate is influenced by age, muscle mass, and hormones.", 2),
        (diet_id, "Mediterranean Diet", "The Mediterranean diet emphasizes fruits, vegetables, whole grains, legumes, nuts, olive oil, fish, and moderate wine. It is associated with reduced risk of heart disease, stroke, type 2 diabetes, and certain cancers. Key principles: plants first, healthy fats over saturated fats, herbs and spices instead of salt, social meals.", 1),
        (diet_id, "Understanding Food Labels", "Nutrition labels list serving size, calories, and amounts of macronutrients, sodium, fiber, and added sugars. The %Daily Value (%DV) indicates how much a nutrient contributes to a 2,000-calorie diet. 5% DV or less is 'low'; 20% or more is 'high'. Reading labels helps make informed choices about packaged foods.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (macro_id, "Glycemic Index", "The glycemic index (GI) ranks foods by how quickly they raise blood sugar. High-GI foods (white bread, candy) spike blood sugar rapidly. Low-GI foods (oats, lentils) release glucose slowly, providing sustained energy.", Some("High-GI is like a bonfire that burns hot and fast; low-GI is a slow-burning campfire that lasts all night."), Some("Why might athletes choose high-GI foods during a race but low-GI foods for breakfast?")),
        (micro_id, "Antioxidants", "Antioxidants are molecules that neutralize free radicals — unstable atoms that damage cells and contribute to aging and diseases. Vitamins C and E, beta-carotene, and selenium are powerful antioxidants found in colorful fruits and vegetables.", Some("Free radicals are like rust on metal; antioxidants are the protective coating that prevents damage."), Some("What colors of fruits and vegetables are richest in antioxidants?")),
        (digest_id, "Gut Microbiome", "The gut microbiome is the community of trillions of bacteria living in your intestines. These bacteria help digest fiber, produce vitamins (K, B12), regulate immunity, and even influence mood through the gut-brain axis.", Some("Your gut is like a garden: diverse, well-fed bacteria create a thriving ecosystem; poor diet creates weeds."), Some("How might antibiotics affect your gut microbiome?")),
        (diet_id, "Calorie Density", "Calorie density is the number of calories per gram of food. Water-rich foods (fruits, vegetables, soups) have low calorie density — you can eat more volume for fewer calories. Oils, nuts, and dried foods have high calorie density.", Some("A big bowl of salad has fewer calories than a small handful of peanuts — same energy, very different volumes."), Some("How could understanding calorie density help someone manage their weight?")),
    ];
    for (tid, concept, expl, analogy, followup) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // Quiz questions
    #[allow(clippy::type_complexity)]
    let quizzes: Vec<QuizRow> = vec![
        (macro_id, "Which macronutrient is the body's primary energy source?", "multiple_choice", "Carbohydrates", Some("Protein"), Some("Carbohydrates"), Some("Fat"), Some("Vitamins"), None, "Carbohydrates break down into glucose, the main fuel for cells."),
        (macro_id, "How many essential amino acids must come from food?", "multiple_choice", "9", Some("5"), Some("9"), Some("12"), Some("20"), None, "Of the 20 amino acids, 9 are essential — our bodies cannot synthesize them."),
        (macro_id, "Trans fats are generally considered healthy.", "true_false", "false", None, None, None, None, None, "Trans fats increase bad cholesterol and are linked to heart disease. They should be avoided."),
        (micro_id, "Scurvy is caused by a deficiency of vitamin ___.", "fill_in_blank", "C", None, None, None, None, Some("Sailors prevented it with citrus fruits"), "Vitamin C deficiency causes scurvy — bleeding gums, fatigue, and poor wound healing."),
        (micro_id, "Which mineral is essential for carrying oxygen in the blood?", "multiple_choice", "Iron", Some("Calcium"), Some("Zinc"), Some("Iron"), Some("Sodium"), None, "Iron is a key component of hemoglobin, which carries oxygen in red blood cells."),
        (micro_id, "Fat-soluble vitamins include A, D, E, and ___.", "fill_in_blank", "K", None, None, None, None, Some("It helps with blood clotting"), "Vitamins A, D, E, and K are fat-soluble — stored in the body's fat tissue."),
        (digest_id, "Where does most nutrient absorption occur?", "multiple_choice", "Small intestine", Some("Stomach"), Some("Mouth"), Some("Small intestine"), Some("Large intestine"), None, "The small intestine's large surface area (villi and microvilli) enables most nutrient absorption."),
        (digest_id, "BMR stands for Basal ___ Rate.", "fill_in_blank", "Metabolic", None, None, None, None, Some("It relates to your body's energy use at rest"), "Basal Metabolic Rate is the number of calories your body burns at rest to maintain basic functions."),
        (diet_id, "The Mediterranean diet emphasizes which type of fat?", "multiple_choice", "Olive oil (unsaturated fat)", Some("Butter"), Some("Margarine"), Some("Olive oil (unsaturated fat)"), Some("Lard"), None, "The Mediterranean diet favors unsaturated fats from olive oil, nuts, and fish."),
        (diet_id, "On a nutrition label, 20% Daily Value or more of a nutrient is considered:", "multiple_choice", "High", Some("Low"), Some("Moderate"), Some("High"), Some("Excessive"), None, "5% DV or less is low; 20% DV or more is high. This helps compare foods quickly."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Learning paths
    let paths = [
        ("nutrition science", 1, macro_id, "Macronutrients — carbs, proteins, and fats: the building blocks of diet"),
        ("nutrition science", 2, micro_id, "Micronutrients — vitamins and minerals that keep your body running"),
        ("nutrition science", 3, digest_id, "Digestion and metabolism — how your body processes food into energy"),
        ("nutrition science", 4, diet_id, "Dietary patterns — evidence-based approaches to healthy eating"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

/// Seed additional Language and Health lessons + quizzes to deepen existing thin subjects.
#[allow(clippy::type_complexity)]
pub fn seed_expanded_language_health(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add more Language topics
    let lang_id: i64 = match conn.query_row("SELECT id FROM subjects WHERE name = 'Language'", [], |r| r.get(0)) {
        Ok(id) => id,
        Err(_) => return Ok(()),
    };

    // Check if already expanded
    let topic_ct: i64 = conn.query_row(
        "SELECT COUNT(*) FROM topics WHERE subject_id = ?1",
        [lang_id], |r| r.get(0),
    )?;
    if topic_ct >= 4 { return Ok(()); }

    // Add new topics
    conn.execute(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Vocabulary Building', 'beginner', 3)",
        [lang_id],
    )?;
    conn.execute(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Essay Writing', 'intermediate', 4)",
        [lang_id],
    )?;

    let vocab_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Vocabulary Building'", [lang_id], |r| r.get(0))?;
    let essay_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Essay Writing'", [lang_id], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (vocab_id, "Context Clues", "Context clues are hints within a sentence that help you figure out the meaning of an unfamiliar word. Types include: definition clues (the word is directly defined), synonym clues (a similar word is nearby), antonym clues (an opposite word is nearby), and example clues (examples illustrate the meaning). Strong readers use context clues automatically.", 1),
        (vocab_id, "Root Words, Prefixes, and Suffixes", "Many English words are built from Greek and Latin roots. Knowing common roots unlocks thousands of words. For example: 'bio' (life), 'graph' (write), 'tele' (far), 'port' (carry). Prefixes change meaning: 'un-' (not), 'pre-' (before), 're-' (again). Suffixes change word type: '-tion' (noun), '-able' (adjective), '-ly' (adverb).", 2),
        (essay_id, "Essay Structure", "A well-structured essay has three main parts: introduction (hook + thesis statement), body paragraphs (each with a topic sentence, evidence, and analysis), and conclusion (restate thesis + broader significance). The thesis statement is the essay's central argument — every paragraph should support it.", 1),
        (essay_id, "Persuasive Writing Techniques", "Persuasive writing aims to convince the reader. Key techniques: ethos (credibility — cite experts), pathos (emotion — use vivid stories), logos (logic — present data and reasoning). A strong persuasive essay acknowledges counterarguments and refutes them, showing the writer has considered multiple perspectives.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Quizzes
    #[allow(clippy::type_complexity)]
    let quizzes: Vec<QuizRow> = vec![
        (vocab_id, "The prefix 'un-' means:", "multiple_choice", "not", Some("not"), Some("again"), Some("before"), Some("after"), None, "'Un-' means 'not' — unhappy means not happy, unclear means not clear."),
        (vocab_id, "The root 'bio' comes from Greek and means:", "fill_in_blank", "life", None, None, None, None, Some("Biology is the study of..."), "'Bio' means life — biology, biography, bioluminescence all relate to living things."),
        (essay_id, "A thesis statement belongs in which part of an essay?", "multiple_choice", "Introduction", Some("Introduction"), Some("Body"), Some("Conclusion"), Some("Bibliography"), None, "The thesis statement appears in the introduction and states the essay's central argument."),
        (essay_id, "Pathos is a persuasive technique that appeals to:", "multiple_choice", "Emotions", Some("Logic"), Some("Credibility"), Some("Emotions"), Some("Authority"), None, "Pathos appeals to the reader's emotions through vivid language and stories."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Add Health topic: Mental Health
    let health_id: i64 = match conn.query_row("SELECT id FROM subjects WHERE name = 'Health'", [], |r| r.get(0)) {
        Ok(id) => id,
        Err(_) => return Ok(()),
    };
    let health_topic_ct: i64 = conn.query_row(
        "SELECT COUNT(*) FROM topics WHERE subject_id = ?1", [health_id], |r| r.get(0),
    )?;
    if health_topic_ct >= 4 { return Ok(()); }

    conn.execute(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Mental Health', 'beginner', 4)",
        [health_id],
    )?;
    let mental_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Mental Health'", [health_id], |r| r.get(0))?;

    let mh_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (mental_id, "Understanding Stress", "Stress is the body's response to challenges or demands. Short-term stress (acute) can be helpful — it sharpens focus. Long-term stress (chronic) harms health: it weakens the immune system, disrupts sleep, and increases risk of heart disease and depression. Healthy coping strategies include exercise, deep breathing, social connection, and adequate sleep.", 1),
        (mental_id, "Emotional Well-Being", "Emotional well-being means recognizing, understanding, and managing your emotions. Everyone experiences difficult emotions — anger, sadness, anxiety. The goal is not to suppress them but to process them healthily. Techniques include journaling, talking to a trusted person, mindfulness meditation, and physical activity. Seeking professional help is a sign of strength, not weakness.", 2),
    ];
    for (tid, title, content, order) in &mh_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let mh_quizzes: Vec<QuizRow> = vec![
        (mental_id, "Chronic stress can weaken the immune system.", "true_false", "true", None, None, None, None, None, "Long-term stress raises cortisol levels, which suppresses immune function over time."),
        (mental_id, "Which of these is a healthy way to cope with stress?", "multiple_choice", "Exercise", Some("Avoiding all social contact"), Some("Exercise"), Some("Skipping meals"), Some("Staying up all night"), None, "Exercise releases endorphins and reduces stress hormones like cortisol."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &mh_quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    Ok(())
}

pub fn seed_astronomy_physics_expanded(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Astronomy expanded content — subject_id=15
    let astro_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Astronomy'", [], |r| r.get(0)
    )?;

    // New topics
    conn.execute(
        "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Stellar Evolution', 'intermediate', 5)",
        [astro_id],
    )?;
    let stellar_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Stellar Evolution'", [astro_id], |r| r.get(0)
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Exoplanets', 'intermediate', 6)",
        [astro_id],
    )?;
    let exo_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Exoplanets'", [astro_id], |r| r.get(0)
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Cosmology', 'advanced', 7)",
        [astro_id],
    )?;
    let cosmo_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Cosmology'", [astro_id], |r| r.get(0)
    )?;

    // Stellar Evolution lessons
    let stellar_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (stellar_id, "Life Cycle of Stars", "Stars form from collapsing clouds of gas and dust (nebulae). Gravity pulls material together until nuclear fusion ignites in the core — hydrogen fuses into helium.\n\nMain sequence: Stars spend most of their lives fusing hydrogen. Our Sun is a main-sequence star.\n\nMassive stars (>8 solar masses) burn hotter and faster. They last millions of years, not billions.\n\nSmall stars (red dwarfs) burn slowly and can last trillions of years.\n\nWhen hydrogen fuel runs out, a star's fate depends on its mass.", 1),
        (stellar_id, "Red Giants and Supernovae", "When a Sun-like star exhausts hydrogen, it expands into a red giant, fusing helium into carbon and oxygen. Eventually it sheds its outer layers as a planetary nebula, leaving a white dwarf.\n\nMassive stars go further — fusing carbon, neon, oxygen, silicon, and finally iron. Iron fusion absorbs energy instead of releasing it. The core collapses in seconds, triggering a supernova explosion.\n\nSupernovae create elements heavier than iron (gold, uranium) and scatter them across space. We are literally made of star stuff.", 2),
        (stellar_id, "Neutron Stars and Black Holes", "After a supernova, the remaining core determines the remnant:\n\n1.4–3 solar masses → neutron star. Incredibly dense: a teaspoon weighs ~6 billion tons. Some spin rapidly (pulsars) and emit radio beams.\n\n>3 solar masses → black hole. Gravity so strong that nothing, not even light, can escape past the event horizon.\n\nBlack holes are detected indirectly: by their gravitational effects on nearby stars, by X-rays from superheated accretion disks, and by gravitational waves when two merge.", 3),
    ];
    for (tid, title, content, order) in &stellar_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Exoplanets lessons
    let exo_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (exo_id, "Discovering Exoplanets", "Exoplanets orbit stars other than our Sun. Over 5,600 have been confirmed (as of 2024).\n\nDetection methods:\n- Transit method: Planet passes in front of its star, causing a tiny dip in brightness. Kepler/TESS missions use this.\n- Radial velocity: Star wobbles due to planet's gravity. Measured via Doppler shift in starlight.\n- Direct imaging: Actually photographing the planet (very difficult — stars are millions of times brighter).\n- Gravitational microlensing: Planet's gravity bends light from a background star.", 1),
        (exo_id, "Types of Exoplanets", "Exoplanets come in stunning variety:\n\n- Hot Jupiters: Gas giants orbiting very close to their stars. Surface temps >1000°C.\n- Super-Earths: Rocky planets 1-10× Earth's mass. Some may be habitable.\n- Mini-Neptunes: Between Earth and Neptune in size. Thick atmospheres.\n- Ocean worlds: Planets potentially covered entirely in deep water.\n- Rogue planets: Ejected from their systems, wandering through space.\n\nThe habitable zone ('Goldilocks zone') is the distance from a star where liquid water could exist on a planet's surface.", 2),
    ];
    for (tid, title, content, order) in &exo_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Cosmology lessons
    let cosmo_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (cosmo_id, "The Big Bang", "The Big Bang theory describes the origin of the universe ~13.8 billion years ago.\n\nKey evidence:\n1. Cosmic Microwave Background (CMB): Faint radiation from when the universe cooled enough for atoms to form (~380,000 years after the Big Bang).\n2. Hubble's Law: Galaxies are moving away from us — the universe is expanding. Farther galaxies recede faster.\n3. Abundance of light elements: The ratio of hydrogen to helium matches Big Bang nucleosynthesis predictions.\n\nThe Big Bang was not an explosion IN space — it was the expansion OF space itself.", 1),
        (cosmo_id, "Dark Matter and Dark Energy", "Ordinary matter (atoms) makes up only ~5% of the universe.\n\nDark matter (~27%): Does not emit or absorb light, but has gravitational effects. Evidence: galaxies rotate too fast for their visible mass; gravitational lensing shows invisible mass bending light.\n\nDark energy (~68%): A mysterious force accelerating the universe's expansion. Discovered in 1998 by observing distant supernovae that were dimmer than expected (farther away → expansion is speeding up).\n\nTogether, 95% of the universe is invisible to us. This is one of physics' greatest open problems.", 2),
    ];
    for (tid, title, content, order) in &cosmo_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Astronomy quiz questions
    let astro_quizzes: Vec<QuizRow> = vec![
        (stellar_id, "What triggers a supernova in a massive star?", "multiple_choice", "Iron core collapse", Some("Hydrogen ignition"), Some("Helium flash"), Some("Iron core collapse"), Some("Dark energy"), None, "Iron fusion absorbs energy. When the core becomes iron, fusion stops and gravity causes a catastrophic collapse."),
        (stellar_id, "A teaspoon of neutron star material weighs approximately:", "multiple_choice", "6 billion tons", Some("6 tons"), Some("6 million tons"), Some("6 billion tons"), Some("6 grams"), None, "Neutron stars are incredibly dense — a teaspoon weighs about 6 billion tons."),
        (stellar_id, "What is a pulsar?", "multiple_choice", "A rapidly spinning neutron star", Some("A type of black hole"), Some("A dying red giant"), Some("A rapidly spinning neutron star"), Some("A collapsing nebula"), None, "Pulsars are neutron stars that emit beams of radiation as they spin rapidly."),
        (stellar_id, "True or false: The Sun will eventually become a black hole.", "true_false", "false", None, None, None, None, None, "The Sun is not massive enough. It will become a red giant and then a white dwarf."),
        (exo_id, "Which method detects exoplanets by measuring tiny dips in starlight?", "multiple_choice", "Transit method", Some("Transit method"), Some("Radial velocity"), Some("Direct imaging"), Some("Spectroscopy"), None, "The transit method detects planets by the slight dimming as they pass in front of their star."),
        (exo_id, "What is the habitable zone also called?", "fill_in_blank", "Goldilocks zone", None, None, None, None, Some("Not too hot, not too cold..."), "The habitable zone is nicknamed the 'Goldilocks zone' — conditions are just right for liquid water."),
        (exo_id, "As of 2024, approximately how many exoplanets have been confirmed?", "multiple_choice", "Over 5,000", Some("About 500"), Some("Over 5,000"), Some("About 50"), Some("Over 50,000"), None, "Over 5,600 exoplanets have been confirmed as of 2024, with thousands more candidates."),
        (cosmo_id, "What percentage of the universe is ordinary matter?", "multiple_choice", "About 5%", Some("About 5%"), Some("About 27%"), Some("About 68%"), Some("About 50%"), None, "Ordinary (baryonic) matter makes up only about 5% of the universe. The rest is dark matter and dark energy."),
        (cosmo_id, "The Cosmic Microwave Background is evidence of:", "multiple_choice", "The Big Bang", Some("Dark energy"), Some("The Big Bang"), Some("Black holes"), Some("Exoplanets"), None, "The CMB is the afterglow of the Big Bang — radiation from when the universe first became transparent to light."),
        (cosmo_id, "True or false: Dark energy causes the universe's expansion to accelerate.", "true_false", "true", None, None, None, None, None, "Dark energy is the mysterious force driving the accelerating expansion of the universe, discovered in 1998."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &astro_quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Explanations
    let astro_explanations: Vec<ExplanationRow> = vec![
        (stellar_id, "Stellar Evolution", "Stars are born, live, and die — just like living things, but over millions to billions of years. Their mass at birth determines everything about their life and death.", Some("Think of stars like candles: a big candle burns brighter but runs out faster. A tiny candle burns dimly for a very long time."), Some("What happens to a star that's 20 times the mass of our Sun?")),
        (exo_id, "Exoplanet Detection", "We can't easily see exoplanets directly because stars outshine them. Instead, we detect them indirectly — like noticing a firefly near a lighthouse by watching the lighthouse flicker.", Some("Imagine trying to spot a moth flying in front of a car headlight from a mile away. The transit method works by detecting that tiny shadow."), Some("Why is the transit method better for finding large planets close to their stars?")),
        (cosmo_id, "Dark Energy", "Something is pushing the universe apart faster and faster. We don't know what it is, so we call it 'dark energy.' It makes up 68% of the universe and is perhaps the biggest mystery in physics.", Some("Imagine throwing a ball up in the air and instead of slowing down, it accelerates upward. That's what dark energy does to the expansion of space."), Some("If the universe is 95% invisible stuff, how can we be sure it exists?")),
    ];
    for (tid, concept, expl, analogy, followup) in &astro_explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // Learning paths
    conn.execute("INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('stellar evolution', 1, ?1, 'Start with the life cycle of stars — how they form and live')", [stellar_id])?;
    conn.execute("INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('stellar evolution', 2, ?1, 'Learn about stellar death — supernovae, neutron stars, black holes')", [stellar_id])?;
    conn.execute("INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('exoplanets', 1, ?1, 'Understand how we detect planets around other stars')", [exo_id])?;
    conn.execute("INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('cosmology', 1, ?1, 'Explore the origin and fate of the universe')", [cosmo_id])?;

    // Physics expanded — subject_id=16
    let physics_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Physics'", [], |r| r.get(0)
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Thermodynamics', 'intermediate', 5)",
        [physics_id],
    )?;
    let thermo_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Thermodynamics'", [physics_id], |r| r.get(0)
    )?;

    let thermo_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (thermo_id, "Laws of Thermodynamics", "The four laws of thermodynamics govern energy and heat:\n\n0th Law: If A is in thermal equilibrium with B, and B with C, then A is with C. (Defines temperature.)\n\n1st Law: Energy cannot be created or destroyed, only transformed. ΔU = Q - W (internal energy change = heat added - work done).\n\n2nd Law: Entropy of an isolated system always increases. Heat flows from hot to cold, never the reverse spontaneously. Perpetual motion machines are impossible.\n\n3rd Law: As temperature approaches absolute zero (0 K / -273.15°C), entropy approaches a minimum.", 1),
        (thermo_id, "Heat Transfer", "Heat moves by three mechanisms:\n\nConduction: Direct contact. Molecules transfer kinetic energy to neighbors. Metals are good conductors; wood and air are poor (insulators).\n\nConvection: Fluid movement. Hot fluid rises, cool fluid sinks, creating circulation. Boiling water, weather patterns, ocean currents.\n\nRadiation: Electromagnetic waves. No medium needed — this is how the Sun heats the Earth across the vacuum of space.\n\nAll three operate simultaneously in everyday situations (a campfire: radiation warms your face, convection carries smoke up, conduction heats the poker).", 2),
    ];
    for (tid, title, content, order) in &thermo_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let thermo_quizzes: Vec<QuizRow> = vec![
        (thermo_id, "Which law of thermodynamics states that energy cannot be created or destroyed?", "multiple_choice", "First Law", Some("Zeroth Law"), Some("First Law"), Some("Second Law"), Some("Third Law"), None, "The First Law of Thermodynamics is the law of conservation of energy: energy can only be transformed, not created or destroyed."),
        (thermo_id, "Heat transfer through direct contact between molecules is called:", "fill_in_blank", "conduction", None, None, None, None, Some("Think of touching a hot pan..."), "Conduction transfers heat through direct molecular contact — fast-vibrating molecules pass energy to slower neighbors."),
        (thermo_id, "True or false: Heat can spontaneously flow from a cold object to a hot object.", "true_false", "false", None, None, None, None, None, "The Second Law of Thermodynamics states that heat flows spontaneously from hot to cold, never the reverse."),
        (thermo_id, "What is absolute zero in Celsius?", "fill_in_blank", "-273.15", None, None, None, None, Some("It's the lowest possible temperature..."), "Absolute zero is 0 Kelvin, which equals -273.15°C. At this temperature, molecular motion reaches its minimum."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &thermo_quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // CS Networking topic — subject_id=6
    let cs_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Computer Science'", [], |r| r.get(0)
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Networking Basics', 'intermediate', 5)",
        [cs_id],
    )?;
    let net_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Networking Basics'", [cs_id], |r| r.get(0)
    )?;

    let net_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (net_id, "How the Internet Works", "The Internet is a global network of connected computers.\n\nKey concepts:\n- IP Address: A unique number identifying each device (e.g., 192.168.1.1 for IPv4, or longer for IPv6).\n- DNS: Translates domain names (google.com) to IP addresses. Like a phone book for the Internet.\n- Packets: Data is broken into small packets, routed independently, and reassembled at the destination.\n- Protocols: Rules for communication. HTTP for web pages, SMTP for email, FTP for files.\n\nThe TCP/IP model has 4 layers: Application, Transport, Internet, Network Access.", 1),
        (net_id, "Protocols and Ports", "Protocols define how computers communicate:\n\n- TCP (Transmission Control Protocol): Reliable, ordered delivery. Used for web, email. Establishes a connection first (three-way handshake).\n- UDP (User Datagram Protocol): Fast but unreliable. Used for video streaming, gaming, DNS.\n- HTTP/HTTPS: Web browsing (port 80/443). HTTPS adds encryption via TLS.\n- SSH: Secure remote access (port 22).\n- DNS: Name resolution (port 53).\n\nPorts are like apartment numbers in a building — the IP is the address, the port identifies the specific service.", 2),
    ];
    for (tid, title, content, order) in &net_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let net_quizzes: Vec<QuizRow> = vec![
        (net_id, "What does DNS stand for?", "fill_in_blank", "Domain Name System", None, None, None, None, Some("It translates domain names to IP addresses..."), "DNS = Domain Name System. It translates human-readable domain names into IP addresses."),
        (net_id, "Which protocol provides reliable, ordered data delivery?", "multiple_choice", "TCP", Some("UDP"), Some("TCP"), Some("FTP"), Some("DNS"), None, "TCP (Transmission Control Protocol) ensures reliable, ordered delivery using a three-way handshake and acknowledgments."),
        (net_id, "HTTPS uses port:", "multiple_choice", "443", Some("80"), Some("22"), Some("443"), Some("53"), None, "HTTPS (HTTP Secure) uses port 443. Regular HTTP uses port 80."),
        (net_id, "True or false: UDP guarantees packet delivery.", "true_false", "false", None, None, None, None, None, "UDP is connectionless and does not guarantee delivery. It's faster but less reliable than TCP."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &net_quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Geography expanded quizzes for under-covered topics
    let geo_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Geography'", [], |r| r.get(0)
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'World Capitals', 'beginner', 5)",
        [geo_id],
    )?;
    let capitals_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'World Capitals'", [geo_id], |r| r.get(0)
    )?;

    let cap_quizzes: Vec<QuizRow> = vec![
        (capitals_id, "What is the capital of Japan?", "multiple_choice", "Tokyo", Some("Osaka"), Some("Tokyo"), Some("Kyoto"), Some("Yokohama"), None, "Tokyo has been Japan's capital since 1868 when the Emperor moved there from Kyoto."),
        (capitals_id, "What is the capital of Brazil?", "multiple_choice", "Brasília", Some("Rio de Janeiro"), Some("São Paulo"), Some("Brasília"), Some("Salvador"), Some("It's not the largest city..."), "Brasília was purpose-built as the capital in 1960, designed by Oscar Niemeyer and Lúcio Costa."),
        (capitals_id, "What is the capital of Australia?", "multiple_choice", "Canberra", Some("Sydney"), Some("Melbourne"), Some("Canberra"), Some("Brisbane"), Some("It's not Sydney or Melbourne..."), "Canberra was chosen as a compromise between rival cities Sydney and Melbourne in 1908."),
        (capitals_id, "The capital of Canada is ___.", "fill_in_blank", "Ottawa", None, None, None, None, Some("It's in Ontario, but it's not Toronto..."), "Ottawa was chosen as Canada's capital by Queen Victoria in 1857."),
        (capitals_id, "What is the capital of South Africa's legislative branch?", "multiple_choice", "Cape Town", Some("Pretoria"), Some("Cape Town"), Some("Johannesburg"), Some("Durban"), Some("South Africa has three capitals..."), "South Africa has three capitals: Pretoria (executive), Cape Town (legislative), Bloemfontein (judicial)."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &cap_quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    let cap_lessons: Vec<(i64, &str, &str, i64)> = vec![
        (capitals_id, "Surprising Capitals", "Many countries' capitals are NOT their largest or most famous city:\n\n- USA: Washington D.C. (not New York)\n- Australia: Canberra (not Sydney)\n- Brazil: Brasília (not São Paulo or Rio)\n- Turkey: Ankara (not Istanbul)\n- Canada: Ottawa (not Toronto)\n- Myanmar: Naypyidaw (not Yangon)\n- Nigeria: Abuja (not Lagos)\n- Pakistan: Islamabad (not Karachi)\n\nCapitals are often chosen for political, geographic, or historical reasons rather than being the biggest city.", 1),
    ];
    for (tid, title, content, order) in &cap_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    Ok(())
}

fn seed_calculus(conn: &Connection) -> Result<(), rusqlite::Error> {
    let calc_id: i64 = conn.query_row(
        "INSERT INTO subjects (name, description) VALUES ('Calculus', 'The mathematics of change — limits, derivatives, integrals, and their applications in science and engineering.') RETURNING id",
        [], |r| r.get(0),
    )?;

    let topics = [
        (calc_id, "Limits & Continuity", "beginner", 1),
        (calc_id, "Derivatives", "intermediate", 2),
        (calc_id, "Applications of Derivatives", "intermediate", 3),
        (calc_id, "Integrals", "intermediate", 4),
        (calc_id, "Fundamental Theorem of Calculus", "advanced", 5),
    ];
    let mut topic_ids = Vec::new();
    for (sid, name, diff, order) in &topics {
        let tid: i64 = conn.query_row(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1,?2,?3,?4) RETURNING id",
            rusqlite::params![sid, name, diff, order],
            |r| r.get(0),
        )?;
        topic_ids.push(tid);
    }

    let lessons: Vec<LessonRow> = vec![
        (topic_ids[0], "What Is a Limit?", "A limit describes what value a function approaches as the input approaches some value.\n\nExample: As x approaches 2, f(x) = x² approaches 4.\nWe write: lim(x→2) x² = 4\n\nKey insight: The limit is about the JOURNEY, not the destination.\nThe function doesn't need to equal that value at the point — it's about what it gets close to.\n\nOne-sided limits:\n- Left limit: approaching from smaller values (x → 2⁻)\n- Right limit: approaching from larger values (x → 2⁺)\n- The limit exists only if both sides agree.", 1),
        (topic_ids[0], "Continuity", "A function is continuous at a point if:\n1. f(a) exists (the function is defined there)\n2. lim(x→a) f(x) exists (the limit exists)\n3. lim(x→a) f(x) = f(a) (they're equal)\n\nIf any condition fails, there's a discontinuity:\n- Removable: a hole (limit exists but ≠ f(a))\n- Jump: left and right limits differ\n- Infinite: function blows up to ±∞\n\nContinuous functions are 'smooth' — you can draw them without lifting your pen.", 2),
        (topic_ids[1], "The Derivative", "The derivative measures the instantaneous rate of change.\n\nDefinition: f'(x) = lim(h→0) [f(x+h) - f(x)] / h\n\nBasic rules:\n- Power rule: d/dx(xⁿ) = n·xⁿ⁻¹\n- Constant rule: d/dx(c) = 0\n- Sum rule: d/dx(f+g) = f' + g'\n- Product rule: d/dx(fg) = f'g + fg'\n- Chain rule: d/dx(f(g(x))) = f'(g(x))·g'(x)\n\nGeometrically, the derivative at a point is the slope of the tangent line.", 1),
        (topic_ids[2], "Optimization", "Derivatives help find maximum and minimum values.\n\nProcess:\n1. Find f'(x) and set it equal to 0\n2. These are critical points\n3. Use the second derivative test:\n   - f''(x) > 0 → local minimum (concave up)\n   - f''(x) < 0 → local maximum (concave down)\n   - f''(x) = 0 → inconclusive\n\nReal-world examples:\n- Maximizing profit\n- Minimizing material cost\n- Finding optimal dimensions", 1),
        (topic_ids[3], "Introduction to Integrals", "Integration is the reverse of differentiation.\n\nThe indefinite integral: ∫f(x)dx = F(x) + C\nwhere F'(x) = f(x) and C is the constant of integration.\n\nBasic rules:\n- ∫xⁿ dx = xⁿ⁺¹/(n+1) + C  (n ≠ -1)\n- ∫1/x dx = ln|x| + C\n- ∫eˣ dx = eˣ + C\n\nThe definite integral ∫[a,b] f(x)dx computes the signed area under the curve from a to b.", 1),
        (topic_ids[4], "The Fundamental Theorem", "The Fundamental Theorem of Calculus connects derivatives and integrals:\n\nPart 1: If F(x) = ∫[a,x] f(t)dt, then F'(x) = f(x).\n(The derivative of the integral gives back the original function.)\n\nPart 2: ∫[a,b] f(x)dx = F(b) - F(a)\n(To evaluate a definite integral, find an antiderivative and plug in the bounds.)\n\nThis is one of the most beautiful results in mathematics — it says accumulation and rate of change are inverse operations.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1,?2,?3,?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (topic_ids[0], "Limit", "A limit is the value a function approaches as the input approaches a specific number. It's about getting infinitely close, not necessarily reaching.", Some("Imagine walking toward a wall — you can always halve the remaining distance. The limit is the wall: you never touch it, but it's clearly where you're heading."), Some("Can a limit exist at a point where the function is undefined?")),
        (topic_ids[1], "Derivative as rate of change", "The derivative tells you how fast something is changing at an exact instant. If position is where you are, velocity (the derivative) is how fast you're moving right now.", Some("Your car's speedometer shows a derivative — your instantaneous speed — not average speed over the whole trip."), Some("What's the derivative of x³?")),
        (topic_ids[3], "Integral as accumulation", "An integral adds up infinitely many tiny pieces to find a total. It's the mathematical equivalent of summing all the infinitesimally thin slices of an area.", Some("Imagine measuring rainfall: the integral of the rain rate over time gives total rainfall. Each moment contributes a tiny amount, and the integral is the bucket that catches it all."), Some("What does a negative integral value mean?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRow> = vec![
        (topic_ids[0], "What is lim(x→3) of x² + 1?", "multiple_choice", "10", Some("7"), Some("9"), Some("10"), Some("12"), None, "Substitute x=3: 3² + 1 = 9 + 1 = 10. Since x² + 1 is continuous, the limit equals the function value."),
        (topic_ids[0], "A function is continuous at x=a if the limit exists and equals ___.", "fill_in_blank", "f(a)", None, None, None, None, Some("The function value at that point..."), "Continuity requires: lim(x→a) f(x) = f(a). The limit must equal the actual function value."),
        (topic_ids[0], "True or false: A limit can exist even if the function is undefined at that point.", "true_false", "true", Some("true"), Some("false"), None, None, None, "Yes! For example, lim(x→0) sin(x)/x = 1, even though sin(0)/0 is undefined (0/0)."),
        (topic_ids[1], "Using the power rule, what is the derivative of x⁵?", "multiple_choice", "5x⁴", Some("5x⁴"), Some("x⁴"), Some("5x⁵"), Some("4x⁵"), None, "Power rule: d/dx(xⁿ) = n·xⁿ⁻¹. So d/dx(x⁵) = 5·x⁴."),
        (topic_ids[1], "The derivative of a constant is ___.", "fill_in_blank", "0", None, None, None, None, Some("Constants don't change..."), "A constant has no rate of change, so its derivative is always 0."),
        (topic_ids[1], "What is the derivative of 3x² + 2x - 7?", "multiple_choice", "6x + 2", Some("6x + 2"), Some("3x + 2"), Some("6x² + 2"), Some("6x - 7"), None, "Apply the power rule term by term: d/dx(3x²) = 6x, d/dx(2x) = 2, d/dx(-7) = 0. Sum: 6x + 2."),
        (topic_ids[2], "To find maximum/minimum values, we set the ___ equal to zero.", "fill_in_blank", "derivative", None, None, None, None, Some("The rate of change at a peak or valley is..."), "At a maximum or minimum, the function is momentarily not changing — the slope (derivative) is zero."),
        (topic_ids[2], "If f''(x) > 0 at a critical point, it's a:", "multiple_choice", "local minimum", Some("local maximum"), Some("local minimum"), Some("inflection point"), Some("saddle point"), None, "f'' > 0 means concave up (like a cup), so the critical point is a local minimum."),
        (topic_ids[3], "What is ∫x² dx?", "multiple_choice", "x³/3 + C", Some("x³/3 + C"), Some("2x + C"), Some("x³ + C"), Some("3x² + C"), None, "Using the power rule for integrals: ∫xⁿ dx = xⁿ⁺¹/(n+1) + C. So ∫x² dx = x³/3 + C."),
        (topic_ids[3], "The 'C' in ∫f(x)dx = F(x) + C is called the constant of ___.", "fill_in_blank", "integration", None, None, None, None, Some("It represents any constant that disappears when you differentiate..."), "The constant of integration accounts for the fact that many functions have the same derivative (they differ by a constant)."),
        (topic_ids[4], "The Fundamental Theorem of Calculus connects which two operations?", "multiple_choice", "Differentiation and integration", Some("Addition and subtraction"), Some("Differentiation and integration"), Some("Limits and series"), Some("Algebra and geometry"), None, "The FTC shows that differentiation and integration are inverse operations — the two central ideas of calculus are fundamentally linked."),
        (topic_ids[4], "True or false: ∫[a,b] f(x)dx = F(b) - F(a) where F is any antiderivative of f.", "true_false", "true", Some("true"), Some("false"), None, None, None, "This is Part 2 of the Fundamental Theorem. You evaluate a definite integral by finding an antiderivative and computing F(b) - F(a)."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    for (i, (tid, desc)) in [
        (topic_ids[0], "Master limits and continuity — the foundation of calculus"),
        (topic_ids[1], "Learn derivatives and differentiation rules"),
        (topic_ids[2], "Apply derivatives to real-world optimization problems"),
        (topic_ids[3], "Understand integrals and the area under curves"),
        (topic_ids[4], "Connect it all with the Fundamental Theorem of Calculus"),
    ].iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('calculus', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_programming_basics(conn: &Connection) -> Result<(), rusqlite::Error> {
    let prog_id: i64 = conn.query_row(
        "INSERT INTO subjects (name, description) VALUES ('Programming', 'Learn to code — variables, control flow, functions, data structures, and algorithmic thinking.') RETURNING id",
        [], |r| r.get(0),
    )?;

    let topics = [
        (prog_id, "Variables & Data Types", "beginner", 1),
        (prog_id, "Control Flow", "beginner", 2),
        (prog_id, "Functions", "intermediate", 3),
        (prog_id, "Data Structures", "intermediate", 4),
        (prog_id, "Algorithms", "advanced", 5),
    ];
    let mut topic_ids = Vec::new();
    for (sid, name, diff, order) in &topics {
        let tid: i64 = conn.query_row(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1,?2,?3,?4) RETURNING id",
            rusqlite::params![sid, name, diff, order],
            |r| r.get(0),
        )?;
        topic_ids.push(tid);
    }

    let lessons: Vec<LessonRow> = vec![
        (topic_ids[0], "Variables", "A variable is a named container for data.\n\nThink of it as a labeled box:\n- The label is the variable name\n- The contents are the value\n- The box type is the data type\n\nCommon data types:\n- Integer (int): whole numbers like 42, -7\n- Float/Double: decimal numbers like 3.14\n- String: text like \"Hello\"\n- Boolean: true or false\n- Char: single character like 'A'\n\nExample:\n  let age = 25;        // integer\n  let pi = 3.14159;    // float\n  let name = \"Alice\";   // string\n  let active = true;   // boolean", 1),
        (topic_ids[1], "If-Else & Loops", "Control flow decides which code runs and when.\n\nConditionals (if-else):\n  if temperature > 30 {\n      print(\"It's hot!\");\n  } else if temperature > 20 {\n      print(\"Nice weather\");\n  } else {\n      print(\"It's cold\");\n  }\n\nLoops repeat code:\n- For loop: repeat a known number of times\n  for i in 1..=5 { print(i); }\n- While loop: repeat while a condition is true\n  while count > 0 { count -= 1; }\n- Break: exit a loop early\n- Continue: skip to the next iteration", 1),
        (topic_ids[2], "Defining & Calling Functions", "Functions are reusable blocks of code.\n\nWhy use functions?\n1. Avoid repeating code (DRY: Don't Repeat Yourself)\n2. Break complex problems into smaller pieces\n3. Make code readable and testable\n\nAnatomy of a function:\n  fn greet(name: &str) -> String {\n      format!(\"Hello, {}!\", name)\n  }\n\n- fn: keyword to define a function\n- greet: the function name\n- name: &str: parameter with its type\n- -> String: return type\n- The body contains the logic\n\nCalling: let message = greet(\"Alice\");", 1),
        (topic_ids[3], "Arrays, Lists & Maps", "Data structures organize collections of data.\n\nArray/Vector: ordered collection, accessed by index (0-based)\n  let fruits = [\"apple\", \"banana\", \"cherry\"];\n  fruits[0]  // \"apple\"\n\nHashMap/Dictionary: key-value pairs\n  let ages = {\"Alice\": 30, \"Bob\": 25};\n  ages[\"Alice\"]  // 30\n\nStack: Last In, First Out (LIFO) — like a stack of plates\nQueue: First In, First Out (FIFO) — like a line at a store\n\nChoosing the right data structure is a key programming skill.", 1),
        (topic_ids[4], "Sorting & Searching", "Algorithms are step-by-step procedures to solve problems.\n\nSearching:\n- Linear search: check each element one by one. O(n)\n- Binary search: divide sorted data in half each step. O(log n)\n  Much faster for sorted data!\n\nSorting:\n- Bubble sort: compare adjacent pairs, swap if needed. O(n²)\n- Merge sort: divide, sort halves, merge. O(n log n)\n- Quick sort: pick a pivot, partition. O(n log n) average\n\nBig-O notation measures efficiency:\n- O(1): constant — instant\n- O(log n): logarithmic — very fast\n- O(n): linear — proportional\n- O(n²): quadratic — slow for large data", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1,?2,?3,?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (topic_ids[0], "Variable", "A variable stores a value in memory with a name you choose. You can read it, change it, and use it in calculations.", Some("Like a whiteboard with a label — you can erase and rewrite the contents, but the label stays."), Some("What happens if you assign a new value to an existing variable?")),
        (topic_ids[2], "Function", "A function is a named, reusable block of code that takes inputs (parameters), does something, and optionally returns an output.", Some("Like a kitchen recipe: it has a name, a list of ingredients (parameters), instructions (the body), and produces a dish (the return value)."), Some("Can a function call itself?")),
        (topic_ids[4], "Big-O notation", "Big-O notation describes how an algorithm's running time or space grows as input size increases. It's about the worst-case growth rate, ignoring constants.", Some("If sorting 10 items takes 1 second, O(n²) means 100 items takes ~100 seconds — it grows with the square. O(n log n) means 100 items takes ~7 seconds."), Some("Which is faster for large data: O(n log n) or O(n²)?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRow> = vec![
        (topic_ids[0], "Which data type stores true or false?", "multiple_choice", "Boolean", Some("Integer"), Some("Boolean"), Some("String"), Some("Float"), None, "Boolean is the data type for logical values: true or false. Named after George Boole."),
        (topic_ids[0], "A variable that stores 3.14 is most likely a ___.", "fill_in_blank", "float", None, None, None, None, Some("It has a decimal point..."), "Decimal numbers are stored as floating-point (float or double) values."),
        (topic_ids[0], "True or false: Variable names can start with a number.", "true_false", "false", Some("true"), Some("false"), None, None, None, "In most programming languages, variable names must start with a letter or underscore, not a number."),
        (topic_ids[1], "Which loop runs a known number of times?", "multiple_choice", "for loop", Some("while loop"), Some("for loop"), Some("do-while loop"), Some("infinite loop"), None, "A for loop iterates over a range or collection with a predetermined count."),
        (topic_ids[1], "What keyword exits a loop early?", "fill_in_blank", "break", None, None, None, None, Some("It stops the loop immediately..."), "The 'break' keyword immediately exits the innermost loop."),
        (topic_ids[2], "What is the term for a function that calls itself?", "multiple_choice", "Recursion", Some("Iteration"), Some("Recursion"), Some("Abstraction"), Some("Encapsulation"), None, "Recursion is when a function calls itself, typically with a simpler version of the problem, until it reaches a base case."),
        (topic_ids[2], "The values passed to a function are called ___.", "fill_in_blank", "arguments", None, None, None, None, Some("Also called parameters when defining the function..."), "Arguments are the actual values passed to a function when calling it. Parameters are the names used in the function definition."),
        (topic_ids[3], "In a zero-indexed array, what index is the first element?", "multiple_choice", "0", Some("0"), Some("1"), Some("-1"), Some("none"), None, "Most programming languages use 0-based indexing: the first element is at index 0."),
        (topic_ids[3], "A HashMap stores data as ___ pairs.", "fill_in_blank", "key-value", None, None, None, None, Some("Each entry has a lookup key and associated data..."), "HashMaps (also called dictionaries) map unique keys to their associated values for fast lookup."),
        (topic_ids[4], "What is the time complexity of binary search?", "multiple_choice", "O(log n)", Some("O(n)"), Some("O(log n)"), Some("O(n²)"), Some("O(1)"), None, "Binary search halves the search space each step, giving O(log n) — searching 1 million items takes only ~20 steps."),
        (topic_ids[4], "Put these complexities in order from fastest to slowest: O(n²), O(1), O(n log n), O(n)", "ordering", "O(1),O(n),O(n log n),O(n²)", None, None, None, None, None, "O(1) is constant (fastest), then O(n) linear, O(n log n) linearithmic, O(n²) quadratic (slowest)."),
        (topic_ids[4], "True or false: Merge sort is always O(n log n), even in the worst case.", "true_false", "true", Some("true"), Some("false"), None, None, None, "Unlike quicksort which can degrade to O(n²), merge sort is always O(n log n) because it always divides evenly."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    for (i, (tid, desc)) in [
        (topic_ids[0], "Learn variables and data types — the building blocks"),
        (topic_ids[1], "Master control flow — if-else and loops"),
        (topic_ids[2], "Understand functions and code reuse"),
        (topic_ids[3], "Explore data structures for organizing information"),
        (topic_ids[4], "Study algorithms and computational complexity"),
    ].iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('programming', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

/// Add more quiz questions to subjects with sparse coverage.
fn seed_extra_math_quizzes(conn: &Connection) -> Result<(), rusqlite::Error> {
    let algebra_id: Option<i64> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Algebra Basics' LIMIT 1",
        [], |r| r.get(0),
    ).ok();

    if let Some(tid) = algebra_id {
        let existing: i64 = conn.query_row(
            "SELECT COUNT(*) FROM quiz_questions WHERE topic_id = ?1",
            [tid], |r| r.get(0),
        )?;
        if existing < 5 {
            let extra_quizzes: Vec<QuizRow> = vec![
                (tid, "Solve: 2x + 6 = 14. What is x?", "multiple_choice", "4", Some("3"), Some("4"), Some("5"), Some("8"), Some("Subtract 6 from both sides first..."), "2x + 6 = 14 → 2x = 8 → x = 4."),
                (tid, "If y = 3x - 1 and x = 5, then y = ___.", "fill_in_blank", "14", None, None, None, None, Some("Substitute x = 5..."), "y = 3(5) - 1 = 15 - 1 = 14."),
                (tid, "True or false: The equation x² = 9 has exactly one solution.", "true_false", "false", Some("true"), Some("false"), None, None, None, "x² = 9 has TWO solutions: x = 3 and x = -3. Don't forget the negative root!"),
            ];
            for (tid2, question, qtype, answer, a, b, c, d, hint, expl) in &extra_quizzes {
                conn.execute(
                    "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
                    rusqlite::params![tid2, question, qtype, answer, a, b, c, d, hint, expl],
                )?;
            }
        }
    }

    let geometry_id: Option<i64> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Geometry' LIMIT 1",
        [], |r| r.get(0),
    ).ok();

    if let Some(tid) = geometry_id {
        let existing: i64 = conn.query_row(
            "SELECT COUNT(*) FROM quiz_questions WHERE topic_id = ?1",
            [tid], |r| r.get(0),
        )?;
        if existing < 5 {
            let extra_quizzes: Vec<QuizRow> = vec![
                (tid, "What is the area of a circle with radius 5? (Use π ≈ 3.14)", "multiple_choice", "78.5", Some("31.4"), Some("78.5"), Some("25"), Some("157"), Some("Area = π × r²"), "Area = π × r² = 3.14 × 25 = 78.5 square units."),
                (tid, "A triangle with all three sides equal is called ___.", "fill_in_blank", "equilateral", None, None, None, None, Some("Equi- means equal..."), "An equilateral triangle has all three sides (and all three angles at 60°) equal."),
                (tid, "How many degrees are in the interior angles of a triangle?", "multiple_choice", "180", Some("90"), Some("180"), Some("270"), Some("360"), None, "The interior angles of any triangle always sum to exactly 180 degrees."),
            ];
            for (tid2, question, qtype, answer, a, b, c, d, hint, expl) in &extra_quizzes {
                conn.execute(
                    "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
                    rusqlite::params![tid2, question, qtype, answer, a, b, c, d, hint, expl],
                )?;
            }
        }
    }

    Ok(())
}


fn seed_creative_writing(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Expand Creative Writing with additional topics, lessons, and quizzes
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Creative Writing'", [], |r| r.get(0),
    )?;

    // Add new topics that don't already exist
    let new_topics = [
        ("World Building", "advanced", 6),
        ("Revision & Editing", "intermediate", 7),
        ("Flash Fiction", "intermediate", 8),
    ];
    for (name, diff, order) in &new_topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let world_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'World Building'", [subj_id], |r| r.get(0),
    )?;
    let revision_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Revision & Editing'", [subj_id], |r| r.get(0),
    )?;
    let flash_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Flash Fiction'", [subj_id], |r| r.get(0),
    )?;

    let lessons: &[LessonRow] = &[
        (world_id, "Building Believable Worlds", "World-building isn't just maps and magic systems. A believable world needs:\n\n1. **Internal consistency** — rules must be followed once established\n2. **Sensory details** — what does it smell, sound, taste like?\n3. **Culture and history** — how do people live, what do they believe?\n4. **Economics** — who has power and why?\n5. **The iceberg principle** — know 10x more than you show\n\nDon't info-dump your world-building. Reveal it through character experience: a character buying bread teaches us about the economy. A prayer before meals shows us religion. Let the world emerge naturally.", 1),
        (revision_id, "The Art of Revision", "First drafts are for getting ideas down. Revision is where writing becomes good.\n\n**The revision hierarchy (work top-down):**\n1. **Structure:** Does the story arc work? Cut scenes that don't serve the plot.\n2. **Character:** Are motivations clear? Do arcs feel earned?\n3. **Scene-level:** Does each scene have conflict and change?\n4. **Line-level:** Tighten prose. Kill adverbs. Vary sentence length.\n5. **Proofreading:** Grammar, spelling, consistency.\n\n**Key principle:** Let the draft rest (days, weeks) before revising. Fresh eyes catch what tired eyes miss.\n\n**Murder your darlings:** If a beautiful sentence doesn't serve the story, cut it. Save it in a scraps file if you must.", 1),
        (flash_id, "Flash Fiction: Big Stories, Small Spaces", "Flash fiction tells a complete story in under 1,000 words (some say under 500).\n\n**Keys to flash fiction:**\n- Start *in medias res* (in the middle of the action)\n- Every word must earn its place — no room for filler\n- Imply backstory rather than stating it\n- End with a twist, revelation, or emotional punch\n- One character, one conflict, one moment\n\n**Famous examples:** Hemingway's alleged six-word story: 'For sale: baby shoes, never worn.'\n\nFlash fiction is excellent practice for any writer — it forces economy and precision. If you can move a reader in 500 words, imagine what you can do in 50,000.", 1),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: &[ExplanationRow] = &[
        (world_id, "The Iceberg Principle", "Coined by Hemingway: the writer should know everything about their world but only show a fraction on the page. The depth beneath the surface gives the visible part its weight and authenticity.", Some("Like a duck gliding on a lake — calm on top, paddling furiously underneath."), Some("How do you decide what to reveal and what to keep hidden?")),
        (revision_id, "Murder Your Darlings", "The advice (often attributed to Faulkner) to cut beloved passages that don't serve the story. Just because a line is beautiful doesn't mean it belongs. Every element should advance plot, character, or theme.", Some("Like pruning a rose bush — cutting healthy branches so the remaining ones bloom more beautifully."), Some("How do you know when a passage is self-indulgent vs. essential?")),
        (flash_id, "In Medias Res", "Starting 'in the middle of things' — dropping the reader into action or conflict without preamble. Especially powerful in flash fiction where every word counts. The reader catches up through context clues.", Some("Like walking into a movie 10 minutes late — you're instantly engaged trying to figure out what's happening."), Some("When is it better to start with setup instead of action?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    let quizzes: &[QuizRow] = &[
        (world_id, "The 'iceberg principle' in world-building means:", "multiple_choice", "Know much more than you reveal", Some("Show every detail you've created"), Some("Know much more than you reveal"), Some("Focus only on geography"), Some("Always include a map"), None, "The iceberg principle (from Hemingway) means the writer should know 10x more about their world than appears on the page."),
        (world_id, "Revealing world-building through a character buying bread is an example of:", "multiple_choice", "Organic exposition", Some("Info-dumping"), Some("Organic exposition"), Some("Foreshadowing"), Some("Purple prose"), None, "Organic exposition weaves world-building into character actions and experiences instead of stopping the story to explain."),
        (revision_id, "In the revision hierarchy, what should you address FIRST?", "multiple_choice", "Story structure", Some("Grammar errors"), Some("Adverb removal"), Some("Story structure"), Some("Sentence variety"), None, "Work top-down: fix structure first (the big picture), then drill down to scenes, lines, and finally grammar."),
        (revision_id, "'Murder your darlings' means:", "multiple_choice", "Cut beloved passages that don't serve the story", Some("Write dark content"), Some("Kill off main characters"), Some("Cut beloved passages that don't serve the story"), Some("Delete your first draft"), None, "It means removing writing you love if it doesn't serve the story — beautiful but unnecessary passages should go."),
        (flash_id, "Flash fiction is typically under ___ words.", "fill_in_blank", "1000", None, None, None, None, Some("Some say 500, but the upper bound is..."), "Flash fiction is generally defined as stories under 1,000 words, though some definitions use 500 or even 300."),
        (flash_id, "Starting a story in the middle of the action is called:", "multiple_choice", "In medias res", Some("Prologue"), Some("In medias res"), Some("Exposition"), Some("Denouement"), None, "In medias res (Latin: 'in the middle of things') drops the reader into action without preamble."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    Ok(())
}

fn seed_earth_science(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add Earth Science as a new subject (geology, oceanography, soil science)
    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('Earth Science', 'Understanding our planet — geology, oceanography, soil science, and the forces that shape Earth.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Earth Science'", [], |r| r.get(0),
    )?;

    // Add new topics
    let new_topics = [
        ("Earthquakes and Volcanoes", "intermediate", 4),
        ("Oceanography", "intermediate", 5),
        ("Soil Science", "beginner", 6),
    ];
    for (name, diff, order) in &new_topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let quake_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Earthquakes and Volcanoes'", [subj_id], |r| r.get(0),
    )?;
    let ocean_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Oceanography'", [subj_id], |r| r.get(0),
    )?;
    let soil_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Soil Science'", [subj_id], |r| r.get(0),
    )?;

    let lessons: &[LessonRow] = &[
        (quake_id, "When the Earth Shakes", "**Earthquakes** occur when stress along a fault line overcomes friction, releasing energy as seismic waves.\n\n- **Focus:** The point underground where the rupture starts\n- **Epicenter:** The point on the surface directly above the focus\n- **Magnitude:** Measured on the moment magnitude scale (logarithmic — each whole number ≈ 32x more energy)\n\n**Volcanoes** form where magma reaches the surface:\n- **Shield volcanoes:** Gentle slopes, fluid lava (Hawaii)\n- **Stratovolcanoes:** Steep, explosive (Mt. St. Helens, Fuji)\n- **Cinder cones:** Small, built from ejected fragments\n\nThe Ring of Fire (Pacific Rim) hosts 75% of the world's volcanoes and 90% of earthquakes.", 1),
        (ocean_id, "The World Ocean", "Earth's ocean covers 71% of the surface and holds 97% of all water.\n\n**Ocean layers (by depth):**\n- **Epipelagic (0-200m):** Sunlit zone, most marine life\n- **Mesopelagic (200-1000m):** Twilight zone, bioluminescent creatures\n- **Bathypelagic (1000-4000m):** Midnight zone, near-freezing, crushing pressure\n- **Abyssopelagic (4000-6000m):** The abyss\n- **Hadopelagic (6000m+):** Deep trenches only\n\n**Ocean currents** are driven by wind, temperature, salinity, and Earth's rotation. The thermohaline circulation (global conveyor belt) moves warm water toward the poles and cold water toward the equator, regulating global climate.\n\nThe ocean absorbs ~30% of human CO₂ emissions — but this is causing ocean acidification.", 1),
        (soil_id, "The Ground Beneath Our Feet", "Soil is a living ecosystem, not just dirt.\n\n**Soil horizons (layers):**\n- **O horizon:** Organic matter (decomposing leaves)\n- **A horizon (topsoil):** Rich in humus, dark, where roots grow\n- **B horizon (subsoil):** Minerals washed down from above\n- **C horizon:** Weathered rock fragments\n- **R horizon (bedrock):** Solid rock below\n\n**Soil types:** Sand (large particles, drains fast), clay (tiny particles, holds water), silt (medium), loam (ideal mix).\n\nIt takes ~500 years to form 1 inch of topsoil. Erosion, deforestation, and poor farming practices can destroy it in a season. Soil contains more microorganisms in a teaspoon than there are people on Earth.", 1),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: &[ExplanationRow] = &[
        (quake_id, "Seismic Waves", "Earthquakes produce three types of waves: P-waves (fast, compressional), S-waves (slower, shearing), and surface waves (slowest but most destructive). Seismographs detect the time difference between P and S waves to locate the epicenter.", Some("Like dropping a stone in water — different ripple types spread at different speeds."), Some("Why can't S-waves travel through liquids?")),
        (ocean_id, "Thermohaline Circulation", "The global ocean 'conveyor belt' driven by differences in water temperature (thermo) and salinity (haline). Cold, salty water sinks in the North Atlantic, flows deep to the Southern Ocean, and slowly returns as warm surface current. Takes ~1,000 years for a complete cycle.", Some("Like a massive, slow conveyor belt looping through all the world's oceans, redistributing heat."), Some("What would happen if thermohaline circulation stopped?")),
        (soil_id, "Humus", "The dark, organic component of soil formed by decomposition of plant and animal matter. Humus is not the same as compost — it's the final stage of decomposition, a stable substance that improves soil structure, water retention, and nutrient availability.", Some("Humus is like a sponge and a pantry combined — it holds water and slowly releases nutrients to plant roots."), Some("Why is humus more stable than fresh compost?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    let quizzes: &[QuizRow] = &[
        (quake_id, "The point on Earth's surface directly above an earthquake's focus is the ___.", "fill_in_blank", "epicenter", None, None, None, None, Some("Epi- means 'above'..."), "The epicenter is the surface point directly above the focus (hypocenter) where the earthquake originates underground."),
        (quake_id, "What percentage of Earth's earthquakes occur in the Ring of Fire?", "multiple_choice", "90%", Some("50%"), Some("75%"), Some("90%"), Some("100%"), None, "The Ring of Fire around the Pacific Rim hosts about 90% of the world's earthquakes due to dense plate boundary activity."),
        (quake_id, "Shield volcanoes are characterized by:", "multiple_choice", "Gentle slopes and fluid lava", Some("Steep slopes and explosive eruptions"), Some("Gentle slopes and fluid lava"), Some("Only underwater eruptions"), Some("No lava, only ash"), None, "Shield volcanoes (like Hawaii) have gentle slopes built by layers of fluid basaltic lava."),
        (ocean_id, "What percentage of Earth's surface is covered by ocean?", "multiple_choice", "71%", Some("50%"), Some("60%"), Some("71%"), Some("85%"), None, "Earth's ocean covers approximately 71% of the planet's surface — that's why Earth looks blue from space."),
        (ocean_id, "The sunlit zone of the ocean (0-200m) is called the ___ zone.", "fill_in_blank", "epipelagic", None, None, None, None, Some("Epi- means 'upon' or 'above'..."), "The epipelagic (sunlit) zone extends from the surface to 200m depth and supports most marine life through photosynthesis."),
        (ocean_id, "The global ocean conveyor belt is called ___ circulation.", "fill_in_blank", "thermohaline", None, None, None, None, Some("Thermo = heat, haline = salt..."), "Thermohaline circulation is driven by differences in temperature and salinity, moving water around the globe over ~1,000 years."),
        (ocean_id, "Ocean acidification is caused by the ocean absorbing:", "multiple_choice", "Carbon dioxide", Some("Methane"), Some("Nitrogen"), Some("Carbon dioxide"), Some("Ozone"), None, "The ocean absorbs ~30% of atmospheric CO₂, which reacts with seawater to form carbonic acid, lowering pH."),
        (soil_id, "How long does it take to form approximately 1 inch of topsoil?", "multiple_choice", "500 years", Some("50 years"), Some("100 years"), Some("500 years"), Some("5 years"), None, "Topsoil formation is incredibly slow — about 500 years per inch — making soil conservation critical."),
        (soil_id, "The dark, organic component of soil is called ___.", "fill_in_blank", "humus", None, None, None, None, Some("Not the chickpea dip..."), "Humus is the stable, dark organic matter in soil formed from fully decomposed plant and animal material."),
        (soil_id, "Which soil type is the ideal mix for growing plants?", "multiple_choice", "Loam", Some("Clay"), Some("Sand"), Some("Loam"), Some("Gravel"), None, "Loam is a balanced mixture of sand, silt, and clay that provides good drainage, water retention, and nutrient availability."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    Ok(())
}

pub fn seed_data_science(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('Data Science', 'Turning data into insight — statistics, machine learning, visualization, and data-driven decision making.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Data Science'", [], |r| r.get(0),
    )?;

    let topics = [
        ("Data Wrangling", "beginner", 1),
        ("Exploratory Data Analysis", "intermediate", 2),
        ("Machine Learning Fundamentals", "intermediate", 3),
        ("Neural Networks", "advanced", 4),
        ("Data Visualization", "beginner", 5),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let wrangle_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Data Wrangling'", [subj_id], |r| r.get(0),
    )?;
    let eda_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Exploratory Data Analysis'", [subj_id], |r| r.get(0),
    )?;
    let ml_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Machine Learning Fundamentals'", [subj_id], |r| r.get(0),
    )?;
    let nn_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Neural Networks'", [subj_id], |r| r.get(0),
    )?;
    let viz_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Data Visualization'", [subj_id], |r| r.get(0),
    )?;

    let lessons: &[LessonRow] = &[
        (wrangle_id, "Cleaning Messy Data", "Real-world data is messy. **Data wrangling** (or munging) is the process of transforming raw data into a usable format.\n\n**Common issues:**\n- **Missing values:** NaN, NULL, blank cells — handle by imputation (mean/median), deletion, or flagging\n- **Duplicates:** Exact or near-duplicate rows that inflate counts\n- **Inconsistent formats:** '2024-01-15' vs 'Jan 15, 2024' vs '15/01/2024'\n- **Outliers:** Values far from the norm — real signal or data entry errors?\n\n**The 80/20 rule of data science:** ~80% of time is spent on data wrangling, ~20% on actual analysis.\n\n**Key operations:** filtering, sorting, grouping, joining (merging datasets), pivoting (reshaping), and type conversion.", 1),
        (eda_id, "Exploring Your Data", "**Exploratory Data Analysis (EDA)** is the detective work of data science — understanding what your data looks like before building models.\n\n**The EDA toolkit:**\n- **Summary statistics:** Mean, median, mode, standard deviation, quartiles\n- **Distributions:** Is it normal (bell curve)? Skewed? Bimodal?\n- **Correlations:** Do variables move together? Pearson r ranges from -1 to +1\n- **Grouping:** How do patterns differ across categories?\n\n**John Tukey** (inventor of EDA) said: 'Far better an approximate answer to the right question than an exact answer to the wrong question.'\n\n**Visual EDA tools:** histograms, box plots, scatter plots, heatmaps, pair plots.", 1),
        (ml_id, "What Is Machine Learning?", "**Machine learning** is teaching computers to learn patterns from data instead of being explicitly programmed.\n\n**Three main types:**\n1. **Supervised learning:** Learn from labeled examples (input → known output)\n   - Classification: predict categories (spam/not spam)\n   - Regression: predict numbers (house prices)\n2. **Unsupervised learning:** Find hidden patterns in unlabeled data\n   - Clustering: group similar items (customer segments)\n   - Dimensionality reduction: simplify complex data (PCA)\n3. **Reinforcement learning:** Learn by trial and error with rewards\n   - Agent takes actions in an environment to maximize cumulative reward\n\n**The ML workflow:** Collect data → Clean → Split (train/test) → Train model → Evaluate → Tune → Deploy\n\n**Overfitting** = memorizing training data (performs poorly on new data)\n**Underfitting** = too simple to capture patterns", 1),
        (nn_id, "Introduction to Neural Networks", "**Neural networks** are computing systems inspired by the brain's interconnected neurons.\n\n**Architecture:**\n- **Input layer:** Receives features (data)\n- **Hidden layers:** Process information through weighted connections\n- **Output layer:** Produces predictions\n\n**How a neuron works:**\n1. Receive inputs (x₁, x₂, ...)\n2. Multiply by weights (w₁, w₂, ...)\n3. Sum: z = Σ(wᵢ × xᵢ) + bias\n4. Apply activation function: a = f(z)\n\n**Common activation functions:**\n- **ReLU:** max(0, x) — most popular for hidden layers\n- **Sigmoid:** 1/(1+e⁻ˣ) — squashes to [0,1], good for binary output\n- **Softmax:** Converts to probability distribution (multi-class)\n\n**Training:** Backpropagation adjusts weights to minimize the loss function using gradient descent. Learning rate controls step size.", 1),
        (viz_id, "Telling Stories with Data", "**Data visualization** transforms numbers into insight through visual encoding.\n\n**Choose the right chart:**\n- **Bar chart:** Compare categories\n- **Line chart:** Show trends over time\n- **Scatter plot:** Reveal relationships between two variables\n- **Histogram:** Show distribution of a single variable\n- **Heatmap:** Display patterns in matrices (correlations)\n- **Box plot:** Summarize distribution + outliers\n- **Pie chart:** Show parts of a whole (use sparingly!)\n\n**Principles of good visualization (Edward Tufte):**\n- Maximize the data-ink ratio (remove chartjunk)\n- Show the data, not decoration\n- Don't distort or mislead\n- Label clearly, use consistent scales\n\n**Color matters:** Use sequential palettes for ordered data, diverging for +/- from a midpoint, categorical for distinct groups. ~8% of men have color vision deficiency — always test accessibility.", 1),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: &[ExplanationRow] = &[
        (wrangle_id, "Missing Value Imputation", "Replacing missing values with estimated ones. Common strategies: mean/median (numeric), mode (categorical), forward/backward fill (time series), or model-based (KNN, regression). Each has trade-offs — mean imputation reduces variance, while deletion (listwise) can introduce bias if data isn't missing completely at random.", Some("Like filling gaps in a puzzle — you estimate the missing piece from its neighbors."), Some("When would deleting rows with missing values be worse than imputing?")),
        (eda_id, "Correlation vs. Causation", "Just because two variables move together doesn't mean one causes the other. Correlation measures linear association (Pearson r). Causation requires: temporal precedence, covariation, and ruling out confounds. Famous spurious correlation: ice cream sales and drowning rates both rise in summer — but ice cream doesn't cause drowning.", Some("Two clocks showing the same time doesn't mean one controls the other — they share a common cause (the time)."), Some("How would you design an experiment to establish causation?")),
        (ml_id, "Bias-Variance Tradeoff", "A model's total error = bias² + variance + irreducible noise. High bias = underfitting (too simple). High variance = overfitting (too complex). The sweet spot minimizes both. Regularization (L1/L2) adds a penalty for complexity, reducing variance at the cost of slight bias increase.", Some("Like adjusting a telescope: too blurry (high bias) vs. too shaky (high variance). You want clear AND stable."), Some("How does increasing training data affect bias and variance?")),
        (nn_id, "Backpropagation", "The algorithm that trains neural networks by computing gradients of the loss function with respect to each weight, then updating weights to reduce the loss. Uses the chain rule of calculus to propagate error backwards from the output layer through hidden layers. Combined with gradient descent (or variants like Adam), it iteratively improves the network.", Some("Like tracing blame backwards — if the output is wrong, figure out which weights contributed most to the error, and adjust them proportionally."), Some("Why might very deep networks suffer from vanishing gradients?")),
        (viz_id, "Data-Ink Ratio", "Edward Tufte's principle: the proportion of a graphic's ink devoted to non-redundant display of data-information. Maximize this ratio by removing gridlines, borders, backgrounds, 3D effects, and other 'chartjunk' that doesn't convey information. Every visual element should earn its place.", Some("Like editing a sentence — remove every word that doesn't add meaning."), Some("Can a minimalist chart ever be TOO minimal? When?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    let quizzes: &[QuizRow] = &[
        (wrangle_id, "Approximately what percentage of a data scientist's time is spent on data wrangling?", "multiple_choice", "80%", Some("20%"), Some("50%"), Some("80%"), Some("95%"), None, "The 80/20 rule of data science: roughly 80% of time goes to data wrangling and preparation, leaving 20% for modeling and analysis."),
        (wrangle_id, "Replacing missing values with estimated ones is called ___.", "fill_in_blank", "imputation", None, None, None, None, Some("Im-put-ation, not amputation!"), "Imputation replaces missing data with substituted values — common methods include mean, median, mode, and model-based approaches."),
        (wrangle_id, "Which is NOT a common data quality issue?", "multiple_choice", "Data being too clean", Some("Missing values"), Some("Duplicate rows"), Some("Inconsistent date formats"), Some("Data being too clean"), None, "Missing values, duplicates, and inconsistent formats are all common issues. Data being 'too clean' is every data scientist's dream, not a problem!"),
        (eda_id, "Pearson correlation coefficient (r) ranges from:", "multiple_choice", "-1 to +1", Some("0 to 1"), Some("-1 to +1"), Some("-100 to +100"), Some("0 to 100"), None, "Pearson's r ranges from -1 (perfect negative correlation) through 0 (no correlation) to +1 (perfect positive correlation)."),
        (eda_id, "Who is considered the inventor of Exploratory Data Analysis?", "multiple_choice", "John Tukey", Some("Ronald Fisher"), Some("John Tukey"), Some("Karl Pearson"), Some("Florence Nightingale"), None, "John Tukey pioneered EDA in his 1977 book, emphasizing the importance of looking at data before making assumptions."),
        (eda_id, "True or false: Correlation between two variables proves that one causes the other.", "true_false", "false", Some("true"), Some("false"), None, None, None, "Correlation does not imply causation. Two variables may correlate due to a shared confounding variable or pure coincidence."),
        (ml_id, "Predicting whether an email is spam or not is an example of:", "multiple_choice", "Classification", Some("Regression"), Some("Classification"), Some("Clustering"), Some("Dimensionality reduction"), None, "Spam detection is a classification task — predicting a categorical label (spam vs. not spam) from features."),
        (ml_id, "When a model memorizes training data but fails on new data, it is ___.", "fill_in_blank", "overfitting", None, None, None, None, Some("Over-..."), "Overfitting occurs when a model learns noise and specific patterns in training data that don't generalize to unseen data."),
        (ml_id, "Which type of machine learning uses labeled training examples?", "multiple_choice", "Supervised learning", Some("Supervised learning"), Some("Unsupervised learning"), Some("Reinforcement learning"), Some("Transfer learning"), None, "Supervised learning trains on labeled data (input-output pairs) to learn a mapping function."),
        (nn_id, "The most popular activation function for hidden layers in modern neural networks is:", "multiple_choice", "ReLU", Some("Sigmoid"), Some("Tanh"), Some("ReLU"), Some("Step function"), None, "ReLU (Rectified Linear Unit) = max(0, x) is the most widely used activation due to computational efficiency and reduced vanishing gradient problems."),
        (nn_id, "The algorithm that trains neural networks by computing gradients backwards is called ___.", "fill_in_blank", "backpropagation", None, None, None, None, Some("Back-..."), "Backpropagation uses the chain rule of calculus to compute gradients of the loss with respect to each weight, enabling gradient descent to update the network."),
        (nn_id, "True or false: A neural network with no hidden layers can learn non-linear patterns.", "true_false", "false", Some("true"), Some("false"), None, None, None, "Without hidden layers, a neural network is just a linear model (perceptron). Hidden layers with non-linear activation functions enable learning complex, non-linear patterns."),
        (viz_id, "Which chart type is best for showing trends over time?", "multiple_choice", "Line chart", Some("Pie chart"), Some("Line chart"), Some("Bar chart"), Some("Scatter plot"), None, "Line charts excel at showing how values change over time, with the x-axis representing time and the y-axis the measured variable."),
        (viz_id, "Edward Tufte's principle about maximizing data vs. decoration is called the ___.", "fill_in_blank", "data-ink ratio", None, None, None, None, Some("___-___ ratio"), "The data-ink ratio measures the proportion of a chart's 'ink' devoted to actual data. Tufte advocates maximizing it by removing chartjunk."),
        (viz_id, "Approximately what percentage of men have some form of color vision deficiency?", "multiple_choice", "8%", Some("1%"), Some("4%"), Some("8%"), Some("15%"), None, "About 8% of men (and 0.5% of women) have color vision deficiency. Good visualizations use colorblind-friendly palettes."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path for Data Science
    let path_topics = [
        (viz_id, "Start with visualization — learn to see patterns in data"),
        (wrangle_id, "Master data cleaning — the foundation of all analysis"),
        (eda_id, "Explore data systematically before modeling"),
        (ml_id, "Learn core machine learning concepts and algorithms"),
        (nn_id, "Dive into neural networks and deep learning"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Data Science Foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_music_theory(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('Music Theory', 'Understanding the building blocks of music — scales, chords, rhythm, harmony, and form.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Music Theory'", [], |r| r.get(0),
    )?;

    let topics = [
        ("Scales and Keys", "beginner", 1),
        ("Chords and Harmony", "intermediate", 2),
        ("Rhythm and Meter", "beginner", 3),
        ("Musical Form", "intermediate", 4),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let scales_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Scales and Keys'", [subj_id], |r| r.get(0),
    )?;
    let chords_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Chords and Harmony'", [subj_id], |r| r.get(0),
    )?;
    let rhythm_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Rhythm and Meter'", [subj_id], |r| r.get(0),
    )?;
    let form_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Musical Form'", [subj_id], |r| r.get(0),
    )?;

    let lessons: &[LessonRow] = &[
        (scales_id, "The Major Scale", "A **scale** is a sequence of notes in ascending/descending order. The **major scale** follows a specific pattern of whole (W) and half (H) steps:\n\n**W-W-H-W-W-W-H**\n\nStarting on C: C-D-E-F-G-A-B-C (all white keys on piano — no sharps or flats)\n\n**Key signatures** tell you which sharps or flats to use:\n- **Sharp keys (Circle of Fifths clockwise):** G(1♯), D(2♯), A(3♯), E(4♯), B(5♯), F♯(6♯)\n- **Flat keys (Circle of Fifths counter-clockwise):** F(1♭), B♭(2♭), E♭(3♭), A♭(4♭), D♭(5♭), G♭(6♭)\n\nEvery major scale has a **relative minor** that shares the same key signature (starts on the 6th degree). C major's relative minor is A minor.", 1),
        (chords_id, "Building Chords", "A **chord** is three or more notes sounded together.\n\n**Triads** (3 notes, built in thirds):\n- **Major:** Root + Major 3rd + Perfect 5th (happy sound) → C-E-G\n- **Minor:** Root + Minor 3rd + Perfect 5th (sad sound) → C-E♭-G\n- **Diminished:** Root + Minor 3rd + Diminished 5th (tense) → C-E♭-G♭\n- **Augmented:** Root + Major 3rd + Augmented 5th (dreamy) → C-E-G♯\n\n**Seventh chords** add a 4th note:\n- **Major 7th:** Cmaj7 = C-E-G-B (smooth, jazzy)\n- **Dominant 7th:** C7 = C-E-G-B♭ (bluesy, wants to resolve)\n- **Minor 7th:** Cm7 = C-E♭-G-B♭ (mellow)\n\n**Chord progressions** are sequences that create movement. The most common: **I-V-vi-IV** (used in countless pop songs).", 2),
        (rhythm_id, "Understanding Rhythm", "**Rhythm** is the pattern of sounds and silences in time.\n\n**Note values:**\n- Whole note = 4 beats\n- Half note = 2 beats\n- Quarter note = 1 beat\n- Eighth note = ½ beat\n- Sixteenth note = ¼ beat\n\n**Time signatures** tell you the meter:\n- **4/4 (Common time):** 4 quarter-note beats per measure (most pop/rock)\n- **3/4 (Waltz time):** 3 quarter-note beats per measure\n- **6/8 (Compound duple):** 6 eighth-note beats, grouped in 2 sets of 3\n\n**Tempo** is the speed (BPM):\n- Largo: 40-60 BPM (very slow)\n- Andante: 76-108 BPM (walking pace)\n- Allegro: 120-156 BPM (fast, lively)\n- Presto: 168-200 BPM (very fast)\n\n**Syncopation** = emphasis on normally weak beats — creates groove and tension.", 1),
        (form_id, "Musical Structure", "**Form** is the architecture of a piece — how sections are organized.\n\n**Common forms:**\n- **Binary (AB):** Two contrasting sections. Common in Baroque dances.\n- **Ternary (ABA):** Statement-contrast-return. Da capo arias, minuets.\n- **Rondo (ABACA...):** Main theme returns between contrasting episodes.\n- **Sonata form:** Exposition (themes) → Development (transformation) → Recapitulation (return). The backbone of Classical-era first movements.\n- **Verse-Chorus:** Modern pop structure. Verse (story) + Chorus (hook).\n- **12-Bar Blues:** I-I-I-I / IV-IV-I-I / V-IV-I-V — foundation of blues, rock, and jazz.\n\n**Bridge** = contrasting section that provides variety before the final chorus.\n**Coda** = concluding section that wraps up the piece.", 1),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: &[ExplanationRow] = &[
        (scales_id, "Circle of Fifths", "A visual diagram arranging all 12 major keys by ascending perfect fifths (clockwise) and descending fifths/ascending fourths (counter-clockwise). Adjacent keys differ by one sharp or flat, making it invaluable for understanding key relationships, transposition, and chord progressions.", Some("Like a clock face where each hour is a musical key — neighboring hours sound most naturally connected."), Some("Why do adjacent keys on the Circle of Fifths share so many chords?")),
        (chords_id, "Chord Inversions", "Rearranging which note is in the bass. Root position: root on bottom. First inversion: 3rd on bottom. Second inversion: 5th on bottom. Inversions create smoother voice leading between chords and change the 'weight' or character of the chord without changing its function.", Some("Like rearranging furniture — same pieces, different layout, different feel."), Some("How do inversions help create smooth bass lines?")),
        (rhythm_id, "Polyrhythm", "Two or more conflicting rhythmic patterns played simultaneously. Common example: 3 against 2 (triplets over duplets). Found extensively in African music, jazz, progressive rock, and Afro-Cuban styles. Creates complexity and forward motion by layering rhythmic tension.", Some("Like patting your head while rubbing your stomach — two different motions happening at once that feel right together."), Some("What is the difference between polyrhythm and syncopation?")),
        (form_id, "Sonata Form", "The most important form in Classical music. Three sections: Exposition (presents two contrasting themes in different keys), Development (transforms and fragments the themes dramatically), and Recapitulation (restates both themes in the home key). Often bookended by an introduction and coda.", Some("Like a three-act story: introduce characters (exposition), put them in conflict (development), resolve everything (recapitulation)."), Some("Why does the recapitulation bring both themes back in the home key?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    let quizzes: &[QuizRow] = &[
        (scales_id, "The pattern of whole and half steps in a major scale is:", "multiple_choice", "W-W-H-W-W-W-H", Some("W-H-W-W-H-W-W"), Some("W-W-H-W-W-W-H"), Some("H-W-W-H-W-W-W"), Some("W-W-W-H-W-W-H"), None, "The major scale follows Whole-Whole-Half-Whole-Whole-Whole-Half. This pattern produces the familiar 'do-re-mi' sound."),
        (scales_id, "C major's relative minor is ___.", "fill_in_blank", "A minor", None, None, None, None, Some("Start on the 6th degree of C major..."), "The relative minor starts on the 6th degree of the major scale. In C major: C(1) D(2) E(3) F(4) G(5) A(6) — so A minor."),
        (scales_id, "How many sharps does the key of D major have?", "multiple_choice", "2", Some("0"), Some("1"), Some("2"), Some("3"), None, "D major has 2 sharps: F♯ and C♯. Following the Circle of Fifths: G(1♯), D(2♯)."),
        (chords_id, "A major triad consists of a root, a major third, and a ___.", "fill_in_blank", "perfect fifth", None, None, None, None, Some("The fifth is neither augmented nor diminished..."), "A major triad = Root + Major 3rd + Perfect 5th. For example, C major = C-E-G."),
        (chords_id, "The most common chord progression in pop music is:", "multiple_choice", "I-V-vi-IV", Some("I-IV-V-I"), Some("I-V-vi-IV"), Some("ii-V-I"), Some("I-vi-IV-V"), None, "I-V-vi-IV is the most ubiquitous pop progression, used in songs from 'Let It Be' to 'No Woman No Cry' to countless modern hits."),
        (chords_id, "True or false: A diminished triad contains a minor third and a diminished fifth.", "true_false", "true", Some("true"), Some("false"), None, None, None, "A diminished triad is built with a minor 3rd (3 semitones) and a diminished 5th (6 semitones from the root)."),
        (rhythm_id, "A whole note lasts ___ beats in 4/4 time.", "fill_in_blank", "4", None, None, None, None, Some("It fills the whole measure..."), "A whole note sustains for 4 beats — the entire duration of one measure in 4/4 time."),
        (rhythm_id, "A waltz is typically written in ___ time.", "fill_in_blank", "3/4", None, None, None, None, Some("ONE-two-three, ONE-two-three..."), "Waltzes use 3/4 time: three quarter-note beats per measure, with emphasis on beat 1."),
        (rhythm_id, "What tempo marking means 'walking pace' (76-108 BPM)?", "multiple_choice", "Andante", Some("Largo"), Some("Andante"), Some("Allegro"), Some("Presto"), None, "Andante (Italian for 'walking') indicates a moderate, walking-pace tempo of about 76-108 BPM."),
        (form_id, "The three sections of sonata form are:", "multiple_choice", "Exposition, Development, Recapitulation", Some("Verse, Chorus, Bridge"), Some("Exposition, Development, Recapitulation"), Some("Introduction, Theme, Coda"), Some("A, B, A"), None, "Sonata form: Exposition (present themes) → Development (transform them) → Recapitulation (restate in home key)."),
        (form_id, "The ___ Blues form uses the chord progression I-I-I-I / IV-IV-I-I / V-IV-I-V.", "fill_in_blank", "12-bar", None, None, None, None, Some("How many measures?"), "The 12-bar blues is a 12-measure chord progression that forms the foundation of blues, rock, and jazz music."),
        (form_id, "In a Rondo form (ABACA), what section keeps returning?", "multiple_choice", "A", Some("A"), Some("B"), Some("C"), Some("All sections return equally"), None, "In a Rondo, the A section (main theme) returns between each contrasting episode (B, C, etc.)."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path
    let path_topics = [
        (rhythm_id, "Start with rhythm — the heartbeat of music"),
        (scales_id, "Learn scales and keys — the melodic building blocks"),
        (chords_id, "Build chords from scales — the foundation of harmony"),
        (form_id, "Understand how musical sections are organized into larger structures"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Music Theory Foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_civics_and_media(conn: &Connection) -> Result<(), rusqlite::Error> {
    // --- Civics & Government (subject_id = 18) ---
    conn.execute(
        "INSERT INTO subjects (name, description) VALUES (?1, ?2)",
        ["Civics & Government", "Understanding democracy, rights, laws, and how governments work."],
    )?;
    let civics_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Civics & Government'", [], |r| r.get(0),
    )?;

    let civics_topics = [
        ("Democracy & Voting", "beginner", 1),
        ("Branches of Government", "beginner", 2),
        ("Rights & Responsibilities", "intermediate", 3),
        ("International Organizations", "intermediate", 4),
    ];
    let mut civics_topic_ids = Vec::new();
    for (name, diff, order) in &civics_topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![civics_id, name, diff, order],
        )?;
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND name = ?2",
            rusqlite::params![civics_id, name], |r| r.get(0),
        )?;
        civics_topic_ids.push(tid);
    }

    // Lessons
    let civics_lessons: Vec<LessonRow> = vec![
        (civics_topic_ids[0], "What is Democracy?", "Democracy means 'rule by the people.' In a democracy, citizens have the power to choose their leaders through voting. There are two main forms: direct democracy (citizens vote on every issue) and representative democracy (citizens elect representatives who make decisions for them). Most modern countries use representative democracy.", 1),
        (civics_topic_ids[0], "How Voting Works", "Voting is the foundation of democracy. In most countries, citizens above a certain age can vote in elections. Different voting systems exist: first-past-the-post (the candidate with the most votes wins), proportional representation (seats are allocated based on vote share), and ranked-choice voting (voters rank candidates in order of preference).", 2),
        (civics_topic_ids[1], "The Three Branches", "Most democracies have three branches of government: Legislative (makes laws), Executive (enforces laws), and Judicial (interprets laws). This separation of powers prevents any single branch from becoming too powerful. Each branch can check the others — this is called 'checks and balances.'", 1),
        (civics_topic_ids[1], "How Laws Are Made", "A bill starts as a proposal, is debated in the legislature, may be amended, and must pass a vote. In many systems it must pass both houses of a bicameral legislature. The executive (president or prime minister) then signs it into law or vetoes it. Courts can later review laws for constitutionality.", 2),
        (civics_topic_ids[2], "Fundamental Rights", "Human rights are basic freedoms that belong to every person. Key categories include: civil rights (freedom of speech, religion, assembly), political rights (right to vote, run for office), and social rights (education, healthcare). Many countries protect these in a constitution or bill of rights.", 1),
        (civics_topic_ids[2], "Civic Responsibilities", "Along with rights come responsibilities: obeying laws, paying taxes, serving on juries, staying informed, and participating in civic life. Responsible citizenship strengthens democracy and ensures that government serves the people effectively.", 2),
        (civics_topic_ids[3], "The United Nations", "Founded in 1945, the UN promotes international cooperation, peace, and human rights. Its main bodies include the General Assembly (all member states), the Security Council (15 members, 5 permanent with veto power), and specialized agencies like UNESCO and WHO.", 1),
    ];
    for (tid, title, content, order) in &civics_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let civics_explanations: Vec<ExplanationRow> = vec![
        (civics_topic_ids[0], "Democracy", "A system of government where power comes from the people, usually through elections.", Some("Think of it like a classroom where students vote on the rules — everyone gets a say."), Some("What's the difference between direct and representative democracy?")),
        (civics_topic_ids[1], "Checks and Balances", "A system where each branch of government can limit the power of the others, preventing abuse of power.", Some("Like a game of rock-paper-scissors — no single choice always wins."), Some("Can you name one way the legislative branch checks the executive?")),
        (civics_topic_ids[2], "Human Rights", "Basic rights and freedoms that belong to every person in the world, from birth until death.", Some("They're like the rules of a fair game — everyone deserves to play by the same rules."), Some("Why might some rights need to be limited in certain situations?")),
        (civics_topic_ids[3], "United Nations", "An international organization founded in 1945 to promote peace, cooperation, and human rights among all nations.", Some("Think of it as a global student council where countries come together to solve shared problems."), Some("What is the role of the UN Security Council?")),
    ];
    for (tid, concept, expl, analogy, followup) in &civics_explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // Quiz questions
    let civics_quizzes: Vec<QuizRowHint> = vec![
        (civics_topic_ids[0], "What does 'democracy' literally mean?", "multiple_choice", "Rule by the people",
         Some("Rule by the wealthy"), Some("Rule by the people"), Some("Rule by the military"), Some("Rule by the king"),
         "Democracy comes from Greek: 'demos' (people) + 'kratos' (rule).",
         "The word has Greek roots: demos = people, kratos = power or rule."),
        (civics_topic_ids[0], "In a representative democracy, citizens:", "multiple_choice", "Elect representatives to make decisions",
         Some("Vote on every law directly"), Some("Elect representatives to make decisions"), Some("Have no political power"), Some("Are ruled by a monarch"),
         "Representatives act on behalf of voters.",
         "In representative democracy, elected officials make decisions on behalf of the people who voted for them."),
        (civics_topic_ids[0], "True or false: In a direct democracy, citizens vote on every issue themselves.", "true_false", "true",
         None, None, None, None,
         "Direct democracy gives every citizen a direct vote.",
         "In a direct democracy, citizens participate directly in decision-making rather than through elected representatives."),
        (civics_topic_ids[1], "Which branch of government makes laws?", "multiple_choice", "Legislative",
         Some("Executive"), Some("Legislative"), Some("Judicial"), Some("Military"),
         "Think about the word 'legislate.'",
         "The legislative branch (parliament or congress) is responsible for creating, debating, and passing laws."),
        (civics_topic_ids[1], "What is the purpose of checks and balances?", "multiple_choice", "Prevent any one branch from having too much power",
         Some("Make government slower"), Some("Prevent any one branch from having too much power"), Some("Give the president total control"), Some("Eliminate the courts"),
         "It's about distributing power.",
         "Checks and balances ensure that no single branch of government can dominate the others, protecting against tyranny."),
        (civics_topic_ids[1], "The judicial branch's main role is to:", "multiple_choice", "Interpret laws",
         Some("Write laws"), Some("Enforce laws"), Some("Interpret laws"), Some("Collect taxes"),
         "Judges and courts are part of this branch.",
         "Courts interpret laws, determine their meaning, and decide whether they align with the constitution."),
        (civics_topic_ids[2], "Freedom of speech is an example of a:", "multiple_choice", "Civil right",
         Some("Social right"), Some("Civil right"), Some("Economic right"), Some("Military right"),
         "Civil rights protect individual freedoms.",
         "Civil rights are personal freedoms that protect individuals from government overreach, including speech, religion, and assembly."),
        (civics_topic_ids[2], "True or false: Citizens have responsibilities as well as rights.", "true_false", "true",
         None, None, None, None,
         "Rights and responsibilities go together.",
         "Civic responsibilities like obeying laws, paying taxes, and voting help maintain a functioning democracy."),
        (civics_topic_ids[3], "When was the United Nations founded?", "multiple_choice", "1945",
         Some("1918"), Some("1945"), Some("1960"), Some("1900"),
         "It was founded after a major world conflict.",
         "The UN was established in 1945 after World War II to prevent future conflicts and promote international cooperation."),
        (civics_topic_ids[3], "How many permanent members does the UN Security Council have?", "fill_in_blank", "5",
         None, None, None, None,
         "Think about the victors of World War II.",
         "The five permanent members are the US, UK, France, Russia, and China — each with veto power."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &civics_quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path
    for (i, (tid, desc)) in [
        (civics_topic_ids[0], "Start with the foundations of democracy and how voting works"),
        (civics_topic_ids[1], "Learn how government is structured with branches and checks"),
        (civics_topic_ids[2], "Explore the rights citizens have and the responsibilities that come with them"),
        (civics_topic_ids[3], "Understand how nations work together through international organizations"),
    ].iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Civics Foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    // --- Media Literacy (subject_id = 19) ---
    conn.execute(
        "INSERT INTO subjects (name, description) VALUES (?1, ?2)",
        ["Media Literacy", "Critical thinking about media, information sources, and digital citizenship."],
    )?;
    let media_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Media Literacy'", [], |r| r.get(0),
    )?;

    let media_topics = [
        ("Evaluating Sources", "beginner", 1),
        ("Misinformation & Bias", "intermediate", 2),
        ("Digital Citizenship", "beginner", 3),
        ("Data Privacy", "intermediate", 4),
    ];
    let mut media_topic_ids = Vec::new();
    for (name, diff, order) in &media_topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![media_id, name, diff, order],
        )?;
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND name = ?2",
            rusqlite::params![media_id, name], |r| r.get(0),
        )?;
        media_topic_ids.push(tid);
    }

    // Lessons
    let media_lessons: Vec<LessonRow> = vec![
        (media_topic_ids[0], "How to Evaluate a Source", "Not all information is equally trustworthy. To evaluate a source, ask: Who wrote it? What are their credentials? When was it published? Is it supported by evidence? Does the publication have editorial standards? Primary sources (original documents, data) are generally more reliable than secondary sources (summaries, opinions).", 1),
        (media_topic_ids[0], "The CRAAP Test", "The CRAAP test helps evaluate information: Currency (Is it recent?), Relevance (Does it relate to your topic?), Authority (Who is the author/publisher?), Accuracy (Is it supported by evidence?), Purpose (Why does this information exist? To inform, persuade, sell, or entertain?).", 2),
        (media_topic_ids[1], "Types of Misinformation", "Misinformation is false information spread without intent to harm. Disinformation is deliberately false. Malinformation is true information shared to cause harm. Common forms include: misleading headlines, out-of-context quotes, manipulated images, satire mistaken for news, and deepfakes.", 1),
        (media_topic_ids[1], "Recognizing Bias", "All media has some bias. Types include: confirmation bias (favoring information that confirms beliefs), selection bias (choosing which stories to cover), framing (how a story is presented), and omission (what's left out). Reading multiple sources helps identify bias.", 2),
        (media_topic_ids[2], "Being a Good Digital Citizen", "Digital citizenship means using technology responsibly. Key principles: think before you post (it's permanent), respect others online, protect your personal information, give credit to original creators, and report harmful content. Your digital footprint follows you.", 1),
        (media_topic_ids[2], "Online Communication", "Online communication lacks tone and body language, making misunderstandings common. Best practices: re-read before sending, assume good intent, avoid ALL CAPS (it reads as shouting), use clear language, and remember there's a real person on the other side.", 2),
        (media_topic_ids[3], "Understanding Data Privacy", "Every time you use the internet, you leave data behind — browsing history, location data, purchases, social media activity. Companies collect this data to build profiles for advertising. Understanding what data you share and how to control it is essential digital literacy.", 1),
        (media_topic_ids[3], "Protecting Your Privacy", "Practical privacy steps: use strong unique passwords, enable two-factor authentication, review app permissions, use privacy settings on social media, be cautious with public Wi-Fi, and understand what data terms of service allow companies to collect.", 2),
    ];
    for (tid, title, content, order) in &media_lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let media_explanations: Vec<ExplanationRow> = vec![
        (media_topic_ids[0], "Primary Source", "An original document or first-hand account — the raw evidence before anyone interprets it.", Some("Like getting the recipe from the chef who created the dish, not from someone who tasted it once."), Some("Why are primary sources considered more reliable?")),
        (media_topic_ids[1], "Confirmation Bias", "The tendency to seek out, remember, and favor information that confirms what you already believe.", Some("Like only reading reviews from people who agree with your opinion of a movie."), Some("How can you guard against confirmation bias in your own research?")),
        (media_topic_ids[2], "Digital Footprint", "The trail of data you leave behind when you use the internet — posts, searches, likes, and more.", Some("Think of it like footprints in wet cement — they harden and become permanent."), Some("What are some things that contribute to your digital footprint?")),
        (media_topic_ids[3], "Two-Factor Authentication", "A security method requiring two different forms of identification to access an account — something you know (password) plus something you have (phone code).", Some("Like a bank vault that needs both a key and a combination — one alone isn't enough."), Some("Why is two-factor authentication more secure than a password alone?")),
    ];
    for (tid, concept, expl, analogy, followup) in &media_explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // Quiz questions
    let media_quizzes: Vec<QuizRowHint> = vec![
        (media_topic_ids[0], "What does the 'A' in the CRAAP test stand for?", "multiple_choice", "Authority and Accuracy",
         Some("Audience"), Some("Authority and Accuracy"), Some("Authenticity"), Some("Allegiance"),
         "There are actually two A's in CRAAP.",
         "The two A's stand for Authority (who wrote it?) and Accuracy (is it supported by evidence?)."),
        (media_topic_ids[0], "A primary source is:", "multiple_choice", "An original document or first-hand account",
         Some("A Wikipedia article"), Some("An original document or first-hand account"), Some("A newspaper editorial"), Some("A textbook summary"),
         "Think about who created it and when.",
         "Primary sources are original, uninterpreted materials like diaries, data sets, photographs, or official documents."),
        (media_topic_ids[0], "True or false: A source's recency is irrelevant to its reliability.", "true_false", "false",
         None, None, None, None,
         "Think about fast-changing fields like medicine or technology.",
         "Currency matters because outdated information may no longer be accurate, especially in rapidly evolving fields."),
        (media_topic_ids[1], "What is the difference between misinformation and disinformation?", "multiple_choice", "Disinformation is spread deliberately; misinformation is not",
         Some("They are the same thing"), Some("Disinformation is spread deliberately; misinformation is not"), Some("Misinformation is more harmful"), Some("Disinformation only exists online"),
         "Intent is the key difference.",
         "Misinformation is false info shared without harmful intent; disinformation is deliberately created and spread to deceive."),
        (media_topic_ids[1], "Which of these is an example of framing bias?", "multiple_choice", "Describing a glass as 'half empty' vs 'half full'",
         Some("Publishing a story late"), Some("Describing a glass as 'half empty' vs 'half full'"), Some("Using a large font"), Some("Adding a photo to an article"),
         "Framing is about how information is presented.",
         "Framing bias is about how the same facts can be presented in ways that influence perception, like emphasizing positive or negative aspects."),
        (media_topic_ids[1], "True or false: Reading only one news source is sufficient to get an unbiased view.", "true_false", "false",
         None, None, None, None,
         "Every source has some perspective.",
         "All media has some bias. Reading multiple sources with different perspectives helps you form a more complete and balanced understanding."),
        (media_topic_ids[2], "Your digital footprint includes:", "multiple_choice", "All of the above",
         Some("Social media posts"), Some("Search history"), Some("Online purchases"), Some("All of the above"),
         "Think about everything you do online.",
         "Your digital footprint encompasses all traces of your online activity: posts, searches, purchases, location data, and more."),
        (media_topic_ids[2], "True or false: Content you delete from social media is always permanently removed.", "true_false", "false",
         None, None, None, None,
         "Think about screenshots and cached copies.",
         "Deleted content may persist in backups, caches, screenshots, or archives. Once something is posted online, it can be very difficult to fully remove."),
        (media_topic_ids[3], "Two-factor authentication combines:", "multiple_choice", "Something you know and something you have",
         Some("Two passwords"), Some("Something you know and something you have"), Some("A username and email"), Some("Two security questions"),
         "Think about the two different types of verification.",
         "2FA requires two different types: typically a password (something you know) plus a code sent to your phone (something you have)."),
        (media_topic_ids[3], "Which is the best practice for passwords?", "multiple_choice", "Use a unique strong password for each account",
         Some("Use the same password everywhere"), Some("Use a unique strong password for each account"), Some("Write passwords on a sticky note"), Some("Use your birthday"),
         "Think about what happens if one account is compromised.",
         "Unique strong passwords for each account mean that if one is compromised, your other accounts remain secure. A password manager helps manage this."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in &media_quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path
    for (i, (tid, desc)) in [
        (media_topic_ids[0], "Learn to evaluate whether information sources are trustworthy"),
        (media_topic_ids[1], "Understand how misinformation spreads and how to spot bias"),
        (media_topic_ids[2], "Practice responsible behavior online as a digital citizen"),
        (media_topic_ids[3], "Protect your personal data and understand privacy"),
    ].iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Media Literacy Essentials', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    // --- Extra Geography content ---
    // Add more quiz questions for Geography topics (Continents & Oceans = topic ~22, Weather & Climate = ~23)
    let geo_continents_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Continents & Oceans'", [], |r| r.get(0),
    ).unwrap_or(0);

    let geo_weather_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Weather & Climate'", [], |r| r.get(0),
    ).unwrap_or(0);

    let geo_maps_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Maps & Navigation'", [], |r| r.get(0),
    ).unwrap_or(0);

    if geo_continents_id > 0 {
        let extra_geo_quizzes: Vec<QuizRowNoTopic> = vec![
            ("Which is the largest ocean on Earth?", "multiple_choice", "Pacific Ocean",
             Some("Atlantic Ocean"), Some("Pacific Ocean"), Some("Indian Ocean"), Some("Arctic Ocean"),
             "It covers more area than all the land on Earth combined.",
             "The Pacific Ocean is the largest and deepest ocean, covering about 63 million square miles — more than all land areas combined."),
            ("How many continents are there?", "fill_in_blank", "7",
             None, None, None, None,
             "Think about the major landmasses.",
             "The seven continents are: Africa, Antarctica, Asia, Australia/Oceania, Europe, North America, and South America."),
            ("Which continent is the smallest by area?", "multiple_choice", "Australia/Oceania",
             Some("Europe"), Some("Antarctica"), Some("Australia/Oceania"), Some("South America"),
             "It's also an island nation.",
             "Australia (sometimes called Oceania) is the smallest continent at about 8.5 million square kilometers."),
            ("True or false: Africa is the second-largest continent.", "true_false", "true",
             None, None, None, None,
             "Asia is the largest.",
             "Africa is the second-largest continent by both area and population, after Asia."),
            ("Which ocean lies between Europe and North America?", "multiple_choice", "Atlantic Ocean",
             Some("Pacific Ocean"), Some("Atlantic Ocean"), Some("Indian Ocean"), Some("Southern Ocean"),
             "Think about transatlantic travel.",
             "The Atlantic Ocean separates the Americas from Europe and Africa."),
        ];
        for (question, qtype, answer, a, b, c, d, hint, expl) in &extra_geo_quizzes {
            conn.execute(
                "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                rusqlite::params![geo_continents_id, question, qtype, answer, a, b, c, d, hint, expl],
            )?;
        }

        // Extra lessons for Geography
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![geo_continents_id, "The Five Oceans",
                "Earth has five oceans: Pacific (largest), Atlantic (second largest), Indian (warmest), Southern (around Antarctica), and Arctic (smallest and shallowest). Together they cover about 71% of Earth's surface and contain 97% of all water on Earth. Ocean currents act like a global conveyor belt, distributing heat and influencing weather patterns worldwide.",
                3],
        )?;
    }

    if geo_weather_id > 0 {
        let weather_quizzes: Vec<QuizRowNoTopic> = vec![
            ("What is the difference between weather and climate?", "multiple_choice", "Weather is short-term; climate is long-term average",
             Some("They are the same thing"), Some("Weather is short-term; climate is long-term average"), Some("Climate only exists in cold places"), Some("Weather doesn't change"),
             "Think about time scales.",
             "Weather describes atmospheric conditions over hours or days; climate describes average patterns over decades."),
            ("Which gas is most responsible for the greenhouse effect?", "multiple_choice", "Carbon dioxide",
             Some("Oxygen"), Some("Carbon dioxide"), Some("Nitrogen"), Some("Helium"),
             "It's released by burning fossil fuels.",
             "Carbon dioxide (CO₂) is the primary greenhouse gas driving climate change, mainly from burning fossil fuels."),
            ("True or false: Humidity measures the amount of water vapor in the air.", "true_false", "true",
             None, None, None, None,
             "Think about muggy summer days.",
             "Humidity is the concentration of water vapor in the air. High humidity makes hot days feel even warmer."),
        ];
        for (question, qtype, answer, a, b, c, d, hint, expl) in &weather_quizzes {
            conn.execute(
                "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                rusqlite::params![geo_weather_id, question, qtype, answer, a, b, c, d, hint, expl],
            )?;
        }
    }

    if geo_maps_id > 0 {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![geo_maps_id, "Latitude and Longitude",
                "Latitude lines run east-west and measure distance north or south of the equator (0° to 90°). Longitude lines run north-south and measure distance east or west of the Prime Meridian (0° to 180°). Together, they form a grid that can pinpoint any location on Earth. For example, the Eiffel Tower is at approximately 48.86°N, 2.35°E.",
                3],
        )?;
    }

    Ok(())
}

pub fn seed_world_languages(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add subject
    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('World Languages', 'Introduction to languages of the world — basic vocabulary, grammar, and cultural context.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'World Languages'", [], |r| r.get(0),
    )?;

    // Topics
    let topics = [
        ("German Basics", "beginner", 1),
        ("Spanish Basics", "beginner", 2),
        ("French Basics", "beginner", 3),
        ("Language Families", "intermediate", 4),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let german_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'German Basics'", [], |r| r.get(0))?;
    let spanish_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Spanish Basics'", [], |r| r.get(0))?;
    let french_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'French Basics'", [], |r| r.get(0))?;
    let families_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Language Families'", [], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (german_id, "Greetings in German", "Hallo = Hello\nGuten Morgen = Good morning\nGuten Tag = Good day\nGuten Abend = Good evening\nTschüss = Bye\nAuf Wiedersehen = Goodbye (formal)\nDanke = Thank you\nBitte = Please / You're welcome", 1),
        (german_id, "Numbers 1-10 in German", "eins = 1, zwei = 2, drei = 3, vier = 4, fünf = 5\nsechs = 6, sieben = 7, acht = 8, neun = 9, zehn = 10\nGerman numbers are straightforward — learn these and you can build larger numbers.\n11 = elf, 12 = zwölf, then 13-19 follow the pattern: dreizehn, vierzehn, etc.", 2),
        (spanish_id, "Greetings in Spanish", "Hola = Hello\nBuenos días = Good morning\nBuenas tardes = Good afternoon\nBuenas noches = Good night\nAdiós = Goodbye\nPor favor = Please\nGracias = Thank you\nDe nada = You're welcome", 1),
        (spanish_id, "Numbers 1-10 in Spanish", "uno = 1, dos = 2, tres = 3, cuatro = 4, cinco = 5\nseis = 6, siete = 7, ocho = 8, nueve = 9, diez = 10\nSpanish numbers are used throughout Latin America and Spain.\n11 = once, 12 = doce, 13 = trece, 14 = catorce, 15 = quince", 2),
        (french_id, "Greetings in French", "Bonjour = Hello / Good day\nBonsoir = Good evening\nSalut = Hi (informal)\nAu revoir = Goodbye\nMerci = Thank you\nS'il vous plaît = Please (formal)\nDe rien = You're welcome\nComment allez-vous? = How are you? (formal)", 1),
        (french_id, "Numbers 1-10 in French", "un = 1, deux = 2, trois = 3, quatre = 4, cinq = 5\nsix = 6, sept = 7, huit = 8, neuf = 9, dix = 10\nFrench numbers get interesting at 70: soixante-dix (60+10), 80: quatre-vingts (4×20), 90: quatre-vingt-dix (4×20+10).", 2),
        (families_id, "Major Language Families", "Indo-European: Includes English, Spanish, Hindi, German, French, Russian — spoken by ~3 billion people.\nSino-Tibetan: Mandarin Chinese, Cantonese, Burmese — ~1.5 billion speakers.\nAfroasiatic: Arabic, Hebrew, Amharic, Hausa.\nNiger-Congo: Swahili, Yoruba, Zulu — largest family by number of languages.\nAustronesian: Malay, Tagalog, Hawaiian, Maori.\nDravidian: Tamil, Telugu, Kannada — mainly in South India.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (german_id, "German Cases", "German has four grammatical cases: Nominative (subject), Accusative (direct object), Dative (indirect object), and Genitive (possession). Each case changes the article before a noun. For example, 'der Hund' (the dog, nominative) becomes 'den Hund' (accusative).", Some("Think of cases like different hats a noun wears depending on its job in the sentence."), Some("Can you think of how 'I' vs 'me' in English is similar to German cases?")),
        (spanish_id, "Spanish Gender", "Every Spanish noun has a gender: masculine or feminine. Generally, words ending in -o are masculine (el libro = the book) and words ending in -a are feminine (la mesa = the table). The article changes accordingly: el (masc) / la (fem).", Some("Imagine every object in Spanish is either wearing a blue hat (masculine) or a pink hat (feminine)."), Some("Why do you think 'el agua' uses a masculine article even though agua ends in -a?")),
        (french_id, "French Liaison", "In French, normally silent consonants at the end of a word are pronounced when followed by a vowel sound. This is called 'liaison.' For example: 'les amis' is pronounced 'lez-ami' — the s in 'les' connects to the vowel in 'amis.'", Some("It's like words holding hands across a gap — they connect their sounds when a vowel follows."), Some("Can you spot the liaison in 'nous avons' (we have)?")),
        (families_id, "Cognates", "Cognates are words in different languages that share a common origin and similar form. For example: English 'mother', German 'Mutter', Spanish 'madre', French 'mère' — all from Proto-Indo-European *méh₂tēr. Recognizing cognates helps you learn new languages faster!", Some("Cognates are like linguistic cousins — they grew up in different countries but still look alike at the family reunion."), Some("Can you find cognates between English and Spanish for the word 'telephone'?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let quiz_questions: Vec<QuizRowHint> = vec![
        (german_id, "What does 'Guten Morgen' mean in English?", "multiple_choice", "Good morning", Some("Good night"), Some("Good morning"), Some("Goodbye"), Some("Thank you"), "B", "'Morgen' means morning, 'Guten' means good."),
        (german_id, "How do you say 'thank you' in German?", "fill_in_blank", "Danke", None, None, None, None, "Danke", "'Danke' is one of the most common German words. 'Danke schön' is the more formal version."),
        (german_id, "What is the German word for the number 5?", "fill_in_blank", "fünf", None, None, None, None, "fünf", "German numbers 1-5: eins, zwei, drei, vier, fünf. Note the umlaut (ü) in fünf."),
        (spanish_id, "How do you say 'please' in Spanish?", "multiple_choice", "Por favor", Some("Gracias"), Some("De nada"), Some("Por favor"), Some("Adiós"), "C", "'Por favor' literally translates to 'as a favor.'"),
        (spanish_id, "What is 'siete' in English?", "fill_in_blank", "seven", None, None, None, None, "7", "The Spanish numbers: seis (6), siete (7), ocho (8)."),
        (spanish_id, "What gender is the Spanish word 'libro' (book)?", "true_false", "True", None, None, None, None, "True", "Words ending in -o are typically masculine in Spanish: el libro."),
        (french_id, "What does 'Merci' mean?", "multiple_choice", "Thank you", Some("Hello"), Some("Goodbye"), Some("Thank you"), Some("Please"), "C", "'Merci' is the standard French word for thank you. 'Merci beaucoup' means thank you very much."),
        (french_id, "How do you say 'goodbye' in French?", "fill_in_blank", "Au revoir", None, None, None, None, "Au revoir", "'Au revoir' literally means 'until seeing again.'"),
        (families_id, "Which language family does English belong to?", "multiple_choice", "Indo-European", Some("Sino-Tibetan"), Some("Afroasiatic"), Some("Indo-European"), Some("Austronesian"), "C", "English is part of the Germanic branch of the Indo-European family."),
        (families_id, "True or False: Mandarin Chinese and English belong to the same language family.", "true_false", "False", None, None, None, None, "False", "Mandarin is Sino-Tibetan; English is Indo-European. They are completely unrelated language families."),
    ];
    for (tid, question, qtype, correct, oa, ob, oc, od, hint, explanation) in &quiz_questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'medium')",
            rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, hint, explanation],
        )?;
    }

    // Learning path
    let path_steps = [
        (1, families_id, "Understand how languages are related"),
        (2, german_id, "Learn basic German greetings and numbers"),
        (3, spanish_id, "Learn basic Spanish greetings and numbers"),
        (4, french_id, "Learn basic French greetings and numbers"),
    ];
    for (order, tid, desc) in &path_steps {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('world languages', ?1, ?2, ?3)",
            rusqlite::params![order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_geography_expanded(conn: &Connection) -> Result<(), rusqlite::Error> {
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Geography'", [], |r| r.get(0),
    )?;

    // Add more topics
    let topics = [
        ("Climate Zones", "intermediate", 10),
        ("Oceans and Seas", "beginner", 11),
        ("Plate Tectonics", "intermediate", 12),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let climate_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Climate Zones'", [], |r| r.get(0))?;
    let oceans_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Oceans and Seas'", [], |r| r.get(0))?;
    let tectonics_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Plate Tectonics'", [], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (climate_id, "The Five Major Climate Zones", "Tropical: Hot and wet year-round, near the equator (Amazon, Congo).\nDry (Arid): Very little rainfall (Sahara, Gobi Desert).\nTemperate: Moderate temperatures, distinct seasons (Western Europe, Eastern US).\nContinental: Extreme seasonal variation, cold winters (Russia, Canada).\nPolar: Freezing year-round (Antarctica, Arctic).\nKöppen classification uses temperature and precipitation to define these zones precisely.", 1),
        (oceans_id, "The Five Oceans", "Pacific Ocean: Largest and deepest, covers more area than all land combined.\nAtlantic Ocean: Second largest, separates Americas from Europe/Africa.\nIndian Ocean: Third largest, warmest ocean.\nSouthern Ocean: Encircles Antarctica, recognized as a distinct ocean in 2000.\nArctic Ocean: Smallest and shallowest, partially covered in sea ice year-round.", 1),
        (tectonics_id, "How Plate Tectonics Works", "Earth's outer shell (lithosphere) is broken into ~15 major plates that float on the semi-fluid asthenosphere below.\nPlates move 1-10 cm per year driven by convection currents in the mantle.\nConvergent boundaries: plates collide → mountains (Himalayas) or subduction zones (Mariana Trench).\nDivergent boundaries: plates pull apart → mid-ocean ridges, rift valleys.\nTransform boundaries: plates slide past each other → earthquakes (San Andreas Fault).", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    conn.execute(
        "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, 'Plate Tectonics', 'The theory that Earth''s outer layer is divided into large plates that move, float, and interact, causing earthquakes, volcanoes, and mountain formation.', 'Imagine Earth''s surface as a cracked eggshell floating on a soft-boiled egg — the pieces can shift and bump into each other.', 'If continents are still moving, what might the world map look like in 250 million years?')",
        [tectonics_id],
    )?;
    conn.execute(
        "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, 'Climate Zones', 'Earth''s climate zones are determined primarily by latitude, altitude, and proximity to oceans. The equator receives the most direct sunlight, creating tropical zones, while the poles receive the least.', 'Think of Earth like a rotisserie chicken — the middle (equator) gets the most heat, while the top and bottom stay cooler.', 'Why do coastal cities often have milder climates than inland cities at the same latitude?')",
        [climate_id],
    )?;

    // Quiz questions
    let quizzes: Vec<QuizRowHint> = vec![
        (oceans_id, "Which is the largest ocean on Earth?", "multiple_choice", "Pacific Ocean", Some("Atlantic Ocean"), Some("Pacific Ocean"), Some("Indian Ocean"), Some("Arctic Ocean"), "B", "The Pacific Ocean covers about 165.25 million km² — more than all land on Earth combined."),
        (oceans_id, "True or False: The Southern Ocean was officially recognized as a distinct ocean in 2000.", "true_false", "True", None, None, None, None, "True", "National Geographic officially recognized the Southern Ocean in 2000, though it was debated for decades."),
        (climate_id, "Which climate zone experiences extreme seasonal variation with very cold winters?", "multiple_choice", "Continental", Some("Tropical"), Some("Temperate"), Some("Continental"), Some("Polar"), "C", "Continental climates are found far from ocean influence, leading to hot summers and very cold winters."),
        (tectonics_id, "What happens at a convergent plate boundary?", "multiple_choice", "Plates collide", Some("Plates slide past each other"), Some("Plates pull apart"), Some("Plates collide"), Some("Plates disappear"), "C", "At convergent boundaries, plates push together, creating mountains or subduction zones."),
        (tectonics_id, "The San Andreas Fault is an example of which type of plate boundary?", "fill_in_blank", "transform", None, None, None, None, "transform", "At transform boundaries, plates slide horizontally past each other, causing earthquakes."),
    ];
    for (tid, question, qtype, correct, oa, ob, oc, od, hint, explanation) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'medium')",
            rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, hint, explanation],
        )?;
    }

    Ok(())
}

pub fn seed_psychology_expanded(conn: &Connection) -> Result<(), rusqlite::Error> {
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Psychology'", [], |r| r.get(0),
    )?;

    let topics = [
        ("Memory and Learning", "intermediate", 10),
        ("Cognitive Biases", "intermediate", 11),
        ("Developmental Psychology", "beginner", 12),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let memory_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Memory and Learning'", [], |r| r.get(0))?;
    let biases_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Cognitive Biases'", [], |r| r.get(0))?;
    let dev_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Developmental Psychology'", [], |r| r.get(0))?;

    let lessons: Vec<LessonRow> = vec![
        (memory_id, "How Memory Works", "Memory has three stages:\n1. Encoding: Converting sensory input into a form the brain can store.\n2. Storage: Maintaining information over time — short-term (seconds to minutes) vs long-term (days to lifetime).\n3. Retrieval: Accessing stored information when needed.\n\nKey concepts:\n- Working memory holds ~4 items at once (updated from Miller's 'magical number 7').\n- Spaced repetition strengthens long-term memory by reviewing at optimal intervals.\n- Sleep is critical for memory consolidation — the brain replays and strengthens memories during sleep.", 1),
        (biases_id, "Common Cognitive Biases", "Cognitive biases are systematic errors in thinking:\n\nConfirmation Bias: Seeking information that confirms what you already believe.\nAnchoring: Over-relying on the first piece of information encountered.\nDunning-Kruger Effect: Low-skilled individuals overestimate their ability; experts underestimate theirs.\nAvailability Heuristic: Judging likelihood based on how easily examples come to mind.\nSunk Cost Fallacy: Continuing something because of past investment, not future value.\nHalo Effect: Letting one positive trait influence overall judgment.", 1),
        (dev_id, "Piaget's Stages of Development", "Jean Piaget identified four stages of cognitive development:\n\n1. Sensorimotor (0-2 years): Learning through senses and movement. Object permanence develops.\n2. Preoperational (2-7 years): Symbolic thinking, but egocentric — difficulty seeing others' perspectives.\n3. Concrete Operational (7-11 years): Logical thinking about concrete events. Understanding conservation.\n4. Formal Operational (12+): Abstract and hypothetical thinking. Scientific reasoning emerges.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (memory_id, "Spaced Repetition", "Spaced repetition is a learning technique where reviews are scheduled at increasing intervals. It exploits the 'spacing effect' — we remember better when study sessions are spread out over time rather than crammed together.", Some("It's like watering a plant — a little water spread across many days helps it grow better than dumping a bucket all at once."), Some("Why do you think cramming the night before an exam doesn't work as well as studying a little each day?")),
        (biases_id, "Confirmation Bias", "The tendency to search for, interpret, and remember information that confirms our pre-existing beliefs, while ignoring contradictory evidence. This affects everything from scientific research to political opinions.", Some("Imagine wearing tinted glasses — everything looks the color of your lenses, and you forget the world has other colors."), Some("Can you think of a time when you only noticed evidence that supported what you already believed?")),
        (dev_id, "Object Permanence", "The understanding that objects continue to exist even when they cannot be seen. Infants develop this around 8-12 months. Before this, 'out of sight' literally means 'out of mind' — peek-a-boo is genuinely surprising!", Some("For a baby without object permanence, you hiding behind your hands is like you teleporting to another dimension and back."), Some("How might lack of object permanence affect how a baby reacts when a parent leaves the room?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRowHint> = vec![
        (memory_id, "How many items can working memory typically hold at once?", "multiple_choice", "About 4", Some("About 4"), Some("About 20"), Some("About 100"), Some("Unlimited"), "A", "Modern research (Cowan, 2001) suggests working memory holds about 4 'chunks' of information, updated from Miller's classic estimate of 7±2."),
        (memory_id, "True or False: Sleep is important for memory consolidation.", "true_false", "True", None, None, None, None, "True", "During sleep, the brain replays and strengthens neural connections formed during learning, moving memories from short-term to long-term storage."),
        (biases_id, "What is the Dunning-Kruger Effect?", "multiple_choice", "Low-skilled people overestimate their ability", Some("Experts overestimate their ability"), Some("Low-skilled people overestimate their ability"), Some("Everyone accurately estimates their ability"), Some("Memory gets worse with age"), "B", "The Dunning-Kruger effect describes how people with limited knowledge in an area tend to overestimate their competence, while true experts often underestimate theirs."),
        (biases_id, "The tendency to continue investing in something because of past investment is called:", "fill_in_blank", "sunk cost fallacy", None, None, None, None, "sunk cost", "The sunk cost fallacy leads us to throw good money after bad, because we don't want our past investment to be 'wasted.'"),
        (dev_id, "At what age do infants typically develop object permanence?", "multiple_choice", "8-12 months", Some("Birth"), Some("8-12 months"), Some("3-4 years"), Some("6-7 years"), "B", "Object permanence — understanding that things exist even when hidden — develops around 8-12 months according to Piaget."),
    ];
    for (tid, question, qtype, correct, oa, ob, oc, od, hint, explanation) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'medium')",
            rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, hint, explanation],
        )?;
    }

    Ok(())
}

fn seed_game_theory(conn: &Connection) -> Result<(), rusqlite::Error> {
    // --- Subject ---
    conn.execute(
        "INSERT INTO subjects (name, description) VALUES (?1, ?2)",
        ["Game Theory", "Strategic decision-making — analyzing how rational agents interact, compete, and cooperate in games of strategy."],
    )?;
    let subject_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Game Theory'", [], |r| r.get(0),
    )?;

    // --- Topics ---
    let topics = [
        ("Nash Equilibrium", "intermediate", 1),
        ("Prisoner's Dilemma", "beginner", 2),
        ("Dominant Strategies", "beginner", 3),
        ("Zero-Sum Games", "intermediate", 4),
        ("Evolutionary Game Theory", "advanced", 5),
        ("Mechanism Design", "advanced", 6),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subject_id, name, diff, order],
        )?;
    }

    let nash_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Nash Equilibrium'",
        [subject_id], |r| r.get(0),
    )?;
    let prisoner_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Prisoner''s Dilemma'",
        [subject_id], |r| r.get(0),
    )?;
    let dominant_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Dominant Strategies'",
        [subject_id], |r| r.get(0),
    )?;
    let zerosum_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Zero-Sum Games'",
        [subject_id], |r| r.get(0),
    )?;
    let evo_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Evolutionary Game Theory'",
        [subject_id], |r| r.get(0),
    )?;
    let mech_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Mechanism Design'",
        [subject_id], |r| r.get(0),
    )?;

    // --- Lessons ---
    let lessons: Vec<LessonRow> = vec![
        (nash_id, "What is a Nash Equilibrium?",
         "A Nash Equilibrium is a situation where no player can benefit by changing their strategy while the other players keep theirs unchanged. Named after mathematician John Nash (1950), it represents a stable state of a game.\n\nKey insight: at a Nash Equilibrium, every player is doing the best they can *given* what everyone else is doing. It doesn't mean the outcome is optimal for everyone — just that nobody has an incentive to unilaterally deviate.\n\nExample: In a two-player game where both can choose Left or Right, if both choosing Left gives each player 3 points and any deviation gives the deviator only 1 point, then (Left, Left) is a Nash Equilibrium.", 1),
        (nash_id, "Finding Nash Equilibria",
         "To find Nash Equilibria in a normal-form game:\n\n1. For each player, find their best response to each possible strategy of the other players.\n2. A strategy profile where every player is playing a best response is a Nash Equilibrium.\n\nMethod: Underline best responses in the payoff matrix. Cells where ALL payoffs are underlined are Nash Equilibria.\n\nSome games have multiple Nash Equilibria (coordination games), some have exactly one (like Prisoner's Dilemma), and some have none in pure strategies but always have one in mixed strategies (Nash's theorem).", 2),
        (prisoner_id, "The Classic Prisoner's Dilemma",
         "Two suspects are arrested and separated. Each can either Cooperate (stay silent) or Defect (betray the other).\n\nPayoffs:\n- Both cooperate: 1 year each\n- Both defect: 3 years each\n- One defects, one cooperates: defector goes free, cooperator gets 5 years\n\nThe dilemma: rational self-interest leads both to defect, even though mutual cooperation would be better for both. This illustrates why individually rational decisions can lead to collectively irrational outcomes.\n\nReal-world examples: arms races, climate agreements, price wars between companies.", 1),
        (prisoner_id, "Iterated Prisoner's Dilemma",
         "When the Prisoner's Dilemma is played repeatedly, cooperation can emerge through strategies like Tit-for-Tat:\n\n1. Cooperate on the first move\n2. Then copy whatever the opponent did last round\n\nRobert Axelrod's tournaments (1980s) showed that Tit-for-Tat performed remarkably well — it's simple, nice (never defects first), retaliatory (punishes defection), and forgiving (returns to cooperation).\n\nThe 'shadow of the future': when players expect to interact again, the threat of future punishment makes cooperation sustainable.", 2),
        (dominant_id, "What is a Dominant Strategy?",
         "A dominant strategy is one that gives a player a better (or equal) payoff than any other strategy, regardless of what the opponents do.\n\n- Strictly dominant: always gives a strictly better payoff\n- Weakly dominant: always gives at least as good a payoff, and strictly better in at least one case\n\nIterative elimination of dominated strategies (IEDS): remove strategies that are never best responses, then repeat. If this process leads to a single outcome, the game is dominance-solvable.\n\nNot all games have dominant strategies — when they don't, we need Nash Equilibrium analysis.", 1),
        (zerosum_id, "Zero-Sum and Constant-Sum Games",
         "In a zero-sum game, one player's gain is exactly the other's loss. The payoffs always sum to zero (or a constant).\n\nExamples: chess, poker (ignoring the house), penalty kicks in soccer.\n\nVon Neumann's Minimax Theorem (1928): In every finite two-player zero-sum game, there exists an optimal mixed strategy for each player. The value of the game is uniquely determined.\n\nMinimax strategy: maximize your minimum payoff (equivalently, minimize the opponent's maximum payoff). This leads to the game's 'value' — the expected payoff under optimal play.", 1),
        (evo_id, "Evolution and Strategy",
         "Evolutionary Game Theory applies game-theoretic concepts to biological evolution. Instead of rational players choosing strategies, natural selection favors strategies that produce more offspring.\n\nKey concept: Evolutionarily Stable Strategy (ESS) — a strategy that, if adopted by a population, cannot be invaded by any alternative strategy. Introduced by John Maynard Smith and George Price (1973).\n\nClassic example: Hawk-Dove game — competing for resources with aggressive (Hawk) or peaceful (Dove) strategies. Pure populations of either type are unstable; evolution produces a mix.", 1),
        (mech_id, "Designing the Rules of the Game",
         "Mechanism design is 'reverse game theory' — instead of analyzing a given game, you design the rules to achieve a desired outcome.\n\nApplications:\n- Auction design (how to sell goods efficiently)\n- Voting systems (how to aggregate preferences fairly)\n- Market design (matching students to schools, organ donors to recipients)\n\nKey concepts:\n- Incentive compatibility: players reveal true preferences\n- Individual rationality: players voluntarily participate\n- The Revelation Principle: any mechanism can be replicated by a direct mechanism where truth-telling is optimal\n\nNobel Prize 2007: Hurwicz, Maskin, Myerson for mechanism design theory.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // --- Explanations ---
    let explanations: Vec<ExplanationRow> = vec![
        (nash_id, "Nash Equilibrium",
         "A Nash Equilibrium is like a group of friends choosing restaurants — once everyone has settled on a place they're happy with *given* where everyone else is going, nobody wants to switch alone. It's not necessarily the best outcome for the group, just one where no individual wants to change.",
         Some("Think of cars at an intersection with no traffic light — eventually drivers settle into a pattern where nobody gains by changing their behavior."),
         Some("Can a game have more than one Nash Equilibrium?")),
        (prisoner_id, "Prisoner's Dilemma",
         "The Prisoner's Dilemma shows why cooperation is hard even when it benefits everyone. Each player thinking 'I'm better off defecting no matter what the other does' leads to a worse outcome for both.",
         Some("It's like two roommates who'd both prefer a clean apartment but each hopes the other will clean — so neither does."),
         Some("How does repeating the game change the outcome?")),
        (dominant_id, "Dominant Strategy",
         "A dominant strategy is your 'no-brainer' choice — it's best regardless of what others do. Like bringing an umbrella when there's a 90% chance of rain: whether it rains or not, you won't regret it.",
         Some("Imagine a test where one answer is always correct regardless of the question — that's a dominant strategy."),
         Some("What happens when no player has a dominant strategy?")),
        (zerosum_id, "Zero-Sum Game",
         "In a zero-sum game, the pie is fixed — you can only gain by taking from someone else. Your gain of $10 means someone else loses $10.",
         Some("Think of siblings splitting a pizza — every extra slice for one means fewer for the other."),
         Some("Are most real-world interactions zero-sum?")),
        (evo_id, "Evolutionarily Stable Strategy",
         "An ESS is a strategy so successful that a whole population using it can't be overtaken by mutants using a different strategy. It's nature's version of a Nash Equilibrium, enforced by survival rather than rationality.",
         Some("Like a language everyone speaks — a few people switching to a new language won't succeed because nobody understands them."),
         Some("Can cooperation evolve through natural selection?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // --- Quiz Questions ---
    let quizzes: Vec<QuizRowHint> = vec![
        (nash_id, "In a Nash Equilibrium, what is true about each player's strategy?", "multiple_choice",
         "No player can improve their payoff by unilaterally changing strategy",
         Some("All players maximize total payoff"),
         Some("No player can improve their payoff by unilaterally changing strategy"),
         Some("Players always cooperate"),
         Some("The game must be zero-sum"),
         "Think about what 'equilibrium' means — stability",
         "At a Nash Equilibrium, each player's strategy is a best response to the others. No one can do better by changing their own strategy alone."),
        (nash_id, "Who proved that every finite game has at least one Nash Equilibrium (in mixed strategies)?", "fill_in_blank",
         "John Nash",
         None, None, None, None,
         "Nobel Prize in Economics, 1994",
         "John Nash proved this in 1950 in his PhD dissertation, using Kakutani's fixed-point theorem."),
        (nash_id, "True or False: A Nash Equilibrium always produces the best outcome for all players.", "true_false",
         "False",
         None, None, None, None,
         "Think about the Prisoner's Dilemma",
         "The Prisoner's Dilemma is a famous counter-example: the Nash Equilibrium (both defect) is worse for both players than mutual cooperation."),
        (prisoner_id, "In the classic Prisoner's Dilemma, what is the Nash Equilibrium?", "multiple_choice",
         "Both defect",
         Some("Both cooperate"),
         Some("Both defect"),
         Some("One cooperates, one defects"),
         Some("There is no equilibrium"),
         "What does each player prefer regardless of the other's choice?",
         "Defecting is a dominant strategy for both players, so the unique Nash Equilibrium is mutual defection — even though mutual cooperation would be better."),
        (prisoner_id, "Which strategy won Robert Axelrod's iterated Prisoner's Dilemma tournament?", "fill_in_blank",
         "Tit-for-Tat",
         None, None, None, None,
         "It starts by cooperating, then copies the opponent's last move",
         "Tit-for-Tat, submitted by Anatol Rapoport, won both of Axelrod's tournaments. It's simple, retaliatory, forgiving, and clear."),
        (dominant_id, "A strategy that always gives a better payoff than alternatives regardless of opponents' choices is called:", "fill_in_blank",
         "dominant strategy",
         None, None, None, None,
         "It 'dominates' all other options",
         "A strictly dominant strategy yields a strictly higher payoff than any other strategy for every possible combination of opponents' strategies."),
        (dominant_id, "What is IEDS in game theory?", "multiple_choice",
         "Iterative Elimination of Dominated Strategies",
         Some("Iterative Elimination of Dominated Strategies"),
         Some("Iterative Evaluation of Decision Systems"),
         Some("Internal Equilibrium Detection System"),
         Some("Integrated Economic Decision Solver"),
         "It's a method for simplifying games step by step",
         "IEDS works by repeatedly removing strategies that are never best responses, progressively simplifying the game until (ideally) a single solution remains."),
        (zerosum_id, "In a zero-sum game, the sum of all players' payoffs is always:", "fill_in_blank",
         "zero",
         None, None, None, None,
         "The name gives it away!",
         "In a zero-sum game, one player's gain is another's loss, so the total payoff across all players is always zero (or a constant)."),
        (zerosum_id, "The Minimax Theorem was proved by:", "multiple_choice",
         "John von Neumann",
         Some("John Nash"),
         Some("John von Neumann"),
         Some("Adam Smith"),
         Some("Robert Axelrod"),
         "He's also known as the father of computer science",
         "John von Neumann proved the Minimax Theorem in 1928, establishing that every finite two-player zero-sum game has an optimal solution."),
        (evo_id, "What does ESS stand for in evolutionary game theory?", "fill_in_blank",
         "Evolutionarily Stable Strategy",
         None, None, None, None,
         "It's a strategy that can't be invaded by mutants",
         "An ESS is a strategy that, if adopted by a population, cannot be invaded by any rare alternative strategy through natural selection."),
        (evo_id, "Who introduced the concept of Evolutionarily Stable Strategies?", "multiple_choice",
         "John Maynard Smith and George Price",
         Some("Charles Darwin"),
         Some("John Maynard Smith and George Price"),
         Some("John Nash"),
         Some("Richard Dawkins"),
         "Published in 1973",
         "John Maynard Smith and George Price introduced the ESS concept in their 1973 paper 'The Logic of Animal Conflict'."),
        (mech_id, "Mechanism design is sometimes called:", "multiple_choice",
         "Reverse game theory",
         Some("Forward game theory"),
         Some("Reverse game theory"),
         Some("Applied statistics"),
         Some("Decision theory"),
         "Instead of analyzing a game, you design one",
         "Mechanism design works backwards from desired outcomes to design the rules of a game that will produce those outcomes when players act rationally."),
        (mech_id, "The Revelation Principle states that any mechanism can be replaced by one where players:", "fill_in_blank",
         "tell the truth",
         None, None, None, None,
         "It's about honesty being optimal",
         "The Revelation Principle says any equilibrium of any mechanism can be replicated by a direct mechanism in which truth-telling is an equilibrium strategy."),
    ];
    for (tid, question, qtype, correct, oa, ob, oc, od, hint, explanation) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'medium')",
            rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, hint, explanation],
        )?;
    }

    // --- Learning Path ---
    let path_steps = [
        (1, prisoner_id, "Start with the Prisoner's Dilemma — the most intuitive game theory scenario"),
        (2, dominant_id, "Learn about dominant strategies — the simplest solution concept"),
        (3, nash_id, "Master Nash Equilibrium — the central concept of game theory"),
        (4, zerosum_id, "Study zero-sum games and minimax strategies"),
        (5, evo_id, "Explore how game theory applies to evolution"),
        (6, mech_id, "Advanced: learn to design games with mechanism design"),
    ];
    for (order, tid, desc) in &path_steps {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Game Theory Foundations', ?1, ?2, ?3)",
            rusqlite::params![order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_architecture(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Subject: Architecture & Design (id assigned dynamically)
    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES (?1, ?2)",
        ["Architecture & Design", "The art and science of designing buildings and spaces — from ancient temples to modern skyscrapers."],
    )?;
    let subject_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Architecture & Design'", [], |r| r.get(0),
    )?;

    // Topics
    let topics = [
        ("Architectural Styles", "beginner", 1),
        ("Structural Engineering Basics", "intermediate", 2),
        ("Sustainable Design", "intermediate", 3),
        ("Interior Design Principles", "beginner", 4),
        ("Urban Planning", "advanced", 5),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subject_id, name, diff, order],
        )?;
    }

    let tid_styles: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Architectural Styles'",
        [subject_id], |r| r.get(0),
    )?;
    let tid_structural: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Structural Engineering Basics'",
        [subject_id], |r| r.get(0),
    )?;
    let tid_sustainable: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Sustainable Design'",
        [subject_id], |r| r.get(0),
    )?;
    let tid_interior: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Interior Design Principles'",
        [subject_id], |r| r.get(0),
    )?;
    let tid_urban: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Urban Planning'",
        [subject_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (tid_styles, "Classical Architecture", "Classical architecture originated in ancient Greece and Rome.\n\nKey features:\n- Columns: Doric (simple), Ionic (scrolls), Corinthian (ornate leaves)\n- Symmetry and proportion based on mathematical ratios\n- Pediments (triangular gables) above entrances\n- Use of marble and stone\n\nFamous examples: The Parthenon (Athens), The Pantheon (Rome), The US Capitol Building.\n\nThe classical orders (Doric, Ionic, Corinthian) define column proportions and decorative styles that influenced architecture for over 2,000 years.", 1),
        (tid_styles, "Gothic Architecture", "Gothic architecture dominated Europe from the 12th to 16th centuries.\n\nKey innovations:\n- Pointed arches (distribute weight more efficiently than round arches)\n- Flying buttresses (external supports allowing thinner walls)\n- Ribbed vaults (intersecting arches creating a skeletal framework)\n- Large stained glass windows (enabled by thinner walls)\n- Vertical emphasis — buildings reach toward the heavens\n\nFamous examples: Notre-Dame de Paris, Cologne Cathedral, Westminster Abbey.\n\nThe term 'Gothic' was originally pejorative — Renaissance critics thought these buildings were barbaric compared to classical forms.", 2),
        (tid_styles, "Modern & Contemporary Architecture", "Modern architecture (1920s–1970s) rejected ornament in favor of function.\n\nKey principles:\n- 'Form follows function' (Louis Sullivan)\n- Open floor plans\n- Use of steel, glass, and reinforced concrete\n- Flat roofs and clean lines\n- 'Less is more' (Mies van der Rohe)\n\nKey movements:\n- Bauhaus: merged art and industry (Walter Gropius)\n- International Style: glass curtain walls, steel frames\n- Brutalism: raw concrete, bold geometric forms\n\nContemporary architecture (1970s–present) embraces diversity:\n- Deconstructivism (Frank Gehry, Zaha Hadid)\n- Parametricism (computer-generated organic forms)\n- Green architecture (sustainability-focused design)", 3),
        (tid_structural, "Forces and Loads", "Every building must withstand multiple forces:\n\n1. Dead loads: the building's own weight (walls, floors, roof)\n2. Live loads: occupants, furniture, equipment\n3. Wind loads: lateral forces from wind pressure\n4. Seismic loads: forces from earthquakes\n5. Snow loads: weight of accumulated snow\n\nStructural members:\n- Beams: horizontal, resist bending\n- Columns: vertical, resist compression\n- Trusses: triangulated frameworks (very efficient)\n- Arches: curved, convert loads to compression\n\nThe triangle is the strongest geometric shape — it cannot be deformed without changing the length of its sides.", 1),
        (tid_structural, "Materials in Construction", "Common structural materials and their properties:\n\nSteel:\n- High tensile strength (resists pulling apart)\n- Ductile (bends before breaking)\n- Vulnerable to fire and corrosion\n\nConcrete:\n- Excellent in compression (pushing together)\n- Weak in tension → reinforced with steel rebar\n- Fire-resistant and durable\n\nWood:\n- Renewable, lightweight, good insulator\n- Vulnerable to fire, rot, and insects\n- Excellent strength-to-weight ratio\n\nMasonry (brick/stone):\n- Very durable in compression\n- Heavy and labor-intensive\n- Weak in tension (needs mortar and reinforcement)", 2),
        (tid_sustainable, "Green Building Principles", "Sustainable design minimizes environmental impact:\n\n1. Energy efficiency: insulation, LED lighting, efficient HVAC\n2. Water conservation: rainwater harvesting, low-flow fixtures\n3. Material selection: recycled, local, renewable materials\n4. Site planning: orientation for solar gain, natural ventilation\n5. Indoor air quality: non-toxic materials, ventilation\n\nCertification systems:\n- LEED (Leadership in Energy and Environmental Design)\n- BREEAM (UK-based)\n- Passive House (extreme energy efficiency)\n\nA Passive House uses up to 90% less heating energy than a conventional building by using super-insulation, airtight construction, and heat recovery ventilation.", 1),
        (tid_sustainable, "Renewable Energy in Buildings", "Buildings can generate their own energy:\n\nSolar:\n- Photovoltaic panels convert sunlight to electricity\n- Solar thermal heats water\n- Building-integrated PV (BIPV) replaces traditional materials\n\nWind:\n- Small turbines for individual buildings\n- More effective in rural/exposed locations\n\nGeothermal:\n- Ground-source heat pumps use stable earth temperature\n- Very efficient for heating and cooling\n- High upfront cost, low operating cost\n\nNet-zero buildings produce as much energy as they consume annually.", 2),
        (tid_interior, "Color and Space", "Color profoundly affects how we experience spaces:\n\nWarm colors (red, orange, yellow):\n- Make spaces feel smaller and cozier\n- Stimulate appetite (used in restaurants)\n- Increase energy and excitement\n\nCool colors (blue, green, purple):\n- Make spaces feel larger and calmer\n- Promote concentration (used in offices)\n- Can feel cold if overused\n\nNeutrals (white, gray, beige):\n- Versatile backgrounds\n- White reflects light, making spaces feel larger\n- Too much white can feel sterile\n\nThe 60-30-10 rule: 60% dominant color, 30% secondary, 10% accent.", 1),
        (tid_interior, "Lighting Design", "Lighting transforms spaces. Three types:\n\n1. Ambient lighting: general illumination (ceiling lights)\n2. Task lighting: focused for specific activities (desk lamp)\n3. Accent lighting: highlights features (spotlights on art)\n\nColor temperature:\n- Warm (2700K): cozy, residential feel\n- Neutral (3500K): balanced, retail/office\n- Cool (5000K+): energizing, clinical\n\nNatural light is ideal — reduces energy use and improves mood.\nStrategies: large windows, skylights, light shelves, clerestory windows.\n\nCircadian lighting adjusts color temperature throughout the day to support natural sleep-wake cycles.", 2),
        (tid_urban, "City Planning Fundamentals", "Urban planning shapes how cities function:\n\nZoning: separating land uses (residential, commercial, industrial)\n- Euclidean zoning: strict separation\n- Mixed-use zoning: combining uses (live/work/shop)\n\nTransportation:\n- Transit-oriented development (TOD): dense housing near transit\n- Complete streets: designed for all users (cars, bikes, pedestrians)\n- The 15-minute city: everything within a 15-min walk/bike\n\nPublic spaces:\n- Parks, plazas, and waterfronts\n- Jane Jacobs: 'eyes on the street' — mixed use creates safety\n- Good public spaces have seating, shade, and reasons to linger\n\nDensity is not the enemy — well-planned density creates vibrant, efficient cities.", 1),
        (tid_urban, "Sustainable Urban Design", "Creating cities that work for people and the planet:\n\nGreen infrastructure:\n- Urban forests reduce heat island effect (cities can be 5-10°C hotter)\n- Bioswales and rain gardens manage stormwater\n- Green roofs insulate buildings and absorb rainfall\n\nWalkability factors (Walk Score):\n- Short blocks with frequent intersections\n- Mixed-use neighborhoods\n- Street trees and protected sidewalks\n- Ground-floor retail with active frontages\n\nResilience:\n- Flood-resistant design in coastal areas\n- Heat mitigation: reflective surfaces, shade, water features\n- Redundant infrastructure systems\n\nThe best cities balance density, nature, mobility, and equity.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow<'_>> = vec![
        (tid_styles, "Flying Buttress", "An external arch that transfers the weight of a wall outward and downward to a pier. This innovation allowed Gothic cathedrals to have thinner walls and larger windows.", Some("Think of it like a friend leaning against you to keep you from falling over — the buttress leans against the wall to support it."), Some("Why would thinner walls be desirable in a cathedral?")),
        (tid_styles, "The Golden Ratio", "The ratio approximately 1:1.618, found throughout classical architecture. The Parthenon's facade fits within a golden rectangle. Many architects use this ratio for aesthetically pleasing proportions.", Some("Like a recipe that just tastes right — certain proportions naturally look harmonious to the human eye."), Some("Can you find the golden ratio in any building near you?")),
        (tid_structural, "Compression vs Tension", "Compression pushes material together (a column bearing weight). Tension pulls material apart (a cable in a suspension bridge). Most structures experience both forces simultaneously.", Some("Squeeze a sponge — that is compression. Pull a rubber band — that is tension."), Some("Why is concrete strong in compression but weak in tension?")),
        (tid_sustainable, "Thermal Mass", "Dense materials (concrete, brick, stone) absorb heat slowly and release it slowly. In hot climates, thick walls absorb daytime heat and release it at night. This passive strategy reduces energy use.", Some("Like a water bottle that stays cool — dense materials act as a thermal battery."), Some("How could you use thermal mass in a cold climate?")),
        (tid_interior, "The 60-30-10 Rule", "A color distribution guideline: 60% dominant color (walls/floors), 30% secondary (furniture/textiles), 10% accent (accessories/art). Creates visual balance and prevents monotony.", Some("Like a well-composed outfit — mostly one color, with complementary pieces and a pop of contrast."), Some("What happens if you use 50-50 instead of 60-30-10?")),
        (tid_urban, "Jane Jacobs' Four Conditions", "Jane Jacobs identified four conditions for vibrant city neighborhoods: 1) Mixed primary uses, 2) Short blocks, 3) Buildings of varying age, 4) Sufficient density. Her book 'The Death and Life of Great American Cities' (1961) revolutionized urban planning.", Some("Like a healthy ecosystem — diversity creates resilience and vitality."), Some("Which of Jacobs' four conditions is most lacking in modern suburbs?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let quizzes: Vec<QuizRowHint<'_>> = vec![
        (tid_styles, "Which column order features ornate acanthus leaf capitals?", "multiple_choice", "Corinthian", Some("Doric"), Some("Ionic"), Some("Corinthian"), Some("Tuscan"), "The Corinthian order has elaborate capitals decorated with acanthus leaves and scrolls.", "medium"),
        (tid_styles, "Flying buttresses are a key feature of which architectural style?", "multiple_choice", "Gothic", Some("Classical"), Some("Gothic"), Some("Art Deco"), Some("Brutalist"), "Flying buttresses were developed in Gothic architecture to support thinner walls and larger windows.", "easy"),
        (tid_styles, "The phrase 'Less is more' is associated with which architect?", "multiple_choice", "Mies van der Rohe", Some("Frank Lloyd Wright"), Some("Le Corbusier"), Some("Mies van der Rohe"), Some("Frank Gehry"), "Ludwig Mies van der Rohe championed minimalism in the International Style.", "medium"),
        (tid_styles, "'Form follows function' was coined by which architect?", "fill_in_blank", "Louis Sullivan", None, None, None, None, "Louis Sullivan, the 'father of skyscrapers', coined this phrase that became a modernist mantra.", "medium"),
        (tid_styles, "The Bauhaus school was founded by Walter Gropius.", "true_false", "true", Some("True"), Some("False"), None, None, "Walter Gropius founded the Bauhaus in Weimar, Germany in 1919.", "easy"),
        (tid_styles, "Put these architectural periods in chronological order: Brutalism, Gothic, Classical, Art Nouveau", "ordering", "Classical,Gothic,Art Nouveau,Brutalism", Some("Classical"), Some("Gothic"), Some("Art Nouveau"), Some("Brutalism"), "Classical (antiquity) → Gothic (12th-16th c.) → Art Nouveau (1890-1910) → Brutalism (1950s-70s).", "hard"),
        (tid_structural, "Which geometric shape is considered the strongest for structural purposes?", "multiple_choice", "Triangle", Some("Square"), Some("Triangle"), Some("Circle"), Some("Hexagon"), "The triangle cannot be deformed without changing the length of its sides, making it inherently rigid.", "easy"),
        (tid_structural, "Concrete is strong in compression but weak in ___.", "fill_in_blank", "tension", None, None, None, None, "Concrete resists compressive forces well but cracks easily under tensile (pulling) forces, which is why it is reinforced with steel.", "medium"),
        (tid_structural, "Which structural member primarily resists bending forces?", "multiple_choice", "Beam", Some("Column"), Some("Beam"), Some("Foundation"), Some("Truss"), "Beams are horizontal members designed to resist bending from loads applied perpendicular to their length.", "easy"),
        (tid_structural, "Steel is vulnerable to fire.", "true_false", "true", Some("True"), Some("False"), None, None, "Steel loses strength rapidly at high temperatures — at 600°C it retains only about 40% of its room-temperature strength.", "medium"),
        (tid_sustainable, "What does LEED stand for?", "fill_in_blank", "Leadership in Energy and Environmental Design", None, None, None, None, "LEED is the most widely used green building certification system in the world.", "hard"),
        (tid_sustainable, "A Passive House can reduce heating energy by up to what percentage?", "multiple_choice", "90%", Some("50%"), Some("70%"), Some("90%"), Some("100%"), "Passive House buildings use up to 90% less heating energy through super-insulation, airtight construction, and heat recovery.", "medium"),
        (tid_sustainable, "Which renewable energy system uses stable underground temperatures?", "multiple_choice", "Geothermal", Some("Solar PV"), Some("Wind"), Some("Geothermal"), Some("Tidal"), "Ground-source heat pumps exploit the stable temperature of the earth (around 10-15°C) for efficient heating and cooling.", "easy"),
        (tid_sustainable, "Net-zero buildings produce as much energy as they consume.", "true_false", "true", Some("True"), Some("False"), None, None, "Net-zero energy buildings generate enough renewable energy to offset their annual consumption.", "easy"),
        (tid_interior, "According to the 60-30-10 rule, what percentage should be the accent color?", "multiple_choice", "10%", Some("10%"), Some("20%"), Some("30%"), Some("5%"), "The 60-30-10 rule allocates 10% to accent colors — small pops that add interest.", "easy"),
        (tid_interior, "A color temperature of 2700K would feel ___.", "fill_in_blank", "warm", None, None, None, None, "Lower color temperatures (2700K) produce a warm, yellowish light similar to incandescent bulbs.", "medium"),
        (tid_interior, "Which type of lighting is best for reading at a desk?", "multiple_choice", "Task lighting", Some("Ambient lighting"), Some("Task lighting"), Some("Accent lighting"), Some("Decorative lighting"), "Task lighting provides focused illumination for specific activities like reading or cooking.", "easy"),
        (tid_interior, "Warm colors make a room feel larger.", "true_false", "false", Some("True"), Some("False"), None, None, "Warm colors make spaces feel smaller and cozier. Cool colors create the illusion of more space.", "medium"),
        (tid_urban, "The '15-minute city' concept means everything is within a 15-minute ___.", "fill_in_blank", "walk", None, None, None, None, "The 15-minute city aims to have all daily needs accessible within a 15-minute walk or bike ride.", "easy"),
        (tid_urban, "Who wrote 'The Death and Life of Great American Cities'?", "multiple_choice", "Jane Jacobs", Some("Robert Moses"), Some("Jane Jacobs"), Some("Le Corbusier"), Some("Frank Lloyd Wright"), "Jane Jacobs published this influential critique of modernist urban planning in 1961.", "medium"),
        (tid_urban, "Urban heat island effect can make cities how much hotter than surrounding areas?", "multiple_choice", "5-10°C", Some("1-2°C"), Some("3-4°C"), Some("5-10°C"), Some("15-20°C"), "Dense urban areas with dark surfaces and waste heat can be 5-10°C warmer than surrounding rural areas.", "medium"),
        (tid_urban, "Transit-oriented development places dense housing near public transit.", "true_false", "true", Some("True"), Some("False"), None, None, "TOD concentrates housing, jobs, and services around transit stations to reduce car dependency.", "easy"),
    ];
    for (tid, question, qtype, correct, oa, ob, oc, od, explanation, difficulty) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10)",
            rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, explanation, difficulty],
        )?;
    }

    // Learning path
    let path_steps: Vec<(i64, &str)> = vec![
        (tid_styles, "Learn the major architectural styles from classical to contemporary"),
        (tid_interior, "Understand how color, light, and space create interior environments"),
        (tid_structural, "Learn the engineering principles that make buildings stand up"),
        (tid_sustainable, "Explore green building and renewable energy in architecture"),
        (tid_urban, "Understand how cities are planned and designed for people"),
    ];
    for (i, (tid, desc)) in path_steps.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["Architecture & Design Fundamentals", i + 1, tid, desc],
        )?;
    }

    Ok(())
}

/// Additional quiz questions for subjects that have fewer quizzes.
pub fn seed_extra_quizzes_round2(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Extra quizzes for Music (Musical Notes & Scales)
    let tid_music: Result<i64, _> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Musical Notes & Scales'", [], |r| r.get(0),
    );
    if let Ok(tid) = tid_music {
        let quizzes: Vec<QuizRowNoTopic<'_>> = vec![
            ("How many notes are in a chromatic scale?", "multiple_choice", "12", Some("7"), Some("8"), Some("12"), Some("14"), "A chromatic scale includes all 12 semitones within an octave.", "medium"),
            ("The distance between two adjacent keys on a piano is called a ___.", "fill_in_blank", "semitone", None, None, None, None, "A semitone (or half step) is the smallest interval in Western music.", "easy"),
            ("A major scale has 7 notes.", "true_false", "true", Some("True"), Some("False"), None, None, "Major scales consist of 7 unique notes plus the octave.", "easy"),
            ("Which note is the fifth degree of a C major scale?", "multiple_choice", "G", Some("D"), Some("E"), Some("G"), Some("A"), "C-D-E-F-G: G is the fifth note (dominant) of C major.", "medium"),
        ];
        for (question, qtype, correct, oa, ob, oc, od, explanation, difficulty) in &quizzes {
            conn.execute(
                "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10)",
                rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, explanation, difficulty],
            )?;
        }
    }

    // Extra quizzes for Art (Color Theory)
    let tid_art: Result<i64, _> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Color Theory'", [], |r| r.get(0),
    );
    if let Ok(tid) = tid_art {
        let quizzes: Vec<QuizRowNoTopic<'_>> = vec![
            ("Which color model is used for printing?", "multiple_choice", "CMYK", Some("RGB"), Some("CMYK"), Some("HSL"), Some("RYB"), "CMYK (Cyan, Magenta, Yellow, Key/Black) is the subtractive color model used in printing.", "medium"),
            ("Complementary colors are opposite each other on the color wheel.", "true_false", "true", Some("True"), Some("False"), None, None, "Complementary colors (e.g., red-green, blue-orange) create maximum contrast.", "easy"),
            ("What is the complementary color of blue?", "multiple_choice", "Orange", Some("Red"), Some("Green"), Some("Orange"), Some("Yellow"), "On the RYB color wheel, blue and orange are complementary colors.", "easy"),
            ("Adding white to a color creates a ___.", "fill_in_blank", "tint", None, None, None, None, "A tint is a color mixed with white, making it lighter. A shade is mixed with black.", "medium"),
        ];
        for (question, qtype, correct, oa, ob, oc, od, explanation, difficulty) in &quizzes {
            conn.execute(
                "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10)",
                rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, explanation, difficulty],
            )?;
        }
    }

    // Extra quizzes for Environmental Science
    let tid_eco: Result<i64, _> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Ecosystems & Biomes'", [], |r| r.get(0),
    );
    if let Ok(tid) = tid_eco {
        let quizzes: Vec<QuizRowNoTopic<'_>> = vec![
            ("Which biome has the highest biodiversity?", "multiple_choice", "Tropical rainforest", Some("Desert"), Some("Tundra"), Some("Tropical rainforest"), Some("Taiga"), "Tropical rainforests contain more than half of the world's species despite covering only ~6% of land.", "easy"),
            ("The tundra biome is characterized by permafrost.", "true_false", "true", Some("True"), Some("False"), None, None, "Tundra has permanently frozen subsoil (permafrost) and very short growing seasons.", "easy"),
            ("What is the largest biome on Earth by area?", "multiple_choice", "Taiga", Some("Desert"), Some("Taiga"), Some("Grassland"), Some("Tropical rainforest"), "The taiga (boreal forest) stretches across Russia, Canada, and Scandinavia — the largest terrestrial biome.", "hard"),
            ("An organism that breaks down dead matter is called a ___.", "fill_in_blank", "decomposer", None, None, None, None, "Decomposers (fungi, bacteria) break down dead organisms, recycling nutrients back into the ecosystem.", "easy"),
        ];
        for (question, qtype, correct, oa, ob, oc, od, explanation, difficulty) in &quizzes {
            conn.execute(
                "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10)",
                rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, explanation, difficulty],
            )?;
        }
    }

    // Extra quizzes for Economics (Supply & Demand)
    let tid_econ: Result<i64, _> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Supply & Demand'", [], |r| r.get(0),
    );
    if let Ok(tid) = tid_econ {
        let quizzes: Vec<QuizRowNoTopic<'_>> = vec![
            ("When demand increases and supply stays the same, price tends to ___.", "fill_in_blank", "increase", None, None, None, None, "Higher demand with fixed supply creates scarcity, pushing prices up.", "easy"),
            ("A price ceiling set below equilibrium causes a ___.", "fill_in_blank", "shortage", None, None, None, None, "Price ceilings below equilibrium make the good cheaper, increasing demand while reducing supply.", "medium"),
            ("The law of demand states that as price increases, quantity demanded ___.", "fill_in_blank", "decreases", None, None, None, None, "The law of demand: price and quantity demanded move in opposite directions (ceteris paribus).", "easy"),
            ("Which type of good sees demand increase when income rises?", "multiple_choice", "Normal good", Some("Inferior good"), Some("Normal good"), Some("Giffen good"), Some("Veblen good"), "Normal goods have a positive income elasticity — demand rises with income.", "medium"),
        ];
        for (question, qtype, correct, oa, ob, oc, od, explanation, difficulty) in &quizzes {
            conn.execute(
                "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10)",
                rusqlite::params![tid, question, qtype, correct, oa, ob, oc, od, explanation, difficulty],
            )?;
        }
    }

    Ok(())
}

pub fn seed_cybersecurity(conn: &Connection) -> Result<(), rusqlite::Error> {
    let has_topics: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM topics t JOIN subjects s ON t.subject_id = s.id WHERE s.name = 'Cybersecurity'",
        [], |r| r.get(0),
    ).unwrap_or(false);
    if has_topics { return Ok(()); }

    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('Cybersecurity', 'Protecting systems, networks, and data from digital attacks — understanding threats, defenses, and security principles.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Cybersecurity'", [], |r| r.get(0))?;

    let topics = [
        (subj_id, "Network Security Fundamentals", "beginner", 1),
        (subj_id, "Cryptography Basics", "intermediate", 2),
        (subj_id, "Common Attack Vectors", "intermediate", 3),
        (subj_id, "Authentication & Access Control", "beginner", 4),
        (subj_id, "Security Best Practices", "beginner", 5),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let net_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Network Security Fundamentals'", [subj_id], |r| r.get(0))?;
    let crypto_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Cryptography Basics'", [subj_id], |r| r.get(0))?;
    let attack_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Common Attack Vectors'", [subj_id], |r| r.get(0))?;
    let auth_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Authentication & Access Control'", [subj_id], |r| r.get(0))?;
    let best_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Security Best Practices'", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: &[LessonRow] = &[
        (net_id, "Network Security Basics", "Network security protects data as it travels between computers.\n\nKey concepts:\n- Firewall: filters incoming and outgoing network traffic based on rules.\n  - Packet filtering: inspects individual packets (source/dest IP, port).\n  - Stateful inspection: tracks active connections for context.\n  - Application-layer: understands protocols like HTTP.\n\n- Ports & Protocols:\n  - Port 80: HTTP (unencrypted web).\n  - Port 443: HTTPS (encrypted web).\n  - Port 22: SSH (secure remote access).\n  - Port 25: SMTP (email sending).\n\n- Defense in depth: multiple layers of security.\n  - Perimeter (firewall) → Network (IDS/IPS) → Host (antivirus) → Application → Data (encryption).\n\n- VPN (Virtual Private Network): creates an encrypted tunnel for data.\n- IDS/IPS: Intrusion Detection/Prevention Systems monitor for suspicious activity.\n\n- The CIA triad:\n  - Confidentiality: only authorized access.\n  - Integrity: data is accurate and unaltered.\n  - Availability: systems are accessible when needed.", 1),
        (net_id, "DNS, TLS, and Secure Communication", "DNS (Domain Name System) translates domain names to IP addresses.\n\nDNS attacks:\n- DNS spoofing/cache poisoning: attacker injects false DNS entries.\n- DNS tunneling: hides malicious data inside DNS queries.\n- DNSSEC: cryptographic signatures to verify DNS responses.\n\nTLS (Transport Layer Security):\n- Successor to SSL; encrypts data in transit.\n- TLS handshake: client hello → server hello + certificate → key exchange → encrypted session.\n- Certificate authorities (CAs) verify server identity.\n- HTTPS = HTTP + TLS.\n\nCommon issues:\n- Mixed content: loading HTTP resources on an HTTPS page.\n- Certificate pinning: app only trusts specific certificates.\n- Perfect forward secrecy (PFS): compromising one key doesn't expose past sessions.\n\nZero trust architecture: 'never trust, always verify' — every request is authenticated regardless of network location.", 2),
        (crypto_id, "Symmetric & Asymmetric Encryption", "Cryptography transforms readable data (plaintext) into unreadable form (ciphertext).\n\nSymmetric encryption: same key for encryption and decryption.\n- AES (Advanced Encryption Standard): most widely used. 128, 192, or 256-bit keys.\n- Fast, efficient for bulk data.\n- Problem: how do you safely share the key?\n\nAsymmetric encryption: two keys — public (encrypt) and private (decrypt).\n- RSA: based on difficulty of factoring large primes.\n- ECC (Elliptic Curve Cryptography): smaller keys, same security level.\n- Slower than symmetric, used for key exchange and digital signatures.\n\nHybrid approach (used in TLS):\n1. Asymmetric encryption to exchange a symmetric key.\n2. Symmetric encryption for the actual data transfer.\n\nHashing: one-way function producing a fixed-size digest.\n- SHA-256: produces a 256-bit hash. Used in Bitcoin, certificates.\n- Hashing is NOT encryption — you can't reverse it.\n- Salting: adding random data before hashing to prevent rainbow table attacks.\n- bcrypt/scrypt/Argon2: password hashing functions designed to be slow.", 1),
        (crypto_id, "Digital Signatures & PKI", "Digital signatures prove authenticity and integrity.\n\nHow they work:\n1. Sender hashes the message.\n2. Sender encrypts the hash with their private key = signature.\n3. Receiver decrypts the signature with the sender's public key.\n4. Receiver hashes the message independently and compares.\n\nPKI (Public Key Infrastructure):\n- Certificate Authority (CA): trusted entity that issues digital certificates.\n- Certificate: binds a public key to an identity.\n- Chain of trust: Root CA → Intermediate CA → End-entity certificate.\n- Certificate revocation: CRL (Certificate Revocation List) or OCSP.\n\nCommon uses:\n- HTTPS certificates (Let's Encrypt, DigiCert).\n- Code signing (proving software hasn't been tampered with).\n- Email signing (S/MIME, PGP).\n\nKey management challenges:\n- Key rotation: regularly changing keys.\n- Key escrow: storing backup keys securely.\n- Perfect forward secrecy: unique session keys so past sessions remain safe even if long-term key is compromised.", 2),
        (attack_id, "Social Engineering & Phishing", "Social engineering exploits human psychology rather than technical vulnerabilities.\n\nPhishing: fraudulent emails/messages designed to steal credentials or install malware.\n- Spear phishing: targeted at a specific person or organization.\n- Whaling: targets executives or high-value individuals.\n- Vishing: voice phishing (phone calls).\n- Smishing: SMS phishing.\n\nOther social engineering attacks:\n- Pretexting: creating a fabricated scenario to gain trust.\n- Baiting: offering something enticing (free USB drive with malware).\n- Tailgating/piggybacking: following someone through a secure door.\n- Quid pro quo: offering a service in exchange for information.\n\nDefenses:\n- Security awareness training.\n- Email filtering and DMARC/SPF/DKIM authentication.\n- Multi-factor authentication (even if password is stolen).\n- Verify requests through a separate channel.\n- Look for red flags: urgency, unusual sender, mismatched URLs.", 1),
        (attack_id, "Technical Attacks", "Common technical attack vectors:\n\nSQL Injection: inserting malicious SQL into input fields.\n- Example: username = ' OR 1=1 -- bypasses authentication.\n- Defense: parameterized queries, input validation, ORMs.\n\nCross-Site Scripting (XSS): injecting scripts into web pages.\n- Stored XSS: malicious script saved in database, served to all users.\n- Reflected XSS: script in URL parameters, reflected back.\n- Defense: output encoding, Content Security Policy (CSP).\n\nCross-Site Request Forgery (CSRF): tricking a user into making unintended requests.\n- Defense: CSRF tokens, SameSite cookies.\n\nDenial of Service (DoS/DDoS): overwhelming a system with traffic.\n- Volumetric: flood bandwidth.\n- Protocol: exploit protocol weaknesses (SYN flood).\n- Application: target specific application features.\n\nMan-in-the-Middle (MitM): intercepting communication between two parties.\n- Defense: TLS, certificate pinning, HSTS.\n\nZero-day: exploiting unknown vulnerabilities before patches exist.", 2),
        (auth_id, "Authentication Methods", "Authentication verifies identity ('you are who you claim to be').\n\nFactors of authentication:\n- Something you know: password, PIN, security question.\n- Something you have: phone, security key, smart card.\n- Something you are: fingerprint, face, iris (biometrics).\n\nMulti-Factor Authentication (MFA): combining 2+ factors.\n- 2FA examples: password + SMS code, password + authenticator app.\n- TOTP (Time-based One-Time Password): generates codes that change every 30 seconds.\n- FIDO2/WebAuthn: hardware security keys (YubiKey, passkeys).\n\nPassword security:\n- Length > complexity (a long passphrase beats P@$$w0rd).\n- Password managers: generate and store unique passwords.\n- Never reuse passwords across sites.\n- Check haveibeenpwned.com for breached credentials.\n\nSSO (Single Sign-On): one login for multiple services.\n- OAuth 2.0: authorization framework (lets apps access data without passwords).\n- OpenID Connect: identity layer on top of OAuth 2.0.\n- SAML: XML-based SSO standard (common in enterprises).", 1),
        (auth_id, "Access Control Models", "Authorization determines what an authenticated user can do.\n\nAccess control models:\n- DAC (Discretionary): resource owner decides who has access (Unix file permissions).\n- MAC (Mandatory): system-enforced labels (military classifications: Top Secret, Secret, etc.).\n- RBAC (Role-Based): permissions assigned to roles, users assigned to roles.\n  - Example: 'Editor' role can create/edit posts but not delete users.\n- ABAC (Attribute-Based): policies based on attributes (time of day, location, department).\n\nPrinciple of Least Privilege: users get minimum access needed for their job.\n\nSeparation of Duties: critical tasks require multiple people.\n- Example: one person approves expenses, another processes payment.\n\nPrivilege escalation:\n- Vertical: gaining higher privileges (user → admin).\n- Horizontal: accessing another user's resources at the same level.\n- Defense: regular privilege audits, just-in-time access.", 2),
        (best_id, "Security Hygiene", "Everyday security practices that prevent most attacks:\n\nSoftware updates: patch vulnerabilities promptly.\n- Most breaches exploit known, already-patched vulnerabilities.\n- Enable automatic updates where possible.\n\nBackup strategy (3-2-1 rule):\n- 3 copies of data.\n- 2 different storage media.\n- 1 offsite (cloud or physical).\n- Test restores regularly!\n\nEncryption at rest: encrypt stored data.\n- Full disk encryption: BitLocker (Windows), FileVault (macOS), LUKS (Linux).\n- Database encryption: transparent data encryption (TDE).\n\nSecure configuration:\n- Change default passwords and settings.\n- Disable unnecessary services and ports.\n- Use SSH keys instead of passwords.\n\nIncident response plan:\n1. Preparation: have a plan before incidents occur.\n2. Identification: detect and confirm the incident.\n3. Containment: limit the damage.\n4. Eradication: remove the threat.\n5. Recovery: restore systems.\n6. Lessons learned: improve defenses.", 1),
        (best_id, "Secure Development & DevSecOps", "Building security into software from the start.\n\nSecure coding principles:\n- Input validation: never trust user input.\n- Output encoding: prevent injection attacks.\n- Parameterized queries: prevent SQL injection.\n- Principle of least privilege in code: request minimum permissions.\n\nOWASP Top 10: most critical web application security risks.\n1. Broken access control.\n2. Cryptographic failures.\n3. Injection.\n4. Insecure design.\n5. Security misconfiguration.\n6. Vulnerable components.\n7. Authentication failures.\n8. Data integrity failures.\n9. Logging failures.\n10. Server-side request forgery (SSRF).\n\nDevSecOps: integrating security into the development pipeline.\n- SAST (Static Analysis): scan code for vulnerabilities.\n- DAST (Dynamic Analysis): test running applications.\n- SCA (Software Composition Analysis): check dependencies for known vulnerabilities.\n- Container scanning: check Docker images for vulnerabilities.\n\nSupply chain security: verify dependencies, use lock files, audit third-party code.", 2),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (net_id, "firewall", "A firewall is a network security device that monitors and filters incoming and outgoing traffic based on predefined rules.", Some("A firewall is like a bouncer at a club. The bouncer has a list of rules — dress code, guest list, age requirement. They check everyone at the door and only let in people who meet the criteria. Traffic that doesn't match the rules gets turned away."), Some("Why might a company need both a network firewall and a host-based firewall?")),
        (crypto_id, "symmetric encryption", "Symmetric encryption uses the same key for both encrypting and decrypting data — like a physical lock where the same key locks and unlocks.", Some("Imagine you and a friend share a diary that locks with a key. You both have a copy of the same key. The diary is secure as long as nobody else gets a copy of your key. That's the strength AND weakness of symmetric encryption — fast and simple, but key distribution is the challenge."), Some("If symmetric encryption is faster, why do we also need asymmetric encryption?")),
        (attack_id, "SQL injection", "SQL injection is an attack where malicious SQL code is inserted into input fields to manipulate a database — like slipping extra instructions into a bank withdrawal slip.", Some("Imagine a librarian who takes book requests by reading slips of paper aloud. If you write 'Give me Hamlet; also give me every book in the vault', a naive librarian just reads and executes everything. SQL injection works the same way — the database 'reads' malicious input as commands because it wasn't taught to distinguish data from instructions."), Some("What's the difference between parameterized queries and input sanitization?")),
        (auth_id, "multi-factor authentication", "MFA requires two or more verification factors — something you know, have, or are — making it dramatically harder for attackers.", Some("MFA is like a bank vault that needs both a combination AND a physical key. Even if a thief learns the combination (steals your password), they still can't open the vault without the physical key (your phone or security token). Each factor they need to steal independently multiplies the difficulty."), Some("Why are SMS codes considered weaker than authenticator apps for 2FA?")),
        (best_id, "defense in depth", "Defense in depth is a security strategy that uses multiple layers of protection, so if one layer fails, others still protect the system.", Some("Think of a medieval castle: moat, outer wall, inner wall, keep, armored guards. An attacker who crosses the moat still faces the wall. Breaching the wall still leaves the inner defenses. No single layer is perfect, but together they're formidable. Modern security works the same way — firewalls, encryption, access controls, monitoring, all layered together."), Some("Can you identify three different security layers in a typical web application?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let questions: &[QuizRow] = &[
        // Network Security
        (net_id, "What does the 'C' in the CIA triad stand for?", "multiple_choice", "Confidentiality", Some("Compliance"), Some("Confidentiality"), Some("Continuity"), Some("Certification"), Some("Who should have access?"), "The CIA triad: Confidentiality (authorized access only), Integrity (accurate data), Availability (accessible when needed)."),
        (net_id, "Which port is used by HTTPS?", "multiple_choice", "443", Some("80"), Some("443"), Some("22"), Some("25"), Some("Secure web traffic"), "Port 443 is the standard port for HTTPS (HTTP over TLS)."),
        (net_id, "True or false: A VPN encrypts all traffic between your device and the VPN server.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Virtual Private Network"), "True. A VPN creates an encrypted tunnel for all traffic between the client and VPN server."),
        (net_id, "The security approach 'never trust, always verify' is called ___.", "fill_in_blank", "zero trust", None, None, None, None, Some("No implicit trust based on network location"), "Zero trust architecture assumes no implicit trust — every request must be authenticated and authorized regardless of where it originates."),
        // Cryptography
        (crypto_id, "Which encryption type uses different keys for encryption and decryption?", "multiple_choice", "Asymmetric", Some("Symmetric"), Some("Asymmetric"), Some("Hashing"), Some("Encoding"), Some("Public and private keys"), "Asymmetric encryption uses a public key (encrypt) and private key (decrypt), unlike symmetric which uses the same key for both."),
        (crypto_id, "True or false: Hashing is reversible — you can recover the original data from a hash.", "true_false", "false", Some("true"), Some("false"), None, None, Some("One-way function"), "False. Hashing is a one-way function — you cannot recover the original input from the hash output."),
        (crypto_id, "Adding random data before hashing a password is called ___.", "fill_in_blank", "salting", None, None, None, None, Some("Prevents rainbow table attacks"), "Salting adds random data to each password before hashing, making precomputed attack tables (rainbow tables) useless."),
        (crypto_id, "What ensures past encrypted sessions remain safe even if a long-term key is compromised?", "multiple_choice", "Perfect forward secrecy", Some("Key escrow"), Some("Certificate pinning"), Some("Perfect forward secrecy"), Some("Key rotation"), Some("Unique session keys"), "Perfect forward secrecy (PFS) uses unique session keys so compromising the long-term key doesn't expose past sessions."),
        // Attack Vectors
        (attack_id, "What type of phishing specifically targets executives?", "multiple_choice", "Whaling", Some("Spear phishing"), Some("Whaling"), Some("Vishing"), Some("Smishing"), Some("Big fish"), "Whaling is phishing that specifically targets high-value individuals like C-level executives."),
        (attack_id, "True or false: SQL injection can be prevented by using parameterized queries.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Separate data from commands"), "True. Parameterized queries ensure user input is treated as data, not executable SQL commands."),
        (attack_id, "An attack that injects malicious scripts into web pages viewed by other users is called ___.", "fill_in_blank", "cross-site scripting", None, None, None, None, Some("XSS"), "Cross-Site Scripting (XSS) injects malicious scripts into web pages, which then execute in other users' browsers."),
        (attack_id, "Which attack overwhelms a system with more traffic than it can handle?", "multiple_choice", "DDoS", Some("SQL injection"), Some("Man-in-the-middle"), Some("DDoS"), Some("Phishing"), Some("Flood of requests"), "Distributed Denial of Service (DDoS) attacks overwhelm a target with traffic from many sources, making it unavailable."),
        // Authentication
        (auth_id, "Which is NOT a factor of authentication?", "multiple_choice", "Something you want", Some("Something you know"), Some("Something you have"), Some("Something you are"), Some("Something you want"), Some("Three recognized factors"), "The three authentication factors are: something you know (password), something you have (token), something you are (biometric). 'Something you want' is not a recognized factor."),
        (auth_id, "True or false: A long passphrase is generally more secure than a short complex password.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Length vs complexity"), "True. Length provides more entropy than complexity. 'correct horse battery staple' is stronger than 'P@s$1' and easier to remember."),
        (auth_id, "TOTP codes change every ___ seconds.", "fill_in_blank", "30", None, None, None, None, Some("Time-based One-Time Password"), "TOTP (Time-based One-Time Password) generates codes that change every 30 seconds, synchronized between the server and authenticator app."),
        (auth_id, "Which access control model assigns permissions based on user roles?", "multiple_choice", "RBAC", Some("DAC"), Some("MAC"), Some("RBAC"), Some("ABAC"), Some("Role-Based..."), "RBAC (Role-Based Access Control) assigns permissions to roles (e.g., 'editor', 'admin'), and users are assigned to roles."),
        // Best Practices
        (best_id, "What is the 3-2-1 backup rule?", "multiple_choice", "3 copies, 2 media types, 1 offsite", Some("3 backups daily, 2 weekly, 1 monthly"), Some("3 copies, 2 media types, 1 offsite"), Some("3 servers, 2 clouds, 1 local"), Some("3 passwords, 2 factors, 1 key"), Some("Redundancy strategy"), "The 3-2-1 rule: keep 3 copies of data on 2 different media types with 1 copy stored offsite."),
        (best_id, "True or false: Most breaches exploit unknown zero-day vulnerabilities.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Patch management matters"), "False. Most breaches exploit known, already-patched vulnerabilities. Keeping software updated prevents the majority of attacks."),
        (best_id, "The first item in the OWASP Top 10 (2021) is ___.", "fill_in_blank", "broken access control", None, None, None, None, Some("Number one web security risk"), "Broken Access Control moved to #1 in the OWASP Top 10 (2021), reflecting how common it is for applications to fail at enforcing proper authorization."),
        (best_id, "Which testing method analyzes source code for vulnerabilities without running the program?", "multiple_choice", "SAST", Some("DAST"), Some("SAST"), Some("SCA"), Some("Penetration testing"), Some("Static vs dynamic"), "SAST (Static Application Security Testing) analyzes source code or binaries without executing the program, finding vulnerabilities early in development."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    // Ordering question
    conn.execute(
        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
         VALUES (?1, 'Order the incident response steps:', 'ordering', 'Preparation,Identification,Containment,Eradication,Recovery,Lessons Learned', 'Containment', 'Recovery', 'Preparation', 'Identification', 'Plan first, then detect, then respond', 'Incident response follows: Preparation → Identification → Containment → Eradication → Recovery → Lessons Learned.')",
        [best_id],
    )?;

    // Learning paths
    let paths = [
        ("cybersecurity fundamentals", 1, net_id, "Network security — firewalls, protocols, and the CIA triad"),
        ("cybersecurity fundamentals", 2, auth_id, "Authentication & access control — proving identity and managing permissions"),
        ("cybersecurity fundamentals", 3, crypto_id, "Cryptography — encryption, hashing, and digital signatures"),
        ("cybersecurity fundamentals", 4, attack_id, "Attack vectors — understanding how systems are compromised"),
        ("cybersecurity fundamentals", 5, best_id, "Security best practices — defense in depth and secure development"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_discrete_mathematics(conn: &Connection) -> Result<(), rusqlite::Error> {
    let has_topics: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM topics t JOIN subjects s ON t.subject_id = s.id WHERE s.name = 'Discrete Mathematics'",
        [], |r| r.get(0),
    ).unwrap_or(false);
    if has_topics { return Ok(()); }

    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('Discrete Mathematics', 'The mathematics of countable structures — sets, graphs, combinatorics, number theory, and proof techniques that underpin computer science.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Discrete Mathematics'", [], |r| r.get(0))?;

    let topics = [
        (subj_id, "Sets & Logic", "beginner", 1),
        (subj_id, "Graph Theory", "intermediate", 2),
        (subj_id, "Combinatorics", "intermediate", 3),
        (subj_id, "Number Theory", "intermediate", 4),
        (subj_id, "Proof Techniques", "advanced", 5),
        (subj_id, "Recurrence Relations", "advanced", 6),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let sets_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Sets & Logic'", [subj_id], |r| r.get(0))?;
    let graph_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Graph Theory'", [subj_id], |r| r.get(0))?;
    let comb_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Combinatorics'", [subj_id], |r| r.get(0))?;
    let numth_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Number Theory'", [subj_id], |r| r.get(0))?;
    let proof_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Proof Techniques'", [subj_id], |r| r.get(0))?;
    let recur_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Recurrence Relations'", [subj_id], |r| r.get(0))?;

    // --- Lessons ---
    let lessons: &[LessonRow] = &[
        (sets_id, "Sets and Set Operations", "A **set** is an unordered collection of distinct objects.\n\nNotation: A = {1, 2, 3}, B = {2, 3, 4}\n\n**Operations:**\n- Union (A ∪ B): elements in A or B or both → {1, 2, 3, 4}\n- Intersection (A ∩ B): elements in both A and B → {2, 3}\n- Difference (A \\ B): elements in A but not B → {1}\n- Complement (A'): everything NOT in A (relative to a universal set)\n- Symmetric difference (A △ B): elements in exactly one of A or B → {1, 4}\n\n**Special sets:**\n- ∅ (empty set): contains no elements\n- ℕ = {0, 1, 2, ...} (natural numbers)\n- ℤ = {..., -2, -1, 0, 1, 2, ...} (integers)\n- |A| = cardinality (number of elements)\n\n**Power set** P(A): set of all subsets. If |A| = n, then |P(A)| = 2ⁿ.", 1),
        (sets_id, "Propositional Logic", "**Propositional logic** deals with statements that are true or false.\n\n**Connectives:**\n- ¬p (NOT): negation\n- p ∧ q (AND): both true\n- p ∨ q (OR): at least one true\n- p → q (IF-THEN): false only when p is true and q is false\n- p ↔ q (IFF): both same truth value\n\n**Truth tables** enumerate all possibilities.\n\n**Key equivalences:**\n- De Morgan: ¬(p ∧ q) ≡ ¬p ∨ ¬q\n- De Morgan: ¬(p ∨ q) ≡ ¬p ∧ ¬q\n- Contrapositive: (p → q) ≡ (¬q → ¬p)\n- Implication: (p → q) ≡ (¬p ∨ q)\n\n**Quantifiers:**\n- ∀x P(x): for all x, P(x) is true\n- ∃x P(x): there exists an x where P(x) is true", 2),
        (graph_id, "Introduction to Graphs", "A **graph** G = (V, E) consists of vertices (V) and edges (E).\n\n**Types:**\n- Undirected: edges have no direction (friendships)\n- Directed (digraph): edges have direction (Twitter follows)\n- Weighted: edges have values (road distances)\n\n**Terminology:**\n- Degree: number of edges at a vertex\n- Path: sequence of vertices connected by edges\n- Cycle: path that starts and ends at the same vertex\n- Connected: path exists between any two vertices\n- Complete graph Kn: every vertex connected to every other\n\n**Handshaking lemma:** The sum of all vertex degrees = 2 × |E|.\n(Every edge contributes to the degree of exactly two vertices.)\n\n**Adjacency matrix:** n×n matrix where entry (i,j) = 1 if edge exists.", 1),
        (graph_id, "Trees and Special Graphs", "A **tree** is a connected graph with no cycles.\n\nProperties:\n- n vertices → exactly n-1 edges\n- Exactly one path between any two vertices\n- Removing any edge disconnects the graph\n- Adding any edge creates exactly one cycle\n\n**Rooted trees:** one vertex designated as root. Used in file systems, HTML DOM, family trees.\n\n**Binary trees:** each node has at most 2 children.\n- Full: every node has 0 or 2 children\n- Complete: all levels filled except possibly the last\n- Height h tree has at most 2^(h+1) - 1 nodes\n\n**Spanning tree:** subgraph that includes all vertices and is a tree.\n- Minimum spanning tree (MST): spanning tree with minimum total edge weight.\n- Algorithms: Kruskal's (sort edges, add if no cycle) and Prim's (grow from a vertex).", 2),
        (comb_id, "Counting Principles", "**Fundamental counting:**\n- Addition principle: if task A has m ways and task B has n ways (mutually exclusive), total = m + n.\n- Multiplication principle: if task A has m ways AND task B has n ways, total = m × n.\n\n**Permutations** (order matters):\n- P(n,r) = n! / (n-r)! = ways to arrange r items from n\n- P(5,3) = 60 (e.g., gold/silver/bronze from 5 athletes)\n\n**Combinations** (order doesn't matter):\n- C(n,r) = n! / (r!(n-r)!) = ways to choose r items from n\n- C(5,3) = 10 (e.g., choosing a committee of 3 from 5)\n\n**Key identities:**\n- C(n,r) = C(n, n-r) — symmetry\n- C(n,0) = C(n,n) = 1\n- Sum of row n in Pascal's triangle = 2ⁿ\n- Pascal's rule: C(n,r) = C(n-1,r-1) + C(n-1,r)", 1),
        (comb_id, "Pigeonhole & Inclusion-Exclusion", "**Pigeonhole Principle:** If n+1 pigeons sit in n holes, at least one hole has ≥ 2 pigeons.\n\nExamples:\n- Among 13 people, at least 2 share a birth month.\n- In any group of 5 integers, at least 2 have the same remainder when divided by 4.\n\n**Generalized:** If n items go into k boxes, at least one box has ≥ ⌈n/k⌉ items.\n\n**Inclusion-Exclusion Principle:**\n|A ∪ B| = |A| + |B| - |A ∩ B|\n|A ∪ B ∪ C| = |A| + |B| + |C| - |A∩B| - |A∩C| - |B∩C| + |A∩B∩C|\n\nUsed to count elements in unions by alternately adding and subtracting intersections.\n\nExample: How many integers 1-100 are divisible by 2 or 3?\n|div2| = 50, |div3| = 33, |div6| = 16\nAnswer = 50 + 33 - 16 = 67.", 2),
        (numth_id, "Divisibility and Primes", "**Divisibility:** a divides b (a|b) if b = ka for some integer k.\n\n**Prime numbers:** only divisible by 1 and themselves.\n- 2 is the only even prime.\n- Fundamental Theorem of Arithmetic: every integer > 1 has a unique prime factorization.\n- There are infinitely many primes (Euclid's proof by contradiction).\n\n**GCD and LCM:**\n- GCD(a,b): greatest common divisor.\n- LCM(a,b): least common multiple.\n- GCD(a,b) × LCM(a,b) = a × b\n\n**Euclidean algorithm** for GCD:\n  GCD(252, 105): 252 = 2×105 + 42 → GCD(105,42)\n  105 = 2×42 + 21 → GCD(42,21)\n  42 = 2×21 + 0 → GCD = 21\n\n**Sieve of Eratosthenes:** systematically find all primes up to n by crossing out multiples.", 1),
        (numth_id, "Modular Arithmetic", "**Modular arithmetic** is 'clock arithmetic.'\n\na ≡ b (mod n) means n divides (a - b).\n17 ≡ 2 (mod 5) because 17 - 2 = 15 is divisible by 5.\n\n**Properties:**\n- (a + b) mod n = ((a mod n) + (b mod n)) mod n\n- (a × b) mod n = ((a mod n) × (b mod n)) mod n\n- (aᵏ) mod n can be computed efficiently via repeated squaring\n\n**Fermat's Little Theorem:** If p is prime and gcd(a,p) = 1:\n  aᵖ⁻¹ ≡ 1 (mod p)\n\n**Applications:**\n- Cryptography (RSA relies on modular exponentiation)\n- Hash functions\n- Check digits (ISBN, credit cards)\n- Day-of-week calculations", 2),
        (proof_id, "Proof Techniques", "**Direct proof:** Assume premises, derive conclusion step by step.\n  'If n is even, then n² is even.'\n  Proof: n = 2k → n² = 4k² = 2(2k²), which is even. ∎\n\n**Proof by contradiction:** Assume the negation, derive a contradiction.\n  '√2 is irrational.'\n  Assume √2 = p/q (fully reduced). Then 2q² = p², so p² is even → p is even.\n  Let p = 2m. Then 2q² = 4m² → q² = 2m² → q is even.\n  But p and q both even contradicts 'fully reduced.' ∎\n\n**Proof by contrapositive:** Prove ¬q → ¬p instead of p → q.\n  'If n² is odd, then n is odd.'\n  Contrapositive: 'If n is even, then n² is even.' (Easy to prove directly.)\n\n**Proof by cases:** Split into exhaustive cases, prove each.\n  'For all integers n, n² + n is even.'\n  Case 1: n even → n² + n = even + even = even.\n  Case 2: n odd → n² + n = odd + odd = even. ∎", 1),
        (proof_id, "Mathematical Induction", "**Mathematical induction** proves statements for all natural numbers.\n\n**Structure:**\n1. **Base case:** Prove P(0) or P(1).\n2. **Inductive step:** Assume P(k) (inductive hypothesis). Prove P(k+1).\n3. Conclude: P(n) holds for all n ≥ base.\n\n**Example:** Prove 1 + 2 + ... + n = n(n+1)/2.\nBase: n=1 → 1 = 1(2)/2 = 1. ✓\nInductive step: Assume 1+...+k = k(k+1)/2.\n  Then 1+...+k+(k+1) = k(k+1)/2 + (k+1) = (k+1)(k+2)/2. ✓\n\n**Strong induction:** Assume P(1), P(2), ..., P(k) to prove P(k+1).\nUseful when P(k+1) depends on multiple predecessors.\n\n**Structural induction:** For recursively defined structures (trees, formulas).\nBase: prove for base structures. Step: prove for composite structures assuming sub-structures satisfy the property.", 2),
        (recur_id, "Solving Recurrence Relations", "A **recurrence relation** defines a sequence where each term depends on previous terms.\n\n**Examples:**\n- Fibonacci: F(n) = F(n-1) + F(n-2), F(0)=0, F(1)=1\n- Tower of Hanoi: T(n) = 2T(n-1) + 1, T(1)=1\n\n**Methods:**\n1. **Substitution:** Guess and verify by induction.\n2. **Iteration (unrolling):** Expand until a pattern emerges.\n3. **Characteristic equation (for linear recurrences):**\n   aₙ = c₁aₙ₋₁ + c₂aₙ₋₂ → solve t² = c₁t + c₂\n   Roots r₁, r₂ → aₙ = A·r₁ⁿ + B·r₂ⁿ\n   (Use initial conditions to find A, B.)\n\nFibonacci: t² = t + 1 → roots (1±√5)/2\nClosed form: F(n) = (φⁿ - ψⁿ)/√5 where φ = (1+√5)/2 ≈ 1.618 (golden ratio).\n\n**Master Theorem** for divide-and-conquer recurrences:\nT(n) = aT(n/b) + f(n) → three cases depending on how f(n) compares to n^(log_b a).", 1),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // --- Explanations ---
    let explanations: &[ExplanationRow] = &[
        (sets_id, "De Morgan's Laws", "De Morgan's Laws relate AND, OR, and NOT:\n¬(p ∧ q) ≡ ¬p ∨ ¬q — 'not (both)' means 'not one or not the other'\n¬(p ∨ q) ≡ ¬p ∧ ¬q — 'not (either)' means 'not one and not the other'\nThey work for sets too: (A ∩ B)' = A' ∪ B'", Some("Think of a club where you must be tall AND rich to enter. Saying 'you CAN'T enter' means you're NOT tall OR NOT rich."), Some("How are De Morgan's Laws used in programming (e.g., simplifying if-statements)?")),
        (graph_id, "Handshaking Lemma", "The sum of all vertex degrees in a graph equals twice the number of edges. Each edge contributes 1 to the degree of each of its endpoints, so it's counted exactly twice.", Some("If everyone at a party shakes hands, and you count how many handshakes each person did, the total count is exactly double the number of handshakes — because each handshake involves two people."), Some("What does the Handshaking Lemma tell us about the number of vertices with odd degree?")),
        (comb_id, "Pigeonhole Principle", "If you put more items into containers than there are containers, at least one container must have more than one item. Simple but surprisingly powerful for existence proofs.", Some("If you have 13 socks and 12 drawers, at least one drawer has ≥ 2 socks. You don't know WHICH drawer, but you know it exists."), Some("Can you prove that in any group of 6 people, at least 3 are mutual friends or at least 3 are mutual strangers?")),
        (numth_id, "Euclidean Algorithm", "An efficient method for computing GCD. Repeatedly replace the larger number with the remainder of dividing the two. The last non-zero remainder is the GCD. Runs in O(log(min(a,b))) steps.", Some("Like measuring with a ruler that's too short: lay it end to end, and the leftover tells you to try a shorter ruler. Eventually you find a length that fits perfectly."), Some("Why does the Euclidean algorithm always terminate?")),
        (proof_id, "Proof by Contradiction", "Assume the opposite of what you want to prove, then show this leads to a logical impossibility. Since the assumption leads to nonsense, the original statement must be true.", Some("Like proving you locked the door by assuming you didn't — then noticing the door is still locked. The assumption is impossible, so you must have locked it."), Some("What's the difference between proof by contradiction and proof by contrapositive?")),
        (recur_id, "The Golden Ratio", "φ = (1+√5)/2 ≈ 1.618 appears in the closed-form solution of the Fibonacci sequence. It's the positive root of x² = x + 1. The ratio of consecutive Fibonacci numbers converges to φ. It appears in nature, art, and architecture.", Some("The golden ratio is mathematics' most recurring celebrity — it shows up everywhere from sunflower spirals to the Parthenon's proportions to stock market analysis."), Some("Why does F(n+1)/F(n) converge to the golden ratio?")),
    ];
    for (tid, concept, expl, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, follow_up],
        )?;
    }

    // --- Quiz Questions ---
    let quizzes: &[QuizRow] = &[
        // Sets & Logic
        (sets_id, "If A = {1,2,3} and B = {2,3,4}, what is A ∪ B?", "multiple_choice", "{1,2,3,4}", Some("{1,2,3,4}"), Some("{2,3}"), Some("{1,4}"), Some("{1,2,3}"), None, "Union includes all elements from both sets: {1,2,3} ∪ {2,3,4} = {1,2,3,4}."),
        (sets_id, "If |A| = 4, how many elements does the power set P(A) have?", "multiple_choice", "16", Some("4"), Some("8"), Some("16"), Some("32"), Some("2 raised to what power?"), "The power set of a set with n elements has 2ⁿ elements. 2⁴ = 16."),
        (sets_id, "De Morgan's Law states: ¬(p ∧ q) ≡", "multiple_choice", "¬p ∨ ¬q", Some("¬p ∧ ¬q"), Some("¬p ∨ ¬q"), Some("p ∨ q"), Some("¬p → ¬q"), None, "De Morgan: NOT (p AND q) = (NOT p) OR (NOT q)."),
        (sets_id, "The contrapositive of 'if p then q' is:", "multiple_choice", "if ¬q then ¬p", Some("if q then p"), Some("if ¬p then ¬q"), Some("if ¬q then ¬p"), Some("if p then ¬q"), None, "The contrapositive reverses and negates: (p → q) ≡ (¬q → ¬p). It's always logically equivalent."),
        (sets_id, "True or false: The empty set is a subset of every set.", "true_false", "true", Some("true"), Some("false"), None, None, None, "The empty set ∅ is a subset of every set — the statement '∀x ∈ ∅, x ∈ A' is vacuously true."),
        // Graph Theory
        (graph_id, "A tree with 10 vertices has ___ edges.", "fill_in_blank", "9", None, None, None, None, Some("n vertices → n-? edges"), "A tree with n vertices always has exactly n-1 edges. So 10 vertices → 9 edges."),
        (graph_id, "The sum of all vertex degrees in any graph equals:", "multiple_choice", "Twice the number of edges", Some("The number of edges"), Some("Twice the number of edges"), Some("The number of vertices"), Some("Three times the number of edges"), None, "The Handshaking Lemma: Σ deg(v) = 2|E|. Each edge contributes 2 to the total degree."),
        (graph_id, "In a complete graph K₅, how many edges are there?", "multiple_choice", "10", Some("5"), Some("10"), Some("15"), Some("20"), Some("C(n,2) for Kn"), "K₅ has C(5,2) = 10 edges. Every pair of vertices is connected."),
        (graph_id, "True or false: Every tree is a connected graph.", "true_false", "true", Some("true"), Some("false"), None, None, None, "By definition, a tree is a connected acyclic graph. If it weren't connected, it would be a forest."),
        // Combinatorics
        (comb_id, "How many ways can 5 people finish 1st, 2nd, 3rd in a race?", "multiple_choice", "60", Some("10"), Some("30"), Some("60"), Some("120"), Some("P(5,3) = ?"), "P(5,3) = 5!/(5-3)! = 5×4×3 = 60. Order matters (it's a permutation)."),
        (comb_id, "C(10,3) = ___", "fill_in_blank", "120", None, None, None, None, Some("10! / (3! × 7!)"), "C(10,3) = 10!/(3! × 7!) = (10×9×8)/(3×2×1) = 720/6 = 120."),
        (comb_id, "True or false: C(n,r) = C(n, n-r).", "true_false", "true", Some("true"), Some("false"), None, None, None, "Choosing r items to include is the same as choosing n-r items to exclude. This symmetry is a key property."),
        (comb_id, "Among 13 people, at least ___ share a birth month.", "fill_in_blank", "2", None, None, None, None, Some("Pigeonhole principle: 13 people, 12 months"), "By the Pigeonhole Principle: 13 people in 12 months means at least ⌈13/12⌉ = 2 share a month."),
        // Number Theory
        (numth_id, "GCD(48, 18) = ___", "fill_in_blank", "6", None, None, None, None, Some("Use the Euclidean algorithm"), "48 = 2×18 + 12, then 18 = 1×12 + 6, then 12 = 2×6 + 0. GCD = 6."),
        (numth_id, "The only even prime number is:", "fill_in_blank", "2", None, None, None, None, Some("All other even numbers are divisible by..."), "2 is the only even prime. Every other even number is divisible by 2, hence not prime."),
        (numth_id, "17 mod 5 = ___", "fill_in_blank", "2", None, None, None, None, Some("17 = 3×5 + ?"), "17 = 3×5 + 2, so 17 mod 5 = 2."),
        (numth_id, "True or false: Every integer greater than 1 has a unique prime factorization.", "true_false", "true", Some("true"), Some("false"), None, None, None, "This is the Fundamental Theorem of Arithmetic — every integer > 1 can be expressed as a product of primes in exactly one way (up to order)."),
        // Proof Techniques
        (proof_id, "To prove P(n) for all n by induction, you need:", "multiple_choice", "A base case and an inductive step", Some("Just a base case"), Some("A base case and an inductive step"), Some("Just an inductive step"), Some("A counterexample"), None, "Mathematical induction requires: (1) proving the base case and (2) proving that if P(k) holds, then P(k+1) holds."),
        (proof_id, "Proof by contradiction starts by:", "multiple_choice", "Assuming the negation of the statement", Some("Proving the statement directly"), Some("Assuming the negation of the statement"), Some("Finding a counterexample"), Some("Using induction"), None, "Proof by contradiction assumes ¬P (the opposite of what you want to prove) and derives a logical contradiction."),
        (proof_id, "True or false: Strong induction assumes P(1) through P(k) to prove P(k+1).", "true_false", "true", Some("true"), Some("false"), None, None, None, "Strong induction uses all prior cases P(1)...P(k) in the inductive step, unlike ordinary induction which only uses P(k)."),
        // Recurrence Relations
        (recur_id, "The Fibonacci recurrence is F(n) = F(n-1) + F(n-2). What is F(7)?", "multiple_choice", "13", Some("8"), Some("11"), Some("13"), Some("21"), Some("F(0)=0, F(1)=1, compute step by step"), "F: 0,1,1,2,3,5,8,13. So F(7) = 13."),
        (recur_id, "The golden ratio φ ≈ ___", "fill_in_blank", "1.618", None, None, None, None, Some("(1+√5)/2"), "φ = (1+√5)/2 ≈ 1.618033... It's the positive root of x² = x + 1."),
        (recur_id, "The Tower of Hanoi with 4 disks requires ___ moves.", "fill_in_blank", "15", None, None, None, None, Some("T(n) = 2ⁿ - 1"), "T(n) = 2ⁿ - 1. For n=4: 2⁴ - 1 = 15 moves."),
    ];
    for (tid, question, qtype, answer, a, b, c, d, hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, question, qtype, answer, a, b, c, d, hint, expl],
        )?;
    }

    // --- Learning Path ---
    let path_steps: &[(i64, &str)] = &[
        (sets_id, "Start with sets and propositional logic — the language of discrete math"),
        (proof_id, "Learn proof techniques — the tools you'll use everywhere"),
        (numth_id, "Explore number theory — primes, divisibility, and modular arithmetic"),
        (comb_id, "Master combinatorics — counting, permutations, and combinations"),
        (graph_id, "Study graph theory — vertices, edges, trees, and algorithms"),
        (recur_id, "Advanced: solve recurrence relations and analyze algorithms"),
    ];
    for (i, (tid, desc)) in path_steps.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Discrete Mathematics Foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_linear_algebra(conn: &Connection) -> Result<(), rusqlite::Error> {
    let la_id: i64 = conn.query_row(
        "INSERT INTO subjects (name, description) VALUES ('Linear Algebra', 'The mathematics of vectors, matrices, and linear transformations — essential for computer graphics, machine learning, and engineering.') RETURNING id",
        [], |r| r.get(0),
    )?;

    // Topics
    let vectors_id: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Vectors & Vector Spaces', 'beginner', 1) RETURNING id",
        [la_id], |r| r.get(0),
    )?;
    let matrices_id: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Matrices & Operations', 'beginner', 2) RETURNING id",
        [la_id], |r| r.get(0),
    )?;
    let systems_id: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Systems of Linear Equations', 'intermediate', 3) RETURNING id",
        [la_id], |r| r.get(0),
    )?;
    let determinants_id: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Determinants', 'intermediate', 4) RETURNING id",
        [la_id], |r| r.get(0),
    )?;
    let eigen_id: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Eigenvalues & Eigenvectors', 'advanced', 5) RETURNING id",
        [la_id], |r| r.get(0),
    )?;
    let transforms_id: i64 = conn.query_row(
        "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, 'Linear Transformations', 'advanced', 6) RETURNING id",
        [la_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: &[LessonRow] = &[
        (vectors_id, "What Are Vectors?",
         "A vector is a quantity with both magnitude and direction. In linear algebra, vectors are ordered lists of numbers (components). A 2D vector like (3, 4) points 3 units right and 4 units up.\n\nVectors can be added component-wise: (1, 2) + (3, 4) = (4, 6). Scalar multiplication scales each component: 2 × (3, 4) = (6, 8).\n\nA vector space is a collection of vectors that is closed under addition and scalar multiplication. The standard vector spaces ℝ² (2D) and ℝ³ (3D) are the most common examples.", 1),
        (vectors_id, "Dot Product & Orthogonality",
         "The dot product of two vectors a·b = a₁b₁ + a₂b₂ + ... + aₙbₙ. It measures how much two vectors point in the same direction.\n\nKey properties: a·b = |a||b|cos(θ), where θ is the angle between them. If a·b = 0, the vectors are orthogonal (perpendicular).\n\nThe dot product gives us vector length: |a| = √(a·a). This is the Euclidean norm.", 2),
        (matrices_id, "Introduction to Matrices",
         "A matrix is a rectangular array of numbers arranged in rows and columns. An m×n matrix has m rows and n columns.\n\nMatrix addition works element-wise (same dimensions required). Scalar multiplication multiplies every entry by a scalar.\n\nThe identity matrix I has 1s on the diagonal and 0s elsewhere. For any matrix A: AI = IA = A.", 1),
        (matrices_id, "Matrix Multiplication",
         "To multiply matrices A (m×n) and B (n×p), the result C is m×p where C[i,j] = sum of A[i,k]×B[k,j] for k=1..n.\n\nKey rule: the number of columns in A must equal the number of rows in B.\n\nMatrix multiplication is NOT commutative: AB ≠ BA in general. But it IS associative: (AB)C = A(BC).", 2),
        (systems_id, "Solving Linear Systems",
         "A system of linear equations can be written as Ax = b, where A is the coefficient matrix, x is the unknown vector, and b is the constants vector.\n\nGaussian elimination transforms the augmented matrix [A|b] into row echelon form using three operations: swap rows, multiply a row by a nonzero scalar, add a multiple of one row to another.\n\nA system has a unique solution when the coefficient matrix has full rank, no solution when the system is inconsistent, or infinitely many solutions when there are free variables.", 1),
        (determinants_id, "What Are Determinants?",
         "The determinant is a scalar value computed from a square matrix that encodes important geometric and algebraic information.\n\nFor a 2×2 matrix [[a,b],[c,d]], det = ad - bc. For larger matrices, use cofactor expansion along any row or column.\n\nGeometric meaning: |det(A)| equals the factor by which A scales areas (2D) or volumes (3D). If det(A) < 0, the transformation reverses orientation. If det(A) = 0, the matrix is singular (not invertible).", 1),
        (eigen_id, "Eigenvalues and Eigenvectors",
         "An eigenvector v of matrix A satisfies Av = λv, where λ is the eigenvalue. The eigenvector's direction is unchanged by the transformation — only its magnitude is scaled by λ.\n\nTo find eigenvalues: solve det(A - λI) = 0 (the characteristic equation). Then for each λ, solve (A - λI)v = 0 to find eigenvectors.\n\nEigenvalues reveal the stretching factors of a linear transformation. They are crucial in stability analysis, PCA (principal component analysis), Google's PageRank, and quantum mechanics.", 1),
        (transforms_id, "Linear Transformations",
         "A linear transformation T: V → W satisfies T(u+v) = T(u)+T(v) and T(cv) = cT(v). Every linear transformation between finite-dimensional spaces can be represented by a matrix.\n\nCommon 2D transformations: rotation by θ uses matrix [[cos θ, -sin θ], [sin θ, cos θ]]; reflection across x-axis uses [[1,0],[0,-1]]; scaling uses [[sx,0],[0,sy]].\n\nThe kernel (null space) of T is the set of vectors mapped to zero. The image (range) is the set of all outputs. The rank-nullity theorem states: dim(kernel) + dim(image) = dim(domain).", 1),
    ];

    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (vectors_id, "Vector Addition", "Adding vectors means combining their components: (a₁+b₁, a₂+b₂). Geometrically, place one vector's tail at the other's head.",
         Some("Think of walking: go 3 blocks east then 4 blocks north. Your total displacement is the vector (3,4). Adding another walk (1,2) means you end up at (4,6)."),
         Some("If v = (2, -1) and w = (3, 5), what is v + w?")),
        (matrices_id, "Matrix Inverse", "The inverse A⁻¹ satisfies AA⁻¹ = A⁻¹A = I. Only square matrices with nonzero determinant have inverses.",
         Some("An inverse is like an undo button. If matrix A rotates 90° clockwise, A⁻¹ rotates 90° counterclockwise."),
         Some("Why can't a matrix with determinant 0 be inverted?")),
        (eigen_id, "Characteristic Equation", "det(A - λI) = 0 is a polynomial in λ. Its degree equals the matrix size. The roots are the eigenvalues.",
         Some("Finding eigenvalues is like finding the resonant frequencies of a vibrating string — they're the natural modes of the system."),
         Some("What is the characteristic equation of [[2,1],[0,3]]?")),
    ];

    for (tid, concept, expl, analogy, followup) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // Quiz questions — Vectors
    let vector_qs: &[QuizRowNoTopic] = &[
        ("What is the dot product of (1, 2, 3) and (4, 5, 6)?", "fill_in_blank", "32", None, None, None, None,
         "Think: 1×4 + 2×5 + 3×6", "1×4 + 2×5 + 3×6 = 4 + 10 + 18 = 32"),
        ("Two vectors are orthogonal when their dot product equals ___", "fill_in_blank", "0", None, None, None, None,
         "Perpendicular vectors have no component in the same direction", "Orthogonal means perpendicular. The dot product a·b = |a||b|cos(90°) = 0."),
        ("What is the magnitude of vector (3, 4)?", "fill_in_blank", "5", None, None, None, None,
         "Use the Pythagorean theorem", "√(3² + 4²) = √(9+16) = √25 = 5"),
        ("Which operation is NOT valid for vectors?", "multiple_choice", "Division of two vectors",
         Some("Addition"), Some("Scalar multiplication"), Some("Dot product"), Some("Division of two vectors"),
         "There is no standard vector division", "Vector division is not defined. You can add, scale, and take dot/cross products, but not divide vectors."),
        ("What is 3 × (2, -1)?", "fill_in_blank", "(6, -3)", None, None, None, None,
         "Multiply each component by the scalar", "Scalar multiplication: 3×2=6, 3×(-1)=-3, giving (6, -3)."),
        ("True or false: The zero vector is in every vector space.", "true_false", "true", Some("True"), Some("False"), None, None,
         "Think about what closure under scalar multiplication implies", "Multiplying any vector by 0 gives the zero vector, so every vector space must contain it."),
    ];

    for (q, qt, ans, a, b, c, d, hint, expl) in vector_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![vectors_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }

    // Quiz questions — Matrices
    let matrix_qs: &[QuizRowNoTopic] = &[
        ("What is the size of the product of a 3×2 matrix and a 2×5 matrix?", "fill_in_blank", "3x5", None, None, None, None,
         "The result has as many rows as the first matrix and as many columns as the second",
         "(m×n)(n×p) = m×p, so 3×5."),
        ("True or false: Matrix multiplication is commutative.", "true_false", "false", Some("True"), Some("False"), None, None,
         "Try multiplying two small matrices in both orders", "In general AB ≠ BA. Matrix multiplication is associative but NOT commutative."),
        ("What is the identity matrix for 2×2 matrices?", "multiple_choice", "[[1,0],[0,1]]",
         Some("[[1,1],[1,1]]"), Some("[[1,0],[0,1]]"), Some("[[0,1],[1,0]]"), Some("[[1,0],[0,0]]"),
         "The identity has 1s on the diagonal", "The 2×2 identity matrix I = [[1,0],[0,1]] satisfies AI = IA = A for any 2×2 matrix A."),
        ("How many elements does a 4×3 matrix contain?", "fill_in_blank", "12", None, None, None, None,
         "Count rows × columns", "A 4×3 matrix has 4 rows and 3 columns = 12 elements total."),
        ("The transpose of a matrix swaps its ___ and ___.", "fill_in_blank", "rows and columns", None, None, None, None,
         "The (i,j) entry becomes the (j,i) entry", "Transposing a matrix reflects it across the main diagonal, swapping rows and columns."),
        ("Can you multiply a 2×3 matrix by a 4×2 matrix?", "true_false", "false", Some("True"), Some("False"), None, None,
         "Check: columns of first must equal rows of second", "No. The first matrix has 3 columns but the second has 4 rows. They must match."),
    ];

    for (q, qt, ans, a, b, c, d, hint, expl) in matrix_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![matrices_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }

    // Quiz questions — Systems
    let system_qs: &[QuizRowNoTopic] = &[
        ("In Gaussian elimination, which operation is NOT allowed?", "multiple_choice", "Multiply a row by zero",
         Some("Swap two rows"), Some("Add a multiple of one row to another"), Some("Multiply a row by a nonzero scalar"), Some("Multiply a row by zero"),
         "All three elementary row operations preserve the solution set", "Multiplying a row by zero destroys information and is not a valid row operation."),
        ("A system Ax = b has no solution when the system is ___.", "fill_in_blank", "inconsistent", None, None, None, None,
         "What do we call contradictory equations?", "An inconsistent system has contradictory equations (e.g., 0 = 5), meaning no solution exists."),
        ("What does 'full rank' mean for an m×n matrix?", "multiple_choice", "rank = min(m, n)",
         Some("rank = 0"), Some("rank = max(m, n)"), Some("rank = min(m, n)"), Some("rank = m + n"),
         "Rank counts linearly independent rows or columns", "Full rank means the rank equals the smaller of m and n — all possible rows/columns are independent."),
        ("True or false: A homogeneous system Ax = 0 always has at least one solution.", "true_false", "true",
         Some("True"), Some("False"), None, None,
         "What happens when you plug in x = 0?", "x = 0 (the trivial solution) always satisfies Ax = 0."),
        ("Row echelon form requires all entries ___ a pivot to be zero.", "fill_in_blank", "below", None, None, None, None,
         "Think about the staircase pattern", "In row echelon form, each pivot is to the right of the one above, and all entries below each pivot are zero."),
    ];

    for (q, qt, ans, a, b, c, d, hint, expl) in system_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![systems_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }

    // Quiz questions — Determinants
    let det_qs: &[QuizRowNoTopic] = &[
        ("What is det([[3, 1], [2, 4]])?", "fill_in_blank", "10", None, None, None, None,
         "For 2×2: ad - bc", "det = 3×4 - 1×2 = 12 - 2 = 10."),
        ("If det(A) = 0, then A is ___.", "fill_in_blank", "singular", None, None, None, None,
         "What do we call a matrix that cannot be inverted?", "A singular matrix has determinant 0 and is not invertible."),
        ("True or false: det(AB) = det(A) × det(B).", "true_false", "true", Some("True"), Some("False"), None, None,
         "This is a fundamental property of determinants", "The determinant is multiplicative: det(AB) = det(A)·det(B) for square matrices of the same size."),
        ("What is det([[1, 0], [0, 1]])?", "fill_in_blank", "1", None, None, None, None,
         "This is the identity matrix", "det(I) = 1×1 - 0×0 = 1. The identity always has determinant 1."),
        ("Swapping two rows of a matrix ___ the sign of the determinant.", "fill_in_blank", "changes", None, None, None, None,
         "Row swap affects orientation", "Each row swap negates the determinant. Two swaps return it to the original sign."),
    ];

    for (q, qt, ans, a, b, c, d, hint, expl) in det_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![determinants_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }

    // Quiz questions — Eigenvalues
    let eigen_qs: &[QuizRowNoTopic] = &[
        ("If Av = 3v, then 3 is a(n) ___ of A.", "fill_in_blank", "eigenvalue", None, None, None, None,
         "What scalar satisfies Av = λv?", "In Av = λv, the scalar λ = 3 is an eigenvalue and v is the corresponding eigenvector."),
        ("What equation do we solve to find eigenvalues?", "multiple_choice", "det(A - λI) = 0",
         Some("det(A + λI) = 0"), Some("det(A - λI) = 0"), Some("det(A) = λ"), Some("Av = b"),
         "Set up the characteristic equation", "The characteristic equation det(A - λI) = 0 yields a polynomial whose roots are the eigenvalues."),
        ("How many eigenvalues (counted with multiplicity) does a 3×3 matrix have?", "fill_in_blank", "3", None, None, None, None,
         "The characteristic polynomial has degree n for an n×n matrix", "A 3×3 matrix has a degree-3 characteristic polynomial, so exactly 3 eigenvalues (counting multiplicity, possibly complex)."),
        ("True or false: Eigenvectors corresponding to distinct eigenvalues are linearly independent.", "true_false", "true",
         Some("True"), Some("False"), None, None,
         "This is a key theorem in linear algebra", "Eigenvectors for distinct eigenvalues are always linearly independent. This is fundamental for diagonalization."),
        ("If A has eigenvalues 2 and 5, what is det(A)?", "fill_in_blank", "10", None, None, None, None,
         "The determinant equals the product of all eigenvalues", "det(A) = product of eigenvalues = 2 × 5 = 10."),
    ];

    for (q, qt, ans, a, b, c, d, hint, expl) in eigen_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![eigen_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }

    // Quiz questions — Transformations
    let transform_qs: &[QuizRowNoTopic] = &[
        ("A linear transformation must preserve ___ and scalar multiplication.", "fill_in_blank", "addition", None, None, None, None,
         "T(u+v) = T(u) + T(v) is one condition", "Linearity means T(u+v) = T(u)+T(v) and T(cv) = cT(v) — it preserves addition and scalar multiplication."),
        ("The kernel of a transformation is the set of vectors mapped to ___.", "fill_in_blank", "zero", None, None, None, None,
         "Also called the null space", "The kernel (null space) = {v : T(v) = 0}. It measures how much information the transformation 'loses'."),
        ("The rank-nullity theorem states: dim(kernel) + dim(image) = ___.", "fill_in_blank", "dim(domain)", None, None, None, None,
         "The dimensions must add up", "Rank-nullity: nullity + rank = dimension of the domain. It's a conservation law for dimensions."),
        ("Which matrix represents a 90° counter-clockwise rotation in 2D?", "multiple_choice", "[[0,-1],[1,0]]",
         Some("[[1,0],[0,1]]"), Some("[[0,-1],[1,0]]"), Some("[[0,1],[-1,0]]"), Some("[[-1,0],[0,-1]]"),
         "Apply to (1,0): it should go to (0,1)", "cos(90°)=0, sin(90°)=1, so the rotation matrix is [[0,-1],[1,0]]."),
        ("True or false: Every matrix represents a linear transformation.", "true_false", "true", Some("True"), Some("False"), None, None,
         "Consider what multiplication by a matrix does to vectors", "Yes. Multiplying by a matrix is always a linear transformation (it satisfies both linearity conditions)."),
    ];

    for (q, qt, ans, a, b, c, d, hint, expl) in transform_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![transforms_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path
    let path_steps: &[(i64, &str)] = &[
        (vectors_id, "Start with vectors — the fundamental objects of linear algebra"),
        (matrices_id, "Learn matrices — the computational workhorses"),
        (systems_id, "Master solving systems of equations with Gaussian elimination"),
        (determinants_id, "Understand determinants — key to invertibility and volume"),
        (eigen_id, "Explore eigenvalues — the 'DNA' of a matrix"),
        (transforms_id, "Advanced: see matrices as geometric transformations"),
    ];
    for (i, (tid, desc)) in path_steps.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Linear Algebra Foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_electrical_engineering(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Subject
    conn.execute(
        "INSERT OR IGNORE INTO subjects (name, description) VALUES ('Electrical Engineering', 'Circuits, signals, digital logic, and the principles that power modern technology.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Electrical Engineering'", [], |r| r.get(0),
    )?;

    // Topics
    let topics: &[(&str, &str, i64)] = &[
        ("Circuit Fundamentals", "beginner", 1),
        ("Ohm's Law & Kirchhoff's Laws", "beginner", 2),
        ("Capacitors & Inductors", "intermediate", 3),
        ("AC Circuit Analysis", "intermediate", 4),
        ("Digital Logic Gates", "beginner", 5),
        ("Semiconductor Basics", "intermediate", 6),
    ];
    for (name, diff, order) in topics {
        conn.execute(
            "INSERT OR IGNORE INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let circuit_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Circuit Fundamentals' AND subject_id = ?1", [subj_id], |r| r.get(0),
    )?;
    let ohm_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Ohm''s Law & Kirchhoff''s Laws' AND subject_id = ?1", [subj_id], |r| r.get(0),
    )?;
    let cap_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Capacitors & Inductors' AND subject_id = ?1", [subj_id], |r| r.get(0),
    )?;
    let ac_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'AC Circuit Analysis' AND subject_id = ?1", [subj_id], |r| r.get(0),
    )?;
    let logic_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Digital Logic Gates' AND subject_id = ?1", [subj_id], |r| r.get(0),
    )?;
    let semi_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Semiconductor Basics' AND subject_id = ?1", [subj_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: &[(i64, &str, &str, i64)] = &[
        (circuit_id, "What is an Electrical Circuit?",
         "An electrical circuit is a closed loop that allows electric current to flow.\n\nKey components:\n- **Voltage source** (battery/generator): provides the 'push' (EMF)\n- **Conductor** (wire): provides the path\n- **Load** (resistor/LED/motor): does useful work\n- **Switch**: controls the flow\n\nCircuits come in two basic configurations:\n- **Series**: components connected end-to-end (same current flows through all)\n- **Parallel**: components connected side-by-side (same voltage across all)\n\nThink of it like water flowing through pipes: voltage is water pressure, current is flow rate, and resistance is pipe narrowness.", 1),
        (ohm_id, "Ohm's Law and Kirchhoff's Laws",
         "**Ohm's Law: V = I × R**\n- V = voltage (volts), I = current (amps), R = resistance (ohms)\n- Double the voltage → double the current (for fixed resistance)\n\n**Kirchhoff's Current Law (KCL):**\nThe total current entering a node equals the total current leaving it.\nΣI_in = ΣI_out (conservation of charge)\n\n**Kirchhoff's Voltage Law (KVL):**\nThe sum of all voltages around any closed loop is zero.\nΣV = 0 (conservation of energy)\n\nThese three laws are the foundation for analyzing any circuit, no matter how complex.", 1),
        (cap_id, "Capacitors and Inductors",
         "**Capacitors** store energy in an electric field between two plates.\n- Capacitance C measured in Farads (F)\n- Q = C × V (charge = capacitance × voltage)\n- Energy stored: E = ½CV²\n- Block DC, pass AC (impedance decreases with frequency)\n\n**Inductors** store energy in a magnetic field created by current through a coil.\n- Inductance L measured in Henrys (H)\n- V = L × dI/dt (voltage opposes changes in current)\n- Energy stored: E = ½LI²\n- Pass DC, block AC (impedance increases with frequency)\n\nTogether, capacitors and inductors create filters, oscillators, and tuned circuits.", 1),
        (ac_id, "AC Circuit Analysis",
         "**Alternating Current (AC)** changes direction periodically.\n- v(t) = V_peak × sin(2πft + φ)\n- Frequency f in Hz, period T = 1/f\n- RMS voltage: V_rms = V_peak / √2 ≈ 0.707 × V_peak\n\n**Impedance** (Z) generalizes resistance for AC:\n- Resistor: Z_R = R\n- Capacitor: Z_C = 1/(jωC)\n- Inductor: Z_L = jωL\n\n**Resonance** occurs when Z_L = Z_C:\n- f_resonant = 1/(2π√(LC))\n- At resonance, impedance is purely resistive\n\n**Power in AC circuits:**\n- Real power P = V_rms × I_rms × cos(φ) [watts]\n- Reactive power Q = V_rms × I_rms × sin(φ) [VAR]\n- Power factor = cos(φ), ideally = 1", 1),
        (logic_id, "Digital Logic Gates",
         "Digital circuits operate on binary: 0 (LOW) and 1 (HIGH).\n\n**Basic gates:**\n- AND: output 1 only if ALL inputs are 1\n- OR: output 1 if ANY input is 1\n- NOT: inverts the input (0→1, 1→0)\n\n**Derived gates:**\n- NAND = NOT(AND) — universal gate\n- NOR = NOT(OR) — universal gate\n- XOR: output 1 if inputs DIFFER\n- XNOR: output 1 if inputs are SAME\n\n**Key concepts:**\n- Any logic function can be built from just NAND (or just NOR) gates\n- Boolean algebra simplifies circuits: De Morgan's laws, distributive law\n- Truth tables enumerate all input/output combinations\n- Karnaugh maps minimize Boolean expressions visually", 1),
        (semi_id, "Semiconductor Basics",
         "**Semiconductors** (silicon, germanium) have conductivity between metals and insulators.\n\n**Doping:**\n- N-type: add phosphorus → extra electrons (negative carriers)\n- P-type: add boron → extra holes (positive carriers)\n\n**PN Junction (Diode):**\n- Forward bias: current flows (voltage > ~0.7V for silicon)\n- Reverse bias: no current (depletion zone widens)\n- Applications: rectifiers, LEDs, solar cells\n\n**Transistor (BJT):**\n- Three terminals: Base, Collector, Emitter\n- Small base current controls large collector current\n- Acts as amplifier or switch\n- hFE (beta) = I_C / I_B (current gain, typically 50-300)\n\n**MOSFET:**\n- Voltage-controlled (gate voltage controls drain current)\n- Very high input impedance\n- Foundation of modern digital circuits (CMOS)", 1),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (circuit_id, "Voltage", "Voltage (V) is the electrical potential difference — the 'push' that drives current through a circuit.", Some("Like water pressure in a pipe: higher pressure pushes more water."), Some("What happens to current in a circuit if you double the voltage?")),
        (circuit_id, "Current", "Current (I) is the flow rate of electric charge, measured in amperes. 1 amp = 1 coulomb/second.", Some("Like the flow rate of water: gallons per minute."), Some("If 3 coulombs pass a point in 2 seconds, what is the current?")),
        (ohm_id, "Resistance", "Resistance (R) opposes current flow. Measured in ohms (Ω). R = V/I.", Some("Like a narrow section of pipe that restricts water flow."), Some("A 12V battery drives 2A through a resistor. What is the resistance?")),
        (cap_id, "Time Constant", "The RC time constant τ = R × C determines how fast a capacitor charges/discharges. After 5τ, it's ~99% complete.", Some("Like filling a bathtub: a bigger tub (C) or smaller faucet (R) takes longer."), Some("A 1kΩ resistor and 10μF capacitor — what is the time constant?")),
        (logic_id, "Boolean Algebra", "Boolean algebra uses AND (·), OR (+), and NOT (') to manipulate logic expressions. De Morgan's: (A·B)' = A'+B'.", Some("Like grammar rules for a language that only uses TRUE and FALSE."), Some("Simplify: A·B + A·B' using Boolean algebra.")),
        (semi_id, "PN Junction", "When P-type and N-type semiconductors meet, a depletion region forms. Forward bias shrinks it (current flows); reverse bias widens it (no current).", Some("Like a one-way valve: water flows one direction easily but is blocked the other way."), Some("Why does an LED need a current-limiting resistor?")),
    ];
    for (tid, concept, expl, analogy, followup) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // Quiz questions
    let circuit_qs: &[QuizRowNoTopic] = &[
        ("In a series circuit, what is the same through every component?", "multiple_choice", "Current",
         Some("Voltage"), Some("Current"), Some("Resistance"), Some("Power"),
         "Think about what must be conserved in a single path", "In series, there's only one path for current, so the same current flows through every component."),
        ("In a parallel circuit, what is the same across every branch?", "multiple_choice", "Voltage",
         Some("Current"), Some("Voltage"), Some("Resistance"), Some("Impedance"),
         "Each branch connects to the same two nodes", "Parallel branches share the same two nodes, so they all have the same voltage across them."),
        ("True or false: An open circuit has zero current.", "true_false", "true", Some("True"), Some("False"), None, None,
         "Can current flow through a gap?", "An open circuit is a broken loop — no current can flow because the path is incomplete."),
        ("The unit of electrical resistance is the ___.", "fill_in_blank", "ohm", None, None, None, None,
         "Named after Georg Simon ___", "Resistance is measured in ohms (Ω), named after Georg Simon Ohm."),
        ("What device stores energy in an electric field?", "multiple_choice", "Capacitor",
         Some("Resistor"), Some("Inductor"), Some("Capacitor"), Some("Transformer"),
         "Two plates separated by a dielectric", "Capacitors store energy in the electric field between their plates."),
    ];

    let ohm_qs: &[QuizRowNoTopic] = &[
        ("If V = 12V and R = 4Ω, what is I?", "multiple_choice", "3A",
         Some("2A"), Some("3A"), Some("48A"), Some("0.33A"),
         "V = I × R, solve for I", "I = V/R = 12/4 = 3A. Ohm's Law in action."),
        ("Kirchhoff's Current Law is based on conservation of ___.", "fill_in_blank", "charge", None, None, None, None,
         "What can't be created or destroyed at a circuit node?", "KCL follows from conservation of charge: all current entering a node must leave it."),
        ("The sum of voltages around a closed loop equals ___.", "fill_in_blank", "zero", None, None, None, None,
         "Energy conservation around a loop", "KVL: ΣV = 0 around any closed loop, based on conservation of energy."),
        ("Two 10Ω resistors in parallel have a combined resistance of ___.", "fill_in_blank", "5", None, None, None, None,
         "1/R_total = 1/R1 + 1/R2", "1/R = 1/10 + 1/10 = 2/10, so R = 5Ω. Parallel resistance is always less than the smallest branch."),
        ("Three resistors of 2Ω, 3Ω, 5Ω in series total ___ ohms.", "fill_in_blank", "10", None, None, None, None,
         "Series resistances add directly", "R_total = 2 + 3 + 5 = 10Ω. In series, resistances simply add up."),
    ];

    let logic_qs: &[QuizRowNoTopic] = &[
        ("An AND gate outputs 1 only when ___.", "fill_in_blank", "all inputs are 1", None, None, None, None,
         "Every input matters", "AND requires ALL inputs to be 1. If any input is 0, the output is 0."),
        ("Which gate is universal (can implement any logic function)?", "multiple_choice", "NAND",
         Some("AND"), Some("OR"), Some("NAND"), Some("XOR"),
         "It's the negation of the most basic gate", "NAND (and NOR) are universal gates — any Boolean function can be built from them alone."),
        ("XOR outputs 1 when the inputs are ___.", "fill_in_blank", "different", None, None, None, None,
         "Exclusive — one or the other but not both", "XOR = exclusive OR. Output is 1 when inputs differ (01 or 10), 0 when they match."),
        ("De Morgan's theorem: (A AND B)' equals ___.", "multiple_choice", "A' OR B'",
         Some("A' AND B'"), Some("A' OR B'"), Some("A OR B"), Some("(A OR B)'"),
         "Break the bar, change the sign", "De Morgan's: (A·B)' = A'+B'. The complement of AND is OR of complements."),
        ("True or false: A NOT gate has exactly one input.", "true_false", "true", Some("True"), Some("False"), None, None,
         "It just inverts", "A NOT gate (inverter) takes a single input and outputs its complement."),
    ];

    let semi_qs: &[QuizRowNoTopic] = &[
        ("A silicon diode has a forward voltage drop of approximately ___ volts.", "fill_in_blank", "0.7", None, None, None, None,
         "The most common number in electronics", "Silicon PN junctions have ~0.7V forward drop. Germanium is ~0.3V."),
        ("In a BJT, the current gain (beta) equals ___.", "multiple_choice", "I_C / I_B",
         Some("I_B / I_C"), Some("I_C / I_B"), Some("I_E / I_C"), Some("I_C / I_E"),
         "Collector current divided by the controlling current", "β = I_C / I_B. A small base current controls a much larger collector current."),
        ("N-type silicon is doped with atoms that have ___ valence electrons.", "multiple_choice", "5",
         Some("3"), Some("4"), Some("5"), Some("6"),
         "Group V elements like phosphorus", "N-type uses pentavalent dopants (5 valence electrons) like phosphorus — the extra electron is the carrier."),
        ("True or false: A MOSFET is voltage-controlled.", "true_false", "true", Some("True"), Some("False"), None, None,
         "Gate voltage, not gate current, controls the channel", "MOSFETs are voltage-controlled devices with very high input impedance — gate current is negligible."),
        ("What does LED stand for?", "fill_in_blank", "light emitting diode", None, None, None, None,
         "It's a diode that ___", "LED = Light Emitting Diode. It emits photons when forward-biased as electrons recombine with holes."),
    ];

    let cap_qs: &[QuizRowNoTopic] = &[
        ("The energy stored in a capacitor is E = ___.", "multiple_choice", "½CV²",
         Some("CV"), Some("½CV²"), Some("CV²"), Some("C²V"),
         "Half of something squared", "E = ½CV². Energy scales with the square of voltage — doubling V quadruples the energy."),
        ("An inductor opposes changes in ___.", "fill_in_blank", "current", None, None, None, None,
         "V = L × d___/dt", "Inductors oppose changes in current via Lenz's law: V = L × dI/dt."),
        ("The impedance of a capacitor ___ as frequency increases.", "multiple_choice", "decreases",
         Some("increases"), Some("decreases"), Some("stays the same"), Some("becomes infinite"),
         "Z_C = 1/(jωC)", "Z_C = 1/(ωC). Higher frequency → higher ω → lower impedance. Capacitors pass high frequencies easily."),
        ("The time constant of an RC circuit is τ = ___.", "fill_in_blank", "RC", None, None, None, None,
         "Resistance times capacitance", "τ = R × C. After one time constant, a charging capacitor reaches ~63% of final voltage."),
        ("True or false: At resonance, a series LC circuit has minimum impedance.", "true_false", "true", Some("True"), Some("False"), None, None,
         "The reactive components cancel", "At resonance, X_L = X_C and they cancel, leaving only resistance — minimum impedance."),
    ];

    let ac_qs: &[QuizRowNoTopic] = &[
        ("RMS voltage is V_peak divided by ___.", "multiple_choice", "√2",
         Some("2"), Some("√2"), Some("π"), Some("√3"),
         "Root mean square of a sine wave", "V_rms = V_peak / √2 ≈ 0.707 × V_peak. RMS gives the equivalent DC heating value."),
        ("The power factor equals cos(φ), where φ is the ___.", "fill_in_blank", "phase angle", None, None, None, None,
         "The angle between voltage and current phasors", "Power factor = cos(φ). φ is the phase angle between voltage and current. PF=1 means purely resistive."),
        ("What is the resonant frequency of an LC circuit with L=1mH and C=1μF?", "multiple_choice", "~5033 Hz",
         Some("~1000 Hz"), Some("~5033 Hz"), Some("~15900 Hz"), Some("~159 Hz"),
         "f = 1/(2π√(LC))", "f = 1/(2π√(0.001 × 0.000001)) = 1/(2π × 0.001) ≈ 5033 Hz."),
        ("True or false: In a purely capacitive AC circuit, current leads voltage by 90°.", "true_false", "true", Some("True"), Some("False"), None, None,
         "ICE — current leads in capacitive circuits", "Yes! Remember ICE: In a Capacitor, current (I) leads voltage (E) by 90°."),
        ("Real power is measured in ___.", "fill_in_blank", "watts", None, None, None, None,
         "The unit of actual useful power", "Real power P (watts) does actual work. Reactive power Q (VAR) just oscillates back and forth."),
    ];

    for (q, qt, ans, a, b, c, d, hint, expl) in circuit_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![circuit_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }
    for (q, qt, ans, a, b, c, d, hint, expl) in ohm_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![ohm_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }
    for (q, qt, ans, a, b, c, d, hint, expl) in cap_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![cap_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }
    for (q, qt, ans, a, b, c, d, hint, expl) in ac_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![ac_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }
    for (q, qt, ans, a, b, c, d, hint, expl) in logic_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![logic_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }
    for (q, qt, ans, a, b, c, d, hint, expl) in semi_qs {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![semi_id, q, qt, ans, a, b, c, d, hint, expl],
        )?;
    }

    // Learning path
    let path_steps: &[(i64, &str)] = &[
        (circuit_id, "Start with circuit fundamentals — voltage, current, and basic topologies"),
        (ohm_id, "Master Ohm's Law and Kirchhoff's Laws — the foundation of circuit analysis"),
        (cap_id, "Learn about energy storage elements — capacitors and inductors"),
        (ac_id, "Analyze AC circuits — impedance, resonance, and power"),
        (logic_id, "Enter the digital world — Boolean algebra and logic gates"),
        (semi_id, "Understand semiconductors — diodes, transistors, and modern electronics"),
    ];
    for (i, (tid, desc)) in path_steps.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Electrical Engineering Foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

// ── Robotics & AI Subject ────────────────────────────────────────────────
pub fn seed_robotics_ai(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row("SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Robotics & AI'", [], |r| r.get(0))
        .unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Robotics & AI', 'Intelligent machines — from sensors and actuators to neural networks and reinforcement learning.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Robotics & AI'", [], |r| r.get(0))?;

    let topics = [
        ("Sensors & Perception", "beginner"),
        ("Actuators & Motion", "beginner"),
        ("Search Algorithms", "intermediate"),
        ("Neural Networks", "intermediate"),
        ("Reinforcement Learning", "advanced"),
        ("Computer Vision", "intermediate"),
        ("Natural Language Processing", "advanced"),
        ("Robot Kinematics", "advanced"),
    ];
    for (i, (name, diff)) in topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, i + 1],
        )?;
    }

    let sensor_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Sensors & Perception' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let actuator_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Actuators & Motion' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let search_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Search Algorithms' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let nn_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Neural Networks' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let rl_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Reinforcement Learning' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let cv_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Computer Vision' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let nlp_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Natural Language Processing' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let kin_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Robot Kinematics' AND subject_id = ?1", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: &[LessonRow] = &[
        (sensor_id, "Introduction to Sensors", "Robots perceive the world through sensors. Common types include ultrasonic (distance), infrared (proximity), LiDAR (3D mapping), cameras (vision), and IMUs (orientation). Each sensor has a range, accuracy, and update rate that determines its suitability for a task.", 1),
        (sensor_id, "Sensor Fusion", "No single sensor is perfect. Sensor fusion combines data from multiple sources — e.g., a Kalman filter merging GPS and IMU data — to produce a more accurate and robust estimate of the robot's state.", 2),
        (actuator_id, "Motors and Servos", "Actuators convert electrical signals into physical motion. DC motors provide continuous rotation, servos give precise angle control, and stepper motors offer discrete positioning. The choice depends on torque, speed, and precision requirements.", 1),
        (actuator_id, "Locomotion Strategies", "Wheeled robots are simplest (differential drive, omnidirectional). Legged robots handle rough terrain. Drones use rotors for flight. Soft robots use pneumatic or hydraulic actuators for flexible, adaptive movement.", 2),
        (search_id, "Graph Search: BFS & DFS", "Search algorithms explore a graph of states to find a goal. Breadth-first search (BFS) explores level by level (optimal for unweighted graphs). Depth-first search (DFS) goes deep first (memory efficient but not optimal).", 1),
        (search_id, "A* and Heuristic Search", "A* combines path cost g(n) with a heuristic estimate h(n) to the goal. If h is admissible (never overestimates), A* finds the optimal path. It is the foundation of most robot path-planning systems.", 2),
        (nn_id, "Perceptrons to Deep Networks", "A neural network is a stack of layers that transform inputs through weighted connections and nonlinear activation functions. A single perceptron can only learn linear boundaries; deep networks with multiple layers learn hierarchical features — edges, shapes, objects.", 1),
        (nn_id, "Backpropagation", "Training a neural network uses gradient descent: compute the loss (error), propagate gradients backward through the network using the chain rule, and update weights to minimize the loss. Learning rate, batch size, and regularization are key hyperparameters.", 2),
        (rl_id, "Markov Decision Processes", "Reinforcement learning models problems as MDPs: an agent in state s takes action a, transitions to state s' with probability P(s'|s,a), and receives reward r. The goal is to learn a policy π(s) that maximizes cumulative discounted reward.", 1),
        (rl_id, "Q-Learning and Policy Gradient", "Q-learning estimates the value of each (state, action) pair. Deep Q-Networks (DQN) use neural networks for large state spaces. Policy gradient methods (like PPO and A3C) directly optimize the policy, which handles continuous action spaces better.", 2),
        (cv_id, "Image Processing Basics", "Computer vision starts with image processing: converting to grayscale, edge detection (Sobel, Canny), filtering (Gaussian blur, median filter), and thresholding. These operations extract features that higher-level algorithms can interpret.", 1),
        (cv_id, "Convolutional Neural Networks", "CNNs are the backbone of modern vision. Convolutional layers detect local patterns (edges, textures), pooling layers reduce spatial dimensions, and fully connected layers produce classification. Architectures like ResNet and YOLO handle object detection in real time.", 2),
        (nlp_id, "Tokenization and Embeddings", "NLP converts text into numbers. Tokenization splits text into words or subwords. Embeddings (Word2Vec, GloVe) map tokens to dense vectors where semantic similarity corresponds to geometric proximity.", 1),
        (nlp_id, "Transformers and Attention", "The Transformer architecture (Vaswani et al., 2017) replaced RNNs with self-attention: each token attends to every other token in parallel. This enables models like BERT (bidirectional understanding) and GPT (autoregressive generation) that power modern AI.", 2),
        (kin_id, "Forward and Inverse Kinematics", "Forward kinematics: given joint angles, compute the end-effector position. Inverse kinematics: given a target position, find the joint angles. FK is straightforward geometry; IK may have multiple solutions or none.", 1),
        (kin_id, "Degrees of Freedom", "A robot arm's degrees of freedom (DOF) determine its workspace. 6-DOF arms can reach any position and orientation in 3D space. Redundant arms (7+ DOF) offer extra flexibility to avoid obstacles or optimize posture.", 2),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (sensor_id, "LiDAR", "LiDAR (Light Detection and Ranging) fires laser pulses and measures the time-of-flight to build a 3D point cloud of the environment. Used in self-driving cars and mapping drones.", Some("Think of LiDAR as a bat's echolocation but with light instead of sound — it bounces photons off surfaces to map the world."), Some("Why might LiDAR struggle in heavy rain or fog?")),
        (nn_id, "Activation Function", "An activation function introduces nonlinearity into a neural network. Without it, stacking linear layers would just produce another linear function. Common choices: ReLU (max(0,x)), sigmoid, and tanh.", Some("An activation function is like a bouncer at a club — it decides which signals get through and how strong they are."), Some("What happens if you use a linear activation everywhere?")),
        (rl_id, "Exploration vs Exploitation", "An RL agent must balance trying new actions (exploration) to discover better strategies versus using its current best strategy (exploitation). Epsilon-greedy, UCB, and entropy bonus are common approaches.", Some("Imagine choosing a restaurant: do you try the new place (explore) or go to your favorite (exploit)?"), Some("Why would pure exploitation fail in a changing environment?")),
        (cv_id, "Convolution", "A convolution slides a small filter (kernel) over an image, computing dot products to produce a feature map. Different kernels detect different patterns: horizontal edges, vertical edges, corners, textures.", Some("A convolution is like running a magnifying glass over a photo — each position reveals specific details."), Some("What is the effect of increasing the kernel size?")),
        (nlp_id, "Attention Mechanism", "Attention computes a weighted sum of values, where weights are determined by query-key compatibility. It allows a model to focus on relevant parts of the input regardless of distance.", Some("Attention is like highlighting the important words in a sentence — the model learns which words matter for each prediction."), Some("How does multi-head attention differ from single-head?")),
        (kin_id, "Jacobian Matrix", "The Jacobian relates joint velocities to end-effector velocities. It's essential for velocity control, singularity detection, and solving inverse kinematics iteratively.", Some("The Jacobian is like a translation table between the language of joints and the language of the end-effector."), Some("What happens at a singularity of the Jacobian?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let quizzes: &[QuizRow] = &[
        (sensor_id, "Which sensor type creates a 3D point cloud using laser pulses?", "multiple_choice", "LiDAR", Some("Ultrasonic"), Some("LiDAR"), Some("Infrared"), Some("Camera"), None, "LiDAR measures time-of-flight of laser pulses to build 3D maps."),
        (sensor_id, "True or false: A Kalman filter is used to combine data from multiple sensors.", "true_false", "true", None, None, None, None, None, "Kalman filters are the most common sensor fusion technique."),
        (sensor_id, "An IMU measures ___ and angular velocity.", "fill_in_blank", "acceleration", None, None, None, None, None, "IMU stands for Inertial Measurement Unit — it contains accelerometers and gyroscopes."),
        (actuator_id, "Which motor type provides precise angular positioning?", "multiple_choice", "Servo motor", Some("DC motor"), Some("Servo motor"), Some("Stepper motor"), Some("Linear actuator"), None, "Servos have built-in feedback for precise angle control."),
        (actuator_id, "True or false: Differential drive robots can rotate in place.", "true_false", "true", None, None, None, None, None, "By spinning wheels in opposite directions, the robot pivots around its center."),
        (search_id, "Which search algorithm is guaranteed to find the shortest path in an unweighted graph?", "multiple_choice", "BFS", Some("DFS"), Some("BFS"), Some("Random walk"), Some("Greedy search"), None, "BFS explores level by level, ensuring the first path found is shortest."),
        (search_id, "A* uses f(n) = g(n) + h(n). What does h(n) represent?", "fill_in_blank", "heuristic estimate to goal", None, None, None, None, None, "h(n) is the heuristic function — an estimate of the remaining cost to reach the goal."),
        (search_id, "Order the algorithms from least to most memory usage: A*, DFS, BFS", "ordering", "DFS,BFS,A*", None, None, None, None, None, "DFS uses O(d) memory (depth), BFS uses O(b^d), A* stores the open+closed lists."),
        (nn_id, "What problem does the ReLU activation function solve compared to sigmoid?", "multiple_choice", "Vanishing gradient problem", Some("Overfitting"), Some("Vanishing gradient problem"), Some("Underfitting"), Some("Memory overflow"), None, "ReLU's gradient is 1 for positive values, preventing gradient vanishing during backpropagation."),
        (nn_id, "The process of computing gradients layer by layer is called ___.", "fill_in_blank", "backpropagation", None, None, None, None, None, "Backpropagation uses the chain rule to compute gradients from output to input."),
        (nn_id, "True or false: A single-layer perceptron can learn the XOR function.", "true_false", "false", None, None, None, None, None, "XOR is not linearly separable — it requires at least one hidden layer."),
        (rl_id, "In Q-learning, Q(s,a) estimates the expected ___.", "fill_in_blank", "cumulative reward", None, None, None, None, None, "Q-values represent the expected total discounted future reward for taking action a in state s."),
        (rl_id, "Which RL approach directly optimizes the policy without learning a value function?", "multiple_choice", "Policy gradient", Some("Q-learning"), Some("Dynamic programming"), Some("Policy gradient"), Some("Monte Carlo tree search"), None, "Policy gradient methods parameterize and directly optimize the policy."),
        (rl_id, "Match the RL concept with its description: Epsilon-greedy=Random actions with probability epsilon;Discount factor=Reduces weight of future rewards;Replay buffer=Stores past transitions for training", "matching", "Epsilon-greedy=Random actions with probability epsilon;Discount factor=Reduces weight of future rewards;Replay buffer=Stores past transitions for training", None, None, None, None, None, "These are fundamental components of modern RL systems."),
        (cv_id, "Which CNN architecture introduced residual (skip) connections?", "multiple_choice", "ResNet", Some("AlexNet"), Some("VGG"), Some("ResNet"), Some("LeNet"), None, "ResNet (2015) introduced skip connections to train very deep networks (100+ layers)."),
        (cv_id, "The Canny edge detector includes ___ suppression to thin edges.", "fill_in_blank", "non-maximum", None, None, None, None, None, "Non-maximum suppression keeps only the local maxima of gradient magnitude, thinning edges to one pixel wide."),
        (cv_id, "True or false: Max pooling reduces the spatial dimensions of a feature map.", "true_false", "true", None, None, None, None, None, "Max pooling selects the maximum value in each patch, reducing width and height."),
        (nlp_id, "Which model architecture replaced RNNs for most NLP tasks after 2017?", "multiple_choice", "Transformer", Some("LSTM"), Some("Transformer"), Some("CNN"), Some("GAN"), None, "The Transformer (Vaswani et al., 2017) uses self-attention for parallel processing of sequences."),
        (nlp_id, "Word2Vec learns word ___ by predicting context words.", "fill_in_blank", "embeddings", None, None, None, None, None, "Word2Vec maps words to dense vectors where similar words are geometrically close."),
        (nlp_id, "Order these NLP milestones chronologically: Transformer, Word2Vec, GPT-3, BERT", "ordering", "Word2Vec,Transformer,BERT,GPT-3", None, None, None, None, None, "Word2Vec (2013), Transformer (2017), BERT (2018), GPT-3 (2020)."),
        (kin_id, "How many degrees of freedom does a standard industrial robot arm have?", "multiple_choice", "6", Some("3"), Some("4"), Some("6"), Some("12"), None, "6 DOF allows reaching any position and orientation in 3D space."),
        (kin_id, "Forward kinematics computes end-effector position from ___.", "fill_in_blank", "joint angles", None, None, None, None, None, "FK is the straightforward direction: given joints, compute the tool position."),
        (kin_id, "True or false: Inverse kinematics always has a unique solution.", "true_false", "false", None, None, None, None, None, "IK can have zero, one, or multiple solutions depending on the configuration."),
    ];
    for (tid, question, qtype, answer, oa, ob, oc, od, _hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![tid, question, qtype, answer, oa, ob, oc, od, expl],
        )?;
    }

    // Learning path
    let path_steps: &[(i64, &str)] = &[
        (sensor_id, "Start with how robots perceive the world — sensors and sensor fusion"),
        (actuator_id, "Learn how robots move — motors, servos, and locomotion strategies"),
        (search_id, "Explore pathfinding — BFS, DFS, and A* for robot navigation"),
        (cv_id, "Dive into computer vision — image processing and CNNs"),
        (nn_id, "Understand neural networks — the brain behind intelligent robots"),
        (rl_id, "Master reinforcement learning — teaching robots through trial and error"),
        (nlp_id, "Explore natural language processing — enabling robots to understand text"),
        (kin_id, "Complete your journey with robot kinematics — precise arm control"),
    ];
    for (i, (tid, desc)) in path_steps.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Robotics & AI Foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

// ── Number Theory Subject ────────────────────────────────────────────────
pub fn seed_number_theory(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row("SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Number Theory'", [], |r| r.get(0))
        .unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Number Theory', 'The queen of mathematics — primes, divisibility, modular arithmetic, and the elegant properties of integers.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Number Theory'", [], |r| r.get(0))?;

    let topics = [
        ("Divisibility & GCD", "beginner"),
        ("Prime Numbers", "beginner"),
        ("Modular Arithmetic", "intermediate"),
        ("Diophantine Equations", "intermediate"),
        ("Euler's Totient Function", "advanced"),
        ("Cryptographic Applications", "advanced"),
    ];
    for (i, (name, diff)) in topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, i + 1],
        )?;
    }

    let div_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Divisibility & GCD' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let prime_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Prime Numbers' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let mod_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Modular Arithmetic' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let dioph_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Diophantine Equations' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let euler_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = ?1 AND subject_id = ?2", rusqlite::params!["Euler's Totient Function", subj_id], |r| r.get(0))?;
    let crypto_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Cryptographic Applications' AND subject_id = ?1", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: &[LessonRow] = &[
        (div_id, "Divisibility Rules", "An integer a divides b (written a|b) if there exists an integer k such that b = a·k. The GCD (Greatest Common Divisor) of two numbers is the largest number that divides both. Euclid's algorithm computes GCD efficiently: gcd(a,b) = gcd(b, a mod b).", 1),
        (div_id, "The Euclidean Algorithm", "To find gcd(252, 105): 252 = 2×105 + 42, then 105 = 2×42 + 21, then 42 = 2×21 + 0. So gcd(252,105) = 21. The Extended Euclidean Algorithm also finds integers x,y such that ax + by = gcd(a,b).", 2),
        (prime_id, "What Makes a Prime", "A prime number p > 1 has exactly two divisors: 1 and itself. The Fundamental Theorem of Arithmetic says every integer > 1 has a unique prime factorization. Primes are the atoms of number theory.", 1),
        (prime_id, "The Sieve of Eratosthenes", "To find all primes up to N: start with 2, mark all multiples of 2 as composite, advance to the next unmarked number (3), mark its multiples, and repeat up to √N. Time complexity: O(N log log N).", 2),
        (prime_id, "Famous Conjectures", "The Twin Prime Conjecture (infinitely many primes p where p+2 is also prime) and Goldbach's Conjecture (every even number ≥ 4 is the sum of two primes) remain unproven. The Riemann Hypothesis connects primes to the zeros of the zeta function.", 3),
        (mod_id, "Clock Arithmetic", "Modular arithmetic works like a clock: 17 mod 12 = 5 (5 hours past 12). Formally, a ≡ b (mod n) means n divides (a-b). Addition and multiplication work naturally: (a+b) mod n = ((a mod n)+(b mod n)) mod n.", 1),
        (mod_id, "Fermat's Little Theorem", "If p is prime and gcd(a,p) = 1, then a^(p-1) ≡ 1 (mod p). This is the basis of primality testing and is fundamental to RSA encryption. Example: 2^6 = 64 ≡ 1 (mod 7).", 2),
        (dioph_id, "Linear Diophantine Equations", "The equation ax + by = c has integer solutions if and only if gcd(a,b) divides c. If (x₀,y₀) is one solution, then all solutions are x = x₀ + (b/d)t, y = y₀ - (a/d)t where d = gcd(a,b) and t is any integer.", 1),
        (dioph_id, "The Chinese Remainder Theorem", "If n₁,n₂,...,nk are pairwise coprime, the system x ≡ a₁ (mod n₁), x ≡ a₂ (mod n₂), ..., x ≡ ak (mod nk) has a unique solution modulo N = n₁·n₂·...·nk. This is used in RSA and parallel computation.", 2),
        (euler_id, "Euler's Totient", "φ(n) counts the integers from 1 to n that are coprime to n. For prime p: φ(p) = p-1. For prime power: φ(p^k) = p^k - p^(k-1). Euler's theorem: if gcd(a,n) = 1, then a^φ(n) ≡ 1 (mod n).", 1),
        (euler_id, "Computing the Totient", "For n = p₁^a₁ · p₂^a₂ · ... · pk^ak: φ(n) = n · ∏(1 - 1/pᵢ). Example: φ(12) = 12 · (1-1/2)(1-1/3) = 12 · 1/2 · 2/3 = 4. The integers coprime to 12 are {1,5,7,11}.", 2),
        (crypto_id, "RSA Encryption", "RSA relies on number theory: pick large primes p,q; compute n=pq and φ(n)=(p-1)(q-1); choose e coprime to φ(n); find d such that ed ≡ 1 (mod φ(n)). Public key: (n,e). Encrypt: c = m^e mod n. Decrypt: m = c^d mod n.", 1),
        (crypto_id, "Diffie-Hellman Key Exchange", "Alice and Bob agree on a prime p and generator g. Alice picks secret a, sends g^a mod p. Bob picks secret b, sends g^b mod p. Both compute the shared secret g^(ab) mod p. Security relies on the discrete logarithm problem.", 2),
    ];
    for (tid, title, content, order) in lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: &[ExplanationRow] = &[
        (div_id, "GCD", "The Greatest Common Divisor of two numbers is the largest integer that divides both without a remainder. Euclid's algorithm finds it by repeated division.", Some("Finding the GCD is like finding the largest tile that perfectly covers two rectangular floors."), Some("What is gcd(48, 18)?")),
        (prime_id, "Prime Factorization", "Every integer greater than 1 can be written as a product of primes in exactly one way (up to order). For example, 60 = 2² × 3 × 5.", Some("Prime factorization is like breaking a molecule into its constituent atoms — primes are the indivisible building blocks."), Some("What is the prime factorization of 84?")),
        (mod_id, "Modular Inverse", "The modular inverse of a mod n is a number b such that a·b ≡ 1 (mod n). It exists if and only if gcd(a,n) = 1. Found via the Extended Euclidean Algorithm.", Some("A modular inverse is like an 'undo button' for multiplication on a circular number line."), Some("What is the inverse of 3 mod 7?")),
        (euler_id, "Euler's Theorem", "If gcd(a,n) = 1, then a^φ(n) ≡ 1 (mod n). This generalizes Fermat's Little Theorem to composite moduli.", Some("Euler's theorem says: if you keep multiplying a by itself mod n, you always cycle back to 1 after φ(n) steps."), Some("How does Euler's theorem help in RSA decryption?")),
        (crypto_id, "Discrete Logarithm", "Given g, p, and g^x mod p, finding x is computationally hard for large primes. This one-way function is the security foundation of Diffie-Hellman and ElGamal.", Some("The discrete log is like a one-way lock: easy to scramble, nearly impossible to unscramble without the key."), Some("Why can't you just try all possible x values?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let quizzes: &[QuizRow] = &[
        (div_id, "What is gcd(48, 18)?", "fill_in_blank", "6", None, None, None, None, None, "48 = 2×18 + 12, 18 = 1×12 + 6, 12 = 2×6 + 0. So gcd = 6."),
        (div_id, "True or false: If a|b and a|c, then a|(b+c).", "true_false", "true", None, None, None, None, None, "If b = a·k₁ and c = a·k₂, then b+c = a(k₁+k₂)."),
        (div_id, "The Euclidean algorithm computes GCD using repeated ___.", "fill_in_blank", "division", None, None, None, None, None, "Each step replaces the larger number with the remainder of division."),
        (prime_id, "Which of these is NOT a prime number?", "multiple_choice", "91", Some("97"), Some("91"), Some("89"), Some("83"), None, "91 = 7 × 13."),
        (prime_id, "True or false: There are infinitely many prime numbers.", "true_false", "true", None, None, None, None, None, "Euclid proved this around 300 BCE by contradiction."),
        (prime_id, "The Sieve of Eratosthenes only needs to check multiples up to ___.", "fill_in_blank", "square root of N", None, None, None, None, None, "Any composite ≤ N must have a factor ≤ √N."),
        (prime_id, "What is the prime factorization of 60?", "fill_in_blank", "2^2 * 3 * 5", None, None, None, None, None, "60 = 4 × 15 = 2² × 3 × 5."),
        (mod_id, "What is 17 mod 5?", "fill_in_blank", "2", None, None, None, None, None, "17 = 3×5 + 2."),
        (mod_id, "By Fermat's Little Theorem, 2^10 mod 11 = ___.", "fill_in_blank", "1", None, None, None, None, None, "Since 11 is prime and gcd(2,11)=1, 2^(11-1) = 2^10 ≡ 1 (mod 11)."),
        (mod_id, "True or false: a ≡ b (mod n) means n divides (a - b).", "true_false", "true", None, None, None, None, None, "This is the definition of modular congruence."),
        (dioph_id, "The equation 6x + 10y = 3 has integer solutions. True or false?", "true_false", "false", None, None, None, None, None, "gcd(6,10) = 2, and 2 does not divide 3."),
        (dioph_id, "The Chinese Remainder Theorem requires that the moduli are pairwise ___.", "fill_in_blank", "coprime", None, None, None, None, None, "Pairwise coprime means gcd(nᵢ, nⱼ) = 1 for all i ≠ j."),
        (euler_id, "What is φ(12)?", "fill_in_blank", "4", None, None, None, None, None, "φ(12) = 12 × (1-1/2) × (1-1/3) = 4. Coprime to 12: {1,5,7,11}."),
        (euler_id, "φ(p) for a prime p equals ___.", "fill_in_blank", "p-1", None, None, None, None, None, "Every integer from 1 to p-1 is coprime to a prime p."),
        (euler_id, "True or false: Euler's theorem generalizes Fermat's Little Theorem.", "true_false", "true", None, None, None, None, None, "For prime p, φ(p) = p-1, so Euler's theorem reduces to Fermat's."),
        (crypto_id, "In RSA, the public key consists of n and ___.", "fill_in_blank", "e", None, None, None, None, None, "The public key is (n, e) where n = pq and e is coprime to φ(n)."),
        (crypto_id, "Which mathematical problem makes RSA secure?", "multiple_choice", "Integer factorization", Some("Integer factorization"), Some("Graph coloring"), Some("Sorting"), Some("Matrix inversion"), None, "RSA security depends on the difficulty of factoring the product of two large primes."),
        (crypto_id, "Diffie-Hellman key exchange relies on the difficulty of the ___ problem.", "fill_in_blank", "discrete logarithm", None, None, None, None, None, "Given g^x mod p, finding x is computationally infeasible for large primes."),
        (crypto_id, "Order the RSA steps: Choose primes p and q, Compute n=pq, Select public exponent e, Compute private exponent d", "ordering", "Choose primes p and q,Compute n=pq,Select public exponent e,Compute private exponent d", None, None, None, None, None, "You need p and q first, then n, then e (coprime to φ(n)), then d (modular inverse of e)."),
    ];
    for (tid, question, qtype, answer, oa, ob, oc, od, _hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![tid, question, qtype, answer, oa, ob, oc, od, expl],
        )?;
    }

    // Learning path
    let path_steps: &[(i64, &str)] = &[
        (div_id, "Start with divisibility rules and the Euclidean algorithm"),
        (prime_id, "Explore prime numbers — the atoms of arithmetic"),
        (mod_id, "Learn modular arithmetic — the clock-like algebra of remainders"),
        (dioph_id, "Solve Diophantine equations — integer solutions to polynomial equations"),
        (euler_id, "Master Euler's totient function and its powerful theorem"),
        (crypto_id, "Apply number theory to cryptography — RSA and Diffie-Hellman"),
    ];
    for (i, (tid, desc)) in path_steps.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('Number Theory Journey', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_formal_languages(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Formal Languages'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Formal Languages', 'Automata theory, grammars, and computability — the mathematical foundations of computer science.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Formal Languages'", [], |r| r.get(0),
    )?;

    let topics = [
        (subj_id, "Finite Automata", "beginner", 1),
        (subj_id, "Regular Expressions", "beginner", 2),
        (subj_id, "Context-Free Grammars", "intermediate", 3),
        (subj_id, "Turing Machines", "advanced", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let fa_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Finite Automata'",
        [subj_id], |r| r.get(0),
    )?;
    let re_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Regular Expressions'",
        [subj_id], |r| r.get(0),
    )?;
    let cfg_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Context-Free Grammars'",
        [subj_id], |r| r.get(0),
    )?;
    let tm_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Turing Machines'",
        [subj_id], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (fa_id, "Deterministic Finite Automata (DFA)", "A DFA is the simplest model of computation — a machine with a finite number of states that reads input one symbol at a time.\n\nFormally, a DFA is a 5-tuple (Q, Σ, δ, q₀, F):\n- Q: finite set of states\n- Σ: input alphabet (finite set of symbols)\n- δ: transition function (Q × Σ → Q)\n- q₀: start state (q₀ ∈ Q)\n- F: set of accept states (F ⊆ Q)\n\nThe DFA starts in q₀, reads each input symbol, follows δ to the next state, and accepts if it ends in a state in F.\n\nExample: A DFA that accepts binary strings ending in '1':\n- States: {q0, q1}, Start: q0, Accept: {q1}\n- δ(q0, 0)=q0, δ(q0, 1)=q1, δ(q1, 0)=q0, δ(q1, 1)=q1\n\nKey property: DFAs are deterministic — for every state and symbol, there is exactly one transition.", 1),
        (fa_id, "Nondeterministic Finite Automata (NFA)", "An NFA is like a DFA but can be in multiple states at once.\n\nDifferences from DFA:\n- δ maps to a SET of states (Q × Σ → P(Q))\n- Can have ε-transitions (move without reading input)\n- Can have multiple transitions for the same symbol\n\nAn NFA accepts if ANY possible path leads to an accept state.\n\nKey theorem (Rabin-Scott, 1959): Every NFA can be converted to an equivalent DFA using the **subset construction**. The DFA may have up to 2ⁿ states for an NFA with n states.\n\nThis means NFAs and DFAs recognize exactly the same class of languages: the **regular languages**.\n\nWhy use NFAs? They're often much simpler to design. The exponential blowup in conversion is worst-case; in practice, many states are unreachable.", 2),
        (re_id, "Regular Expression Syntax", "Regular expressions describe patterns in strings using a concise algebraic notation.\n\nBasic operations:\n- Concatenation: ab means 'a followed by b'\n- Union: a|b means 'a or b'\n- Kleene star: a* means 'zero or more a's'\n\nExtended operations:\n- a+ = one or more a's (= aa*)\n- a? = zero or one a\n- [abc] = character class (a or b or c)\n- . = any single character\n\nExamples:\n- (0|1)*1 matches binary strings ending in 1\n- a*b*c* matches strings of a's then b's then c's\n- (ab)* matches ε, ab, abab, ababab, ...\n\nKleene's Theorem: A language is regular ↔ it can be described by a regular expression ↔ it is recognized by a DFA/NFA.", 1),
        (re_id, "Regular Languages & the Pumping Lemma", "Not every language is regular. The **Pumping Lemma** is used to prove a language is NOT regular.\n\nPumping Lemma: For any regular language L, there exists a pumping length p such that any string s in L with |s| ≥ p can be split into s = xyz where:\n1. |y| > 0 (y is non-empty)\n2. |xy| ≤ p\n3. For all i ≥ 0, xy^i z ∈ L (pumping y any number of times stays in L)\n\nExample: L = {aⁿbⁿ | n ≥ 0} is NOT regular.\nProof: Assume regular with pumping length p. Take s = aᵖbᵖ.\nThen y = aᵏ for some k > 0 (since |xy| ≤ p and first p chars are a's).\nPumping: xy²z = aᵖ⁺ᵏbᵖ, but p+k ≠ p, so not in L. Contradiction.\n\nLanguages that need 'counting' or 'matching' are typically not regular.", 2),
        (cfg_id, "Context-Free Grammars", "A CFG generates strings by repeatedly replacing variables with combinations of variables and terminals.\n\nFormally: G = (V, Σ, R, S)\n- V: variables (non-terminals)\n- Σ: terminals (alphabet symbols)\n- R: production rules (V → (V ∪ Σ)*)\n- S: start variable\n\nExample: Grammar for aⁿbⁿ:\n  S → aSb | ε\n  Derivation: S → aSb → aaSbb → aabb\n\nCFGs are strictly more powerful than regular expressions. They can describe:\n- Matched parentheses: S → (S) | SS | ε\n- Palindromes: S → aSa | bSb | a | b | ε\n- Most programming language syntax\n\nParse trees visualize how a string is derived from the grammar. Ambiguous grammars can produce multiple parse trees for the same string.", 1),
        (cfg_id, "Pushdown Automata", "A Pushdown Automaton (PDA) is an NFA with a stack — giving it memory to handle context-free languages.\n\nKey idea: the stack lets the PDA 'remember' things that finite automata cannot, like how many a's it has seen.\n\nExample PDA for {aⁿbⁿ}:\n1. Push an 'A' onto the stack for each 'a' read\n2. Pop an 'A' for each 'b' read\n3. Accept if stack is empty at end of input\n\nTheorem: A language is context-free ↔ some PDA recognizes it.\n\nDeterministic PDAs (DPDA) are strictly weaker than nondeterministic PDAs. This is unlike finite automata, where DFA = NFA in power.\n\nContext-free languages are closed under union, concatenation, and star, but NOT under intersection or complement.", 2),
        (tm_id, "The Turing Machine", "A Turing Machine is the most powerful model of computation — anything computable can be computed by a TM.\n\nComponents:\n- An infinite tape divided into cells (each holds a symbol)\n- A head that reads/writes symbols and moves left or right\n- A finite set of states with a transition function\n- δ: Q × Γ → Q × Γ × {L, R}\n\nThe Church-Turing Thesis: Any function that can be computed by an algorithm can be computed by a Turing Machine. This is a thesis, not a theorem — it cannot be formally proven.\n\nTMs can simulate any computer, any programming language, any algorithm. They define the boundary of what is computable.\n\nThe Universal Turing Machine (UTM) takes a description of another TM as input and simulates it — essentially the first 'programmable computer' concept (Turing, 1936).", 1),
        (tm_id, "Undecidability & the Halting Problem", "Some problems are fundamentally unsolvable — no algorithm can ever solve them.\n\nThe Halting Problem: Given a program P and input I, does P halt (finish) on I?\n\nTuring proved this is undecidable (1936):\nAssume a halting decider H(P, I) exists.\nConstruct D(P) = if H(P, P) says 'halts', then loop forever; else halt.\nWhat does D(D) do?\n- If H says D(D) halts → D loops. Contradiction.\n- If H says D(D) loops → D halts. Contradiction.\nTherefore H cannot exist.\n\nConsequences:\n- No compiler can detect all infinite loops\n- No antivirus can detect all malware\n- Rice's Theorem: ANY non-trivial property of programs is undecidable\n\nThis connects to Gödel's Incompleteness Theorems: some true mathematical statements cannot be proven.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (fa_id, "finite automata", "A finite automaton is a simple machine with a fixed number of states that processes input one symbol at a time.", Some("A DFA is like a vending machine — it has a fixed number of states (idle, coin inserted, item selected), reads inputs (coins, button presses), and transitions between states. It has no memory beyond which state it's currently in."), Some("Can a DFA count how many 1s are in a binary string? What about checking if the count is even?")),
        (re_id, "regular expressions", "Regular expressions are patterns that describe sets of strings using a compact algebraic notation.", Some("A regex is like a bouncer at a club with a checklist — it looks at each string and decides 'you match, come in' or 'you don't match, go away.' The pattern describes the rules."), Some("Can you write a regex for email addresses? Why might that be tricky?")),
        (cfg_id, "context-free grammars", "A context-free grammar is a set of recursive rewriting rules used to generate patterns of strings.", Some("A CFG is like a recipe with sub-recipes — 'make a sentence' becomes 'make a noun phrase + make a verb phrase', which recursively breaks down further until you reach actual words."), Some("Why can't regular expressions handle matched parentheses but CFGs can?")),
        (tm_id, "Turing machines", "A Turing Machine is a theoretical device with an infinite tape, a read/write head, and a finite control — the most general model of computation.", Some("A Turing Machine is like a person with a pencil, an infinitely long roll of paper, and a rulebook. They read a symbol, look up what to do in the rulebook, write a new symbol, move left or right, and repeat. Despite being so simple, this can compute anything any modern computer can!"), Some("If Turing Machines can compute anything computable, why do we build different kinds of computers?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let questions: Vec<QuizRow> = vec![
        (fa_id, "A DFA has exactly ___ transition(s) for each state-symbol pair.", "fill_in_blank", "one", None, None, None, None, Some("Deterministic means no ambiguity"), "In a DFA, the transition function is total: exactly one transition for every (state, symbol) combination."),
        (fa_id, "Which theorem states that NFAs and DFAs are equivalent in power?", "multiple_choice", "Rabin-Scott theorem", Some("Rabin-Scott theorem"), Some("Pumping lemma"), Some("Church-Turing thesis"), Some("Rice's theorem"), Some("It involves subset construction"), "The Rabin-Scott theorem (1959) proves every NFA can be converted to an equivalent DFA via subset construction."),
        (fa_id, "True or false: An NFA can be in multiple states simultaneously.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Non-determinism allows branching"), "True. An NFA can follow multiple transitions at once, effectively being in a set of states."),
        (fa_id, "The subset construction can produce a DFA with at most ___ states from an NFA with n states.", "fill_in_blank", "2^n", None, None, None, None, Some("Each DFA state is a subset of NFA states"), "The subset construction creates DFA states that are subsets of NFA states, giving at most 2^n possible states."),
        (re_id, "The Kleene star (a*) matches ___ or more occurrences.", "fill_in_blank", "zero", None, None, None, None, Some("It includes the empty string"), "a* matches zero or more occurrences of a, including the empty string ε."),
        (re_id, "Which operation is NOT a basic regular expression operation?", "multiple_choice", "Intersection", Some("Concatenation"), Some("Union"), Some("Kleene star"), Some("Intersection"), Some("There are exactly three basic operations"), "The three basic regex operations are concatenation, union (|), and Kleene star (*). Intersection is not a basic operation."),
        (re_id, "True or false: Regular expressions and DFAs recognize the same class of languages.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Kleene's theorem"), "True. Kleene's theorem establishes that regular expressions and finite automata describe exactly the regular languages."),
        (re_id, "The language {aⁿbⁿ | n ≥ 0} is ___.", "multiple_choice", "Not regular", Some("Regular"), Some("Not regular"), Some("Undecidable"), Some("Random"), Some("Can a finite automaton count?"), "This language requires matching counts of a's and b's, which finite automata cannot do. The Pumping Lemma proves it."),
        (cfg_id, "In a context-free grammar, production rules replace ___.", "multiple_choice", "A single variable", Some("A pair of variables"), Some("A single variable"), Some("A terminal symbol"), Some("The entire string"), Some("Context-FREE means the left side is just one variable"), "In a CFG, each production rule has a single variable on the left side, which can be replaced regardless of context."),
        (cfg_id, "The computational model equivalent to CFGs is the ___.", "fill_in_blank", "pushdown automaton", None, None, None, None, Some("It's an NFA with a stack"), "Pushdown automata (PDAs) recognize exactly the context-free languages, just as DFAs recognize regular languages."),
        (cfg_id, "True or false: Context-free languages are closed under intersection.", "true_false", "false", Some("true"), Some("false"), None, None, Some("This differs from regular languages"), "False. Unlike regular languages, context-free languages are NOT closed under intersection or complement."),
        (cfg_id, "A grammar that produces multiple parse trees for the same string is called ___.", "fill_in_blank", "ambiguous", None, None, None, None, Some("Think about the word for 'unclear' or 'having multiple meanings'"), "An ambiguous grammar can derive the same string in multiple ways, producing different parse trees."),
        (tm_id, "The Church-Turing Thesis states that anything computable can be computed by a ___.", "fill_in_blank", "Turing Machine", None, None, None, None, Some("Named after Alan Turing"), "The Church-Turing Thesis posits that Turing Machines capture the intuitive notion of computability."),
        (tm_id, "Who proved the Halting Problem is undecidable?", "multiple_choice", "Alan Turing", Some("Kurt Gödel"), Some("Alan Turing"), Some("Alonzo Church"), Some("John von Neumann"), Some("1936, same year as the Turing Machine paper"), "Alan Turing proved the Halting Problem undecidable in 1936 using a diagonalization argument."),
        (tm_id, "True or false: A universal Turing Machine can simulate any other Turing Machine.", "true_false", "true", Some("true"), Some("false"), None, None, Some("It's the theoretical ancestor of programmable computers"), "True. A UTM takes a description of any TM and its input, then simulates that TM — making it a general-purpose computer."),
        (tm_id, "Rice's Theorem says that any ___ property of programs is undecidable.", "fill_in_blank", "non-trivial", None, None, None, None, Some("Trivial = true for all or false for all"), "Rice's Theorem: any non-trivial semantic property of the language recognized by a Turing Machine is undecidable."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    // Learning paths
    let paths = [
        ("automata theory", 1, fa_id, "Finite automata — the simplest computational models"),
        ("automata theory", 2, re_id, "Regular expressions and regular languages"),
        ("automata theory", 3, cfg_id, "Context-free grammars and pushdown automata"),
        ("automata theory", 4, tm_id, "Turing machines, computability, and the limits of computation"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_philosophy_of_mind(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Philosophy of Mind'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Philosophy of Mind', 'Consciousness, AI, free will, and the nature of thought — where philosophy meets cognitive science.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Philosophy of Mind'", [], |r| r.get(0),
    )?;

    let topics = [
        (subj_id, "The Mind-Body Problem", "beginner", 1),
        (subj_id, "Consciousness", "intermediate", 2),
        (subj_id, "Artificial Intelligence & Minds", "intermediate", 3),
        (subj_id, "Free Will", "advanced", 4),
    ];
    for (sid, name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sid, name, diff, order],
        )?;
    }

    let mb_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'The Mind-Body Problem'",
        [subj_id], |r| r.get(0),
    )?;
    let con_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Consciousness'",
        [subj_id], |r| r.get(0),
    )?;
    let ai_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Artificial Intelligence & Minds'",
        [subj_id], |r| r.get(0),
    )?;
    let fw_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE subject_id = ?1 AND name = 'Free Will'",
        [subj_id], |r| r.get(0),
    )?;

    let lessons: Vec<LessonRow> = vec![
        (mb_id, "Dualism vs Physicalism", "The mind-body problem asks: what is the relationship between mental states (thoughts, feelings) and physical states (brain activity)?\n\n**Dualism** (Descartes): Mind and body are fundamentally different substances.\n- Substance dualism: mind is non-physical\n- Interaction problem: how does a non-physical mind cause physical actions?\n- Descartes proposed the pineal gland as the interface (now discredited)\n\n**Physicalism** (Materialism): Everything is physical, including the mind.\n- Identity theory: mental states ARE brain states (pain = C-fiber firing)\n- Functionalism: mental states are defined by their functional role, not their physical substrate\n- Eliminativism: 'beliefs' and 'desires' are folk psychology that neuroscience will replace\n\nModern neuroscience generally supports physicalism, but explaining subjective experience remains a challenge.", 1),
        (con_id, "The Hard Problem of Consciousness", "David Chalmers (1995) distinguished the 'easy' and 'hard' problems of consciousness.\n\n**Easy problems** (hard in practice, but conceptually clear):\n- How does the brain integrate information?\n- How can we discriminate stimuli and react?\n- How does attention work?\n\n**The Hard Problem:** Why is there subjective experience at all? Why does seeing red FEEL like something? This is the problem of **qualia** — the qualitative, subjective character of experience.\n\nThought experiments:\n- **Mary's Room:** Mary knows everything about color science but has never seen color. When she sees red for the first time, does she learn something new?\n- **Philosophical Zombies:** Could there be a being physically identical to you but with no inner experience?\n\nThese challenges suggest that explaining consciousness may require fundamentally new ideas.", 1),
        (ai_id, "Can Machines Think?", "Alan Turing (1950) proposed the **Turing Test**: if a machine can fool a human into thinking it's human through conversation, it exhibits intelligent behavior.\n\n**The Chinese Room** (John Searle, 1980):\nImagine a person in a room following rules to manipulate Chinese symbols — producing correct responses without understanding Chinese. Searle argues that computers similarly manipulate symbols without genuine understanding.\n\n**Strong AI:** Machines can have genuine minds and consciousness.\n**Weak AI:** Machines can simulate intelligent behavior without truly understanding.\n\nModern large language models can pass versions of the Turing Test, but the question of whether they 'understand' or merely process patterns remains deeply debated. The answer depends on your theory of mind — a functionalist might say yes, a dualist no.", 1),
        (fw_id, "Determinism & Free Will", "If the universe follows physical laws, are our choices truly free?\n\n**Hard Determinism:** Every event (including your decisions) is caused by prior events. Free will is an illusion.\n\n**Libertarianism** (philosophical, not political): We have genuine free will. Quantum indeterminacy or agent causation provides the gap.\n- Problem: randomness isn't the same as free choice.\n\n**Compatibilism** (most popular among philosophers): Free will is compatible with determinism. You act freely when you act according to your own desires without external coercion, even if those desires are determined.\n- Frankfurt cases: You can be morally responsible even if you couldn't have done otherwise.\n\n**Neuroscience angle:** Libet's experiments (1983) showed brain activity (readiness potential) precedes conscious awareness of decisions by ~350ms. But the interpretation is hotly debated.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (mb_id, "mind-body problem", "The mind-body problem asks how mental states (thoughts, feelings) relate to physical states (brain activity).", Some("Imagine you're watching a movie on your phone. The movie is the 'mind' — the story, emotions, images. The phone's circuits are the 'body.' The mind-body problem asks: is the movie just what the circuits do, or is it something extra?"), Some("If scientists could perfectly copy every atom in your brain, would the copy have your consciousness?")),
        (con_id, "consciousness", "Consciousness is the subjective experience of being aware — the 'what it's like' of seeing, thinking, and feeling.", Some("Consciousness is like the difference between a security camera and your eyes. The camera records images, but nobody is 'home' watching. Your eyes are connected to someone who experiences seeing. What makes that difference?"), Some("Do you think a sufficiently advanced AI could be conscious? What would count as evidence?")),
        (ai_id, "Chinese Room", "Searle's Chinese Room argues that symbol manipulation alone doesn't produce understanding — syntax isn't semantics.", Some("A calculator can do arithmetic faster than you, but does it 'understand' numbers? The Chinese Room argument says: even if a machine produces perfect outputs, that doesn't mean it understands the way you do."), Some("Is there a meaningful difference between perfectly simulating understanding and actually understanding?")),
        (fw_id, "free will", "Free will is the ability to have chosen differently — the idea that you are the genuine author of your actions.", Some("Imagine a river flowing downhill. It follows physics perfectly. But we don't say the river 'chose' its path. Are you like the river — following brain chemistry — or is there something more to your choices?"), Some("If you found out the universe is deterministic, would that change how you think about blame and praise?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let questions: Vec<QuizRow> = vec![
        (mb_id, "Descartes' view that mind and body are separate substances is called ___.", "fill_in_blank", "dualism", None, None, None, None, Some("Two kinds of stuff"), "Descartes proposed substance dualism: the mind is a non-physical substance distinct from the physical body."),
        (mb_id, "Which theory says mental states ARE brain states?", "multiple_choice", "Identity theory", Some("Dualism"), Some("Functionalism"), Some("Identity theory"), Some("Behaviorism"), Some("Mental states are identical to neural states"), "Identity theory (or type physicalism) holds that every mental state is identical to a specific brain state."),
        (mb_id, "True or false: Functionalism says mental states depend on what they're made of.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Function over substance"), "False. Functionalism says mental states are defined by their causal role (function), not their physical makeup."),
        (con_id, "The 'Hard Problem of Consciousness' was named by ___.", "fill_in_blank", "David Chalmers", None, None, None, None, Some("An Australian philosopher, in 1995"), "David Chalmers coined the term in 1995, distinguishing it from the 'easy problems' of cognitive science."),
        (con_id, "In Mary's Room, what does Mary learn when she first sees red?", "multiple_choice", "What red looks like (qualia)", Some("The wavelength of red light"), Some("What red looks like (qualia)"), Some("How the eye detects red"), Some("Nothing new"), Some("She already knew all the physical facts"), "Mary gains knowledge of qualia — the subjective experience of seeing red — which she couldn't get from physical facts alone."),
        (ai_id, "The Turing Test evaluates whether a machine can ___.", "fill_in_blank", "fool a human into thinking it is human", None, None, None, None, Some("Proposed by Alan Turing in 1950"), "The Turing Test measures whether a machine's conversation is indistinguishable from a human's."),
        (ai_id, "Searle's Chinese Room argument targets which claim about AI?", "multiple_choice", "Strong AI", Some("Weak AI"), Some("Strong AI"), Some("Machine learning"), Some("Neural networks"), Some("The claim that machines can genuinely understand"), "The Chinese Room argues against Strong AI — the claim that a computer running a program could have genuine understanding."),
        (fw_id, "The philosophical position that free will and determinism are compatible is called ___.", "fill_in_blank", "compatibilism", None, None, None, None, Some("They are 'compatible'"), "Compatibilism holds that free will can exist even in a deterministic universe, as long as actions flow from one's own desires."),
        (fw_id, "True or false: Libet's experiments showed conscious decisions precede brain activity.", "true_false", "false", Some("true"), Some("false"), None, None, Some("The readiness potential comes BEFORE awareness"), "False. Libet found brain activity (readiness potential) precedes conscious awareness of the decision by ~350ms."),
        (fw_id, "Hard determinism claims that free will is ___.", "multiple_choice", "An illusion", Some("Compatible with determinism"), Some("An illusion"), Some("Caused by quantum effects"), Some("Only available to humans"), Some("If everything is caused, choices aren't free"), "Hard determinism holds that since all events are causally determined, genuine free will cannot exist."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in &questions {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    let paths = [
        ("philosophy of mind", 1, mb_id, "The mind-body problem — dualism, physicalism, functionalism"),
        ("philosophy of mind", 2, con_id, "Consciousness and the hard problem"),
        ("philosophy of mind", 3, ai_id, "Can machines think? Turing Test and Chinese Room"),
        ("philosophy of mind", 4, fw_id, "Free will, determinism, and compatibilism"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

// ── Organic Chemistry & Graph Theory subjects ──────────────────────────

pub fn seed_organic_chemistry(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Organic Chemistry'", [], |r| r.get(0),
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Organic Chemistry', 'The chemistry of carbon compounds — functional groups, reactions, and the molecules of life.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Organic Chemistry'", [], |r| r.get(0))?;

    let topics = [
        ("Hydrocarbons", "beginner", 1),
        ("Functional Groups", "beginner", 2),
        ("Isomerism", "intermediate", 3),
        ("Reaction Mechanisms", "intermediate", 4),
        ("Stereochemistry", "advanced", 5),
        ("Polymers", "intermediate", 6),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let hc_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Hydrocarbons'", [subj_id], |r| r.get(0))?;
    let fg_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Functional Groups'", [subj_id], |r| r.get(0))?;
    let iso_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Isomerism'", [subj_id], |r| r.get(0))?;
    let rm_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Reaction Mechanisms'", [subj_id], |r| r.get(0))?;
    let sc_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Stereochemistry'", [subj_id], |r| r.get(0))?;
    let poly_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Polymers'", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (hc_id, "Alkanes, Alkenes, and Alkynes", "Hydrocarbons are molecules made of only carbon and hydrogen. Alkanes have single bonds (saturated), alkenes have at least one double bond, and alkynes have at least one triple bond. The general formulas are CnH2n+2, CnH2n, and CnH2n−2 respectively.", 1),
        (hc_id, "Naming Hydrocarbons", "IUPAC nomenclature uses prefixes (meth-, eth-, prop-, but-) for the carbon chain length and suffixes (-ane, -ene, -yne) for bond type. Branch chains are named as substituents with position numbers.", 2),
        (fg_id, "Common Functional Groups", "Functional groups determine chemical reactivity. Key groups: -OH (hydroxyl/alcohol), -COOH (carboxyl/acid), -NH2 (amine), C=O (carbonyl), -CHO (aldehyde). Each gives the molecule distinct properties.", 1),
        (fg_id, "Alcohols, Aldehydes, and Ketones", "Alcohols contain -OH bonded to a carbon. Aldehydes have C=O at the end of a chain (R-CHO). Ketones have C=O between two carbons (R-CO-R'). Oxidation converts alcohols → aldehydes → carboxylic acids.", 2),
        (iso_id, "Structural and Geometric Isomers", "Structural isomers have the same molecular formula but different connectivity. Geometric (cis/trans) isomers differ in spatial arrangement around a double bond. Cis = same side, trans = opposite side.", 1),
        (rm_id, "SN1 and SN2 Reactions", "In SN2 reactions, the nucleophile attacks simultaneously as the leaving group departs (one step, inversion). In SN1 reactions, the leaving group departs first forming a carbocation, then the nucleophile attacks (two steps, racemization).", 1),
        (rm_id, "Electrophilic Addition", "Alkenes undergo electrophilic addition: an electrophile attacks the electron-rich double bond. In HBr addition, the H+ adds first (Markovnikov's rule: H goes to the carbon with more H's), then Br− attacks.", 2),
        (sc_id, "Chirality and Enantiomers", "A chiral carbon has four different substituents. Enantiomers are non-superimposable mirror images. They have identical physical properties except they rotate plane-polarized light in opposite directions (R vs S configuration).", 1),
        (poly_id, "Addition and Condensation Polymers", "Addition polymers form when monomers with double bonds join (e.g., polyethylene from ethylene). Condensation polymers form when monomers link by losing a small molecule like water (e.g., nylon, polyester).", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (hc_id, "Saturation", "A saturated hydrocarbon has only single bonds (all carbon-hydrogen bonding slots are 'full'). Unsaturated hydrocarbons have double or triple bonds.", Some("Think of a sponge: saturated = soaked full, unsaturated = can absorb more."), Some("Why are alkenes more reactive than alkanes?")),
        (fg_id, "Functional Group", "A specific atom or group of atoms within a molecule that determines its chemical behavior. Same functional group = similar reactions regardless of the rest of the molecule.", Some("Like a power tool attachment — the drill body stays the same, but the bit determines what it can do."), Some("What functional group makes vinegar sour?")),
        (iso_id, "Isomerism", "Molecules with the same molecular formula but different structural arrangements. Like anagrams — same letters, different words, different meanings.", Some("'Listen' and 'silent' use the same letters but mean different things — isomers use the same atoms but connect differently."), Some("Can you draw two structural isomers of C4H10?")),
        (rm_id, "Nucleophile vs Electrophile", "A nucleophile is electron-rich and seeks positive centers. An electrophile is electron-poor and seeks negative centers. Reactions happen when they meet.", Some("Nucleophile = the generous friend who always shares. Electrophile = the friend who always borrows."), None),
        (sc_id, "Chirality", "A molecule is chiral if it cannot be superimposed on its mirror image, like left and right hands. This arises from a carbon with four different groups attached.", Some("Your hands are mirror images but you can't stack them perfectly — that's chirality."), Some("Why does chirality matter in pharmaceuticals?")),
        (poly_id, "Polymerization", "The process of linking many small monomer molecules into a long chain polymer. Addition polymerization opens double bonds; condensation polymerization releases water.", Some("Like snapping LEGO bricks together — each brick is a monomer, the finished structure is a polymer."), None),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions
    let quizzes: &[QuizRowHint] = &[
        (hc_id, "What is the general formula for alkanes?", "multiple_choice", "CnH2n+2", Some("CnH2n"), Some("CnH2n+2"), Some("CnH2n−2"), Some("CnHn"), "Saturated hydrocarbons", "Alkanes are saturated — all single bonds — so each carbon bonds to the maximum number of hydrogens: CnH2n+2."),
        (hc_id, "True or false: Alkenes contain at least one triple bond.", "true_false", "false", None, None, None, None, "Think about bond types", "Alkenes contain double bonds. Alkynes contain triple bonds."),
        (hc_id, "What is the IUPAC name for CH3-CH2-CH3?", "fill_in_blank", "propane", None, None, None, None, "Three carbons, all single bonds", "Three carbon alkane: prop- (3 carbons) + -ane (single bonds) = propane."),
        (hc_id, "Which hydrocarbon is most reactive?", "multiple_choice", "Ethyne", Some("Ethane"), Some("Ethene"), Some("Ethyne"), Some("Methane"), "More bonds = more reactive", "Alkynes (triple bonds) are most reactive due to the high electron density in the triple bond."),
        (fg_id, "Which functional group makes a molecule an alcohol?", "multiple_choice", "-OH", Some("-OH"), Some("-COOH"), Some("-NH2"), Some("-CHO"), "Think about ethanol", "The hydroxyl group (-OH) defines alcohols. Ethanol is CH3CH2OH."),
        (fg_id, "What functional group is present in acetic acid (vinegar)?", "fill_in_blank", "carboxyl", None, None, None, None, "Acid group", "Acetic acid (CH3COOH) contains the carboxyl group (-COOH), which makes it acidic."),
        (fg_id, "True or false: Aldehydes and ketones both contain a carbonyl group.", "true_false", "true", None, None, None, None, "C=O is the key", "Both have C=O. Aldehydes have it at the end of the chain, ketones in the middle."),
        (fg_id, "Which type of organic compound contains nitrogen?", "multiple_choice", "Amine", Some("Alcohol"), Some("Ether"), Some("Amine"), Some("Ester"), "Think NH2", "Amines contain the -NH2 group. They're derived from ammonia (NH3) with one H replaced."),
        (iso_id, "Structural isomers of C4H10 include butane and ___.", "fill_in_blank", "isobutane", None, None, None, None, "Branched version", "Isobutane (2-methylpropane) is the branched isomer of butane."),
        (iso_id, "In cis-2-butene, the methyl groups are on which side of the double bond?", "multiple_choice", "Same side", Some("Same side"), Some("Opposite sides"), Some("Alternating"), Some("Random"), "Cis means 'on this side'", "Cis = same side. Trans = opposite sides. Cis-2-butene has both CH3 groups on the same side."),
        (rm_id, "In an SN2 reaction, how many steps are there?", "fill_in_blank", "1", None, None, None, None, "Simultaneous attack and departure", "SN2 is a concerted (one-step) mechanism — the nucleophile attacks as the leaving group departs."),
        (rm_id, "Markovnikov's rule states that H adds to the carbon with ___.", "fill_in_blank", "more hydrogens", None, None, None, None, "The rich get richer", "Markovnikov: in HX addition to an alkene, H goes to the carbon already bearing more H atoms."),
        (rm_id, "Which reaction type involves a carbocation intermediate?", "multiple_choice", "SN1", Some("SN2"), Some("SN1"), Some("E2"), Some("Addition"), "Two-step mechanism", "SN1 proceeds via a carbocation intermediate — the leaving group departs first, then the nucleophile attacks."),
        (sc_id, "How many different groups must be attached to a carbon for it to be chiral?", "fill_in_blank", "4", None, None, None, None, "All different", "A chiral center (stereocenter) has four different substituents attached to one carbon."),
        (sc_id, "Enantiomers rotate plane-polarized light in ___ directions.", "multiple_choice", "Opposite", Some("Same"), Some("Opposite"), Some("No"), Some("Random"), "Mirror image = opposite", "Enantiomers are mirror images and rotate light equally but in opposite directions (+/−)."),
        (poly_id, "Polyethylene is an example of what type of polymer?", "multiple_choice", "Addition", Some("Addition"), Some("Condensation"), Some("Copolymer"), Some("Natural"), "Double bonds open up", "Polyethylene forms by addition polymerization — ethylene monomers link when their C=C double bonds open."),
        (poly_id, "True or false: Condensation polymerization releases a small molecule like water.", "true_false", "true", None, None, None, None, "Condensation = losing something", "Condensation polymers form when monomers join by eliminating a small molecule (often H2O)."),
        (poly_id, "Name one natural polymer found in living organisms.", "fill_in_blank", "protein", None, None, None, None, "Made of amino acids", "Proteins are natural condensation polymers — amino acids link via peptide bonds, releasing water."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    let paths = [
        ("organic chemistry", 1, hc_id, "Start with hydrocarbons — the carbon backbone"),
        ("organic chemistry", 2, fg_id, "Learn functional groups that determine reactivity"),
        ("organic chemistry", 3, iso_id, "Understand isomerism — same formula, different structures"),
        ("organic chemistry", 4, rm_id, "Master reaction mechanisms — how reactions actually happen"),
        ("organic chemistry", 5, sc_id, "Explore stereochemistry — 3D molecular arrangement"),
        ("organic chemistry", 6, poly_id, "Apply knowledge to polymers — giant molecules"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_graph_theory(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Graph Theory'", [], |r| r.get(0),
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Graph Theory', 'The mathematics of networks — vertices, edges, paths, and the structures that connect everything.')",
        [],
    )?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Graph Theory'", [], |r| r.get(0))?;

    let topics = [
        ("Vertices and Edges", "beginner", 1),
        ("Paths and Cycles", "beginner", 2),
        ("Trees", "intermediate", 3),
        ("Graph Coloring", "intermediate", 4),
        ("Eulerian and Hamiltonian Graphs", "advanced", 5),
        ("Planar Graphs", "advanced", 6),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let ve_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Vertices and Edges'", [subj_id], |r| r.get(0))?;
    let pc_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Paths and Cycles'", [subj_id], |r| r.get(0))?;
    let tree_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Trees'", [subj_id], |r| r.get(0))?;
    let gc_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Graph Coloring'", [subj_id], |r| r.get(0))?;
    let eh_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Eulerian and Hamiltonian Graphs'", [subj_id], |r| r.get(0))?;
    let pg_id: i64 = conn.query_row("SELECT id FROM topics WHERE subject_id=?1 AND name='Planar Graphs'", [subj_id], |r| r.get(0))?;

    let lessons: Vec<LessonRow> = vec![
        (ve_id, "What is a Graph?", "A graph G = (V, E) consists of a set of vertices (nodes) V and a set of edges E connecting pairs of vertices. Graphs model relationships: social networks, road maps, molecular structures. The degree of a vertex is the number of edges connected to it.", 1),
        (ve_id, "Directed vs Undirected Graphs", "In an undirected graph, edges have no direction (friendship is mutual). In a directed graph (digraph), edges have direction (following someone on Twitter isn't mutual). The Handshaking Lemma states that the sum of all vertex degrees equals twice the number of edges.", 2),
        (pc_id, "Paths, Walks, and Cycles", "A walk is any sequence of adjacent vertices. A path is a walk with no repeated vertices. A cycle is a path that starts and ends at the same vertex. The shortest path between two vertices is called the distance.", 1),
        (pc_id, "Connected Graphs", "A graph is connected if there's a path between every pair of vertices. A connected component is a maximal connected subgraph. Disconnected graphs have multiple components.", 2),
        (tree_id, "Trees and Forests", "A tree is a connected graph with no cycles. A forest is a graph with no cycles (a collection of trees). A tree with n vertices has exactly n−1 edges. Every tree has at least one leaf (vertex of degree 1).", 1),
        (tree_id, "Spanning Trees", "A spanning tree of a connected graph is a subgraph that is a tree and includes all vertices. Kruskal's and Prim's algorithms find minimum spanning trees — spanning trees with minimum total edge weight.", 2),
        (gc_id, "Graph Coloring Basics", "A proper coloring assigns colors to vertices so that no two adjacent vertices share a color. The chromatic number χ(G) is the minimum number of colors needed. A bipartite graph has χ(G) = 2.", 1),
        (gc_id, "The Four Color Theorem", "Every planar graph can be properly colored with at most 4 colors. This was the first major theorem proved with computer assistance (1976, Appel & Haken). It means any map can be colored with 4 colors so no adjacent regions share a color.", 2),
        (eh_id, "Euler Paths and Circuits", "An Euler path visits every edge exactly once. An Euler circuit is an Euler path that starts and ends at the same vertex. A connected graph has an Euler circuit iff every vertex has even degree (Euler's theorem, 1736 — the Königsberg bridge problem).", 1),
        (eh_id, "Hamiltonian Paths and Cycles", "A Hamiltonian path visits every vertex exactly once. A Hamiltonian cycle is a Hamiltonian path that returns to the starting vertex. Unlike Euler paths, there's no simple necessary and sufficient condition — the Hamiltonian problem is NP-complete.", 2),
        (pg_id, "Planar Graphs and Euler's Formula", "A planar graph can be drawn in the plane without edge crossings. Euler's formula for connected planar graphs: V − E + F = 2, where V = vertices, E = edges, F = faces (including the outer face). This limits edges: E ≤ 3V − 6.", 1),
        (pg_id, "Kuratowski's Theorem", "A graph is planar if and only if it contains no subdivision of K5 (complete graph on 5 vertices) or K3,3 (complete bipartite graph). These are the two fundamental non-planar graphs.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (ve_id, "Graph", "A mathematical structure of nodes (vertices) connected by links (edges). Abstractly captures any relationship between pairs of things.", Some("Think of a social network: people are vertices, friendships are edges."), Some("What's the degree of a vertex with 5 friends?")),
        (pc_id, "Cycle", "A path that starts and ends at the same vertex without repeating any vertex in between.", Some("Like walking around a city block and ending up where you started."), Some("What's the shortest possible cycle?")),
        (tree_id, "Tree", "A connected graph with no cycles. The simplest way to connect all vertices with the fewest edges.", Some("Like a family tree or organizational chart — there's exactly one path between any two nodes."), Some("How many edges does a tree with 10 vertices have?")),
        (gc_id, "Chromatic Number", "The minimum number of colors needed to properly color a graph (no adjacent vertices share a color).", Some("Like scheduling exams so no student has two exams at the same time — each time slot is a 'color'."), Some("What's the chromatic number of a cycle with 5 vertices?")),
        (eh_id, "Euler Circuit", "A route that traverses every edge exactly once and returns to the start. Only possible when all vertices have even degree.", Some("Imagine a postal worker who must walk every street exactly once and return home."), None),
        (pg_id, "Planar Graph", "A graph that can be drawn flat without any edges crossing.", Some("Like drawing a circuit diagram where no wires cross — some circuits need multiple layers (non-planar)."), Some("Is the complete graph K4 planar?")),
    ];
    for (tid, concept, explanation, analogy, follow_up) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let quizzes: &[QuizRowHint] = &[
        (ve_id, "In a graph G = (V, E), what does V represent?", "multiple_choice", "Vertices", Some("Vertices"), Some("Values"), Some("Vectors"), Some("Variables"), "Nodes in the graph", "V is the set of vertices (nodes) — the objects being connected."),
        (ve_id, "The Handshaking Lemma states that the sum of all vertex degrees equals ___.", "fill_in_blank", "2E", None, None, None, None, "Each edge contributes to two vertices", "Every edge connects two vertices, so it adds 1 to the degree of each endpoint: sum of degrees = 2|E|."),
        (ve_id, "True or false: In a directed graph, edges have no direction.", "true_false", "false", None, None, None, None, "Directed = has direction", "Directed graphs (digraphs) have edges with direction — from one vertex to another."),
        (pc_id, "A path that starts and ends at the same vertex is called a ___.", "fill_in_blank", "cycle", None, None, None, None, "Round trip", "A cycle is a closed path — it returns to its starting vertex without repeating any other vertex."),
        (pc_id, "What is the minimum number of vertices in a cycle?", "fill_in_blank", "3", None, None, None, None, "Smallest loop", "The smallest cycle is a triangle — 3 vertices connected in a loop (C3)."),
        (pc_id, "A graph is connected if there is a path between every pair of ___.", "fill_in_blank", "vertices", None, None, None, None, "All nodes reachable", "Connected means you can get from any vertex to any other vertex by following edges."),
        (tree_id, "How many edges does a tree with n vertices have?", "fill_in_blank", "n-1", None, None, None, None, "Minimal connectivity", "A tree is minimally connected: removing any edge disconnects it. It always has exactly n−1 edges."),
        (tree_id, "True or false: A tree can contain a cycle.", "true_false", "false", None, None, None, None, "Definition of a tree", "A tree is defined as a connected acyclic graph — no cycles allowed."),
        (tree_id, "Which algorithm finds a minimum spanning tree using edge sorting?", "multiple_choice", "Kruskal's", Some("Dijkstra's"), Some("Kruskal's"), Some("Bellman-Ford"), Some("Floyd-Warshall"), "Sort edges by weight", "Kruskal's algorithm sorts all edges by weight and greedily adds the smallest edge that doesn't create a cycle."),
        (gc_id, "What is the chromatic number of a bipartite graph?", "fill_in_blank", "2", None, None, None, None, "Two groups, no conflicts", "Bipartite graphs can be 2-colored: one color per partition. Vertices within a partition are never adjacent."),
        (gc_id, "The Four Color Theorem applies to which type of graphs?", "multiple_choice", "Planar graphs", Some("Complete graphs"), Some("Planar graphs"), Some("Bipartite graphs"), Some("Trees"), "Flat drawings", "The Four Color Theorem: every planar graph (drawable without crossings) can be 4-colored."),
        (gc_id, "True or false: A graph with chromatic number 1 has no edges.", "true_false", "true", None, None, None, None, "If only one color suffices...", "If all vertices can be the same color, no two adjacent vertices exist — meaning no edges."),
        (eh_id, "A connected graph has an Euler circuit if and only if every vertex has ___ degree.", "fill_in_blank", "even", None, None, None, None, "Euler's theorem", "For an Euler circuit (visiting every edge once, returning to start), every vertex must have even degree."),
        (eh_id, "The Hamiltonian path problem is in which complexity class?", "multiple_choice", "NP-complete", Some("P"), Some("NP-complete"), Some("PSPACE"), Some("Undecidable"), "Very hard problem", "Determining whether a Hamiltonian path exists is NP-complete — no known polynomial-time algorithm."),
        (eh_id, "What famous problem did Euler solve in 1736?", "multiple_choice", "Königsberg bridge problem", Some("Traveling salesman"), Some("Königsberg bridge problem"), Some("Four color problem"), Some("Shortest path"), "Seven bridges", "Euler proved it was impossible to cross all 7 bridges of Königsberg exactly once — founding graph theory."),
        (pg_id, "Euler's formula for connected planar graphs is V − E + F = ___.", "fill_in_blank", "2", None, None, None, None, "Euler's polyhedral formula", "V − E + F = 2 for any connected planar graph, where F includes the unbounded outer face."),
        (pg_id, "Which graph is NOT planar?", "multiple_choice", "K5", Some("K4"), Some("K3"), Some("K5"), Some("A tree"), "Five vertices, all connected", "K5 (complete graph on 5 vertices) is non-planar. K4 is planar. Trees are always planar."),
        (pg_id, "True or false: Every tree is a planar graph.", "true_false", "true", None, None, None, None, "No cycles = easy to draw flat", "Trees have no cycles and can always be drawn without edge crossings — they're planar."),
    ];
    for (tid, q, qtype, correct, a, b, c, d, hint, expl) in quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qtype, correct, *a, *b, *c, *d, hint, expl],
        )?;
    }

    let paths = [
        ("graph theory", 1, ve_id, "Foundations: vertices, edges, and graph types"),
        ("graph theory", 2, pc_id, "Paths, cycles, and connectivity"),
        ("graph theory", 3, tree_id, "Trees — the simplest connected graphs"),
        ("graph theory", 4, gc_id, "Graph coloring and the chromatic number"),
        ("graph theory", 5, eh_id, "Euler and Hamiltonian paths — traversal problems"),
        ("graph theory", 6, pg_id, "Planar graphs, Euler's formula, and Kuratowski"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_thermodynamics(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO subjects (name, description) VALUES (?1, ?2)",
        ["Thermodynamics", "The science of heat, energy, and work — from engines to entropy."],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Thermodynamics'", [], |r| r.get(0),
    )?;

    let topics = [
        (subj_id, "Zeroth & First Law", "beginner", 1),
        (subj_id, "Second Law & Entropy", "intermediate", 2),
        (subj_id, "Heat Engines & Carnot Cycle", "intermediate", 3),
        (subj_id, "Thermodynamic Potentials", "advanced", 4),
        (subj_id, "Phase Transitions", "advanced", 5),
    ];
    for (sid, name, diff, ord) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1,?2,?3,?4)",
            rusqlite::params![sid, name, diff, ord],
        )?;
    }

    let t1: i64 = conn.query_row("SELECT id FROM topics WHERE name='Zeroth & First Law'", [], |r| r.get(0))?;
    let t2: i64 = conn.query_row("SELECT id FROM topics WHERE name='Second Law & Entropy'", [], |r| r.get(0))?;
    let t3: i64 = conn.query_row("SELECT id FROM topics WHERE name='Heat Engines & Carnot Cycle'", [], |r| r.get(0))?;
    let t4: i64 = conn.query_row("SELECT id FROM topics WHERE name='Thermodynamic Potentials'", [], |r| r.get(0))?;
    let t5: i64 = conn.query_row("SELECT id FROM topics WHERE name='Phase Transitions'", [], |r| r.get(0))?;

    let lessons: Vec<LessonRow> = vec![
        (t1, "Temperature & Thermal Equilibrium", "The zeroth law states that if A is in thermal equilibrium with B, and B with C, then A is in equilibrium with C — establishing temperature as a measurable property.", 1),
        (t1, "Conservation of Energy", "The first law: energy cannot be created or destroyed, only transformed. In any process, dU = Q - W (change in internal energy = heat added minus work done).", 2),
        (t2, "Entropy & Disorder", "Entropy measures the number of microscopic configurations consistent with a system's macroscopic state. The second law: in an isolated system, entropy never decreases.", 1),
        (t2, "Irreversibility", "Real processes are irreversible — they increase total entropy. Heat flows spontaneously from hot to cold, never the reverse without work input.", 2),
        (t3, "Heat Engines", "A heat engine converts thermal energy into mechanical work by exploiting temperature differences between a hot source and a cold sink.", 1),
        (t3, "Carnot Efficiency", "The Carnot cycle is the most efficient possible engine between two temperatures: efficiency = 1 - Tc/Th. No real engine can exceed this.", 2),
        (t4, "Free Energy", "Gibbs free energy G = H - TS determines spontaneity at constant pressure and temperature. If dG < 0, the process is spontaneous.", 1),
        (t4, "Helmholtz & Enthalpy", "Helmholtz free energy F = U - TS governs constant-volume processes. Enthalpy H = U + PV is useful for constant-pressure processes.", 2),
        (t5, "States of Matter", "Phase transitions (melting, boiling, sublimation) occur when thermal energy overcomes intermolecular forces.", 1),
        (t5, "Critical Points & Phase Diagrams", "Beyond the critical point, liquid and gas become indistinguishable (supercritical fluid). Phase diagrams map transitions in P-T space.", 2),
    ];
    for (tid, title, content, ord) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1,?2,?3,?4)",
            rusqlite::params![tid, title, content, ord],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (t1, "First Law", "Energy is conserved: the change in internal energy equals heat added minus work done. Think of a bank account — deposits (heat) minus withdrawals (work) = balance change.", Some("A thermos minimizes heat transfer so internal energy stays constant."), Some("If you compress a gas in an insulated container, what happens to its temperature?")),
        (t2, "Entropy", "Entropy measures the number of microstates. High entropy = many possible arrangements. The universe trends toward maximum entropy.", Some("A shuffled deck has high entropy — astronomically more disordered arrangements than ordered ones."), Some("Can entropy ever decrease locally? What is the cost?")),
        (t3, "Carnot Cycle", "The theoretical maximum efficiency of a heat engine between temperatures Th and Tc. Real engines always fall short due to friction and irreversibility.", Some("Like a waterfall powering a mill — the bigger the height difference, the more work you extract."), Some("Why can't we build an engine that converts 100% of heat into work?")),
    ];
    for (tid, concept, expl, analogy, followup) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    // QuizRowHint: (topic_id, question, q_type, correct_answer, opt_a, opt_b, opt_c, opt_d, hint, difficulty)
    let quizzes: Vec<QuizRowHint> = vec![
        (t1, "What does the zeroth law of thermodynamics establish?", "multiple_choice", "Temperature as a measurable property", Some("Temperature as a measurable property"), Some("Entropy"), Some("Pressure"), Some("Volume"), "Think about what thermal equilibrium defines", "medium"),
        (t1, "In the first law, dU = Q - W. What does W represent?", "multiple_choice", "Work done by the system", Some("Work done by the system"), Some("Weight"), Some("Wavelength"), Some("Work done on the system"), "Energy leaving the system", "easy"),
        (t1, "True or false: Energy can be created in an exothermic reaction.", "true_false", "false", Some("true"), Some("false"), None, None, "The first law — energy is transformed not created", "easy"),
        (t1, "The first law is a statement of ___.", "fill_in_blank", "conservation of energy", None, None, None, None, "What fundamental principle does it express?", "easy"),
        (t2, "What happens to total entropy of an isolated system over time?", "multiple_choice", "It increases or stays the same", Some("It increases or stays the same"), Some("It decreases"), Some("It oscillates"), Some("It becomes zero"), "The second law", "medium"),
        (t2, "In Boltzmann's formula S = k_B ln(W), W represents ___.", "fill_in_blank", "number of microstates", None, None, None, None, "Boltzmann's formula", "hard"),
        (t2, "True or false: A refrigerator violates the second law.", "true_false", "false", Some("true"), Some("false"), None, None, "Consider external work input", "medium"),
        (t3, "The Carnot efficiency formula is ___.", "fill_in_blank", "1 - Tc/Th", None, None, None, None, "Uses cold and hot reservoir temperatures", "medium"),
        (t3, "A Carnot engine operates between 600K and 300K. Its efficiency is ___.", "fill_in_blank", "50%", None, None, None, None, "Apply 1 - Tc/Th", "medium"),
        (t3, "Order the Carnot cycle steps:", "ordering", "Isothermal expansion,Adiabatic expansion,Isothermal compression,Adiabatic compression", None, None, None, None, "Two isothermal and two adiabatic steps", "hard"),
        (t4, "When is a process spontaneous in terms of Gibbs free energy?", "multiple_choice", "dG < 0", Some("dG < 0"), Some("dG > 0"), Some("dG = 0"), Some("dH < 0 always"), "Think about free energy available for work", "medium"),
        (t4, "Match thermodynamic potentials with their natural variables:", "matching", "Gibbs=T and P;Helmholtz=T and V;Enthalpy=S and P;Internal Energy=S and V", None, None, None, None, "Each potential has two natural variables", "hard"),
        (t5, "What is the triple point?", "multiple_choice", "Where solid liquid and gas coexist", Some("Where solid liquid and gas coexist"), Some("Where only liquid exists"), Some("Maximum temperature"), Some("Zero entropy point"), "Three phases at once", "easy"),
        (t5, "True or false: Above the critical point there is no distinction between liquid and gas.", "true_false", "true", Some("true"), Some("false"), None, None, "Supercritical fluid", "medium"),
        (t5, "The energy absorbed during a phase change at constant temperature is called ___.", "fill_in_blank", "latent heat", None, None, None, None, "Hidden heat — temperature does not change during transition", "easy"),
    ];
    for (tid, q, qt, ans, oa, ob, oc, od, hint, diff) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?9,?10)",
            rusqlite::params![tid, q, qt, ans, oa, ob, oc, od, hint, diff],
        )?;
    }

    let paths = [
        ("thermodynamics", 1, t1, "Start with the fundamental laws"),
        ("thermodynamics", 2, t2, "Understand entropy and irreversibility"),
        ("thermodynamics", 3, t3, "Apply to heat engines"),
        ("thermodynamics", 4, t4, "Thermodynamic potentials and free energy"),
        ("thermodynamics", 5, t5, "Phase transitions and critical phenomena"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_cognitive_science(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO subjects (name, description) VALUES (?1, ?2)",
        ["Cognitive Science", "The interdisciplinary study of the mind — perception, memory, language, and decision-making."],
    )?;
    let subj_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Cognitive Science'", [], |r| r.get(0),
    )?;

    let topics = [
        (subj_id, "Perception & Attention", "beginner", 1),
        (subj_id, "Memory Systems", "beginner", 2),
        (subj_id, "Language & Cognition", "intermediate", 3),
        (subj_id, "Decision Making & Heuristics", "intermediate", 4),
        (subj_id, "Cognitive Development", "advanced", 5),
    ];
    for (sid, name, diff, ord) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1,?2,?3,?4)",
            rusqlite::params![sid, name, diff, ord],
        )?;
    }

    let t1: i64 = conn.query_row("SELECT id FROM topics WHERE name='Perception & Attention'", [], |r| r.get(0))?;
    let t2: i64 = conn.query_row("SELECT id FROM topics WHERE name='Memory Systems'", [], |r| r.get(0))?;
    let t3: i64 = conn.query_row("SELECT id FROM topics WHERE name='Language & Cognition'", [], |r| r.get(0))?;
    let t4: i64 = conn.query_row("SELECT id FROM topics WHERE name='Decision Making & Heuristics'", [], |r| r.get(0))?;
    let t5: i64 = conn.query_row("SELECT id FROM topics WHERE name='Cognitive Development'", [], |r| r.get(0))?;

    let lessons: Vec<LessonRow> = vec![
        (t1, "Selective Attention", "We cannot process everything — attention acts as a filter. The cocktail party effect shows we can focus on one conversation in a noisy room yet still notice our name.", 1),
        (t1, "Change Blindness", "Large changes in a visual scene can go unnoticed if they coincide with a disruption. This reveals that we do not store a detailed representation of the world.", 2),
        (t2, "Working Memory", "Baddeley's model: working memory has a central executive, phonological loop (verbal), visuospatial sketchpad (visual), and episodic buffer. Capacity is about 4 chunks.", 1),
        (t2, "Long-Term Memory", "Declarative memory (facts and events) vs procedural memory (skills). Consolidation transfers memories from hippocampus to cortex during sleep.", 2),
        (t3, "Sapir-Whorf Hypothesis", "Does language shape thought? The strong version (linguistic determinism) is largely rejected, but the weak version (linguistic relativity) has experimental support.", 1),
        (t3, "Language Acquisition", "Children acquire language through innate capacity (Chomsky's universal grammar) and environmental exposure. Critical period: roughly birth to puberty.", 2),
        (t4, "Cognitive Biases", "Heuristics are mental shortcuts that usually work but can cause systematic errors: anchoring, availability, representativeness, confirmation bias.", 1),
        (t4, "Prospect Theory", "Kahneman and Tversky showed people are loss-averse: losing $100 feels worse than gaining $100 feels good. We evaluate outcomes relative to a reference point.", 2),
        (t5, "Piaget's Stages", "Cognitive development proceeds through stages: sensorimotor (0-2), preoperational (2-7), concrete operational (7-11), formal operational (11+).", 1),
        (t5, "Theory of Mind", "Around age 4, children develop the ability to attribute mental states to others — understanding that others can hold false beliefs.", 2),
    ];
    for (tid, title, content, ord) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1,?2,?3,?4)",
            rusqlite::params![tid, title, content, ord],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (t1, "Selective Attention", "The brain filters sensory input to focus on relevant information. Without this filter we would be overwhelmed by sensory data.", Some("Like a spotlight on a dark stage — you illuminate one area at a time even though the whole stage exists."), Some("What happens to the unattended information? Is it completely lost?")),
        (t2, "Memory Consolidation", "New memories are initially fragile and gradually stabilize through consolidation especially during sleep. Rehearsal and spaced practice strengthen this.", Some("Like saving a document — RAM (working memory) is fast but volatile; the hard drive (long-term memory) is slower but permanent."), Some("Why does sleep deprivation impair memory formation?")),
        (t4, "Anchoring Bias", "Initial information disproportionately influences subsequent judgments. Even arbitrary anchors affect estimates.", Some("Like a ship's anchor — even if you try to drift you stay near where you first dropped anchor."), Some("How can awareness of anchoring help in negotiations?")),
    ];
    for (tid, concept, expl, analogy, followup) in &explanations {
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1,?2,?3,?4,?5)",
            rusqlite::params![tid, concept, expl, analogy, followup],
        )?;
    }

    let quizzes: Vec<QuizRowHint> = vec![
        (t1, "The cocktail party effect demonstrates which cognitive process?", "multiple_choice", "Selective attention", Some("Selective attention"), Some("Memory consolidation"), Some("Perception binding"), Some("Motor planning"), "Focusing on one voice in a crowd", "easy"),
        (t1, "True or false: Change blindness shows that our visual perception stores every detail.", "true_false", "false", Some("true"), Some("false"), None, None, "The opposite is true", "easy"),
        (t1, "The inability to notice large changes in visual scenes is called ___.", "fill_in_blank", "change blindness", None, None, None, None, "A blindness to changes", "medium"),
        (t2, "How many chunks can working memory typically hold?", "fill_in_blank", "4", None, None, None, None, "Cowan's updated estimate not Miller's 7", "medium"),
        (t2, "Match memory systems with their descriptions:", "matching", "Procedural=Skills and habits;Episodic=Personal experiences;Semantic=General knowledge;Working=Short-term manipulation", None, None, None, None, "Four types of memory", "medium"),
        (t2, "Order the memory processes:", "ordering", "Encoding,Storage,Consolidation,Retrieval", None, None, None, None, "From input to recall", "easy"),
        (t3, "Who proposed the concept of universal grammar?", "fill_in_blank", "Chomsky", None, None, None, None, "A famous MIT linguist", "easy"),
        (t3, "The weak version of the Sapir-Whorf hypothesis is called ___.", "fill_in_blank", "linguistic relativity", None, None, None, None, "Language influences but does not determine thought", "medium"),
        (t3, "True or false: The critical period for language acquisition ends around puberty.", "true_false", "true", Some("true"), Some("false"), None, None, "After this period native-like acquisition becomes harder", "medium"),
        (t4, "Which bias describes relying too heavily on the first piece of information encountered?", "multiple_choice", "Anchoring", Some("Anchoring"), Some("Confirmation bias"), Some("Availability heuristic"), Some("Framing effect"), "First information anchors the judgment", "easy"),
        (t4, "In prospect theory people are ___ — losses hurt more than equivalent gains feel good.", "fill_in_blank", "loss-averse", None, None, None, None, "Losing $100 vs gaining $100", "medium"),
        (t4, "Select ALL cognitive biases from this list:", "select_all", "Anchoring,Confirmation bias,Availability heuristic", Some("Photosynthesis"), Some("Anchoring"), Some("Confirmation bias"), Some("Availability heuristic"), "Three are biases one is biology", "medium"),
        (t5, "At what approximate age do children develop theory of mind?", "fill_in_blank", "4", None, None, None, None, "False belief task", "medium"),
        (t5, "Order Piaget's stages of cognitive development:", "ordering", "Sensorimotor,Preoperational,Concrete operational,Formal operational", None, None, None, None, "From birth to adolescence", "medium"),
        (t5, "True or false: In the preoperational stage children can perform conservation tasks.", "true_false", "false", Some("true"), Some("false"), None, None, "Conservation develops in the concrete operational stage", "medium"),
    ];
    for (tid, q, qt, ans, oa, ob, oc, od, hint, diff) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?9,?10)",
            rusqlite::params![tid, q, qt, ans, oa, ob, oc, od, hint, diff],
        )?;
    }

    let paths = [
        ("cognitive science", 1, t1, "How we perceive and attend to the world"),
        ("cognitive science", 2, t2, "Memory: encoding storage and retrieval"),
        ("cognitive science", 3, t3, "How language and thought interact"),
        ("cognitive science", 4, t4, "Decision-making and cognitive biases"),
        ("cognitive science", 5, t5, "How cognition develops across the lifespan"),
    ];
    for (goal, order, tid, desc) in &paths {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES (?1,?2,?3,?4)",
            rusqlite::params![goal, order, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_cloze_questions(conn: &Connection) -> Result<(), rusqlite::Error> {
    let arith_id: Option<i64> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Arithmetic'", [], |r| r.get(0),
    ).ok();

    if let Some(tid) = arith_id {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?5,?6)",
            rusqlite::params![tid, "The order of operations is: ___, exponents, ___, division, ___, subtraction.", "cloze", "parentheses;multiplication;addition", "Think PEMDAS", "medium"],
        )?;
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?5,?6)",
            rusqlite::params![tid, "In the equation 3 x ___ = 12, the missing number is ___.", "cloze", "4;4", "Division is the inverse of multiplication", "easy"],
        )?;
    }

    let cells_id: Option<i64> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Cells'", [], |r| r.get(0),
    ).ok();
    if let Some(tid) = cells_id {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?5,?6)",
            rusqlite::params![tid, "The ___ is the powerhouse of the cell, producing ___ through cellular ___.", "cloze", "mitochondria;ATP;respiration", "Energy production organelle", "medium"],
        )?;
    }

    let thermo_t1: Option<i64> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Zeroth & First Law'", [], |r| r.get(0),
    ).ok();
    if let Some(tid) = thermo_t1 {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?5,?6)",
            rusqlite::params![tid, "The first law states dU = ___ - ___, where U is internal energy.", "cloze", "Q;W", "Heat and work", "medium"],
        )?;
    }

    let mem_id: Option<i64> = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Memory Systems'", [], |r| r.get(0),
    ).ok();
    if let Some(tid) = mem_id {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?5,?6)",
            rusqlite::params![tid, "Baddeley's working memory model has four components: central executive, ___ loop, ___ sketchpad, and episodic ___.", "cloze", "phonological;visuospatial;buffer", "Verbal visual and integration components", "hard"],
        )?;
    }

    Ok(())
}

fn seed_ecology(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Ecology'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Ecology', 'The study of ecosystems, populations, and how organisms interact with each other and their environment.')",
        [],
    )?;
    let eco_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Ecology'",
        [],
        |r| r.get(0),
    )?;

    let topics = [
        ("Population Dynamics", "beginner"),
        ("Community Ecology", "intermediate"),
        ("Ecosystem Energy Flow", "beginner"),
        ("Biogeochemical Cycles", "intermediate"),
        ("Biodiversity & Conservation", "advanced"),
    ];
    for (name, diff) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty) VALUES (?1, ?2, ?3)",
            rusqlite::params![eco_id, name, diff],
        )?;
    }

    let pop_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Population Dynamics'", [], |r| r.get(0),
    )?;
    let comm_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Community Ecology'", [], |r| r.get(0),
    )?;
    let energy_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Ecosystem Energy Flow'", [], |r| r.get(0),
    )?;
    let cycles_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Biogeochemical Cycles'", [], |r| r.get(0),
    )?;
    let biodiv_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Biodiversity & Conservation'", [], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (pop_id, "Exponential vs Logistic Growth", "Populations grow exponentially (J-curve) when resources are unlimited, but switch to logistic growth (S-curve) as they approach carrying capacity (K). The growth rate r determines how fast a population grows.", 1),
        (pop_id, "Carrying Capacity & Limiting Factors", "Carrying capacity (K) is the maximum population size an environment can sustain. Limiting factors include food, water, space, and predation. Density-dependent factors intensify as population grows.", 2),
        (comm_id, "Symbiosis & Species Interactions", "Species interact through mutualism (+/+), commensalism (+/0), parasitism (+/−), and competition (−/−). These relationships shape community structure and drive coevolution.", 1),
        (comm_id, "Ecological Succession", "Primary succession occurs on bare rock; secondary succession follows a disturbance. Pioneer species arrive first, and through facilitation, the community progresses toward a climax community.", 2),
        (energy_id, "Trophic Levels & Food Webs", "Energy flows from producers (autotrophs) to primary consumers to secondary consumers. Only ~10% of energy transfers between trophic levels (10% rule). Food webs show complex feeding relationships.", 1),
        (energy_id, "Primary Productivity", "Gross primary productivity (GPP) is total photosynthesis. Net primary productivity (NPP = GPP − respiration) is energy available to consumers. Tropical rainforests and estuaries have the highest NPP.", 2),
        (cycles_id, "Carbon & Nitrogen Cycles", "Carbon cycles through atmosphere (CO₂), biosphere (organic compounds), lithosphere (fossil fuels), and hydrosphere (dissolved CO₂). Nitrogen fixation converts N₂ to usable NH₃ via bacteria.", 1),
        (cycles_id, "Water & Phosphorus Cycles", "The water cycle involves evaporation, transpiration, condensation, and precipitation. Phosphorus has no atmospheric phase — it cycles through rocks, soil, water, and organisms.", 2),
        (biodiv_id, "Measuring Biodiversity", "Biodiversity includes genetic, species, and ecosystem diversity. Species richness counts species; Shannon index (H') measures diversity including evenness. Higher H' = more diverse.", 1),
        (biodiv_id, "Conservation Biology", "Threats include habitat loss, invasive species, overexploitation, pollution, and climate change (HIPPO). Conservation strategies include protected areas, corridors, captive breeding, and rewilding.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Quiz questions
    let quizzes: Vec<QuizRowHint> = vec![
        (pop_id, "What shape describes logistic population growth?", "multiple_choice", "S-curve", Some("J-curve"), Some("W-curve"), Some("Linear"), None, "Logistic growth produces an S-shaped (sigmoid) curve as the population approaches carrying capacity.", "easy"),
        (pop_id, "True or false: Density-independent factors become stronger as population grows.", "true_false", "false", None, None, None, None, "Density-DEPENDENT factors become stronger with population size. Density-independent factors (storms, fires) affect all population sizes equally.", "easy"),
        (comm_id, "A clownfish living in a sea anemone is an example of:", "multiple_choice", "Mutualism", Some("Parasitism"), Some("Commensalism"), Some("Competition"), None, "Both species benefit: the clownfish gets protection, and the anemone gets food scraps and cleaning.", "easy"),
        (comm_id, "What is the first stage of primary succession?", "fill_in_blank", "colonization by pioneer species", None, None, None, None, "Pioneer species like lichens and mosses colonize bare rock first, breaking it down into soil.", "medium"),
        (energy_id, "Approximately what percentage of energy transfers between trophic levels?", "multiple_choice", "10%", Some("1%"), Some("50%"), Some("90%"), None, "The 10% rule: only about 10% of energy at one trophic level is passed to the next. The rest is lost as heat.", "easy"),
        (energy_id, "Which ecosystem has the highest net primary productivity?", "multiple_choice", "Tropical rainforest", Some("Open ocean"), Some("Desert"), Some("Tundra"), None, "Tropical rainforests have the highest NPP due to abundant sunlight, water, and warm temperatures year-round.", "medium"),
        (cycles_id, "Which process converts atmospheric N₂ into ammonia (NH₃)?", "fill_in_blank", "nitrogen fixation", None, None, None, None, "Nitrogen fixation is performed by bacteria (e.g., Rhizobium in root nodules) that convert N₂ to NH₃.", "medium"),
        (cycles_id, "True or false: Phosphorus has a significant atmospheric component.", "true_false", "false", None, None, None, None, "Unlike carbon and nitrogen, phosphorus does not have a gaseous phase. It cycles through rocks, water, soil, and organisms.", "easy"),
        (biodiv_id, "What does the Shannon diversity index (H') measure?", "multiple_choice", "Species diversity including evenness", Some("Only species richness"), Some("Genetic diversity"), Some("Ecosystem area"), None, "Shannon index accounts for both the number of species (richness) and their relative abundance (evenness).", "hard"),
        (biodiv_id, "Name the five major threats to biodiversity (HIPPO acronym).", "fill_in_blank", "Habitat loss, Invasive species, Pollution, Population growth, Overexploitation", None, None, None, None, "HIPPO: Habitat loss, Invasive species, Pollution, Population growth (human), Overexploitation.", "hard"),
    ];
    for (tid, q, qt, ca, a, b, c, d, expl, diff) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qt, ca, a, b, c, d, expl, diff],
        )?;
    }

    // Categorize quiz questions (new type)
    conn.execute(
        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?6,?7)",
        rusqlite::params![
            comm_id,
            "Categorize each interaction: clownfish+anemone, tapeworm+host, barnacle+whale, lion+hyena",
            "categorize",
            "Mutualism:clownfish+anemone;Parasitism:tapeworm+host;Commensalism:barnacle+whale;Competition:lion+hyena",
            "Think about who benefits and who is harmed",
            "Mutualism (+/+), Parasitism (+/−), Commensalism (+/0), Competition (−/−)",
            "hard"
        ],
    )?;

    // Learning paths
    let path_topics = [
        (pop_id, "Understand how populations grow and what limits them"),
        (energy_id, "Follow energy through trophic levels and food webs"),
        (comm_id, "Explore species interactions and community dynamics"),
        (cycles_id, "Trace matter through biogeochemical cycles"),
        (biodiv_id, "Measure and protect biodiversity"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('ecology foundations', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_abstract_algebra(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Abstract Algebra'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(false);
    if exists {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Abstract Algebra', 'The study of algebraic structures — groups, rings, and fields — and their properties and transformations.')",
        [],
    )?;
    let alg_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Abstract Algebra'",
        [],
        |r| r.get(0),
    )?;

    let topics = [
        ("Groups & Subgroups", "intermediate"),
        ("Ring Theory", "advanced"),
        ("Field Extensions", "advanced"),
        ("Homomorphisms & Isomorphisms", "intermediate"),
    ];
    for (name, diff) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty) VALUES (?1, ?2, ?3)",
            rusqlite::params![alg_id, name, diff],
        )?;
    }

    let groups_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Groups & Subgroups'", [], |r| r.get(0),
    )?;
    let rings_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Ring Theory'", [], |r| r.get(0),
    )?;
    let fields_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Field Extensions'", [], |r| r.get(0),
    )?;
    let homo_id: i64 = conn.query_row(
        "SELECT id FROM topics WHERE name = 'Homomorphisms & Isomorphisms'", [], |r| r.get(0),
    )?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (groups_id, "What is a Group?", "A group (G, •) is a set G with a binary operation satisfying four axioms: closure, associativity, identity element, and inverse element. Example: (ℤ, +) is a group with identity 0.", 1),
        (groups_id, "Subgroups & Lagrange's Theorem", "A subgroup H ≤ G is a subset of G that is itself a group under the same operation. Lagrange's theorem: |H| divides |G| for finite groups. The order of any element divides |G|.", 2),
        (groups_id, "Cyclic Groups", "A cyclic group is generated by a single element g: G = ⟨g⟩ = {gⁿ | n ∈ ℤ}. Every cyclic group is abelian. ℤₙ under addition is cyclic of order n.", 3),
        (rings_id, "Ring Axioms", "A ring (R, +, ·) is an abelian group under + with an associative multiplication that distributes over addition. A commutative ring has ab = ba. ℤ is a commutative ring.", 1),
        (rings_id, "Ideals & Quotient Rings", "An ideal I ⊆ R satisfies: r·a ∈ I and a·r ∈ I for all r ∈ R, a ∈ I. The quotient ring R/I consists of cosets a + I. Prime ideals yield integral domains.", 2),
        (fields_id, "Fields & Their Properties", "A field is a commutative ring where every nonzero element has a multiplicative inverse. ℚ, ℝ, ℂ, and ℤₚ (p prime) are fields. Fields have no zero divisors.", 1),
        (fields_id, "Field Extensions & Degree", "A field extension L/K means L is a field containing K. The degree [L:K] = dim_K(L). Example: [ℂ:ℝ] = 2, [ℚ(√2):ℚ] = 2. The tower law: [M:K] = [M:L]·[L:K].", 2),
        (homo_id, "Group Homomorphisms", "A group homomorphism φ: G → H satisfies φ(ab) = φ(a)φ(b). The kernel ker(φ) = {g | φ(g) = e_H} is a normal subgroup. The image im(φ) is a subgroup of H.", 1),
        (homo_id, "Isomorphism Theorems", "First Isomorphism Theorem: G/ker(φ) ≅ im(φ). An isomorphism is a bijective homomorphism. Isomorphic groups have identical algebraic structure.", 2),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    // Quiz questions
    let quizzes: Vec<QuizRowHint> = vec![
        (groups_id, "Which of these is NOT a group axiom?", "multiple_choice", "Commutativity", Some("Closure"), Some("Associativity"), Some("Identity"), None, "Commutativity is NOT required for a general group. Groups with commutativity are called abelian groups.", "medium"),
        (groups_id, "What is the identity element of (ℤ, +)?", "fill_in_blank", "0", None, None, None, None, "For addition on integers, 0 is the identity since a + 0 = 0 + a = a for all a.", "easy"),
        (groups_id, "True or false: Every cyclic group is abelian.", "true_false", "true", None, None, None, None, "Yes — if G = ⟨g⟩, then gᵃ · gᵇ = gᵃ⁺ᵇ = gᵇ · gᵃ, so the operation commutes.", "easy"),
        (groups_id, "If |G| = 12, which of these CANNOT be the order of a subgroup?", "multiple_choice", "5", Some("2"), Some("3"), Some("6"), None, "By Lagrange's theorem, subgroup order must divide group order. 5 does not divide 12.", "medium"),
        (rings_id, "A ring requires multiplication to be:", "multiple_choice", "Associative and distributive over addition", Some("Commutative"), Some("Invertible for all elements"), Some("Idempotent"), None, "Ring multiplication must be associative and distribute over addition. Commutativity and inverses are not required.", "medium"),
        (rings_id, "In ℤ₆, what are the zero divisors?", "fill_in_blank", "2, 3, 4", None, None, None, None, "2·3=0, 3·4=0 in ℤ₆. Elements 2, 3, and 4 are zero divisors. 1 and 5 are units.", "hard"),
        (fields_id, "True or false: ℤ₄ is a field.", "true_false", "false", None, None, None, None, "ℤ₄ is not a field because 2 has no multiplicative inverse (2·x ≡ 1 mod 4 has no solution), and 2·2 = 0 makes 2 a zero divisor.", "medium"),
        (fields_id, "What is the degree [ℂ:ℝ]?", "fill_in_blank", "2", None, None, None, None, "ℂ = ℝ(i), and {1, i} is a basis for ℂ over ℝ, so the degree is 2.", "medium"),
        (homo_id, "The kernel of a group homomorphism is always a:", "fill_in_blank", "normal subgroup", None, None, None, None, "The kernel ker(φ) is always a normal subgroup of the domain group G.", "medium"),
        (homo_id, "State the First Isomorphism Theorem conclusion: G/ker(φ) ≅ ___", "cloze", "im(φ)", None, None, None, None, "The First Isomorphism Theorem: the quotient of G by the kernel is isomorphic to the image of φ.", "hard"),
    ];
    for (tid, q, qt, ca, a, b, c, d, expl, diff) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            rusqlite::params![tid, q, qt, ca, a, b, c, d, expl, diff],
        )?;
    }

    // Categorize quiz
    conn.execute(
        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation, difficulty) VALUES (?1,?2,?3,?4,?5,?6,?7)",
        rusqlite::params![
            homo_id,
            "Categorize each map: φ(x)=2x from ℤ→ℤ, det from GL_n→ℝ*, f(x)=x² from ℝ→ℝ, exp from (ℝ,+)→(ℝ⁺,·)",
            "categorize",
            "Homomorphism:φ(x)=2x from ℤ→ℤ,det from GL_n→ℝ*,exp from (ℝ,+)→(ℝ⁺,·);Not homomorphism:f(x)=x² from ℝ→ℝ",
            "Check if each preserves the group operation",
            "φ(x)=2x preserves addition, det preserves multiplication, exp(a+b)=exp(a)·exp(b), but (a+b)²≠a²+b² in general.",
            "hard"
        ],
    )?;

    // Learning path
    let path_topics = [
        (groups_id, "Master group axioms, subgroups, and cyclic groups"),
        (homo_id, "Understand structure-preserving maps and isomorphism theorems"),
        (rings_id, "Learn ring axioms, ideals, and quotient rings"),
        (fields_id, "Explore fields, extensions, and degree theory"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('abstract algebra journey', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_molecular_biology(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute("INSERT INTO subjects (name, description) VALUES ('Molecular Biology', 'The study of biological molecules — DNA, RNA, proteins, and the machinery of life at the molecular level.')", [])?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Molecular Biology'", [], |r| r.get(0))?;

    let topics = [
        ("DNA Structure & Replication", "beginner", 1),
        ("Transcription", "beginner", 2),
        ("Translation & Protein Synthesis", "intermediate", 3),
        ("Gene Regulation", "intermediate", 4),
        ("Mutations & Repair", "intermediate", 5),
        ("Epigenetics", "advanced", 6),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let dna_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'DNA Structure & Replication' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let transcription_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Transcription' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let translation_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Translation & Protein Synthesis' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let regulation_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Gene Regulation' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let mutations_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Mutations & Repair' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let epigenetics_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Epigenetics' AND subject_id = ?1", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (dna_id, "The Double Helix", "DNA is a double-stranded helix made of nucleotides. Each nucleotide has a sugar (deoxyribose), phosphate group, and nitrogenous base (A, T, G, C). Adenine pairs with thymine (2 hydrogen bonds), guanine pairs with cytosine (3 hydrogen bonds). The strands run antiparallel (5'→3' and 3'→5'). Replication is semiconservative — each new double helix has one old strand and one new strand.", 1),
        (transcription_id, "From DNA to mRNA", "Transcription copies a gene's DNA sequence into messenger RNA. RNA polymerase binds to the promoter, unwinds DNA, and synthesizes mRNA in the 5'→3' direction using the template strand. In eukaryotes, the pre-mRNA is processed: a 5' cap and 3' poly-A tail are added, and introns are spliced out by spliceosomes, leaving only exons.", 1),
        (translation_id, "Ribosomes & the Genetic Code", "Translation converts mRNA into protein at ribosomes. Transfer RNAs (tRNAs) carry amino acids and match codons via anticodons. The ribosome has A (aminoacyl), P (peptidyl), and E (exit) sites. Translation starts at AUG (methionine) and ends at a stop codon (UAA, UAG, UGA). The genetic code is degenerate — multiple codons can code for the same amino acid.", 1),
        (regulation_id, "Controlling Gene Expression", "Gene regulation determines when and how much protein is produced. Prokaryotes use operons (e.g., lac operon: repressor blocks transcription unless lactose is present). Eukaryotes regulate at multiple levels: chromatin remodeling, transcription factors binding enhancers/silencers, mRNA processing, mRNA stability, and post-translational modification.", 1),
        (mutations_id, "Types of Mutations", "Mutations are changes in DNA sequence. Point mutations include silent (no amino acid change), missense (different amino acid), and nonsense (premature stop codon). Frameshift mutations (insertions/deletions not divisible by 3) alter the reading frame. DNA repair mechanisms include proofreading by DNA polymerase, mismatch repair, and nucleotide excision repair.", 1),
        (epigenetics_id, "Beyond the DNA Sequence", "Epigenetics studies heritable changes in gene expression without altering DNA sequence. Key mechanisms: DNA methylation (adding methyl groups to cytosine, usually silencing genes), histone modification (acetylation opens chromatin, methylation can activate or silence), and non-coding RNAs (like microRNAs that degrade mRNA). Environmental factors can alter epigenetic marks.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute("INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![tid, title, content, order])?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (dna_id, "Base Pairing", "A always pairs with T (2 H-bonds), G always pairs with C (3 H-bonds). This complementarity enables accurate replication.", Some("Like a zipper where each tooth only fits one specific partner"), Some("Why does G-C pairing make DNA more thermally stable?")),
        (transcription_id, "mRNA Processing", "Eukaryotic pre-mRNA undergoes capping, polyadenylation, and splicing before export from the nucleus.", Some("Like editing a rough draft: add a cover (cap), a signature (poly-A), and cut out the irrelevant paragraphs (introns)"), Some("What is alternative splicing and why does it matter?")),
        (translation_id, "The Genetic Code", "64 codons map to 20 amino acids plus stop signals. The code is universal (shared across almost all life) and degenerate (multiple codons per amino acid).", Some("A dictionary where several different words can mean the same thing"), Some("Why is the genetic code called 'degenerate' rather than 'redundant'?")),
        (regulation_id, "Lac Operon", "A classic prokaryotic regulatory system. When lactose is absent, a repressor protein blocks transcription. When lactose is present, it binds the repressor, changing its shape so it releases from DNA, allowing transcription.", Some("A security guard (repressor) blocks the door until someone with the right badge (lactose) shows up"), Some("What role does cAMP play in lac operon regulation?")),
        (mutations_id, "Frameshift Mutations", "Insertions or deletions of nucleotides not divisible by 3 shift the entire reading frame downstream, usually producing a nonfunctional protein.", Some("Removing one letter from a sentence: THE BIG CAT ATE → THB IGC ATA TE — everything after changes"), Some("Why are frameshifts typically more damaging than point mutations?")),
        (epigenetics_id, "DNA Methylation", "Adding a methyl group (-CH₃) to cytosine in CpG islands silences gene expression. This is a key mechanism in X-inactivation and genomic imprinting.", Some("Putting a padlock on a book — the text inside hasn't changed, but nobody can read it"), Some("Can epigenetic changes be reversed? How?")),
    ];
    for (tid, concept, explanation, analogy, followup) in &explanations {
        conn.execute("INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)", rusqlite::params![tid, concept, explanation, analogy, followup])?;
    }

    // Quiz questions
    let quizzes: Vec<QuizRowHint> = vec![
        (dna_id, "What type of bond holds complementary DNA bases together?", "multiple_choice", "Hydrogen bonds", Some("Covalent bonds"), Some("Ionic bonds"), Some("Peptide bonds"), None, "These are relatively weak bonds that allow strand separation during replication", "Hydrogen bonds hold A-T (2 bonds) and G-C (3 bonds) pairs together. The phosphodiester bonds in the backbone are covalent."),
        (dna_id, "DNA replication is described as semiconservative. What does this mean?", "multiple_choice", "Each new double helix has one old and one new strand", Some("Both strands are entirely new"), Some("The original molecule is preserved intact"), Some("Only the leading strand is conserved"), None, "Think about what 'semi' means", "Meselson and Stahl proved that each daughter DNA molecule retains one parental strand and synthesizes one new complementary strand."),
        (dna_id, "Which base pairs with adenine in DNA?", "fill_in_blank", "Thymine", None, None, None, None, "A pairs with T via 2 hydrogen bonds", "Adenine (a purine) pairs with thymine (a pyrimidine) through 2 hydrogen bonds. In RNA, adenine pairs with uracil instead."),
        (transcription_id, "Which enzyme catalyzes transcription?", "fill_in_blank", "RNA polymerase", None, None, None, None, "This enzyme reads DNA and builds RNA", "RNA polymerase binds to the promoter region, unwinds DNA, and synthesizes mRNA in the 5' to 3' direction using ribonucleoside triphosphates."),
        (transcription_id, "In eukaryotes, which parts of pre-mRNA are removed during splicing?", "multiple_choice", "Introns", Some("Exons"), Some("Promoters"), Some("Codons"), None, "Think: introns are 'in' the way and need to come out", "Introns (intervening sequences) are removed by spliceosomes. Exons (expressed sequences) are joined together to form the mature mRNA."),
        (transcription_id, "True or false: mRNA is synthesized in the 3' to 5' direction.", "true_false", "false", None, None, None, None, "Think about which end gets the cap", "mRNA is synthesized in the 5' to 3' direction. The template DNA strand is read 3' to 5'."),
        (translation_id, "What is the start codon and which amino acid does it code for?", "fill_in_blank", "AUG, methionine", None, None, None, None, "This codon signals the beginning of translation", "AUG codes for methionine and signals the ribosome to begin translation. In prokaryotes, the first amino acid is formyl-methionine."),
        (translation_id, "Which ribosomal site holds the growing polypeptide chain?", "multiple_choice", "P site (peptidyl)", Some("A site (aminoacyl)"), Some("E site (exit)"), Some("S site (start)"), None, "Think: 'P' for peptide chain", "The P site holds the tRNA carrying the growing polypeptide. The A site accepts incoming aminoacyl-tRNAs. The E site is where empty tRNAs exit."),
        (translation_id, "Put these translation steps in order:", "ordering", "mRNA binds ribosome,Start codon recognized,tRNA brings amino acid to A site,Peptide bond forms,Ribosome translocates", None, None, None, None, "Follow the flow from initiation to elongation", "Translation proceeds through initiation (mRNA binds, start codon found), elongation (tRNA delivery, peptide bond formation, translocation), and termination (stop codon reached)."),
        (regulation_id, "In the lac operon, what molecule acts as the inducer?", "multiple_choice", "Allolactose", Some("Glucose"), Some("Tryptophan"), Some("cAMP"), None, "It's derived from the sugar the operon is named after", "Allolactose (an isomer of lactose) binds the lac repressor, causing a conformational change that releases it from the operator, allowing transcription."),
        (regulation_id, "Which of the following are levels of eukaryotic gene regulation?", "select_all", "Chromatin remodeling,Transcription,mRNA processing,Translation", Some("Chromatin remodeling"), Some("Transcription"), Some("mRNA processing"), Some("Translation"), "Eukaryotes regulate at many stages", "Eukaryotic gene expression is regulated at chromatin, transcriptional, post-transcriptional (splicing), translational, and post-translational levels."),
        (mutations_id, "A mutation that changes a codon to a stop codon is called a:", "multiple_choice", "Nonsense mutation", Some("Missense mutation"), Some("Silent mutation"), Some("Frameshift mutation"), None, "This type of mutation makes 'no sense' — it stops translation early", "Nonsense mutations create premature stop codons, truncating the protein. Missense mutations change one amino acid. Silent mutations don't change the protein."),
        (mutations_id, "Which DNA repair mechanism removes thymine dimers caused by UV light?", "multiple_choice", "Nucleotide excision repair", Some("Mismatch repair"), Some("Base excision repair"), Some("Homologous recombination"), None, "This mechanism cuts out a whole section of damaged nucleotides", "Nucleotide excision repair recognizes bulky distortions like thymine dimers, excises a ~30 nucleotide segment, and fills the gap using the complementary strand."),
        (epigenetics_id, "Which histone modification is generally associated with active gene transcription?", "multiple_choice", "Acetylation", Some("Methylation of H3K9"), Some("Deacetylation"), Some("Ubiquitination"), None, "This modification loosens chromatin structure", "Histone acetylation neutralizes positive charges on lysine residues, loosening the histone-DNA interaction and making DNA more accessible to transcription machinery."),
        (epigenetics_id, "DNA methylation typically occurs at which dinucleotide sequence?", "fill_in_blank", "CpG", None, None, None, None, "Think: which two bases are connected by a phosphodiester bond?", "DNA methylation occurs primarily at CpG dinucleotides (cytosine followed by guanine). Clusters of CpGs are called CpG islands and are often found near gene promoters."),
    ];

    for (tid, q, qtype, correct, oa, ob, oc, _od, hint, expl) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![tid, q, qtype, correct, oa, ob, oc, hint, expl],
        )?;
    }

    // Learning path
    let path_topics = [
        (dna_id, "Master DNA structure, base pairing, and semiconservative replication"),
        (transcription_id, "Understand how genes are transcribed into mRNA"),
        (translation_id, "Learn how ribosomes translate mRNA into proteins"),
        (regulation_id, "Explore how cells control gene expression"),
        (mutations_id, "Study mutation types and DNA repair mechanisms"),
        (epigenetics_id, "Discover heritable changes beyond the DNA sequence"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('molecular biology journey', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_set_theory(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute("INSERT INTO subjects (name, description) VALUES ('Set Theory', 'The mathematical study of collections of objects — the foundation of modern mathematics.')", [])?;
    let subj_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Set Theory'", [], |r| r.get(0))?;

    let topics = [
        ("Sets & Notation", "beginner", 1),
        ("Set Operations", "beginner", 2),
        ("Relations & Functions", "intermediate", 3),
        ("Cardinality & Countability", "intermediate", 4),
        ("Axiom of Choice & Zorn's Lemma", "advanced", 5),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![subj_id, name, diff, order],
        )?;
    }

    let notation_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Sets & Notation' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let operations_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Set Operations' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let relations_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Relations & Functions' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let cardinality_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Cardinality & Countability' AND subject_id = ?1", [subj_id], |r| r.get(0))?;
    let axiom_id: i64 = conn.query_row("SELECT id FROM topics WHERE name = 'Axiom of Choice & Zorn''s Lemma' AND subject_id = ?1", [subj_id], |r| r.get(0))?;

    // Lessons
    let lessons: Vec<LessonRow> = vec![
        (notation_id, "What Is a Set?", "A set is a well-defined collection of distinct objects called elements or members. We write a ∈ A to mean 'a is an element of A'. Sets can be described by listing elements {1, 2, 3} or by set-builder notation {x | x > 0}. The empty set ∅ contains no elements. Two sets are equal if and only if they have exactly the same elements (Axiom of Extensionality).", 1),
        (operations_id, "Combining Sets", "Union (A ∪ B): all elements in A or B or both. Intersection (A ∩ B): elements in both A and B. Difference (A \\ B): elements in A but not B. Complement (Aᶜ): elements not in A (relative to a universal set U). Symmetric difference (A △ B): elements in exactly one of A or B. De Morgan's laws: (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ and (A ∩ B)ᶜ = Aᶜ ∪ Bᶜ.", 1),
        (relations_id, "Relations and Functions", "A relation R from A to B is a subset of A × B (the Cartesian product). A function f: A → B is a relation where each element of A maps to exactly one element of B. Injective (one-to-one): different inputs give different outputs. Surjective (onto): every element of B is mapped to. Bijective: both injective and surjective — establishes a one-to-one correspondence.", 1),
        (cardinality_id, "Counting the Infinite", "Two sets have the same cardinality if there exists a bijection between them. Countably infinite sets (|S| = ℵ₀) can be put in bijection with ℕ — examples: ℤ, ℚ. Cantor's diagonal argument proves ℝ is uncountable (|ℝ| = 2^ℵ₀ > ℵ₀). The power set P(A) always has strictly greater cardinality than A (Cantor's theorem).", 1),
        (axiom_id, "The Axiom of Choice", "The Axiom of Choice (AC) states that given any collection of non-empty sets, there exists a function that picks one element from each. Equivalent statements: Zorn's Lemma (every partially ordered set where every chain has an upper bound contains a maximal element) and the Well-Ordering Theorem (every set can be well-ordered). AC is independent of ZF set theory.", 1),
    ];
    for (tid, title, content, order) in &lessons {
        conn.execute("INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![tid, title, content, order])?;
    }

    // Explanations
    let explanations: Vec<ExplanationRow> = vec![
        (notation_id, "Set-Builder Notation", "Set-builder notation defines a set by a property: {x ∈ S | P(x)} reads 'the set of all x in S such that P(x) is true'.", Some("Like a filter: take all items from a collection and keep only those matching your criteria"), Some("Why can't we define the 'set of all sets'?")),
        (operations_id, "De Morgan's Laws", "The complement of a union is the intersection of complements: (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ. The complement of an intersection is the union of complements: (A ∩ B)ᶜ = Aᶜ ∪ Bᶜ.", Some("If you're NOT (tall OR strong), you must be (NOT tall AND NOT strong)"), Some("How do De Morgan's laws extend to arbitrary unions and intersections?")),
        (cardinality_id, "Cantor's Diagonal Argument", "Proves ℝ is uncountable. Assume you could list all real numbers in [0,1]. Construct a new number by changing the nth digit of the nth number. This new number differs from every listed number, contradicting the assumption.", Some("Like trying to make a guest list for a party where the guests keep creating new identities"), Some("Can you apply the diagonal argument to prove the power set of ℕ is uncountable?")),
    ];
    for (tid, concept, explanation, analogy, followup) in &explanations {
        conn.execute("INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)", rusqlite::params![tid, concept, explanation, analogy, followup])?;
    }

    // Quiz questions
    let quizzes: Vec<QuizRowHint> = vec![
        (notation_id, "What symbol denotes the empty set?", "fill_in_blank", "∅", None, None, None, None, "It looks like a zero with a line through it", "The empty set ∅ (or {}) is the unique set containing no elements. It is a subset of every set."),
        (notation_id, "True or false: {1, 2, 3} = {3, 1, 2}", "true_false", "true", None, None, None, None, "Sets are defined by their elements, not order", "Sets are unordered collections. Two sets are equal if they contain exactly the same elements, regardless of order or repetition."),
        (notation_id, "If A = {1, 2, 3, 4, 5}, how many elements does the power set P(A) have?", "fill_in_blank", "32", None, None, None, None, "The power set has 2^n elements", "The power set P(A) is the set of all subsets of A. If |A| = n, then |P(A)| = 2^n. Here 2^5 = 32."),
        (operations_id, "If A = {1,2,3} and B = {2,3,4}, what is A ∩ B?", "fill_in_blank", "{2,3}", None, None, None, None, "Intersection means elements in BOTH sets", "A ∩ B = {x | x ∈ A and x ∈ B} = {2, 3}. These are the elements common to both sets."),
        (operations_id, "By De Morgan's law, (A ∪ B)ᶜ equals:", "multiple_choice", "Aᶜ ∩ Bᶜ", Some("Aᶜ ∪ Bᶜ"), Some("A ∩ B"), Some("(A ∩ B)ᶜ"), None, "The complement of a union becomes an intersection of complements", "De Morgan's first law: (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ. Not being in A-or-B means not in A AND not in B."),
        (operations_id, "What is the symmetric difference A △ B?", "multiple_choice", "Elements in exactly one of A or B", Some("Elements in both A and B"), Some("All elements of A"), Some("Elements in neither A nor B"), None, "Symmetric difference is like XOR", "A △ B = (A \\ B) ∪ (B \\ A) = (A ∪ B) \\ (A ∩ B). It contains elements that are in one set but not both."),
        (relations_id, "A function that is both injective and surjective is called:", "fill_in_blank", "bijective", None, None, None, None, "It establishes a perfect one-to-one correspondence", "A bijection (bijective function) maps each element of the domain to a unique element of the codomain, with every codomain element covered. It has an inverse function."),
        (relations_id, "True or false: every surjective function is also injective.", "true_false", "false", None, None, None, None, "Can two inputs map to the same output while still covering all outputs?", "A surjection maps onto every element of the codomain but may send multiple inputs to the same output. Example: f(x) = x² from ℝ to [0,∞) is surjective but not injective."),
        (cardinality_id, "Which of these sets is uncountable?", "multiple_choice", "The real numbers ℝ", Some("The integers ℤ"), Some("The rationals ℚ"), Some("The natural numbers ℕ"), None, "Cantor proved this with his famous diagonal argument", "ℝ is uncountable — there is no bijection between ℕ and ℝ. Cantor's diagonal argument shows that any attempted listing of real numbers must miss at least one."),
        (cardinality_id, "Cantor's theorem states that for any set A:", "multiple_choice", "|P(A)| > |A|", Some("|P(A)| = |A|"), Some("|P(A)| < |A|"), Some("|P(A)| = 2|A|"), None, "The power set is always strictly larger", "Cantor's theorem proves that the power set (set of all subsets) always has strictly greater cardinality than the original set, even for infinite sets."),
        (axiom_id, "Which statement is equivalent to the Axiom of Choice?", "multiple_choice", "Zorn's Lemma", Some("Cantor's theorem"), Some("De Morgan's laws"), Some("The Pigeonhole Principle"), None, "It involves maximal elements in partially ordered sets", "Zorn's Lemma, the Well-Ordering Theorem, and the Axiom of Choice are all logically equivalent in ZF set theory. Each can be derived from any of the others."),
        (axiom_id, "True or false: the Axiom of Choice can be proved from the other ZF axioms.", "true_false", "false", None, None, None, None, "Gödel and Cohen showed something important about its independence", "The Axiom of Choice is independent of ZF: Gödel showed ZFC is consistent (if ZF is), and Cohen showed ZF + ¬AC is also consistent. It can be neither proved nor disproved from ZF alone."),
    ];

    for (tid, q, qtype, correct, oa, ob, oc, _od, hint, expl) in &quizzes {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![tid, q, qtype, correct, oa, ob, oc, hint, expl],
        )?;
    }

    // Analogy quiz questions (new quiz type)
    let analogies: Vec<(i64, &str, &str, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
        (notation_id, "∈ is to 'element of' as ⊆ is to ___", "subset of", Some("superset of"), Some("equal to"), Some("complement of"), "Both symbols describe a relationship between an object and a set", "∈ means membership (element in set), ⊆ means subset (set contained in set). Both describe containment relationships at different levels."),
        (operations_id, "Union is to OR as Intersection is to ___", "AND", Some("NOT"), Some("XOR"), Some("NOR"), "Think about logical operations", "Union (∪) corresponds to logical OR (in either), intersection (∩) corresponds to logical AND (in both). This connection is formalized in Boolean algebra."),
    ];
    for (tid, q, correct, oa, ob, oc, hint, expl) in &analogies {
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, hint, explanation) VALUES (?1, ?2, 'analogy', ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![tid, q, correct, oa, ob, oc, hint, expl],
        )?;
    }

    // Learning path
    let path_topics = [
        (notation_id, "Learn set notation, membership, and the empty set"),
        (operations_id, "Master union, intersection, complement, and De Morgan's laws"),
        (relations_id, "Understand relations, functions, and bijectivity"),
        (cardinality_id, "Explore countable vs uncountable infinity"),
        (axiom_id, "Study the Axiom of Choice and its equivalents"),
    ];
    for (i, (tid, desc)) in path_topics.iter().enumerate() {
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('set theory journey', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_analogy_questions(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Add analogy questions across existing subjects for variety
    let analogies: Vec<(&str, &str, &str, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
        ("Arithmetic", "Addition is to subtraction as multiplication is to ___", "division", Some("addition"), Some("exponentiation"), Some("logarithm"), "Think inverse operations", "Division is the inverse of multiplication, just as subtraction is the inverse of addition. Each pair undoes the other."),
        ("Photosynthesis", "Chloroplast is to photosynthesis as mitochondria is to ___", "cellular respiration", Some("photosynthesis"), Some("fermentation"), Some("osmosis"), "Where does each process occur?", "Chloroplasts perform photosynthesis (light → chemical energy). Mitochondria perform cellular respiration (chemical energy → ATP). Each organelle is the site of its signature process."),
        ("Sentence Structure", "Noun is to person/place/thing as verb is to ___", "action", Some("description"), Some("modifier"), Some("conjunction"), "What does each part of speech represent?", "Nouns name entities (person, place, thing). Verbs express actions or states of being. Both are fundamental parts of speech with complementary roles."),
        ("Ancient Civilizations", "Athens is to democracy as Rome is to ___", "republic", Some("monarchy"), Some("oligarchy"), Some("theocracy"), "What form of government was each city famous for?", "Athens pioneered direct democracy. Rome developed the republic (representative government). Both were foundational political systems of the ancient world."),
        ("Search Algorithms", "Linear search is to O(n) as binary search is to ___", "O(log n)", Some("O(n)"), Some("O(n²)"), Some("O(1)"), "Binary search halves the search space each step", "Linear search checks every element: O(n). Binary search halves the sorted array each step: O(log n). The analogy maps algorithm to its time complexity."),
    ];

    for (topic_name, q, correct, oa, ob, oc, hint, expl) in &analogies {
        let topic_id: Option<i64> = conn.query_row(
            "SELECT id FROM topics WHERE name = ?1",
            [topic_name],
            |r| r.get(0),
        ).ok();
        if let Some(tid) = topic_id {
            conn.execute(
                "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, hint, explanation) VALUES (?1, ?2, 'analogy', ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![tid, q, correct, oa, ob, oc, hint, expl],
            )?;
        }
    }
    Ok(())
}

fn seed_paleontology(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Paleontology'", [], |r| r.get(0)
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Paleontology', 'The study of ancient life through fossils — dinosaurs, mass extinctions, and the history of life on Earth.')",
        [],
    )?;
    let sub_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Paleontology'", [], |r| r.get(0)
    )?;

    let topics = [
        ("Fossils & Fossilization", "beginner", 1),
        ("Dinosaurs", "beginner", 2),
        ("Mass Extinctions", "intermediate", 3),
        ("Geologic Time Scale", "beginner", 4),
        ("Early Life & Cambrian Explosion", "intermediate", 5),
        ("Human Evolution", "advanced", 6),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sub_id, name, diff, order],
        )?;
    }

    let lessons: Vec<LessonRow> = vec![
        (1, "What Are Fossils?", "Fossils are preserved remains or traces of organisms from the past. They form through mineralization, where minerals replace organic material over millions of years. Types include body fossils (bones, shells), trace fossils (footprints, burrows), and chemical fossils (organic molecules).", 1),
        (2, "The Age of Dinosaurs", "Dinosaurs dominated Earth for over 160 million years during the Mesozoic Era (252-66 MYA). They were divided into two groups: Saurischia (lizard-hipped, including theropods and sauropods) and Ornithischia (bird-hipped, including triceratops and stegosaurus). Birds are living dinosaurs, descended from small theropod dinosaurs.", 1),
        (3, "The Big Five Extinctions", "Earth has experienced five major mass extinctions. The most famous is the K-Pg extinction (66 MYA) that wiped out non-avian dinosaurs. The largest was the Permian-Triassic extinction (252 MYA), eliminating ~96% of marine species. Each extinction reshaped life's trajectory.", 1),
        (4, "Reading the Rock Record", "The geologic time scale divides Earth's 4.6-billion-year history into eons, eras, periods, and epochs. Key boundaries often correspond to mass extinctions. Relative dating uses stratigraphy (older layers below), while absolute dating uses radioactive isotopes.", 1),
        (5, "Life's Explosive Beginning", "The Cambrian Explosion (~541 MYA) saw the rapid appearance of most major animal phyla in ~20 million years. Key fossils from the Burgess Shale (Canada) and Chengjiang (China) reveal strange creatures like Anomalocaris and Hallucigenia.", 1),
        (6, "From Ape to Human", "Human evolution spans ~7 million years. Key milestones: bipedalism (Australopithecus, ~4 MYA), tool use (Homo habilis, ~2.5 MYA), fire control (Homo erectus, ~1 MYA), and symbolic thought (Homo sapiens, ~300,000 years ago). We share a common ancestor with chimpanzees.", 1),
    ];
    for (idx, title, content, order) in &lessons {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (1, "Fossilization Process", "When an organism dies and is quickly buried, minerals in groundwater gradually replace the original material, turning it to stone.", Some("Like a 3D printer slowly replacing plastic with metal, atom by atom"), Some("Why are soft-bodied organisms rarely fossilized?")),
        (2, "Theropod Dinosaurs", "Theropods were bipedal, mostly carnivorous dinosaurs including T. rex, Velociraptor, and the ancestors of modern birds.", Some("Think of birds as tiny surviving dinosaurs wearing feather coats"), Some("What evidence links birds to theropod dinosaurs?")),
        (3, "K-Pg Extinction", "An asteroid impact 66 million years ago triggered global wildfires, a nuclear winter effect, and acid rain, killing ~75% of all species.", Some("Imagine turning off the sun for months"), Some("What survived the K-Pg extinction and why?")),
    ];
    for (idx, concept, explanation, analogy, follow_up) in &explanations {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRowHint> = vec![
        (1, "What is the most common type of fossilization?", "multiple_choice", "Mineralization", Some("Carbonization"), Some("Freezing"), Some("Amber preservation"), None, "Think about what replaces bone over millions of years", "Mineralization (permineralization) is the most common process, where minerals fill pores and replace original material."),
        (1, "Trace fossils include:", "multiple_choice", "Footprints and burrows", Some("Bones and teeth"), Some("Shells and exoskeletons"), Some("Amber insects"), None, "These show behavior, not body parts", "Trace fossils preserve evidence of activity (tracks, burrows, coprolites) rather than the organism itself."),
        (2, "Birds are descendants of which dinosaur group?", "multiple_choice", "Theropods", Some("Sauropods"), Some("Ornithischians"), Some("Pterosaurs"), None, "Think about bipedal carnivores", "Birds evolved from small feathered theropod dinosaurs. Key evidence includes the fossil Archaeopteryx."),
        (2, "The Mesozoic Era is also known as the:", "multiple_choice", "Age of Reptiles", Some("Age of Mammals"), Some("Age of Fish"), Some("Age of Insects"), None, "Dinosaurs were the dominant land animals", "The Mesozoic (252-66 MYA) is called the Age of Reptiles because dinosaurs dominated all major ecosystems."),
        (3, "Which mass extinction was the largest?", "multiple_choice", "Permian-Triassic", Some("K-Pg (Cretaceous-Paleogene)"), Some("Ordovician-Silurian"), Some("Triassic-Jurassic"), None, "It happened about 252 million years ago", "The Permian-Triassic extinction killed ~96% of marine species — the Great Dying."),
        (3, "What caused the K-Pg mass extinction?", "multiple_choice", "Asteroid impact", Some("Volcanic eruption only"), Some("Ice age"), Some("Sea level drop"), None, "A large space rock hit the Yucatan Peninsula", "A ~10 km asteroid struck Chicxulub, Mexico, causing global devastation."),
        (4, "The largest division of geologic time is:", "multiple_choice", "Eon", Some("Era"), Some("Period"), Some("Epoch"), None, "Think of the hierarchy from biggest to smallest", "Eons are the largest: Hadean, Archean, Proterozoic, Phanerozoic."),
        (5, "The Cambrian Explosion occurred approximately ___ million years ago.", "fill_in_blank", "541", None, None, None, None, "It marks the start of the Phanerozoic eon", "The Cambrian Explosion began ~541 MYA with rapid diversification of complex animal life."),
        (6, "Homo sapiens first appeared approximately ___ thousand years ago.", "fill_in_blank", "300", None, None, None, None, "Think hundreds of thousands, not millions", "Anatomically modern Homo sapiens appeared ~300,000 years ago in Africa."),
        (6, "True or false: Humans evolved directly from chimpanzees.", "true_false", "false", None, None, None, None, "Think about common ancestors", "Humans and chimpanzees share a common ancestor from ~7 million years ago. We are cousins, not descendants."),
    ];
    for (idx, q, qtype, correct, oa, ob, oc, _od, hint, expl) in &quizzes {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            rusqlite::params![tid, q, qtype, correct, oa, ob, oc, hint, expl],
        )?;
    }

    let path_steps = [
        (1, "Learn how fossils form and the types of fossils"),
        (4, "Understand the geologic time scale"),
        (5, "Explore the Cambrian Explosion and early life"),
        (2, "Study dinosaurs and the Mesozoic Era"),
        (3, "Understand mass extinctions and their causes"),
        (6, "Explore human evolution"),
    ];
    for (i, (sort, desc)) in path_steps.iter().enumerate() {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('paleontology journey', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_marine_biology(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Marine Biology'", [], |r| r.get(0)
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Marine Biology', 'The study of ocean life — from microscopic plankton to blue whales, coral reefs to deep-sea vents.')",
        [],
    )?;
    let sub_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Marine Biology'", [], |r| r.get(0)
    )?;

    let topics = [
        ("Ocean Zones", "beginner", 1),
        ("Coral Reefs", "beginner", 2),
        ("Marine Mammals", "beginner", 3),
        ("Deep Sea Ecosystems", "intermediate", 4),
        ("Plankton & Marine Food Webs", "intermediate", 5),
        ("Ocean Conservation", "intermediate", 6),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sub_id, name, diff, order],
        )?;
    }

    let lessons: Vec<LessonRow> = vec![
        (1, "Layers of the Ocean", "The ocean is divided into zones by depth: Epipelagic (sunlight, 0-200m), Mesopelagic (twilight, 200-1000m), Bathypelagic (midnight, 1000-4000m), Abyssopelagic (abyss, 4000-6000m), and Hadopelagic (trenches, 6000m+).", 1),
        (2, "Coral Reef Ecosystems", "Coral reefs are built by tiny polyps that secrete calcium carbonate skeletons. They host ~25% of all marine species despite covering <1% of the ocean floor. Coral depends on symbiotic algae (zooxanthellae) for nutrition.", 1),
        (3, "Whales, Dolphins & Seals", "Marine mammals breathe air but are adapted for ocean life. Blue whales are the largest animals ever (up to 30m). Echolocation allows toothed whales to navigate and hunt in dark waters.", 1),
        (4, "Life in the Abyss", "Hydrothermal vents support entire ecosystems through chemosynthesis. Giant tube worms (Riftia) can grow 2m tall near vents. The deep sea contains more biomass than all tropical rainforests combined.", 1),
        (5, "The Ocean's Invisible Forest", "Phytoplankton produce ~50% of Earth's oxygen. They form the base of marine food webs: phytoplankton to zooplankton to small fish to larger predators.", 1),
        (6, "Protecting Our Oceans", "Major threats: overfishing, plastic pollution (~8M tons/year), ocean acidification, and habitat destruction. Marine Protected Areas cover ~8% of the ocean.", 1),
    ];
    for (idx, title, content, order) in &lessons {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (1, "Photic Zone", "The sunlight zone extends to ~200m depth where photosynthesis occurs, supporting most ocean life.", Some("Like the ground floor of a skyscraper — all the life is here"), Some("Why can't photosynthesis occur below 200m?")),
        (2, "Coral Bleaching", "When water temperatures rise, corals expel their symbiotic algae, turning white and losing their food source.", Some("Like evicting your cook — you'll starve without them"), Some("Can bleached coral recover?")),
        (4, "Chemosynthesis", "Unlike photosynthesis, chemosynthesis uses chemical reactions from vents to produce food. This proved life doesn't require sunlight.", Some("Instead of solar panels, these bacteria run on chemical batteries"), Some("Could chemosynthesis support life on other planets?")),
    ];
    for (idx, concept, explanation, analogy, follow_up) in &explanations {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRowHint> = vec![
        (1, "The sunlight zone extends to approximately:", "multiple_choice", "200 meters", Some("500 meters"), Some("1000 meters"), Some("50 meters"), None, "This is where photosynthesis can occur", "The epipelagic zone extends to ~200m, where enough light penetrates for photosynthesis."),
        (1, "The deepest ocean zone is the:", "multiple_choice", "Hadopelagic", Some("Abyssopelagic"), Some("Bathypelagic"), Some("Mesopelagic"), None, "Named after Hades, god of the underworld", "The hadopelagic zone (6000m+) includes the Mariana Trench (nearly 11,000m deep)."),
        (2, "Coral reefs host approximately what percentage of marine species?", "multiple_choice", "25%", Some("5%"), Some("50%"), Some("75%"), None, "Think about one quarter", "Coral reefs support ~25% of marine biodiversity despite covering less than 1% of the ocean floor."),
        (2, "Coral bleaching is caused by:", "multiple_choice", "Rising water temperatures", Some("Excessive sunlight"), Some("Low salinity"), Some("Overfishing"), None, "Think about climate change", "Elevated temperatures cause corals to expel their symbiotic zooxanthellae algae."),
        (3, "Blue whales can grow up to ___ meters long.", "fill_in_blank", "30", None, None, None, None, "About the length of three school buses", "Blue whales reach ~30m, making them the largest animals ever to have lived."),
        (3, "True or false: All marine mammals are born underwater.", "true_false", "false", None, None, None, None, "Think about seals and sea lions", "Pinnipeds give birth on land. Only cetaceans and sirenians give birth in water."),
        (4, "Hydrothermal vent ecosystems are powered by:", "multiple_choice", "Chemosynthesis", Some("Photosynthesis"), Some("Geothermal heat alone"), Some("Organic debris"), None, "Bacteria use chemical energy, not light", "Chemosynthetic bacteria convert hydrogen sulfide into energy at vent food webs."),
        (5, "Phytoplankton produce approximately what percentage of Earth's oxygen?", "multiple_choice", "50%", Some("10%"), Some("25%"), Some("75%"), None, "About half!", "Phytoplankton produce roughly 50% of Earth's oxygen through photosynthesis."),
        (5, "The correct marine food chain order is:", "multiple_choice", "Phytoplankton, Zooplankton, Small fish, Large predators", Some("Zooplankton, Phytoplankton, Small fish, Large predators"), Some("Small fish, Zooplankton, Phytoplankton, Large predators"), Some("Phytoplankton, Small fish, Zooplankton, Large predators"), None, "Start with the producers", "Energy flows from producers (phytoplankton) through primary consumers to tertiary consumers."),
        (6, "Approximately how many tons of plastic enter the ocean each year?", "multiple_choice", "8 million", Some("1 million"), Some("100 million"), Some("500 thousand"), None, "Think of millions", "An estimated 8 million metric tons of plastic enter the ocean annually."),
    ];
    for (idx, q, qtype, correct, oa, ob, oc, _od, hint, expl) in &quizzes {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, hint, explanation) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            rusqlite::params![tid, q, qtype, correct, oa, ob, oc, hint, expl],
        )?;
    }

    let path_steps = [
        (1, "Understand ocean zones and how depth affects life"),
        (5, "Learn about plankton and marine food webs"),
        (2, "Explore coral reef ecosystems"),
        (3, "Study marine mammals and their adaptations"),
        (4, "Discover deep sea ecosystems and chemosynthesis"),
        (6, "Understand ocean conservation challenges"),
    ];
    for (i, (sort, desc)) in path_steps.iter().enumerate() {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('ocean explorer', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_astrophysics(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Astrophysics'", [], |r| r.get(0)
    )?;
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Astrophysics', 'The physics of stars, galaxies, black holes, and the universe — from stellar nucleosynthesis to cosmic expansion.')",
        [],
    )?;
    let sub_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Astrophysics'", [], |r| r.get(0))?;

    let topics = [
        (1, "Stellar Evolution", "beginner"),
        (2, "Black Holes", "intermediate"),
        (3, "Cosmology & Big Bang", "intermediate"),
        (4, "Neutron Stars & Pulsars", "advanced"),
        (5, "Dark Matter & Dark Energy", "advanced"),
        (6, "Exoplanets & Habitability", "beginner"),
    ];
    for (sort, name, diff) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sub_id, name, diff, sort],
        )?;
    }

    let lessons: Vec<LessonRow> = vec![
        (1, "Life Cycle of a Star", "Stars form from collapsing gas clouds (nebulae). Gravity pulls hydrogen together until nuclear fusion ignites in the core. A star spends most of its life on the main sequence, fusing hydrogen into helium. When hydrogen runs out, the star expands into a red giant. Massive stars end in supernovae; smaller ones shed their outer layers as planetary nebulae, leaving white dwarfs behind.", 1),
        (1, "The Hertzsprung-Russell Diagram", "The H-R diagram plots stars by luminosity (brightness) vs temperature (color). Main sequence stars form a diagonal band. Red giants sit upper-right (cool but bright), white dwarfs lower-left (hot but dim). A star's position reveals its mass, age, and evolutionary stage.", 2),
        (2, "What Is a Black Hole?", "A black hole forms when a massive star collapses so completely that nothing — not even light — can escape its gravitational pull. The boundary beyond which escape is impossible is called the event horizon. The singularity at the center is where density becomes theoretically infinite.", 1),
        (2, "Types of Black Holes", "Stellar black holes (5-100 solar masses) form from supernovae. Supermassive black holes (millions to billions of solar masses) sit at galaxy centers — Sagittarius A* in our Milky Way is ~4 million solar masses. Intermediate-mass black holes are rarer and still being studied.", 2),
        (3, "The Big Bang Theory", "The universe began ~13.8 billion years ago from an extremely hot, dense state. Evidence: cosmic microwave background radiation (CMB), Hubble's observation of galaxy redshift (expansion), and the abundance of light elements (hydrogen, helium) matching nucleosynthesis predictions.", 1),
        (3, "Expansion of the Universe", "Edwin Hubble discovered that galaxies are moving away from us, and farther galaxies recede faster (Hubble's Law: v = H₀d). This means the universe is expanding. In 1998, observations of Type Ia supernovae showed the expansion is accelerating, driven by dark energy.", 2),
        (4, "Neutron Star Basics", "When a star 8-25× the Sun's mass explodes as a supernova, the core collapses into a neutron star — an incredibly dense remnant just 20 km across but with 1.4-2 solar masses. A teaspoon of neutron star material weighs ~6 billion tonnes.", 1),
        (4, "Pulsars and Magnetars", "Pulsars are rapidly spinning neutron stars that emit beams of radiation from their magnetic poles. As they rotate, the beams sweep past Earth like a lighthouse. Magnetars are neutron stars with extreme magnetic fields (10¹⁵ gauss) — a trillion times stronger than Earth's.", 2),
        (5, "The Dark Matter Mystery", "Galaxies rotate too fast for the visible matter they contain — something unseen provides extra gravity. This 'dark matter' makes up ~27% of the universe's mass-energy. It doesn't emit, absorb, or reflect light. Leading candidates: WIMPs (weakly interacting massive particles) and axions.", 1),
        (5, "Dark Energy and Cosmic Acceleration", "Dark energy comprises ~68% of the universe and drives its accelerating expansion. Its nature is unknown. The cosmological constant (Λ) treats it as a property of space itself. Alternatively, quintessence models propose a dynamic energy field.", 2),
        (6, "Detecting Exoplanets", "Exoplanets are found using the transit method (dimming as a planet crosses its star), radial velocity (star wobble from gravitational tug), and direct imaging. NASA's Kepler mission discovered thousands, revealing that planets are common — most stars have at least one.", 1),
        (6, "The Habitable Zone", "The habitable zone (or Goldilocks zone) is the orbital region where liquid water could exist on a planet's surface. Too close → water evaporates; too far → water freezes. Factors include star luminosity, planet atmosphere, and greenhouse effects.", 2),
    ];
    for (sort_topic, title, content, sort_order) in &lessons {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort_topic], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, sort_order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (1, "Nuclear Fusion", "Nuclear fusion is the process where lighter atomic nuclei combine to form heavier ones, releasing enormous energy. In stars, hydrogen fuses into helium in the core at temperatures exceeding 15 million Kelvin.", Some("Imagine pressing two magnets together — it takes enormous force, but once they snap together, energy is released."), Some("Why does fusion require such extreme temperatures?")),
        (2, "Event Horizon", "The event horizon is the boundary around a black hole beyond which nothing can escape. It's not a physical surface — it's a mathematical boundary where escape velocity equals the speed of light.", Some("Think of it like a waterfall's point of no return — once you pass it, the current is too strong to swim back."), Some("Can information escape a black hole?")),
        (3, "Cosmic Microwave Background", "The CMB is the afterglow of the Big Bang — radiation from when the universe cooled enough for atoms to form (~380,000 years after the Big Bang). It fills the entire sky at a temperature of 2.725 K.", Some("It's like the echo of a massive explosion still reverberating through the universe."), Some("What do temperature variations in the CMB tell us?")),
        (5, "Dark Matter", "Dark matter is invisible matter that interacts gravitationally but not electromagnetically. Its existence is inferred from galaxy rotation curves, gravitational lensing, and cosmic structure formation.", Some("Imagine a crowded dance floor where you can see some dancers being pushed around by invisible partners."), Some("How do we know dark matter isn't just regular matter we can't see?")),
        (6, "Habitable Zone", "The habitable zone is the region around a star where conditions might support liquid water. It depends on the star's luminosity — dimmer stars have closer habitable zones, brighter stars have farther ones.", Some("Like sitting at the right distance from a campfire — close enough to stay warm, far enough not to burn."), Some("Could a planet outside the habitable zone still support life?")),
    ];
    for (sort_topic, concept, explanation, analogy, follow_up) in &explanations {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort_topic], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    // Quiz questions (using QuizRowHint-compatible inserts)
    let quizzes: Vec<QuizRowHint> = vec![
        (1, "What fuels a main-sequence star?", "multiple_choice", "Hydrogen fusion",
         Some("Helium fusion"), Some("Hydrogen fusion"), Some("Carbon combustion"), Some("Gravitational collapse"),
         "Think about the most abundant element in the universe.", "Stars on the main sequence primarily fuse hydrogen into helium in their cores."),
        (1, "A star more massive than ~8 solar masses ends its life as a supernova.", "true_false", "true",
         None, None, None, None, "Consider what happens when fusion can no longer support the core.", "Massive stars undergo core collapse when iron builds up, triggering a supernova."),
        (2, "What is the boundary around a black hole called?", "multiple_choice", "Event horizon",
         Some("Event horizon"), Some("Schwarzschild sphere"), Some("Singularity border"), Some("Photon ring"),
         "It marks the point of no return.", "The event horizon is the boundary beyond which nothing can escape a black hole's gravity."),
        (2, "The _____ at the center of a black hole is where density becomes theoretically infinite.", "fill_in_blank", "singularity",
         None, None, None, None, "It's a point of infinite density.", "The singularity is the theoretical point at the center where gravitational forces crush matter to infinite density."),
        (3, "The cosmic microwave background radiation is evidence of what event?", "multiple_choice", "The Big Bang",
         Some("Star formation"), Some("Galaxy collision"), Some("The Big Bang"), Some("Supernova explosion"),
         "This radiation fills the entire observable universe uniformly.", "The CMB is the thermal afterglow of the Big Bang, released about 380,000 years after the universe began."),
        (3, "Hubble's Law states that farther galaxies move away faster.", "true_false", "true",
         None, None, None, None, "Think about the expanding universe.", "Hubble discovered that galaxy recession velocity is proportional to distance (v = H₀d), proving the universe is expanding."),
        (4, "How wide is a typical neutron star?", "multiple_choice", "About 20 km",
         Some("About 20 km"), Some("About 200 km"), Some("About 2,000 km"), Some("About 20,000 km"),
         "Incredibly dense but surprisingly small.", "Despite containing 1.4-2 solar masses, neutron stars are only about 20 km in diameter."),
        (4, "Magnetars have magnetic fields about _____ times stronger than Earth's.", "fill_in_blank", "a trillion",
         None, None, None, None, "Think of an extraordinarily large number.", "Magnetars possess magnetic fields of ~10¹⁵ gauss, roughly a trillion times Earth's magnetic field."),
        (5, "What percentage of the universe's mass-energy is dark matter?", "multiple_choice", "About 27%",
         Some("About 5%"), Some("About 27%"), Some("About 68%"), Some("About 50%"),
         "Regular matter is only about 5%.", "Dark matter makes up approximately 27% of the universe, while dark energy is about 68% and ordinary matter only ~5%."),
        (5, "Dark energy causes the universe's expansion to decelerate.", "true_false", "false",
         None, None, None, None, "The 1998 Nobel Prize was awarded for discovering the opposite.", "Dark energy drives the accelerating expansion of the universe, discovered via Type Ia supernovae observations."),
        (6, "Which method detects exoplanets by measuring starlight dimming?", "multiple_choice", "Transit method",
         Some("Radial velocity"), Some("Transit method"), Some("Astrometry"), Some("Microlensing"),
         "The planet passes in front of its star.", "The transit method detects the slight dimming when a planet crosses in front of its host star, as seen from Earth."),
        (6, "The habitable zone depends only on the distance from the star.", "true_false", "false",
         None, None, None, None, "Consider what else affects surface temperature.", "The habitable zone also depends on stellar luminosity, planetary atmosphere, and greenhouse effects — not just distance."),
    ];
    for (sort_topic, question, qtype, answer, oa, ob, oc, od, hint, explanation) in &quizzes {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort_topic], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![tid, question, qtype, answer, oa, ob, oc, od, hint, explanation],
        )?;
    }

    // Learning path
    let path_steps: Vec<(i64, &str)> = vec![
        (1, "Learn how stars are born, live, and die"),
        (6, "Discover exoplanets and where life might exist"),
        (3, "Understand the Big Bang and cosmic expansion"),
        (2, "Explore the physics of black holes"),
        (4, "Study extreme objects: neutron stars and pulsars"),
        (5, "Dive into dark matter and dark energy mysteries"),
    ];
    for (i, (sort, desc)) in path_steps.iter().enumerate() {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('astrophysics explorer', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

fn seed_neuroscience(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Neuroscience'", [], |r| r.get(0)
    )?;
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Neuroscience', 'The science of the brain and nervous system — from neurons and synapses to consciousness, memory, and behavior.')",
        [],
    )?;
    let sub_id: i64 = conn.query_row("SELECT id FROM subjects WHERE name = 'Neuroscience'", [], |r| r.get(0))?;

    let topics = [
        (1, "Neurons & Synapses", "beginner"),
        (2, "Brain Anatomy", "beginner"),
        (3, "Memory & Learning", "intermediate"),
        (4, "Neuroplasticity", "intermediate"),
        (5, "Neurotransmitters", "advanced"),
        (6, "Consciousness & Sleep", "advanced"),
    ];
    for (sort, name, diff) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sub_id, name, diff, sort],
        )?;
    }

    let lessons: Vec<LessonRow> = vec![
        (1, "The Neuron", "Neurons are specialized cells that transmit information via electrical and chemical signals. Each neuron has a cell body (soma), dendrites (receive signals), and an axon (sends signals). The human brain contains ~86 billion neurons.", 1),
        (1, "Synaptic Transmission", "Synapses are the gaps between neurons. When an electrical signal (action potential) reaches the axon terminal, neurotransmitters are released into the synaptic cleft. These chemicals bind to receptors on the next neuron, continuing the signal.", 2),
        (2, "Major Brain Regions", "The cerebrum (thinking, language, movement) has four lobes: frontal (planning), parietal (sensation), temporal (hearing, memory), and occipital (vision). The cerebellum coordinates movement. The brainstem controls vital functions like breathing and heart rate.", 1),
        (2, "The Limbic System", "The limbic system handles emotions and memory. Key structures: the amygdala (fear, emotional processing), hippocampus (forming new memories), and hypothalamus (hormones, hunger, body temperature). Damage to the hippocampus can prevent forming new long-term memories.", 2),
        (3, "How Memories Form", "Memories form through a process called encoding → storage → retrieval. Short-term (working) memory holds ~7 items for seconds. Long-term memories are consolidated during sleep through hippocampal replay — neural patterns from the day are replayed and strengthened.", 1),
        (3, "Types of Memory", "Declarative memory includes episodic (personal events) and semantic (facts). Procedural memory covers skills (riding a bike). Declarative memories depend on the hippocampus; procedural memories rely on the basal ganglia and cerebellum.", 2),
        (4, "The Plastic Brain", "Neuroplasticity is the brain's ability to reorganize by forming new neural connections. It enables learning, recovery from injury, and adaptation. London taxi drivers who memorize complex routes develop larger hippocampi — a famous example of experience-dependent plasticity.", 1),
        (4, "Hebbian Learning", "Donald Hebb proposed that 'neurons that fire together wire together.' Repeated activation of connected neurons strengthens their synaptic connections (long-term potentiation, LTP). This is the cellular basis of learning and memory.", 2),
        (5, "Key Neurotransmitters", "Dopamine: reward, motivation. Serotonin: mood, sleep. GABA: inhibition, calming. Glutamate: excitation, learning. Acetylcholine: memory, muscle control. Norepinephrine: alertness, attention. Imbalances in these systems underlie many neurological and psychiatric conditions.", 1),
        (5, "Drugs and Neurotransmitters", "Psychoactive drugs work by altering neurotransmitter systems. SSRIs (antidepressants) block serotonin reuptake. Caffeine blocks adenosine receptors. L-DOPA treats Parkinson's by replenishing dopamine. Understanding these mechanisms is key to pharmacology.", 2),
        (6, "What Is Consciousness?", "Consciousness remains one of neuroscience's greatest mysteries. The 'hard problem' asks why subjective experience exists at all. Leading theories: Global Workspace Theory (information broadcast to many brain areas) and Integrated Information Theory (consciousness = integrated information, Φ).", 1),
        (6, "The Science of Sleep", "Sleep has distinct stages: NREM (N1-N3, progressively deeper) and REM (dreaming, memory consolidation). During deep sleep (N3), slow waves coordinate hippocampal-cortical memory transfer. REM sleep strengthens emotional memories and creative problem-solving.", 2),
    ];
    for (sort_topic, title, content, sort_order) in &lessons {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort_topic], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, sort_order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (1, "Action Potential", "An action potential is a rapid electrical signal that travels along a neuron's axon. It's triggered when the neuron reaches a threshold voltage, causing sodium channels to open and the cell to depolarize.", Some("Like a row of dominoes falling — once the first tips, the chain reaction propagates to the end."), Some("Why can't an action potential travel backwards?")),
        (3, "Long-Term Potentiation", "LTP is the strengthening of synaptic connections through repeated activation. It's the primary cellular mechanism behind learning and memory — when you practice something, the synapses involved become more efficient.", Some("Like a path through a forest — the more you walk it, the clearer and easier it becomes."), Some("What conditions are needed for LTP to occur?")),
        (4, "Neuroplasticity", "The brain's ability to physically change its structure and function in response to experience. It happens through synaptogenesis (new connections), pruning (removing unused ones), and myelination (insulating active pathways).", Some("Your brain is like clay that reshapes itself based on what you do — the more you practice, the deeper the impression."), Some("Does neuroplasticity decrease with age?")),
        (5, "Dopamine", "Dopamine is a neurotransmitter central to reward, motivation, and learning. It signals prediction errors — when something is better or worse than expected. This drives goal-directed behavior and is crucial for habit formation.", Some("Dopamine is like your brain's gold star sticker — it marks experiences worth repeating."), Some("How does dopamine relate to addiction?")),
        (6, "REM Sleep", "Rapid Eye Movement sleep is characterized by vivid dreaming, rapid eye movements, and temporary muscle paralysis. It plays a crucial role in emotional memory processing and creative insight.", Some("REM sleep is like your brain's nightly editing session — reviewing the day's footage and filing the important clips."), Some("Why are we paralyzed during REM sleep?")),
    ];
    for (sort_topic, concept, explanation, analogy, follow_up) in &explanations {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort_topic], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRowHint> = vec![
        (1, "How many neurons does the human brain contain?", "multiple_choice", "About 86 billion",
         Some("About 86 million"), Some("About 86 billion"), Some("About 860 billion"), Some("About 8.6 trillion"),
         "It's billions, not millions or trillions.", "The human brain contains approximately 86 billion neurons."),
        (1, "Dendrites send signals away from the cell body.", "true_false", "false",
         None, None, None, None, "Think about the direction of information flow.", "Dendrites receive signals; axons send signals away from the cell body."),
        (2, "Which brain lobe is primarily responsible for vision?", "multiple_choice", "Occipital lobe",
         Some("Frontal lobe"), Some("Temporal lobe"), Some("Parietal lobe"), Some("Occipital lobe"),
         "It's at the back of the head.", "The occipital lobe at the rear of the brain processes visual information."),
        (2, "The _____ is the brain structure most important for forming new long-term memories.", "fill_in_blank", "hippocampus",
         None, None, None, None, "It's named after a sea creature it resembles.", "The hippocampus is critical for converting short-term memories into long-term ones."),
        (3, "Working memory can hold about _____ items.", "fill_in_blank", "7",
         None, None, None, None, "Miller's magic number.", "George Miller's research showed working memory capacity is approximately 7 (±2) items."),
        (3, "Procedural memories (like riding a bike) depend on the hippocampus.", "true_false", "false",
         None, None, None, None, "Think about what brain area handles motor skills.", "Procedural memories rely on the basal ganglia and cerebellum, not the hippocampus."),
        (4, "What phrase summarizes Hebbian learning?", "multiple_choice", "Neurons that fire together wire together",
         Some("Use it or lose it"), Some("Neurons that fire together wire together"), Some("Practice makes perfect"), Some("The brain never changes"),
         "It's about simultaneous activation.", "Hebb's rule states that when neurons repeatedly fire together, their connections strengthen."),
        (4, "London taxi drivers develop larger hippocampi from memorizing routes.", "true_false", "true",
         None, None, None, None, "This is a famous neuroplasticity study.", "Maguire et al. (2000) showed London taxi drivers have enlarged posterior hippocampi from spatial navigation practice."),
        (5, "Which neurotransmitter is most associated with reward and motivation?", "multiple_choice", "Dopamine",
         Some("Serotonin"), Some("GABA"), Some("Dopamine"), Some("Acetylcholine"),
         "Think about what drives you to pursue goals.", "Dopamine is the primary neurotransmitter for reward signaling, motivation, and reinforcement learning."),
        (5, "SSRIs work by blocking the reuptake of _____.", "fill_in_blank", "serotonin",
         None, None, None, None, "The S in SSRI stands for this.", "Selective Serotonin Reuptake Inhibitors block serotonin reuptake, increasing its availability in the synaptic cleft."),
        (6, "During which sleep stage does most memory consolidation occur?", "multiple_choice", "Deep sleep (N3) and REM",
         Some("N1 (light sleep)"), Some("Deep sleep (N3) and REM"), Some("Only REM"), Some("Only N2"),
         "Two stages work together.", "Deep NREM sleep transfers declarative memories to cortex; REM consolidates emotional and procedural memories."),
        (6, "Integrated Information Theory measures consciousness using the symbol Φ (phi).", "true_false", "true",
         None, None, None, None, "IIT uses a specific mathematical measure.", "Giulio Tononi's IIT proposes that consciousness equals integrated information, quantified as Φ."),
    ];
    for (sort_topic, question, qtype, answer, oa, ob, oc, od, hint, explanation) in &quizzes {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort_topic], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![tid, question, qtype, answer, oa, ob, oc, od, hint, explanation],
        )?;
    }

    let path_steps: Vec<(i64, &str)> = vec![
        (1, "Learn how neurons communicate via electrical and chemical signals"),
        (2, "Explore the major brain regions and their functions"),
        (5, "Understand the neurotransmitter systems that drive behavior"),
        (3, "Discover how memories form, consolidate, and are retrieved"),
        (4, "Learn how the brain rewires itself through experience"),
        (6, "Explore the mysteries of consciousness and sleep"),
    ];
    for (i, (sort, desc)) in path_steps.iter().enumerate() {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('brain explorer', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_cryptography(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Cryptography'", [], |r| r.get(0)
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Cryptography', 'The science of secure communication — ciphers, encryption, hashing, and the mathematical foundations of digital security.')",
        [],
    )?;
    let sub_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Cryptography'", [], |r| r.get(0)
    )?;

    let topics = [
        ("Classical Ciphers", "beginner", 1),
        ("Symmetric Encryption", "intermediate", 2),
        ("Asymmetric Encryption", "intermediate", 3),
        ("Hash Functions", "intermediate", 4),
        ("Digital Signatures", "advanced", 5),
        ("Cryptographic Protocols", "advanced", 6),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sub_id, name, diff, order],
        )?;
    }

    let lessons: Vec<LessonRow> = vec![
        (1, "From Caesar to Enigma", "Classical ciphers are the ancestors of modern encryption. The Caesar cipher shifts each letter by a fixed number (ROT13 shifts by 13). The Vigenère cipher uses a keyword to vary the shift per letter, resisting simple frequency analysis. The Enigma machine used rotors and plugboards — its breaking by Turing's team shortened WWII by years.", 1),
        (1, "Breaking Classical Ciphers", "Frequency analysis exploits the fact that letters in natural language have predictable frequencies (E is most common in English). The Kasiski examination breaks Vigenère by finding repeated ciphertext sequences to deduce key length. Brute force works for short keys.", 2),
        (2, "Symmetric Key Encryption", "Symmetric encryption uses the same key for encryption and decryption. AES (Advanced Encryption Standard) is the gold standard: it processes data in 128-bit blocks using 10-14 rounds of substitution and permutation. Modes like CBC chain blocks together; CTR turns a block cipher into a stream cipher.", 1),
        (2, "Block Cipher Modes", "ECB (Electronic Codebook) encrypts each block independently — identical plaintext blocks produce identical ciphertext (bad!). CBC (Cipher Block Chaining) XORs each block with the previous ciphertext block. GCM (Galois/Counter Mode) provides both encryption and authentication.", 2),
        (3, "Public Key Cryptography", "Asymmetric encryption uses a key pair: a public key (shared freely) for encryption and a private key (kept secret) for decryption. RSA relies on the difficulty of factoring large semiprimes. Diffie-Hellman enables key exchange over insecure channels using the discrete logarithm problem.", 1),
        (3, "Elliptic Curve Cryptography", "ECC achieves the same security as RSA with much smaller keys. A 256-bit ECC key ≈ 3072-bit RSA key. It's based on the difficulty of the elliptic curve discrete logarithm problem (ECDLP). Used in TLS, Bitcoin, and modern key exchange (X25519).", 2),
        (4, "Cryptographic Hash Functions", "A hash function maps arbitrary data to a fixed-size digest. Properties: deterministic, fast, avalanche effect (small input change → huge output change), pre-image resistant (can't reverse), collision resistant (hard to find two inputs with same hash). SHA-256 produces a 256-bit hash.", 1),
        (4, "Applications of Hashing", "Password storage uses hashed+salted passwords (bcrypt, Argon2). Merkle trees hash pairs of transactions for blockchain integrity. HMAC combines a hash with a secret key for message authentication. Content-addressable storage (Git, IPFS) uses hashes as identifiers.", 2),
        (5, "How Digital Signatures Work", "Digital signatures prove authorship and integrity. The signer hashes the message, then encrypts the hash with their private key. Anyone can verify by decrypting with the public key and comparing hashes. RSA, ECDSA, and EdDSA are common algorithms.", 1),
        (5, "Certificates and PKI", "Public Key Infrastructure (PKI) uses certificate authorities (CAs) to bind public keys to identities. X.509 certificates contain a public key, identity info, and the CA's signature. Certificate chains create a trust hierarchy from root CAs to end-entity certificates.", 2),
        (6, "TLS and Secure Communication", "TLS (Transport Layer Security) secures web traffic. The handshake: client sends supported ciphers, server picks one and sends its certificate, they establish a shared session key via Diffie-Hellman, then communicate using symmetric encryption (AES-GCM).", 1),
        (6, "Zero-Knowledge Proofs", "A zero-knowledge proof lets you prove you know a secret without revealing it. Properties: completeness (honest prover convinces verifier), soundness (cheater can't fool verifier), zero-knowledge (verifier learns nothing beyond the statement's truth). Used in cryptocurrency privacy (ZK-SNARKs).", 2),
    ];
    for (idx, title, content, order) in &lessons {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (1, "Caesar Cipher", "A substitution cipher where each letter is shifted by a fixed number. With shift 3: A→D, B→E, C→F. Only 25 possible keys, trivially broken by brute force.", Some("Like a combination lock with only 25 positions — you can try them all in seconds."), Some("Why is the Caesar cipher still taught if it's so weak?")),
        (2, "AES Encryption", "AES processes 128-bit blocks through multiple rounds of SubBytes (substitution), ShiftRows (permutation), MixColumns (diffusion), and AddRoundKey (key mixing). Each round increases diffusion — a single bit change affects all output bits.", Some("Like mixing ingredients in a cake — each round of stirring makes it harder to separate the original ingredients."), Some("Why does AES use multiple rounds instead of just one complex operation?")),
        (3, "RSA Algorithm", "RSA security relies on the difficulty of factoring N = p × q where p and q are large primes. Public key is (N, e), private key is d where ed ≡ 1 (mod φ(N)). Encryption: c = m^e mod N. Decryption: m = c^d mod N.", Some("Like a mailbox — anyone can drop a letter in (encrypt with public key), but only the key holder can open it (decrypt with private key)."), Some("What makes RSA vulnerable to quantum computers?")),
        (4, "SHA-256", "SHA-256 processes data in 512-bit blocks through 64 rounds of compression. The avalanche effect means changing one input bit changes ~50% of output bits. Used in Bitcoin mining, certificate fingerprints, and file integrity verification.", Some("Like a fingerprint — unique to each person (message), but you can't reconstruct the person from the fingerprint."), Some("Why is SHA-256 considered collision resistant?")),
        (5, "Digital Signatures", "A digital signature is created by hashing a message and encrypting the hash with the sender's private key. Verification decrypts with the public key and compares hashes. This provides authentication (who sent it), integrity (not modified), and non-repudiation (sender can't deny it).", Some("Like signing a letter with a wax seal — it proves who sent it and shows if someone tampered with it."), Some("Why do digital signatures hash the message first instead of signing the whole message?")),
        (6, "Zero-Knowledge Proofs", "Imagine you want to prove you know the password to a cave with two paths that meet in the middle. You enter from a random side; the verifier asks you to exit from a specific side. If you know the password, you always succeed. After many rounds, the verifier is convinced without learning the password.", Some("Like proving you can solve a Rubik's cube blindfolded — the audience sees the result but not the moves."), Some("What is the difference between interactive and non-interactive zero-knowledge proofs?")),
    ];
    for (idx, concept, explanation, analogy, follow_up) in &explanations {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRowHint> = vec![
        (1, "The Caesar cipher shifts each letter by a fixed ___.", "fill_in_blank", "number",
         None, None, None, None, "It's a simple substitution technique.", "The Caesar cipher shifts each letter by a constant number of positions in the alphabet."),
        (1, "Which technique is used to break simple substitution ciphers?", "multiple_choice", "Frequency analysis",
         Some("Brute force only"), Some("Frequency analysis"), Some("Rainbow tables"), Some("Side-channel attacks"),
         "Think about letter frequency in natural language.", "Frequency analysis exploits predictable letter distributions (e.g., E is most common in English)."),
        (1, "The Enigma machine was broken by:", "multiple_choice", "Alan Turing's team at Bletchley Park",
         Some("The CIA"), Some("Albert Einstein"), Some("Alan Turing's team at Bletchley Park"), Some("IBM"),
         "A famous British codebreaker.", "Turing and colleagues built electromechanical 'bombes' to systematically test Enigma settings."),
        (2, "AES operates on blocks of ___ bits.", "fill_in_blank", "128",
         None, None, None, None, "It's a power of 2, between 64 and 256.", "AES uses a fixed block size of 128 bits, though key sizes can be 128, 192, or 256 bits."),
        (2, "ECB mode is insecure because:", "multiple_choice", "Identical plaintext blocks produce identical ciphertext blocks",
         Some("It's too slow"), Some("Identical plaintext blocks produce identical ciphertext blocks"), Some("It doesn't use a key"), Some("It only works with small files"),
         "Think about patterns leaking through.", "ECB encrypts each block independently, so patterns in plaintext are visible in ciphertext."),
        (3, "RSA security relies on the difficulty of:", "multiple_choice", "Factoring large semiprimes",
         Some("The discrete logarithm problem"), Some("Factoring large semiprimes"), Some("Solving linear equations"), Some("Finding hash collisions"),
         "It involves two large prime numbers multiplied together.", "RSA uses N = p × q where p and q are large primes; factoring N back into p and q is computationally infeasible."),
        (3, "A 256-bit ECC key provides security comparable to a ___-bit RSA key.", "fill_in_blank", "3072",
         None, None, None, None, "ECC is much more efficient — think thousands.", "ECC achieves equivalent security with much smaller keys: 256-bit ECC ≈ 3072-bit RSA."),
        (4, "Which property means a hash function's output changes dramatically with a tiny input change?", "multiple_choice", "Avalanche effect",
         Some("Determinism"), Some("Collision resistance"), Some("Avalanche effect"), Some("Pre-image resistance"),
         "Think about a small cause having a big effect.", "The avalanche effect ensures that changing even one bit of input changes approximately 50% of output bits."),
        (4, "True or false: You can reverse a SHA-256 hash to recover the original input.", "true_false", "false",
         None, None, None, None, "Think about pre-image resistance.", "Cryptographic hash functions are designed to be pre-image resistant — you cannot feasibly recover the input from the hash."),
        (5, "Digital signatures provide authentication, integrity, and ___.", "fill_in_blank", "non-repudiation",
         None, None, None, None, "The sender cannot deny having signed.", "Non-repudiation means the signer cannot later deny having created the signature."),
        (5, "In PKI, who signs end-entity certificates?", "multiple_choice", "Certificate Authorities (CAs)",
         Some("The end user"), Some("Certificate Authorities (CAs)"), Some("The web browser"), Some("DNS servers"),
         "There's a trusted third party.", "Certificate Authorities verify identities and sign certificates, creating a chain of trust."),
        (6, "TLS handshakes typically use ___ for key exchange.", "fill_in_blank", "Diffie-Hellman",
         None, None, None, None, "A key exchange protocol named after two cryptographers.", "Modern TLS uses (Elliptic Curve) Diffie-Hellman key exchange for forward secrecy."),
        (6, "Zero-knowledge proofs reveal the secret being proved.", "true_false", "false",
         None, None, None, None, "The name gives it away.", "By definition, zero-knowledge proofs prove knowledge of a secret without revealing any information about the secret itself."),
    ];
    for (idx, q, qtype, correct, oa, ob, oc, od, hint, expl) in &quizzes {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![tid, q, qtype, correct, oa, ob, oc, od, hint, expl],
        )?;
    }

    let path_steps: Vec<(i64, &str)> = vec![
        (1, "Start with classical ciphers to understand substitution and transposition"),
        (2, "Learn symmetric encryption (AES) and block cipher modes"),
        (4, "Understand hash functions and their applications"),
        (3, "Explore public key cryptography (RSA, ECC, Diffie-Hellman)"),
        (5, "Learn how digital signatures and PKI create trust"),
        (6, "Study protocols like TLS and advanced concepts like zero-knowledge proofs"),
    ];
    for (i, (sort, desc)) in path_steps.iter().enumerate() {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('crypto master', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

pub fn seed_information_theory(conn: &Connection) -> Result<(), rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Information Theory'", [], |r| r.get(0)
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Information Theory', 'The mathematics of information — entropy, compression, channel capacity, and the fundamental limits of communication.')",
        [],
    )?;
    let sub_id: i64 = conn.query_row(
        "SELECT id FROM subjects WHERE name = 'Information Theory'", [], |r| r.get(0)
    )?;

    let topics = [
        ("Entropy & Information", "beginner", 1),
        ("Source Coding", "intermediate", 2),
        ("Channel Capacity", "intermediate", 3),
        ("Error Correction", "intermediate", 4),
        ("Data Compression", "advanced", 5),
    ];
    for (name, diff, order) in &topics {
        conn.execute(
            "INSERT INTO topics (subject_id, name, difficulty, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![sub_id, name, diff, order],
        )?;
    }

    let lessons: Vec<LessonRow> = vec![
        (1, "What Is Information?", "Claude Shannon defined information as the reduction of uncertainty. A coin flip carries 1 bit of information (two equally likely outcomes). A die roll carries ~2.58 bits. Information content of an event with probability p is -log₂(p) bits. Rare events carry more information than common ones.", 1),
        (1, "Shannon Entropy", "Entropy H(X) = -Σ p(x) log₂ p(x) measures the average information per symbol in a source. Maximum entropy occurs when all outcomes are equally likely. English text has entropy ~1.0-1.5 bits per character (far less than 4.7 bits for 26 equiprobable letters) due to statistical patterns.", 2),
        (2, "The Source Coding Theorem", "Shannon's first theorem: a source with entropy H can be compressed to an average of H bits per symbol, but no fewer. This sets the fundamental limit of lossless compression. Huffman coding and arithmetic coding approach this limit.", 1),
        (2, "Huffman Coding", "Huffman coding assigns shorter codes to more frequent symbols and longer codes to rarer ones. Build a binary tree bottom-up: repeatedly merge the two least frequent nodes. The code for each symbol is the path from root to leaf. Optimal for symbol-by-symbol coding.", 2),
        (3, "Channel Capacity", "Shannon's channel coding theorem: every noisy channel has a capacity C (bits per use). Communication is possible at any rate below C with arbitrarily low error probability. The binary symmetric channel with crossover probability p has capacity C = 1 - H(p).", 1),
        (3, "Shannon's Noisy Channel Theorem", "The remarkable insight: by adding structured redundancy (error-correcting codes), we can communicate reliably over noisy channels at rates up to capacity C. This was initially doubted — many believed noise would always degrade communication proportionally.", 2),
        (4, "Error Detection and Correction", "Parity bits detect single errors. Hamming codes correct single errors and detect double errors using check bits at power-of-2 positions. The Hamming distance between codewords determines error-correcting capability: distance d corrects ⌊(d-1)/2⌋ errors.", 1),
        (4, "Modern Error Correction", "Reed-Solomon codes (used in CDs, QR codes) correct burst errors by operating on blocks of symbols. Turbo codes and LDPC codes approach Shannon's channel capacity. Polar codes (Arıkan, 2008) provably achieve capacity and are used in 5G communications.", 2),
        (5, "Lossless Compression", "Lossless compression preserves all data. LZ77/LZ78 (used in gzip, PNG) find repeated patterns and replace them with references. Burrows-Wheeler Transform (used in bzip2) rearranges data to group similar characters. Dictionary-based and statistical methods can be combined.", 1),
        (5, "Lossy Compression", "Lossy compression discards imperceptible information. JPEG uses DCT (Discrete Cosine Transform) to convert spatial data to frequency domain, then quantizes high-frequency components. MP3 uses psychoacoustic models to remove sounds below the hearing threshold. Rate-distortion theory describes the tradeoff.", 2),
    ];
    for (idx, title, content, order) in &lessons {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO lessons (topic_id, title, content, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![tid, title, content, order],
        )?;
    }

    let explanations: Vec<ExplanationRow> = vec![
        (1, "Shannon Entropy", "Entropy measures the average surprise or uncertainty in a random variable. High entropy = high uncertainty = more information per symbol. A biased coin (90% heads) has lower entropy than a fair coin because outcomes are more predictable.", Some("Entropy is like the average 'surprise factor' of a news feed — the more unpredictable the stories, the higher the entropy."), Some("Why does English text have much lower entropy than random letter sequences?")),
        (2, "Huffman Coding", "Huffman coding creates a prefix-free binary code where no codeword is a prefix of another. Frequent symbols get short codes (like Morse code: E = '.' is short because E is common). The resulting code is optimal among all symbol-by-symbol codes.", Some("Like packing a suitcase — frequently used items go on top (short codes) and rarely used items go at the bottom (long codes)."), Some("When might arithmetic coding outperform Huffman coding?")),
        (3, "Channel Capacity", "Channel capacity is the maximum rate at which information can be transmitted reliably. Shannon proved the existence of codes achieving capacity but didn't show how to construct them — this took 50 years until turbo codes (1993) and polar codes (2008).", Some("Like a highway's speed limit — you can drive at any speed below the limit safely, but exceeding it guarantees crashes."), Some("Why is it surprising that reliable communication is possible at any rate below capacity?")),
        (4, "Hamming Codes", "Hamming codes place check bits at positions that are powers of 2 (positions 1, 2, 4, 8...). Each check bit covers a specific set of data positions. When an error occurs, the pattern of failed check bits (the syndrome) directly indicates the position of the error.", Some("Like a Sudoku grid — if a number is wrong, the row and column constraints pinpoint exactly where the error is."), Some("What is the relationship between Hamming distance and error-correcting capability?")),
        (5, "Lossy Compression", "Lossy compression exploits human perceptual limitations. JPEG removes high-frequency visual details humans can barely see. MP3 removes sounds masked by louder nearby frequencies. The quality-size tradeoff is governed by rate-distortion theory.", Some("Like a painter creating an impression rather than a photograph — it captures the essence while leaving out details the viewer won't miss."), Some("How does the human visual system's sensitivity to luminance vs. chrominance affect JPEG compression?")),
    ];
    for (idx, concept, explanation, analogy, follow_up) in &explanations {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO explanations (topic_id, concept, explanation, analogy, follow_up_question) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tid, concept, explanation, analogy, follow_up],
        )?;
    }

    let quizzes: Vec<QuizRowHint> = vec![
        (1, "A fair coin flip carries ___ bit(s) of information.", "fill_in_blank", "1",
         None, None, None, None, "Two equally likely outcomes = log₂(2).", "A fair coin has two equally likely outcomes, so information = log₂(2) = 1 bit."),
        (1, "Entropy is maximized when all outcomes are:", "multiple_choice", "Equally likely",
         Some("Very rare"), Some("Equally likely"), Some("Deterministic"), Some("Binary"),
         "Think about maximum uncertainty.", "Maximum entropy occurs when all outcomes have equal probability — maximum uncertainty."),
        (1, "The entropy of English text is approximately ___ bits per character.", "fill_in_blank", "1",
         None, None, None, None, "It's much less than log₂(26) ≈ 4.7 bits.", "English text has ~1.0-1.5 bits per character due to statistical patterns and redundancy."),
        (2, "Shannon's source coding theorem says data cannot be compressed below its:", "multiple_choice", "Entropy",
         Some("Average length"), Some("Entropy"), Some("Variance"), Some("Bandwidth"),
         "It's the fundamental measure of information content.", "The source coding theorem establishes entropy as the fundamental compression limit."),
        (2, "In Huffman coding, more frequent symbols get ___ codes.", "fill_in_blank", "shorter",
         None, None, None, None, "Think about efficiency — common things should be quick.", "Huffman coding assigns shorter codewords to more frequent symbols, minimizing average code length."),
        (3, "Shannon proved reliable communication is possible at any rate below channel ___.", "fill_in_blank", "capacity",
         None, None, None, None, "The fundamental limit of a noisy channel.", "Shannon's channel coding theorem guarantees reliable communication at any rate below the channel capacity C."),
        (3, "True or false: Adding redundancy always reduces the effective data rate to zero.", "true_false", "false",
         None, None, None, None, "Shannon showed something surprising about redundancy.", "Shannon proved that structured redundancy allows reliable communication at positive rates up to capacity."),
        (4, "A Hamming code with distance d can correct up to ___ errors.", "fill_in_blank", "⌊(d-1)/2⌋",
         None, None, None, None, "Think about the formula relating distance and correction.", "A code with minimum Hamming distance d can correct up to ⌊(d-1)/2⌋ errors."),
        (4, "Which modern code is used in 5G communications?", "multiple_choice", "Polar codes",
         Some("Hamming codes"), Some("Reed-Solomon codes"), Some("Polar codes"), Some("Morse code"),
         "Invented by Arıkan in 2008.", "Polar codes, invented by Erdal Arıkan, provably achieve channel capacity and are used in 5G."),
        (5, "JPEG compression uses the ___ transform.", "fill_in_blank", "DCT",
         None, None, None, None, "Discrete Cosine... ", "JPEG uses the Discrete Cosine Transform (DCT) to convert image data to frequency domain."),
        (5, "True or false: Lossless compression can compress all files to at least 50% of their original size.", "true_false", "false",
         None, None, None, None, "Think about the pigeonhole principle.", "By the pigeonhole principle, no lossless algorithm can compress all possible inputs — some must grow."),
    ];
    for (idx, q, qtype, correct, oa, ob, oc, od, hint, expl) in &quizzes {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, idx], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, option_a, option_b, option_c, option_d, hint, explanation)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![tid, q, qtype, correct, oa, ob, oc, od, hint, expl],
        )?;
    }

    let path_steps: Vec<(i64, &str)> = vec![
        (1, "Understand entropy and the mathematical definition of information"),
        (2, "Learn source coding and compression fundamentals"),
        (3, "Explore channel capacity and Shannon's noisy channel theorem"),
        (4, "Study error detection and correction codes"),
        (5, "Apply compression techniques: lossless and lossy"),
    ];
    for (i, (sort, desc)) in path_steps.iter().enumerate() {
        let tid: i64 = conn.query_row(
            "SELECT id FROM topics WHERE subject_id = ?1 AND sort_order = ?2",
            rusqlite::params![sub_id, sort], |r| r.get(0)
        )?;
        conn.execute(
            "INSERT INTO learning_paths (goal, step_order, topic_id, description) VALUES ('information theorist', ?1, ?2, ?3)",
            rusqlite::params![i + 1, tid, desc],
        )?;
    }

    Ok(())
}

/// Seed additional quiz questions for core subjects to increase quiz depth.
pub fn seed_extra_core_quizzes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Extra Mathematics quizzes (subject_id=1)
    let math_topics: Vec<(i64, String)> = conn.prepare(
        "SELECT t.id, t.name FROM topics t JOIN subjects s ON t.subject_id = s.id WHERE s.name = 'Mathematics'"
    )?.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.filter_map(|r| r.ok()).collect();

    for (tid, name) in &math_topics {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM quiz_questions WHERE topic_id = ?1", [tid], |r| r.get(0)
        )?;
        if count >= 6 { continue; } // Already has enough questions

        match name.as_str() {
            "Arithmetic" => {
                let extras = [
                    ("What is 15 × 12?", "fill_in_blank", "180", "Multiply step by step: 15 × 12 = 15 × 10 + 15 × 2.", "Use the distributive property."),
                    ("True or false: Division by zero is undefined.", "true_false", "true", "Division by zero has no meaningful result — it's undefined in standard arithmetic.", "There is no number that, multiplied by zero, gives a non-zero result."),
                    ("Order these operations by precedence: Addition, Exponents, Multiplication, Parentheses", "ordering", "Parentheses,Exponents,Multiplication,Addition", "PEMDAS/BODMAS: Parentheses first, then Exponents, then Multiplication/Division, then Addition/Subtraction.", "Think of the PEMDAS acronym."),
                ];
                for (q, qt, ans, expl, hint) in &extras {
                    conn.execute(
                        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        rusqlite::params![tid, q, qt, ans, hint, expl],
                    )?;
                }
            }
            "Fractions" => {
                let extras = [
                    ("What is 3/4 + 1/4?", "fill_in_blank", "1", "3/4 + 1/4 = 4/4 = 1. When denominators match, add numerators.", "Same denominators make this easy."),
                    ("Which fraction is equivalent to 0.75?", "multiple_choice", "3/4", "0.75 = 75/100 = 3/4. Divide both numerator and denominator by 25.", "Think about what fraction of a dollar is 75 cents."),
                    ("True or false: 2/3 is greater than 3/4.", "true_false", "false", "2/3 ≈ 0.667 while 3/4 = 0.75. So 3/4 > 2/3.", "Convert both to decimals to compare."),
                ];
                for (q, qt, ans, expl, hint) in &extras {
                    conn.execute(
                        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        rusqlite::params![tid, q, qt, ans, hint, expl],
                    )?;
                }
            }
            "Algebra Basics" => {
                let extras = [
                    ("If 2x + 6 = 14, what is x?", "fill_in_blank", "4", "2x + 6 = 14 → 2x = 8 → x = 4. Subtract 6, then divide by 2.", "Isolate x by undoing operations."),
                    ("The expression x² - 9 factors into:", "multiple_choice", "(x+3)(x-3)", "x² - 9 is a difference of squares: a² - b² = (a+b)(a-b) where a=x, b=3.", "This is a difference of squares."),
                ];
                for (q, qt, ans, expl, hint) in &extras {
                    conn.execute(
                        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        rusqlite::params![tid, q, qt, ans, hint, expl],
                    )?;
                }
            }
            _ => {}
        }
    }

    // Extra Science quizzes (subject_id=2)
    let sci_topics: Vec<(i64, String)> = conn.prepare(
        "SELECT t.id, t.name FROM topics t JOIN subjects s ON t.subject_id = s.id WHERE s.name = 'Science'"
    )?.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.filter_map(|r| r.ok()).collect();

    for (tid, name) in &sci_topics {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM quiz_questions WHERE topic_id = ?1", [tid], |r| r.get(0)
        )?;
        if count >= 6 { continue; }

        match name.as_str() {
            "Photosynthesis" => {
                let extras = [
                    ("The chemical equation for photosynthesis: 6CO₂ + 6H₂O → C₆H₁₂O₆ + ___.", "fill_in_blank", "6O₂", "Photosynthesis produces glucose and oxygen: 6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂.", "The byproduct is what we breathe."),
                    ("Photosynthesis occurs in which organelle?", "multiple_choice", "Chloroplast", "Chloroplasts contain chlorophyll, the green pigment that captures light energy.", "It contains chlorophyll."),
                ];
                for (q, qt, ans, expl, hint) in &extras {
                    conn.execute(
                        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        rusqlite::params![tid, q, qt, ans, hint, expl],
                    )?;
                }
            }
            "States of Matter" => {
                let extras = [
                    ("Order the states of matter by increasing molecular energy: Solid, Gas, Liquid", "ordering", "Solid,Liquid,Gas", "Molecules in solids have the least kinetic energy, gases the most.", "Think about how much molecules move in each state."),
                    ("The process of a solid turning directly into a gas is called ___.", "fill_in_blank", "sublimation", "Sublimation skips the liquid phase. Dry ice (CO₂) is a common example.", "Think of dry ice."),
                ];
                for (q, qt, ans, expl, hint) in &extras {
                    conn.execute(
                        "INSERT INTO quiz_questions (topic_id, question, question_type, correct_answer, hint, explanation) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        rusqlite::params![tid, q, qt, ans, hint, expl],
                    )?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
