use crate::Select;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSelector {
    /// All but expensed months
    All,
    Vendor,
    Client,
    Information,
    PaymentInfo,
    ServiceFees,
}

impl Select for DataSelector {
    fn includes(&self, target: Self) -> bool {
        match self {
            DataSelector::All => true,
            DataSelector::Vendor => matches!(target, DataSelector::Vendor),
            DataSelector::Client => matches!(target, DataSelector::Client),
            DataSelector::Information => matches!(target, DataSelector::Information),
            DataSelector::PaymentInfo => matches!(target, DataSelector::PaymentInfo),
            DataSelector::ServiceFees => matches!(target, DataSelector::ServiceFees),
        }
    }
}
