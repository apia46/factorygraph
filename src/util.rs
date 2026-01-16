use super::*;
use std::{ops::{Add, Sub, Neg, Mul, Div}};

pub const WRAPPER_SCREEN_POSITION:Point<i32> = Point::new(0, 52);

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x:T, y:T) -> Point<T> {
        Point::<T> {
            x,
            y,
        }
    }
}

impl From<Point<i32>> for Point<f64> {
    fn from(value: Point<i32>) -> Self {
        Point::new(value.x as f64, value.y as f64)
    }
}
impl From<Point<f64>> for Point<i32> {
    fn from(value: Point<f64>) -> Self {
        Point::new(value.x as i32, value.y as i32)
    }
}


// Notice that the implementation uses the associated type `Output`.
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: Neg<Output = T>> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl<T: Mul<Output = T>> Mul<T> for Point<T>
where T: Copy {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl<T: Div<Output = T>> Div<T> for Point<T>
where T: Copy {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

pub trait MoveTowardExp {
    fn move_toward_exp(self:Self, target:Self, amount:Self) -> Self;
}
impl MoveTowardExp for f64 {
    fn move_toward_exp(self:Self, target:Self, amount:Self) -> Self {
        const EPSILON:f64 = 0.01;
        if (target - self).abs() < EPSILON { target }
        else {self + (target - self) * amount}
    }
}

pub trait ElementExt {
    fn as_html_element(self:&Self) -> &HtmlElement;
    fn query_selector_expect(self:&Self, query:&str) -> Element;
    fn create_child(self:&Self, name:&str) -> Element;
    fn with_attr(self:Self, attr:&str, value:&str) -> Element;
    fn with_class(self:Self, class:&str) -> Element;
    fn with_text_content(self:Self, content:&str) -> Element;
}
impl ElementExt for Element {
    fn as_html_element(self:&Self) -> &HtmlElement {
        self.dyn_ref::<HtmlElement>().unwrap()
    }

    fn query_selector_expect(self:&Self, query:&str) -> Element {
        self.query_selector(query).unwrap().unwrap()
    }

    fn create_child(self:&Self, name:&str) -> Element {
        let element = document().create_element(name).unwrap();
        _ = self.append_child(&element);
        element
    }

    fn with_attr(self:Self, attr:&str, value:&str) -> Element {
        _ = self.set_attribute(attr, value);
        self
    }

    fn with_class(self:Self, class:&str) -> Element {
        _ = self.class_list().add_1(class);
        self
    }

    fn with_text_content(self:Self, content:&str) -> Element {
        _ = self.set_text_content(Some(content));
        self
    }
}

pub fn get_wrapper() -> Element {
    document().get_element_by_id("wrapper").unwrap()
}

pub fn screen_to_world(point:Point<i32>, state:&state::State) -> Point<f64> {
    (Point::<f64>::from(point - WRAPPER_SCREEN_POSITION) - state::graph::get_position(state))/state::graph::get_scale(state)
}

pub fn world_to_screen(point:Point<f64>, state:&state::State) -> Point<i32> {
    Point::<i32>::from(point * state::graph::get_scale(state) + state::graph::get_position(state)) + WRAPPER_SCREEN_POSITION
}
