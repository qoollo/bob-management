use super::prelude::*;

pub type HttpClient = ContextWrapper<
    Client<
        DropContextService<hyper::Client<HttpConnector, Body>, ClientContext>,
        ClientContext,
        Basic,
    >,
    ClientContext,
>;
pub type HttpBobClient = BobClient<ClientContext, HttpClient>;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Couldn't load user")]
    LoadError,
    #[error("Couldn't lock user store")]
    PoisonError,
    #[error("Auth layer failed to deserialize data")]
    DeserializeError,
    #[error("Couldn't login user")]
    LoginError,
    #[error("Couldn't logout user")]
    LogoutError,
    #[error("Couldn't update session")]
    SessionError,
}

/// Optional credentials for a BOB cluster
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

/// Login to a BOB cluster
///
/// # Errors
/// This function can return the following errors
///
/// 1. [`StatusCode::BAD_REQUEST`]
/// The function failed to parse hostname of the request
///
/// 2. [`StatusCode::NOT_FOUND`]
/// The client was unable to reach the host
///
/// 3. [`StatusCode::UNAUTHORIZED`]
/// The client couldn't authorize on the host
///
#[cfg_attr(all(feature = "swagger", debug_assertions),
    utoipa::path(
        post,
        context_path = ApiV1::to_path(),
        path = "/login",
        params(
            BobConnectionData
        ),
        responses(
            (status = 200, description = "Successful authorization"),
            (status = 400, description = "Bad Hostname"),
            (status = 401, description = "Bad Credentials"),
            (status = 404, description = "Can't reach specified hostname")
        )
))]
#[tracing::instrument(ret, skip(auth), level = "info", fields(method = "POST"))]
pub async fn login(
    mut auth: BobAuth<HttpBobClient>,
    Extension(request_timeout): Extension<RequestTimeout>,
    Json(bob): Json<BobConnectionData>,
) -> AxumResult<StatusCode> {
    let bob_client = BobClient::<HttpClient>::try_new(bob.clone(), request_timeout)
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            match err.current_context() {
                ClientError::InitClient => StatusCode::BAD_REQUEST,
                ClientError::Inaccessible => StatusCode::NOT_FOUND,
                ClientError::PermissionDenied => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;
    let res = match bob_client.probe_main().await {
        Ok(res) => res,
        Err(err) => {
            tracing::error!("{err:?}");
            return Err(StatusCode::UNAUTHORIZED.into());
        }
    };

    if res == StatusCode::OK {
        auth.user_store
            .save(
                *bob_client.id(),
                BobUser {
                    login: if let Some(creds) = bob.credentials {
                        creds.login
                    } else {
                        "Unknown".to_string()
                    },
                },
            )
            .await
            .map_err(|err| {
                tracing::error!("{err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        auth.login(bob_client.id()).await.map_err(|err| {
            tracing::error!("{err:?}");
            StatusCode::UNAUTHORIZED
        })?;
        auth.client_store.insert(*bob_client.id(), bob_client);
    }

    Ok(res)
}

#[async_trait]
pub trait Store<Id, Value> {
    type Error: std::error::Error + Context;

    /// Load `Value` from abstract store
    ///
    /// # Errors
    ///
    /// This function will return an error if a `Value` couldn't be loaded
    async fn load(&self, user_id: &Id) -> Result<Option<Value>, Self::Error>;

    /// Save `Value` into abstract store
    /// Returns old data, if there was any
    ///
    /// # Errors
    ///
    /// This function will return an error if a `Value` couldn't be saved
    async fn save(&mut self, user_id: Id, value: Value) -> Result<Option<Value>, Self::Error>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthData<User, UserId> {
    user: Option<User>,
    user_id: Option<UserId>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BobUser {
    login: String,
}

#[derive(Debug, Clone)]
pub struct AuthState<User, Id, UserStore, ClientStore> {
    user_store: UserStore,
    client_store: ClientStore,
    _user: PhantomData<User>,
    _id: PhantomData<Id>,
}

impl<User, Id, UserStore, ClientStore> AuthState<User, Id, UserStore, ClientStore> {
    pub const fn new(user_store: UserStore, client_store: ClientStore) -> Self {
        Self {
            user_store,
            _user: PhantomData,
            client_store,
            _id: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthStore<User, Id, Client, ClientStore, SessionStore>
where
    SessionStore: Store<Id, User>,
{
    session: Session,
    auth_data: AuthData<User, Id>,
    user_store: SessionStore,
    client_store: ClientStore,
    _client: PhantomData<Client>,
}

impl<User, Id, Client, ClientStore, SessionStore>
    AuthStore<User, Id, Client, ClientStore, SessionStore>
where
    User: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
    Id: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send,
    SessionStore: Store<Id, User> + Sync + Send,
    ClientStore: Store<Id, Client> + Sync + Send,
    Client: Send,
{
    async fn login(&mut self, user_id: &Id) -> Result<(), AuthError> {
        if let Some(user) = self
            .user_store
            .load(user_id)
            .await
            .change_context(AuthError::LoginError)?
        {
            self.auth_data.user = Some(user);
            self.auth_data.user_id = Some(user_id.clone());
        }

        self.update_session().change_context(AuthError::LoginError)
    }

    fn logout(&mut self) -> Result<(), AuthError> {
        self.auth_data = AuthData {
            user: None,
            user_id: None,
        };

        self.update_session().change_context(AuthError::LogoutError)
    }

    const fn user(&self) -> Option<&User> {
        self.auth_data.user.as_ref()
    }
}

impl<User, Id, Client, ClientStore, SessionStore>
    AuthStore<User, Id, Client, ClientStore, SessionStore>
where
    User: Clone + Serialize + for<'a> Deserialize<'a>,
    Id: Clone + Serialize + for<'a> Deserialize<'a>,
    SessionStore: Store<Id, User>,
    ClientStore: Store<Id, Client>,
{
    const AUTH_DATA_KEY: &'static str = "_auth_data";
    /// Update session of this [`AuthStore<User, Id, Client, SessionStore>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if `auth_data` couldn't be serialized.
    fn update_session(&self) -> Result<(), AuthError> {
        self.session
            .insert(Self::AUTH_DATA_KEY, self.auth_data.clone())
            .change_context(AuthError::SessionError)
    }
}

// NOTE: async_trait is used in `FromRequestParts` declaration, so we still need to use it here
#[async_trait]
impl<S, User, Id, Client, ClientStore, UserStore> FromRequestParts<S>
    for AuthStore<User, Id, Client, ClientStore, UserStore>
where
    S: Send + Sync,
    User: Serialize + for<'a> Deserialize<'a> + Clone + Send,
    Id: Serialize + for<'a> Deserialize<'a> + Clone + Send + Sync,
    UserStore: Store<Id, User> + Send + Sync,
    ClientStore: Store<Id, Client> + Send + Sync,
    AuthState<User, Id, UserStore, ClientStore>: FromRef<S>,
    Client: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;

        let mut auth_data: AuthData<User, Id> = session
            .get(Self::AUTH_DATA_KEY)
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "Auth layer failed to deserialize data",
                )
            })?
            .unwrap_or(AuthData {
                user: None,
                user_id: None,
            });

        let AuthState {
            user_store,
            client_store,
            ..
        } = AuthState::from_ref(state);

        // Poll store to refresh current user.
        if let Some(ref user_id) = auth_data.user_id {
            match user_store.load(user_id).await {
                Ok(user) => auth_data.user = user,

                Err(_) => return Err((StatusCode::BAD_REQUEST, "Could not load from user store")),
            }
        };

        Ok(Self {
            session,
            auth_data,
            user_store,
            client_store,
            _client: PhantomData,
        })
    }
}

/// Handles protected routes
///
/// # Errors
///
/// This function will return an error if a protected route was called from unauthorized context
pub async fn require_auth<User, Id, Client, ClientStore, UserStore, Body>(
    auth: AuthStore<User, Id, Client, ClientStore, UserStore>,
    mut request: Request<Body>,
    next: Next<Body>,
) -> std::result::Result<Response, StatusCode>
where
    User: Serialize + for<'a> Deserialize<'a> + Clone + Send + Sync,
    Id: Serialize + for<'a> Deserialize<'a> + Clone + Send + Sync + Hash + Eq,
    UserStore: Store<Id, User> + Send + Sync,
    ClientStore: Store<Id, Client> + Send + Sync,
    Client: Send + Sync + Clone + 'static,
    Body: Send + Sync,
{
    if let Some(id) = &auth.auth_data.user_id {
        request.extensions_mut().insert(
            auth.client_store
                .load(id)
                .await
                .map_err(|err| {
                    tracing::error!("{err:?}");
                    StatusCode::UNAUTHORIZED
                })?
                .ok_or(StatusCode::UNAUTHORIZED)?,
        );
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Debug, Clone)]
pub struct InMemorySessionStore<Id, Value>(Arc<Mutex<HashMap<Id, Value>>>);

impl<Id, Value> Default for InMemorySessionStore<Id, Value> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(HashMap::default())))
    }
}

