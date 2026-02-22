use crate::core::tester::{Interceptor, TesterID};
use crate::testers::SupportedTester;
use origen_metal::prog_gen::SupportedTester as ProgGenSupportedTester;
use super::base::IGXLBase;


/// Teradyne UltraFlex tester implementation.
/// 
/// Inherits IGXL pattern generation from IGXLBase trait.
/// No tester-specific customization currently required.
#[derive(Debug, Clone)]
pub struct UltraFlex {}

impl UltraFlex {
    pub fn default() -> Self {
        Self {}
    }
}

impl std::default::Default for UltraFlex {
    fn default() -> Self {
        Self::default()
    }
}

/// Uses default interceptor behavior (no command interception).
impl Interceptor for UltraFlex {}

/// Provides IGXL pattern generation via blanket impl in base.rs.
impl IGXLBase for UltraFlex {}

impl TesterID for UltraFlex {
    fn id(&self) -> SupportedTester {
        SupportedTester::ULTRAFLEX
    }
    
    fn id_prog_gen(&self) -> ProgGenSupportedTester {
        ProgGenSupportedTester::ULTRAFLEX
    }
}
