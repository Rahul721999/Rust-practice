
fn main() {
    let password = "PAsWorD@12#3*";
    analyze_pass(password);
    let _pass = gen_pass();
    let hash = gen_pass_with_salt(&password);
    verify_pass_with_salt(&hash.0, &password, &hash.1)
}

fn gen_pass()-> String {
    use passwords::PasswordGenerator;

    let pg = PasswordGenerator {
        length: 8,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    println!("{}", pg.generate_one().expect("failed to gen pass"));
    println!(
        "{:?}",
        pg.generate(4).expect("failed to gen pass with generate()")
    );
    return pg.generate_one().unwrap()
}

fn gen_pass_with_salt(password : &str)-> ([u8;16],[u8; 24]) {
    use passwords::hasher;

    let salt  = hasher::gen_salt();

    let hashed = hasher::bcrypt(10, &salt, password).unwrap();

    println!("{:?}", hashed);
    return (salt,hashed)
}


/// to verify same salt is must.
fn verify_pass_with_salt(salt: &[u8], password : &str, hashed : &[u8]){
    let flag = unsafe { passwords::hasher::identify_bcrypt(10, salt, password, hashed) };
    println!("{}", flag);
}

fn analyze_pass(password : &str){
    use passwords::{analyzer, scorer};
    let res = analyzer::analyze(password);
    let score = scorer::score(&res) as i64;
    println!("password-score: {}", score);
    match score{
        0..=20 =>{print!("{score}")},
        21..=40=>{print!("{score}")},
        41..=60=>{print!("{score}")},
        61..=80=>{print!("{score}")},
        81..=90=>{print!("{score}")},
        91..=95=>{print!("{score}")},
        96..=100=>{print!("{score}")},
        _ => {print!("out of bound score : {score}")}
    }
}