#[async_trait]
impl<ItemId, Item> Store<ItemId, Item> for InMemorySessionStore<ItemId, Item>
where
    Item: Send + Sync + Clone,
    ItemId: Sync + Hash + Eq + Send,
{
    type Error = AuthError;

    async fn load(&self, item_id: &ItemId) -> Result<Option<Item>, Self::Error> {
        Ok(self.0.lock().await.get(item_id).cloned())
    }

    async fn save(&mut self, user_id: ItemId, value: Item) -> Result<Option<Item>, Self::Error> {
        Ok(self.0.lock().await.insert(user_id, value))
    }
}

pub type BobAuth<Client> = AuthStore<
    BobUser,
    Uuid,
    Client,
    InMemorySessionStore<Uuid, Client>,
    InMemorySessionStore<Uuid, BobUser>,
>;

#[cfg_attr(all(feature = "swagger", debug_assertions),
    utoipa::path(
        post,
        context_path = ApiV1::to_path(),
        path = "/logout",
        responses(
            (status = 200, description = "Logged out")
        )
    ))]
#[tracing::instrument(ret, skip(auth), level = "info", fields(method = "POST"))]
pub async fn logout(mut auth: BobAuth<HttpBobClient>) -> impl IntoResponse {
    tracing::info!("post /logout : {:?}", &auth.auth_data);
    auth.logout().map_or_else(
        |_| StatusCode::OK.into_response(),
        axum::response::IntoResponse::into_response,
    )
}
