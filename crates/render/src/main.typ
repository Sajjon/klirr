// #import "data.typ": provide as provide_data
// #import "l18n.typ": provide as provide_localization
#import "/crates/logic/src/fixtures/expected_input_services.typ": provide as provide_data
#import "/crates/logic/src/fixtures/expected_l18n_english.typ": provide as provide_localization

#import "/crates/render/src/invoice.typ": render_invoice
#render_invoice(provide_data(), provide_localization())