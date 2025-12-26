use crate::data::todo::Todo;
use crate::models::app::FlashData;
use askama::Template;
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum NavItem {
    Home,
    Create,
    Todos,
    Login,
    Signup,
}

impl fmt::Display for NavItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NavItem::Home => "home",
            NavItem::Create => "create",
            NavItem::Todos => "todos",
            NavItem::Login => "login",
            NavItem::Signup => "signup",
        };
        write!(f, "{s}")
    }
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
    pub flash_data: FlashData,
    pub todos: Vec<Todo>,
    pub current_page_number: i32,
    pub total_pages: i32,
    pub next_page: fn(i32) -> i32,
    pub previous_page: fn(i32) -> i32,
}

impl<'a> TodosTemplate<'a> {
    pub fn previous_page(&self) -> Option<i32> {
        if self.current_page_number > 1 {
            Some(self.current_page_number - 1)
        } else {
            None
        }
    }

    pub fn next_page(&self) -> Option<i32> {
        if self.current_page_number < self.total_pages {
            Some(self.current_page_number + 1)
        } else {
            None
        }
    }
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
    pub flash_data: FlashData,
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
    pub flash_data: FlashData,
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
