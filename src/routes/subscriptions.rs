use crate::model::domain::NewSubscriber;
use actix_web::{web, HttpResponse};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Adding a new subscriber",skip(form,pool),fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name
))]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = match NewSubscriber::parse(form.email.clone(), form.name.clone()) {
        Ok(new_subscriber) => new_subscriber,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };
    match insert_subscriber_to_db(new_subscriber, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber_to_db(
    new_subscriber: NewSubscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id,email, name,subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
