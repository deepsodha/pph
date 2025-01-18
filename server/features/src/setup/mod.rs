pub mod dbsetup;
pub mod dev_setup;
pub mod prod_setup;
pub mod set_up;
pub mod stag_setup;
pub mod test_setup;
pub mod prelude {
    pub use crate::setup::dbsetup::*;
    pub use crate::setup::dev_setup::*;
    pub use crate::setup::prod_setup::*;
    pub use crate::setup::set_up::*;
    pub use crate::setup::stag_setup::*;
    pub use crate::setup::test_setup::*;
}
