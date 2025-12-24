use askama::Template;

#[derive(PartialEq)]
pub enum NavItem {
    Home,
    Create,
    Todos,
    Login,
    Signup,
}

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
pub struct TodosTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/create.html")]
pub struct CreateTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
pub struct LoginTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str,
    pub is_authenticated: bool,
}

#[derive(Template)]
#[template(path = "pages/sign-up.html")]
pub struct SignupTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str,
    pub is_authenticated: bool,
    pub flash: String,
}

#[derive(Template)]
#[template(path = "pages/server-error.html")]
pub struct ServerErrorTemplate<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "pages/not-found.html")]
pub struct PageNotFoundTemplate<'a> {
    pub title: &'a str,
}
