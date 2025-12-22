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
}

#[derive(Template)]
#[template(path = "pages/todos.html")]
pub struct TodosTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
}

#[derive(Template)]
#[template(path = "pages/create.html")]
pub struct CreateTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
}

#[derive(Template)]
#[template(path = "pages/log-in.html")]
pub struct LoginTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
}

#[derive(Template)]
#[template(path = "pages/sign-up.html")]
pub struct SignupTemplate<'a> {
    pub title: &'a str,
    pub current_page: NavItem,
}
