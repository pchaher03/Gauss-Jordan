use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;
use std::fs;

fn gauss_jordan(matrix: &mut Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let n = matrix.len();

    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n {
            if matrix[k][i].abs() > matrix[max_row][i].abs() {
                max_row = k;
            }
        }

        if matrix[max_row][i] == 0.0 {
            return None;
        }

        matrix.swap(i, max_row);
        let pivot = matrix[i][i];
        for j in 0..n + 1 {
            matrix[i][j] /= pivot;
        }

        for k in 0..n {
            if k != i {
                let factor = matrix[k][i];
                for j in 0..n + 1 {
                    matrix[k][j] -= factor * matrix[i][j];
                }
            }
        }
    }

    Some(matrix.iter().map(|row| row[n]).collect())
}

#[derive(Serialize, Deserialize)]
struct MatrixRequest {
    matrix: Vec<Vec<f64>>,
}

async fn solve_matrix(data: web::Json<MatrixRequest>) -> impl Responder {
    let mut matrix = data.matrix.clone();
    let solution = gauss_jordan(&mut matrix);
    HttpResponse::Ok().json(solution)
}

fn open_browser() {
    if cfg!(target_os = "linux") {
        let _ = Command::new("xdg-open")
            .arg("http://localhost:3000") // URL del frontend
            .spawn();
    } else if cfg!(target_os = "macos") {
        let _ = Command::new("open")
            .arg("http://localhost:3000") // URL del frontend
            .spawn();
    } else if cfg!(target_os = "windows") {
        let _ = Command::new("cmd")
            .args(&["/C", "start", "http://localhost:3000"]) // URL del frontend
            .spawn();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ruta absoluta del frontend
    let frontend_path = "../frontend";

    // Verificar si el directorio de build existe
    if !fs::metadata(format!("{}/build", frontend_path)).is_ok() {
        println!("Compilando el frontend...");
        Command::new("npm")
            .arg("install")
            .current_dir(frontend_path)
            .status()
            .expect("Error instalando dependencias del frontend");

        Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir(frontend_path)
            .status()
            .expect("Error compilando el frontend");
    }

    // Iniciar el frontend en el puerto 3000
    thread::spawn(move || {
        Command::new("npm")
            .arg("start")
            .current_dir(frontend_path)
            .status()
            .expect("Error iniciando el frontend");
    });

    // Abrir el navegador
    thread::sleep(std::time::Duration::from_secs(3)); // Esperar unos segundos
    open_browser(); // Llamar a la función para abrir el navegador

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/solve", web::post().to(solve_matrix))
            .service(Files::new("/", "./frontend/build").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
