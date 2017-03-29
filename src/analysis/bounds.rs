use std::collections::vec_deque::VecDeque;
use std::slice::Iter;
use std::vec::Vec;

use env::Env;
use analysis::imports::Imports;
use analysis::parameter::Parameter;
use analysis::rust_type::bounds_rust_type;
use library::{Function, Fundamental, Nullable, Type, TypeId, ParameterDirection};
use traits::IntoString;

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
pub enum BoundType {
    IsA,
    AsRef,
    //lifetime
    Into(char),
}

#[derive(Debug)]
pub struct Bounds {
    unused: VecDeque<char>,
    //Vector tuples <parameter name>, <alias>, <type>, <bound type>
    used: Vec<(String, char, String, BoundType)>,
    unused_lifetimes: VecDeque<char>,
    lifetimes: Vec<char>,
}

impl Default for Bounds {
    fn default () -> Bounds {
        Bounds {
            unused: "TUVWXYZ".chars().collect(),
            used: Vec::new(),
            unused_lifetimes: "abcdefg".chars().collect(),
            lifetimes: Vec::new(),
        }
    }
}

impl Bounds {
    pub fn add_for_parameter(&mut self, env: &Env, func: &Function, par: &mut Parameter) {
        if !par.instance_parameter && par.direction != ParameterDirection::Out {
            if let Some(bound_type) = Bounds::type_for(env, par.typ, par.nullable) {
                let to_glib_extra = Bounds::to_glib_extra(bound_type);
                par.to_glib_extra = to_glib_extra;
                let type_name = bounds_rust_type(env, par.typ);
                if !self.add_parameter(&par.name, &type_name.into_string(), bound_type) {
                    panic!("Too many type constraints for {}", func.c_identifier.as_ref().unwrap())
                }
            }
        }
    }

    fn type_for(env: &Env, type_id: TypeId, nullable: Nullable) -> Option<BoundType> {
        use self::BoundType::*;
        match *env.library.type_(type_id) {
            Type::Fundamental(Fundamental::Filename) => Some(AsRef),
            Type::Fundamental(Fundamental::Utf8) if *nullable => Some(Into('_')),
            Type::Class(..) => {
                if env.class_hierarchy.subtypes(type_id).next().is_some() {
                    Some(IsA)
                } else {
                    None
                }
            }
            Type::Interface(..) => Some(IsA),
            _ => None,
        }
    }
    fn to_glib_extra(bound_type: BoundType) -> String {
        use self::BoundType::*;
        match bound_type {
            AsRef => ".as_ref()".to_owned(),
            Into(_) => ".into()".to_owned(),
            _ => String::new(),
        }
    }
    pub fn add_parameter(&mut self, name: &str, type_str: &str, mut bound_type: BoundType) -> bool {
        if self.used.iter().any(|n| n.0 == name) { return false; }
        if let BoundType::Into(_) = bound_type {
            if let Some(lifetime) = self.unused_lifetimes.pop_front() {
                self.lifetimes.push(lifetime);
                bound_type = BoundType::Into(lifetime);
            } else {
                return false;
            }
        }
        if let Some(alias) = self.unused.pop_front() {
            self.used.push((name.into(), alias, type_str.into(), bound_type));
            true
        } else {
            false
        }
    }
    pub fn get_parameter_alias_info(&self, name: &str) -> Option<(char, BoundType)> {
        self.used.iter().find(|n| n.0 == name)
            .map(|t| (t.1, t.3))
    }
    pub fn update_imports(&self, imports: &mut Imports) {
        //TODO: import with versions
        use self::BoundType::*;
        for used in &self.used {
            match used.3 {
                IsA => imports.add("glib::object::IsA", None),
                AsRef => imports.add_used_type(&used.2, None),
                Into(_) => {}
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.used.is_empty()
    }
    pub fn iter(&self) ->  Iter<(String, char, String, BoundType)> {
        self.used.iter()
    }
    pub fn iter_lifetimes(&self) -> Iter<char> {
        self.lifetimes.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_new_all() {
        let mut bounds: Bounds = Default::default();
        let typ = BoundType::IsA;
        assert_eq!(bounds.add_parameter("a", "", typ), true);
        // Don't add second time
        assert_eq!(bounds.add_parameter("a", "", typ), false);
        assert_eq!(bounds.add_parameter("b", "", typ), true);
        assert_eq!(bounds.add_parameter("c", "", typ), true);
        assert_eq!(bounds.add_parameter("d", "", typ), true);
        assert_eq!(bounds.add_parameter("e", "", typ), true);
        assert_eq!(bounds.add_parameter("f", "", typ), true);
        assert_eq!(bounds.add_parameter("g", "", typ), true);
        assert_eq!(bounds.add_parameter("h", "", typ), false);
    }

    #[test]
    fn get_parameter_alias_info() {
        let mut bounds: Bounds = Default::default();
        let typ = BoundType::IsA;
        bounds.add_parameter("a", "", typ);
        bounds.add_parameter("b", "", typ);
        assert_eq!(bounds.get_parameter_alias_info("a"), Some(('T', typ)));
        assert_eq!(bounds.get_parameter_alias_info("b"), Some(('U', typ)));
        assert_eq!(bounds.get_parameter_alias_info("c"), None);
    }
}
