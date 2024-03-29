//! Macros for navigating the "Woog" domain
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  sarzak gen -d woog sarzak -d true -m true -i true -e false
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"1.0.0"}

/// Macro to traverse [`ObjectMethod`][🦀] ➡ [`Visibility`][🦞], via _R7_
///
/// This macro expects a &[`ObjectMethod`][🦀], and returns a &[`Visibility`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::ObjectMethod
/// [🦞]: crate::woog::types::Visibility
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::sarzak::Object;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::woog::ObjectMethod;
/// # use sarzak::woog_get_one_viz_across_r7;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_zjt = Mutability::test_default(&mut store);
/// let type_qoh = Type::test_default(&mut sarzak_store);
///
/// let visibility_eys = Visibility::test_default(&mut store);
/// let ill_fated_clam = "important_collar".to_owned();
/// let parameter = Parameter::new(&mut store, &mutability_zjt, None, &type_qoh, &visibility_eys, ill_fated_clam);
/// let roasted_business = "arrogant_interest".to_owned();
/// let drunk_parcel = "barbarous_morning".to_owned();
/// let complete_brake = "alleged_ship".to_owned();
/// let object_pya = Object::default();
///
/// let type_jhp = Type::test_default(&mut sarzak_store);
///
/// let visibility_tnq = Visibility::test_default(&mut store);
/// let old_things = "parallel_yam".to_owned();
/// let chunky_paper = "simple_shoes".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_pya, &type_jhp, &visibility_tnq, old_things, chunky_paper);
///
/// let visibility_kpx = woog_get_one_viz_across_r7!(object_method, store);
/// assert_eq!(&visibility_tnq, visibility_kpx);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_viz_across_r7-emit_binary_main"}}}
macro_rules! woog_get_one_viz_across_r7 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-emit_one_unconditional"}}}
        // nut::codegen::template::macros::emit_one_unconditional
        $store.exhume_visibility(&$input.visibility).unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-emit_one_unconditional"}}}
    }};
}
pub use woog_get_one_viz_across_r7;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_viz_across_r7-emit_binary_main"}}}

/// Macro to traverse [`Visibility`][🦀] ➡ [`ObjectMethod`][🦞], via _R7_
/// Macro to traverse [`Visibility`][🦀] ➡ [`ObjectMethod`][🦞], via _R7(c)_
///
/// This macro expects a &[`Visibility`][🦀], and returns a &[`ObjectMethod`][🦞].
/// This macro expects a &[`Visibility`][🦀], and returns an Option<&[`ObjectMethod`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Visibility
/// [🦞]: crate::woog::types::ObjectMethod
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::woog::ObjectMethod;
/// # use sarzak::v1::sarzak::Object;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::woog_maybe_get_one_meth_across_r7;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_rma = Mutability::test_default(&mut store);
/// let type_cxy = Type::test_default(&mut sarzak_store);
///
/// let visibility_dbf = Visibility::test_default(&mut store);
/// let puny_yak = "physical_feet".to_owned();
/// let parameter = Parameter::new(&mut store, &mutability_rma, None, &type_cxy, &visibility_dbf, puny_yak);
/// let harmonious_smell = "squalid_table".to_owned();
/// let wise_grandfather = "helpless_feeling".to_owned();
/// let finicky_thread = "cheap_flavor".to_owned();
/// let object_rjv = Object::default();
///
/// let type_nzp = Type::test_default(&mut sarzak_store);
///
/// let visibility_bdh = Visibility::test_default(&mut store);
/// let kind_balls = "pleasant_system".to_owned();
/// let selective_friend = "jolly_sound".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_rjv, &type_nzp, &visibility_bdh, kind_balls, selective_friend);
/// let object_method_stu = woog_maybe_get_one_meth_across_r7!(visibility_bdh, store);
///
/// assert_eq!(Some(&object_method), object_method_stu);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_maybe_get_one_meth_across_r7-emit_binary_main"}}}
macro_rules! woog_maybe_get_one_meth_across_r7 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-object_method-emit_one_conditional_lookup"}}}
        // nut::codegen::template::macros::emit_one_conditional_lookup
        $store
            .iter_object_method()
            .find(|z| z.1.visibility == $input.get_id())
            .map(|(_, z)| z)
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-object_method-emit_one_conditional_lookup"}}}
    }};
}
pub use woog_maybe_get_one_meth_across_r7;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_maybe_get_one_meth_across_r7-emit_binary_main"}}}

