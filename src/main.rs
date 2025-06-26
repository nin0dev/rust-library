use std::io;

// Structure représentant un livre
#[derive(Debug, Clone)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

impl Livre {
    fn new(titre: String, auteur: String, annee: u32) -> Self {
        Livre {
            titre,
            auteur,
            annee,
            disponible: true,
        }
    }

    fn emprunter(&mut self) -> bool {
        if self.disponible {
            self.disponible = false;
            true
        } else {
            false
        }
    }

    fn retourner(&mut self) -> bool {
        if !self.disponible {
            self.disponible = true;
            true
        } else {
            false
        }
    }
}

// Structure principale de la bibliothèque
struct Bibliotheque {
    livres: Vec<Livre>,
}

impl Bibliotheque {
    fn new() -> Self {
        Bibliotheque {
            livres: Vec::new(),
        }
    }

    fn ajouter_livre(&mut self, titre: String, auteur: String, annee: u32) -> Result<(), String> {
        // Validation : pas de titre en double
        if self.livres.iter().any(|livre| livre.titre.to_lowercase() == titre.to_lowercase()) {
            return Err("Un livre avec ce titre existe déjà".to_string());
        }

        let livre = Livre::new(titre, auteur, annee);
        self.livres.push(livre);
        Ok(())
    }

    fn emprunter_livre(&mut self, titre: &str) -> Result<(), String> {
        match self.livres.iter_mut().find(|livre| livre.titre.to_lowercase() == titre.to_lowercase()) {
            Some(livre) => {
                if livre.emprunter() {
                    Ok(())
                } else {
                    Err("Ce livre n'est pas disponible".to_string())
                }
            }
            None => Err("Livre non trouvé".to_string()),
        }
    }

    fn retourner_livre(&mut self, titre: &str) -> Result<(), String> {
        match self.livres.iter_mut().find(|livre| livre.titre.to_lowercase() == titre.to_lowercase()) {
            Some(livre) => {
                if livre.retourner() {
                    Ok(())
                } else {
                    Err("Ce livre n'était pas emprunté".to_string())
                }
            }
            None => Err("Livre non trouvé".to_string()),
        }
    }

    fn afficher_tous_livres(&self) {
        if self.livres.is_empty() {
            println!("Aucun livre dans la bibliothèque.");
            return;
        }

        println!("\n=== TOUS LES LIVRES ===");
        for (index, livre) in self.livres.iter().enumerate() {
            let statut = if livre.disponible { "Disponible" } else { "Emprunté" };
            println!("{}. {} - {} ({}) - {}", 
                index + 1, livre.titre, livre.auteur, livre.annee, statut);
        }
        println!();
    }

    fn afficher_livres_disponibles(&self) {
        let livres_disponibles: Vec<&Livre> = self.livres
            .iter()
            .filter(|livre| livre.disponible)
            .collect();

        if livres_disponibles.is_empty() {
            println!("Aucun livre disponible.");
            return;
        }

        println!("\n=== LIVRES DISPONIBLES ===");
        for (index, livre) in livres_disponibles.iter().enumerate() {
            println!("{}. {} - {} ({})", 
                index + 1, livre.titre, livre.auteur, livre.annee);
        }
        println!();
    }
}

fn afficher_menu() {
    println!("\n=== GESTION DE BIBLIOTHÈQUE ===");
    println!("1. Ajouter un livre");
    println!("2. Emprunter un livre");
    println!("3. Retourner un livre");
    println!("4. Afficher tous les livres");
    println!("5. Afficher les livres disponibles");
    println!("6. Quitter");
    print!("Votre choix : ");
}

fn lire_entree() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn lire_annee() -> Result<u32, String> {
    let input = lire_entree();
    match input.parse::<u32>() {
        Ok(annee) if annee >= 1000 && annee <= 2025 => Ok(annee),
        Ok(_) => Err("L'année doit être entre 1000 et 2025".to_string()),
        Err(_) => Err("Veuillez entrer une année valide".to_string()),
    }
}

fn ajouter_livre_interface(bibliotheque: &mut Bibliotheque) {
    println!("\n=== AJOUTER UN LIVRE ===");
    
    print!("Titre : ");
    let titre = lire_entree();
    if titre.is_empty() {
        println!("Le titre ne peut pas être vide.");
        return;
    }

    print!("Auteur : ");
    let auteur = lire_entree();
    if auteur.is_empty() {
        println!("L'auteur ne peut pas être vide.");
        return;
    }

    print!("Année de publication : ");
    let annee = match lire_annee() {
        Ok(a) => a,
        Err(e) => {
            println!("Erreur : {}", e);
            return;
        }
    };

    match bibliotheque.ajouter_livre(titre.clone(), auteur, annee) {
        Ok(()) => println!("Livre '{}' ajouté avec succès !", titre),
        Err(e) => println!("Erreur : {}", e),
    }
}

fn emprunter_livre_interface(bibliotheque: &mut Bibliotheque) {
    println!("\n=== EMPRUNTER UN LIVRE ===");
    
    if bibliotheque.livres.iter().all(|livre| !livre.disponible) {
        println!("Aucun livre disponible pour l'emprunt.");
        return;
    }

    print!("Titre du livre à emprunter : ");
    let titre = lire_entree();
    
    if titre.is_empty() {
        println!("Le titre ne peut pas être vide.");
        return;
    }

    match bibliotheque.emprunter_livre(&titre) {
        Ok(()) => println!("Livre '{}' emprunté avec succès !", titre),
        Err(e) => println!("Erreur : {}", e),
    }
}

fn retourner_livre_interface(bibliotheque: &mut Bibliotheque) {
    println!("\n=== RETOURNER UN LIVRE ===");
    
    if bibliotheque.livres.iter().all(|livre| livre.disponible) {
        println!("Aucun livre n'est actuellement emprunté.");
        return;
    }

    print!("Titre du livre à retourner : ");
    let titre = lire_entree();
    
    if titre.is_empty() {
        println!("Le titre ne peut pas être vide.");
        return;
    }

    match bibliotheque.retourner_livre(&titre) {
        Ok(()) => println!("Livre '{}' retourné avec succès !", titre),
        Err(e) => println!("Erreur : {}", e),
    }
}

fn main() {
    let mut bibliotheque = Bibliotheque::new();
    
    println!("Bienvenue dans le gestionnaire de bibliothèque !");

    loop {
        afficher_menu();
        let choix = lire_entree();

        match choix.as_str() {
            "1" => ajouter_livre_interface(&mut bibliotheque),
            "2" => emprunter_livre_interface(&mut bibliotheque),
            "3" => retourner_livre_interface(&mut bibliotheque),
            "4" => bibliotheque.afficher_tous_livres(),
            "5" => bibliotheque.afficher_livres_disponibles(),
            "6" => {
                println!("Merci d'avoir utilisé le gestionnaire de bibliothèque !");
                break;
            }
            _ => println!("Choix invalide. Veuillez choisir une option entre 1 et 6."),
        }
    }
}