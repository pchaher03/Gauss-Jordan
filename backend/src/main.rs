use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MatrixRequest {
    matrix: Vec<Vec<f64>>,
}

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

async fn solve_matrix(data: web::Json<MatrixRequest>) -> impl Responder {
    let mut matrix = data.matrix.clone();
    let solution = gauss_jordan(&mut matrix);
    HttpResponse::Ok().json(solution)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor backend iniciado en http://localhost:8080");
    println!("Ejecuta el frontend por separado en otra terminal");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/solve", web::post().to(solve_matrix))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}