/// Macro to traverse [`ObjectMethod`][🦀] ➡ [`Parameter`][🦞], via _R5(c)_
///
/// This macro expects a &[`ObjectMethod`][🦀], and returns an Option<&[`Parameter`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::ObjectMethod
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::sarzak::Object;
/// # use sarzak::v1::woog::ObjectMethod;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::woog_maybe_get_one_param_across_r5;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_mtd = Mutability::test_default(&mut store);
/// let type_ytl = Type::test_default(&mut sarzak_store);
///
/// let visibility_qvf = Visibility::test_default(&mut store);
/// let acid_sweater = "sour_wound".to_owned();
/// let parameter = Parameter::new(&mut store, &mutability_mtd, None, &type_ytl, &visibility_qvf, acid_sweater);
/// let left_route = "unwieldy_anger".to_owned();
/// let irate_square = "slow_attempt".to_owned();
/// let shy_side = "well_made_bears".to_owned();
/// let object_aux = Object::default();
///
/// let type_pgw = Type::test_default(&mut sarzak_store);
///
/// let visibility_ayx = Visibility::test_default(&mut store);
/// let zonked_protest = "nebulous_vest".to_owned();
/// let eminent_tooth = "judicious_town".to_owned();
/// let mutability_pwa = Mutability::test_default(&mut store);
/// let type_qxk = Type::test_default(&mut sarzak_store);
///
/// let visibility_mdv = Visibility::test_default(&mut store);
/// let needy_acoustics = "lean_trail".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_aux, &type_pgw, &visibility_ayx, zonked_protest, eminent_tooth);
///
/// let parameter_jhm = woog_maybe_get_one_param_across_r5!(object_method, store);
/// assert_eq!(Some(&parameter), parameter_jhm);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_maybe_get_one_param_across_r5-emit_binary_main"}}}
macro_rules! woog_maybe_get_one_param_across_r5 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-emit_one_conditional"}}}
        // nut::codegen::template::macros::emit_one_conditional
        match &$input.param {
            Some(i) => $store.exhume_parameter(i),
            None => None,
        }
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-emit_one_conditional"}}}
    }};
}
pub use woog_maybe_get_one_param_across_r5;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_maybe_get_one_param_across_r5-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`ObjectMethod`][🦞], via _R5_
///
/// This macro expects a &[`Parameter`][🦀], and returns a &[`ObjectMethod`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::ObjectMethod
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::woog::ObjectMethod;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::sarzak::Object;
/// # use sarzak::woog_get_one_meth_across_r5;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_zsr = Mutability::test_default(&mut store);
/// let type_pot = Type::test_default(&mut sarzak_store);
///
/// let visibility_ysi = Visibility::test_default(&mut store);
/// let incandescent_horn = "eatable_regret".to_owned();
/// let mutability_vtu = Mutability::test_default(&mut store);
/// let type_abb = Type::test_default(&mut sarzak_store);
///
/// let visibility_wll = Visibility::test_default(&mut store);
/// let tearful_pail = "efficacious_touch".to_owned();
/// let parameter = Parameter::new(&mut store, &mutability_vtu, None, &type_abb, &visibility_wll, tearful_pail);
/// let invincible_wheel = "spooky_nerve".to_owned();
/// let hurt_place = "wary_birds".to_owned();
/// let fascinated_owl = "needless_north".to_owned();
/// let object_opk = Object::default();
///
/// let type_irk = Type::test_default(&mut sarzak_store);
///
/// let visibility_fmw = Visibility::test_default(&mut store);
/// let new_grain = "minor_icicle".to_owned();
/// let raspy_wilderness = "jumpy_wound".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_opk, &type_irk, &visibility_fmw, new_grain, raspy_wilderness);
///
/// let object_method_vwc = woog_get_one_meth_across_r5!(parameter, store);
/// assert_eq!(&object_method, object_method_vwc);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_meth_across_r5-emit_binary_main"}}}
macro_rules! woog_get_one_meth_across_r5 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-object_method-emit_one_unconditional_lookup", "is_uber":true}}}}
        // nut::codegen::template::macros::emit_one_unconditional_lookup
        $store
            .iter_object_method()
            .find(|z| z.1.param == Some($input.id))
            .map(|z| z.1)
            .unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-object_method-emit_one_unconditional_lookup"}}}
    }};
}
pub use woog_get_one_meth_across_r5;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_meth_across_r5-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`Parameter`][🦞], via _R1(c)_
///
/// This macro expects a &[`Parameter`][🦀], and returns an Option<&[`Parameter`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::woog_maybe_get_one_param_across_r1;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_pyn = Mutability::test_default(&mut store);
/// let type_wzg = Type::test_default(&mut sarzak_store);
///
/// let visibility_tka = Visibility::test_default(&mut store);
/// let witty_observation = "great_stem".to_owned();
/// let mutability_psf = Mutability::test_default(&mut store);
/// let type_uzj = Type::test_default(&mut sarzak_store);
///
/// let visibility_ean = Visibility::test_default(&mut store);
/// let crowded_visitor = "healthy_swing".to_owned();
///
/// let parameter = Parameter::new(&mut store, &mutability_pyn, None, &type_wzg, &visibility_tka, witty_observation);
/// let parameter_0 = Parameter::new(&mut store, &mutability_pyn, Some(&parameter), &type_wzg, &visibility_tka, "witty_observation".to_owned());
///
/// let parameter_bhv = woog_maybe_get_one_param_across_r1!(parameter_0, store);
/// assert_eq!(Some(&parameter), parameter_bhv);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_maybe_get_one_param_across_r1-emit_binary_main"}}}
macro_rules! woog_maybe_get_one_param_across_r1 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-emit_one_conditional"}}}
        // nut::codegen::template::macros::emit_one_conditional
        match &$input.next {
            Some(i) => $store.exhume_parameter(i),
            None => None,
        }
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-emit_one_conditional"}}}
    }};
}
pub use woog_maybe_get_one_param_across_r1;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_maybe_get_one_param_across_r1-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`Parameter`][🦞], via _R1_
///
/// This macro expects a &[`Parameter`][🦀], and returns a &[`Parameter`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::woog_get_one_param_across_r1;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_mfe = Mutability::test_default(&mut store);
/// let type_dag = Type::test_default(&mut sarzak_store);
///
/// let visibility_kez = Visibility::test_default(&mut store);
/// let sweltering_stop = "spiffy_sweater".to_owned();
/// let mutability_buy = Mutability::test_default(&mut store);
/// let type_kol = Type::test_default(&mut sarzak_store);
///
/// let visibility_sin = Visibility::test_default(&mut store);
/// let important_downtown = "stormy_selection".to_owned();
///
/// let parameter = Parameter::new(&mut store, &mutability_buy, None, &type_kol, &visibility_sin, important_downtown);
/// let parameter_0 = Parameter::new(&mut store, &mutability_buy, Some(&parameter), &type_kol, &visibility_sin, "cut_farm".to_owned());
///
/// let parameter_mwo = woog_get_one_param_across_r1!(parameter, store);
/// assert_eq!(&parameter_0, parameter_mwo);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_param_across_r1-emit_binary_main"}}}
macro_rules! woog_get_one_param_across_r1 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-parameter-emit_one_unconditional_lookup", "is_uber":true}}}}
        // nut::codegen::template::macros::emit_one_unconditional_lookup
        $store
            .iter_parameter()
            .find(|z| z.1.next == Some($input.id))
            .map(|z| z.1)
            .unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-parameter-emit_one_unconditional_lookup"}}}
    }};
}
pub use woog_get_one_param_across_r1;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_param_across_r1-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`Visibility`][🦞], via _R8_
///
/// This macro expects a &[`Parameter`][🦀], and returns a &[`Visibility`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::Visibility
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::woog_get_one_viz_across_r8;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_llv = Mutability::test_default(&mut store);
/// let type_ety = Type::test_default(&mut sarzak_store);
///
/// let visibility_ayu = Visibility::test_default(&mut store);
/// let handsome_quince = "little_faucet".to_owned();
///
/// let parameter = Parameter::new(&mut store, &mutability_llv, None, &type_ety, &visibility_ayu, handsome_quince);
///
/// let visibility_frb = woog_get_one_viz_across_r8!(parameter, store);
/// assert_eq!(&visibility_ayu, visibility_frb);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_viz_across_r8-emit_binary_main"}}}
macro_rules! woog_get_one_viz_across_r8 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-emit_one_unconditional"}}}
        // nut::codegen::template::macros::emit_one_unconditional
        $store.exhume_visibility(&$input.visibility).unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-emit_one_unconditional"}}}
    }};
}
pub use woog_get_one_viz_across_r8;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_viz_across_r8-emit_binary_main"}}}

