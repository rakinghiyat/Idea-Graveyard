#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data DeadIdea
#[contracttype]
#[derive(Clone, Debug)]
pub struct DeadIdea {
    pub id: u64,
    pub title: String,
    pub reason: String,
}

// Storage key
const IDEA_DATA: Symbol = symbol_short!("IDEA_DATA");

#[contract]
pub struct IdeaGraveyardContract;

#[contractimpl]
impl IdeaGraveyardContract {

    // =========================
    // GET ALL IDEAS
    // =========================
    pub fn get_ideas(env: Env) -> Vec<DeadIdea> {
        return env
            .storage()
            .instance()
            .get(&IDEA_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // =========================
    // CREATE IDEA
    // =========================
    pub fn create_idea(env: Env, title: String, reason: String) -> String {
        // 1. ambil data lama
        let mut ideas: Vec<DeadIdea> = env
            .storage()
            .instance()
            .get(&IDEA_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. buat object baru
        let idea = DeadIdea {
            id: env.prng().gen::<u64>(),
            title: title,
            reason: reason,
        };

        // 3. push ke vector
        ideas.push_back(idea);

        // 4. simpan kembali
        env.storage().instance().set(&IDEA_DATA, &ideas);

        String::from_str(&env, "Idea berhasil dikuburkan 💀")
    }

    // =========================
    // DELETE IDEA
    // =========================
    pub fn delete_idea(env: Env, id: u64) -> String {
        // 1. ambil data
        let mut ideas: Vec<DeadIdea> = env
            .storage()
            .instance()
            .get(&IDEA_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. cari & hapus
        for i in 0..ideas.len() {
            if ideas.get(i).unwrap().id == id {
                ideas.remove(i);

                // 3. simpan ulang
                env.storage().instance().set(&IDEA_DATA, &ideas);

                return String::from_str(&env, "Idea berhasil dihapus dari kuburan 🪦");
            }
        }

        String::from_str(&env, "Idea tidak ditemukan")
    }
}

mod test;