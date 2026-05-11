
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec,
};

// Struktur email
#[contracttype]
#[derive(Clone, Debug)]
pub struct Email {
    id: u64,
    sender: String,
    subject: String,
    content: String,
    is_spam: bool,
}

// Storage key
const EMAIL_DATA: Symbol = symbol_short!("EMAILS");

#[contract]
pub struct SpamFilterContract;

#[contractimpl]
impl SpamFilterContract {

    // Ambil semua email
    pub fn get_emails(env: Env) -> Vec<Email> {
        env.storage()
            .instance()
            .get(&EMAIL_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah email baru + deteksi spam sederhana
    pub fn add_email(
        env: Env,
        sender: String,
        subject: String,
        content: String,
    ) -> String {

        // Ambil data lama
        let mut emails: Vec<Email> = env.storage()
            .instance()
            .get(&EMAIL_DATA)
            .unwrap_or(Vec::new(&env));

        // Keyword spam sederhana
        let spam_keywords = [
            "gratis",
            "menang",
            "hadiah",
            "klik",
            "promo",
            "uang",
        ];

        // Default bukan spam
        let mut is_spam = false;

        // Gabungkan subject + content
        let text = subject.clone() + &content;

        // Cek keyword spam
        for keyword in spam_keywords.iter() {
            if text.contains(String::from_str(&env, keyword)) {
                is_spam = true;
            }
        }

        // Buat object email
        let email = Email {
            id: env.prng().gen::<u64>(),
            sender,
            subject,
            content,
            is_spam,
        };

        // Simpan email
        emails.push_back(email);

        env.storage().instance().set(&EMAIL_DATA, &emails);

        String::from_str(&env, "Email berhasil ditambahkan")
    }

    // Ambil email spam saja
    pub fn get_spam_emails(env: Env) -> Vec<Email> {

        let emails: Vec<Email> = env.storage()
            .instance()
            .get(&EMAIL_DATA)
            .unwrap_or(Vec::new(&env));

        let mut spam_emails = Vec::new(&env);

        for i in 0..emails.len() {
            let email = emails.get(i).unwrap();

            if email.is_spam {
                spam_emails.push_back(email);
            }
        }

        spam_emails
    }

    // Hapus email berdasarkan id
    pub fn delete_email(env: Env, id: u64) -> String {

        let mut emails: Vec<Email> = env.storage()
            .instance()
            .get(&EMAIL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..emails.len() {

            if emails.get(i).unwrap().id == id {

                emails.remove(i);

                env.storage().instance().set(&EMAIL_DATA, &emails);

                return String::from_str(&env, "Email berhasil dihapus");
            }
        }

        String::from_str(&env, "Email tidak ditemukan")
    }
}

mod test;