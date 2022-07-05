use super::super::viewmodel::user_viewmodel::UserId;
use super::response::{AppError, Result};
use crate::domain::model::user_model::Model as User;
use crate::usecase::Usecases;
use axum::{
    extract::{rejection::PathRejection, Extension, Path},
    Json,
};
use std::sync::Arc;

pub async fn index(Extension(usecases): Extension<Arc<Usecases>>) -> Result<Json<Vec<User>>> {
    let users = usecases.user_usecase.get_all().await;
    Ok(Json(users))
}

pub async fn get_by_id(
    Extension(usecases): Extension<Arc<Usecases>>,
    param: Result<Path<UserId>, PathRejection>,
) -> Result<Json<User>> {
    let id = match param {
        Ok(Path(id)) => id,
        Err(_) => return Err(AppError::BadRequest("User ID is invalid.")),
    };
    match usecases.user_usecase.get_by_id(id).await? {
        None => Err(AppError::NotFound("User is not found.")),
        Some(user) => Ok(Json(user)),
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::model::user_model::Model as User;
    use crate::domain::user_domain::MockUserUsecase;
    use crate::test::{
        fixture::user_fixture::user_fixture,
        mock::{handler_mock::request_mock, usecase_mock::create_usecases_for_mock},
    };
    use crate::usecase::Usecases;
    use anyhow::Error;
    use axum::{body::Body, http::StatusCode};
    use std::sync::Arc;

    #[tokio::test]
    async fn get_by_id_test() {
        let usecases = _usecases_for_get_by_id_test();
        let url = "/users/1";
        let response = request_mock(url, Body::empty(), usecases.clone()).await;
        assert_eq!(response.status(), StatusCode::OK);

        let usecases = _usecases_for_get_by_id_test();
        let url = "/users/a";
        let response = request_mock(url, Body::empty(), usecases.clone()).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let usecases = _usecases_for_get_by_id_test();
        let url = "/users/100";
        let response = request_mock(url, Body::empty(), usecases).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    fn _usecases_for_get_by_id_test() -> Arc<Usecases> {
        let mut usecases = create_usecases_for_mock();
        let mut mock_user_usecase = MockUserUsecase::new();
        let res: Result<Option<User>, Error> = Ok(Some(user_fixture()));
        mock_user_usecase
            .expect_get_by_id()
            .return_once(move |x| if x == 1 { res } else { Ok(None) });
        usecases.user_usecase = Box::new(mock_user_usecase);
        Arc::new(usecases)
    }
}
