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
        // Extra lessons for existing topics
        (1, "Order of Operations", "When an expression has multiple operations, follow PEMDAS/BODMAS:\n\n1. Parentheses / Brackets\n2. Exponents / Orders\n3. Multiplication & Division (left to right)\n4. Addition & Subtraction (left to right)\n\nExample: 3 + 4 × 2 = 3 + 8 = 11 (not 14!)\nExample: (3 + 4) × 2 = 7 × 2 = 14", 3),
        (2, "Multiplying & Dividing Fractions", "Multiplying: multiply numerators, multiply denominators.\n2/3 × 4/5 = 8/15\n\nDividing: flip the second fraction and multiply.\n2/3 ÷ 4/5 = 2/3 × 5/4 = 10/12 = 5/6\n\nTip: simplify before multiplying to keep numbers small.\nExample: 3/4 × 2/9 → cancel 3: 1/4 × 2/3 = 2/12 = 1/6", 3),
        (5, "Perimeter and Area", "Perimeter is the distance around a shape.\nArea is the space inside.\n\nRectangle: P = 2(l + w), A = l × w\nTriangle: A = 1/2 × base × height\nCircle: C = 2πr, A = πr²\n\nπ ≈ 3.14159\n\nExample: Circle with r=5: A = π × 25 ≈ 78.5 square units", 2),
        (8, "Newton's Laws of Motion", "First Law (Inertia): An object stays at rest or in motion unless a force acts on it.\n\nSecond Law: Force = mass × acceleration (F = ma)\n\nThird Law: Every action has an equal and opposite reaction.\n\nExamples:\n- Seatbelts (1st law)\n- Pushing a heavy vs light box (2nd law)\n- Rocket engines (3rd law)", 2),
        (9, "Plasma and Beyond", "Beyond the three common states:\n\nPlasma: super-heated gas with charged particles.\nExamples: lightning, the Sun, neon signs.\n\nBose-Einstein Condensate: super-cooled atoms near absolute zero.\n\nPhase diagram: shows which state exists at different temperatures and pressures.\n\nTriple point: where solid, liquid, and gas coexist.", 2),
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
        // True/false questions for existing topics
        (1, "True or false: 7 × 0 = 7", "true_false", "false", Some("true"), Some("false"), None, None, Some("Any number times zero is..."), "False. Any number multiplied by zero equals zero."),
        (5, "True or false: A triangle can have two right angles.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Angles in a triangle sum to 180°"), "False. Two right angles = 180°, leaving 0° for the third angle, which is impossible."),
        (8, "True or false: Objects fall at the same rate in a vacuum regardless of mass.", "true_false", "true", Some("true"), Some("false"), None, None, Some("Think about Galileo's experiment"), "True. In a vacuum (no air resistance), all objects accelerate at the same rate due to gravity."),
        // Extra multiple-choice for under-served topics
        (10, "Which sentence is correct?", "multiple_choice", "She and I went to the store.", Some("Her and me went to the store."), Some("She and I went to the store."), Some("Her and I went to the store."), Some("She and me went to the store."), Some("Try each pronoun alone: 'I went' or 'me went'?"), "Use subject pronouns (she, I) for subjects. 'She went' and 'I went' are both correct."),
        (11, "What is the main idea of a passage?", "multiple_choice", "The central point the author is making", Some("The first sentence"), Some("The central point the author is making"), Some("The longest paragraph"), Some("Any interesting detail"), Some("It's what the whole text is about"), "The main idea is the central point or message the author wants to convey."),
        (12, "Which civilization developed cuneiform writing?", "multiple_choice", "Mesopotamia", Some("Egypt"), Some("Mesopotamia"), Some("China"), Some("Indus Valley"), Some("Between two famous rivers"), "Mesopotamia (modern Iraq) developed cuneiform, one of the earliest writing systems."),
        (13, "What event triggered World War I?", "multiple_choice", "Assassination of Archduke Franz Ferdinand", Some("Sinking of the Lusitania"), Some("Assassination of Archduke Franz Ferdinand"), Some("Invasion of Poland"), Some("Treaty of Versailles"), Some("It happened in Sarajevo in 1914"), "The assassination of Archduke Franz Ferdinand of Austria-Hungary in Sarajevo (1914) triggered WWI."),
        (14, "Where did the Industrial Revolution begin?", "multiple_choice", "Britain", Some("France"), Some("Germany"), Some("Britain"), Some("United States"), Some("An island nation"), "The Industrial Revolution began in Britain in the late 18th century."),
        // Computer Science questions
        (18, "What is the binary representation of the number 10?", "multiple_choice", "1010", Some("1001"), Some("1010"), Some("1100"), Some("1110"), Some("10 = 8 + 2"), "10 in binary = 1010 (8 + 0 + 2 + 0)."),
        (18, "How many values can 1 byte store?", "multiple_choice", "256", Some("8"), Some("128"), Some("256"), Some("512"), Some("2 to the power of 8"), "1 byte = 8 bits. 2^8 = 256 possible values (0-255)."),
        (19, "Which sorting algorithm divides the list in half repeatedly?", "multiple_choice", "Merge sort", Some("Bubble sort"), Some("Merge sort"), Some("Selection sort"), Some("Insertion sort"), Some("Divide and conquer"), "Merge sort divides the list in half, sorts each half, then merges them."),
        (19, "True or false: Binary search requires a sorted list.", "true_false", "true", Some("true"), Some("false"), None, None, Some("How does it know which half to pick?"), "True. Binary search only works on sorted data — it needs order to eliminate half the options."),
        (20, "What does DRY stand for in programming?", "multiple_choice", "Don't Repeat Yourself", Some("Do Run Yesterday"), Some("Don't Repeat Yourself"), Some("Data Reads Yield"), Some("Debug Runtime Yearly"), Some("It's about avoiding duplication"), "DRY = Don't Repeat Yourself. Avoid writing the same code in multiple places."),
        (20, "True or false: A syntax error means the code logic is wrong.", "true_false", "false", Some("true"), Some("false"), None, None, Some("Syntax is about grammar, not logic"), "False. A syntax error is a grammar mistake (like a missing bracket). A logic error means the code runs but produces wrong results."),
        (21, "What does CPU stand for?", "multiple_choice", "Central Processing Unit", Some("Central Processing Unit"), Some("Computer Personal Utility"), Some("Core Program Unit"), Some("Central Power Unit"), Some("It's the 'brain' of the computer"), "CPU = Central Processing Unit. It executes instructions."),
        // Geography questions
        (22, "What is the largest continent by area?", "multiple_choice", "Asia", Some("Africa"), Some("Asia"), Some("North America"), Some("Europe"), Some("It has the most people too"), "Asia is the largest continent, covering about 44.6 million km²."),
        (22, "Which is the deepest ocean?", "multiple_choice", "Pacific", Some("Atlantic"), Some("Indian"), Some("Pacific"), Some("Arctic"), Some("It's also the largest"), "The Pacific Ocean is both the largest and deepest, containing the Mariana Trench."),
        (23, "True or false: Climate and weather are the same thing.", "true_false", "false", Some("true"), Some("false"), None, None, Some("One is short-term, one is long-term"), "False. Weather is short-term conditions; climate is the average pattern over 30+ years."),
        (24, "What do latitude lines measure?", "multiple_choice", "Distance north or south of the equator", Some("Distance east or west of Greenwich"), Some("Distance north or south of the equator"), Some("Elevation above sea level"), Some("Distance between cities"), Some("Think horizontal lines"), "Latitude lines run horizontally, measuring distance north or south of the equator (0°)."),
        (25, "Which of these is a renewable resource?", "multiple_choice", "Solar energy", Some("Coal"), Some("Natural gas"), Some("Solar energy"), Some("Oil"), Some("It won't run out"), "Solar energy is renewable — the sun continuously provides energy."),
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
        ("programming", 1, 18, "Understand binary — how computers represent data"),
        ("programming", 2, 19, "Learn algorithms — step-by-step problem solving"),
        ("programming", 3, 20, "Programming concepts — variables, loops, functions"),
        ("programming", 4, 21, "Computer hardware — understand what runs your code"),
        ("world geography", 1, 22, "Continents & oceans — the big picture"),
        ("world geography", 2, 23, "Weather & climate — understanding our atmosphere"),
        ("world geography", 3, 24, "Maps & navigation — finding your way"),
        ("world geography", 4, 25, "Natural resources — what Earth provides"),
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
        assert_eq!(count, 7);
    }

    #[test]
    fn test_seed_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        schema::create_tables(&conn).unwrap();
        seed_if_empty(&conn).unwrap();
        seed_if_empty(&conn).unwrap(); // should not duplicate
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM subjects", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 7);
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
