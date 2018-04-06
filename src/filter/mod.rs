use ::reply::{NOT_FOUND, NotFound, Reply};
use ::route::Route;
use ::server::WarpService;

mod and;
mod map;
mod or;
mod service;

pub use self::and::{And, AndUnit, UnitAnd};
pub use self::map::Map;
pub use self::or::{Either, Or};
pub use self::service::FilteredService;

pub trait Filter {

    type Extract;

    fn filter<'a>(&self, input: Route<'a>) -> Option<(Route<'a>, Self::Extract)>;

    fn and<F>(self, other: F) -> And<Self, F>
    where
        Self: FilterAnd + Sized,
        F: Filter,
    {
        And {
            first: self,
            second: other,
        }
    }

    fn unit_and<F>(self, other: F) -> UnitAnd<Self, F>
    where
        Self: Filter<Extract=()> + FilterAnd + Sized,
        F: Filter,
    {
        UnitAnd {
            first: self,
            second: other,
        }
    }

    fn and_unit<F>(self, other: F) -> AndUnit<Self, F>
    where
        Self: FilterAnd + Sized,
        F: Filter<Extract=()>,
    {
        AndUnit {
            first: self,
            second: other,
        }
    }

    fn or<F>(self, other: F) -> Or<Self, F>
    where
        Self: Sized,
        F: Filter,
    {
        Or {
            first: self,
            second: other,
        }
    }

    fn map<F, U>(self, fun: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Extract) -> U,
    {
        Map {
            filter: self,
            callback: fun,
        }
    }

    fn service(self) -> FilteredService<Self, NotFound>
    where
        Self: Sized,
        Self::Extract: Reply,
    {
        self.service_with_not_found(NOT_FOUND)
    }

    fn service_with_not_found<N>(self, not_found: N) -> FilteredService<Self, N>
    where
        Self: Sized,
        Self::Extract: Reply,
        N: WarpService,
    {
        FilteredService {
            filter: self,
            not_found,
        }
    }
}

pub trait FilterAnd: Filter {}