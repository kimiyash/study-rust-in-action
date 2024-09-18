use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum THing {
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut THing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng()
            .gen_bool(probability_of_success);

        println!("{:?} が呪文をつぶやく", self);
        if spell_is_successful {
            println!("{:?} は明るく光る", thing);
        } else {
            println!("{:?} はシューッと言って、つまらないものに変わる", thing);
            *thing = THing::Trinket {};
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

fn main() {
    let mut it = THing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);
}
