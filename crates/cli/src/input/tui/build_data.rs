use crate::{
    Data, DataSelector, PeriodAnno, Result, build_company, build_invoice_info, build_payment_info,
    build_service_fees, config_render, curry2, select_or_default,
};

pub fn ask_for_data(
    default: Data<PeriodAnno>,
    data_selector: Option<DataSelector>,
) -> Result<Data<PeriodAnno>> {
    config_render();

    let vendor = select_or_default(data_selector, DataSelector::Vendor, default.vendor(), |d| {
        build_company("Your company", d)
    })?;

    let client = select_or_default(data_selector, DataSelector::Client, default.client(), |d| {
        build_company("Your client", d)
    })?;

    let service_fees = select_or_default(
        data_selector,
        DataSelector::ServiceFees,
        default.service_fees(),
        build_service_fees,
    )?;

    let invoice_info = select_or_default(
        data_selector,
        DataSelector::Information,
        default.information(),
        curry2(build_invoice_info, *service_fees.cadence()),
    )?;

    let payment_info = select_or_default(
        data_selector,
        DataSelector::PaymentInfo,
        default.payment_info(),
        build_payment_info,
    )?;

    let data = Data::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .service_fees(service_fees)
        .information(invoice_info)
        .expensed_periods(default.expensed_periods().clone())
        .build();

    Ok(data)
}
