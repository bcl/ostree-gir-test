use analysis::parameter::Parameter;
use env::Env;
use library;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SafetyAssertionMode {
    None,
    Skip,
    InMainThread,
}

impl Default for SafetyAssertionMode {
    fn default() -> SafetyAssertionMode {
        SafetyAssertionMode::None
    }
}

impl SafetyAssertionMode {
    pub fn of(env: &Env, params: &[Parameter]) -> SafetyAssertionMode {
        use self::SafetyAssertionMode::*;
        use library::Type::*;
        if !env.config.generate_safety_asserts {
            return None;
        }
        if !params.is_empty() && params[0].instance_parameter {
            return None;
        }
        for par in params {
            match *env.library.type_(par.typ) {
                Class(..) | Interface(..) if !*par.nullable && par.typ.ns_id == library::MAIN_NAMESPACE
                    => return Skip,
                _ => (),
            }
        }

        InMainThread
    }
}