/// Macro to traverse [`Visibility`][🦀] ➡ [`Parameter`][🦞], via _R8_
/// Macro to traverse [`Visibility`][🦀] ➡ [`Parameter`][🦞], via _R8(c)_
///
/// This macro expects a &[`Visibility`][🦀], and returns a &[`Parameter`][🦞].
/// This macro expects a &[`Visibility`][🦀], and returns an Option<&[`Parameter`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Visibility
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::woog_maybe_get_one_param_across_r8;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_kwh = Mutability::test_default(&mut store);
/// let type_kiq = Type::test_default(&mut sarzak_store);
///
/// let visibility_fhm = Visibility::test_default(&mut store);
/// let rotten_jewel = "combative_achiever".to_owned();
///
/// let parameter = Parameter::new(&mut store, &mutability_kwh, None, &type_kiq, &visibility_fhm, rotten_jewel);
/// let parameter_oyb = woog_maybe_get_one_param_across_r8!(visibility_fhm, store);
///
/// assert_eq!(Some(&parameter), parameter_oyb);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_maybe_get_one_param_across_r8-emit_binary_main"}}}
macro_rules! woog_maybe_get_one_param_across_r8 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-parameter-emit_one_conditional_lookup"}}}
        // nut::codegen::template::macros::emit_one_conditional_lookup
        $store
            .iter_parameter()
            .find(|z| z.1.visibility == $input.get_id())
            .map(|(_, z)| z)
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-parameter-emit_one_conditional_lookup"}}}
    }};
}
pub use woog_maybe_get_one_param_across_r8;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_maybe_get_one_param_across_r8-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`Mutability`][🦞], via _R10_
///
/// This macro expects a &[`Parameter`][🦀], and returns a &[`Mutability`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::Mutability
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::woog_get_one_mut_across_r10;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_fmr = Mutability::test_default(&mut store);
/// let type_sra = Type::test_default(&mut sarzak_store);
///
/// let visibility_rze = Visibility::test_default(&mut store);
/// let somber_stretch = "roasted_texture".to_owned();
///
/// let parameter = Parameter::new(&mut store, &mutability_fmr, None, &type_sra, &visibility_rze, somber_stretch);
///
/// let mutability_kmy = woog_get_one_mut_across_r10!(parameter, store);
/// assert_eq!(&mutability_fmr, mutability_kmy);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_mut_across_r10-emit_binary_main"}}}
macro_rules! woog_get_one_mut_across_r10 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"mutability-emit_one_unconditional"}}}
        // nut::codegen::template::macros::emit_one_unconditional
        $store.exhume_mutability(&$input.mutability).unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"mutability-emit_one_unconditional"}}}
    }};
}
pub use woog_get_one_mut_across_r10;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_mut_across_r10-emit_binary_main"}}}

