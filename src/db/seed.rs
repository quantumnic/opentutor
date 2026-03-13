use rusqlite::Connection;

type LessonRow<'a> = (i64, &'a str, &'a str, i64);
type ExplanationRow<'a> = (i64, &'a str, &'a str, Option<&'a str>, Option<&'a str>);
type QuizRow<'a> = (i64, &'a str, &'a str, &'a str, Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>, &'a str);

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
        assert_eq!(count, 24); // 16 original + Chemistry + Biology + Sociology + Linguistics + Statistics & Data + Ethics + World Literature
    }

    #[test]
    fn test_seed_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 24);
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
        for name in &["Psychology", "Environmental Science"] {
            let topic_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM topics t JOIN subjects s ON s.id = t.subject_id WHERE s.name = ?1",
                [name], |r| r.get(0)
            ).unwrap();
            assert_eq!(topic_count, 4, "{} should have 4 topics", name);

            let lesson_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM lessons l JOIN topics t ON t.id = l.topic_id JOIN subjects s ON s.id = t.subject_id WHERE s.name = ?1",
                [name], |r| r.get(0)
            ).unwrap();
            assert!(lesson_count >= 8, "{} should have at least 8 lessons, got {}", name, lesson_count);
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
        assert_eq!(topic_count, 4);
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
    let lessons: Vec<(i64, &str, &str, i64)> = vec![
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
    let explanations: Vec<(i64, &str, &str, Option<&str>, Option<&str>)> = vec![
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
    let questions: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str)> = vec![
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
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) > 0 FROM subjects WHERE name = 'Linguistics'",
        [], |r| r.get(0),
    ).unwrap_or(false);
    if exists { return Ok(()); }

    conn.execute(
        "INSERT INTO subjects (name, description) VALUES ('Linguistics', 'The scientific study of language — its structure, meaning, sounds, and evolution across cultures.')",
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
    let questions: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
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
    let questions: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
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
    let lessons: Vec<(i64, &str, &str, i64)> = vec![
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
    let explanations: Vec<(i64, &str, &str, Option<&str>, Option<&str>)> = vec![
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
    let questions: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str, &str)> = vec![
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
    let logic_qs: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str)> = vec![
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
    let logic_explanations: Vec<(i64, &str, &str, Option<&str>, Option<&str>)> = vec![
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
    let fa_qs: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str)> = vec![
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
    let nut_qs: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str)> = vec![
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
    let hyg_qs: Vec<(i64, &str, &str, &str, Option<&str>, Option<&str>, Option<&str>, Option<&str>, Option<&str>, &str)> = vec![
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
