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
        assert_eq!(count, 15);
    }

    #[test]
    fn test_seed_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 15);
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
}