/// Macro to traverse [`Mutability`][🦀] ➡ [`Parameter`][🦞], via _R10(c)_
///
/// This macro expects a &[`Mutability`][🦀], and returns an Option<&[`Parameter`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Mutability
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::v1::sarzak::Type;
/// # use sarzak::v1::woog::Parameter;
/// # use sarzak::v1::woog::Mutability;
/// # use sarzak::v1::woog::Visibility;
/// # use sarzak::woog_maybe_get_one_param_across_r10;
/// # let mut store = sarzak::v1::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::v1::sarzak::ObjectStore::new();
///
/// let mutability_eny = Mutability::test_default(&mut store);
/// let type_zml = Type::test_default(&mut sarzak_store);
///
/// let visibility_kmc = Visibility::test_default(&mut store);
/// let cheerful_stocking = "lame_debt".to_owned();
///
/// let parameter = Parameter::new(&mut store, &mutability_eny, None, &type_zml, &visibility_kmc, cheerful_stocking);
/// let parameter_bsh = woog_maybe_get_one_param_across_r10!(mutability_eny, store);
///
/// assert_eq!(Some(&parameter), parameter_bsh);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_maybe_get_one_param_across_r10-emit_binary_main"}}}
macro_rules! woog_maybe_get_one_param_across_r10 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"mutability-parameter-emit_one_conditional_lookup"}}}
        // nut::codegen::template::macros::emit_one_conditional_lookup
        $store
            .iter_parameter()
            .find(|z| z.1.mutability == $input.get_id())
            .map(|(_, z)| z)
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"mutability-parameter-emit_one_conditional_lookup"}}}
    }};
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_param_across_r8-emit_binary_main"}}}
pub use woog_maybe_get_one_param_across_r10;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_maybe_get_one_param_across_r10-emit_binary_main"}}}
